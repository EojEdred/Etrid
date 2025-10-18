// ═══════════════════════════════════════════════════════════════════════════════
// Query Commands
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::Result;
use colored::Colorize;

use crate::{cli::QueryCommands, rpc_client::{EtridRpcClient, format_balance}};

pub async fn execute(command: QueryCommands, endpoint: &str) -> Result<()> {
    match command {
        QueryCommands::Block { block_id } => query_block(block_id, endpoint).await,
        QueryCommands::Transaction { tx_hash } => query_transaction(tx_hash, endpoint).await,
        QueryCommands::Balance { account } => query_balance(account, endpoint).await,
        QueryCommands::ChainInfo => query_chain_info(endpoint).await,
        QueryCommands::Peers => query_peers(endpoint).await,
        QueryCommands::BlockNumber => query_block_number(endpoint).await,
    }
}

async fn query_block(block_id: String, endpoint: &str) -> Result<()> {
    let client = EtridRpcClient::new(endpoint).await?;

    println!("{}", "Block Information".bright_green().bold());
    println!();

    let block = client.get_block(&block_id).await?;

    println!("{}", serde_json::to_string_pretty(&block)?);
    println!();

    Ok(())
}

async fn query_transaction(tx_hash: String, endpoint: &str) -> Result<()> {
    let _client = EtridRpcClient::new(endpoint).await?;

    println!("{}", "Transaction Information".bright_green().bold());
    println!();
    println!("  {}: {}", "Hash".bold(), tx_hash.bright_white());
    println!();
    println!("{}", "Note: Transaction queries require block indexing".bright_yellow());
    println!("      Use chain explorer or block scanning for full transaction history.");
    println!();

    Ok(())
}

async fn query_balance(account: String, endpoint: &str) -> Result<()> {
    let client = EtridRpcClient::new(endpoint).await?;

    println!("{}", "Account Balance".bright_green().bold());
    println!();
    println!("  {}: {}", "Account".bold(), account.bright_white());
    println!();

    match client.get_balance(&account).await {
        Ok(balance) => {
            println!("  {}: {}", "Free".bold(), format_balance(balance.free).bright_green());
            println!("  {}: {}", "Reserved".bold(), format_balance(balance.reserved).bright_yellow());
            println!("  {}: {}", "Total".bold(), format_balance(balance.total()).bright_white().bold());
        }
        Err(e) => {
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}

async fn query_chain_info(endpoint: &str) -> Result<()> {
    let client = EtridRpcClient::new(endpoint).await?;

    println!("{}", "Chain Information".bright_green().bold());
    println!();

    // Get chain name
    match client.get_chain_name().await {
        Ok(name) => println!("  {}: {}", "Chain".bold(), name.bright_white()),
        Err(e) => println!("  {}: {}", "Chain".bold(), format!("Error: {}", e).bright_red()),
    }

    // Get version
    match client.get_version().await {
        Ok(version) => println!("  {}: {}", "Version".bold(), version.bright_white()),
        Err(e) => println!("  {}: {}", "Version".bold(), format!("Error: {}", e).bright_red()),
    }

    // Get block number
    match client.get_block_number().await {
        Ok(number) => println!("  {}: {}", "Latest Block".bold(), number.to_string().bright_white()),
        Err(e) => println!("  {}: {}", "Latest Block".bold(), format!("Error: {}", e).bright_red()),
    }

    // Get system properties
    match client.get_system_properties().await {
        Ok(props) => {
            println!();
            println!("  {}:", "Properties".bold());
            println!("{}", serde_json::to_string_pretty(&props)?);
        }
        Err(e) => println!("  {}: {}", "Properties".bold(), format!("Error: {}", e).bright_red()),
    }

    println!();
    Ok(())
}

async fn query_peers(endpoint: &str) -> Result<()> {
    let client = EtridRpcClient::new(endpoint).await?;

    println!("{}", "Network Peers".bright_green().bold());
    println!();

    match client.get_peers().await {
        Ok(peers) => {
            if peers.is_empty() {
                println!("  {}", "No peers connected".bright_yellow());
            } else {
                println!("  {}: {}", "Connected Peers".bold(), peers.len().to_string().bright_white());
                println!();

                for (i, peer) in peers.iter().enumerate() {
                    println!("  {} {}:", "Peer".bright_cyan(), (i + 1).to_string().bright_white());
                    println!("    {}: {}", "ID".bold(), peer.peer_id);
                    println!("    {}: {}", "Role".bold(), peer.roles);
                    println!("    {}: {}", "Best Block".bold(), peer.best_number);
                    println!("    {}: {}...", "Best Hash".bold(), &peer.best_hash[..16]);
                    println!();
                }
            }
        }
        Err(e) => {
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}

async fn query_block_number(endpoint: &str) -> Result<()> {
    let client = EtridRpcClient::new(endpoint).await?;

    match client.get_block_number().await {
        Ok(number) => {
            println!("{}", number);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
