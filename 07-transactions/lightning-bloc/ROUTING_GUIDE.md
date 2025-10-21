# Lightning Bloc Routing Guide

## Overview

The Lightning Bloc routing protocol enables multi-hop payments across the Ëtrid network. This guide explains how to use the routing system to find optimal payment paths and execute multi-hop transactions.

## Architecture

```
┌─────────────────────────────────────────┐
│         Network Graph                    │
│  (Nodes + Channels + Capacities)        │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│            Router                        │
│     (Dijkstra's Algorithm)              │
└────────────────┬────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────┐
│           Route                          │
│  (Hops + Fees + Time Locks)             │
└─────────────────────────────────────────┘
```

## Quick Start

### 1. Create a Network Graph

```rust
use etrid_lightning_bloc::{NetworkGraph, ChannelEdge};

let mut graph = NetworkGraph::new();

// Add nodes implicitly by adding channels
graph.add_channel(ChannelEdge {
    channel_id: "alice-bob".to_string(),
    from_node: "Alice".to_string(),
    to_node: "Bob".to_string(),
    capacity: 10_000,
    base_fee: 1,
    fee_rate: 100,  // 1% (100 basis points)
    min_htlc: 1,
    max_htlc: 5_000,
    time_lock_delta: 40,
}).unwrap();

graph.add_channel(ChannelEdge {
    channel_id: "bob-charlie".to_string(),
    from_node: "Bob".to_string(),
    to_node: "Charlie".to_string(),
    capacity: 10_000,
    base_fee: 2,
    fee_rate: 50,   // 0.5%
    min_htlc: 1,
    max_htlc: 5_000,
    time_lock_delta: 40,
}).unwrap();
```

### 2. Create a Router

```rust
use etrid_lightning_bloc::Router;

let mut router = Router::new(graph);

// Optional: Configure routing parameters
router.set_max_route_length(20);      // Maximum 20 hops
router.set_max_fee_percent(500);      // Maximum 5% fee
```

### 3. Find a Route

```rust
let route = router.find_route(
    &"Alice".to_string(),
    &"Charlie".to_string(),
    1_000  // Amount to send
).unwrap();

// Inspect the route
println!("Path: {:?}", route.path());
println!("Hops: {}", route.hop_count());
println!("Total amount (with fees): {}", route.total_amount);
println!("Total fees: {}", route.total_fees);
println!("Total time lock: {}", route.total_time_lock);

// Iterate through hops
for (i, hop) in route.hops.iter().enumerate() {
    println!("Hop {}: {} -> {} via {}",
        i + 1,
        hop.from_node,
        hop.to_node,
        hop.channel_id
    );
    println!("  Amount to forward: {}", hop.amount_to_forward);
    println!("  Fee: {}", hop.fee);
    println!("  Time lock: {}", hop.time_lock);
}
```

### 4. Find Multiple Routes

```rust
let routes = router.find_routes(
    &"Alice".to_string(),
    &"Charlie".to_string(),
    1_000,
    3  // Find up to 3 alternative routes
);

for (i, route) in routes.iter().enumerate() {
    println!("Route {}: {:?}", i + 1, route.path());
    println!("  Fees: {}", route.total_fees);
}
```

## Network Management

### Add Channels Dynamically

```rust
// Channels are automatically bidirectional
graph.add_channel(ChannelEdge {
    channel_id: "charlie-dave".to_string(),
    from_node: "Charlie".to_string(),
    to_node: "Dave".to_string(),
    capacity: 15_000,
    base_fee: 1,
    fee_rate: 75,
    min_htlc: 1,
    max_htlc: 10_000,
    time_lock_delta: 40,
}).unwrap();
```

### Update Channel Capacity

```rust
// Update capacity after payment
graph.update_capacity(&"alice-bob".to_string(), 9_000).unwrap();
```

### Remove Channels

```rust
// Close a channel
graph.remove_channel(&"alice-bob".to_string()).unwrap();
```

### Query Network State

```rust
// Get network statistics
let stats = graph.stats();
println!("Nodes: {}", stats.node_count);
println!("Channels: {}", stats.channel_count);

// Get total network capacity
let total = graph.total_capacity();
println!("Total capacity: {}", total);

// Find neighbors of a node
let neighbors = graph.neighbors(&"Alice".to_string());
println!("Alice's neighbors: {:?}", neighbors);

// Get outgoing channels from a node
let channels = graph.get_outgoing_channels(&"Alice".to_string());
for channel in channels {
    println!("Channel {}: capacity {}", channel.channel_id, channel.capacity);
}
```

## Fee Calculation

Fees are calculated per hop:

```
fee = base_fee + (amount × fee_rate / 10000)
```

### Example

```rust
let edge = ChannelEdge {
    base_fee: 10,
    fee_rate: 100,  // 1%
    // ... other fields
};

let amount = 1_000;
let fee = edge.calculate_fee(amount);
// fee = 10 + (1000 × 100 / 10000) = 10 + 10 = 20
```

### Fee Rates in Basis Points

- 1 basis point = 0.01%
- 100 basis points = 1%
- 10,000 basis points = 100%

Common fee rates:
- `10` = 0.1%
- `50` = 0.5%
- `100` = 1%
- `500` = 5%

## Routing Constraints

### HTLC Limits

Each channel has minimum and maximum HTLC values:

```rust
let edge = ChannelEdge {
    min_htlc: 100,      // Minimum 100 units
    max_htlc: 5_000,    // Maximum 5,000 units
    capacity: 10_000,   // Total capacity
    // ...
};

// This will fail routing
assert!(!edge.can_route(50));        // Below min_htlc
assert!(!edge.can_route(6_000));     // Above max_htlc
assert!(!edge.can_route(11_000));    // Above capacity

// This will succeed
assert!(edge.can_route(1_000));
```

### Route Length

By default, routes are limited to 20 hops:

```rust
router.set_max_route_length(10);  // Limit to 10 hops
```

### Fee Limits

Maximum fee as percentage of payment amount:

```rust
router.set_max_fee_percent(300);  // Maximum 3% fee
```

If a route exceeds the fee limit, it will be rejected.

## Route Verification

Always verify routes before execution:

```rust
let route = router.find_route(
    &"Alice".to_string(),
    &"Dave".to_string(),
    1_000
).unwrap();

// Verify route is valid
match route.verify() {
    Ok(()) => println!("Route is valid"),
    Err(e) => println!("Route error: {}", e),
}
```

## Error Handling

```rust
use etrid_lightning_bloc::RoutingError;

match router.find_route(&source, &dest, amount) {
    Ok(route) => {
        // Use route
    }
    Err(RoutingError::NodeNotFound(node)) => {
        println!("Node {} not in network", node);
    }
    Err(RoutingError::NoRouteFound { from, to }) => {
        println!("No route from {} to {}", from, to);
    }
    Err(RoutingError::FeeTooHigh { fee, max }) => {
        println!("Fee {} exceeds maximum {}", fee, max);
    }
    Err(e) => {
        println!("Routing error: {}", e);
    }
}
```

## Integration with Payment Channels

```rust
use etrid_lightning_bloc::{LightningBloc, PaymentChannel, Router};

// Create payment channel manager
let mut bloc = LightningBloc::new();

// Open channels
let channel1 = PaymentChannel::new(
    "alice-bob".to_string(),
    "Alice".to_string(),
    "Bob".to_string(),
    10_000,
    10_000,
    now(),
    now() + 86400,  // 1 day expiry
).unwrap();
bloc.open_channel(channel1).unwrap();

// Build network graph from active channels
let mut graph = NetworkGraph::new();

// Add edges for each active channel
for channel_id in &["alice-bob", "bob-charlie"] {
    let channel = bloc.get_channel(channel_id).unwrap();
    graph.add_channel(ChannelEdge {
        channel_id: channel.id.clone(),
        from_node: channel.party_a.clone(),
        to_node: channel.party_b.clone(),
        capacity: channel.current_balance_a.min(channel.current_balance_b),
        base_fee: 1,
        fee_rate: 100,
        min_htlc: 1,
        max_htlc: channel.current_balance_a,
        time_lock_delta: 40,
    }).unwrap();
}

// Find route
let router = Router::new(graph);
let route = router.find_route(&"Alice".to_string(), &"Charlie".to_string(), 1_000).unwrap();

// Execute multi-hop payment
for hop in route.hops {
    // Execute payment through each channel
    bloc.execute_payment(
        &hop.channel_id,
        true,  // Direction
        hop.amount_to_forward
    ).unwrap();
}
```

## Advanced Features

### Custom Pathfinding

The router uses Dijkstra's algorithm to find the minimum-cost path. The cost function is the total fees.

### Load Balancing

Use multiple routes to split a large payment:

```rust
let amount = 10_000;
let routes = router.find_routes(&source, &dest, amount / 2, 2);

// Send half through each route
for route in routes {
    execute_payment(route);
}
```

### Route Caching

For frequently used paths, cache routes:

```rust
use std::collections::HashMap;

let mut route_cache: HashMap<(String, String), Route> = HashMap::new();

let key = (source.clone(), dest.clone());
let route = match route_cache.get(&key) {
    Some(cached_route) => cached_route.clone(),
    None => {
        let new_route = router.find_route(&source, &dest, amount).unwrap();
        route_cache.insert(key, new_route.clone());
        new_route
    }
};
```

## Performance Characteristics

- **Pathfinding**: O(E log V) using Dijkstra's algorithm
- **Multi-route discovery**: O(k × E log V) for k routes
- **Graph updates**: O(1) for capacity changes
- **Memory**: O(V + E) for graph storage

Where:
- V = number of nodes
- E = number of channels
- k = number of routes

## Best Practices

1. **Always verify routes** before execution
2. **Set appropriate fee limits** to prevent excessive costs
3. **Update capacities** after each payment
4. **Use multiple routes** for large payments
5. **Cache frequently used paths**
6. **Monitor channel health** (capacity, uptime)
7. **Implement fallback routes** for reliability

## Example: Complete Payment Flow

```rust
use etrid_lightning_bloc::*;

fn send_payment(
    bloc: &mut LightningBloc,
    router: &Router,
    from: &str,
    to: &str,
    amount: u128,
) -> Result<(), Box<dyn std::error::Error>> {
    // Find route
    let route = router.find_route(
        &from.to_string(),
        &to.to_string(),
        amount
    )?;

    // Verify route
    route.verify()?;

    // Display route info
    println!("Sending {} from {} to {}", amount, from, to);
    println!("Route: {:?}", route.path());
    println!("Total fees: {}", route.total_fees);
    println!("Total amount: {}", route.total_amount);

    // Execute payment through each hop
    for (i, hop) in route.hops.iter().enumerate() {
        println!("Executing hop {}/{}", i + 1, route.hops.len());

        bloc.execute_payment(
            &hop.channel_id,
            true,  // from_a_to_b
            hop.amount_to_forward
        )?;

        // Update graph capacity after payment
        // (In real implementation, this would be done by the channel manager)
    }

    println!("Payment successful!");
    Ok(())
}
```

## Next Steps

- Implement HTLC (Hash Time-Locked Contracts) for atomic multi-hop payments
- Add route probing to test channel capacity before payment
- Implement onion routing for privacy
- Add channel rebalancing strategies
- Implement watchtowers for security

## Resources

- Main implementation: `src/routing.rs`
- Payment channels: `src/lib.rs`
- Tests: `src/routing.rs` (bottom of file)
- API documentation: `cargo doc --open`

---

**Lightning Bloc Routing Protocol**
Version 1.0.0 | Ëtrid Blockchain
