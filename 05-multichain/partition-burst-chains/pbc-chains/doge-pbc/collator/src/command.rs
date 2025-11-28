//! Command execution for DOGE-PBC Collator

use crate::cli::{Cli, Subcommand};
use crate::service;
use sc_cli::{SubstrateCli, RuntimeVersion, Result};
use sc_service::Configuration;
use doge_pbc_runtime::Block;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "DOGE-PBC Collator".into()
    }

    fn impl_version() -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn description() -> String {
        "Dogecoin Partition Burst Chain Collator".into()
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

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        crate::chain_spec::load_spec(id)
    }
}

/// Parse and run command line arguments
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
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
        Some(Subcommand::ExportBlocks(cmd)) => {
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
        Some(Subcommand::ExportState(cmd)) => {
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
        Some(Subcommand::ImportBlocks(cmd)) => {
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
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
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
        Some(Subcommand::ChainInfo(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run::<Block>(&config))
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;

            // Set P2P environment variables from CLI args if provided
            if let Some(listen) = &cli.p2p_listen {
                std::env::set_var("DETR_P2P_LISTEN_ADDR", listen);
            }
            if let Some(announce) = &cli.p2p_announce_ip {
                std::env::set_var("DETR_P2P_ANNOUNCE_IP", announce);
            }
            if let Some(bootstrap) = &cli.p2p_bootstrap {
                std::env::set_var("DETR_P2P_BOOTSTRAP_PEERS", bootstrap);
            }
            std::env::set_var("DETR_P2P_MAX_CONNECTIONS", cli.p2p_max_connections.to_string());

            runner.run_node_until_exit(|config| async move {
                service::start_collator(config)
                    .await
                    .map_err(sc_cli::Error::Service)
            })
        }
    }
}
