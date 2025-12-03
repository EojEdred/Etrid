//! SC-USDT PBC Collator Node
//!
//! This collator produces blocks for the SC-USDT (Tether stablecoin) Partition Burst Chain
//! and submits state roots to Primearc Core Chain for multichain state aggregation.
//!
//! Features:
//! - DETR P2P networking with auto-detection and encryption
//! - Peer identity remapping for incoming connections
//! - Automatic reconnection and DHT maintenance
//! - Stablecoin-specific transaction handling

mod chain_spec;
mod cli;
mod rpc;
mod p2p_config;
mod p2p_bridge;
mod service;

use clap::Parser;
use sc_cli::SubstrateCli;
use crate::p2p_config::{P2PConfig, parse_bootstrap_peers, peer_id_from_public_key};

fn main() -> sc_cli::Result<()> {
    let cli = cli::Cli::parse();

    match &cli.subcommand {
        Some(cli::Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(cli::Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(cli::Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let sc_service::PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(cli::Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let sc_service::PartialComponents {
                    client,
                    task_manager,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(cli::Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let sc_service::PartialComponents {
                    client,
                    task_manager,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(cli::Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let sc_service::PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(cli::Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(cli::Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let sc_service::PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, backend, None), task_manager))
            })
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                // Build P2P configuration from CLI arguments
                let p2p_config = if cli.p2p_enabled {
                    // Parse bind address
                    let bind_address = cli.p2p_bind_address.parse()
                        .map_err(|e| sc_cli::Error::Input(format!("Invalid P2P bind address: {}", e)))?;

                    // Parse announce address if provided
                    let announce_address = if let Some(ref addr_str) = cli.p2p_announce_address {
                        Some(addr_str.parse()
                            .map_err(|e| sc_cli::Error::Input(format!("Invalid P2P announce address: {}", e)))?)
                    } else {
                        None
                    };

                    // Parse bootstrap peers
                    let bootstrap_peers = parse_bootstrap_peers(&cli.p2p_bootstrap_peers)
                        .map_err(|e| sc_cli::Error::Input(format!("Invalid bootstrap peers: {}", e)))?;

                    // Generate local node ID from PBC ID
                    // For production, derive from validator's session keys
                    let local_node_id = {
                        // Create deterministic node ID based on PBC ID
                        let dev_seed = format!("sc-usdt-pbc-{}", cli.pbc_id);
                        let mut node_id_bytes = [0u8; 32];
                        let seed_bytes = dev_seed.as_bytes();
                        let len = std::cmp::min(seed_bytes.len(), 32);
                        node_id_bytes[..len].copy_from_slice(&seed_bytes[..len]);
                        detrp2p::PeerId::new(node_id_bytes)
                    };

                    let mut cfg = P2PConfig::new(local_node_id, bind_address, bootstrap_peers);
                    cfg.enabled = true;
                    if let Some(announce) = announce_address {
                        cfg = cfg.with_announce_address(announce);
                    }

                    log::info!("🌐 SC-USDT-PBC: P2P networking enabled");
                    Some(cfg)
                } else {
                    log::info!("ℹ️ SC-USDT-PBC: P2P networking disabled");
                    None
                };

                service::start_collator_with_p2p(config, p2p_config)
                    .await
                    .map_err(sc_cli::Error::Service)
            })
        }
    }
}