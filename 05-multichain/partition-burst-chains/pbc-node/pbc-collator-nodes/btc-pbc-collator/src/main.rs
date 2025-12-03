//! Generic PBC Collator Node
//!
//! This collator produces blocks for a Partition Burst Chain (PBC) and
//! submits state roots to Primearc Core Chain for multichain state aggregation.

mod chain_spec;
mod cli;
mod rpc;
mod service;
mod p2p_config;
mod p2p_bridge;

use clap::Parser;
use sc_cli::SubstrateCli;
use sp_core::crypto::Pair;

/// Prepare P2P configuration from CLI arguments
fn prepare_p2p_config(cli: &cli::Cli) -> sc_cli::Result<Option<p2p_config::P2PConfig>> {
    if !cli.p2p_enabled {
        return Ok(None);
    }

    // Parse bind address
    let bind_address = cli.p2p_bind_address
        .parse()
        .map_err(|e| sc_cli::Error::Input(format!("Invalid P2P bind address: {}", e)))?;

    // Parse announce address if provided
    let announce_address = if let Some(ref addr_str) = cli.p2p_announce_address {
        Some(
            addr_str
                .parse()
                .map_err(|e| sc_cli::Error::Input(format!("Invalid P2P announce address: {}", e)))?,
        )
    } else {
        None
    };

    // Parse bootstrap peers
    let bootstrap_peers = p2p_config::parse_bootstrap_peers(&cli.p2p_bootstrap_peers)
        .map_err(|e| sc_cli::Error::Input(format!("Invalid bootstrap peers: {}", e)))?;

    // Generate or derive local node ID from keystore
    // For now, we'll use a deterministic ID based on the node's network key
    // In production, this should be derived from the validator's cryptographic keys
    let local_node_id = {
        // Try to use the node's Ed25519 key if available
        let keypair = sp_core::ed25519::Pair::generate().0;
        let public_key_bytes = keypair.public();
        p2p_config::peer_id_from_public_key(public_key_bytes.as_ref())
    };

    let mut config = p2p_config::P2PConfig::new(local_node_id, bind_address, bootstrap_peers);

    if let Some(addr) = announce_address {
        config = config.with_announce_address(addr);
    }

    Ok(Some(config))
}

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

            // Prepare P2P configuration from CLI arguments
            let p2p_config = prepare_p2p_config(&cli)?;

            runner.run_node_until_exit(|config| async move {
                service::start_collator_with_p2p(config, p2p_config)
                    .await
                    .map_err(sc_cli::Error::Service)
            })
        }
    }
}