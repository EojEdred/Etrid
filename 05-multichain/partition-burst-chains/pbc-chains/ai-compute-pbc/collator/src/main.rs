//! AI Compute PBC Collator Node
//!
//! This is the main entry point for the AI-Compute-PBC parachain collator.
//! It handles:
//! - Chain initialization
//! - Block production (Aura consensus)
//! - Finality (GRANDPA)
//! - RPC endpoints
//! - Checkpoint submission to FlareChain

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
    command::run()
}
