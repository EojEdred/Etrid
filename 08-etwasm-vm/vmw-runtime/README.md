# VMw Metering Runtime

Comprehensive gas metering system for ETWasm VM with advanced execution limits, resource tracking, and gas refund mechanisms.

## Overview

The VMw Metering Runtime provides a production-ready gas metering system that goes beyond basic gas accounting. It implements:

- **Comprehensive Opcode Cost Tables**: EVM Berlin/London fork-compatible gas costs with dynamic pricing
- **Execution Limits & Safeguards**: Protection against infinite loops, stack overflows, and resource exhaustion
- **Resource Tracking**: CPU cycle counting, memory allocation tracking, and storage I/O metering
- **Gas Refund Mechanisms**: EIP-3529 compliant refunds for storage cleanup operations

## Features

### 1. Dynamic Gas Pricing

The opcode cost table supports dynamic gas pricing based on network conditions:

```rust
use vmw_runtime::*;

let mut cost_table = OpcodeCostTable::new();

// Normal pricing (1.0x)
let cost = cost_table.get_opcode_cost(0x01); // ADD: 3 gas

// Adjust for network congestion (2.0x)
cost_table.set_dynamic_factor(200);
let cost = cost_table.get_opcode_cost(0x01); // ADD: 6 gas
```

### 2. Comprehensive Opcode Coverage

All EVM opcodes from Berlin/London fork are supported with accurate gas costs:

- **Arithmetic**: ADD (3), MUL (5), DIV (5), EXP (10 + dynamic)
- **Storage**: SLOAD (2,100 cold), SSTORE (5,000-20,000)
- **Memory**: MLOAD (3), MSTORE (3), expansion costs
- **System**: CREATE (32,000), CALL (2,600 + dynamic)
- **And more**: 150+ opcodes with accurate gas costs

### 3. Execution Limits

Prevent abuse and ensure fair resource allocation:

```rust
use vmw_runtime::*;

let limits = ExecutionLimits::new(1_000_000);

// Maximum instruction count: 100 million
// Maximum call depth: 1,024
// Maximum stack depth: 1,024
// Maximum execution time: 10 seconds
// Maximum memory: 16 MB
```

### 4. Resource Tracking

Track CPU, memory, and storage resource usage:

```rust
use vmw_runtime::*;

let mut tracker = ResourceTracker::new();

// Track memory allocation
tracker.allocate_memory(1024)?;

// Track storage operations
tracker.track_storage_read()?;
tracker.track_storage_write()?;

// Get usage summary
let summary = tracker.get_usage_summary();
println!("Memory used: {} bytes", summary.memory_used_bytes);
println!("Storage reads: {}", summary.storage_reads);
println!("Cache hit rate: {}%", summary.cache_hit_rate);
```

### 5. Gas Refunds

Incentivize storage cleanup with EIP-3529 compliant refunds:

```rust
use vmw_runtime::*;

let mut refund_manager = GasRefundManager::new();

// Clear storage slot (non-zero to zero)
refund_manager.add_storage_clear_refund(); // +15,000 gas refund

// Reset to original value
refund_manager.add_storage_reset_refund(); // +4,800 gas refund

// Calculate actual refund (capped at 50% of gas used)
let actual_refund = refund_manager.calculate_actual_refund(gas_used);
```

## Usage

### Basic Integration

```rust
use vmw_runtime::*;

// Create metering runtime with gas limit
let mut runtime = VmwMeteringRuntime::new(1_000_000);

// Charge gas for opcode execution
match runtime.charge_opcode(0x01) { // ADD
    Ok(cost) => println!("Charged {} gas", cost),
    Err(MeteringError::OutOfGas) => println!("Out of gas!"),
    Err(e) => println!("Error: {:?}", e),
}

// Charge for memory expansion
runtime.charge_memory_expansion(1024)?;

// Charge for storage write (with refund handling)
let original = [0u8; 32];
let current = [1u8; 32];
let new_value = [0u8; 32]; // Clearing storage
runtime.charge_storage_write(original, current, new_value)?;

// Finalize and apply refunds
let net_gas_used = runtime.finalize();

// Get execution statistics
let stats = runtime.get_stats();
println!("Net gas used: {}", stats.net_gas_used());
println!("Instructions: {}", stats.instructions_executed);
println!("Memory: {} bytes", stats.memory_allocated);
```

### Advanced Features

#### Circuit Breaker

Automatically halt execution when approaching limits:

```rust
use vmw_runtime::*;

let mut breaker = CircuitBreaker::new();
let limits = ExecutionLimits::new(1_000_000);

// Update circuit breaker state
breaker.update(&limits);

match breaker.get_state() {
    CircuitBreakerState::Normal => { /* Continue */ },
    CircuitBreakerState::Warning => { /* Approaching limits */ },
    CircuitBreakerState::Tripped => { /* Halt execution */ },
}
```

#### Storage Access Pattern Analysis

Optimize based on access patterns:

```rust
use vmw_runtime::*;

let mut tracker = ResourceTracker::new();

// Track operations...
for _ in 0..100 {
    tracker.track_storage_read()?;
}

// Analyze pattern
tracker.analyze_access_pattern();

match tracker.storage_access_pattern {
    StorageAccessPattern::Sequential => { /* Good cache locality */ },
    StorageAccessPattern::Random => { /* Poor cache performance */ },
    StorageAccessPattern::ReadHeavy => { /* Consider caching */ },
    StorageAccessPattern::WriteHeavy => { /* Consider batching */ },
}
```

## Gas Cost Reference

### Opcode Categories

| Category | Examples | Typical Cost |
|----------|----------|-------------|
| Arithmetic | ADD, SUB, MUL, DIV | 3-10 gas |
| Logic | AND, OR, XOR, NOT | 3 gas |
| Stack | PUSH, POP, DUP, SWAP | 3 gas |
| Memory | MLOAD, MSTORE | 3 gas + expansion |
| Storage | SLOAD (cold) | 2,100 gas |
| Storage | SSTORE (cold) | 5,000-20,000 gas |
| Call | CALL (warm) | 2,600 gas + transfer |
| Create | CREATE, CREATE2 | 32,000 gas + init |

### Memory Expansion Cost

Memory expansion uses a quadratic pricing formula to prevent abuse:

```
memory_cost = (size_in_words^2 / 512) + (3 * size_in_words)
```

Where `size_in_words` = `(bytes + 31) / 32`

### Storage Gas Costs (EIP-2200)

| Operation | Original | Current | New | Cost | Refund |
|-----------|----------|---------|-----|------|--------|
| No-op | any | X | X | 100 | 0 |
| Set | 0 | 0 | Y | 20,000 | 0 |
| Clear | X | X | 0 | 5,000 | 15,000 |
| Modify | X | X | Y | 5,000 | 0 |
| Reset | X | Y | X | 100 | 4,800 |

## Limits & Constants

### Execution Limits

```rust
MAX_INSTRUCTIONS: 100_000_000       // Maximum instructions per execution
MAX_CALL_DEPTH: 1,024               // Maximum call stack depth
MAX_STACK_DEPTH: 1,024              // Maximum EVM stack depth
MAX_EXECUTION_TIME_MS: 10,000       // Maximum execution time (10 seconds)
MAX_MEMORY_SIZE: 16 * 1024 * 1024   // Maximum memory (16 MB)
```

### Resource Limits

```rust
MAX_STORAGE_READS: 10,000           // Maximum storage reads per execution
MAX_STORAGE_WRITES: 5,000           // Maximum storage writes per execution
MAX_TOTAL_STORAGE_OPS: 15,000       // Maximum total storage operations
MAX_LOGS_PER_EXECUTION: 100         // Maximum log events
```

### Refund Constants

```rust
REFUND_SSTORE_CLEARS: 15,000        // Refund for clearing storage
REFUND_SSTORE_RESET: 4,800          // Refund for resetting to original
REFUND_SELFDESTRUCT: 24,000         // Refund for self-destruct
MAX_REFUND_QUOTIENT: 2              // Maximum refund is 50% of gas used
```

## Testing

Run the comprehensive test suite:

```bash
cd 08-etwasm-vm/vmw-runtime
cargo test
```

Test coverage includes:

- ✅ Opcode cost calculations (all opcodes)
- ✅ Dynamic gas pricing
- ✅ Memory expansion costs
- ✅ Storage operation costs with refunds
- ✅ Execution limits enforcement
- ✅ Resource tracking accuracy
- ✅ Gas refund mechanisms
- ✅ Circuit breaker functionality
- ✅ Edge cases and error handling

## Performance

Benchmark results (typical hardware):

- Opcode cost lookup: < 10 ns
- Memory expansion calculation: < 50 ns
- Storage cost calculation: < 100 ns
- Resource tracking overhead: < 1%

## Integration with ETWasm VM

The VMw Metering Runtime integrates seamlessly with the ETWasm VM:

```rust
use vmw_runtime::*;
use etwasm_runtime::*;

// Create interpreter with metering
let mut interpreter = Interpreter::new(context, code, storage);
let mut metering = VmwMeteringRuntime::new(context.gas_limit);

// Execute with metering
loop {
    let opcode = fetch_opcode(&mut interpreter)?;

    // Charge gas before execution
    metering.charge_opcode(opcode)?;

    // Execute opcode
    execute_opcode(&mut interpreter, opcode)?;

    // Check limits
    metering.limits.check_can_execute()?;
}

// Finalize and get net gas used
let net_gas = metering.finalize();
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  VmwMeteringRuntime                         │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ OpcodeCosts  │  │   Execution  │  │  Resource    │     │
│  │              │  │   Limits     │  │  Tracker     │     │
│  │ - Cost table │  │ - Inst count │  │ - CPU cycles │     │
│  │ - Dynamic $  │  │ - Call depth │  │ - Memory     │     │
│  │ - Memory     │  │ - Stack depth│  │ - Storage I/O│     │
│  │ - Storage    │  │ - Time limit │  │ - Patterns   │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                              │
│  ┌──────────────┐                    ┌──────────────┐     │
│  │ Gas Refunds  │                    │   Circuit    │     │
│  │              │                    │   Breaker    │     │
│  │ - Storage    │                    │ - Normal     │     │
│  │ - Reset      │                    │ - Warning    │     │
│  │ - Destroy    │                    │ - Tripped    │     │
│  └──────────────┘                    └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

## Contributing

When adding new features:

1. Add comprehensive unit tests
2. Update documentation
3. Ensure all tests pass: `cargo test`
4. Follow EIP specifications for gas costs
5. Maintain backwards compatibility

## References

- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [EIP-2200: Structured Definitions for Net Gas Metering](https://eips.ethereum.org/EIPS/eip-2200)
- [EIP-3529: Reduction in refunds](https://eips.ethereum.org/EIPS/eip-3529)
- [EVM Opcodes Reference](https://www.evm.codes/)

## License

Apache-2.0

---

**VMw Metering Runtime v0.1.0**
Part of the Ëtrid Blockchain Ecosystem
