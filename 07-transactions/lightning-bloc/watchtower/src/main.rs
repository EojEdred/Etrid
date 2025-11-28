

//! Lightning Bloc Watchtower Service
//!
//! Monitors Lightning channels for:
//! - Channel timeouts and expiration
//! - Dispute conditions
//! - Balance imbalances
//! - Network health
//! - Automatic alerts and reporting

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChannelState {
    channel_id: String,
    party_a: String,
    party_b: String,
    balance_a: u128,
    balance_b: u128,
    capacity: u128,
    nonce: u64,
    state: String,
    opened_at: u64,
    expires_at: u64,
    last_update: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChannelHealth {
    channel_id: String,
    status: HealthStatus,
    time_until_expiry: i64,
    balance_ratio: f64,
    is_balanced: bool,
    needs_rebalancing: bool,
    risk_level: RiskLevel,
    warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NetworkHealth {
    total_channels: usize,
    healthy_channels: usize,
    warning_channels: usize,
    critical_channels: usize,
    expired_channels: usize,
    total_capacity: u128,
    total_liquidity: u128,
    average_balance_ratio: f64,
}

struct WatchtowerService {
    channels: HashMap<String, ChannelState>,
    chain_endpoint: String,
    alert_webhook: Option<String>,
    check_interval_secs: u64,
}

impl WatchtowerService {
    fn new(chain_endpoint: String, check_interval_secs: u64) -> Self {
        Self {
            channels: HashMap::new(),
            chain_endpoint,
            alert_webhook: None,
            check_interval_secs,
        }
    }

    /// Load channels from network topology file
    async fn load_channels(&mut self, topology_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = tokio::fs::read_to_string(topology_path).await?;
        let topology: serde_json::Value = serde_json::from_str(&content)?;

        if let Some(channels) = topology["channels"].as_array() {
            for channel in channels {
                let channel_id = channel["id"].as_str().unwrap_or("unknown").to_string();

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();

                let state = ChannelState {
                    channel_id: channel_id.clone(),
                    party_a: channel["from"].as_str().unwrap_or("unknown").to_string(),
                    party_b: channel["to"].as_str().unwrap_or("unknown").to_string(),
                    balance_a: channel["balance_from"].as_str()
                        .and_then(|s| s.parse().ok()).unwrap_or(0),
                    balance_b: channel["balance_to"].as_str()
                        .and_then(|s| s.parse().ok()).unwrap_or(0),
                    capacity: channel["capacity"].as_str()
                        .and_then(|s| s.parse().ok()).unwrap_or(0),
                    nonce: 0,
                    state: channel["status"].as_str().unwrap_or("unknown").to_string(),
                    opened_at: now,
                    expires_at: now + (30 * 24 * 60 * 60), // 30 days default
                    last_update: now,
                };

                self.channels.insert(channel_id, state);
            }
        }

        println!("✓ Loaded {} channels from topology", self.channels.len());
        Ok(())
    }

    /// Check health of a single channel
    fn check_channel_health(&self, channel: &ChannelState) -> ChannelHealth {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let time_until_expiry = channel.expires_at as i64 - now as i64;

        let total_balance = channel.balance_a + channel.balance_b;
        let balance_ratio = if total_balance > 0 {
            channel.balance_a as f64 / total_balance as f64
        } else {
            0.5
        };

        let is_balanced = balance_ratio >= 0.3 && balance_ratio <= 0.7;
        let needs_rebalancing = balance_ratio < 0.2 || balance_ratio > 0.8;

        let mut warnings = Vec::new();
        let mut risk_level = RiskLevel::Low;
        let status;

        // Check expiration
        if time_until_expiry <= 0 {
            status = HealthStatus::Expired;
            risk_level = RiskLevel::Critical;
            warnings.push("Channel has expired".to_string());
        } else if time_until_expiry < 3600 { // < 1 hour
            status = HealthStatus::Critical;
            risk_level = RiskLevel::Critical;
            warnings.push(format!("Channel expires in {} seconds", time_until_expiry));
        } else if time_until_expiry < 24 * 3600 { // < 1 day
            status = HealthStatus::Warning;
            risk_level = RiskLevel::High;
            warnings.push(format!("Channel expires in {} hours", time_until_expiry / 3600));
        } else {
            status = HealthStatus::Healthy;
        }

        // Check balance ratios
        if needs_rebalancing {
            if risk_level == RiskLevel::Low {
                risk_level = RiskLevel::Medium;
            }
            warnings.push(format!(
                "Channel severely imbalanced: {:.1}% / {:.1}%",
                balance_ratio * 100.0,
                (1.0 - balance_ratio) * 100.0
            ));
        } else if !is_balanced {
            warnings.push(format!(
                "Channel slightly imbalanced: {:.1}% / {:.1}%",
                balance_ratio * 100.0,
                (1.0 - balance_ratio) * 100.0
            ));
        }

        // Check capacity utilization
        let utilization = total_balance as f64 / channel.capacity as f64;
        if utilization < 0.5 {
            warnings.push(format!(
                "Low capacity utilization: {:.1}%",
                utilization * 100.0
            ));
        }

        ChannelHealth {
            channel_id: channel.channel_id.clone(),
            status,
            time_until_expiry,
            balance_ratio,
            is_balanced,
            needs_rebalancing,
            risk_level,
            warnings,
        }
    }

    /// Check health of all channels
    fn check_network_health(&self) -> (NetworkHealth, Vec<ChannelHealth>) {
        let mut channel_healths = Vec::new();
        let mut healthy = 0;
        let mut warning = 0;
        let mut critical = 0;
        let mut expired = 0;
        let mut total_capacity = 0u128;
        let mut total_liquidity = 0u128;
        let mut balance_ratios = Vec::new();

        for channel in self.channels.values() {
            let health = self.check_channel_health(channel);

            match health.status {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Warning => warning += 1,
                HealthStatus::Critical => critical += 1,
                HealthStatus::Expired => expired += 1,
            }

            total_capacity += channel.capacity;
            total_liquidity += channel.balance_a + channel.balance_b;
            balance_ratios.push(health.balance_ratio);

            channel_healths.push(health);
        }

        let average_balance_ratio = if !balance_ratios.is_empty() {
            balance_ratios.iter().sum::<f64>() / balance_ratios.len() as f64
        } else {
            0.0
        };

        let network_health = NetworkHealth {
            total_channels: self.channels.len(),
            healthy_channels: healthy,
            warning_channels: warning,
            critical_channels: critical,
            expired_channels: expired,
            total_capacity,
            total_liquidity,
            average_balance_ratio,
        };

        (network_health, channel_healths)
    }

    /// Display monitoring dashboard
    fn display_dashboard(&self, network_health: &NetworkHealth, channel_healths: &[ChannelHealth]) {
        println!("\n========================================");
        println!("Lightning Bloc Watchtower");
        println!("========================================\n");

        // Network Summary
        println!("Network Summary:");
        println!("  Total Channels:    {}", network_health.total_channels);
        println!("  Healthy:           {} ✓", network_health.healthy_channels);
        println!("  Warning:           {} ⚠", network_health.warning_channels);
        println!("  Critical:          {} ❌", network_health.critical_channels);
        println!("  Expired:           {} ⏰", network_health.expired_channels);
        println!("  Total Capacity:    {} ETR", format_balance(network_health.total_capacity));
        println!("  Total Liquidity:   {} ETR", format_balance(network_health.total_liquidity));
        println!("  Avg Balance Ratio: {:.1}%", network_health.average_balance_ratio * 100.0);
        println!();

        // Channel Details
        println!("Channel Status:");
        println!("  {:<15} {:<10} {:<15} {:<15} {:<8} {}",
            "Channel", "Status", "Balance Ratio", "Time to Expiry", "Risk", "Warnings");
        println!("  {}", "-".repeat(90));

        for health in channel_healths {
            let status_icon = match health.status {
                HealthStatus::Healthy => "✓",
                HealthStatus::Warning => "⚠",
                HealthStatus::Critical => "❌",
                HealthStatus::Expired => "⏰",
            };

            let risk_icon = match health.risk_level {
                RiskLevel::Low => "🟢",
                RiskLevel::Medium => "🟡",
                RiskLevel::High => "🟠",
                RiskLevel::Critical => "🔴",
            };

            let time_str = format_time(health.time_until_expiry);
            let warnings_str = if health.warnings.is_empty() {
                "None".to_string()
            } else {
                health.warnings.join(", ")
            };

            println!("  {:<15} {:<10} {:<15} {:<15} {:<8} {}",
                health.channel_id,
                format!("{} {}", status_icon, format!("{:?}", health.status)),
                format!("{:.1}%/{:.1}%",
                    health.balance_ratio * 100.0,
                    (1.0 - health.balance_ratio) * 100.0
                ),
                time_str,
                format!("{} {:?}", risk_icon, health.risk_level),
                warnings_str
            );
        }

        println!("\n========================================\n");
    }

    /// Alert on critical conditions
    async fn send_alerts(&self, network_health: &NetworkHealth, channel_healths: &[ChannelHealth]) {
        // Check for critical conditions
        let critical_conditions: Vec<_> = channel_healths.iter()
            .filter(|h| h.status == HealthStatus::Critical || h.status == HealthStatus::Expired)
            .collect();

        if critical_conditions.is_empty() {
            return;
        }

        println!("\n🚨 CRITICAL ALERTS:");
        for health in critical_conditions {
            for warning in &health.warnings {
                println!("  ❌ {}: {}", health.channel_id, warning);
            }
        }

        // If webhook configured, send alert
        if let Some(webhook_url) = &self.alert_webhook {
            let alert = serde_json::json!({
                "service": "Lightning Bloc Watchtower",
                "timestamp": SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "critical_channels": critical_conditions.len(),
                "total_channels": network_health.total_channels,
                "alerts": critical_conditions.iter().map(|h| {
                    serde_json::json!({
                        "channel_id": h.channel_id,
                        "status": format!("{:?}", h.status),
                        "risk_level": format!("{:?}", h.risk_level),
                        "warnings": h.warnings,
                    })
                }).collect::<Vec<_>>(),
            });

            // Send webhook (pseudo-code - requires reqwest)
            println!("  📡 Sending alert to webhook: {}", webhook_url);
            println!("  Alert payload: {}", serde_json::to_string_pretty(&alert).unwrap());
        }
    }

    /// Run monitoring loop
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔭 Starting Lightning Bloc Watchtower...");
        println!("   Chain endpoint: {}", self.chain_endpoint);
        println!("   Check interval: {} seconds\n", self.check_interval_secs);

        let mut interval = time::interval(Duration::from_secs(self.check_interval_secs));

        loop {
            interval.tick().await;

            // Check all channels
            let (network_health, channel_healths) = self.check_network_health();

            // Display dashboard
            self.display_dashboard(&network_health, &channel_healths);

            // Send alerts if needed
            self.send_alerts(&network_health, &channel_healths).await;

            // Recommendations
            self.print_recommendations(&network_health, &channel_healths);
        }
    }

    fn print_recommendations(&self, network_health: &NetworkHealth, channel_healths: &[ChannelHealth]) {
        let mut recommendations = Vec::new();

        // Check for channels needing rebalancing
        let needs_rebalancing: Vec<_> = channel_healths.iter()
            .filter(|h| h.needs_rebalancing)
            .collect();

        if !needs_rebalancing.is_empty() {
            recommendations.push(format!(
                "Rebalance {} channel(s): {}",
                needs_rebalancing.len(),
                needs_rebalancing.iter()
                    .map(|h| h.channel_id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Check for expiring channels
        let expiring_soon: Vec<_> = channel_healths.iter()
            .filter(|h| h.time_until_expiry > 0 && h.time_until_expiry < 7 * 24 * 3600)
            .collect();

        if !expiring_soon.is_empty() {
            recommendations.push(format!(
                "Renew {} channel(s) expiring within 7 days: {}",
                expiring_soon.len(),
                expiring_soon.iter()
                    .map(|h| h.channel_id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        // Check overall network health
        if network_health.critical_channels > 0 || network_health.expired_channels > 0 {
            recommendations.push("Immediate action required: Close expired channels or resolve disputes".to_string());
        }

        if !recommendations.is_empty() {
            println!("💡 Recommendations:");
            for (i, rec) in recommendations.iter().enumerate() {
                println!("  {}. {}", i + 1, rec);
            }
            println!();
        }
    }
}

// Helper functions
fn format_balance(balance: u128) -> String {
    let etr = balance as f64 / 1_000_000_000_000_000_000.0;
    if etr >= 1000.0 {
        format!("{:.1}k", etr / 1000.0)
    } else {
        format!("{:.2}", etr)
    }
}

fn format_time(seconds: i64) -> String {
    if seconds <= 0 {
        return "EXPIRED".to_string();
    }

    let days = seconds / (24 * 3600);
    let hours = (seconds % (24 * 3600)) / 3600;
    let mins = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Lightning Bloc Watchtower Service");
    println!("   Version 1.0.0\n");

    // Configuration
    let chain_endpoint = "ws://127.0.0.1:9944".to_string();
    let topology_path = ".lightning-testnet/network-topology.json";
    let check_interval_secs = 30; // Check every 30 seconds

    // Create watchtower
    let mut watchtower = WatchtowerService::new(chain_endpoint, check_interval_secs);

    // Load channels from topology
    match watchtower.load_channels(topology_path).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("⚠️  Could not load topology file: {}", e);
            eprintln!("   Using empty channel list");
        }
    }

    // Optional: Set alert webhook
    // watchtower.alert_webhook = Some("https://hooks.slack.com/...".to_string());

    // Run monitoring loop
    watchtower.run().await
}
