//! Message Flow Control (fluent)
//!
//! Handles message queuing, priority routing, rate limiting, and flow control.
//! Implements token bucket rate limiting and message acknowledgment system.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use std::time::{SystemTime, UNIX_EPOCH};

/// Message priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Message envelope with metadata
#[derive(Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub source: String,
    pub destination: String,
    pub priority: Priority,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub requires_ack: bool,
}

impl Message {
    pub fn new(
        id: u64,
        source: String,
        destination: String,
        priority: Priority,
        payload: Vec<u8>,
        requires_ack: bool,
    ) -> Self {
        Self {
            id,
            source,
            destination,
            priority,
            payload,
            timestamp: timestamp_secs(),
            requires_ack,
        }
    }
}

/// Message comparison for priority queue (max heap)
impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Message {}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
            .then_with(|| self.timestamp.cmp(&other.timestamp))
    }
}

/// Token bucket rate limiter
pub struct TokenBucket {
    capacity: u64,
    refill_rate: u64,      // tokens per second
    tokens: Arc<RwLock<u64>>,
    last_refill: Arc<RwLock<u64>>,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        let now = timestamp_secs();
        Self {
            capacity,
            refill_rate,
            tokens: Arc::new(RwLock::new(capacity)),
            last_refill: Arc::new(RwLock::new(now)),
        }
    }

    pub async fn allow_message(&self, size: u64) -> bool {
        let mut tokens = self.tokens.write().await;
        let mut last_refill = self.last_refill.write().await;
        let now = timestamp_secs();

        // Refill tokens based on elapsed time
        let elapsed = now.saturating_sub(*last_refill);
        let new_tokens = (self.refill_rate * elapsed).min(self.capacity);
        *tokens = (*tokens + new_tokens).min(self.capacity);
        *last_refill = now;

        if *tokens >= size {
            *tokens -= size;
            true
        } else {
            false
        }
    }

    pub async fn available_tokens(&self) -> u64 {
        *self.tokens.read().await
    }
}

/// Message acknowledgment tracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AckStatus {
    Pending,
    Acknowledged,
    Failed,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct AckTracker {
    pub message_id: u64,
    pub status: AckStatus,
    pub sent_at: u64,
    pub acked_at: Option<u64>,
}

impl AckTracker {
    pub fn new(message_id: u64) -> Self {
        Self {
            message_id,
            status: AckStatus::Pending,
            sent_at: timestamp_secs(),
            acked_at: None,
        }
    }

    pub fn is_timed_out(&self, timeout_secs: u64) -> bool {
        let now = timestamp_secs();
        now.saturating_sub(self.sent_at) > timeout_secs && self.status == AckStatus::Pending
    }

    pub fn acknowledge(&mut self) {
        self.status = AckStatus::Acknowledged;
        self.acked_at = Some(timestamp_secs());
    }

    pub fn fail(&mut self) {
        self.status = AckStatus::Failed;
    }

    pub fn latency_ms(&self) -> Option<u64> {
        self.acked_at.map(|acked| (acked - self.sent_at) * 1000)
    }
}

/// Message queue with priority routing
pub struct MessageQueue {
    queue: Arc<RwLock<BinaryHeap<Message>>>,
    ack_trackers: Arc<RwLock<HashMap<u64, AckTracker>>>,
    rate_limiter: TokenBucket,
    max_size: usize,
    message_counter: Arc<RwLock<u64>>,
}

impl MessageQueue {
    pub fn new(
        max_size: usize,
        bucket_capacity: u64,
        refill_rate: u64,
    ) -> Self {
        Self {
            queue: Arc::new(RwLock::new(BinaryHeap::new())),
            ack_trackers: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: TokenBucket::new(bucket_capacity, refill_rate),
            max_size,
            message_counter: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn enqueue(&self, mut message: Message) -> Result<u64, String> {
        let mut queue = self.queue.write().await;

        if queue.len() >= self.max_size {
            return Err("Queue full".to_string());
        }

        // Rate limit: 100 bytes = 1 token
        let rate_limited = self.rate_limiter.allow_message((message.payload.len() as u64 / 100).max(1)).await;
        if !rate_limited {
            return Err("Rate limited".to_string());
        }

        let msg_id = message.id;

        if message.requires_ack {
            let mut trackers = self.ack_trackers.write().await;
            trackers.insert(msg_id, AckTracker::new(msg_id));
        }

        queue.push(message);
        Ok(msg_id)
    }

    pub async fn dequeue(&self) -> Option<Message> {
        let mut queue = self.queue.write().await;
        queue.pop()
    }

    pub async fn dequeue_by_destination(&self, dest: &str) -> Option<Message> {
        let mut queue = self.queue.write().await;
        
        let items: Vec<_> = queue.drain().collect();
        let (target, rest): (Vec<_>, Vec<_>) = items.into_iter().partition(|m| m.destination == dest);

        queue.extend(rest);

        if let Some(msg) = target.first() {
            Some(msg.clone())
        } else {
            None
        }
    }

    pub async fn queue_size(&self) -> usize {
        self.queue.read().await.len()
    }

    pub async fn acknowledge(&self, message_id: u64) -> Result<(), String> {
        let mut trackers = self.ack_trackers.write().await;
        let tracker = trackers.get_mut(&message_id).ok_or("Message not found")?;
        tracker.acknowledge();
        Ok(())
    }

    pub async fn fail_message(&self, message_id: u64) -> Result<(), String> {
        let mut trackers = self.ack_trackers.write().await;
        let tracker = trackers.get_mut(&message_id).ok_or("Message not found")?;
        tracker.fail();
        Ok(())
    }

    pub async fn get_ack_status(&self, message_id: u64) -> Option<AckStatus> {
        let trackers = self.ack_trackers.read().await;
        trackers.get(&message_id).map(|t| t.status.clone())
    }

    pub async fn cleanup_expired_acks(&self, timeout_secs: u64) -> usize {
        let mut trackers = self.ack_trackers.write().await;
        let before = trackers.len();

        trackers.retain(|_, tracker| {
            !(tracker.is_timed_out(timeout_secs) && tracker.status == AckStatus::Pending)
        });

        before - trackers.len()
    }

    pub async fn stats(&self) -> MessageQueueStats {
        let queue = self.queue.read().await;
        let trackers = self.ack_trackers.read().await;

        let pending_acks = trackers.values().filter(|t| t.status == AckStatus::Pending).count();
        let acked = trackers.values().filter(|t| t.status == AckStatus::Acknowledged).count();
        let failed = trackers.values().filter(|t| t.status == AckStatus::Failed).count();

        MessageQueueStats {
            queue_length: queue.len(),
            pending_acks,
            acknowledged: acked,
            failed,
            available_tokens: self.rate_limiter.available_tokens().await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MessageQueueStats {
    pub queue_length: usize,
    pub pending_acks: usize,
    pub acknowledged: usize,
    pub failed: usize,
    pub available_tokens: u64,
}

/// Flow control for regulating message transmission
pub struct FlowControl {
    window_size: u32,
    current_window: Arc<RwLock<u32>>,
    peer_windows: Arc<RwLock<HashMap<String, u32>>>,
}

impl FlowControl {
    pub fn new(window_size: u32) -> Self {
        Self {
            window_size,
            current_window: Arc::new(RwLock::new(window_size)),
            peer_windows: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn can_send(&self, peer_id: &str, message_size: u32) -> bool {
        let mut windows = self.peer_windows.write().await;
        let window = windows.entry(peer_id.to_string()).or_insert(self.window_size);
        
        if *window >= message_size {
            *window -= message_size;
            true
        } else {
            false
        }
    }

    pub async fn increment_window(&self, peer_id: &str, size: u32) {
        let mut windows = self.peer_windows.write().await;
        let window = windows.entry(peer_id.to_string()).or_insert(self.window_size);
        *window = (*window + size).min(self.window_size);
    }

    pub async fn reset_window(&self, peer_id: &str) {
        let mut windows = self.peer_windows.write().await;
        windows.insert(peer_id.to_string(), self.window_size);
    }

    pub async fn get_window(&self, peer_id: &str) -> u32 {
        let windows = self.peer_windows.read().await;
        windows.get(peer_id).copied().unwrap_or(self.window_size)
    }
}

fn timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        let msg_low = Message::new(1, "a".to_string(), "b".to_string(), Priority::Low, vec![], false);
        let msg_high = Message::new(2, "a".to_string(), "b".to_string(), Priority::High, vec![], false);
        
        assert!(msg_high < msg_low); // High priority sorts first
    }

    #[tokio::test]
    async fn test_token_bucket_allow() {
        let bucket = TokenBucket::new(10, 1);
        assert!(bucket.allow_message(5).await);
        assert!(bucket.allow_message(5).await);
        assert!(!bucket.allow_message(1).await);
    }

    #[tokio::test]
    async fn test_token_bucket_refill() {
        let bucket = TokenBucket::new(10, 5);
        bucket.allow_message(10).await;
        
        let tokens_before = bucket.available_tokens().await;
        assert_eq!(tokens_before, 0);
    }

    #[tokio::test]
    async fn test_message_queue_enqueue() {
        let queue = MessageQueue::new(100, 100, 10);
        let msg = Message::new(1, "a".to_string(), "b".to_string(), Priority::Normal, vec![], false);
        assert!(queue.enqueue(msg).await.is_ok());
    }

    #[tokio::test]
    async fn test_message_queue_dequeue() {
        let queue = MessageQueue::new(100, 100, 10);
        let msg = Message::new(1, "a".to_string(), "b".to_string(), Priority::Normal, vec![1, 2, 3], false);
        queue.enqueue(msg.clone()).await.unwrap();

        let dequeued = queue.dequeue().await.unwrap();
        assert_eq!(dequeued.id, msg.id);
    }

    #[tokio::test]
    async fn test_priority_queue() {
        let queue = MessageQueue::new(100, 100, 10);
        
        queue.enqueue(Message::new(1, "a".to_string(), "b".to_string(), Priority::Low, vec![], false)).await.unwrap();
        queue.enqueue(Message::new(2, "a".to_string(), "b".to_string(), Priority::High, vec![], false)).await.unwrap();

        let first = queue.dequeue().await.unwrap();
        assert_eq!(first.priority, Priority::High);
    }

    #[tokio::test]
    async fn test_ack_tracker() {
        let mut tracker = AckTracker::new(1);
        assert_eq!(tracker.status, AckStatus::Pending);
        
        tracker.acknowledge();
        assert_eq!(tracker.status, AckStatus::Acknowledged);
    }

    #[tokio::test]
    async fn test_acknowledge_message() {
        let queue = MessageQueue::new(100, 100, 10);
        let msg = Message::new(1, "a".to_string(), "b".to_string(), Priority::Normal, vec![], true);
        queue.enqueue(msg).await.unwrap();

        assert!(queue.acknowledge(1).await.is_ok());
        let status = queue.get_ack_status(1).await;
        assert_eq!(status, Some(AckStatus::Acknowledged));
    }

    #[tokio::test]
    async fn test_queue_full() {
        let queue = MessageQueue::new(1, 100, 10);
        queue.enqueue(Message::new(1, "a".to_string(), "b".to_string(), Priority::Normal, vec![], false)).await.unwrap();

        let result = queue.enqueue(Message::new(2, "a".to_string(), "b".to_string(), Priority::Normal, vec![], false)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dequeue_by_destination() {
        let queue = MessageQueue::new(100, 100, 10);
        queue.enqueue(Message::new(1, "a".to_string(), "b".to_string(), Priority::Normal, vec![], false)).await.unwrap();
        queue.enqueue(Message::new(2, "a".to_string(), "c".to_string(), Priority::Normal, vec![], false)).await.unwrap();

        let msg = queue.dequeue_by_destination("c").await.unwrap();
        assert_eq!(msg.destination, "c");
    }

    #[tokio::test]
    async fn test_flow_control_can_send() {
        let fc = FlowControl::new(1000);
        assert!(fc.can_send("peer1", 500).await);
        assert!(fc.can_send("peer1", 500).await);
        assert!(!fc.can_send("peer1", 100).await);
    }

    #[tokio::test]
    async fn test_flow_control_increment_window() {
        let fc = FlowControl::new(1000);
        fc.can_send("peer1", 1000).await;
        
        fc.increment_window("peer1", 500).await;
        assert!(fc.can_send("peer1", 400).await);
    }

    #[tokio::test]
    async fn test_flow_control_reset_window() {
        let fc = FlowControl::new(1000);
        fc.can_send("peer1", 1000).await;
        
        fc.reset_window("peer1").await;
        assert!(fc.can_send("peer1", 1000).await);
    }

    #[tokio::test]
    async fn test_stats() {
        let queue = MessageQueue::new(100, 100, 10);
        queue.enqueue(Message::new(1, "a".to_string(), "b".to_string(), Priority::Normal, vec![], true)).await.unwrap();
        
        let stats = queue.stats().await;
        assert_eq!(stats.queue_length, 1);
        assert_eq!(stats.pending_acks, 1);
    }
}
