//! Recurring Payments
//!
//! Automated recurring payments for subscriptions, payroll, and
//! scheduled transfers on the Lightning Network.
//!
//! Features:
//! - Multiple frequencies (daily, weekly, monthly, custom)
//! - Authorization management
//! - Payment history tracking
//! - Automatic execution
//! - Cancellation support

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

/// Payment frequency
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaymentFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Custom(u64), // Custom interval in seconds
}

impl PaymentFrequency {
    pub fn seconds(&self) -> u64 {
        match self {
            PaymentFrequency::Daily => 86400,           // 24 hours
            PaymentFrequency::Weekly => 604800,         // 7 days
            PaymentFrequency::Monthly => 2592000,       // 30 days
            PaymentFrequency::Yearly => 31536000,       // 365 days
            PaymentFrequency::Custom(secs) => *secs,
        }
    }
}

/// Recurring payment status
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecurringStatus {
    Active,
    Paused,
    Cancelled,
    Expired,
    Failed,
}

/// Recurring payment
#[derive(Clone, Debug)]
pub struct RecurringPayment {
    pub payment_id: String,
    pub payer: String,
    pub payee: String,
    pub amount: u128,
    pub frequency: PaymentFrequency,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub max_payments: Option<usize>,
    pub status: RecurringStatus,
    pub payments_made: usize,
    pub last_payment_at: Option<u64>,
    pub next_payment_at: u64,
    pub total_paid: u128,
}

impl RecurringPayment {
    pub fn new(
        payment_id: String,
        payer: String,
        payee: String,
        amount: u128,
        frequency: PaymentFrequency,
        start_date: u64,
    ) -> Result<Self, RecurringError> {
        if amount == 0 {
            return Err(RecurringError::InvalidAmount);
        }

        Ok(Self {
            payment_id,
            payer,
            payee,
            amount,
            frequency: frequency.clone(),
            start_date,
            end_date: None,
            max_payments: None,
            status: RecurringStatus::Active,
            payments_made: 0,
            last_payment_at: None,
            next_payment_at: start_date,
            total_paid: 0,
        })
    }

    /// Set end date
    pub fn set_end_date(&mut self, end_date: u64) -> Result<(), RecurringError> {
        if end_date <= self.start_date {
            return Err(RecurringError::InvalidEndDate);
        }
        self.end_date = Some(end_date);
        Ok(())
    }

    /// Set maximum number of payments
    pub fn set_max_payments(&mut self, max: usize) {
        self.max_payments = Some(max);
    }

    /// Check if payment is due
    pub fn is_due(&self, current_time: u64) -> bool {
        if self.status != RecurringStatus::Active {
            return false;
        }

        current_time >= self.next_payment_at
    }

    /// Execute payment
    pub fn execute_payment(&mut self, current_time: u64) -> Result<PaymentExecution, RecurringError> {
        if !self.is_due(current_time) {
            return Err(RecurringError::PaymentNotDue);
        }

        if self.status != RecurringStatus::Active {
            return Err(RecurringError::NotActive);
        }

        // Check if expired
        if let Some(end_date) = self.end_date {
            if current_time > end_date {
                self.status = RecurringStatus::Expired;
                return Err(RecurringError::PaymentExpired);
            }
        }

        // Check if max payments reached
        if let Some(max) = self.max_payments {
            if self.payments_made >= max {
                self.status = RecurringStatus::Expired;
                return Err(RecurringError::MaxPaymentsReached);
            }
        }

        // Execute payment
        self.payments_made += 1;
        self.total_paid += self.amount;
        self.last_payment_at = Some(current_time);
        self.next_payment_at = current_time + self.frequency.seconds();

        // Check if this was the last payment
        if let Some(max) = self.max_payments {
            if self.payments_made >= max {
                self.status = RecurringStatus::Expired;
            }
        }

        Ok(PaymentExecution {
            payment_id: self.payment_id.clone(),
            payer: self.payer.clone(),
            payee: self.payee.clone(),
            amount: self.amount,
            timestamp: current_time,
            payment_number: self.payments_made,
        })
    }

    /// Pause recurring payment
    pub fn pause(&mut self) -> Result<(), RecurringError> {
        if self.status != RecurringStatus::Active {
            return Err(RecurringError::NotActive);
        }

        self.status = RecurringStatus::Paused;
        Ok(())
    }

    /// Resume recurring payment
    pub fn resume(&mut self, current_time: u64) -> Result<(), RecurringError> {
        if self.status != RecurringStatus::Paused {
            return Err(RecurringError::InvalidState);
        }

        self.status = RecurringStatus::Active;

        // Recalculate next payment time
        if let Some(last) = self.last_payment_at {
            self.next_payment_at = last + self.frequency.seconds();
        } else {
            self.next_payment_at = current_time;
        }

        Ok(())
    }

    /// Cancel recurring payment
    pub fn cancel(&mut self) -> Result<(), RecurringError> {
        if self.status == RecurringStatus::Cancelled {
            return Err(RecurringError::AlreadyCancelled);
        }

        self.status = RecurringStatus::Cancelled;
        Ok(())
    }

    /// Get next payment due in seconds
    pub fn next_payment_in(&self, current_time: u64) -> u64 {
        if current_time >= self.next_payment_at {
            0
        } else {
            self.next_payment_at - current_time
        }
    }

    /// Check if payment is active
    pub fn is_active(&self) -> bool {
        self.status == RecurringStatus::Active
    }
}

/// Payment execution record
#[derive(Clone, Debug)]
pub struct PaymentExecution {
    pub payment_id: String,
    pub payer: String,
    pub payee: String,
    pub amount: u128,
    pub timestamp: u64,
    pub payment_number: usize,
}

/// Recurring payment manager
pub struct RecurringManager {
    payments: HashMap<String, RecurringPayment>,
}

impl RecurringManager {
    pub fn new() -> Self {
        Self {
            payments: HashMap::new(),
        }
    }

    /// Create recurring payment
    pub fn create_payment(
        &mut self,
        payment_id: String,
        payer: String,
        payee: String,
        amount: u128,
        frequency: PaymentFrequency,
        start_date: u64,
    ) -> Result<String, RecurringError> {
        if self.payments.contains_key(&payment_id) {
            return Err(RecurringError::PaymentAlreadyExists);
        }

        let payment = RecurringPayment::new(
            payment_id.clone(),
            payer,
            payee,
            amount,
            frequency,
            start_date,
        )?;

        self.payments.insert(payment_id.clone(), payment);
        Ok(payment_id)
    }

    /// Get payment
    pub fn get_payment(&self, payment_id: &str) -> Result<&RecurringPayment, RecurringError> {
        self.payments.get(payment_id).ok_or(RecurringError::PaymentNotFound)
    }

    /// Get mutable payment
    fn get_payment_mut(&mut self, payment_id: &str) -> Result<&mut RecurringPayment, RecurringError> {
        self.payments.get_mut(payment_id).ok_or(RecurringError::PaymentNotFound)
    }

    /// Process all due payments
    pub fn process_due_payments(&mut self, current_time: u64) -> Vec<PaymentExecution> {
        let mut executions = Vec::new();

        for payment in self.payments.values_mut() {
            if payment.is_due(current_time) {
                if let Ok(execution) = payment.execute_payment(current_time) {
                    executions.push(execution);
                }
            }
        }

        executions
    }

    /// Pause payment
    pub fn pause_payment(&mut self, payment_id: &str) -> Result<(), RecurringError> {
        let payment = self.get_payment_mut(payment_id)?;
        payment.pause()
    }

    /// Resume payment
    pub fn resume_payment(&mut self, payment_id: &str, current_time: u64) -> Result<(), RecurringError> {
        let payment = self.get_payment_mut(payment_id)?;
        payment.resume(current_time)
    }

    /// Cancel payment
    pub fn cancel_payment(&mut self, payment_id: &str) -> Result<(), RecurringError> {
        let payment = self.get_payment_mut(payment_id)?;
        payment.cancel()
    }

    /// Get all active payments
    pub fn get_active_payments(&self) -> Vec<&RecurringPayment> {
        self.payments
            .values()
            .filter(|p| p.is_active())
            .collect()
    }

    /// Get statistics
    pub fn statistics(&self) -> RecurringStatistics {
        let total = self.payments.len();
        let active = self.payments.values().filter(|p| p.status == RecurringStatus::Active).count();
        let paused = self.payments.values().filter(|p| p.status == RecurringStatus::Paused).count();
        let total_volume: u128 = self.payments.values().map(|p| p.total_paid).sum();

        RecurringStatistics {
            total_payments: total,
            active_payments: active,
            paused_payments: paused,
            total_volume,
        }
    }
}

impl Default for RecurringManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Recurring payment statistics
#[derive(Clone, Debug)]
pub struct RecurringStatistics {
    pub total_payments: usize,
    pub active_payments: usize,
    pub paused_payments: usize,
    pub total_volume: u128,
}

/// Recurring payment errors
#[derive(Clone, Debug, PartialEq)]
pub enum RecurringError {
    InvalidAmount,
    InvalidEndDate,
    PaymentNotDue,
    PaymentNotFound,
    PaymentAlreadyExists,
    PaymentExpired,
    MaxPaymentsReached,
    NotActive,
    AlreadyCancelled,
    InvalidState,
}

impl core::fmt::Display for RecurringError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            RecurringError::InvalidAmount => write!(f, "Invalid payment amount"),
            RecurringError::InvalidEndDate => write!(f, "Invalid end date"),
            RecurringError::PaymentNotDue => write!(f, "Payment not yet due"),
            RecurringError::PaymentNotFound => write!(f, "Payment not found"),
            RecurringError::PaymentAlreadyExists => write!(f, "Payment already exists"),
            RecurringError::PaymentExpired => write!(f, "Payment has expired"),
            RecurringError::MaxPaymentsReached => write!(f, "Maximum payments reached"),
            RecurringError::NotActive => write!(f, "Payment is not active"),
            RecurringError::AlreadyCancelled => write!(f, "Payment already cancelled"),
            RecurringError::InvalidState => write!(f, "Invalid payment state"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_seconds() {
        assert_eq!(PaymentFrequency::Daily.seconds(), 86400);
        assert_eq!(PaymentFrequency::Weekly.seconds(), 604800);
        assert_eq!(PaymentFrequency::Custom(3600).seconds(), 3600);
    }

    #[test]
    fn test_recurring_payment_creation() {
        let payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Monthly,
            1000,
        );

        assert!(payment.is_ok());
        let p = payment.unwrap();
        assert_eq!(p.amount, 1000);
        assert_eq!(p.payments_made, 0);
        assert_eq!(p.status, RecurringStatus::Active);
    }

    #[test]
    fn test_invalid_amount() {
        let payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            0, // Invalid
            PaymentFrequency::Monthly,
            1000,
        );

        assert_eq!(payment, Err(RecurringError::InvalidAmount));
    }

    #[test]
    fn test_set_end_date() {
        let mut payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Monthly,
            1000,
        ).unwrap();

        assert!(payment.set_end_date(5000).is_ok());
        assert_eq!(payment.end_date, Some(5000));
    }

    #[test]
    fn test_set_max_payments() {
        let mut payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Monthly,
            1000,
        ).unwrap();

        payment.set_max_payments(12);
        assert_eq!(payment.max_payments, Some(12));
    }

    #[test]
    fn test_is_due() {
        let payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        assert!(payment.is_due(1000)); // Due at start
        assert!(payment.is_due(2000)); // Due after start
        assert!(!payment.is_due(500));  // Not due before start
    }

    #[test]
    fn test_execute_payment() {
        let mut payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        let execution = payment.execute_payment(1000);
        assert!(execution.is_ok());

        let exec = execution.unwrap();
        assert_eq!(exec.amount, 1000);
        assert_eq!(exec.payment_number, 1);

        assert_eq!(payment.payments_made, 1);
        assert_eq!(payment.total_paid, 1000);
    }

    #[test]
    fn test_max_payments_reached() {
        let mut payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        payment.set_max_payments(2);

        // First payment
        payment.execute_payment(1000).unwrap();
        assert_eq!(payment.status, RecurringStatus::Active);

        // Second payment
        payment.execute_payment(86400 + 1000).unwrap();
        assert_eq!(payment.status, RecurringStatus::Expired);

        // Third payment should fail
        let result = payment.execute_payment(2 * 86400 + 1000);
        assert_eq!(result, Err(RecurringError::MaxPaymentsReached));
    }

    #[test]
    fn test_pause_resume() {
        let mut payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        assert!(payment.pause().is_ok());
        assert_eq!(payment.status, RecurringStatus::Paused);

        // Cannot execute while paused
        let result = payment.execute_payment(1000);
        assert_eq!(result, Err(RecurringError::NotActive));

        // Resume
        assert!(payment.resume(1000).is_ok());
        assert_eq!(payment.status, RecurringStatus::Active);
    }

    #[test]
    fn test_cancel() {
        let mut payment = RecurringPayment::new(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        assert!(payment.cancel().is_ok());
        assert_eq!(payment.status, RecurringStatus::Cancelled);

        // Cannot cancel again
        assert_eq!(payment.cancel(), Err(RecurringError::AlreadyCancelled));
    }

    #[test]
    fn test_recurring_manager() {
        let mut manager = RecurringManager::new();

        let payment_id = manager.create_payment(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        );

        assert!(payment_id.is_ok());
        assert_eq!(manager.get_active_payments().len(), 1);
    }

    #[test]
    fn test_process_due_payments() {
        let mut manager = RecurringManager::new();

        manager.create_payment(
            "pay1".to_string(),
            "alice".to_string(),
            "bob".to_string(),
            1000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        manager.create_payment(
            "pay2".to_string(),
            "charlie".to_string(),
            "dave".to_string(),
            2000,
            PaymentFrequency::Daily,
            1000,
        ).unwrap();

        // Process at start date
        let executions = manager.process_due_payments(1000);
        assert_eq!(executions.len(), 2);

        // Process again immediately - nothing should be due
        let executions2 = manager.process_due_payments(1000);
        assert_eq!(executions2.len(), 0);

        // Process 1 day later - both should be due again
        let executions3 = manager.process_due_payments(1000 + 86400);
        assert_eq!(executions3.len(), 2);
    }

    #[test]
    fn test_statistics() {
        let mut manager = RecurringManager::new();

        manager.create_payment("pay1".to_string(), "a".to_string(), "b".to_string(), 1000, PaymentFrequency::Daily, 1000).unwrap();
        manager.create_payment("pay2".to_string(), "c".to_string(), "d".to_string(), 2000, PaymentFrequency::Daily, 1000).unwrap();

        manager.process_due_payments(1000);
        manager.pause_payment("pay1").unwrap();

        let stats = manager.statistics();
        assert_eq!(stats.total_payments, 2);
        assert_eq!(stats.active_payments, 1);
        assert_eq!(stats.paused_payments, 1);
        assert_eq!(stats.total_volume, 3000);
    }
}
