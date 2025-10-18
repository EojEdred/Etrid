//! FlareChain Node Library
//!
//! This library exports the ASF consensus service for use by the unified ËTRID binary.
//! FlareChain uses FODDoS ASF consensus (PPFA block production + hybrid GRANDPA finality).

#[path = "chain-spec.rs"]
pub mod chain_spec;
pub mod rpc;
// pub mod service;  // Old Aura-based service - replaced by asf_service

/// ASF consensus service integration
/// Exports new_full() and new_partial() for the unified binary
pub mod asf_service;

// Re-export CLI types if needed by the unified binary
mod cli {
    use clap::Parser;
    use sc_cli::SubstrateCli;
    use crate::chain_spec;

    #[derive(Debug, Parser)]
    pub struct Cli {
        #[command(subcommand)]
        pub subcommand: Option<Subcommand>,

        #[command(flatten)]
        pub run: sc_cli::RunCmd,
    }

    #[derive(Debug, clap::Subcommand)]
    pub enum Subcommand {
        #[command(subcommand)]
        Key(sc_cli::KeySubcommand),
        BuildSpec(sc_cli::BuildSpecCmd),
        CheckBlock(sc_cli::CheckBlockCmd),
        ExportBlocks(sc_cli::ExportBlocksCmd),
        ExportState(sc_cli::ExportStateCmd),
        ImportBlocks(sc_cli::ImportBlocksCmd),
        PurgeChain(sc_cli::PurgeChainCmd),
        Revert(sc_cli::RevertCmd),
        ChainInfo(sc_cli::ChainInfoCmd),
    }

    impl SubstrateCli for Cli {
        fn impl_name() -> String {
            "Ëtrid FlareChain Node".into()
        }

        fn impl_version() -> String {
            env!("CARGO_PKG_VERSION").into()
        }

        fn description() -> String {
            env!("CARGO_PKG_DESCRIPTION").into()
        }

        fn author() -> String {
            env!("CARGO_PKG_AUTHORS").into()
        }

        fn support_url() -> String {
            "https://github.com/etrid/etrid/issues".into()
        }

        fn copyright_start_year() -> i32 {
            2024
        }

        fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
            Ok(match id {
                "dev" => Box::new(chain_spec::development_config()?),
                "" | "local" => Box::new(chain_spec::local_testnet_config()?),
                path => Box::new(chain_spec::ChainSpec::from_json_file(
                    std::path::PathBuf::from(path),
                )?),
            })
        }
    }
}

pub use cli::Cli;
