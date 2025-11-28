//! Example: Staking ETR tokens and nominating validators
//!
//! This example demonstrates how to use the Staking wrapper to:
//! - Bond tokens for staking
//! - Nominate validators
//! - Estimate staking rewards
//! - Query network statistics

use etrid_sdk::{Client, wrappers::{StakingWrapper, BondParams, NominateParams, RewardDestination}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Ëtrid node
    let client = Client::new("ws://localhost:9944").await?;
    println!("Connected to Ëtrid node: {}", client.endpoint());

    // Create staking wrapper
    let staking = StakingWrapper::new(client);

    // Step 1: Bond tokens for staking
    println!("\n=== Bonding Tokens ===");
    let bond_params = BondParams {
        amount: 1_000_000_000_000, // 1000 ETR
        reward_destination: RewardDestination::Staked, // Auto-compound rewards
    };

    let tx_hash = staking.bond(bond_params).await?;
    println!("Bonded 1000 ETR. Transaction: {}", tx_hash);

    // Step 2: Nominate validators
    println!("\n=== Nominating Validators ===");
    let nominate_params = NominateParams {
        targets: vec![
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
            "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
            "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy".to_string(),
        ],
    };

    let tx_hash = staking.nominate(nominate_params).await?;
    println!("Nominated 3 validators. Transaction: {}", tx_hash);

    // Step 3: Estimate staking rewards
    println!("\n=== Estimating Rewards ===");
    let rewards = staking.estimate_rewards(1_000_000_000_000, None).await?;
    println!("Annual Percentage Yield (APY): {}%", rewards.apy);
    println!("Estimated daily rewards: {} ETR", rewards.daily_rewards as f64 / 1e12);
    println!("Estimated monthly rewards: {} ETR", rewards.monthly_rewards as f64 / 1e12);
    println!("Estimated annual rewards: {} ETR", rewards.annual_rewards as f64 / 1e12);

    // Step 4: Get network statistics
    println!("\n=== Network Statistics ===");
    let stats = staking.get_network_stats().await?;
    println!("Total staked: {} ETR", stats.total_staked as f64 / 1e12);
    println!("Active validators: {}", stats.validator_count);
    println!("Total nominators: {}", stats.nominator_count);
    println!("Network average APY: {}%", stats.average_apy);

    // Step 5: Get current era information
    println!("\n=== Current Era ===");
    let era = staking.get_current_era().await?;
    println!("Current era: {}", era.current_era);
    println!("Blocks remaining in era: {}", era.blocks_remaining);
    println!("Era duration: {} blocks", era.duration);

    // Step 6: Check validator status
    println!("\n=== Validator Status ===");
    let validator = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
    let status = staking.get_validator_status(validator).await?;
    println!("Validator: {}", validator);
    println!("Active: {}", status.is_active);
    println!("Total stake: {} ETR", status.total_stake as f64 / 1e12);
    println!("Commission: {}%", status.info.commission);
    println!("Nominators: {}", status.info.nominators);

    println!("\n✅ Staking example completed successfully!");

    Ok(())
}
