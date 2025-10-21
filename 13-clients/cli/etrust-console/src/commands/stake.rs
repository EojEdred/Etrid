// ═══════════════════════════════════════════════════════════════════════════════
// Staking Commands
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::Result;
use colored::Colorize;

use crate::{cli::StakeCommands, rpc_client::EtridRpcClient};

pub async fn execute(command: StakeCommands, endpoint: &str) -> Result<()> {
    match command {
        StakeCommands::Stake {
            amount,
            validator,
            from,
        } => stake_tokens(amount, validator, from, endpoint).await,
        StakeCommands::Unstake { amount, from } => unstake_tokens(amount, from, endpoint).await,
        StakeCommands::Info { account } => show_staking_info(account, endpoint).await,
        StakeCommands::Validators => list_validators(endpoint).await,
        StakeCommands::Nominate {
            validator,
            amount,
            from,
        } => nominate_validator(validator, amount, from, endpoint).await,
    }
}

async fn stake_tokens(
    amount: String,
    validator: Option<String>,
    from: String,
    endpoint: &str,
) -> Result<()> {
    println!("{}", "Staking tokens...".bright_green().bold());
    println!();
    println!("  {}: {}", "Amount".bold(), amount.bright_white());
    println!("  {}: {}", "From".bold(), from.bright_white());

    if let Some(validator) = validator {
        println!("  {}: {}", "Nominating".bold(), validator.bright_white());
    } else {
        println!("  {}: {}", "Type".bold(), "Self-staking".bright_white());
    }

    println!();
    println!("  {}: {}", "Endpoint".bold(), endpoint.bright_cyan());
    println!();

    // In a real implementation, this would:
    // 1. Connect to RPC
    // 2. Create and sign stake transaction
    // 3. Submit transaction
    // 4. Wait for finalization

    println!("{}", "Note: Transaction signing and submission requires full implementation".bright_yellow());
    println!("      This MVP provides the CLI structure and RPC client foundation.");
    println!();

    Ok(())
}

async fn unstake_tokens(amount: String, from: String, endpoint: &str) -> Result<()> {
    println!("{}", "Unstaking tokens...".bright_green().bold());
    println!();
    println!("  {}: {}", "Amount".bold(), amount.bright_white());
    println!("  {}: {}", "From".bold(), from.bright_white());
    println!("  {}: {}", "Endpoint".bold(), endpoint.bright_cyan());
    println!();

    println!("{}", "Note: Unstaking has a 28-day unbonding period on ËTRID".bright_yellow());
    println!();

    Ok(())
}

async fn show_staking_info(account: String, endpoint: &str) -> Result<()> {
    println!("{}", "Staking Information".bright_green().bold());
    println!();
    println!("  {}: {}", "Account".bold(), account.bright_white());
    println!();

    // Connect to RPC and query staking info
    match EtridRpcClient::new(endpoint).await {
        Ok(_client) => {
            println!("  {}: {}", "Status".bold(), "Connected".bright_green());
            println!();
            println!("  {} Querying staking data...", "→".bright_cyan());
            println!();
            println!("  {}", "Note: Staking queries require pallet-specific storage keys".bright_yellow());
            println!("        Full implementation coming soon.");
        }
        Err(e) => {
            println!("  {}: {}", "Status".bold(), "Connection failed".bright_red());
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
            println!();
            println!("  Make sure the ËTRID node is running at: {}", endpoint.bright_cyan());
        }
    }

    println!();
    Ok(())
}

async fn list_validators(endpoint: &str) -> Result<()> {
    println!("{}", "Active Validators".bright_green().bold());
    println!();

    match EtridRpcClient::new(endpoint).await {
        Ok(_client) => {
            println!("  {}: {}", "Status".bold(), "Connected".bright_green());
            println!();
            println!("  {} Querying validator set...", "→".bright_cyan());
            println!();
            println!("  {}", "Note: Validator queries require session pallet integration".bright_yellow());
            println!("        Full implementation coming soon.");
        }
        Err(e) => {
            println!("  {}: {}", "Status".bold(), "Connection failed".bright_red());
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}

async fn nominate_validator(
    validator: String,
    amount: String,
    from: String,
    endpoint: &str,
) -> Result<()> {
    println!("{}", "Nominating validator...".bright_green().bold());
    println!();
    println!("  {}: {}", "Validator".bold(), validator.bright_white());
    println!("  {}: {}", "Amount".bold(), amount.bright_white());
    println!("  {}: {}", "From".bold(), from.bright_white());
    println!("  {}: {}", "Endpoint".bold(), endpoint.bright_cyan());
    println!();

    println!("{}", "Note: Nomination requires staking pallet integration".bright_yellow());
    println!();

    Ok(())
}
