//! Routing Performance Benchmarks
//!
//! Benchmarks for Lightning-Bloc routing algorithm performance
//! Tests pathfinding on networks of various sizes (10, 100, 1000 nodes)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use etrid_lightning_bloc::routing::{NetworkGraph, Router, ChannelEdge};
use std::collections::HashMap;

/// Create a network with specified number of nodes in a ring topology
fn create_ring_network(node_count: usize) -> NetworkGraph {
    let mut graph = NetworkGraph::new();

    for i in 0..node_count {
        let from = format!("N{}", i);
        let to = format!("N{}", (i + 1) % node_count);

        graph.add_channel(ChannelEdge {
            channel_id: format!("CH{}", i),
            from_node: from,
            to_node: to,
            capacity: 1_000_000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1_000_000,
            time_lock_delta: 40,
        }).unwrap();
    }

    graph
}

/// Create a network with specified number of nodes in a mesh topology
fn create_mesh_network(node_count: usize) -> NetworkGraph {
    let mut graph = NetworkGraph::new();

    // Create connections between nearby nodes (mesh with local connectivity)
    for i in 0..node_count {
        for j in (i + 1)..(i + 5).min(node_count) {
            let from = format!("N{}", i);
            let to = format!("N{}", j);

            graph.add_channel(ChannelEdge {
                channel_id: format!("CH_{}_{}", i, j),
                from_node: from,
                to_node: to,
                capacity: 1_000_000,
                base_fee: 1,
                fee_rate: 100,
                min_htlc: 1,
                max_htlc: 1_000_000,
                time_lock_delta: 40,
            }).unwrap();
        }
    }

    graph
}

/// Create a hub-and-spoke network
fn create_hub_spoke_network(spoke_count: usize) -> NetworkGraph {
    let mut graph = NetworkGraph::new();

    // Connect all spokes to central hub
    for i in 0..spoke_count {
        let spoke = format!("S{}", i);

        graph.add_channel(ChannelEdge {
            channel_id: format!("CH_hub_{}", i),
            from_node: "HUB".to_string(),
            to_node: spoke,
            capacity: 1_000_000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 1_000_000,
            time_lock_delta: 40,
        }).unwrap();
    }

    graph
}

/// Benchmark routing on 10-node network
fn bench_routing_10_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("routing_10_nodes");

    // Ring topology
    let ring_graph = create_ring_network(10);
    let ring_router = Router::new(ring_graph);

    group.bench_function("ring_topology", |b| {
        b.iter(|| {
            black_box(ring_router.find_route(
                &"N0".to_string(),
                &"N5".to_string(),
                black_box(1000)
            ))
        })
    });

    // Mesh topology
    let mesh_graph = create_mesh_network(10);
    let mesh_router = Router::new(mesh_graph);

    group.bench_function("mesh_topology", |b| {
        b.iter(|| {
            black_box(mesh_router.find_route(
                &"N0".to_string(),
                &"N9".to_string(),
                black_box(1000)
            ))
        })
    });

    // Hub-and-spoke topology
    let hub_graph = create_hub_spoke_network(10);
    let hub_router = Router::new(hub_graph);

    group.bench_function("hub_spoke_topology", |b| {
        b.iter(|| {
            black_box(hub_router.find_route(
                &"S0".to_string(),
                &"S9".to_string(),
                black_box(1000)
            ))
        })
    });

    group.finish();
}

/// Benchmark routing on 100-node network
fn bench_routing_100_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("routing_100_nodes");
    group.sample_size(50); // Reduce sample size for larger networks

    // Ring topology
    let ring_graph = create_ring_network(100);
    let ring_router = Router::new(ring_graph);

    group.bench_function("ring_topology", |b| {
        b.iter(|| {
            black_box(ring_router.find_route(
                &"N0".to_string(),
                &"N50".to_string(),
                black_box(1000)
            ))
        })
    });

    // Mesh topology
    let mesh_graph = create_mesh_network(100);
    let mesh_router = Router::new(mesh_graph);

    group.bench_function("mesh_topology", |b| {
        b.iter(|| {
            black_box(mesh_router.find_route(
                &"N0".to_string(),
                &"N99".to_string(),
                black_box(1000)
            ))
        })
    });

    // Hub-and-spoke topology
    let hub_graph = create_hub_spoke_network(100);
    let hub_router = Router::new(hub_graph);

    group.bench_function("hub_spoke_topology", |b| {
        b.iter(|| {
            black_box(hub_router.find_route(
                &"S0".to_string(),
                &"S99".to_string(),
                black_box(1000)
            ))
        })
    });

    group.finish();
}

/// Benchmark routing on 1000-node network
fn bench_routing_1000_nodes(c: &mut Criterion) {
    let mut group = c.benchmark_group("routing_1000_nodes");
    group.sample_size(20); // Further reduce sample size for very large networks

    // Ring topology
    let ring_graph = create_ring_network(1000);
    let ring_router = Router::new(ring_graph);

    group.bench_function("ring_topology", |b| {
        b.iter(|| {
            black_box(ring_router.find_route(
                &"N0".to_string(),
                &"N500".to_string(),
                black_box(1000)
            ))
        })
    });

    // Mesh topology (limited connectivity to avoid exponential blowup)
    let mesh_graph = create_mesh_network(1000);
    let mesh_router = Router::new(mesh_graph);

    group.bench_function("mesh_topology", |b| {
        b.iter(|| {
            black_box(mesh_router.find_route(
                &"N0".to_string(),
                &"N999".to_string(),
                black_box(1000)
            ))
        })
    });

    // Hub-and-spoke topology
    let hub_graph = create_hub_spoke_network(1000);
    let hub_router = Router::new(hub_graph);

    group.bench_function("hub_spoke_topology", |b| {
        b.iter(|| {
            black_box(hub_router.find_route(
                &"S0".to_string(),
                &"S999".to_string(),
                black_box(1000)
            ))
        })
    });

    group.finish();
}

/// Benchmark multi-route finding
fn bench_multi_route_finding(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_route_finding");

    let graph = create_mesh_network(50);
    let router = Router::new(graph);

    for num_routes in [1, 2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(num_routes),
            num_routes,
            |b, &num_routes| {
                b.iter(|| {
                    black_box(router.find_routes(
                        &"N0".to_string(),
                        &"N49".to_string(),
                        black_box(1000),
                        num_routes
                    ))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark capacity updates
fn bench_capacity_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("capacity_updates");

    group.bench_function("update_single_channel", |b| {
        b.iter_batched(
            || create_mesh_network(100),
            |mut graph| {
                black_box(graph.update_capacity(&"CH_0_1".to_string(), 500_000))
            },
            criterion::BatchSize::SmallInput
        )
    });

    group.bench_function("update_multiple_channels", |b| {
        b.iter_batched(
            || create_mesh_network(100),
            |mut graph| {
                for i in 0..10 {
                    let channel_id = format!("CH_{}_{}", i, i + 1);
                    let _ = graph.update_capacity(&channel_id, 500_000);
                }
            },
            criterion::BatchSize::SmallInput
        )
    });

    group.finish();
}

/// Benchmark network graph operations
fn bench_graph_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_operations");

    group.bench_function("add_channel", |b| {
        b.iter_batched(
            || NetworkGraph::new(),
            |mut graph| {
                for i in 0..100 {
                    let from = format!("N{}", i);
                    let to = format!("N{}", i + 1);
                    let _ = graph.add_channel(ChannelEdge {
                        channel_id: format!("CH{}", i),
                        from_node: from,
                        to_node: to,
                        capacity: 1_000_000,
                        base_fee: 1,
                        fee_rate: 100,
                        min_htlc: 1,
                        max_htlc: 1_000_000,
                        time_lock_delta: 40,
                    });
                }
            },
            criterion::BatchSize::SmallInput
        )
    });

    group.bench_function("remove_channel", |b| {
        b.iter_batched(
            || {
                let mut graph = NetworkGraph::new();
                for i in 0..100 {
                    let from = format!("N{}", i);
                    let to = format!("N{}", i + 1);
                    graph.add_channel(ChannelEdge {
                        channel_id: format!("CH{}", i),
                        from_node: from,
                        to_node: to,
                        capacity: 1_000_000,
                        base_fee: 1,
                        fee_rate: 100,
                        min_htlc: 1,
                        max_htlc: 1_000_000,
                        time_lock_delta: 40,
                    }).unwrap();
                }
                graph
            },
            |mut graph| {
                for i in 0..50 {
                    let channel_id = format!("CH{}", i);
                    let _ = graph.remove_channel(&channel_id);
                }
            },
            criterion::BatchSize::SmallInput
        )
    });

    group.bench_function("get_neighbors", |b| {
        let graph = create_mesh_network(100);
        b.iter(|| {
            black_box(graph.neighbors(&"N50".to_string()))
        })
    });

    group.bench_function("total_capacity", |b| {
        let graph = create_mesh_network(100);
        b.iter(|| {
            black_box(graph.total_capacity())
        })
    });

    group.finish();
}

/// Benchmark pathfinding with varying route lengths
fn bench_route_length_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("route_length_scaling");

    for hop_count in [2, 5, 10, 20].iter() {
        let graph = create_ring_network(hop_count + 1);
        let router = Router::new(graph);

        group.bench_with_input(
            BenchmarkId::from_parameter(hop_count),
            hop_count,
            |b, _| {
                b.iter(|| {
                    black_box(router.find_route(
                        &"N0".to_string(),
                        &format!("N{}", hop_count),
                        black_box(1000)
                    ))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark with different payment amounts
fn bench_amount_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("amount_scaling");

    let graph = create_mesh_network(50);
    let router = Router::new(graph);

    for amount in [100, 1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(amount),
            amount,
            |b, &amount| {
                b.iter(|| {
                    black_box(router.find_route(
                        &"N0".to_string(),
                        &"N49".to_string(),
                        black_box(amount)
                    ))
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_routing_10_nodes,
    bench_routing_100_nodes,
    bench_routing_1000_nodes,
    bench_multi_route_finding,
    bench_capacity_updates,
    bench_graph_operations,
    bench_route_length_scaling,
    bench_amount_scaling,
);

criterion_main!(benches);
