//! Command handler

use crate::{
    chain_spec,
    cli::{Cli, Subcommand},
    service,
};
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use log::info;
use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::Block as BlockT;
use std::{io::Write, net::SocketAddr};

impl DefaultConfigurationValues for cli::RunCmd {
    fn p2p_listen_port() -> u16 {
        30333
    }

    fn rpc_listen_port() -> u16 {
        9944
    }

    fn prometheus_listen_port() -> u16 {
        9615
    }
}

/// Parse and run command line arguments
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
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
                let PartialComponents {
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
                let PartialComponents {
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
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = service::new_partial(&config)?;
                Ok((cmd.run(client, backend, None), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                let polkadot_cli = cumulus_client_cli::RelayChainCli::new(
                    &config,
                    [cumulus_client_cli::RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                cmd.run(config, polkadot_config)
            })
        }
        Some(Subcommand::ExportGenesisState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                let state_version = Cli::native_runtime_version(&spec).state_version();
                cmd.run::<ai_compute_pbc_runtime::opaque::Block>(&*spec, state_version)
            })
        }
        Some(Subcommand::ExportGenesisWasm(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|_config| {
                let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
                cmd.run(&*spec)
            })
        }
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run::<ai_compute_pbc_runtime::opaque::Block, service::ExecutorDispatch>(config))
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            let collator_options = cli.run.collator_options();

            runner.run_node_until_exit(|config| async move {
                // Para ID (15th PBC = 2015)
                let para_id = ParaId::from(2015);

                let hwbench = (!cli.run.no_hardware_benchmarks)
                    .then_some(config.database.path().map(|database_path| {
                        let _ = std::fs::create_dir_all(database_path);
                        sc_sysinfo::gather_hwbench(Some(database_path))
                    }))
                    .flatten();

                let polkadot_cli = cumulus_client_cli::RelayChainCli::new(
                    &config,
                    [cumulus_client_cli::RelayChainCli::executable_name()]
                        .iter()
                        .chain(cli.relay_chain_args.iter()),
                );

                let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
                    .map(|e| e.para_id)
                    .ok_or_else(|| "Could not find parachain ID in chain-spec.")?;

                let polkadot_config = SubstrateCli::create_configuration(
                    &polkadot_cli,
                    &polkadot_cli,
                    config.tokio_handle.clone(),
                )
                .map_err(|err| format!("Relay chain argument error: {}", err))?;

                info!("🚀 AI Compute PBC Collator");
                info!("⚙️  Para ID: {}", para_id);
                info!("💾 Database: RocksDB at {:?}", config.database);
                info!("⛓  Network: {}", config.network.chain_spec.name());

                service::start_parachain_node(config, polkadot_config, collator_options, para_id, hwbench)
                    .await
                    .map(|r| r.0)
                    .map_err(Into::into)
            })
        }
    }
}

use sc_service::PartialComponents;
use cumulus_client_cli::RelayChainCli;
