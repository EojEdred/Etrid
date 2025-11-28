//! Execution limits and safeguards to prevent abuse

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use crate::{VMw, MeteringError};

/// ============================================================================
/// EXECUTION LIMIT CONSTANTS
/// ============================================================================

/// Maximum number of instructions per execution
pub const MAX_INSTRUCTIONS: u64 = 100_000_000; // 100 million instructions

/// Maximum call depth (to prevent infinite recursion)
pub const MAX_CALL_DEPTH: u32 = 1024;

/// Maximum stack depth
pub const MAX_STACK_DEPTH: u32 = 1024;

/// Maximum execution time in milliseconds
pub const MAX_EXECUTION_TIME_MS: u64 = 10_000; // 10 seconds

/// Maximum memory size (16 MB)
pub const MAX_MEMORY_SIZE: u64 = 16 * 1024 * 1024;

/// Maximum contract code size (1 MB)
pub const MAX_CODE_SIZE: usize = 1024 * 1024;

/// Maximum log data size per LOG operation
pub const MAX_LOG_DATA_SIZE: usize = 1024 * 1024; // 1 MB

/// Maximum number of logs per execution
pub const MAX_LOGS_PER_EXECUTION: u32 = 100;

/// ============================================================================
/// EXECUTION LIMITS TRACKER
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ExecutionLimits {
    /// Gas limit for this execution
    pub gas_limit: VMw,

    /// Number of instructions executed
    pub instruction_count: u64,

    /// Current call depth
    pub call_depth: u32,

    /// Current stack depth
    pub stack_depth: u32,

    /// Execution start time (simulated, in ms)
    pub start_time_ms: u64,

    /// Current execution time (simulated, in ms)
    pub execution_time_ms: u64,

    /// Number of logs emitted
    pub log_count: u32,

    /// Whether execution has timed out
    pub is_timeout: bool,
}

impl ExecutionLimits {
    /// Create new execution limits with specified gas limit
    pub fn new(gas_limit: VMw) -> Self {
        Self {
            gas_limit,
            instruction_count: 0,
            call_depth: 0,
            stack_depth: 0,
            start_time_ms: 0,
            execution_time_ms: 0,
            log_count: 0,
            is_timeout: false,
        }
    }

    /// Check if execution can continue
    pub fn check_can_execute(&self) -> Result<(), MeteringError> {
        // Check instruction count
        if self.instruction_count >= MAX_INSTRUCTIONS {
            return Err(MeteringError::ExecutionTimeout);
        }

        // Check execution time
        if self.execution_time_ms >= MAX_EXECUTION_TIME_MS {
            return Err(MeteringError::ExecutionTimeout);
        }

        // Check call depth
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MeteringError::CallDepthExceeded);
        }

        // Check stack depth
        if self.stack_depth >= MAX_STACK_DEPTH {
            return Err(MeteringError::StackDepthExceeded);
        }

        Ok(())
    }

    /// Increment instruction count
    pub fn increment_instruction_count(&mut self) {
        self.instruction_count = self.instruction_count.saturating_add(1);

        // Simulate execution time (approximately 1ms per 10,000 instructions)
        if self.instruction_count % 10_000 == 0 {
            self.execution_time_ms = self.execution_time_ms.saturating_add(1);
        }
    }

    /// Increment call depth (when entering a call)
    pub fn increment_call_depth(&mut self) -> Result<(), MeteringError> {
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(MeteringError::CallDepthExceeded);
        }
        self.call_depth = self.call_depth.saturating_add(1);
        Ok(())
    }

    /// Decrement call depth (when returning from a call)
    pub fn decrement_call_depth(&mut self) {
        self.call_depth = self.call_depth.saturating_sub(1);
    }

    /// Increment stack depth
    pub fn increment_stack_depth(&mut self) -> Result<(), MeteringError> {
        if self.stack_depth >= MAX_STACK_DEPTH {
            return Err(MeteringError::StackDepthExceeded);
        }
        self.stack_depth = self.stack_depth.saturating_add(1);
        Ok(())
    }

    /// Decrement stack depth
    pub fn decrement_stack_depth(&mut self) {
        self.stack_depth = self.stack_depth.saturating_sub(1);
    }

    /// Increment log count
    pub fn increment_log_count(&mut self) -> Result<(), MeteringError> {
        if self.log_count >= MAX_LOGS_PER_EXECUTION {
            return Err(MeteringError::StorageIOLimitExceeded);
        }
        self.log_count = self.log_count.saturating_add(1);
        Ok(())
    }

    /// Set execution timeout flag
    pub fn set_timeout(&mut self) {
        self.is_timeout = true;
    }

    /// Update execution time (for testing or external time sources)
    pub fn update_execution_time(&mut self, elapsed_ms: u64) {
        self.execution_time_ms = elapsed_ms;
    }

    /// Get execution progress percentage (based on instruction count)
    pub fn get_progress_percentage(&self) -> u8 {
        if MAX_INSTRUCTIONS == 0 {
            return 0;
        }
        let progress = (self.instruction_count * 100) / MAX_INSTRUCTIONS;
        progress.min(100) as u8
    }

    /// Check if execution is approaching limits (80% threshold)
    pub fn is_approaching_limits(&self) -> bool {
        let instruction_threshold = (self.instruction_count * 100) / MAX_INSTRUCTIONS >= 80;
        let time_threshold = (self.execution_time_ms * 100) / MAX_EXECUTION_TIME_MS >= 80;
        let call_depth_threshold = (self.call_depth * 100) / MAX_CALL_DEPTH >= 80;

        instruction_threshold || time_threshold || call_depth_threshold
    }

    /// Get remaining instruction budget
    pub fn get_remaining_instructions(&self) -> u64 {
        MAX_INSTRUCTIONS.saturating_sub(self.instruction_count)
    }

    /// Get remaining time budget (ms)
    pub fn get_remaining_time_ms(&self) -> u64 {
        MAX_EXECUTION_TIME_MS.saturating_sub(self.execution_time_ms)
    }

    /// Reset limits for new execution
    pub fn reset(&mut self, gas_limit: VMw) {
        self.gas_limit = gas_limit;
        self.instruction_count = 0;
        self.call_depth = 0;
        self.stack_depth = 0;
        self.start_time_ms = 0;
        self.execution_time_ms = 0;
        self.log_count = 0;
        self.is_timeout = false;
    }
}

impl Default for ExecutionLimits {
    fn default() -> Self {
        Self::new(1_000_000) // Default 1M gas
    }
}

/// ============================================================================
/// SAFETY GUARDS
/// ============================================================================

/// Safety guard for code size validation
pub fn validate_code_size(code_size: usize) -> Result<(), MeteringError> {
    if code_size > MAX_CODE_SIZE {
        return Err(MeteringError::InvalidOpcode); // Reusing error type
    }
    Ok(())
}

/// Safety guard for memory size validation
pub fn validate_memory_size(memory_size: u64) -> Result<(), MeteringError> {
    if memory_size > MAX_MEMORY_SIZE {
        return Err(MeteringError::MemoryLimitExceeded);
    }
    Ok(())
}

/// Safety guard for log data size validation
pub fn validate_log_data_size(data_size: usize) -> Result<(), MeteringError> {
    if data_size > MAX_LOG_DATA_SIZE {
        return Err(MeteringError::StorageIOLimitExceeded);
    }
    Ok(())
}

/// Safety guard for call depth validation
pub fn validate_call_depth(depth: u32) -> Result<(), MeteringError> {
    if depth >= MAX_CALL_DEPTH {
        return Err(MeteringError::CallDepthExceeded);
    }
    Ok(())
}

/// Safety guard for stack depth validation
pub fn validate_stack_depth(depth: u32) -> Result<(), MeteringError> {
    if depth >= MAX_STACK_DEPTH {
        return Err(MeteringError::StackDepthExceeded);
    }
    Ok(())
}

/// ============================================================================
/// CIRCUIT BREAKER
/// ============================================================================

/// Circuit breaker for emergency execution halt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum CircuitBreakerState {
    /// Normal operation
    Normal,
    /// Warning: approaching limits
    Warning,
    /// Tripped: execution should halt
    Tripped,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    trip_count: u32,
    warning_threshold: u8, // Percentage
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new() -> Self {
        Self {
            state: CircuitBreakerState::Normal,
            trip_count: 0,
            warning_threshold: 80,
        }
    }

    /// Update circuit breaker state based on execution limits
    pub fn update(&mut self, limits: &ExecutionLimits) {
        if limits.is_timeout {
            self.trip();
            return;
        }

        let progress = limits.get_progress_percentage();

        if progress >= 100 {
            self.trip();
        } else if progress >= self.warning_threshold {
            self.warn();
        } else {
            self.state = CircuitBreakerState::Normal;
        }
    }

    /// Manually warn
    pub fn warn(&mut self) {
        if self.state == CircuitBreakerState::Normal {
            self.state = CircuitBreakerState::Warning;
        }
    }

    /// Manually trip
    pub fn trip(&mut self) {
        if self.state != CircuitBreakerState::Tripped {
            self.trip_count = self.trip_count.saturating_add(1);
        }
        self.state = CircuitBreakerState::Tripped;
    }

    /// Reset circuit breaker
    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Normal;
    }

    /// Check if execution should continue
    pub fn can_execute(&self) -> Result<(), MeteringError> {
        match self.state {
            CircuitBreakerState::Normal | CircuitBreakerState::Warning => Ok(()),
            CircuitBreakerState::Tripped => Err(MeteringError::ExecutionTimeout),
        }
    }

    /// Get current state
    pub fn get_state(&self) -> CircuitBreakerState {
        self.state
    }

    /// Get trip count
    pub fn get_trip_count(&self) -> u32 {
        self.trip_count
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_limits_basic() {
        let limits = ExecutionLimits::new(100_000);
        assert_eq!(limits.instruction_count, 0);
        assert_eq!(limits.call_depth, 0);
        assert!(limits.check_can_execute().is_ok());
    }

    #[test]
    fn test_instruction_count() {
        let mut limits = ExecutionLimits::new(100_000);

        for _ in 0..1000 {
            limits.increment_instruction_count();
        }

        assert_eq!(limits.instruction_count, 1000);
        assert!(limits.check_can_execute().is_ok());
    }

    #[test]
    fn test_instruction_limit_exceeded() {
        let mut limits = ExecutionLimits::new(100_000);
        limits.instruction_count = MAX_INSTRUCTIONS;

        assert_eq!(
            limits.check_can_execute(),
            Err(MeteringError::ExecutionTimeout)
        );
    }

    #[test]
    fn test_call_depth() {
        let mut limits = ExecutionLimits::new(100_000);

        // Increment call depth
        assert!(limits.increment_call_depth().is_ok());
        assert_eq!(limits.call_depth, 1);

        // Decrement call depth
        limits.decrement_call_depth();
        assert_eq!(limits.call_depth, 0);
    }

    #[test]
    fn test_call_depth_exceeded() {
        let mut limits = ExecutionLimits::new(100_000);
        limits.call_depth = MAX_CALL_DEPTH;

        assert_eq!(
            limits.increment_call_depth(),
            Err(MeteringError::CallDepthExceeded)
        );
    }

    #[test]
    fn test_stack_depth() {
        let mut limits = ExecutionLimits::new(100_000);

        assert!(limits.increment_stack_depth().is_ok());
        assert_eq!(limits.stack_depth, 1);

        limits.decrement_stack_depth();
        assert_eq!(limits.stack_depth, 0);
    }

    #[test]
    fn test_stack_depth_exceeded() {
        let mut limits = ExecutionLimits::new(100_000);
        limits.stack_depth = MAX_STACK_DEPTH;

        assert_eq!(
            limits.increment_stack_depth(),
            Err(MeteringError::StackDepthExceeded)
        );
    }

    #[test]
    fn test_log_count() {
        let mut limits = ExecutionLimits::new(100_000);

        for _ in 0..10 {
            assert!(limits.increment_log_count().is_ok());
        }

        assert_eq!(limits.log_count, 10);
    }

    #[test]
    fn test_execution_time() {
        let mut limits = ExecutionLimits::new(100_000);

        // Simulate instruction execution (time increments every 10k instructions)
        for _ in 0..10_000 {
            limits.increment_instruction_count();
        }

        assert_eq!(limits.execution_time_ms, 1);
    }

    #[test]
    fn test_execution_timeout() {
        let mut limits = ExecutionLimits::new(100_000);
        limits.execution_time_ms = MAX_EXECUTION_TIME_MS;

        assert_eq!(
            limits.check_can_execute(),
            Err(MeteringError::ExecutionTimeout)
        );
    }

    #[test]
    fn test_progress_percentage() {
        let mut limits = ExecutionLimits::new(100_000);

        // 0%
        assert_eq!(limits.get_progress_percentage(), 0);

        // 50%
        limits.instruction_count = MAX_INSTRUCTIONS / 2;
        assert_eq!(limits.get_progress_percentage(), 50);

        // 100%
        limits.instruction_count = MAX_INSTRUCTIONS;
        assert_eq!(limits.get_progress_percentage(), 100);
    }

    #[test]
    fn test_approaching_limits() {
        let mut limits = ExecutionLimits::new(100_000);

        // Not approaching
        assert!(!limits.is_approaching_limits());

        // Approaching (80% threshold)
        limits.instruction_count = (MAX_INSTRUCTIONS * 81) / 100;
        assert!(limits.is_approaching_limits());
    }

    #[test]
    fn test_remaining_budget() {
        let mut limits = ExecutionLimits::new(100_000);
        limits.instruction_count = 1000;

        assert_eq!(
            limits.get_remaining_instructions(),
            MAX_INSTRUCTIONS - 1000
        );
    }

    #[test]
    fn test_validate_code_size() {
        assert!(validate_code_size(1000).is_ok());
        assert!(validate_code_size(MAX_CODE_SIZE).is_ok());
        assert!(validate_code_size(MAX_CODE_SIZE + 1).is_err());
    }

    #[test]
    fn test_validate_memory_size() {
        assert!(validate_memory_size(1024).is_ok());
        assert!(validate_memory_size(MAX_MEMORY_SIZE).is_ok());
        assert!(validate_memory_size(MAX_MEMORY_SIZE + 1).is_err());
    }

    #[test]
    fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new();

        // Initial state
        assert_eq!(breaker.get_state(), CircuitBreakerState::Normal);
        assert!(breaker.can_execute().is_ok());

        // Warning
        breaker.warn();
        assert_eq!(breaker.get_state(), CircuitBreakerState::Warning);
        assert!(breaker.can_execute().is_ok());

        // Tripped
        breaker.trip();
        assert_eq!(breaker.get_state(), CircuitBreakerState::Tripped);
        assert!(breaker.can_execute().is_err());
        assert_eq!(breaker.get_trip_count(), 1);

        // Reset
        breaker.reset();
        assert_eq!(breaker.get_state(), CircuitBreakerState::Normal);
    }

    #[test]
    fn test_circuit_breaker_with_limits() {
        let mut breaker = CircuitBreaker::new();
        let mut limits = ExecutionLimits::new(100_000);

        // Normal operation
        breaker.update(&limits);
        assert_eq!(breaker.get_state(), CircuitBreakerState::Normal);

        // Approaching limit
        limits.instruction_count = (MAX_INSTRUCTIONS * 85) / 100;
        breaker.update(&limits);
        assert_eq!(breaker.get_state(), CircuitBreakerState::Warning);

        // Exceeded limit
        limits.instruction_count = MAX_INSTRUCTIONS;
        breaker.update(&limits);
        assert_eq!(breaker.get_state(), CircuitBreakerState::Tripped);
    }

    #[test]
    fn test_reset() {
        let mut limits = ExecutionLimits::new(100_000);

        limits.increment_instruction_count();
        limits.increment_call_depth().unwrap();
        limits.increment_stack_depth().unwrap();

        limits.reset(50_000);

        assert_eq!(limits.gas_limit, 50_000);
        assert_eq!(limits.instruction_count, 0);
        assert_eq!(limits.call_depth, 0);
        assert_eq!(limits.stack_depth, 0);
    }
}
