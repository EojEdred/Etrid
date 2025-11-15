use anyhow::{Context, Result};
use clap::ValueEnum;
use sp_core::{ed25519, sr25519, Pair};
use std::fs;
use std::path::PathBuf;

use crate::crypto::*;

// ═══════════════════════════════════════════════════════════════════════════════
// IMPORT FORMATS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ImportFormat {
    Json,
    Hex,
    Mnemonic,
}

// ═══════════════════════════════════════════════════════════════════════════════
// IMPORT FUNCTIONALITY
// ═══════════════════════════════════════════════════════════════════════════════

pub fn import_key(
    data: &str,
    format: ImportFormat,
    output: &PathBuf,
    password: &str,
    scheme: CryptoScheme,
) -> Result<()> {
    let key_data = match format {
        ImportFormat::Json => import_json(data)?,
        ImportFormat::Hex => import_hex(data, scheme)?,
        ImportFormat::Mnemonic => import_mnemonic(data, scheme)?,
    };

    let encrypted = encrypt_key(&key_data, password)?;
    fs::write(output, encrypted).context("Failed to write key file")?;

    println!("✓ Key imported successfully!");
    println!("  Public Key: {}", key_data.public_key);
    println!("  SS58 Address: {}", key_data.ss58_address);
    println!("  Output: {}", output.display());

    Ok(())
}

fn import_json(data: &str) -> Result<KeyData> {
    // Try to read as file first
    let json_str = if std::path::Path::new(data).exists() {
        fs::read_to_string(data).context("Failed to read JSON file")?
    } else {
        data.to_string()
    };

    let key_data: KeyData = serde_json::from_str(&json_str)
        .context("Invalid JSON format")?;

    Ok(key_data)
}

fn import_hex(data: &str, scheme: CryptoScheme) -> Result<KeyData> {
    let hex_data = data.trim().trim_start_matches("0x");
    let seed_bytes = hex::decode(hex_data).context("Invalid hex data")?;

    match scheme {
        CryptoScheme::Sr25519 => {
            let pair = sr25519::Pair::from_seed_slice(&seed_bytes)?;
            let public = pair.public();

            use sp_core::crypto::Ss58Codec;
            Ok(KeyData {
                scheme,
                public_key: hex::encode(public.as_ref()),
                secret_seed: hex::encode(&seed_bytes),
                ss58_address: public.to_ss58check(),
                key_type: None,
            })
        }
        CryptoScheme::Ed25519 => {
            let pair = ed25519::Pair::from_seed_slice(&seed_bytes)?;
            let public = pair.public();

            use sp_core::crypto::Ss58Codec;
            Ok(KeyData {
                scheme,
                public_key: hex::encode(public.as_ref()),
                secret_seed: hex::encode(&seed_bytes),
                ss58_address: public.to_ss58check(),
                key_type: None,
            })
        }
    }
}

fn import_mnemonic(mnemonic: &str, scheme: CryptoScheme) -> Result<KeyData> {
    match scheme {
        CryptoScheme::Sr25519 => {
            let pair = sr25519::Pair::from_string(mnemonic, None)
                .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {:?}", e))?;
            let public = pair.public();

            use sp_core::crypto::Ss58Codec;
            Ok(KeyData {
                scheme,
                public_key: hex::encode(public.as_ref()),
                secret_seed: hex::encode(pair.to_raw_vec()),
                ss58_address: public.to_ss58check(),
                key_type: None,
            })
        }
        CryptoScheme::Ed25519 => {
            let pair = ed25519::Pair::from_string(mnemonic, None)
                .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {:?}", e))?;
            let public = pair.public();

            use sp_core::crypto::Ss58Codec;
            Ok(KeyData {
                scheme,
                public_key: hex::encode(public.as_ref()),
                secret_seed: hex::encode(pair.to_raw_vec()),
                ss58_address: public.to_ss58check(),
                key_type: None,
            })
        }
    }
}

// Re-export encrypt_key from crypto module
pub use crate::crypto::encrypt_key;
