use anyhow::{Context, Result};
use clap::ValueEnum;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

use crate::crypto::*;

// ═══════════════════════════════════════════════════════════════════════════════
// EXPORT FORMATS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ExportFormat {
    Json,
    Hex,
    Mnemonic,
    SubstrateUri,
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXPORT FUNCTIONALITY
// ═══════════════════════════════════════════════════════════════════════════════

pub fn export_key(
    keyfile: &PathBuf,
    password: &str,
    format: ExportFormat,
    output: Option<PathBuf>,
) -> Result<()> {
    let encrypted = fs::read_to_string(keyfile).context("Failed to read key file")?;
    let key_data = decrypt_key(&encrypted, password)?;

    let exported = match format {
        ExportFormat::Json => export_json(&key_data)?,
        ExportFormat::Hex => export_hex(&key_data)?,
        ExportFormat::Mnemonic => {
            return Err(anyhow::anyhow!(
                "Cannot export mnemonic - it's not stored in the key file. Use the original mnemonic from key generation."
            ));
        }
        ExportFormat::SubstrateUri => export_substrate_uri(&key_data)?,
    };

    if let Some(output_path) = output {
        fs::write(&output_path, &exported).context("Failed to write export file")?;
        println!("✓ Exported to: {}", output_path.display());
    } else {
        println!("{}", exported);
    }

    Ok(())
}

fn export_json(key_data: &KeyData) -> Result<String> {
    let output = json!({
        "scheme": key_data.scheme.to_string(),
        "publicKey": key_data.public_key,
        "secretSeed": key_data.secret_seed,
        "ss58Address": key_data.ss58_address,
        "keyType": key_data.key_type,
    });

    Ok(serde_json::to_string_pretty(&output)?)
}

fn export_hex(key_data: &KeyData) -> Result<String> {
    Ok(format!(
        "0x{}",
        key_data.secret_seed
    ))
}

fn export_substrate_uri(key_data: &KeyData) -> Result<String> {
    // Substrate URI format: //seed//path
    Ok(format!(
        "0x{}",
        key_data.secret_seed
    ))
}

// Re-export decrypt_key from crypto module
pub use crate::crypto::decrypt_key;
