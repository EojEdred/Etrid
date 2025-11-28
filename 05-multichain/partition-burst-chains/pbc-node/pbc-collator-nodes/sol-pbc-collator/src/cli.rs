use clap::Parser;
use sc_cli::{RunCmd, SubstrateCli};

/// PBC Collator CLI
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: RunCmd,

    /// PBC ID (0-11 for the 12 chains)
    #[arg(long, default_value = "0")]
    pub pbc_id: u8,

    /// Primearc Core Chain RPC endpoint for state submission
    #[arg(long, default_value = "ws://127.0.0.1:9944")]
    pub relay_chain_rpc: String,

    /// Enable DETR P2P networking
    #[arg(long, default_value = "true")]
    pub p2p_enabled: bool,

    /// P2P bind address (can be 0.0.0.0 for all interfaces)
    #[arg(long, default_value = "0.0.0.0:30333")]
    pub p2p_bind_address: String,

    /// P2P announce address (auto-detected if not specified)
    /// Can also be set via DETR_P2P_ANNOUNCE_IP environment variable
    #[arg(long)]
    pub p2p_announce_address: Option<String>,

    /// Bootstrap peers in format: peer_id@ip:port,peer_id@ip:port
    /// Example: 0000...0001@127.0.0.1:30333
    #[arg(long, default_value = "")]
    pub p2p_bootstrap_peers: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Build a chain specification
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state
    Revert(sc_cli::RevertCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Ëtrid PBC Collator".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "Ëtrid Partition Burst Chain Collator Node".into()
    }

    fn author() -> String {
        "Ëtrid Team".into()
    }

    fn support_url() -> String {
        "https://etrid.org/support".into()
    }

    fn copyright_start_year() -> i32 {
        2025
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(crate::chain_spec::development_config()?),
            "local" => Box::new(crate::chain_spec::local_testnet_config()?),
            "production" | "mainnet" | "prod" => Box::new(crate::chain_spec::production_config()?),
            path => Box::new(crate::chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}