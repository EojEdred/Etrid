//! DOGE-PBC Collator Entry Point
//!
//! Dogecoin Partition Burst Chain with:
//! - DETR P2P networking with auto-reconnection
//! - Dogecoin bridge (20 confirmations)
//! - Lightning channel support
//! - ASF consensus integration

mod cli;
mod command;
mod rpc;
mod service;
mod chain_spec;

fn main() -> sc_cli::Result<()> {
    command::run()
}
