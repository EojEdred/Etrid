// ═══════════════════════════════════════════════════════════════════════════════
// ËTRUST - ËTRID Rust CLI
// Professional command-line interface for ËTRID Protocol
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

mod cli;
mod commands;
mod rpc_client;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Print banner for main commands (not for quiet operations)
    if !matches!(cli.command, Commands::Query { .. }) {
        print_banner();
    }

    // Execute command
    match cli.command {
        Commands::Account { subcommand } => {
            commands::account::execute(subcommand).await?;
        }
        Commands::Stake { subcommand } => {
            commands::stake::execute(subcommand, &cli.endpoint).await?;
        }
        Commands::Query { subcommand } => {
            commands::query::execute(subcommand, &cli.endpoint).await?;
        }
        Commands::Send { subcommand } => {
            commands::send::execute(subcommand, &cli.endpoint).await?;
        }
        Commands::Consensus { subcommand } => {
            commands::consensus::execute(subcommand, &cli.endpoint).await?;
        }
        Commands::Keys { subcommand } => {
            commands::keys::execute(subcommand).await?;
        }
    }

    Ok(())
}

fn print_banner() {
    println!();
    println!("{}", "═══════════════════════════════════════════════════════════".bright_cyan());
    println!("{}", "  ËTRUST - ËTRID Rust CLI v0.1.0".bright_white().bold());
    println!("{}", "  Professional CLI for ËTRID Protocol".bright_white());
    println!("{}", "═══════════════════════════════════════════════════════════".bright_cyan());
    println!();
}
