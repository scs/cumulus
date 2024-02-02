// Copyright 2021 Integritee AG and Supercomputing Systems AG
// This file is part of the "Integritee parachain" and is
// based on Cumulus from Parity Technologies (UK) Ltd.

// Integritee parachain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Integritee parachain.  If not, see <http://www.gnu.org/licenses/>.

//!
//! this file DOES HAVE CUSTOMIZATIONS for integritee runtimes. Upon upgrades of polkadot-sdk,
//! look for blocks with `is_shell()` branching

use crate::{
	chain_spec,
	chain_spec::{
		integritee_chain_spec, integritee_moonbase_config, shell_chain_spec, shell_kusama_config,
		shell_kusama_lease2_config, shell_kusama_lease3_config, shell_polkadot_config,
		shell_rococo_config, shell_westend_config, GenesisKeys, RelayChain, ShellChainSpec,
	},
	cli::{Cli, RelayChainCli, Subcommand},
};
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use integritee_runtime::Block;
use log::info;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_runtime::traits::AccountIdConversion;
use std::net::SocketAddr;

const LOCAL_PARA_ID: u32 = 2015;
const ROCOCO_PARA_ID: u32 = 2015;
const WESTEND_PARA_ID: u32 = 2081;
const KUSAMA_PARA_ID: u32 = 2015;
const KUSAMA_SWAP_PARA_ID: u32 = 2267;
const POLKADOT_PARA_ID: u32 = 2039;
const MOONBASE_PARA_ID: u32 = 2015;

trait IdentifyChain {
	fn is_shell(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
	fn is_shell(&self) -> bool {
		self.name().starts_with("Integritee Shell")
	}
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
	fn is_shell(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_shell(self)
	}
}

// If we don't skipp here, each cmd expands to 5 lines. I think we have better overview like this.
#[rustfmt::skip]
fn load_spec(
	id: &str,
) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match id {
		// live configs (hard coded genesis state. genesis will always be shell for a live system)
		"integritee-rococo" => Box::new(shell_rococo_config()?),
		"integritee-westend" => Box::new(shell_westend_config()?),
		"integritee-kusama" => Box::new(shell_kusama_config()?),
		"integritee-polkadot" => Box::new(shell_polkadot_config()?),
		"integritee-moonbase" => Box::new(integritee_moonbase_config()?),
		// chain-spec that has been registered for the next kusama slot lease
		"shell-kusama-lease2" => Box::new(shell_kusama_lease2_config()?),
		"shell-kusama-lease3" => Box::new(shell_kusama_lease3_config()?),

		// live config initialize
		"integritee-rococo-fresh" => Box::new(shell_chain_spec(ROCOCO_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::Rococo)),
		"integritee-westend-fresh" => Box::new(shell_chain_spec(WESTEND_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::Westend)),
		"integritee-kusama-fresh" => Box::new(shell_chain_spec(KUSAMA_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::Kusama)),
		"integritee-polkadot-fresh" => Box::new(shell_chain_spec(POLKADOT_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::Polkadot)),
		"integritee-moonbase-fresh" => Box::new(integritee_chain_spec(MOONBASE_PARA_ID.into(), GenesisKeys::IntegriteeDev, RelayChain::Moonbase)),
		"shell-kusama-fresh" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::Kusama)),

		// on-the-spot specs
		"integritee-rococo-local" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::RococoLocal)),
		"integritee-rococo-local-dev" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::RococoLocal)),

		"integritee-westend-local" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::WestendLocal)),
		"integritee-westend-local-dev" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::WestendLocal)),

		"integritee-kusama-local" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::KusamaLocal)),
		"integritee-kusama-local-dev" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::KusamaLocal)),

		"integritee-polkadot-local" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::PolkadotLocal)),
		"integritee-polkadot-local-dev" => Box::new(integritee_chain_spec(LOCAL_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::PolkadotLocal)),

		"shell-rococo-local" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::RococoLocal)),
		"shell-rococo-local-dev" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::RococoLocal)),

		"shell-westend-local" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::WestendLocal)),
		"shell-westend-local-dev" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::WestendLocal)),

		"shell-kusama-local" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::KusamaLocal)),
		"shell-kusama-local-dev" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::KusamaLocal)),

		"shell-polkadot-local" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::Integritee, RelayChain::PolkadotLocal)),
		"shell-polkadot-local-dev" => Box::new(shell_chain_spec(KUSAMA_SWAP_PARA_ID.into(), GenesisKeys::WellKnown, RelayChain::PolkadotLocal)),

		"" => panic!("Please supply chain_spec to be loaded."),
		path => {
			let chain_spec = chain_spec::IntegriteeChainSpec::from_json_file(path.into())?;
			if chain_spec.is_shell() {
				Box::new(ShellChainSpec::from_json_file(path.into())?)
			} else {
				Box::new(chain_spec)
			}
		}
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Integritee Collator".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Integritee Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/integritee-network/parachain/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2019
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Integritee Collator".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Integritee Collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/integritee-network/parachain/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2019
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		if runner.config().chain_spec.is_shell() {
			runner.async_run(|$config| {
				let $components = crate::service_shell::new_partial(
					&$config,
				)?;
				let task_manager = $components.task_manager;
				{ $( $code )* }.map(|v| (v, task_manager))
			})
		} else {
			runner.async_run(|$config| {
			let $components = crate::service::new_partial(
				&$config,
			)?;
			let task_manager = $components.task_manager;
			{ $( $code )* }.map(|v| (v, task_manager))
		})
		}
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::Revert(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.backend, None))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
					.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::ExportGenesisState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			if runner.config().chain_spec.is_shell() {
				runner.sync_run(|config| {
					let partials = crate::service_shell::new_partial(&config)?;
					cmd.run(&*config.chain_spec, &*partials.client)
				})

			} else {
				runner.sync_run(|config| {
					let partials = crate::service::new_partial(&config)?;
					cmd.run(&*config.chain_spec, &*partials.client)
				})
			}
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			// Switch on the concrete benchmark sub-command-
			match cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| cmd.run::<Block, ()>(config))
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
							.into())
					},
				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					// there's no point in benchmarking the shell runitme
					let partials = crate::service::new_partial(&config)?;
					cmd.run(partials.client)
				}),
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) =>
					return Err(sc_cli::Error::Input(
						"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
							.into(),
					)
						.into()),
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					let partials = new_partial(&config)?;
					let db = partials.backend.expose_db();
					let storage = partials.backend.expose_storage();
					cmd.run(config, partials.client.clone(), db, storage)
				}),
				BenchmarkCmd::Machine(cmd) =>
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())),
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				#[allow(unreachable_patterns)]
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		},
		Some(Subcommand::TryRuntime) => Err("The `try-runtime` subcommand has been migrated to a standalone CLI (https://github.com/paritytech/try-runtime-cli). It is no longer being maintained here and will be removed entirely some time after January 2024. Please remove this subcommand from your runtime and use the standalone CLI.".into()),
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let hwbench = (!cli.no_hardware_benchmarks)
					.then_some(config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					}))
					.flatten();

				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain ID in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
						&id,
					);

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain Account: {parachain_account}");
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				if config.chain_spec.is_shell() {
					crate::service_shell::start_parachain_node(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
					)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
				} else {
					crate::service::start_parachain_node(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
					)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
				}
			})
		},
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_listen_port() -> u16 {
		9945
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_addr(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_max_connections(&self) -> Result<u32> {
		self.base.base.rpc_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}
