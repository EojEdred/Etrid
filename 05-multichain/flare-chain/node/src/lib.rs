//! FlareChain Node Library
//!
//! This library exports the ASF consensus service for use by the unified ËTRID binary.
//! FlareChain uses FODDoS ASF consensus (PPFA block production + hybrid GRANDPA finality).

#[path = "chain-spec.rs"]
pub mod chain_spec;
pub mod rpc;

/// ASF consensus service integration
/// Exports new_full() and new_partial() for the unified binary
pub mod asf_service;

/// ASF RPC endpoints for querying consensus state
pub mod asf_rpc;

/// ASF telemetry integration for consensus metrics
pub mod asf_telemetry;

// ═══════════════════════════════════════════════════════════════════════════════
// HYBRID CONSENSUS MODE SELECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a new full node with pure ASF consensus
///
/// v108: Always uses ASF consensus (no GRANDPA fallback)
/// - Block Production: PPFA (ASF)
/// - Finality: ASF Finality Gadget (pure)
pub fn new_full(
    config: sc_service::Configuration,
) -> Result<sc_service::TaskManager, sc_service::error::Error> {
    log::info!("🔥 Starting FlareChain node in PURE ASF mode (v108)");
    log::info!("   Block Production: PPFA (ASF)");
    log::info!("   Finality: ASF Finality Gadget (pure)");
    asf_service::new_full(config)
}

/// Create partial node components (pure ASF)
pub fn new_partial(
    config: &sc_service::Configuration,
) -> Result<asf_service::AsfFullParts, sc_service::error::Error> {
    asf_service::new_partial(config)
}

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
