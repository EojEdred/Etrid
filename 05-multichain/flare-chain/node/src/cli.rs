use sc_cli::{RunCmd, SubstrateCli};

/// FlareChain command line interface
#[derive(Debug, clap::Parser)]
pub struct Cli {
    /// Possible subcommands of the main binary
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub run: RunCmd,
}

/// Possible subcommands
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management commands
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

    /// Purge chain data
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert chain to a previous state
    Revert(sc_cli::RevertCmd),

    /// Database benchmark
    #[command(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Ëtrid FlareChain".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "Ëtrid FlareChain - Main Relay Chain Validator".into()
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
            "test_2val" | "test-2val" => Box::new(crate::chain_spec::test_2validator_config()?),
            "staging" | "ember" => Box::new(crate::chain_spec::staging_testnet_config()?),
            "mainnet" | "flarechain_mainnet" => Box::new(crate::chain_spec::mainnet_config()?),
            "" | "flarechain" => Box::new(crate::chain_spec::flarechain_config()?),
            path => Box::new(crate::chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }
}