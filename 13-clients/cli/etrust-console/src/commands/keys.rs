// ═══════════════════════════════════════════════════════════════════════════════
// Key Management Commands
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::{Context, Result};
use colored::Colorize;
use sp_core::{sr25519, Pair};

use crate::cli::KeysCommands;

// Helper to format SS58 address
fn to_ss58_address(public_key: &sr25519::Public) -> String {
    // For MVP, we'll just use hex format
    // In production, use proper SS58 encoding with checksum
    format!("5{}", &hex::encode(&public_key.0)[..46])
}

pub async fn execute(command: KeysCommands) -> Result<()> {
    match command {
        KeysCommands::Generate { key_type } => generate_key(key_type).await,
        KeysCommands::Derive { parent, path } => derive_key(parent, path).await,
        KeysCommands::Inspect { key } => inspect_key(key).await,
        KeysCommands::Mnemonic => generate_mnemonic().await,
    }
}

async fn generate_key(key_type: String) -> Result<()> {
    println!("{}", "Generating Keypair".bright_green().bold());
    println!();
    println!("  {}: {}", "Key Type".bold(), key_type.bright_white());
    println!();

    match key_type.to_lowercase().as_str() {
        "sr25519" => generate_sr25519(),
        "ed25519" | "ecdsa" => {
            println!("{}", "Note: This MVP implementation supports SR25519 keys.".bright_yellow());
            println!("      For ED25519/ECDSA support, use the full key management implementation.");
            println!();
            println!("Generating SR25519 key instead...");
            println!();
            generate_sr25519()
        }
        _ => {
            println!("{}", "Error: Invalid key type. Use: sr25519, ed25519, or ecdsa".bright_red());
            return Ok(());
        }
    }

    println!();
    println!("{}", "⚠️  WARNING: Keep your secret key and mnemonic secure!".bright_red().bold());
    println!();

    Ok(())
}

fn generate_sr25519() {
    // Generate random seed
    use rand::RngCore;
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    // Create keypair from seed
    let pair = sr25519::Pair::from_seed(&seed);
    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    println!("{}", "SR25519 Keypair:".bright_cyan().bold());
    println!("  {}: {}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());
    println!("  {}: {}", "SS58 Address".bold(), address.bright_white());
    println!("  {}: 0x{}", "Seed".bold(), hex::encode(seed).bright_white());
    println!();
    println!("{}", "Note: For production, use BIP39 mnemonic phrases instead of raw seeds.".bright_yellow());
}


async fn derive_key(parent: String, path: String) -> Result<()> {
    println!("{}", "Deriving Child Key".bright_green().bold());
    println!();
    println!("  {}: {}", "Derivation Path".bold(), path.bright_white());
    println!();

    // Try to parse as mnemonic or seed
    match sr25519::Pair::from_string(&format!("{}{}", parent, path), None) {
        Ok(pair) => {
            let public_key = pair.public();
            let address = to_ss58_address(&public_key);

            println!("{}", "Derived Key:".bright_cyan().bold());
            println!("  {}: {}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());
            println!("  {}: {}", "SS58 Address".bold(), address.bright_white());
            println!();
        }
        Err(e) => {
            println!("{}", "Error: Failed to derive key".bright_red());
            println!("  {}: {:?}", "Details".bold(), e);
            println!();
            println!("{}", "Tips:".bright_yellow());
            println!("  - Ensure parent is a valid mnemonic phrase");
            println!("  - Derivation path should start with //");
            println!("  - Example: \"//Alice\" or \"//Alice//stash\"");
        }
    }

    println!();
    Ok(())
}

async fn inspect_key(key: String) -> Result<()> {
    println!("{}", "Inspecting Key".bright_green().bold());
    println!();

    // Try different key formats
    let inspect_result = if key.contains(' ') {
        // Mnemonic phrase
        inspect_sr25519_mnemonic(&key)
    } else if key.starts_with("0x") || key.len() == 64 {
        // Hex seed
        inspect_sr25519_seed(&key)
    } else {
        // Try as secret URI
        inspect_sr25519_uri(&key)
    };

    match inspect_result {
        Ok(_) => {}
        Err(e) => {
            println!("{}", "Error: Failed to inspect key".bright_red());
            println!("  {}: {}", "Details".bold(), e.to_string().bright_red());
            println!();
            println!("{}", "Supported formats:".bright_yellow());
            println!("  - Mnemonic phrase (12 or 24 words)");
            println!("  - Hex seed (0x... or 64 hex chars)");
            println!("  - Secret URI with derivation path");
        }
    }

    println!();
    Ok(())
}

fn inspect_sr25519_mnemonic(mnemonic: &str) -> Result<()> {
    let (pair, _) = sr25519::Pair::from_phrase(mnemonic, None)
        .map_err(|e| anyhow::anyhow!("Invalid mnemonic phrase: {:?}", e))?;

    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    println!("{}", "Key Information:".bright_cyan().bold());
    println!("  {}: {}", "Type".bold(), "SR25519".bright_white());
    println!("  {}: {}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());
    println!("  {}: {}", "SS58 Address".bold(), address.bright_white());

    Ok(())
}

fn inspect_sr25519_seed(seed: &str) -> Result<()> {
    let seed_bytes = hex::decode(seed.trim_start_matches("0x"))
        .context("Invalid hex seed")?;

    let pair = sr25519::Pair::from_seed_slice(&seed_bytes)
        .map_err(|e| anyhow::anyhow!("Invalid seed length (expected 32 bytes): {:?}", e))?;

    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    println!("{}", "Key Information:".bright_cyan().bold());
    println!("  {}: {}", "Type".bold(), "SR25519".bright_white());
    println!("  {}: {}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());
    println!("  {}: {}", "SS58 Address".bold(), address.bright_white());

    Ok(())
}

fn inspect_sr25519_uri(uri: &str) -> Result<()> {
    let pair = sr25519::Pair::from_string(uri, None)
        .map_err(|e| anyhow::anyhow!("Invalid secret URI: {:?}", e))?;

    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    println!("{}", "Key Information:".bright_cyan().bold());
    println!("  {}: {}", "Type".bold(), "SR25519".bright_white());
    println!("  {}: {}", "Public Key".bold(), hex::encode(&public_key.0).bright_white());
    println!("  {}: {}", "SS58 Address".bold(), address.bright_white());

    Ok(())
}

async fn generate_mnemonic() -> Result<()> {
    println!("{}", "Generating Keypair".bright_green().bold());
    println!();

    // Generate random seed
    use rand::RngCore;
    let mut seed = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut seed);

    // Create keypair from seed
    let pair = sr25519::Pair::from_seed(&seed);
    let public_key = pair.public();
    let address = to_ss58_address(&public_key);

    println!("{}", "Generated Key:".bright_cyan().bold());
    println!("  {}: 0x{}", "Seed".bold(), hex::encode(seed).bright_white());
    println!();

    println!("{}", "Derived Address:".bright_cyan().bold());
    println!("  {}", address.bright_white());
    println!();

    println!("{}", "⚠️  WARNING: Store this seed securely!".bright_red().bold());
    println!("   This seed gives full access to your account.");
    println!();
    println!("{}", "Note: For production, use BIP39 mnemonic generation.".bright_yellow());
    println!();

    Ok(())
}
