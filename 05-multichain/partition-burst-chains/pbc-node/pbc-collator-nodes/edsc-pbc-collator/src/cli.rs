use clap::Parser;
use sc_cli::{RunCmd, SubstrateCli};

/// EDSC-PBC Collator CLI
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: RunCmd,

    /// PBC ID (for EDSC-PBC this is typically 12)
    #[arg(long, default_value = "12")]
    pub pbc_id: u8,

    /// FlareChain RPC endpoint for state submission
    #[arg(long, default_value = "ws://127.0.0.1:9944")]
    pub relay_chain_rpc: String,
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
        "Ëtrid EDSC-PBC Collator".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "Ëtrid Dollar Stablecoin Partition Burst Chain Collator Node".into()
    }

    fn author() -> String {
        "Ëtrid Team".into()
    }

    fn support_url() -> String {
        "https://etrid.io/support".into()
    }

    fn copyright_start_year() -> i32 {
        2025
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(crate::chain_spec::development_config()?),
            "local" => Box::new(crate::chain_spec::local_testnet_config()?),
            path => Box::new(crate::chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}
