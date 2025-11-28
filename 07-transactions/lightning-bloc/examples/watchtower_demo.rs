//! Watchtower Incentive System Demo
//!
//! Demonstrates the complete lifecycle of watchtower operations:
//! - Registration with stake
//! - Channel subscription with fees
//! - Fraud detection and reporting
//! - Slashing for misbehavior
//! - Statistics and monitoring

use etrid_lightning_bloc::{
    WatchtowerManager, FraudEvidence, WatchtowerError,
    MIN_WATCHTOWER_STAKE, WATCHTOWER_BASE_REWARD,
};

fn format_etr(amount: u128) -> String {
    let etr = amount as f64 / 1_000_000_000_000_000_000.0;
    format!("{:.2} ETR", etr)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================");
    println!("Watchtower Incentive System Demo");
    println!("========================================\n");

    let mut manager = WatchtowerManager::new();
    let mut timestamp = 1234567890u64;

    // ============================================================
    // Step 1: Register Watchtowers
    // ============================================================
    println!("Step 1: Registering Watchtowers");
    println!("----------------------------------------");

    let watchtowers = vec![
        ("alice_watchtower", MIN_WATCHTOWER_STAKE * 5),
        ("bob_watchtower", MIN_WATCHTOWER_STAKE * 3),
        ("charlie_watchtower", MIN_WATCHTOWER_STAKE * 2),
    ];

    for (name, stake) in &watchtowers {
        manager.register_watchtower(
            name.to_string(),
            *stake,
            timestamp,
        )?;
        println!("✓ Registered: {} with {} stake", name, format_etr(*stake));
        timestamp += 1;
    }
    println!();

    // ============================================================
    // Step 2: Subscribe Watchtowers to Channels
    // ============================================================
    println!("Step 2: Subscribing to Channels");
    println!("----------------------------------------");

    let channels = vec![
        ("channel_001", "alice_watchtower", "user_1", 100_000_000_000_000_000_000u128),
        ("channel_002", "alice_watchtower", "user_2", 150_000_000_000_000_000_000u128),
        ("channel_003", "bob_watchtower", "user_3", 100_000_000_000_000_000_000u128),
        ("channel_004", "bob_watchtower", "user_4", 100_000_000_000_000_000_000u128),
        ("channel_005", "charlie_watchtower", "user_5", 200_000_000_000_000_000_000u128),
    ];

    for (channel, watchtower, subscriber, fee) in &channels {
        manager.subscribe_watchtower(
            channel.to_string(),
            watchtower.to_string(),
            subscriber.to_string(),
            *fee,
            timestamp,
        )?;
        println!("✓ {} monitoring {} (fee: {})", watchtower, channel, format_etr(*fee));
        timestamp += 1;
    }
    println!();

    // ============================================================
    // Step 3: Display Watchtower Status
    // ============================================================
    println!("Step 3: Watchtower Status");
    println!("----------------------------------------");

    for (name, _) in &watchtowers {
        let info = manager.get_watchtower(name)?;
        println!("Watchtower: {}", name);
        println!("  Stake:              {}", format_etr(info.stake));
        println!("  Reward Pool:        {}", format_etr(info.reward_pool));
        println!("  Channels Monitored: {}", info.channels_monitored);
        println!("  Disputes Resolved:  {}", info.disputes_resolved);
        println!("  Reputation:         {}", info.reputation_score);
        println!("  Monitoring Capacity: {}", info.monitoring_capacity());
        println!("  Status:             {}", if info.active { "ACTIVE" } else { "INACTIVE" });
        println!();
    }

    // ============================================================
    // Step 4: Fraud Detection and Reporting
    // ============================================================
    println!("Step 4: Fraud Detection and Reporting");
    println!("----------------------------------------");

    // Alice's watchtower detects fraud on channel_001
    let evidence = FraudEvidence::new(
        "channel_001".to_string(),
        "alice_watchtower".to_string(),
        vec![0x01, 0x02, 0x03, 0x04, 0x05], // Mock evidence
        100, // nonce
        5_000_000_000_000_000_000_000, // claimed balance A (5,000 ETR)
        5_000_000_000_000_000_000_000, // claimed balance B (5,000 ETR)
        timestamp,
    )?;

    let disputed_amount = 10_000_000_000_000_000_000_000u128; // 10,000 ETR
    let report_id = manager.report_fraud(
        evidence,
        disputed_amount,
        "malicious_user_1".to_string(),
        timestamp,
    )?;

    println!("✓ Fraud detected on channel_001");
    println!("  Report ID: {}", report_id);
    println!("  Disputed Amount: {}", format_etr(disputed_amount));

    let expected_reward = WATCHTOWER_BASE_REWARD + (disputed_amount * 10) / 100;
    println!("  Reward Paid: {}", format_etr(expected_reward));
    println!("  Reported By: alice_watchtower");
    println!();

    // Bob's watchtower detects fraud on channel_003
    let evidence2 = FraudEvidence::new(
        "channel_003".to_string(),
        "bob_watchtower".to_string(),
        vec![0x10, 0x20, 0x30, 0x40],
        200,
        3_000_000_000_000_000_000_000,
        7_000_000_000_000_000_000_000,
        timestamp + 1,
    )?;

    let disputed_amount2 = 8_000_000_000_000_000_000_000u128; // 8,000 ETR
    let report_id2 = manager.report_fraud(
        evidence2,
        disputed_amount2,
        "malicious_user_3".to_string(),
        timestamp + 1,
    )?;

    println!("✓ Fraud detected on channel_003");
    println!("  Report ID: {}", report_id2);
    println!("  Disputed Amount: {}", format_etr(disputed_amount2));

    let expected_reward2 = WATCHTOWER_BASE_REWARD + (disputed_amount2 * 10) / 100;
    println!("  Reward Paid: {}", format_etr(expected_reward2));
    println!("  Reported By: bob_watchtower");
    println!();

    timestamp += 2;

    // ============================================================
    // Step 5: Updated Watchtower Status After Fraud Detection
    // ============================================================
    println!("Step 5: Updated Status (After Fraud Detection)");
    println!("----------------------------------------");

    for (name, _) in &watchtowers {
        let info = manager.get_watchtower(name)?;
        if info.disputes_resolved > 0 {
            println!("Watchtower: {}", name);
            println!("  Disputes Resolved:  {} ⭐", info.disputes_resolved);
            println!("  Reputation:         {} (+10 per resolution)", info.reputation_score);
            println!("  Total Rewards:      {}", format_etr(info.reward_pool));
            println!();
        }
    }

    // ============================================================
    // Step 6: Slashing for Misbehavior
    // ============================================================
    println!("Step 6: Slashing for Misbehavior");
    println!("----------------------------------------");

    // Simulate charlie_watchtower submitting false report (caught by network)
    println!("⚠️  charlie_watchtower submitted false fraud report");

    let slash_amount = 500_000_000_000_000_000_000u128; // 500 ETR
    let slashed = manager.slash_watchtower("charlie_watchtower", slash_amount)?;

    println!("✓ Slashed {} from charlie_watchtower", format_etr(slashed));

    let charlie_info = manager.get_watchtower("charlie_watchtower")?;
    println!("  New Stake: {}", format_etr(charlie_info.stake));
    println!("  New Reputation: {} (-50 penalty)", charlie_info.reputation_score);
    println!("  Status: {}", if charlie_info.active { "ACTIVE" } else { "INACTIVE ❌" });
    println!();

    // ============================================================
    // Step 7: Network Statistics
    // ============================================================
    println!("Step 7: Network Statistics");
    println!("----------------------------------------");

    let stats = manager.get_statistics();
    println!("Network Overview:");
    println!("  Total Watchtowers:      {}", stats.total_watchtowers);
    println!("  Active Watchtowers:     {}", stats.active_watchtowers);
    println!("  Total Staked:           {}", format_etr(stats.total_staked));
    println!("  Channels Monitored:     {}", stats.total_channels_monitored);
    println!("  Disputes Resolved:      {}", stats.total_disputes_resolved);
    println!("  Fraud Reports Filed:    {}", stats.total_fraud_reports);
    println!();

    // ============================================================
    // Step 8: Channel Subscription Details
    // ============================================================
    println!("Step 8: Channel Subscription Details");
    println!("----------------------------------------");

    for (channel, _, _, _) in &channels {
        let subs = manager.get_channel_subscriptions(channel);
        if !subs.is_empty() {
            println!("Channel: {}", channel);
            for sub in subs {
                println!("  Watchtower: {}", sub.watchtower);
                println!("  Subscriber: {}", sub.subscriber);
                println!("  Fee Paid:   {}", format_etr(sub.fee_paid));
                println!("  Status:     {}", if sub.active { "ACTIVE" } else { "INACTIVE" });
            }
            println!();
        }
    }

    // ============================================================
    // Summary
    // ============================================================
    println!("========================================");
    println!("Summary");
    println!("========================================");
    println!();
    println!("✓ Successfully demonstrated complete watchtower lifecycle:");
    println!("  • 3 watchtowers registered with stakes");
    println!("  • 5 channel subscriptions created");
    println!("  • 2 fraud reports successfully processed");
    println!("  • 1 watchtower slashed for misbehavior");
    println!("  • Network statistics calculated");
    println!();
    println!("Economic Model Validation:");
    println!("  • Minimum Stake: {}", format_etr(MIN_WATCHTOWER_STAKE));
    println!("  • Base Reward: {}", format_etr(WATCHTOWER_BASE_REWARD));
    println!("  • Fraud Reward %: 10%");
    println!("  • Total Rewards Paid: {}", format_etr(expected_reward + expected_reward2));
    println!();

    Ok(())
}
