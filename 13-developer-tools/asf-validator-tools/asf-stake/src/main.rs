use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod rpc;
mod transactions;

use rpc::*;
use transactions::*;

// ═══════════════════════════════════════════════════════════════════════════════
// CLI STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "asf-stake")]
#[command(author = "Ëtrid Foundation")]
#[command(version = "0.1.0")]
#[command(about = "Staking management for ËTRID ASF validators")]
struct Cli {
    /// RPC endpoint URL
    #[arg(short, long, default_value = "http://localhost:9944")]
    rpc: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bond ETR tokens to a validator
    Bond {
        /// Amount of ETR to bond
        #[arg(short, long)]
        amount: String,

        /// Validator address to bond to
        #[arg(short, long)]
        validator: String,

        /// Key file for signing
        #[arg(short, long)]
        keyfile: PathBuf,

        /// Password for key file
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Unbond ETR tokens from a validator
    Unbond {
        /// Amount of ETR to unbond
        #[arg(short, long)]
        amount: String,

        /// Validator address
        #[arg(short, long)]
        validator: String,

        /// Key file for signing
        #[arg(short, long)]
        keyfile: PathBuf,

        /// Password for key file
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Show staking status for a validator
    Status {
        /// Validator address
        #[arg(short, long)]
        validator: String,

        /// Show detailed information
        #[arg(short = 'd', long)]
        detailed: bool,
    },

    /// Show rewards earned by a validator
    Rewards {
        /// Validator address
        #[arg(short, long)]
        validator: String,

        /// Number of epochs to show
        #[arg(short, long, default_value = "10")]
        epochs: u32,
    },

    /// Claim pending rewards
    Claim {
        /// Validator address
        #[arg(short, long)]
        validator: String,

        /// Key file for signing
        #[arg(short, long)]
        keyfile: PathBuf,

        /// Password for key file
        #[arg(short, long)]
        password: Option<String>,
    },

    /// List all validators and their staking info
    List {
        /// Sort by field (stake, reputation, rewards)
        #[arg(short, long, default_value = "stake")]
        sort: String,

        /// Limit number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ENTRY POINT
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let rpc_client = RpcClient::new(cli.rpc);

    match cli.command {
        Commands::Bond {
            amount,
            validator,
            keyfile,
            password,
        } => {
            let amount_value = parse_amount(&amount)?;
            let password = get_password(password)?;
            bond_tokens(&rpc_client, amount_value, &validator, &keyfile, &password).await?;
        }

        Commands::Unbond {
            amount,
            validator,
            keyfile,
            password,
        } => {
            let amount_value = parse_amount(&amount)?;
            let password = get_password(password)?;
            unbond_tokens(&rpc_client, amount_value, &validator, &keyfile, &password).await?;
        }

        Commands::Status { validator, detailed } => {
            show_status(&rpc_client, &validator, detailed).await?;
        }

        Commands::Rewards { validator, epochs } => {
            show_rewards(&rpc_client, &validator, epochs).await?;
        }

        Commands::Claim {
            validator,
            keyfile,
            password,
        } => {
            let password = get_password(password)?;
            claim_rewards(&rpc_client, &validator, &keyfile, &password).await?;
        }

        Commands::List { sort, limit } => {
            list_validators(&rpc_client, &sort, limit).await?;
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMAND IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

async fn bond_tokens(
    client: &RpcClient,
    amount: u128,
    validator: &str,
    keyfile: &PathBuf,
    password: &str,
) -> Result<()> {
    println!("{}", "Bonding tokens to validator...".cyan());
    println!("  Validator: {}", validator);
    println!("  Amount: {} ETR", format_etr(amount));

    // Create and sign transaction
    let tx_hash = submit_bond_transaction(client, amount, validator, keyfile, password).await?;

    println!("\n{} Transaction submitted!", "✓".green().bold());
    println!("  Tx Hash: {}", tx_hash);
    println!("\n{}", "Waiting for confirmation...".yellow());

    // Wait for transaction confirmation
    wait_for_confirmation(client, &tx_hash).await?;

    println!("{} Tokens bonded successfully!", "✓".green().bold());
    Ok(())
}

async fn unbond_tokens(
    client: &RpcClient,
    amount: u128,
    validator: &str,
    keyfile: &PathBuf,
    password: &str,
) -> Result<()> {
    println!("{}", "Unbonding tokens from validator...".cyan());
    println!("  Validator: {}", validator);
    println!("  Amount: {} ETR", format_etr(amount));

    // Create and sign transaction
    let tx_hash = submit_unbond_transaction(client, amount, validator, keyfile, password).await?;

    println!("\n{} Transaction submitted!", "✓".green().bold());
    println!("  Tx Hash: {}", tx_hash);
    println!("\n{}", "Waiting for confirmation...".yellow());

    // Wait for transaction confirmation
    wait_for_confirmation(client, &tx_hash).await?;

    println!("{} Tokens unbonded successfully!", "✓".green().bold());
    println!("\n{}", "Note: Unbonded tokens have a 7-day unbonding period".yellow());
    Ok(())
}

async fn show_status(client: &RpcClient, validator: &str, detailed: bool) -> Result<()> {
    println!("{}", "Fetching validator staking status...".cyan());

    let status = client.get_validator_stake_info(validator).await?;

    println!("\n{}", "Validator Staking Status".cyan().bold());
    println!("═══════════════════════════════════════");
    println!("Validator: {}", validator);
    println!("Total Stake: {} ETR", format_etr(status.total_stake));
    println!("Self Stake: {} ETR", format_etr(status.self_stake));
    println!("Delegated Stake: {} ETR", format_etr(status.delegated_stake));
    println!("Active: {}", if status.is_active { "Yes".green() } else { "No".red() });
    println!("In Committee: {}", if status.in_committee { "Yes".green() } else { "No".yellow() });

    if detailed {
        println!("\n{}", "Detailed Information".cyan().bold());
        println!("═══════════════════════════════════════");
        println!("Commission: {}%", status.commission);
        println!("Total Delegators: {}", status.delegator_count);
        println!("Pending Rewards: {} ETR", format_etr(status.pending_rewards));
        println!("Last Reward Claim: Epoch {}", status.last_claim_epoch);
        println!("Reputation Score: {}/100", status.reputation);
    }

    Ok(())
}

async fn show_rewards(client: &RpcClient, validator: &str, epochs: u32) -> Result<()> {
    println!("{}", format!("Fetching reward history (last {} epochs)...", epochs).cyan());

    let rewards = client.get_validator_rewards(validator, epochs).await?;
    let total: u128 = rewards.iter().map(|r| r.amount).sum();

    println!("\n{}", "Validator Reward History".cyan().bold());
    println!("═══════════════════════════════════════");
    println!("Validator: {}", validator);
    println!("Total Rewards (last {} epochs): {} ETR\n", epochs, format_etr(total));

    println!("{:<8} {:<15} {:<15} {:<10}", "Epoch", "Rewards (ETR)", "Blocks", "Status");
    println!("{}", "─".repeat(60));

    for reward in rewards.iter().rev() {
        println!(
            "{:<8} {:<15} {:<15} {:<10}",
            reward.epoch,
            format_etr(reward.amount),
            reward.blocks_produced,
            if reward.claimed { "Claimed".green() } else { "Pending".yellow() }
        );
    }

    Ok(())
}

async fn claim_rewards(
    client: &RpcClient,
    validator: &str,
    keyfile: &PathBuf,
    password: &str,
) -> Result<()> {
    println!("{}", "Claiming pending rewards...".cyan());

    // Get pending rewards amount
    let status = client.get_validator_stake_info(validator).await?;
    println!("  Pending Rewards: {} ETR", format_etr(status.pending_rewards));

    if status.pending_rewards == 0 {
        println!("{}", "No pending rewards to claim".yellow());
        return Ok(());
    }

    // Create and sign transaction
    let tx_hash = submit_claim_transaction(client, validator, keyfile, password).await?;

    println!("\n{} Transaction submitted!", "✓".green().bold());
    println!("  Tx Hash: {}", tx_hash);
    println!("\n{}", "Waiting for confirmation...".yellow());

    // Wait for transaction confirmation
    wait_for_confirmation(client, &tx_hash).await?;

    println!("{} Rewards claimed successfully!", "✓".green().bold());
    println!("  Amount: {} ETR", format_etr(status.pending_rewards));
    Ok(())
}

async fn list_validators(client: &RpcClient, sort_by: &str, limit: usize) -> Result<()> {
    println!("{}", "Fetching validator list...".cyan());

    let mut validators = client.get_all_validators().await?;

    // Sort validators
    match sort_by {
        "stake" => validators.sort_by(|a, b| b.total_stake.cmp(&a.total_stake)),
        "reputation" => validators.sort_by(|a, b| b.reputation.cmp(&a.reputation)),
        "rewards" => validators.sort_by(|a, b| b.pending_rewards.cmp(&a.pending_rewards)),
        _ => {}
    }

    // Take only the requested limit
    validators.truncate(limit);

    println!("\n{}", "Active Validators".cyan().bold());
    println!("═══════════════════════════════════════════════════════════════════════");
    println!(
        "{:<10} {:<42} {:<15} {:<8} {:<10}",
        "Rank", "Validator", "Stake (ETR)", "Rep", "Committee"
    );
    println!("{}", "─".repeat(100));

    for (i, validator) in validators.iter().enumerate() {
        let committee_status = if validator.in_committee { "✓" } else { "-" };
        let address_short = format!(
            "{}...{}",
            &validator.address[..8],
            &validator.address[validator.address.len() - 6..]
        );

        println!(
            "{:<10} {:<42} {:<15} {:<8} {:<10}",
            i + 1,
            address_short,
            format_etr(validator.total_stake),
            validator.reputation,
            committee_status
        );
    }

    println!("\n{} Total validators: {}", "ℹ".cyan(), validators.len());
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

fn parse_amount(amount: &str) -> Result<u128> {
    let value: f64 = amount.parse().context("Invalid amount format")?;
    Ok((value * 1_000_000_000_000.0) as u128) // Convert to base units
}

fn format_etr(amount: u128) -> String {
    let etr = amount as f64 / 1_000_000_000_000.0;
    format!("{:.4}", etr)
}

fn get_password(password: Option<String>) -> Result<String> {
    match password {
        Some(p) => Ok(p),
        None => {
            let password = rpassword::prompt_password("Enter password: ".cyan().to_string())
                .context("Failed to read password")?;
            Ok(password)
        }
    }
}

use rpassword;
