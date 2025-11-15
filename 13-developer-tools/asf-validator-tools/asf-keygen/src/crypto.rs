use anyhow::{anyhow, Context, Result};
use clap::ValueEnum;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sp_core::{
    crypto::{Ss58Codec, SecretStringError},
    ed25519, sr25519, Pair,
};
use std::fs;
use std::path::PathBuf;

// ═══════════════════════════════════════════════════════════════════════════════
// CRYPTO TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CryptoScheme {
    Sr25519,
    Ed25519,
}

impl std::fmt::Display for CryptoScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoScheme::Sr25519 => write!(f, "sr25519"),
            CryptoScheme::Ed25519 => write!(f, "ed25519"),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Hex,
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyData {
    pub scheme: CryptoScheme,
    pub public_key: String,
    pub secret_seed: String,
    pub ss58_address: String,
    pub key_type: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEY GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

pub fn generate_key(
    scheme: CryptoScheme,
    output: Option<PathBuf>,
    password: &str,
    mnemonic: Option<String>,
    key_type: Option<String>,
) -> Result<()> {
    println!("{}", "Generating new key pair...".cyan());

    let (public_hex, secret_seed, ss58_address, used_mnemonic) = match scheme {
        CryptoScheme::Sr25519 => generate_sr25519(mnemonic)?,
        CryptoScheme::Ed25519 => generate_ed25519(mnemonic)?,
    };

    let key_data = KeyData {
        scheme,
        public_key: public_hex.clone(),
        secret_seed: secret_seed.clone(),
        ss58_address: ss58_address.clone(),
        key_type: key_type.clone(),
    };

    // Encrypt and save
    let encrypted = encrypt_key(&key_data, password)?;

    if let Some(output_path) = output {
        fs::write(&output_path, encrypted).context("Failed to write key file")?;
        println!("{} Key saved to: {}", "✓".green().bold(), output_path.display());
    } else {
        println!("\n{}", "Encrypted Key:".cyan());
        println!("{}", encrypted);
    }

    // Display key information
    println!("\n{}", "Key Information:".cyan().bold());
    println!("  {} {}", "Scheme:".cyan(), scheme);
    if let Some(ref kt) = key_type {
        println!("  {} {}", "Type:".cyan(), kt);
    }
    println!("  {} {}", "Public Key:".cyan(), public_hex);
    println!("  {} {}", "SS58 Address:".cyan(), ss58_address);

    if used_mnemonic.is_some() {
        println!("\n{}", "Mnemonic Seed Phrase:".yellow().bold());
        println!("  {}", used_mnemonic.as_ref().unwrap().yellow());
        println!("\n{}", "⚠ Store this mnemonic in a secure location!".yellow().bold());
    }

    println!("\n{}", "✓ Key generation complete!".green().bold());
    Ok(())
}

fn generate_sr25519(mnemonic: Option<String>) -> Result<(String, String, String, Option<String>)> {
    let (pair, phrase) = match mnemonic {
        Some(m) => (
            sr25519::Pair::from_string(&m, None)
                .map_err(|e| anyhow!("Invalid mnemonic: {:?}", e))?,
            Some(m),
        ),
        None => {
            let (pair, phrase, _) = sr25519::Pair::generate_with_phrase(None);
            (pair, Some(phrase))
        }
    };

    let public = pair.public();
    let public_hex = hex::encode(public.as_ref());
    let ss58 = public.to_ss58check();
    let seed = hex::encode(pair.to_raw_vec());

    Ok((public_hex, seed, ss58, phrase))
}

fn generate_ed25519(mnemonic: Option<String>) -> Result<(String, String, String, Option<String>)> {
    let (pair, phrase) = match mnemonic {
        Some(m) => (
            ed25519::Pair::from_string(&m, None)
                .map_err(|e| anyhow!("Invalid mnemonic: {:?}", e))?,
            Some(m),
        ),
        None => {
            let (pair, phrase, _) = ed25519::Pair::generate_with_phrase(None);
            (pair, Some(phrase))
        }
    };

    let public = pair.public();
    let public_hex = hex::encode(public.as_ref());
    let ss58 = public.to_ss58check();
    let seed = hex::encode(pair.to_raw_vec());

    Ok((public_hex, seed, ss58, phrase))
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEY INSPECTION
// ═══════════════════════════════════════════════════════════════════════════════

pub fn inspect_key(keyfile: &PathBuf, password: &str, verbose: bool) -> Result<()> {
    let encrypted = fs::read_to_string(keyfile).context("Failed to read key file")?;
    let key_data = decrypt_key(&encrypted, password)?;

    println!("{}", "Key Information:".cyan().bold());
    println!("  {} {}", "Scheme:".cyan(), key_data.scheme);
    if let Some(ref kt) = key_data.key_type {
        println!("  {} {}", "Type:".cyan(), kt);
    }
    println!("  {} {}", "Public Key:".cyan(), key_data.public_key);
    println!("  {} {}", "SS58 Address:".cyan(), key_data.ss58_address);

    if verbose {
        println!("\n{}", "Detailed Information:".cyan().bold());
        println!("  {} {} bytes", "Public Key Length:".cyan(), key_data.public_key.len() / 2);
        println!("  {} {}", "File Path:".cyan(), keyfile.display());
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE SIGNING
// ═══════════════════════════════════════════════════════════════════════════════

pub fn sign_message(
    keyfile: &PathBuf,
    password: &str,
    message: &str,
    format: OutputFormat,
) -> Result<()> {
    let encrypted = fs::read_to_string(keyfile).context("Failed to read key file")?;
    let key_data = decrypt_key(&encrypted, password)?;

    let message_bytes = hex::decode(message).context("Message must be hex encoded")?;

    let signature = match key_data.scheme {
        CryptoScheme::Sr25519 => {
            let seed = hex::decode(&key_data.secret_seed)?;
            let pair = sr25519::Pair::from_seed_slice(&seed)?;
            hex::encode(pair.sign(&message_bytes).0)
        }
        CryptoScheme::Ed25519 => {
            let seed = hex::decode(&key_data.secret_seed)?;
            let pair = ed25519::Pair::from_seed_slice(&seed)?;
            hex::encode(pair.sign(&message_bytes).0)
        }
    };

    match format {
        OutputFormat::Hex => {
            println!("{}", signature);
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "message": message,
                "signature": signature,
                "public_key": key_data.public_key,
                "scheme": key_data.scheme,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// SIGNATURE VERIFICATION
// ═══════════════════════════════════════════════════════════════════════════════

pub fn verify_signature(
    pubkey: &str,
    message: &str,
    signature: &str,
    scheme: CryptoScheme,
) -> Result<()> {
    let message_bytes = hex::decode(message).context("Message must be hex encoded")?;
    let signature_bytes = hex::decode(signature).context("Signature must be hex encoded")?;

    // Try to parse as hex first, then SS58
    let pubkey_bytes = if let Ok(bytes) = hex::decode(pubkey) {
        bytes
    } else {
        // Try SS58
        match scheme {
            CryptoScheme::Sr25519 => {
                sr25519::Public::from_ss58check(pubkey)
                    .map_err(|_| anyhow!("Invalid public key format"))?
                    .0
                    .to_vec()
            }
            CryptoScheme::Ed25519 => {
                ed25519::Public::from_ss58check(pubkey)
                    .map_err(|_| anyhow!("Invalid public key format"))?
                    .0
                    .to_vec()
            }
        }
    };

    let valid = match scheme {
        CryptoScheme::Sr25519 => {
            if pubkey_bytes.len() != 32 || signature_bytes.len() != 64 {
                return Err(anyhow!("Invalid key or signature length for sr25519"));
            }
            let mut pk = [0u8; 32];
            pk.copy_from_slice(&pubkey_bytes);
            let public = sr25519::Public::from_raw(pk);

            let mut sig = [0u8; 64];
            sig.copy_from_slice(&signature_bytes);
            let signature = sr25519::Signature::from_raw(sig);

            sr25519::Pair::verify(&signature, &message_bytes, &public)
        }
        CryptoScheme::Ed25519 => {
            if pubkey_bytes.len() != 32 || signature_bytes.len() != 64 {
                return Err(anyhow!("Invalid key or signature length for ed25519"));
            }
            let mut pk = [0u8; 32];
            pk.copy_from_slice(&pubkey_bytes);
            let public = ed25519::Public::from_raw(pk);

            let mut sig = [0u8; 64];
            sig.copy_from_slice(&signature_bytes);
            let signature = ed25519::Signature::from_raw(sig);

            ed25519::Pair::verify(&signature, &message_bytes, &public)
        }
    };

    if valid {
        println!("{} Signature is valid!", "✓".green().bold());
    } else {
        println!("{} Signature is INVALID!", "✗".red().bold());
        return Err(anyhow!("Invalid signature"));
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// ENCRYPTION/DECRYPTION (Simple XOR for demo - use proper encryption in production)
// ═══════════════════════════════════════════════════════════════════════════════

fn encrypt_key(key_data: &KeyData, password: &str) -> Result<String> {
    let json = serde_json::to_string(key_data)?;
    let encrypted = xor_encrypt(json.as_bytes(), password.as_bytes());
    Ok(base64::encode(encrypted))
}

fn decrypt_key(encrypted: &str, password: &str) -> Result<KeyData> {
    let encrypted_bytes = base64::decode(encrypted.trim())
        .context("Failed to decode base64")?;
    let decrypted = xor_encrypt(&encrypted_bytes, password.as_bytes());
    let json = String::from_utf8(decrypted).context("Failed to decrypt - wrong password?")?;
    let key_data: KeyData = serde_json::from_str(&json)
        .context("Failed to parse key data - corrupted or wrong password")?;
    Ok(key_data)
}

fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION KEYS GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

pub fn generate_session_keys(output_dir: &PathBuf, password: &str, name: &str) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    println!("{}", "Generating session keys for validator...".cyan().bold());
    println!("  {} {}", "Validator:".cyan(), name);

    // Generate AURA key (sr25519)
    let aura_path = output_dir.join(format!("{}-aura.key", name));
    println!("\n{}", "Generating AURA key (sr25519)...".cyan());
    generate_key(
        CryptoScheme::Sr25519,
        Some(aura_path.clone()),
        password,
        None,
        Some("aura".to_string()),
    )?;

    // Generate GRANDPA key (ed25519)
    let grandpa_path = output_dir.join(format!("{}-grandpa.key", name));
    println!("\n{}", "Generating GRANDPA key (ed25519)...".cyan());
    generate_key(
        CryptoScheme::Ed25519,
        Some(grandpa_path.clone()),
        password,
        None,
        Some("grandpa".to_string()),
    )?;

    println!("\n{}", "✓ Session keys generated successfully!".green().bold());
    println!("  {} {}", "AURA key:".cyan(), aura_path.display());
    println!("  {} {}", "GRANDPA key:".cyan(), grandpa_path.display());

    Ok(())
}

// Add base64 dependency
use base64::{engine::general_purpose::STANDARD as base64, Engine};
