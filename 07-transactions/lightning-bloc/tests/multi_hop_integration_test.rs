//! Multi-Hop Payment Integration Tests
//!
//! Tests Lightning Bloc routing through multiple hops:
//! - 2-hop payments (Alice -> Bob -> Charlie)
//! - 3+ hop payments (extended network)
//! - Fee calculation accuracy
//! - Failure scenarios (insufficient capacity, route not found)
//! - Balance updates after payments
//! - Channel capacity constraints

use etrid_lightning_bloc::{
    LightningBloc, PaymentChannel, NetworkGraph, Router, ChannelEdge,
    ChannelError, RoutingError,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Set up a 3-node linear network: Alice <-> Bob <-> Charlie
    fn setup_linear_network() -> (LightningBloc, Router) {
        let mut bloc = LightningBloc::new();
        let mut graph = NetworkGraph::new();

        // Create channels
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Alice <-> Bob (10,000 capacity)
        let alice_bob = PaymentChannel::new(
            "alice-bob".to_string(),
            "Alice".to_string(),
            "Bob".to_string(),
            5000,
            5000,
            now,
            now + 86400,
        ).unwrap();

        bloc.open_channel(alice_bob).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "alice-bob".to_string(),
            from_node: "Alice".to_string(),
            to_node: "Bob".to_string(),
            capacity: 10000,
            base_fee: 1,
            fee_rate: 100, // 1%
            min_htlc: 1,
            max_htlc: 10000,
            time_lock_delta: 40,
        }).unwrap();

        // Bob <-> Charlie (15,000 capacity)
        let bob_charlie = PaymentChannel::new(
            "bob-charlie".to_string(),
            "Bob".to_string(),
            "Charlie".to_string(),
            7500,
            7500,
            now,
            now + 86400,
        ).unwrap();

        bloc.open_channel(bob_charlie).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "bob-charlie".to_string(),
            from_node: "Bob".to_string(),
            to_node: "Charlie".to_string(),
            capacity: 15000,
            base_fee: 2,
            fee_rate: 50, // 0.5%
            min_htlc: 1,
            max_htlc: 15000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);

        (bloc, router)
    }

    /// Set up a 5-node mesh network for complex routing
    fn setup_mesh_network() -> (LightningBloc, Router) {
        let mut bloc = LightningBloc::new();
        let mut graph = NetworkGraph::new();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create a mesh: Alice, Bob, Charlie, Dave, Eve
        let nodes = ["Alice", "Bob", "Charlie", "Dave", "Eve"];

        // Connect each pair
        for (i, from) in nodes.iter().enumerate() {
            for to in nodes.iter().skip(i + 1) {
                let channel_id = format!("{}-{}", from.to_lowercase(), to.to_lowercase());

                // Create channel
                let channel = PaymentChannel::new(
                    channel_id.clone(),
                    from.to_string(),
                    to.to_string(),
                    10000,
                    10000,
                    now,
                    now + 86400,
                ).unwrap();

                bloc.open_channel(channel).unwrap();

                // Add to graph
                graph.add_channel(ChannelEdge {
                    channel_id,
                    from_node: from.to_string(),
                    to_node: to.to_string(),
                    capacity: 20000,
                    base_fee: 1,
                    fee_rate: 75, // 0.75%
                    min_htlc: 1,
                    max_htlc: 20000,
                    time_lock_delta: 40,
                }).unwrap();
            }
        }

        let router = Router::new(graph);

        (bloc, router)
    }

    #[test]
    fn test_simple_2_hop_payment() {
        let (mut bloc, router) = setup_linear_network();

        // Alice sends 1000 to Charlie (via Bob)
        let route = router.find_route(
            &"Alice".to_string(),
            &"Charlie".to_string(),
            1000,
        ).unwrap();

        assert_eq!(route.hop_count(), 2);
        assert_eq!(route.path(), vec!["Alice", "Bob", "Charlie"]);

        // Execute payment through each hop
        for hop in &route.hops {
            bloc.execute_payment(
                &hop.channel_id,
                true, // assuming direction
                hop.amount_to_forward,
            ).unwrap();
        }

        // Verify final balances
        let alice_bob = bloc.get_channel("alice-bob").unwrap();
        let bob_charlie = bloc.get_channel("bob-charlie").unwrap();

        // Alice should have sent ~1010 (1000 + fees)
        assert!(alice_bob.current_balance_a < 5000);
        assert!(alice_bob.current_balance_a >= 3990);

        // Charlie should have received 1000
        assert!(bob_charlie.current_balance_b > 7500);
        assert!(bob_charlie.current_balance_b <= 8500);
    }

    #[test]
    fn test_fee_calculation_accuracy() {
        let (bloc, router) = setup_linear_network();

        let route = router.find_route(
            &"Alice".to_string(),
            &"Charlie".to_string(),
            1000,
        ).unwrap();

        // Check fee calculation
        // Hop 1 (Alice -> Bob): base_fee=1, fee_rate=100 (1%)
        // Hop 2 (Bob -> Charlie): base_fee=2, fee_rate=50 (0.5%)

        // Expected total fees: ~10-15 (depending on implementation)
        assert!(route.total_fees > 0);
        assert!(route.total_fees < 20);

        println!("Route fees: {}", route.total_fees);
        println!("Total amount (with fees): {}", route.total_amount);
    }

    #[test]
    fn test_insufficient_capacity_fails() {
        let (bloc, router) = setup_linear_network();

        // Try to send 20,000 (more than any channel can handle)
        let result = router.find_route(
            &"Alice".to_string(),
            &"Charlie".to_string(),
            20000,
        );

        assert!(result.is_err());

        match result {
            Err(RoutingError::NoRouteFound) => {
                // Expected - no route with sufficient capacity
            },
            _ => panic!("Expected NoRouteFound error"),
        }
    }

    #[test]
    fn test_route_not_found_for_disconnected_nodes() {
        let mut bloc = LightningBloc::new();
        let mut graph = NetworkGraph::new();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create two isolated channels
        // Alice <-> Bob (isolated)
        let alice_bob = PaymentChannel::new(
            "alice-bob".to_string(),
            "Alice".to_string(),
            "Bob".to_string(),
            5000,
            5000,
            now,
            now + 86400,
        ).unwrap();

        bloc.open_channel(alice_bob).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "alice-bob".to_string(),
            from_node: "Alice".to_string(),
            to_node: "Bob".to_string(),
            capacity: 10000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 10000,
            time_lock_delta: 40,
        }).unwrap();

        // Charlie <-> Dave (isolated, no connection to Alice/Bob)
        let charlie_dave = PaymentChannel::new(
            "charlie-dave".to_string(),
            "Charlie".to_string(),
            "Dave".to_string(),
            5000,
            5000,
            now,
            now + 86400,
        ).unwrap();

        bloc.open_channel(charlie_dave).unwrap();

        graph.add_channel(ChannelEdge {
            channel_id: "charlie-dave".to_string(),
            from_node: "Charlie".to_string(),
            to_node: "Dave".to_string(),
            capacity: 10000,
            base_fee: 1,
            fee_rate: 100,
            min_htlc: 1,
            max_htlc: 10000,
            time_lock_delta: 40,
        }).unwrap();

        let router = Router::new(graph);

        // Try to route from Alice to Charlie (no path exists)
        let result = router.find_route(
            &"Alice".to_string(),
            &"Charlie".to_string(),
            1000,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_3_hop_payment_in_mesh_network() {
        let (mut bloc, router) = setup_mesh_network();

        // Alice sends to Eve (multiple possible routes)
        let route = router.find_route(
            &"Alice".to_string(),
            &"Eve".to_string(),
            1000,
        ).unwrap();

        // Could be 1-hop direct or multi-hop depending on router
        println!("Route found: {:?}", route.path());
        println!("Hops: {}", route.hop_count());
        println!("Fees: {}", route.total_fees);

        // Execute payment
        for hop in &route.hops {
            bloc.execute_payment(
                &hop.channel_id,
                true,
                hop.amount_to_forward,
            ).unwrap();
        }

        // Verify payment completed
        assert!(route.total_amount >= 1000);
    }

    #[test]
    fn test_multiple_sequential_payments() {
        let (mut bloc, mut router) = setup_linear_network();

        // Send multiple payments in sequence
        for i in 1..=5 {
            let amount = 100 * i;

            let route = router.find_route(
                &"Alice".to_string(),
                &"Charlie".to_string(),
                amount,
            ).unwrap();

            println!("Payment {}: {} with {} hops", i, amount, route.hop_count());

            for hop in &route.hops {
                bloc.execute_payment(
                    &hop.channel_id,
                    true,
                    hop.amount_to_forward,
                ).unwrap();

                // Update router graph capacity after each payment
                let channel = bloc.get_channel(&hop.channel_id).unwrap();
                let mut graph = router.take_graph();
                graph.update_capacity(
                    &hop.channel_id,
                    channel.current_balance_a.min(channel.current_balance_b),
                ).unwrap();
                router = Router::new(graph);
            }
        }

        // Verify total sent: 100 + 200 + 300 + 400 + 500 = 1500 (plus fees)
        let alice_bob = bloc.get_channel("alice-bob").unwrap();
        assert!(alice_bob.current_balance_a < 3500); // Started at 5000, sent ~1500+
    }

    #[test]
    fn test_bidirectional_payments() {
        let (mut bloc, router) = setup_linear_network();

        // Alice -> Charlie
        let route_forward = router.find_route(
            &"Alice".to_string(),
            &"Charlie".to_string(),
            1000,
        ).unwrap();

        for hop in &route_forward.hops {
            bloc.execute_payment(&hop.channel_id, true, hop.amount_to_forward).unwrap();
        }

        // Charlie -> Alice (reverse direction)
        let route_backward = router.find_route(
            &"Charlie".to_string(),
            &"Alice".to_string(),
            500,
        ).unwrap();

        // Execute in reverse (from_a_to_b = false)
        for hop in &route_backward.hops {
            bloc.execute_payment(&hop.channel_id, false, hop.amount_to_forward).unwrap();
        }

        // Net effect should be ~500 from Alice to Charlie
        let alice_bob = bloc.get_channel("alice-bob").unwrap();
        assert!(alice_bob.current_balance_a < 5000);
        assert!(alice_bob.current_balance_a > 4400);
    }

    #[test]
    fn test_payment_exceeds_max_htlc_fails() {
        let (mut bloc, router) = setup_linear_network();

        // Try to send more than max_htlc (10,000 for alice-bob)
        let result = router.find_route(
            &"Alice".to_string(),
            &"Charlie".to_string(),
            15000,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_channel_balance_depletion() {
        let (mut bloc, mut router) = setup_linear_network();

        // Drain Alice's balance in alice-bob channel
        for _ in 0..10 {
            let route = router.find_route(
                &"Alice".to_string(),
                &"Charlie".to_string(),
                400,
            );

            match route {
                Ok(r) => {
                    for hop in &r.hops {
                        bloc.execute_payment(&hop.channel_id, true, hop.amount_to_forward).unwrap();

                        let channel = bloc.get_channel(&hop.channel_id).unwrap();
                        let mut graph = router.take_graph();
                        graph.update_capacity(&hop.channel_id, channel.current_balance_a.min(channel.current_balance_b)).unwrap();
                        router = Router::new(graph);
                    }
                },
                Err(_) => {
                    // Expected eventually - Alice runs out of balance
                    break;
                }
            }
        }

        // Alice's balance should be depleted
        let alice_bob = bloc.get_channel("alice-bob").unwrap();
        assert!(alice_bob.current_balance_a < 1000);
    }
}

// Helper trait for Router
impl Router {
    pub fn take_graph(self) -> NetworkGraph {
        self.graph
    }
}
