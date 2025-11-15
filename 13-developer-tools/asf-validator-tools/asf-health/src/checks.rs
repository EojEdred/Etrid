use crate::rpc::*;
use crate::{HealthCheck, CheckStatus};
use serde_json::json;
use sysinfo::{System, SystemExt, CpuExt, DiskExt};

// ═══════════════════════════════════════════════════════════════════════════════
// NODE CONNECTIVITY CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_node_connectivity(rpc_url: &str, verbose: bool) -> HealthCheck {
    let client = RpcClient::new(rpc_url.to_string());

    match client.ping().await {
        Ok(latency_ms) => {
            let message = format!("Node is reachable (latency: {}ms)", latency_ms);
            if verbose {
                println!("   ✓ {}", message);
            }

            HealthCheck {
                name: "Node Connectivity".to_string(),
                status: CheckStatus::Pass,
                message,
                details: Some(json!({
                    "endpoint": rpc_url,
                    "latency_ms": latency_ms,
                })),
            }
        }
        Err(e) => {
            let message = format!("Failed to connect to node: {}", e);
            if verbose {
                println!("   ✗ {}", message);
            }

            HealthCheck {
                name: "Node Connectivity".to_string(),
                status: CheckStatus::Fail,
                message,
                details: Some(json!({
                    "endpoint": rpc_url,
                    "error": e.to_string(),
                })),
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEY ACCESSIBILITY CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_key_accessibility(validator: &str, verbose: bool) -> HealthCheck {
    // In production, this would check if the validator's keys are accessible
    // For now, we'll do a simple validation of the address format

    if validator.len() >= 32 {
        let message = "Validator address format is valid".to_string();
        if verbose {
            println!("   ✓ {}", message);
        }

        HealthCheck {
            name: "Key Accessibility".to_string(),
            status: CheckStatus::Pass,
            message,
            details: Some(json!({
                "validator": validator,
            })),
        }
    } else {
        let message = "Invalid validator address format".to_string();
        if verbose {
            println!("   ✗ {}", message);
        }

        HealthCheck {
            name: "Key Accessibility".to_string(),
            status: CheckStatus::Fail,
            message,
            details: Some(json!({
                "validator": validator,
            })),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STAKE CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_stake(rpc_url: &str, validator: &str, verbose: bool) -> HealthCheck {
    let client = RpcClient::new(rpc_url.to_string());

    match client.get_validator_stake(validator).await {
        Ok(stake) => {
            let min_stake = 100_000_000_000_000u128; // 100,000 ETR minimum

            let status = if stake >= min_stake {
                CheckStatus::Pass
            } else if stake > 0 {
                CheckStatus::Warning
            } else {
                CheckStatus::Fail
            };

            let message = format!("Stake: {} ETR (minimum: {} ETR)",
                stake / 1_000_000_000_000,
                min_stake / 1_000_000_000_000
            );

            if verbose {
                let symbol = match status {
                    CheckStatus::Pass => "✓",
                    CheckStatus::Warning => "⚠",
                    CheckStatus::Fail => "✗",
                };
                println!("   {} {}", symbol, message);
            }

            HealthCheck {
                name: "Stake Verification".to_string(),
                status,
                message,
                details: Some(json!({
                    "stake_raw": stake.to_string(),
                    "stake_etr": (stake / 1_000_000_000_000).to_string(),
                    "minimum_stake": (min_stake / 1_000_000_000_000).to_string(),
                    "meets_minimum": stake >= min_stake,
                })),
            }
        }
        Err(e) => {
            let message = format!("Failed to check stake: {}", e);
            if verbose {
                println!("   ✗ {}", message);
            }

            HealthCheck {
                name: "Stake Verification".to_string(),
                status: CheckStatus::Fail,
                message,
                details: Some(json!({
                    "error": e.to_string(),
                })),
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SLASHING STATUS CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_slashing_status(rpc_url: &str, validator: &str, verbose: bool) -> HealthCheck {
    let client = RpcClient::new(rpc_url.to_string());

    match client.is_slashed(validator).await {
        Ok(is_slashed) => {
            let status = if is_slashed {
                CheckStatus::Fail
            } else {
                CheckStatus::Pass
            };

            let message = if is_slashed {
                "Validator is SLASHED!".to_string()
            } else {
                "Validator is not slashed".to_string()
            };

            if verbose {
                let symbol = if is_slashed { "✗" } else { "✓" };
                println!("   {} {}", symbol, message);
            }

            HealthCheck {
                name: "Slashing Status".to_string(),
                status,
                message,
                details: Some(json!({
                    "is_slashed": is_slashed,
                })),
            }
        }
        Err(e) => {
            let message = format!("Failed to check slashing status: {}", e);
            if verbose {
                println!("   ✗ {}", message);
            }

            HealthCheck {
                name: "Slashing Status".to_string(),
                status: CheckStatus::Warning,
                message,
                details: Some(json!({
                    "error": e.to_string(),
                })),
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// P2P PEER COUNT CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_peer_count(rpc_url: &str, verbose: bool) -> HealthCheck {
    let client = RpcClient::new(rpc_url.to_string());

    match client.get_peer_count().await {
        Ok(peer_count) => {
            let status = if peer_count >= 10 {
                CheckStatus::Pass
            } else if peer_count >= 3 {
                CheckStatus::Warning
            } else {
                CheckStatus::Fail
            };

            let message = format!("Connected peers: {} (recommended: 10+)", peer_count);

            if verbose {
                let symbol = match status {
                    CheckStatus::Pass => "✓",
                    CheckStatus::Warning => "⚠",
                    CheckStatus::Fail => "✗",
                };
                println!("   {} {}", symbol, message);
            }

            HealthCheck {
                name: "P2P Peer Count".to_string(),
                status,
                message,
                details: Some(json!({
                    "peer_count": peer_count,
                    "recommended_minimum": 10,
                })),
            }
        }
        Err(e) => {
            let message = format!("Failed to check peer count: {}", e);
            if verbose {
                println!("   ✗ {}", message);
            }

            HealthCheck {
                name: "P2P Peer Count".to_string(),
                status: CheckStatus::Fail,
                message,
                details: Some(json!({
                    "error": e.to_string(),
                })),
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYNC STATUS CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_sync_status(rpc_url: &str, verbose: bool) -> HealthCheck {
    let client = RpcClient::new(rpc_url.to_string());

    match client.get_sync_status().await {
        Ok((is_syncing, current_block, highest_block)) => {
            let status = if !is_syncing {
                CheckStatus::Pass
            } else {
                let blocks_behind = highest_block.saturating_sub(current_block);
                if blocks_behind < 10 {
                    CheckStatus::Warning
                } else {
                    CheckStatus::Fail
                }
            };

            let message = if !is_syncing {
                format!("Node is fully synced (block #{})", current_block)
            } else {
                let blocks_behind = highest_block.saturating_sub(current_block);
                format!("Node is syncing ({} blocks behind)", blocks_behind)
            };

            if verbose {
                let symbol = match status {
                    CheckStatus::Pass => "✓",
                    CheckStatus::Warning => "⚠",
                    CheckStatus::Fail => "✗",
                };
                println!("   {} {}", symbol, message);
            }

            HealthCheck {
                name: "Sync Status".to_string(),
                status,
                message,
                details: Some(json!({
                    "is_syncing": is_syncing,
                    "current_block": current_block,
                    "highest_block": highest_block,
                    "blocks_behind": highest_block.saturating_sub(current_block),
                })),
            }
        }
        Err(e) => {
            let message = format!("Failed to check sync status: {}", e);
            if verbose {
                println!("   ✗ {}", message);
            }

            HealthCheck {
                name: "Sync Status".to_string(),
                status: CheckStatus::Fail,
                message,
                details: Some(json!({
                    "error": e.to_string(),
                })),
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYSTEM RESOURCES CHECK
// ═══════════════════════════════════════════════════════════════════════════════

pub async fn check_system_resources(verbose: bool) -> HealthCheck {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU check
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let cpu_ok = cpu_usage < 90.0;

    // Memory check
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
    let memory_ok = memory_usage_percent < 90.0;

    // Disk check
    let mut disk_ok = true;
    let mut disk_usage_percent = 0.0;

    for disk in sys.disks() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total - available;
        let usage = (used as f64 / total as f64) * 100.0;

        if usage > disk_usage_percent {
            disk_usage_percent = usage;
        }

        if usage > 90.0 {
            disk_ok = false;
        }
    }

    let all_ok = cpu_ok && memory_ok && disk_ok;
    let status = if all_ok {
        CheckStatus::Pass
    } else if cpu_usage > 95.0 || memory_usage_percent > 95.0 || disk_usage_percent > 95.0 {
        CheckStatus::Fail
    } else {
        CheckStatus::Warning
    };

    let message = format!(
        "CPU: {:.1}% | Memory: {:.1}% | Disk: {:.1}%",
        cpu_usage, memory_usage_percent, disk_usage_percent
    );

    if verbose {
        let symbol = match status {
            CheckStatus::Pass => "✓",
            CheckStatus::Warning => "⚠",
            CheckStatus::Fail => "✗",
        };
        println!("   {} {}", symbol, message);
    }

    HealthCheck {
        name: "System Resources".to_string(),
        status,
        message,
        details: Some(json!({
            "cpu_usage_percent": cpu_usage,
            "memory_usage_percent": memory_usage_percent,
            "memory_total_gb": total_memory / (1024 * 1024 * 1024),
            "memory_used_gb": used_memory / (1024 * 1024 * 1024),
            "disk_usage_percent": disk_usage_percent,
        })),
    }
}
