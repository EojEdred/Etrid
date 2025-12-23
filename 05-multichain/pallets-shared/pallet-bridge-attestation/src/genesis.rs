//! Genesis configuration for Bridge Attestation Pallet
//!
//! This module provides genesis configuration capabilities to initialize
//! attesters at chain genesis, avoiding the need to register them via
//! governance after launch.

use super::*;
use frame_support::pallet_prelude::*;
use sp_std::vec::Vec;

/// Genesis attester configuration
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct GenesisAttester {
	/// Attester's public key (32 bytes for SR25519, 33 for ECDSA compressed, 65 for uncompressed)
	pub public_key: Vec<u8>,
	/// Attester name (for identification)
	pub name: Vec<u8>,
	/// Initial status (typically Active)
	pub enabled: bool,
}

/// Genesis threshold configuration
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct GenesisThreshold {
	/// Minimum signatures required (M in M-of-N)
	pub min_signatures: u32,
	/// Total attesters (N in M-of-N)
	pub total_attesters: u32,
}

impl Default for GenesisThreshold {
	fn default() -> Self {
		Self {
			min_signatures: 3,
			total_attesters: 5,
		}
	}
}

/// Genesis configuration for the Bridge Attestation pallet
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct GenesisConfig<T: Config> {
	/// List of attesters to register at genesis
	pub attesters: Vec<GenesisAttester>,
	/// Global threshold configuration
	pub threshold: GenesisThreshold,
	/// Domain-specific threshold configurations
	pub domain_thresholds: Vec<(u32, GenesisThreshold)>,
	/// Phantom data for type parameter
	#[cfg_attr(feature = "std", serde(skip))]
	pub _phantom: sp_std::marker::PhantomData<T>,
}

impl<T: Config> Default for GenesisConfig<T> {
	fn default() -> Self {
		Self {
			attesters: Vec::new(),
			threshold: GenesisThreshold::default(),
			domain_thresholds: Vec::new(),
			_phantom: Default::default(),
		}
	}
}

impl<T: Config> GenesisConfig<T> {
	/// Build genesis configuration
	pub fn build(&self) {
		// Initialize attestation nonce
		<AttestationNonce<T>>::put(0u64);

		// Initialize pause state
		<IsPaused<T>>::put(false);

		// Initialize total attestations counter
		<TotalAttestations<T>>::put(0u64);

		// Register attesters
		let mut active_count = 0u32;
		let current_block = frame_system::Pallet::<T>::block_number();

		for (index, genesis_attester) in self.attesters.iter().enumerate() {
			let attester_id = index as u32;

			// Validate public key length
			if genesis_attester.public_key.len() != 32
				&& genesis_attester.public_key.len() != 33
				&& genesis_attester.public_key.len() != 65
			{
				panic!(
					"Invalid public key length for attester {}: {} bytes",
					attester_id,
					genesis_attester.public_key.len()
				);
			}

			// Convert to bounded vec
			let bounded_key: BoundedVec<u8, ConstU32<64>> = genesis_attester
				.public_key
				.clone()
				.try_into()
				.expect("Public key too long");

			// Check for duplicate public keys
			if <AttesterByPubkey<T>>::contains_key(&bounded_key) {
				panic!(
					"Duplicate public key in genesis config for attester {}",
					attester_id
				);
			}

			// Determine status
			let status = if genesis_attester.enabled {
				active_count = active_count.saturating_add(1);
				pallet::AttesterStatus::Active
			} else {
				pallet::AttesterStatus::Disabled
			};

			// Create attester info
			let attester_info = pallet::AttesterInfo {
				public_key: bounded_key.clone(),
				status,
				registered_at: current_block,
				messages_signed: 0,
				last_signed_at: current_block,
			};

			// Store attester
			<Attesters<T>>::insert(attester_id, attester_info);
			<AttesterByPubkey<T>>::insert(bounded_key, attester_id);
		}

		// Set next attester ID
		<NextAttesterId<T>>::put(self.attesters.len() as u32);

		// Set active attester count
		<ActiveAttesterCount<T>>::put(active_count);

		// Configure global threshold
		let global_threshold = pallet::ThresholdConfig {
			min_signatures: self.threshold.min_signatures,
			total_attesters: self.threshold.total_attesters,
			enabled: true,
		};
		<GlobalThreshold<T>>::put(global_threshold);

		// Configure domain-specific thresholds
		for (domain_id, threshold_config) in self.domain_thresholds.iter() {
			let threshold = pallet::ThresholdConfig {
				min_signatures: threshold_config.min_signatures,
				total_attesters: threshold_config.total_attesters,
				enabled: true,
			};
			<ThresholdConfigs<T>>::insert(domain_id, threshold);
		}

		// Validate threshold configuration
		if self.threshold.min_signatures == 0
			|| self.threshold.min_signatures > self.threshold.total_attesters
		{
			panic!(
				"Invalid threshold configuration: {}/{}",
				self.threshold.min_signatures, self.threshold.total_attesters
			);
		}

		// Validate active count meets threshold
		if active_count < self.threshold.min_signatures {
			panic!(
				"Insufficient active attesters: {} active, {} required",
				active_count, self.threshold.min_signatures
			);
		}

		// Log configuration
		log::info!(
			"Bridge Attestation Genesis: Registered {} attesters ({} active) with {}-of-{} threshold",
			self.attesters.len(),
			active_count,
			self.threshold.min_signatures,
			self.threshold.total_attesters
		);
	}
}

/// Build storage for genesis configuration
#[cfg(feature = "std")]
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
	fn build(&self) {
		GenesisConfig::build(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Runtime};
	use sp_core::H256;

	fn generate_test_key(id: u8) -> Vec<u8> {
		let mut key = [0u8; 33];
		key[0] = 0x02; // ECDSA compressed prefix
		key[1] = id;
		key.to_vec()
	}

	#[test]
	fn genesis_config_builds_successfully() {
		new_test_ext().execute_with(|| {
			let config = GenesisConfig::<Runtime> {
				attesters: vec![
					GenesisAttester {
						public_key: generate_test_key(1),
						name: b"Attester-1".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(2),
						name: b"Attester-2".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(3),
						name: b"Attester-3".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(4),
						name: b"Attester-4".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(5),
						name: b"Attester-5".to_vec(),
						enabled: true,
					},
				],
				threshold: GenesisThreshold {
					min_signatures: 3,
					total_attesters: 5,
				},
				domain_thresholds: vec![],
				_phantom: Default::default(),
			};

			config.build();

			// Verify attesters were registered
			assert_eq!(crate::NextAttesterId::<Runtime>::get(), 5);
			assert_eq!(crate::ActiveAttesterCount::<Runtime>::get(), 5);

			// Verify threshold was set
			let threshold = crate::GlobalThreshold::<Runtime>::get().unwrap();
			assert_eq!(threshold.min_signatures, 3);
			assert_eq!(threshold.total_attesters, 5);

			// Verify specific attester
			let attester = crate::Attesters::<Runtime>::get(0).unwrap();
			assert_eq!(attester.status, crate::AttesterStatus::Active);
			assert_eq!(attester.messages_signed, 0);
		});
	}

	#[test]
	fn genesis_config_with_disabled_attesters() {
		new_test_ext().execute_with(|| {
			let config = GenesisConfig::<Runtime> {
				attesters: vec![
					GenesisAttester {
						public_key: generate_test_key(1),
						name: b"Attester-1".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(2),
						name: b"Attester-2".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(3),
						name: b"Attester-3".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(4),
						name: b"Attester-4".to_vec(),
						enabled: false, // Disabled
					},
					GenesisAttester {
						public_key: generate_test_key(5),
						name: b"Attester-5".to_vec(),
						enabled: false, // Disabled
					},
				],
				threshold: GenesisThreshold {
					min_signatures: 3,
					total_attesters: 5,
				},
				domain_thresholds: vec![],
				_phantom: Default::default(),
			};

			config.build();

			// Verify counts
			assert_eq!(crate::NextAttesterId::<Runtime>::get(), 5);
			assert_eq!(crate::ActiveAttesterCount::<Runtime>::get(), 3); // Only 3 active

			// Verify statuses
			assert_eq!(
				crate::Attesters::<Runtime>::get(0).unwrap().status,
				crate::AttesterStatus::Active
			);
			assert_eq!(
				crate::Attesters::<Runtime>::get(3).unwrap().status,
				crate::AttesterStatus::Disabled
			);
		});
	}

	#[test]
	fn genesis_config_with_domain_thresholds() {
		new_test_ext().execute_with(|| {
			let config = GenesisConfig::<Runtime> {
				attesters: vec![
					GenesisAttester {
						public_key: generate_test_key(1),
						name: b"Attester-1".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(2),
						name: b"Attester-2".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(3),
						name: b"Attester-3".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(4),
						name: b"Attester-4".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(5),
						name: b"Attester-5".to_vec(),
						enabled: true,
					},
				],
				threshold: GenesisThreshold {
					min_signatures: 3,
					total_attesters: 5,
				},
				domain_thresholds: vec![
					(
						1,
						GenesisThreshold {
							min_signatures: 4, // Higher threshold for domain 1
							total_attesters: 5,
						},
					),
					(
						2,
						GenesisThreshold {
							min_signatures: 2, // Lower threshold for domain 2
							total_attesters: 5,
						},
					),
				],
				_phantom: Default::default(),
			};

			config.build();

			// Verify global threshold
			let global = crate::GlobalThreshold::<Runtime>::get().unwrap();
			assert_eq!(global.min_signatures, 3);

			// Verify domain thresholds
			let domain1 = crate::ThresholdConfigs::<Runtime>::get(1).unwrap();
			assert_eq!(domain1.min_signatures, 4);

			let domain2 = crate::ThresholdConfigs::<Runtime>::get(2).unwrap();
			assert_eq!(domain2.min_signatures, 2);
		});
	}

	#[test]
	#[should_panic(expected = "Invalid public key length")]
	fn genesis_config_rejects_invalid_key_length() {
		new_test_ext().execute_with(|| {
			let config = GenesisConfig::<Runtime> {
				attesters: vec![GenesisAttester {
					public_key: vec![0u8; 20], // Invalid length
					name: b"Attester-1".to_vec(),
					enabled: true,
				}],
				threshold: GenesisThreshold::default(),
				domain_thresholds: vec![],
				_phantom: Default::default(),
			};

			config.build();
		});
	}

	#[test]
	#[should_panic(expected = "Duplicate public key")]
	fn genesis_config_rejects_duplicate_keys() {
		new_test_ext().execute_with(|| {
			let same_key = generate_test_key(1);
			let config = GenesisConfig::<Runtime> {
				attesters: vec![
					GenesisAttester {
						public_key: same_key.clone(),
						name: b"Attester-1".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: same_key, // Duplicate!
						name: b"Attester-2".to_vec(),
						enabled: true,
					},
				],
				threshold: GenesisThreshold::default(),
				domain_thresholds: vec![],
				_phantom: Default::default(),
			};

			config.build();
		});
	}

	#[test]
	#[should_panic(expected = "Invalid threshold configuration")]
	fn genesis_config_rejects_invalid_threshold() {
		new_test_ext().execute_with(|| {
			let config = GenesisConfig::<Runtime> {
				attesters: vec![
					GenesisAttester {
						public_key: generate_test_key(1),
						name: b"Attester-1".to_vec(),
						enabled: true,
					},
					GenesisAttester {
						public_key: generate_test_key(2),
						name: b"Attester-2".to_vec(),
						enabled: true,
					},
				],
				threshold: GenesisThreshold {
					min_signatures: 5, // More than total!
					total_attesters: 2,
				},
				domain_thresholds: vec![],
				_phantom: Default::default(),
			};

			config.build();
		});
	}

	#[test]
	#[should_panic(expected = "Insufficient active attesters")]
	fn genesis_config_rejects_insufficient_active() {
		new_test_ext().execute_with(|| {
			let config = GenesisConfig::<Runtime> {
				attesters: vec![
					GenesisAttester {
						public_key: generate_test_key(1),
						name: b"Attester-1".to_vec(),
						enabled: true, // Only 1 active
					},
					GenesisAttester {
						public_key: generate_test_key(2),
						name: b"Attester-2".to_vec(),
						enabled: false,
					},
					GenesisAttester {
						public_key: generate_test_key(3),
						name: b"Attester-3".to_vec(),
						enabled: false,
					},
				],
				threshold: GenesisThreshold {
					min_signatures: 2, // Requires 2 but only 1 active
					total_attesters: 3,
				},
				domain_thresholds: vec![],
				_phantom: Default::default(),
			};

			config.build();
		});
	}
}
