//! XLM-PBC Collator Node
//!
//! This collator produces blocks for the Stellar (XLM) Partition Burst Chain (PBC) and
//! submits state roots to Primearc Core Chain for multichain state aggregation.
//! Includes full DETR P2P networking support with auto-detection and maintenance.

mod chain_spec;
mod cli;
mod rpc;
mod service;
mod p2p_config;
mod p2p_bridge;

use clap::Parser;
use sc_cli::SubstrateCli;

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
                    use sp_core::crypto::Pair;
                    use crate::p2p_config::{P2PConfig, parse_bootstrap_peers, peer_id_from_public_key};
                    use detrp2p::PeerId;

                    // Derive node ID from keystore if available
                    // For now, use a deterministic ID based on the node's data directory
                    let node_id_bytes = sp_core::blake2_256(config.base_path.path().to_str().unwrap_or("default").as_bytes());
                    let local_node_id = PeerId::new(node_id_bytes);

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

                    let mut p2p_cfg = P2PConfig::new(local_node_id, bind_address, bootstrap_peers.clone());
                    p2p_cfg.enabled = cli.p2p_enabled;
                    if let Some(announce_addr) = announce_address {
                        p2p_cfg = p2p_cfg.with_announce_address(announce_addr);
                    }

                    log::info!("🌐 P2P Configuration:");
                    log::info!("  Enabled: {}", p2p_cfg.enabled);
                    log::info!("  Node ID: {:?}", local_node_id);
                    log::info!("  Bind Address: {}", bind_address);
                    log::info!("  Announce Address: {:?}", p2p_cfg.announce_address);
                    log::info!("  Bootstrap Peers: {}", bootstrap_peers.len());

                    Some(p2p_cfg)
                } else {
                    log::info!("ℹ️ P2P networking is disabled");
                    None
                };

                service::start_collator_with_p2p(config, p2p_config).await.map_err(sc_cli::Error::Service)
            })
        }
    }
}