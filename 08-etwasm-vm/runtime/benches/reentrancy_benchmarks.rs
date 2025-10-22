//! Performance Benchmarks for Reentrancy Protection
//!
//! This benchmark suite measures the performance impact of reentrancy protection
//! mechanisms in the ETWasm VM.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use etwasm_runtime::{ExecutionContext, StateLock, host_functions::*, InMemoryStorage};
use sp_core::H256;

// ============================================================================
// BASELINE BENCHMARKS (No reentrancy checks)
// ============================================================================

fn bench_baseline_call_stack_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("call_stack_operations");

    // Benchmark: Enter call (single)
    group.bench_function("enter_call_single", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            let target = [1u8; 32];
            black_box(ctx.enter_call(target)).ok();
        });
    });

    // Benchmark: Enter + Exit call (single)
    group.bench_function("enter_exit_call_single", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            let target = [1u8; 32];
            ctx.enter_call(target).ok();
            black_box(ctx.exit_call(&target));
        });
    });

    // Benchmark: Check if in call stack
    group.bench_function("is_in_call_stack", |b| {
        let mut ctx = ExecutionContext::default();
        let target = [1u8; 32];
        ctx.enter_call(target).ok();

        b.iter(|| {
            black_box(ctx.is_in_call_stack(&target));
        });
    });

    group.finish();
}

// ============================================================================
// DEPTH SCALING BENCHMARKS
// ============================================================================

fn bench_call_depth_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("call_depth_scaling");

    for depth in [1, 2, 5, 10].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            b.iter(|| {
                let mut ctx = ExecutionContext::default();
                // Build call chain
                for i in 0..depth {
                    let target = [i as u8; 32];
                    ctx.enter_call(target).ok();
                }
                // Cleanup
                for i in (0..depth).rev() {
                    let target = [i as u8; 32];
                    ctx.exit_call(&target);
                }
                black_box(ctx);
            });
        });
    }

    group.finish();
}

// ============================================================================
// STATE LOCK BENCHMARKS
// ============================================================================

fn bench_state_lock_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_lock_operations");

    // Benchmark: Lock (single)
    group.bench_function("lock_single", |b| {
        b.iter(|| {
            let mut state_lock = StateLock::new();
            let account = [1u8; 32];
            black_box(state_lock.lock(&account));
        });
    });

    // Benchmark: Lock + Unlock (single)
    group.bench_function("lock_unlock_single", |b| {
        b.iter(|| {
            let mut state_lock = StateLock::new();
            let account = [1u8; 32];
            state_lock.lock(&account);
            black_box(state_lock.unlock(&account));
        });
    });

    // Benchmark: Check if locked
    group.bench_function("is_locked_check", |b| {
        let mut state_lock = StateLock::new();
        let account = [1u8; 32];
        state_lock.lock(&account);

        b.iter(|| {
            black_box(state_lock.is_locked(&account));
        });
    });

    // Benchmark: Nested locking (3 levels)
    group.bench_function("nested_locking_3x", |b| {
        b.iter(|| {
            let mut state_lock = StateLock::new();
            let account = [1u8; 32];
            state_lock.lock(&account);
            state_lock.lock(&account);
            state_lock.lock(&account);
            state_lock.unlock(&account);
            state_lock.unlock(&account);
            black_box(state_lock.unlock(&account));
        });
    });

    group.finish();
}

// ============================================================================
// HOST FUNCTION OVERHEAD BENCHMARKS
// ============================================================================

fn bench_host_function_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("host_function_overhead");

    // Benchmark: host_transfer (with lock checks)
    group.bench_function("host_transfer_with_checks", |b| {
        let ctx = ExecutionContext::default();
        let state_lock = StateLock::new();
        let recipient = [1u8; 32];

        b.iter(|| {
            black_box(host_transfer(&ctx, &state_lock, recipient, 1000)).ok();
        });
    });

    // Benchmark: host_sload (read - no lock check)
    group.bench_function("host_sload_read", |b| {
        let ctx = ExecutionContext::default();
        let storage = InMemoryStorage::default();
        let key = H256::zero();

        b.iter(|| {
            black_box(host_sload(&ctx, &storage, key));
        });
    });

    // Benchmark: host_sstore (write - with lock check)
    group.bench_function("host_sstore_with_checks", |b| {
        let ctx = ExecutionContext::default();
        let state_lock = StateLock::new();
        let mut storage = InMemoryStorage::default();

        b.iter(|| {
            black_box(host_sstore(
                &ctx,
                &state_lock,
                &mut storage,
                H256::zero(),
                H256::from_low_u64_be(42)
            )).ok();
        });
    });

    group.finish();
}

// ============================================================================
// REENTRANCY DETECTION BENCHMARKS
// ============================================================================

fn bench_reentrancy_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("reentrancy_detection");

    // Benchmark: Direct reentrancy detection (A -> A)
    group.bench_function("detect_direct_reentrancy", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            let target = [1u8; 32];
            ctx.enter_call(target).ok();
            // This should be detected as reentrancy
            black_box(ctx.enter_call(target));
        });
    });

    // Benchmark: Indirect reentrancy detection (A -> B -> A)
    group.bench_function("detect_indirect_reentrancy", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            let target_a = [1u8; 32];
            let target_b = [2u8; 32];
            ctx.enter_call(target_a).ok();
            ctx.enter_call(target_b).ok();
            // This should be detected as reentrancy
            black_box(ctx.enter_call(target_a));
        });
    });

    // Benchmark: Max depth check
    group.bench_function("detect_max_depth_exceeded", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            ctx.max_depth = 5;
            // Fill to max depth
            for i in 0..5 {
                ctx.enter_call([i as u8; 32]).ok();
            }
            // This should exceed max depth
            black_box(ctx.enter_call([6u8; 32]));
        });
    });

    group.finish();
}

// ============================================================================
// REALISTIC SCENARIO BENCHMARKS
// ============================================================================

fn bench_realistic_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_scenarios");

    // Benchmark: Simple external call (A -> B)
    group.bench_function("simple_external_call", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            let mut state_lock = StateLock::new();
            let contract_a = [1u8; 32];
            let contract_b = [2u8; 32];

            // A calls B
            ctx.enter_call(contract_a).ok();
            state_lock.lock(&contract_a);

            ctx.enter_call(contract_b).ok();
            ctx.exit_call(&contract_b);

            state_lock.unlock(&contract_a);
            ctx.exit_call(&contract_a);

            black_box((ctx, state_lock));
        });
    });

    // Benchmark: Complex call chain (A -> B -> C)
    group.bench_function("complex_call_chain", |b| {
        b.iter(|| {
            let mut ctx = ExecutionContext::default();
            let mut state_lock = StateLock::new();
            let contracts = [[1u8; 32], [2u8; 32], [3u8; 32]];

            // Build call chain
            for (i, contract) in contracts.iter().enumerate() {
                ctx.enter_call(*contract).ok();
                if i < contracts.len() - 1 {
                    state_lock.lock(contract);
                }
            }

            // Unwind call chain
            for (i, contract) in contracts.iter().enumerate().rev() {
                ctx.exit_call(contract);
                if i > 0 {
                    state_lock.unlock(&contracts[i - 1]);
                }
            }

            black_box((ctx, state_lock));
        });
    });

    // Benchmark: Transfer with reentrancy check
    group.bench_function("transfer_with_full_protection", |b| {
        let ctx = ExecutionContext::default();
        let state_lock = StateLock::new();
        let recipient = [1u8; 32];

        b.iter(|| {
            // Full reentrancy protection overhead
            black_box(host_transfer(&ctx, &state_lock, recipient, 1000)).ok();
        });
    });

    // Benchmark: Storage write with reentrancy check
    group.bench_function("storage_write_with_full_protection", |b| {
        let ctx = ExecutionContext::default();
        let state_lock = StateLock::new();
        let mut storage = InMemoryStorage::default();

        b.iter(|| {
            // Full reentrancy protection overhead
            black_box(host_sstore(
                &ctx,
                &state_lock,
                &mut storage,
                H256::from_low_u64_be(1),
                H256::from_low_u64_be(42)
            )).ok();
        });
    });

    group.finish();
}

// ============================================================================
// CRITERION CONFIGURATION
// ============================================================================

criterion_group!(
    benches,
    bench_baseline_call_stack_operations,
    bench_call_depth_scaling,
    bench_state_lock_operations,
    bench_host_function_overhead,
    bench_reentrancy_detection,
    bench_realistic_scenarios,
);

criterion_main!(benches);
