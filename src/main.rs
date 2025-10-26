//! # ËTRID Unified Node Binary
//!
//! This is the main entry point for the ËTRID multichain protocol.
//! It can run as:
//! - FlareChain validator node (root chain with FODDoS ASF consensus)
//! - PBC collator node (any of 12 partition burst chains)
//!
//! ## Usage Examples
//!
//! ```bash
//! # Run FlareChain validator
//! etrid --chain flare --validator
//!
//! # Run BTC PBC collator
//! etrid --chain btc-pbc --collator
//!
//! # Run ETH PBC collator
//! etrid --chain eth-pbc --collator
//!
//! # Development mode
//! etrid --chain flare --dev
//!
//! # Subcommands
//! etrid build-spec --chain flare
//! etrid key generate
//! ```

#![warn(missing_docs)]

use clap::Parser;
use sc_cli::SubstrateCli;
use std::path::PathBuf;

// Import ASF consensus service from FlareChain
use flare_chain_node::asf_service;

/// ËTRID Chain Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainType {
    /// FlareChain - Root chain with FODDoS ASF consensus
    Flare,
    /// Bitcoin Partition Burst Chain
    BtcPbc,
    /// Ethereum Partition Burst Chain
    EthPbc,
    /// Solana Partition Burst Chain
    SolPbc,
    /// Stellar/XLM Partition Burst Chain
    XlmPbc,
    /// Ripple/XRP Partition Burst Chain
    XrpPbc,
    /// Binance Chain Partition Burst Chain
    BnbPbc,
    /// Tron Partition Burst Chain
    TrxPbc,
    /// Cardano Partition Burst Chain
    AdaPbc,
    /// Chainlink Partition Burst Chain
    LinkPbc,
    /// Polygon/Matic Partition Burst Chain
    MaticPbc,
    /// Smart Contract USDT Partition Burst Chain
    ScUsdtPbc,
    /// Dogecoin Partition Burst Chain
    DogePbc,
}

impl ChainType {
    /// Parse chain type from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "flare" | "flarechain" => Ok(ChainType::Flare),
            "btc-pbc" | "btc" => Ok(ChainType::BtcPbc),
            "eth-pbc" | "eth" => Ok(ChainType::EthPbc),
            "sol-pbc" | "sol" | "solana" => Ok(ChainType::SolPbc),
            "xlm-pbc" | "xlm" | "stellar" => Ok(ChainType::XlmPbc),
            "xrp-pbc" | "xrp" | "ripple" => Ok(ChainType::XrpPbc),
            "bnb-pbc" | "bnb" | "binance" => Ok(ChainType::BnbPbc),
            "trx-pbc" | "trx" | "tron" => Ok(ChainType::TrxPbc),
            "ada-pbc" | "ada" | "cardano" => Ok(ChainType::AdaPbc),
            "link-pbc" | "link" | "chainlink" => Ok(ChainType::LinkPbc),
            "matic-pbc" | "matic" | "polygon" => Ok(ChainType::MaticPbc),
            "sc-usdt-pbc" | "usdt" => Ok(ChainType::ScUsdtPbc),
            "doge-pbc" | "doge" | "dogecoin" => Ok(ChainType::DogePbc),
            _ => Err(format!("Unknown chain type: {}", s)),
        }
    }

    /// Get runtime name for logging
    pub fn runtime_name(&self) -> &'static str {
        match self {
            ChainType::Flare => "FlareChain",
            ChainType::BtcPbc => "Bitcoin PBC",
            ChainType::EthPbc => "Ethereum PBC",
            ChainType::SolPbc => "Solana PBC",
            ChainType::XlmPbc => "Stellar PBC",
            ChainType::XrpPbc => "Ripple PBC",
            ChainType::BnbPbc => "Binance PBC",
            ChainType::TrxPbc => "Tron PBC",
            ChainType::AdaPbc => "Cardano PBC",
            ChainType::LinkPbc => "Chainlink PBC",
            ChainType::MaticPbc => "Polygon PBC",
            ChainType::ScUsdtPbc => "Smart Contract USDT PBC",
            ChainType::DogePbc => "Dogecoin PBC",
        }
    }

    /// Check if this is a PBC (collator) chain
    pub fn is_pbc(&self) -> bool {
        !matches!(self, ChainType::Flare)
    }
}

/// ËTRID Node CLI
#[derive(Debug, Parser)]
#[command(
    name = "etrid",
    about = "ËTRID Multichain Protocol Node",
    version = env!("CARGO_PKG_VERSION"),
    author = "ËTRID Foundation"
)]
pub struct Cli {
    /// Specify which chain to run
    #[arg(
        long,
        value_name = "CHAIN",
        default_value = "flare",
        help = "Chain to run: flare, btc-pbc, eth-pbc, sol-pbc, xlm-pbc, xrp-pbc, bnb-pbc, trx-pbc, ada-pbc, link-pbc, matic-pbc, sc-usdt-pbc, doge-pbc"
    )]
    pub chain: String,

    /// Run as collator (for PBC chains)
    #[arg(long, help = "Run as collator (for PBC chains only)")]
    pub collator: bool,

    /// Subcommands
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Standard Substrate run command options (includes --validator flag)
    #[command(flatten)]
    pub run: sc_cli::RunCmd,
}

/// Available subcommands
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

    /// Database meta columns information
    ChainInfo(sc_cli::ChainInfoCmd),

    /// Try runtime subcommands
    #[cfg(feature = "try-runtime")]
    TryRuntime(try_runtime_cli::TryRuntimeCmd),

    /// Benchmark subcommands
    #[cfg(feature = "runtime-benchmarks")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "ËTRID Node".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "ËTRID Multichain Protocol - FlareChain Root + PBC Collators".into()
    }

    fn author() -> String {
        "ËTRID Foundation".into()
    }

    fn support_url() -> String {
        "https://github.com/etrid/etrid/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2024
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        let chain_type = ChainType::from_str(&self.chain)?;

        match chain_type {
            ChainType::Flare => {
                // Load FlareChain spec
                log::info!("Loading FlareChain specification: {}", id);
                Ok(match id {
                    "dev" => Box::new(flare_chain_node::chain_spec::development_config()?),
                    "local" => Box::new(flare_chain_node::chain_spec::local_testnet_config()?),
                    "staging" | "ember" => Box::new(flare_chain_node::chain_spec::staging_testnet_config()?),
                    "" | "flarechain" => Box::new(flare_chain_node::chain_spec::flarechain_config()?),
                    path => Box::new(flare_chain_node::chain_spec::ChainSpec::from_json_file(
                        std::path::PathBuf::from(path),
                    )?),
                })
            }
            _ => {
                // Load PBC spec
                log::info!("Loading {} specification: {}", chain_type.runtime_name(), id);
                Err(format!("{} specs not yet integrated", chain_type.runtime_name()))
            }
        }
    }
}

/// Main entry point
fn main() -> sc_cli::Result<()> {
    let cli = Cli::parse();

    // Parse chain type
    let chain_type = ChainType::from_str(&cli.chain)
        .map_err(|e| sc_cli::Error::Input(e))?;

    // Validate validator/collator flags (only when running node, not for subcommands)
    if cli.subcommand.is_none() {
        // Running the node - validate correct flag for chain type
        if chain_type.is_pbc() {
            // PBC chains must use --collator
            if !cli.collator {
                return Err(sc_cli::Error::Input(
                    format!("{} requires --collator flag", chain_type.runtime_name())
                ));
            }
            if cli.run.validator {
                log::warn!("--validator flag ignored for PBC chains (use --collator)");
            }
        } else {
            // FlareChain must use --validator
            if !cli.run.validator {
                return Err(sc_cli::Error::Input(
                    "FlareChain requires --validator flag".into()
                ));
            }
            if cli.collator {
                return Err(sc_cli::Error::Input(
                    "Cannot use --collator with FlareChain. Use --validator instead.".into()
                ));
            }
        }
    }

    log::info!("🚀 Starting ËTRID Node");
    log::info!("📡 Chain: {}", chain_type.runtime_name());
    log::info!("🔗 Mode: {}", if chain_type.is_pbc() { "Collator" } else { "Validator" });

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            log::warn!("CheckBlock not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            log::warn!("ExportBlocks not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        Some(Subcommand::ExportState(cmd)) => {
            log::warn!("ExportState not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            log::warn!("ImportBlocks not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            log::warn!("Revert not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        Some(Subcommand::ChainInfo(cmd)) => {
            log::warn!("ChainInfo not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        #[cfg(feature = "try-runtime")]
        Some(Subcommand::TryRuntime(cmd)) => {
            log::warn!("TryRuntime not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            log::warn!("Benchmark not yet implemented for unified node");
            Err(sc_cli::Error::Input("Not implemented".into()))
        }
        None => {
            // Run the node
            let runner = cli.create_runner(&cli.run)?;

            match chain_type {
                ChainType::Flare => {
                    log::info!("🔥 Starting FlareChain validator node...");
                    log::info!("⚙️  Consensus: FODDoS ASF Algorithm");
                    log::info!("🌐 Network: Custom DETR P2P + Substrate Networking");

                    // Start FlareChain with ASF consensus service
                    runner.run_node_until_exit(|config| async move {
                        log::info!("🔥 Starting FlareChain with ASF consensus...");
                        asf_service::new_full(config)
                            .map_err(sc_cli::Error::Service)
                    })
                }
                _ => {
                    log::info!("⚡ Starting {} collator node...", chain_type.runtime_name());
                    log::info!("🔗 Relay: Connected to FlareChain");
                    log::info!("🌐 Network: Cumulus + Custom DETR P2P");

                    // TODO: Create PBC ASF services similar to FlareChain
                    // Each PBC will need its own ASF service integration:
                    // - btc_pbc_node::asf_service::new_full(config)
                    // - eth_pbc_node::asf_service::new_full(config)
                    // - etc.
                    runner.run_node_until_exit(|config| async move {
                        log::error!("{} collator service not yet integrated", chain_type.runtime_name());
                        log::error!("TODO: Create PBC ASF services similar to FlareChain");
                        Err(sc_cli::Error::Input("Not implemented - PBC ASF services pending".into()))
                    })
                }
            }
        }
    }
}

// =============================================================================
// FUTURE INTEGRATION POINTS
// =============================================================================
//
// TODO: Phase 2 - Integrate Services
// 1. Import flare-chain service module
// 2. Import PBC collator services (btc, eth, sol, etc.)
// 3. Wire up chain-spec loaders
// 4. Configure RPC endpoints per chain
// 5. Set up telemetry and monitoring
//
// TODO: Phase 3 - Custom P2P Integration
// 1. Initialize detr-p2p alongside Substrate networking
// 2. Configure encrypted ECIES communication
// 3. Set up Kademlia DHT for peer discovery
// 4. Bridge custom P2P with Substrate networking layer
//
// TODO: Phase 4 - Consensus Integration
// 1. Wire up FODDoS ASF consensus for FlareChain
// 2. Configure validator orchestration
// 3. Set up consensus day scheduling
// 4. Integrate minting and distribution logic
//
// TODO: Phase 5 - Cross-Chain Bridge
// 1. Initialize bridge protocols for each PBC
// 2. Set up relay between FlareChain and PBCs
// 3. Configure message passing
// 4. Enable cross-chain transactions
//
// =============================================================================
