//! CLI configuration for DOGE-PBC Collator

use clap::Parser;
use sc_cli::{RunCmd, SubstrateCli};

#[derive(Debug, Parser)]
#[command(
    propagate_version = true,
    args_conflicts_with_subcommands = true,
    subcommand_negates_reqs = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: RunCmd,

    /// DETR P2P listen address (default: 0.0.0.0:30333)
    #[arg(long, env = "DETR_P2P_LISTEN_ADDR")]
    pub p2p_listen: Option<String>,

    /// DETR P2P announce IP (for NAT traversal)
    #[arg(long, env = "DETR_P2P_ANNOUNCE_IP")]
    pub p2p_announce_ip: Option<String>,

    /// Bootstrap peers (comma-separated addresses)
    #[arg(long, env = "DETR_P2P_BOOTSTRAP_PEERS")]
    pub p2p_bootstrap: Option<String>,

    /// Maximum peer connections
    #[arg(long, default_value = "100", env = "DETR_P2P_MAX_CONNECTIONS")]
    pub p2p_max_connections: usize,
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

    /// Export state
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state
    Revert(sc_cli::RevertCmd),

    /// Database utilities
    #[command(subcommand)]
    ChainInfo(sc_cli::ChainInfoCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "DOGE-PBC Collator".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "Dogecoin Partition Burst Chain Collator\n\n\
        Features:\n\
        - DETR P2P networking with auto-reconnection\n\
        - Dogecoin bridge with 20 confirmations\n\
        - Lightning channel support for instant payments\n\
        - ASF consensus for efficient block production".into()
    }

    fn author() -> String {
        "Ëtrid Foundation".into()
    }

    fn support_url() -> String {
        "https://github.com/etrid/etrid".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "local" => Box::new(chain_spec::local_testnet_config()?),
            "" | "doge-pbc" => Box::new(chain_spec::doge_pbc_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}
