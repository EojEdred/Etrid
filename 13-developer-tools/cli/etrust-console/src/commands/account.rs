// ═══════════════════════════════════════════════════════════════════════════════
// Account Management Commands
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{Context, Result};
use colored::Colorize;
use sp_core::{sr25519, Pair};

use crate::cli::AccountCommands;

// Helper to format SS58 address
fn to_ss58_address(public_key: &sr25519::Public) -> String {
    // For MVP, we'll just use hex format
    // In production, use proper SS58 encoding with checksum
    format!("5{}", &hex::encode(&public_key.0)[..46])
}

pub async fn execute(command: AccountCommands) -> Result<()> {
    match command {
        AccountCommands::Create { name, password } => create_account(name, password).await,
        AccountCommands::List => list_accounts().await,
        AccountCommands::Import {
            secret,
            name,
            password,
        } => import_account(secret, name, password).await,
        AccountCommands::Export { account, format } => export_account(account, format).await,
        AccountCommands::Balance { account } => show_balance(account).await,
    }
}

async fn create_account(name: Option<String>, _password: Option<String>) -> Result<()> {
    println!("{}", "Creating new account...".bright_green().bold());
    println!();

    // Generate random seed
    use rand::RngCore;
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    // Create keypair from seed
    let pair = sr25519::Pair::from_seed(&seed);
    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    // Display account information
    println!("{}", "Account created successfully!".bright_green());
    println!();
    println!("  {}: {}", "Address".bold(), address.bright_white());
    println!("  {}: 0x{}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());
    println!("  {}: 0x{}", "Seed".bold(), hex::encode(seed).bright_white());
    println!();

    if let Some(name) = name {
        println!("  {}: {}", "Name".bold(), name.bright_white());
    }

    println!();
    println!("{}", "⚠️  WARNING: Save your seed securely!".bright_red().bold());
    println!("   This is the ONLY way to recover your account.");
    println!();
    println!("{}", "Note: This MVP generates keypairs without BIP39 mnemonics.".bright_yellow());
    println!("      For production use, enable BIP39 support.");
    println!();

    Ok(())
}

async fn list_accounts() -> Result<()> {
    println!("{}", "Local Accounts".bright_green().bold());
    println!();
    println!("  {}", "No keystore implementation yet - this is MVP".bright_yellow());
    println!("  {}", "Accounts are generated on-the-fly".bright_yellow());
    println!();
    println!("  Use: {} to create a new account", "etrust account create".bright_cyan());
    println!("  Use: {} to import an existing account", "etrust account import".bright_cyan());
    println!();

    Ok(())
}

async fn import_account(secret: String, name: Option<String>, _password: Option<String>) -> Result<()> {
    println!("{}", "Importing account...".bright_green().bold());
    println!();

    // Try to parse as hex secret key
    let pair = {
        // Hex secret key
        let secret_bytes = hex::decode(secret.trim_start_matches("0x"))
            .context("Invalid hex secret key")?;
        sr25519::Pair::from_seed_slice(&secret_bytes)
            .map_err(|e| anyhow::anyhow!("Invalid secret key length (expected 32 bytes): {:?}", e))?
    };

    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    // Display account information
    println!("{}", "Account imported successfully!".bright_green());
    println!();
    println!("  {}: {}", "Address".bold(), address.bright_white());
    println!("  {}: 0x{}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());

    if let Some(name) = name {
        println!("  {}: {}", "Name".bold(), name.bright_white());
    }

    println!();

    Ok(())
}

async fn export_account(account: String, format: String) -> Result<()> {
    println!("{}", "Exporting account...".bright_green().bold());
    println!();
    println!("  {}: {}", "Account".bold(), account.bright_white());
    println!("  {}: {}", "Format".bold(), format.bright_white());
    println!();
    println!("  {}", "Export functionality requires keystore implementation".bright_yellow());
    println!("  {}", "This is a placeholder for MVP".bright_yellow());
    println!();

    Ok(())
}

async fn show_balance(account: String) -> Result<()> {
    println!("{}", "Querying balance...".bright_green().bold());
    println!();
    println!("  {}: {}", "Account".bold(), account.bright_white());
    println!();
    println!("  {}", "Balance queries require RPC connection".bright_yellow());
    println!("  Use: {} to query via RPC", "etrust query balance <address>".bright_cyan());
    println!();

    Ok(())
}
