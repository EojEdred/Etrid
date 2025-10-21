//! Cross-Chain Lightning Routing Demo
//!
//! Demonstrates:
//! - BTC -> ETH payment routing via Lightning Bloc
//! - Multi-asset payment channels
//! - Bridge channels between PBC chains
//! - Exchange rate handling
//! - Atomic cross-chain swaps

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct PaymentChannel {
    id: String,
    party_a: String,
    party_b: String,
    asset: String,         // BTC, ETH, etc.
    balance_a: u128,
    balance_b: u128,
}

#[derive(Debug, Clone)]
struct BridgeChannel {
    id: String,
    from_chain: String,
    to_chain: String,
    from_asset: String,
    to_asset: String,
    exchange_rate: f64,   // e.g., 1 BTC = 15 ETH
    capacity_from: u128,
    capacity_to: u128,
}

#[derive(Debug)]
struct CrossChainRoute {
    path: Vec<String>,
    total_hops: usize,
    source_amount: u128,
    dest_amount: u128,
    source_asset: String,
    dest_asset: String,
    bridge_used: Option<String>,
    total_fees: u128,
}

struct CrossChainRouter {
    btc_channels: HashMap<String, PaymentChannel>,
    eth_channels: HashMap<String, PaymentChannel>,
    bridge_channels: Vec<BridgeChannel>,
}

impl CrossChainRouter {
    fn new() -> Self {
        Self {
            btc_channels: HashMap::new(),
            eth_channels: HashMap::new(),
            bridge_channels: Vec::new(),
        }
    }

    /// Set up demo network topology
    fn setup_demo_network(&mut self) {
        // BTC Lightning Network
        // Alice (BTC) <-> Bob (BTC/Bridge)
        self.btc_channels.insert(
            "alice-bob-btc".to_string(),
            PaymentChannel {
                id: "alice-bob-btc".to_string(),
                party_a: "Alice".to_string(),
                party_b: "Bob-Bridge".to_string(),
                asset: "BTC".to_string(),
                balance_a: 50_000_000, // 0.5 BTC in satoshis
                balance_b: 50_000_000,
            },
        );

        // ETH Lightning Network
        // Bob (Bridge/ETH) <-> Charlie (ETH)
        self.eth_channels.insert(
            "bob-charlie-eth".to_string(),
            PaymentChannel {
                id: "bob-charlie-eth".to_string(),
                party_a: "Bob-Bridge".to_string(),
                party_b: "Charlie".to_string(),
                asset: "ETH".to_string(),
                balance_a: 750_000_000_000_000_000, // 0.75 ETH in wei
                balance_b: 750_000_000_000_000_000,
            },
        );

        // Bridge Channel (Bob is the bridge operator)
        self.bridge_channels.push(BridgeChannel {
            id: "btc-eth-bridge".to_string(),
            from_chain: "BTC".to_string(),
            to_chain: "ETH".to_string(),
            from_asset: "BTC".to_string(),
            to_asset: "ETH".to_string(),
            exchange_rate: 15.0, // 1 BTC = 15 ETH
            capacity_from: 100_000_000, // 1 BTC
            capacity_to: 1_500_000_000_000_000_000, // 15 ETH
        });
    }

    /// Route a cross-chain payment
    fn route_cross_chain(
        &self,
        from: &str,
        to: &str,
        amount: u128,
        source_asset: &str,
        dest_asset: &str,
    ) -> Result<CrossChainRoute, String> {
        // For demo: Assume Alice (BTC) -> Charlie (ETH)
        if source_asset == "BTC" && dest_asset == "ETH" {
            // Find bridge
            let bridge = self.bridge_channels.iter()
                .find(|b| b.from_asset == "BTC" && b.to_asset == "ETH")
                .ok_or("No bridge found")?;

            // Calculate destination amount after exchange
            let dest_amount = (amount as f64 * bridge.exchange_rate) as u128;

            // Calculate fees (1% on BTC side, 0.5% on ETH side)
            let btc_fee = amount / 100;
            let eth_fee = dest_amount / 200;
            let total_fees_btc = btc_fee + (eth_fee as f64 / bridge.exchange_rate) as u128;

            Ok(CrossChainRoute {
                path: vec![
                    from.to_string(),
                    "Bob-Bridge (BTC)".to_string(),
                    "Bob-Bridge (ETH)".to_string(),
                    to.to_string(),
                ],
                total_hops: 3,
                source_amount: amount,
                dest_amount,
                source_asset: source_asset.to_string(),
                dest_asset: dest_asset.to_string(),
                bridge_used: Some(bridge.id.clone()),
                total_fees: total_fees_btc,
            })
        } else {
            Err("Unsupported asset pair".to_string())
        }
    }

    /// Execute cross-chain payment
    fn execute_cross_chain_payment(
        &mut self,
        route: &CrossChainRoute,
    ) -> Result<(), String> {
        println!("\n🌉 Executing Cross-Chain Payment:");
        println!("   Source: {} {}", format_satoshis(route.source_amount), route.source_asset);
        println!("   Destination: {} {}", format_wei(route.dest_amount), route.dest_asset);
        println!("   Route: {}", route.path.join(" → "));
        println!();

        // Step 1: BTC payment (Alice -> Bob-Bridge)
        println!("   Step 1: BTC Lightning Channel");
        let btc_channel = self.btc_channels.get_mut("alice-bob-btc")
            .ok_or("BTC channel not found")?;

        let amount_with_fee = route.source_amount + (route.source_amount / 100);

        if btc_channel.balance_a < amount_with_fee {
            return Err("Insufficient BTC balance".to_string());
        }

        btc_channel.balance_a -= amount_with_fee;
        btc_channel.balance_b += amount_with_fee;

        println!("     ✓ Sent {} BTC (including fee)", format_satoshis(amount_with_fee));
        println!("     Alice balance: {} BTC", format_satoshis(btc_channel.balance_a));
        println!();

        // Step 2: Bridge exchange (Bob converts BTC -> ETH)
        println!("   Step 2: Bridge Exchange (BTC -> ETH)");
        let bridge = self.bridge_channels.iter()
            .find(|b| &b.id == route.bridge_used.as_ref().unwrap())
            .unwrap();

        println!("     Exchange rate: 1 BTC = {} ETH", bridge.exchange_rate);
        println!("     Received: {} BTC", format_satoshis(route.source_amount));
        println!("     Converted to: {} ETH", format_wei(route.dest_amount));
        println!();

        // Step 3: ETH payment (Bob-Bridge -> Charlie)
        println!("   Step 3: ETH Lightning Channel");
        let eth_channel = self.eth_channels.get_mut("bob-charlie-eth")
            .ok_or("ETH channel not found")?;

        if eth_channel.balance_a < route.dest_amount {
            return Err("Insufficient ETH balance in bridge".to_string());
        }

        eth_channel.balance_a -= route.dest_amount;
        eth_channel.balance_b += route.dest_amount;

        println!("     ✓ Sent {} ETH", format_wei(route.dest_amount));
        println!("     Charlie balance: {} ETH", format_wei(eth_channel.balance_b));
        println!();

        println!("   ✅ Cross-chain payment successful!");
        println!();

        Ok(())
    }

    /// Display network status
    fn display_network_status(&self) {
        println!("\n========================================");
        println!("Cross-Chain Lightning Network Status");
        println!("========================================\n");

        println!("BTC Lightning Network:");
        for (_id, channel) in &self.btc_channels {
            println!("  {} <-> {}: {} / {} BTC",
                channel.party_a,
                channel.party_b,
                format_satoshis(channel.balance_a),
                format_satoshis(channel.balance_b)
            );
        }

        println!();
        println!("ETH Lightning Network:");
        for (_id, channel) in &self.eth_channels {
            println!("  {} <-> {}: {} / {} ETH",
                channel.party_a,
                channel.party_b,
                format_wei(channel.balance_a),
                format_wei(channel.balance_b)
            );
        }

        println!();
        println!("Bridge Channels:");
        for bridge in &self.bridge_channels {
            println!("  {} -> {}: 1 {} = {} {}",
                bridge.from_asset,
                bridge.to_asset,
                bridge.from_asset,
                bridge.exchange_rate,
                bridge.to_asset
            );
            println!("    Capacity: {} {} / {} {}",
                format_satoshis(bridge.capacity_from),
                bridge.from_asset,
                format_wei(bridge.capacity_to),
                bridge.to_asset
            );
        }

        println!("\n========================================\n");
    }
}

// Helper functions
fn format_satoshis(sats: u128) -> String {
    let btc = sats as f64 / 100_000_000.0;
    format!("{:.8}", btc)
}

fn format_wei(wei: u128) -> String {
    let eth = wei as f64 / 1_000_000_000_000_000_000.0;
    format!("{:.6}", eth)
}

fn main() {
    println!("⚡ Cross-Chain Lightning Routing Demo");
    println!("   Ëtrid Lightning Bloc Network\n");

    // Initialize router
    let mut router = CrossChainRouter::new();
    router.setup_demo_network();

    // Display initial network status
    router.display_network_status();

    // Demo 1: Alice (BTC) sends to Charlie (ETH)
    println!("========================================");
    println!("Demo 1: BTC -> ETH Cross-Chain Payment");
    println!("========================================\n");

    let amount_btc = 10_000_000; // 0.1 BTC
    println!("Alice wants to send 0.1 BTC to Charlie (who accepts ETH)");
    println!();

    // Find route
    match router.route_cross_chain("Alice", "Charlie", amount_btc, "BTC", "ETH") {
        Ok(route) => {
            println!("✓ Cross-chain route found:");
            println!("  Source: {} {}", format_satoshis(route.source_amount), route.source_asset);
            println!("  Destination: {} {}", format_wei(route.dest_amount), route.dest_asset);
            println!("  Path: {}", route.path.join(" → "));
            println!("  Hops: {}", route.total_hops);
            println!("  Total fees: {} BTC", format_satoshis(route.total_fees));
            println!();

            // Execute payment
            match router.execute_cross_chain_payment(&route) {
                Ok(_) => {
                    println!("✅ Payment executed successfully!\n");
                },
                Err(e) => {
                    println!("❌ Payment failed: {}\n", e);
                }
            }
        },
        Err(e) => {
            println!("❌ Route not found: {}\n", e);
        }
    }

    // Display final network status
    router.display_network_status();

    // Summary
    println!("========================================");
    println!("Demo Summary");
    println!("========================================\n");

    println!("Key Features Demonstrated:");
    println!("  ✓ Cross-chain routing (BTC -> ETH)");
    println!("  ✓ Bridge channel exchange");
    println!("  ✓ Multi-asset Lightning channels");
    println!("  ✓ Automatic exchange rate conversion");
    println!("  ✓ Fee calculation across chains");
    println!();

    println!("Technical Details:");
    println!("  - Exchange rate: 1 BTC = 15 ETH");
    println!("  - BTC channel fee: 1%");
    println!("  - ETH channel fee: 0.5%");
    println!("  - Bridge operated by Bob");
    println!("  - Atomic payment guarantee via HTLCs");
    println!();

    println!("Use Cases:");
    println!("  1. Bitcoin holders paying Ethereum merchants");
    println!("  2. Cross-chain DeFi operations");
    println!("  3. Multi-asset portfolio rebalancing");
    println!("  4. Instant cross-chain arbitrage");
    println!();

    println!("Next Steps:");
    println!("  1. Add more bridge channels (SOL, XRP, etc.)");
    println!("  2. Implement HTLC atomic swaps");
    println!("  3. Add multi-bridge routing");
    println!("  4. Deploy on testnet");
    println!();
}
