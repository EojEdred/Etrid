//! Resource tracking for CPU cycles, memory allocation, and storage I/O

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use crate::{MeteringError, MAX_MEMORY_SIZE};

/// ============================================================================
/// RESOURCE LIMITS
/// ============================================================================

/// Maximum storage reads per execution
pub const MAX_STORAGE_READS: u32 = 10_000;

/// Maximum storage writes per execution
pub const MAX_STORAGE_WRITES: u32 = 5_000;

/// Maximum total storage I/O operations
pub const MAX_TOTAL_STORAGE_OPS: u32 = 15_000;

/// Memory page size (64 KB - WASM standard)
pub const MEMORY_PAGE_SIZE: u64 = 65536;

/// Maximum memory pages
pub const MAX_MEMORY_PAGES: u32 = (MAX_MEMORY_SIZE / MEMORY_PAGE_SIZE) as u32;

/// ============================================================================
/// RESOURCE TRACKER
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ResourceTracker {
    /// Total CPU cycles consumed (simulated)
    pub cpu_cycles: u64,

    /// Total memory allocated (bytes)
    pub memory_allocated: u64,

    /// Peak memory usage (bytes)
    pub peak_memory: u64,

    /// Number of memory allocations
    pub memory_alloc_count: u32,

    /// Number of storage read operations
    pub storage_reads: u32,

    /// Number of storage write operations
    pub storage_writes: u32,

    /// Total bytes read from storage
    pub storage_bytes_read: u64,

    /// Total bytes written to storage
    pub storage_bytes_written: u64,

    /// Number of cold storage accesses (not in cache)
    pub cold_storage_access: u32,

    /// Number of warm storage accesses (in cache)
    pub warm_storage_access: u32,

    /// Storage access pattern (for optimization hints)
    pub storage_access_pattern: StorageAccessPattern,
}

impl ResourceTracker {
    /// Create new resource tracker
    pub fn new() -> Self {
        Self {
            cpu_cycles: 0,
            memory_allocated: 0,
            peak_memory: 0,
            memory_alloc_count: 0,
            storage_reads: 0,
            storage_writes: 0,
            storage_bytes_read: 0,
            storage_bytes_written: 0,
            cold_storage_access: 0,
            warm_storage_access: 0,
            storage_access_pattern: StorageAccessPattern::Sequential,
        }
    }

    /// Track CPU cycle consumption
    pub fn consume_cpu_cycles(&mut self, cycles: u64) {
        self.cpu_cycles = self.cpu_cycles.saturating_add(cycles);
    }

    /// Allocate memory
    pub fn allocate_memory(&mut self, size: u64) -> Result<(), MeteringError> {
        let new_size = self.memory_allocated.saturating_add(size);

        if new_size > MAX_MEMORY_SIZE {
            return Err(MeteringError::MemoryLimitExceeded);
        }

        self.memory_allocated = new_size;
        self.memory_alloc_count = self.memory_alloc_count.saturating_add(1);

        // Update peak memory
        if new_size > self.peak_memory {
            self.peak_memory = new_size;
        }

        Ok(())
    }

    /// Deallocate memory
    pub fn deallocate_memory(&mut self, size: u64) {
        self.memory_allocated = self.memory_allocated.saturating_sub(size);
    }

    /// Track storage read
    pub fn track_storage_read(&mut self) -> Result<(), MeteringError> {
        if self.storage_reads >= MAX_STORAGE_READS {
            return Err(MeteringError::StorageIOLimitExceeded);
        }

        if self.get_total_storage_ops() >= MAX_TOTAL_STORAGE_OPS {
            return Err(MeteringError::StorageIOLimitExceeded);
        }

        self.storage_reads = self.storage_reads.saturating_add(1);
        self.storage_bytes_read = self.storage_bytes_read.saturating_add(32); // 32 bytes per slot
        Ok(())
    }

    /// Track storage write
    pub fn track_storage_write(&mut self) -> Result<(), MeteringError> {
        if self.storage_writes >= MAX_STORAGE_WRITES {
            return Err(MeteringError::StorageIOLimitExceeded);
        }

        if self.get_total_storage_ops() >= MAX_TOTAL_STORAGE_OPS {
            return Err(MeteringError::StorageIOLimitExceeded);
        }

        self.storage_writes = self.storage_writes.saturating_add(1);
        self.storage_bytes_written = self.storage_bytes_written.saturating_add(32); // 32 bytes per slot
        Ok(())
    }

    /// Track cold storage access
    pub fn track_cold_access(&mut self) {
        self.cold_storage_access = self.cold_storage_access.saturating_add(1);
    }

    /// Track warm storage access
    pub fn track_warm_access(&mut self) {
        self.warm_storage_access = self.warm_storage_access.saturating_add(1);
    }

    /// Get total storage operations
    pub fn get_total_storage_ops(&self) -> u32 {
        self.storage_reads.saturating_add(self.storage_writes)
    }

    /// Get storage read percentage
    pub fn get_storage_read_percentage(&self) -> u8 {
        if MAX_STORAGE_READS == 0 {
            return 0;
        }
        ((self.storage_reads as u64 * 100) / MAX_STORAGE_READS as u64).min(100) as u8
    }

    /// Get storage write percentage
    pub fn get_storage_write_percentage(&self) -> u8 {
        if MAX_STORAGE_WRITES == 0 {
            return 0;
        }
        ((self.storage_writes as u64 * 100) / MAX_STORAGE_WRITES as u64).min(100) as u8
    }

    /// Get memory usage percentage
    pub fn get_memory_percentage(&self) -> u8 {
        if MAX_MEMORY_SIZE == 0 {
            return 0;
        }
        ((self.memory_allocated as u64 * 100) / MAX_MEMORY_SIZE as u64).min(100) as u8
    }

    /// Get memory pages used
    pub fn get_memory_pages_used(&self) -> u32 {
        ((self.memory_allocated + MEMORY_PAGE_SIZE - 1) / MEMORY_PAGE_SIZE) as u32
    }

    /// Get storage cache hit rate (percentage)
    pub fn get_cache_hit_rate(&self) -> u8 {
        let total_access = self.cold_storage_access.saturating_add(self.warm_storage_access);
        if total_access == 0 {
            return 0;
        }
        ((self.warm_storage_access as u64 * 100) / total_access as u64) as u8
    }

    /// Analyze storage access pattern
    pub fn analyze_access_pattern(&mut self) {
        let read_write_ratio = if self.storage_writes == 0 {
            100
        } else {
            (self.storage_reads as u64 * 100) / self.storage_writes as u64
        };

        self.storage_access_pattern = if read_write_ratio > 80 {
            StorageAccessPattern::ReadHeavy
        } else if read_write_ratio < 20 {
            StorageAccessPattern::WriteHeavy
        } else if self.get_cache_hit_rate() > 70 {
            StorageAccessPattern::Sequential
        } else {
            StorageAccessPattern::Random
        };
    }

    /// Get resource usage summary
    pub fn get_usage_summary(&self) -> ResourceUsageSummary {
        ResourceUsageSummary {
            cpu_cycles: self.cpu_cycles,
            memory_used_bytes: self.memory_allocated,
            peak_memory_bytes: self.peak_memory,
            memory_percentage: self.get_memory_percentage(),
            storage_reads: self.storage_reads,
            storage_writes: self.storage_writes,
            total_storage_ops: self.get_total_storage_ops(),
            storage_bytes_read: self.storage_bytes_read,
            storage_bytes_written: self.storage_bytes_written,
            cache_hit_rate: self.get_cache_hit_rate(),
            access_pattern: self.storage_access_pattern,
        }
    }

    /// Check if resource usage is within safe limits
    pub fn check_safe_limits(&self) -> Result<(), MeteringError> {
        if self.memory_allocated > MAX_MEMORY_SIZE {
            return Err(MeteringError::MemoryLimitExceeded);
        }

        if self.storage_reads >= MAX_STORAGE_READS {
            return Err(MeteringError::StorageIOLimitExceeded);
        }

        if self.storage_writes >= MAX_STORAGE_WRITES {
            return Err(MeteringError::StorageIOLimitExceeded);
        }

        Ok(())
    }

    /// Reset tracker for new execution
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for ResourceTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// ============================================================================
/// STORAGE ACCESS PATTERN
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum StorageAccessPattern {
    /// Sequential access (good cache performance)
    Sequential,
    /// Random access (poor cache performance)
    Random,
    /// Read-heavy workload
    ReadHeavy,
    /// Write-heavy workload
    WriteHeavy,
}

impl StorageAccessPattern {
    pub fn name(&self) -> &'static str {
        match self {
            StorageAccessPattern::Sequential => "Sequential",
            StorageAccessPattern::Random => "Random",
            StorageAccessPattern::ReadHeavy => "Read-Heavy",
            StorageAccessPattern::WriteHeavy => "Write-Heavy",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            StorageAccessPattern::Sequential => "Sequential storage access with good cache locality",
            StorageAccessPattern::Random => "Random storage access with poor cache locality",
            StorageAccessPattern::ReadHeavy => "Read-heavy workload with minimal writes",
            StorageAccessPattern::WriteHeavy => "Write-heavy workload with many modifications",
        }
    }
}

/// ============================================================================
/// RESOURCE USAGE SUMMARY
/// ============================================================================

#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct ResourceUsageSummary {
    pub cpu_cycles: u64,
    pub memory_used_bytes: u64,
    pub peak_memory_bytes: u64,
    pub memory_percentage: u8,
    pub storage_reads: u32,
    pub storage_writes: u32,
    pub total_storage_ops: u32,
    pub storage_bytes_read: u64,
    pub storage_bytes_written: u64,
    pub cache_hit_rate: u8,
    pub access_pattern: StorageAccessPattern,
}

impl ResourceUsageSummary {
    /// Check if resources are heavily utilized (>80%)
    pub fn is_heavily_utilized(&self) -> bool {
        self.memory_percentage > 80
            || (self.storage_reads * 100 / MAX_STORAGE_READS) > 80
            || (self.storage_writes * 100 / MAX_STORAGE_WRITES) > 80
    }

    /// Get optimization suggestions
    pub fn get_optimization_hints(&self) -> Vec<&'static str> {
        let mut hints = Vec::new();

        if self.memory_percentage > 80 {
            hints.push("Consider reducing memory usage or increasing memory limit");
        }

        if self.cache_hit_rate < 50 {
            hints.push("Poor cache performance - consider sequential access patterns");
        }

        if matches!(self.access_pattern, StorageAccessPattern::WriteHeavy) {
            hints.push("Write-heavy workload - consider batching writes");
        }

        if self.storage_reads > 1000 {
            hints.push("High number of storage reads - consider caching");
        }

        hints
    }
}

/// ============================================================================
/// CPU CYCLE ESTIMATOR
/// ============================================================================

/// Estimate CPU cycles for common operations
pub struct CpuCycleEstimator;

impl CpuCycleEstimator {
    /// Estimate cycles for arithmetic operation
    pub fn arithmetic_op() -> u64 {
        10
    }

    /// Estimate cycles for memory load
    pub fn memory_load() -> u64 {
        50
    }

    /// Estimate cycles for memory store
    pub fn memory_store() -> u64 {
        100
    }

    /// Estimate cycles for storage read
    pub fn storage_read() -> u64 {
        10_000
    }

    /// Estimate cycles for storage write
    pub fn storage_write() -> u64 {
        50_000
    }

    /// Estimate cycles for hash operation
    pub fn hash_operation(bytes: usize) -> u64 {
        1000 + (bytes as u64 * 10)
    }

    /// Estimate cycles for signature verification
    pub fn signature_verification() -> u64 {
        100_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_tracker_new() {
        let tracker = ResourceTracker::new();
        assert_eq!(tracker.memory_allocated, 0);
        assert_eq!(tracker.storage_reads, 0);
        assert_eq!(tracker.storage_writes, 0);
    }

    #[test]
    fn test_memory_allocation() {
        let mut tracker = ResourceTracker::new();

        assert!(tracker.allocate_memory(1024).is_ok());
        assert_eq!(tracker.memory_allocated, 1024);
        assert_eq!(tracker.peak_memory, 1024);
        assert_eq!(tracker.memory_alloc_count, 1);

        // Allocate more
        assert!(tracker.allocate_memory(2048).is_ok());
        assert_eq!(tracker.memory_allocated, 3072);
        assert_eq!(tracker.peak_memory, 3072);
    }

    #[test]
    fn test_memory_deallocation() {
        let mut tracker = ResourceTracker::new();

        tracker.allocate_memory(1024).unwrap();
        tracker.deallocate_memory(512);

        assert_eq!(tracker.memory_allocated, 512);
        assert_eq!(tracker.peak_memory, 1024); // Peak remains
    }

    #[test]
    fn test_memory_limit() {
        let mut tracker = ResourceTracker::new();

        let result = tracker.allocate_memory(MAX_MEMORY_SIZE + 1);
        assert_eq!(result, Err(MeteringError::MemoryLimitExceeded));
    }

    #[test]
    fn test_storage_operations() {
        let mut tracker = ResourceTracker::new();

        // Track reads
        assert!(tracker.track_storage_read().is_ok());
        assert_eq!(tracker.storage_reads, 1);
        assert_eq!(tracker.storage_bytes_read, 32);

        // Track writes
        assert!(tracker.track_storage_write().is_ok());
        assert_eq!(tracker.storage_writes, 1);
        assert_eq!(tracker.storage_bytes_written, 32);

        // Total ops
        assert_eq!(tracker.get_total_storage_ops(), 2);
    }

    #[test]
    fn test_storage_read_limit() {
        let mut tracker = ResourceTracker::new();
        tracker.storage_reads = MAX_STORAGE_READS;

        let result = tracker.track_storage_read();
        assert_eq!(result, Err(MeteringError::StorageIOLimitExceeded));
    }

    #[test]
    fn test_storage_write_limit() {
        let mut tracker = ResourceTracker::new();
        tracker.storage_writes = MAX_STORAGE_WRITES;

        let result = tracker.track_storage_write();
        assert_eq!(result, Err(MeteringError::StorageIOLimitExceeded));
    }

    #[test]
    fn test_cache_tracking() {
        let mut tracker = ResourceTracker::new();

        // Track accesses
        tracker.track_cold_access();
        tracker.track_cold_access();
        tracker.track_warm_access();
        tracker.track_warm_access();
        tracker.track_warm_access();

        // Cache hit rate: 3/5 = 60%
        assert_eq!(tracker.get_cache_hit_rate(), 60);
    }

    #[test]
    fn test_memory_percentage() {
        let mut tracker = ResourceTracker::new();

        tracker.allocate_memory(MAX_MEMORY_SIZE / 2).unwrap();
        assert_eq!(tracker.get_memory_percentage(), 50);

        tracker.allocate_memory(MAX_MEMORY_SIZE / 4).unwrap();
        assert_eq!(tracker.get_memory_percentage(), 75);
    }

    #[test]
    fn test_memory_pages() {
        let mut tracker = ResourceTracker::new();

        tracker.allocate_memory(MEMORY_PAGE_SIZE).unwrap();
        assert_eq!(tracker.get_memory_pages_used(), 1);

        tracker.allocate_memory(MEMORY_PAGE_SIZE * 2).unwrap();
        assert_eq!(tracker.get_memory_pages_used(), 3);
    }

    #[test]
    fn test_access_pattern_analysis() {
        let mut tracker = ResourceTracker::new();

        // Read-heavy pattern
        for _ in 0..100 {
            tracker.track_storage_read().unwrap();
        }
        for _ in 0..10 {
            tracker.track_storage_write().unwrap();
        }

        tracker.analyze_access_pattern();
        assert_eq!(tracker.storage_access_pattern, StorageAccessPattern::ReadHeavy);
    }

    #[test]
    fn test_cpu_cycle_tracking() {
        let mut tracker = ResourceTracker::new();

        tracker.consume_cpu_cycles(1000);
        assert_eq!(tracker.cpu_cycles, 1000);

        tracker.consume_cpu_cycles(500);
        assert_eq!(tracker.cpu_cycles, 1500);
    }

    #[test]
    fn test_usage_summary() {
        let mut tracker = ResourceTracker::new();

        tracker.allocate_memory(1024).unwrap();
        tracker.track_storage_read().unwrap();
        tracker.track_storage_write().unwrap();
        tracker.consume_cpu_cycles(5000);

        let summary = tracker.get_usage_summary();
        assert_eq!(summary.cpu_cycles, 5000);
        assert_eq!(summary.memory_used_bytes, 1024);
        assert_eq!(summary.storage_reads, 1);
        assert_eq!(summary.storage_writes, 1);
    }

    #[test]
    fn test_safe_limits_check() {
        let mut tracker = ResourceTracker::new();

        assert!(tracker.check_safe_limits().is_ok());

        tracker.memory_allocated = MAX_MEMORY_SIZE + 1;
        assert!(tracker.check_safe_limits().is_err());
    }

    #[test]
    fn test_reset() {
        let mut tracker = ResourceTracker::new();

        tracker.allocate_memory(1024).unwrap();
        tracker.track_storage_read().unwrap();
        tracker.consume_cpu_cycles(1000);

        tracker.reset();

        assert_eq!(tracker.memory_allocated, 0);
        assert_eq!(tracker.storage_reads, 0);
        assert_eq!(tracker.cpu_cycles, 0);
    }

    #[test]
    fn test_cpu_cycle_estimator() {
        assert_eq!(CpuCycleEstimator::arithmetic_op(), 10);
        assert_eq!(CpuCycleEstimator::memory_load(), 50);
        assert_eq!(CpuCycleEstimator::storage_read(), 10_000);
        assert!(CpuCycleEstimator::hash_operation(100) > 1000);
    }
}
