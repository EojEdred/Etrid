use frame_support::{
	storage::migration::{clear_storage_prefix, storage_key_iter},
	storage::types::{StorageValue, ValueQuery},
	traits::{OnRuntimeUpgrade, StorageInstance},
	weights::Weight,
	Blake2_128Concat,
};
use parity_scale_codec::Decode;

use crate::{Runtime, TokenMessengerMinBurnAmount};

#[derive(Decode)]
struct OldDomainConfig {
	enabled: bool,
	max_burn_amount: u128,
	daily_burn_limit: u128,
}

struct MigrationMarker;

impl StorageInstance for MigrationMarker {
	const STORAGE_PREFIX: &'static str = "EdscBridgeMigration";

	fn pallet_prefix() -> &'static str {
		"RuntimeUpgrade"
	}
}

type MigrationComplete = StorageValue<MigrationMarker, bool, ValueQuery>;

pub struct EdscBridgeMigrations;

impl OnRuntimeUpgrade for EdscBridgeMigrations {
	fn on_runtime_upgrade() -> Weight {
		let db_weight = <Runtime as frame_system::Config>::DbWeight::get();
		let mut weight = Weight::zero();

		if MigrationComplete::get() {
			return weight;
		}

		for (domain, nonce) in storage_key_iter::<u32, u64, Blake2_128Concat>(
			b"TokenMessenger",
			b"Nonce",
		)
		.drain()
		{
			let current = pallet_token_messenger::MessageNonce::<Runtime>::get(domain);
			weight = weight.saturating_add(db_weight.reads(1));

			if nonce > current {
				pallet_token_messenger::MessageNonce::<Runtime>::insert(domain, nonce);
				weight = weight.saturating_add(db_weight.writes(1));
			}
		}

		for (domain, old_config) in storage_key_iter::<u32, OldDomainConfig, Blake2_128Concat>(
			b"TokenMessenger",
			b"DomainConfigs",
		)
		.drain()
		{
			let new_config = pallet_token_messenger::DomainConfig {
				enabled: old_config.enabled,
				max_burn_amount: old_config.max_burn_amount,
				daily_burn_limit: old_config.daily_burn_limit,
				min_burn_amount: TokenMessengerMinBurnAmount::get(),
			};

			pallet_token_messenger::DomainConfigs::<Runtime>::insert(domain, new_config);
			pallet_token_messenger::SupportedDomains::<Runtime>::insert(domain, old_config.enabled);
			weight = weight.saturating_add(db_weight.reads_writes(1, 2));
		}

		let _ = clear_storage_prefix(b"BridgeAttestation", b"Attestations", &[], None, None);
		let _ = clear_storage_prefix(b"BridgeAttestation", b"UsedNonces", &[], None, None);
		pallet_bridge_attestation::AttestationNonce::<Runtime>::put(0u64);
		weight = weight.saturating_add(db_weight.writes(1));

		MigrationComplete::put(true);
		weight = weight.saturating_add(db_weight.writes(1));

		weight
	}
}
