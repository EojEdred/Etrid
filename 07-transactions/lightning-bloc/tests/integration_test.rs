//! Integration Tests for Lightning-Bloc
//!
//! Tests full payment flow: channel open -> route -> HTLC -> settle -> close

use etrid_lightning_bloc::*;
use etrid_lightning_bloc::routing::{NetworkGraph, Router, ChannelEdge};
use std::collections::HashMap;

/// Test full payment channel lifecycle
#[test]
fn test_full_channel_lifecycle() {
    let mut bloc = LightningBloc::new();

    // 1. Open channel
    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();
    assert_eq!(bloc.active_channels_count(), 1);

    // 2. Execute payments
    bloc.execute_payment(&channel_id, true, 100).unwrap();
    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.current_balance_a, 900);
    assert_eq!(ch.current_balance_b, 1100);

    // 3. Verify channel state
    assert!(bloc.verify_channel(&channel_id).unwrap());

    // 4. Transition to closing
    bloc.transition_channel_state(&channel_id, ChannelState::Closing).unwrap();

    // 5. Close channel
    bloc.transition_channel_state(&channel_id, ChannelState::Closed).unwrap();

    // 6. Settle channel
    let settlement = Settlement::new(channel_id.clone(), 900, 1100, 1, 500);
    bloc.settle_channel(&channel_id, settlement).unwrap();

    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.state, ChannelState::Settled);
}

/// Test multi-hop payment routing through multiple channels
#[test]
fn test_multi_hop_payment_routing() {
    let mut bloc = LightningBloc::new();

    // Create a payment network: Alice -> Bob -> Carol -> Dave
    let mut graph = NetworkGraph::new();

    // Alice -> Bob
    graph.add_channel(ChannelEdge {
        channel_id: "alice_bob".to_string(),
        from_node: "alice".to_string(),
        to_node: "bob".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 100,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    // Bob -> Carol
    graph.add_channel(ChannelEdge {
        channel_id: "bob_carol".to_string(),
        from_node: "bob".to_string(),
        to_node: "carol".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 100,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    // Carol -> Dave
    graph.add_channel(ChannelEdge {
        channel_id: "carol_dave".to_string(),
        from_node: "carol".to_string(),
        to_node: "dave".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 100,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    // Find route from Alice to Dave
    let mut router = Router::new(graph);
    router.set_max_fee_percent(1000); // 10% max fee
    let route = router.find_route(
        &"alice".to_string(),
        &"dave".to_string(),
        100
    ).unwrap();

    // Verify route
    assert_eq!(route.hop_count(), 3);
    assert_eq!(route.path(), vec!["alice", "bob", "carol", "dave"]);
    assert!(route.verify().is_ok());

    // Verify fees are calculated
    assert!(route.total_fees > 0);
    assert_eq!(route.total_amount, 100 + route.total_fees);

    // Verify time locks are accumulated
    assert_eq!(route.total_time_lock, 120); // 3 hops * 40 delta
}

/// Test HTLC creation and settlement flow
#[test]
fn test_htlc_flow() {
    let mut bloc = LightningBloc::new();

    // Open channel
    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    // Create channel update (simulating HTLC)
    let update = ChannelUpdate::new(
        channel_id.clone(),
        1,
        900,
        1100,
        vec![1, 2, 3], // Signature A
        150,
    );

    // Submit update
    bloc.submit_update(update).unwrap();

    // Sign by party B
    bloc.sign_update(&channel_id, vec![4, 5, 6]).unwrap();

    // Verify update is fully signed
    let updates = bloc.get_updates(&channel_id).unwrap();
    assert_eq!(updates.len(), 1);
    assert!(updates[0].is_fully_signed());

    // Execute payment
    bloc.execute_payment(&channel_id, true, 100).unwrap();

    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.current_balance_a, 900);
    assert_eq!(ch.current_balance_b, 1100);
}

/// Test concurrent payments across multiple channels
#[test]
fn test_concurrent_payments() {
    let mut bloc = LightningBloc::new();

    // Open multiple channels
    for i in 0..5 {
        let channel = PaymentChannel::new(
            format!("ch{}", i),
            "alice".to_string(),
            format!("peer{}", i),
            1000,
            1000,
            100,
            1000,
        ).unwrap();
        bloc.open_channel(channel).unwrap();
    }

    assert_eq!(bloc.active_channels_count(), 5);

    // Execute concurrent payments
    for i in 0..5 {
        let channel_id = format!("ch{}", i);
        bloc.execute_payment(&channel_id, true, 50 * (i as u128 + 1)).unwrap();
    }

    // Verify all payments executed correctly
    for i in 0..5 {
        let channel_id = format!("ch{}", i);
        let ch = bloc.get_channel(&channel_id).unwrap();
        let expected_balance_a = 1000 - 50 * (i as u128 + 1);
        let expected_balance_b = 1000 + 50 * (i as u128 + 1);
        assert_eq!(ch.current_balance_a, expected_balance_a);
        assert_eq!(ch.current_balance_b, expected_balance_b);
    }

    // Calculate total locked value
    let total_value = bloc.total_locked_value();
    assert_eq!(total_value, 10000); // 5 channels * 2000 total
}

/// Test channel rebalancing scenario
#[test]
fn test_channel_rebalancing() {
    let mut bloc = LightningBloc::new();

    // Open channel with unbalanced state
    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1500, // Alice has more
        500,  // Bob has less
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    // Rebalance by sending from Alice to Bob
    bloc.execute_payment(&channel_id, true, 500).unwrap();

    // Verify balanced state
    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.current_balance_a, 1000);
    assert_eq!(ch.current_balance_b, 1000);

    // Rebalance back
    bloc.execute_payment(&channel_id, false, 250).unwrap();

    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.current_balance_a, 1250);
    assert_eq!(ch.current_balance_b, 750);
}

/// Test failed payment due to insufficient balance
#[test]
fn test_failed_payment_insufficient_balance() {
    let mut bloc = LightningBloc::new();

    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        100,  // Low balance
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    // Try to send more than available balance
    let result = bloc.execute_payment(&channel_id, true, 200);
    assert!(result.is_err());

    // Verify channel state unchanged
    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.current_balance_a, 100);
    assert_eq!(ch.current_balance_b, 1000);
}

/// Test dispute resolution flow
#[test]
fn test_dispute_resolution() {
    let mut bloc = LightningBloc::new();

    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    // File a dispute
    let evidence = DisputeEvidence::new(5, 900, 1100, "fraud_detected".to_string());
    let dispute = Dispute {
        channel_id: channel_id.clone(),
        complained_by: "alice".to_string(),
        reason: DisputeReason::InvalidStateUpdate,
        evidence,
        created_at: 200,
        resolved: false,
    };

    let dispute_id = bloc.file_dispute(dispute).unwrap();

    // Verify channel is in disputed state
    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.state, ChannelState::Disputed);

    // Resolve dispute
    bloc.resolve_dispute(&dispute_id).unwrap();

    let resolved_dispute = bloc.get_dispute(&dispute_id).unwrap();
    assert!(resolved_dispute.resolved);
}

/// Test routing with capacity constraints
#[test]
fn test_routing_with_capacity_constraints() {
    let mut graph = NetworkGraph::new();

    // Create network with varying capacities
    graph.add_channel(ChannelEdge {
        channel_id: "AB_high".to_string(),
        from_node: "A".to_string(),
        to_node: "B".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 100,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    graph.add_channel(ChannelEdge {
        channel_id: "BC_low".to_string(),
        from_node: "B".to_string(),
        to_node: "C".to_string(),
        capacity: 100, // Low capacity bottleneck
        base_fee: 1,
        fee_rate: 100,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    let router = Router::new(graph);

    // Small payment should succeed
    let route1 = router.find_route(&"A".to_string(), &"C".to_string(), 50);
    assert!(route1.is_ok());

    // Large payment should fail due to bottleneck
    let route2 = router.find_route(&"A".to_string(), &"C".to_string(), 500);
    assert!(route2.is_err());
}

/// Test alternative route finding when primary route fails
#[test]
fn test_alternative_routes() {
    let mut graph = NetworkGraph::new();

    // Create diamond topology with two paths: A->B->D and A->C->D
    graph.add_channel(ChannelEdge {
        channel_id: "AB".to_string(),
        from_node: "A".to_string(),
        to_node: "B".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 200, // Higher fee
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    graph.add_channel(ChannelEdge {
        channel_id: "AC".to_string(),
        from_node: "A".to_string(),
        to_node: "C".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 50, // Lower fee
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    graph.add_channel(ChannelEdge {
        channel_id: "BD".to_string(),
        from_node: "B".to_string(),
        to_node: "D".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 200,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    graph.add_channel(ChannelEdge {
        channel_id: "CD".to_string(),
        from_node: "C".to_string(),
        to_node: "D".to_string(),
        capacity: 1000,
        base_fee: 1,
        fee_rate: 50,
        min_htlc: 1,
        max_htlc: 1000,
        time_lock_delta: 40,
    }).unwrap();

    let router = Router::new(graph);

    // Find multiple routes
    let routes = router.find_routes(&"A".to_string(), &"D".to_string(), 100, 2);
    assert_eq!(routes.len(), 2);

    // First route should be the cheapest (A->C->D)
    assert_eq!(routes[0].path(), vec!["A", "C", "D"]);
    assert!(routes[0].total_fees < routes[1].total_fees);
}

/// Test state transitions
#[test]
fn test_channel_state_transitions() {
    let mut bloc = LightningBloc::new();

    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    // Valid transitions
    assert!(bloc.transition_channel_state(&channel_id, ChannelState::Suspended).is_ok());
    assert!(bloc.transition_channel_state(&channel_id, ChannelState::Open).is_ok());
    assert!(bloc.transition_channel_state(&channel_id, ChannelState::Closing).is_ok());
    assert!(bloc.transition_channel_state(&channel_id, ChannelState::Closed).is_ok());

    let settlement = Settlement::new(channel_id.clone(), 1000, 1000, 0, 500);
    assert!(bloc.settle_channel(&channel_id, settlement).is_ok());

    // Invalid transition from settled
    let result = bloc.transition_channel_state(&channel_id, ChannelState::Open);
    assert!(result.is_err());
}

/// Test balance invariants are maintained
#[test]
fn test_balance_invariants() {
    let mut bloc = LightningBloc::new();

    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    // Execute many payments
    for i in 0..10 {
        if i % 2 == 0 {
            bloc.execute_payment(&channel_id, true, 10).unwrap();
        } else {
            bloc.execute_payment(&channel_id, false, 10).unwrap();
        }
    }

    // Verify balance invariant holds
    assert!(bloc.verify_channel(&channel_id).unwrap());

    let ch = bloc.get_channel(&channel_id).unwrap();
    assert_eq!(ch.current_balance_a + ch.current_balance_b, 2000);
}

/// Test channel expiration handling
#[test]
fn test_channel_expiration() {
    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        200, // Expires at time 200
    ).unwrap();

    assert!(!channel.is_expired(150));
    assert!(!channel.is_expired(200));
    assert!(channel.is_expired(201));
    assert!(channel.is_expired(300));
}

/// Test payment nonce increment
#[test]
fn test_nonce_increment() {
    let mut bloc = LightningBloc::new();

    let channel = PaymentChannel::new(
        "ch1".to_string(),
        "alice".to_string(),
        "bob".to_string(),
        1000,
        1000,
        100,
        1000,
    ).unwrap();

    let channel_id = bloc.open_channel(channel).unwrap();

    for i in 0..10 {
        bloc.execute_payment(&channel_id, true, 10).unwrap();
        let ch = bloc.get_channel(&channel_id).unwrap();
        assert_eq!(ch.nonce, (i + 1) as u64);
    }
}

/// Test large-scale network routing
#[test]
fn test_large_scale_routing() {
    let mut graph = NetworkGraph::new();

    // Create 20-node linear network
    for i in 0..19 {
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

    let mut router = Router::new(graph);
    router.set_max_fee_percent(5000); // 50% max fee for large-scale routing

    // Route from N0 to N19 (19 hops)
    let route = router.find_route(&"N0".to_string(), &"N19".to_string(), 1000).unwrap();
    assert_eq!(route.hop_count(), 19);
    assert!(route.total_fees > 0);
    assert!(route.verify().is_ok());
}

/// Test concurrent channel operations
#[test]
fn test_concurrent_channel_operations() {
    let mut bloc = LightningBloc::new();

    // Open 100 channels
    for i in 0..100 {
        let channel = PaymentChannel::new(
            format!("ch{}", i),
            "alice".to_string(),
            format!("peer{}", i),
            1000,
            1000,
            100,
            1000,
        ).unwrap();
        bloc.open_channel(channel).unwrap();
    }

    assert_eq!(bloc.active_channels_count(), 100);
    assert_eq!(bloc.total_locked_value(), 200_000);

    // Close half of them
    for i in 0..50 {
        let channel_id = format!("ch{}", i);
        bloc.transition_channel_state(&channel_id, ChannelState::Closing).unwrap();
        bloc.transition_channel_state(&channel_id, ChannelState::Closed).unwrap();
    }

    assert_eq!(bloc.active_channels_count(), 50);
}
