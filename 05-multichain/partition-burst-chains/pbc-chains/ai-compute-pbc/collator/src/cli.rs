//! CLI argument parsing

use clap::Parser;
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};

/// AI Compute PBC Collator CLI
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: cumulus_client_cli::RunCmd,

    /// Relay chain arguments
    #[arg(raw = true)]
    pub relay_chain_args: Vec<String>,
}

/// Subcommands
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
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

    /// Revert chain to previous state
    Revert(sc_cli::RevertCmd),

    /// Remove the whole chain
    PurgeChain(cumulus_client_cli::PurgeChainCmd),

    /// Export genesis state
    ExportGenesisState(cumulus_client_cli::ExportGenesisStateCommand),

    /// Export genesis wasm
    ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

    /// Benchmark runtime pallets
    #[cfg(feature = "runtime-benchmarks")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "AI Compute PBC Collator".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "Ëtrid AI Compute Network - Decentralized GPU Marketplace\n\n\
         The collator node is responsible for:\n\
         - Producing blocks using Aura consensus\n\
         - Submitting checkpoints to FlareChain every 256 blocks\n\
         - Processing AI compute jobs and payments\n\
         - Managing GPU provider registry and reputation\n\n\
         Visit https://etrid.network for more information."
            .into()
    }

    fn author() -> String {
        "Ëtrid Foundation".into()
    }

    fn support_url() -> String {
        "https://github.com/etrid/etrid/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(crate::chain_spec::development_config()?),
            "local" => Box::new(crate::chain_spec::local_testnet_config()?),
            "" | "ai-compute-pbc" => Box::new(crate::chain_spec::mainnet_config()?),
            path => Box::new(crate::chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &ai_compute_pbc_runtime::VERSION
    }
}
