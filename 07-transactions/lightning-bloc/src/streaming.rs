//! Streaming Payments
//!
//! Enables per-second micropayments for continuous services like
//! video streaming, API calls, or time-based billing.
//!
//! Features:
//! - Per-second payment rates
//! - Automatic payment execution
//! - Real-time balance tracking
//! - Stream pause/resume
//! - Usage-based billing

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    vec::Vec,
    string::{String, ToString},
    collections::BTreeMap as HashMap,
    format,
};

#[cfg(feature = "std")]
use std::{
    vec::Vec,
    string::String,
    collections::HashMap,
};

/// Minimum rate per second
pub const MIN_RATE_PER_SECOND: u128 = 1;

/// Payment interval in seconds
pub const DEFAULT_PAYMENT_INTERVAL: u64 = 10; // Pay every 10 seconds

/// Stream status
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamStatus {
    Active,
    Paused,
    Stopped,
    Expired,
}

/// Streaming payment
#[derive(Clone, Debug)]
pub struct StreamingPayment {
    pub stream_id: String,
    pub payer: String,
    pub payee: String,
    pub rate_per_second: u128,
    pub started_at: u64,
    pub last_payment_at: u64,
    pub total_paid: u128,
    pub total_seconds: u64,
    pub status: StreamStatus,
    pub max_total: Option<u128>,
}

impl StreamingPayment {
    pub fn new(
        stream_id: String,
        payer: String,
        payee: String,
        rate_per_second: u128,
        started_at: u64,
    ) -> Result<Self, StreamError> {
        if rate_per_second < MIN_RATE_PER_SECOND {
            return Err(StreamError::RateTooLow);
        }

        Ok(Self {
            stream_id,
            payer,
            payee,
            rate_per_second,
            started_at,
            last_payment_at: started_at,
            total_paid: 0,
            total_seconds: 0,
            status: StreamStatus::Active,
            max_total: None,
        })
    }

    /// Set maximum total payment
    pub fn set_max_total(&mut self, max: u128) {
        self.max_total = Some(max);
    }

    /// Calculate payment for elapsed time
    pub fn calculate_payment(&self, current_time: u64) -> u128 {
        if self.status != StreamStatus::Active {
            return 0;
        }

        let elapsed = current_time.saturating_sub(self.last_payment_at);
        let payment = (elapsed as u128) * self.rate_per_second;

        // Cap at max_total if set
        if let Some(max) = self.max_total {
            let remaining = max.saturating_sub(self.total_paid);
            payment.min(remaining)
        } else {
            payment
        }
    }

    /// Update payment (call this periodically)
    pub fn update_payment(&mut self, current_time: u64) -> Result<u128, StreamError> {
        if self.status != StreamStatus::Active {
            return Err(StreamError::StreamNotActive);
        }

        let payment = self.calculate_payment(current_time);

        if payment == 0 {
            return Ok(0);
        }

        // Check if we've hit the max
        if let Some(max) = self.max_total {
            if self.total_paid + payment >= max {
                self.status = StreamStatus::Expired;
            }
        }

        let elapsed = current_time.saturating_sub(self.last_payment_at);
        self.total_paid += payment;
        self.total_seconds += elapsed;
        self.last_payment_at = current_time;

        Ok(payment)
    }

    /// Pause the stream
    pub fn pause(&mut self, current_time: u64) -> Result<u128, StreamError> {
        if self.status != StreamStatus::Active {
            return Err(StreamError::StreamNotActive);
        }

        // Process final payment before pausing
        let payment = self.update_payment(current_time)?;
        self.status = StreamStatus::Paused;

        Ok(payment)
    }

    /// Resume the stream
    pub fn resume(&mut self, current_time: u64) -> Result<(), StreamError> {
        if self.status != StreamStatus::Paused {
            return Err(StreamError::InvalidState);
        }

        self.status = StreamStatus::Active;
        self.last_payment_at = current_time;

        Ok(())
    }

    /// Stop the stream permanently
    pub fn stop(&mut self, current_time: u64) -> Result<u128, StreamError> {
        if self.status == StreamStatus::Stopped {
            return Err(StreamError::AlreadyStopped);
        }

        // Process final payment
        let payment = if self.status == StreamStatus::Active {
            self.update_payment(current_time)?
        } else {
            0
        };

        self.status = StreamStatus::Stopped;

        Ok(payment)
    }

    /// Get current rate (cost per hour)
    pub fn rate_per_hour(&self) -> u128 {
        self.rate_per_second * 3600
    }

    /// Get streaming duration in seconds
    pub fn duration(&self) -> u64 {
        self.total_seconds
    }

    /// Check if stream is still active
    pub fn is_active(&self) -> bool {
        self.status == StreamStatus::Active
    }
}

/// Streaming payment manager
pub struct StreamManager {
    streams: HashMap<String, StreamingPayment>,
    payment_interval: u64,
}

impl StreamManager {
    pub fn new() -> Self {
        Self {
            streams: HashMap::new(),
            payment_interval: DEFAULT_PAYMENT_INTERVAL,
        }
    }

    /// Start new streaming payment
    pub fn start_stream(
        &mut self,
        stream_id: String,
        payer: String,
        payee: String,
        rate_per_second: u128,
        current_time: u64,
    ) -> Result<String, StreamError> {
        if self.streams.contains_key(&stream_id) {
            return Err(StreamError::StreamAlreadyExists);
        }

        let stream = StreamingPayment::new(
            stream_id.clone(),
            payer,
            payee,
            rate_per_second,
            current_time,
        )?;

        self.streams.insert(stream_id.clone(), stream);
        Ok(stream_id)
    }

    /// Get stream
    pub fn get_stream(&self, stream_id: &str) -> Result<&StreamingPayment, StreamError> {
        self.streams.get(stream_id).ok_or(StreamError::StreamNotFound)
    }

    /// Get mutable stream
    fn get_stream_mut(&mut self, stream_id: &str) -> Result<&mut StreamingPayment, StreamError> {
        self.streams.get_mut(stream_id).ok_or(StreamError::StreamNotFound)
    }

    /// Update all active streams
    pub fn update_all_streams(&mut self, current_time: u64) -> Vec<StreamPayment> {
        let mut payments = Vec::new();

        for stream in self.streams.values_mut() {
            if stream.status == StreamStatus::Active {
                if let Ok(amount) = stream.update_payment(current_time) {
                    if amount > 0 {
                        payments.push(StreamPayment {
                            stream_id: stream.stream_id.clone(),
                            payer: stream.payer.clone(),
                            payee: stream.payee.clone(),
                            amount,
                        });
                    }
                }
            }
        }

        payments
    }

    /// Pause stream
    pub fn pause_stream(&mut self, stream_id: &str, current_time: u64) -> Result<u128, StreamError> {
        let stream = self.get_stream_mut(stream_id)?;
        stream.pause(current_time)
    }

    /// Resume stream
    pub fn resume_stream(&mut self, stream_id: &str, current_time: u64) -> Result<(), StreamError> {
        let stream = self.get_stream_mut(stream_id)?;
        stream.resume(current_time)
    }

    /// Stop stream
    pub fn stop_stream(&mut self, stream_id: &str, current_time: u64) -> Result<u128, StreamError> {
        let stream = self.get_stream_mut(stream_id)?;
        stream.stop(current_time)
    }

    /// Get all active streams
    pub fn get_active_streams(&self) -> Vec<&StreamingPayment> {
        self.streams
            .values()
            .filter(|s| s.status == StreamStatus::Active)
            .collect()
    }

    /// Get statistics
    pub fn statistics(&self) -> StreamStatistics {
        let total_streams = self.streams.len();
        let active = self.streams.values().filter(|s| s.status == StreamStatus::Active).count();
        let paused = self.streams.values().filter(|s| s.status == StreamStatus::Paused).count();
        let total_paid: u128 = self.streams.values().map(|s| s.total_paid).sum();

        StreamStatistics {
            total_streams,
            active_streams: active,
            paused_streams: paused,
            total_volume: total_paid,
        }
    }
}

impl Default for StreamManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Stream payment event
#[derive(Clone, Debug)]
pub struct StreamPayment {
    pub stream_id: String,
    pub payer: String,
    pub payee: String,
    pub amount: u128,
}

/// Stream statistics
#[derive(Clone, Debug)]
pub struct StreamStatistics {
    pub total_streams: usize,
    pub active_streams: usize,
    pub paused_streams: usize,
    pub total_volume: u128,
}

/// Stream errors
#[derive(Clone, Debug, PartialEq)]
pub enum StreamError {
    RateTooLow,
    StreamNotFound,
    StreamAlreadyExists,
    StreamNotActive,
    AlreadyStopped,
    InvalidState,
}

impl core::fmt::Display for StreamError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            StreamError::RateTooLow => write!(f, "Rate per second too low"),
            StreamError::StreamNotFound => write!(f, "Stream not found"),
            StreamError::StreamAlreadyExists => write!(f, "Stream already exists"),
            StreamError::StreamNotActive => write!(f, "Stream is not active"),
            StreamError::AlreadyStopped => write!(f, "Stream already stopped"),
            StreamError::InvalidState => write!(f, "Invalid stream state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_payment_creation() {
        let stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100, // 100 units per second
            1000,
        );

        assert!(stream.is_ok());
        let s = stream.unwrap();
        assert_eq!(s.rate_per_second, 100);
        assert_eq!(s.total_paid, 0);
        assert_eq!(s.status, StreamStatus::Active);
    }

    #[test]
    fn test_rate_too_low() {
        let stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            0, // Too low
            1000,
        );

        assert_eq!(stream, Err(StreamError::RateTooLow));
    }

    #[test]
    fn test_calculate_payment() {
        let stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        // 10 seconds elapsed
        let payment = stream.calculate_payment(1010);
        assert_eq!(payment, 1000); // 10 * 100
    }

    #[test]
    fn test_update_payment() {
        let mut stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        // 10 seconds later
        let payment = stream.update_payment(1010).unwrap();
        assert_eq!(payment, 1000);
        assert_eq!(stream.total_paid, 1000);
        assert_eq!(stream.total_seconds, 10);
    }

    #[test]
    fn test_max_total() {
        let mut stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        stream.set_max_total(500); // Max 500 units

        // 10 seconds would cost 1000, but capped at 500
        let payment = stream.calculate_payment(1010);
        assert_eq!(payment, 500);

        stream.update_payment(1010).unwrap();
        assert_eq!(stream.status, StreamStatus::Expired);
    }

    #[test]
    fn test_pause_resume() {
        let mut stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        // Run for 10 seconds
        stream.update_payment(1010).unwrap();
        assert_eq!(stream.total_paid, 1000);

        // Pause at 15 seconds (5 more seconds)
        let final_payment = stream.pause(1015).unwrap();
        assert_eq!(final_payment, 500);
        assert_eq!(stream.status, StreamStatus::Paused);

        // Resume at 20 seconds
        stream.resume(1020).unwrap();
        assert_eq!(stream.status, StreamStatus::Active);

        // Update at 25 seconds (5 seconds of active streaming)
        stream.update_payment(1025).unwrap();
        assert_eq!(stream.total_paid, 2000); // 1000 + 500 + 500
    }

    #[test]
    fn test_stop() {
        let mut stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        stream.update_payment(1010).unwrap();

        let final_payment = stream.stop(1015).unwrap();
        assert_eq!(final_payment, 500);
        assert_eq!(stream.status, StreamStatus::Stopped);

        // Cannot stop again
        assert_eq!(stream.stop(1020), Err(StreamError::AlreadyStopped));
    }

    #[test]
    fn test_rate_per_hour() {
        let stream = StreamingPayment::new(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        assert_eq!(stream.rate_per_hour(), 360000); // 100 * 3600
    }

    #[test]
    fn test_stream_manager_start() {
        let mut manager = StreamManager::new();

        let stream_id = manager.start_stream(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        );

        assert!(stream_id.is_ok());
        assert_eq!(manager.get_active_streams().len(), 1);
    }

    #[test]
    fn test_stream_manager_duplicate() {
        let mut manager = StreamManager::new();

        manager.start_stream(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        ).unwrap();

        let result = manager.start_stream(
            "stream1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            100,
            1000,
        );

        assert_eq!(result, Err(StreamError::StreamAlreadyExists));
    }

    #[test]
    fn test_update_all_streams() {
        let mut manager = StreamManager::new();

        manager.start_stream("stream1".to_string(), "alice".to_string(), "bob".to_string(), 100, 1000).unwrap();
        manager.start_stream("stream2".to_string(), "charlie".to_string(), "dave".to_string(), 200, 1000).unwrap();

        // Update all streams 10 seconds later
        let payments = manager.update_all_streams(1010);

        assert_eq!(payments.len(), 2);
        assert_eq!(payments[0].amount, 1000); // 100 * 10
        assert_eq!(payments[1].amount, 2000); // 200 * 10
    }

    #[test]
    fn test_statistics() {
        let mut manager = StreamManager::new();

        manager.start_stream("stream1".to_string(), "alice".to_string(), "bob".to_string(), 100, 1000).unwrap();
        manager.start_stream("stream2".to_string(), "charlie".to_string(), "dave".to_string(), 200, 1000).unwrap();

        manager.update_all_streams(1010);
        manager.pause_stream("stream1", 1010).unwrap();

        let stats = manager.statistics();
        assert_eq!(stats.total_streams, 2);
        assert_eq!(stats.active_streams, 1);
        assert_eq!(stats.paused_streams, 1);
        assert_eq!(stats.total_volume, 3000);
    }
}
