use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use sp_core::{
    crypto::{PublicError, Ss58Codec},
    ed25519, sr25519, Pair,
};
use std::fs;
use std::path::PathBuf;

mod crypto;
mod export;
mod import;

use crypto::*;
use export::*;
use import::*;

// ═══════════════════════════════════════════════════════════════════════════════
// CLI STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Parser)]
#[command(name = "asf-keygen")]
#[command(author = "Ëtrid Foundation")]
#[command(version = "0.1.0")]
#[command(about = "Key generation and management for ËTRID ASF validators", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate new cryptographic keys
    Generate {
        /// Cryptographic scheme to use
        #[arg(short, long, value_enum, default_value = "sr25519")]
        scheme: CryptoScheme,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Password to encrypt the key (interactive prompt if not provided)
        #[arg(short, long)]
        password: Option<String>,

        /// Optional mnemonic seed phrase (generates random if not provided)
        #[arg(short, long)]
        mnemonic: Option<String>,

        /// Key type label (e.g., "aura", "grandpa", "session")
        #[arg(short = 't', long)]
        key_type: Option<String>,
    },

    /// Inspect a key file and show public key details
    Inspect {
        /// Path to the key file
        #[arg(short, long)]
        keyfile: PathBuf,

        /// Password to decrypt the key
        #[arg(short, long)]
        password: Option<String>,

        /// Show detailed information
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Sign a message with a private key
    Sign {
        /// Path to the key file
        #[arg(short, long)]
        keyfile: PathBuf,

        /// Password to decrypt the key
        #[arg(short, long)]
        password: Option<String>,

        /// Message to sign (hex encoded)
        #[arg(short, long)]
        message: String,

        /// Output signature format
        #[arg(short, long, value_enum, default_value = "hex")]
        format: OutputFormat,
    },

    /// Verify a signature
    Verify {
        /// Public key (hex or SS58)
        #[arg(short = 'k', long)]
        pubkey: String,

        /// Message that was signed (hex encoded)
        #[arg(short, long)]
        message: String,

        /// Signature (hex encoded)
        #[arg(short, long)]
        signature: String,

        /// Cryptographic scheme
        #[arg(short = 'c', long, value_enum, default_value = "sr25519")]
        scheme: CryptoScheme,
    },

    /// Export keys in various formats
    Export {
        /// Path to the key file
        #[arg(short, long)]
        keyfile: PathBuf,

        /// Password to decrypt the key
        #[arg(short, long)]
        password: Option<String>,

        /// Export format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,

        /// Output file (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Import keys from various formats
    Import {
        /// Input data (hex string, JSON file, or mnemonic)
        #[arg(short, long)]
        data: String,

        /// Import format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ImportFormat,

        /// Output keyfile path
        #[arg(short, long)]
        output: PathBuf,

        /// Password to encrypt the key
        #[arg(short, long)]
        password: Option<String>,

        /// Cryptographic scheme (for hex/mnemonic import)
        #[arg(short = 'c', long, value_enum, default_value = "sr25519")]
        scheme: CryptoScheme,
    },

    /// Generate session keys for a validator
    GenerateSession {
        /// Output directory for session keys
        #[arg(short, long, default_value = "./session-keys")]
        output_dir: PathBuf,

        /// Password to encrypt the keys
        #[arg(short, long)]
        password: Option<String>,

        /// Validator name/identifier
        #[arg(short = 'n', long)]
        name: String,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ENTRY POINT
// ═══════════════════════════════════════════════════════════════════════════════

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            scheme,
            output,
            password,
            mnemonic,
            key_type,
        } => {
            let password = get_password(password, "Enter password to encrypt key")?;
            generate_key(scheme, output, &password, mnemonic, key_type)?;
        }

        Commands::Inspect {
            keyfile,
            password,
            verbose,
        } => {
            let password = get_password(password, "Enter password to decrypt key")?;
            inspect_key(&keyfile, &password, verbose)?;
        }

        Commands::Sign {
            keyfile,
            password,
            message,
            format,
        } => {
            let password = get_password(password, "Enter password to decrypt key")?;
            sign_message(&keyfile, &password, &message, format)?;
        }

        Commands::Verify {
            pubkey,
            message,
            signature,
            scheme,
        } => {
            verify_signature(&pubkey, &message, &signature, scheme)?;
        }

        Commands::Export {
            keyfile,
            password,
            format,
            output,
        } => {
            let password = get_password(password, "Enter password to decrypt key")?;
            export_key(&keyfile, &password, format, output)?;
        }

        Commands::Import {
            data,
            format,
            output,
            password,
            scheme,
        } => {
            let password = get_password(password, "Enter password to encrypt key")?;
            import_key(&data, format, &output, &password, scheme)?;
        }

        Commands::GenerateSession {
            output_dir,
            password,
            name,
        } => {
            let password = get_password(password, "Enter password to encrypt keys")?;
            generate_session_keys(&output_dir, &password, &name)?;
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

fn get_password(password: Option<String>, prompt: &str) -> Result<String> {
    match password {
        Some(p) => Ok(p),
        None => {
            let password = rpassword::prompt_password(format!("{}: ", prompt.cyan()))
                .context("Failed to read password")?;
            Ok(password)
        }
    }
}

fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message.green());
}

fn print_info(label: &str, value: &str) {
    println!("  {} {}", format!("{}:", label).cyan(), value);
}

fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}
