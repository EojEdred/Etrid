//! Comprehensive Event System with Indexing
//!
//! This module provides a full-featured event system for smart contracts:
//! - Event emission from contracts (LOG0-LOG4)
//! - Event indexing for efficient querying
//! - Topic-based filtering
//! - Event storage and retrieval
//! - Bloom filters for fast event lookup

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_std::prelude::*;

/// Maximum number of topics per event (EVM standard)
pub const MAX_TOPICS: usize = 4;

/// Maximum event data size (64 KB)
pub const MAX_EVENT_DATA_SIZE: usize = 65536;

/// Event topic type (256-bit hash)
pub type EventTopic = H256;

/// Event log entry (compatible with EVM LOG0-LOG4)
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct EventLog {
    /// Contract address that emitted the event
    pub address: [u8; 32],
    /// Event topics (indexed parameters)
    pub topics: Vec<EventTopic>,
    /// Event data (non-indexed parameters)
    pub data: Vec<u8>,
    /// Block number when event was emitted
    pub block_number: u64,
    /// Transaction index in block
    pub transaction_index: u32,
    /// Log index in transaction
    pub log_index: u32,
}

impl EventLog {
    /// Create a new event log
    pub fn new(
        address: [u8; 32],
        topics: Vec<EventTopic>,
        data: Vec<u8>,
        block_number: u64,
        transaction_index: u32,
        log_index: u32,
    ) -> Result<Self, EventError> {
        // Validate topics count
        if topics.len() > MAX_TOPICS {
            return Err(EventError::TooManyTopics);
        }

        // Validate data size
        if data.len() > MAX_EVENT_DATA_SIZE {
            return Err(EventError::DataTooLarge);
        }

        Ok(Self {
            address,
            topics,
            data,
            block_number,
            transaction_index,
            log_index,
        })
    }

    /// Get the number of topics (LOG0=0, LOG1=1, ..., LOG4=4)
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    /// Check if event matches a topic filter
    pub fn matches_topics(&self, filter: &[Option<EventTopic>]) -> bool {
        if filter.len() > self.topics.len() {
            return false;
        }

        for (i, filter_topic) in filter.iter().enumerate() {
            if let Some(topic) = filter_topic {
                if self.topics.get(i) != Some(topic) {
                    return false;
                }
            }
        }

        true
    }

    /// Check if event matches address filter
    pub fn matches_address(&self, addresses: &[[u8; 32]]) -> bool {
        addresses.is_empty() || addresses.contains(&self.address)
    }
}

/// Event filter for querying events
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct EventFilter {
    /// Filter by contract addresses (empty = all)
    pub addresses: Vec<[u8; 32]>,
    /// Filter by topics (None = any)
    pub topics: Vec<Option<EventTopic>>,
    /// From block number (inclusive)
    pub from_block: u64,
    /// To block number (inclusive)
    pub to_block: u64,
    /// Maximum number of results
    pub limit: Option<u32>,
}

impl EventFilter {
    /// Create a new event filter
    pub fn new(from_block: u64, to_block: u64) -> Self {
        Self {
            addresses: Vec::new(),
            topics: Vec::new(),
            from_block,
            to_block,
            limit: None,
        }
    }

    /// Add address filter
    pub fn with_address(mut self, address: [u8; 32]) -> Self {
        self.addresses.push(address);
        self
    }

    /// Add topic filter
    pub fn with_topic(mut self, topic: Option<EventTopic>) -> Self {
        self.topics.push(topic);
        self
    }

    /// Set result limit
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Check if an event matches this filter
    pub fn matches(&self, event: &EventLog) -> bool {
        // Check block range
        if event.block_number < self.from_block || event.block_number > self.to_block {
            return false;
        }

        // Check address
        if !event.matches_address(&self.addresses) {
            return false;
        }

        // Check topics
        if !event.matches_topics(&self.topics) {
            return false;
        }

        true
    }
}

/// Event index entry for efficient lookups
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct EventIndexEntry {
    /// Block number
    pub block_number: u64,
    /// Transaction index
    pub transaction_index: u32,
    /// Log index
    pub log_index: u32,
}

impl EventIndexEntry {
    /// Create a new index entry
    pub fn new(block_number: u64, transaction_index: u32, log_index: u32) -> Self {
        Self {
            block_number,
            transaction_index,
            log_index,
        }
    }

    /// Create from an event log
    pub fn from_event(event: &EventLog) -> Self {
        Self::new(event.block_number, event.transaction_index, event.log_index)
    }
}

/// Bloom filter for fast event existence checks
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct EventBloomFilter {
    /// Bloom filter bits (2048 bits = 256 bytes)
    bits: [u8; 256],
}

impl EventBloomFilter {
    /// Create a new empty bloom filter
    pub fn new() -> Self {
        Self { bits: [0u8; 256] }
    }

    /// Add an item to the bloom filter
    pub fn add(&mut self, item: &[u8]) {
        for i in 0..3 {
            let hash = self.hash(item, i);
            let byte_index = (hash / 8) as usize % 256;
            let bit_index = hash % 8;
            self.bits[byte_index] |= 1 << bit_index;
        }
    }

    /// Check if an item might be in the filter
    pub fn contains(&self, item: &[u8]) -> bool {
        for i in 0..3 {
            let hash = self.hash(item, i);
            let byte_index = (hash / 8) as usize % 256;
            let bit_index = hash % 8;
            if (self.bits[byte_index] & (1 << bit_index)) == 0 {
                return false;
            }
        }
        true
    }

    /// Combine with another bloom filter (OR operation)
    pub fn combine(&mut self, other: &EventBloomFilter) {
        for i in 0..256 {
            self.bits[i] |= other.bits[i];
        }
    }

    /// Simple hash function for bloom filter
    fn hash(&self, data: &[u8], seed: u32) -> u32 {
        let mut hash = seed;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }
}

impl Default for EventBloomFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Event storage and indexing system
#[derive(Debug, Clone)]
pub struct EventStore {
    /// All events by block number
    events_by_block: BTreeMap<u64, Vec<EventLog>>,
    /// Index by contract address
    address_index: BTreeMap<[u8; 32], Vec<EventIndexEntry>>,
    /// Index by topic
    topic_index: BTreeMap<EventTopic, Vec<EventIndexEntry>>,
    /// Bloom filters by block
    block_blooms: BTreeMap<u64, EventBloomFilter>,
    /// Total number of events
    total_events: u64,
}

impl EventStore {
    /// Create a new event store
    pub fn new() -> Self {
        Self {
            events_by_block: BTreeMap::new(),
            address_index: BTreeMap::new(),
            topic_index: BTreeMap::new(),
            block_blooms: BTreeMap::new(),
            total_events: 0,
        }
    }

    /// Add an event to the store
    pub fn add_event(&mut self, event: EventLog) -> Result<(), EventError> {
        let block_number = event.block_number;
        let address = event.address;
        let index_entry = EventIndexEntry::from_event(&event);

        // Add to block index
        self.events_by_block
            .entry(block_number)
            .or_insert_with(Vec::new)
            .push(event.clone());

        // Add to address index
        self.address_index
            .entry(address)
            .or_insert_with(Vec::new)
            .push(index_entry.clone());

        // Add to topic indices
        for topic in &event.topics {
            self.topic_index
                .entry(*topic)
                .or_insert_with(Vec::new)
                .push(index_entry.clone());
        }

        // Update bloom filter
        let bloom = self.block_blooms
            .entry(block_number)
            .or_insert_with(EventBloomFilter::new);

        bloom.add(&address);
        for topic in &event.topics {
            bloom.add(topic.as_bytes());
        }

        self.total_events += 1;

        Ok(())
    }

    /// Query events with a filter
    pub fn query_events(&self, filter: &EventFilter) -> Vec<EventLog> {
        let mut results = Vec::new();

        // Iterate through block range
        for block_num in filter.from_block..=filter.to_block {
            // Quick check with bloom filter
            if let Some(bloom) = self.block_blooms.get(&block_num) {
                let mut should_check_block = false;

                // Check if any address matches
                if filter.addresses.is_empty() {
                    should_check_block = true;
                } else {
                    for address in &filter.addresses {
                        if bloom.contains(address) {
                            should_check_block = true;
                            break;
                        }
                    }
                }

                // Check if any topic matches
                if should_check_block {
                    for topic_option in &filter.topics {
                        if let Some(topic) = topic_option {
                            if !bloom.contains(topic.as_bytes()) {
                                should_check_block = false;
                                break;
                            }
                        }
                    }
                }

                if !should_check_block {
                    continue;
                }
            }

            // Get events for this block
            if let Some(events) = self.events_by_block.get(&block_num) {
                for event in events {
                    if filter.matches(event) {
                        results.push(event.clone());

                        // Check limit
                        if let Some(limit) = filter.limit {
                            if results.len() >= limit as usize {
                                return results;
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Get events by contract address
    pub fn get_events_by_address(&self, address: &[u8; 32]) -> Vec<EventLog> {
        let mut results = Vec::new();

        if let Some(indices) = self.address_index.get(address) {
            for index in indices {
                if let Some(events) = self.events_by_block.get(&index.block_number) {
                    if let Some(event) = events.iter().find(|e| {
                        e.transaction_index == index.transaction_index
                            && e.log_index == index.log_index
                    }) {
                        results.push(event.clone());
                    }
                }
            }
        }

        results
    }

    /// Get events by topic
    pub fn get_events_by_topic(&self, topic: &EventTopic) -> Vec<EventLog> {
        let mut results = Vec::new();

        if let Some(indices) = self.topic_index.get(topic) {
            for index in indices {
                if let Some(events) = self.events_by_block.get(&index.block_number) {
                    if let Some(event) = events.iter().find(|e| {
                        e.transaction_index == index.transaction_index
                            && e.log_index == index.log_index
                    }) {
                        results.push(event.clone());
                    }
                }
            }
        }

        results
    }

    /// Get total number of events
    pub fn event_count(&self) -> u64 {
        self.total_events
    }

    /// Get bloom filter for a block
    pub fn get_block_bloom(&self, block_number: u64) -> Option<&EventBloomFilter> {
        self.block_blooms.get(&block_number)
    }

    /// Clear events before a certain block (for pruning)
    pub fn prune_before_block(&mut self, block_number: u64) {
        self.events_by_block.retain(|&k, _| k >= block_number);
        self.block_blooms.retain(|&k, _| k >= block_number);

        // Rebuild address and topic indices
        self.address_index.clear();
        self.topic_index.clear();

        for events in self.events_by_block.values() {
            for event in events {
                let index_entry = EventIndexEntry::from_event(event);

                self.address_index
                    .entry(event.address)
                    .or_insert_with(Vec::new)
                    .push(index_entry.clone());

                for topic in &event.topics {
                    self.topic_index
                        .entry(*topic)
                        .or_insert_with(Vec::new)
                        .push(index_entry.clone());
                }
            }
        }
    }
}

impl Default for EventStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Event errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventError {
    /// Too many topics (max 4)
    TooManyTopics,
    /// Event data too large
    DataTooLarge,
    /// Invalid event format
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_log_creation() {
        let address = [1u8; 32];
        let topics = vec![H256::from_low_u64_be(1), H256::from_low_u64_be(2)];
        let data = vec![1, 2, 3, 4];

        let log = EventLog::new(address, topics.clone(), data.clone(), 100, 0, 0);
        assert!(log.is_ok());

        let log = log.unwrap();
        assert_eq!(log.address, address);
        assert_eq!(log.topics, topics);
        assert_eq!(log.data, data);
        assert_eq!(log.topic_count(), 2);
    }

    #[test]
    fn test_too_many_topics() {
        let address = [1u8; 32];
        let topics = vec![
            H256::from_low_u64_be(1),
            H256::from_low_u64_be(2),
            H256::from_low_u64_be(3),
            H256::from_low_u64_be(4),
            H256::from_low_u64_be(5), // Too many!
        ];

        let result = EventLog::new(address, topics, vec![], 100, 0, 0);
        assert_eq!(result, Err(EventError::TooManyTopics));
    }

    #[test]
    fn test_event_topic_matching() {
        let address = [1u8; 32];
        let topics = vec![H256::from_low_u64_be(1), H256::from_low_u64_be(2)];
        let log = EventLog::new(address, topics, vec![], 100, 0, 0).unwrap();

        // Exact match
        assert!(log.matches_topics(&[Some(H256::from_low_u64_be(1)), Some(H256::from_low_u64_be(2))]));

        // Partial match with wildcard
        assert!(log.matches_topics(&[Some(H256::from_low_u64_be(1)), None]));

        // No match
        assert!(!log.matches_topics(&[Some(H256::from_low_u64_be(3))]));
    }

    #[test]
    fn test_event_store_add_and_query() {
        let mut store = EventStore::new();
        let address = [1u8; 32];
        let topic = H256::from_low_u64_be(42);

        let log = EventLog::new(address, vec![topic], vec![1, 2, 3], 100, 0, 0).unwrap();
        store.add_event(log).unwrap();

        assert_eq!(store.event_count(), 1);

        // Query by address
        let events = store.get_events_by_address(&address);
        assert_eq!(events.len(), 1);

        // Query by topic
        let events = store.get_events_by_topic(&topic);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_event_filter() {
        let mut store = EventStore::new();
        let address1 = [1u8; 32];
        let address2 = [2u8; 32];
        let topic = H256::from_low_u64_be(42);

        // Add events
        store.add_event(EventLog::new(address1, vec![topic], vec![], 100, 0, 0).unwrap()).unwrap();
        store.add_event(EventLog::new(address2, vec![topic], vec![], 100, 0, 1).unwrap()).unwrap();
        store.add_event(EventLog::new(address1, vec![topic], vec![], 101, 0, 0).unwrap()).unwrap();

        // Filter by address
        let filter = EventFilter::new(100, 101).with_address(address1);
        let results = store.query_events(&filter);
        assert_eq!(results.len(), 2);

        // Filter by block range
        let filter = EventFilter::new(100, 100);
        let results = store.query_events(&filter);
        assert_eq!(results.len(), 2);

        // Filter with limit
        let filter = EventFilter::new(100, 101).with_limit(1);
        let results = store.query_events(&filter);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_bloom_filter() {
        let mut bloom = EventBloomFilter::new();

        let item1 = b"hello";
        let item2 = b"world";
        let item3 = b"test";

        bloom.add(item1);
        bloom.add(item2);

        assert!(bloom.contains(item1));
        assert!(bloom.contains(item2));
        // May have false positives, but should not have false negatives
    }

    #[test]
    fn test_bloom_filter_combine() {
        let mut bloom1 = EventBloomFilter::new();
        let mut bloom2 = EventBloomFilter::new();

        bloom1.add(b"hello");
        bloom2.add(b"world");

        bloom1.combine(&bloom2);

        assert!(bloom1.contains(b"hello"));
        assert!(bloom1.contains(b"world"));
    }

    #[test]
    fn test_event_store_pruning() {
        let mut store = EventStore::new();
        let address = [1u8; 32];

        // Add events at different blocks
        store.add_event(EventLog::new(address, vec![], vec![], 100, 0, 0).unwrap()).unwrap();
        store.add_event(EventLog::new(address, vec![], vec![], 200, 0, 0).unwrap()).unwrap();
        store.add_event(EventLog::new(address, vec![], vec![], 300, 0, 0).unwrap()).unwrap();

        assert_eq!(store.event_count(), 3);

        // Prune events before block 200
        store.prune_before_block(200);

        let filter = EventFilter::new(0, 1000);
        let results = store.query_events(&filter);
        assert_eq!(results.len(), 2); // Only blocks 200 and 300 remain
    }
}
