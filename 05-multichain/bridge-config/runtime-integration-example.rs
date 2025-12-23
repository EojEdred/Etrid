//! Example Runtime Integration for Bridge Attestation Pallet
//!
//! This file demonstrates how to integrate the bridge attestation pallet
//! into a PBC (Partition Burst Chain) runtime with genesis configuration.

use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_runtime::{traits::IdentityLookup, BuildStorage};

// Import the bridge attestation pallet
use pallet_bridge_attestation::{self, GenesisAttester, GenesisConfig, GenesisThreshold};

/// Example Runtime Configuration
impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    /// Chain ID for this PBC
    /// Ethereum = 1, BSC = 56, Polygon = 137, etc.
    type ChainId = ConstU32<1001>; // Custom chain ID for this PBC

    /// Maximum number of attesters that can be registered
    type MaxAttesters = ConstU32<100>;

    /// Maximum attesters per message signature collection
    type MaxAttestersPerMessage = ConstU32<20>;

    /// Minimum signatures required (default M in M-of-N)
    /// Can be overridden by governance
    type MinSignatureThreshold = ConstU32<3>;

    /// Maximum age of an attestation (in blocks)
    /// After this many blocks, attestations expire
    /// 1000 blocks ≈ 2 hours at 6s block time
    type AttestationMaxAge = ConstU32<1000>;

    /// Weight information for extrinsics
    type WeightInfo = ();
}

/// Genesis Configuration Example
///
/// This shows how to configure attesters at genesis instead of
/// registering them via governance after launch.
pub fn example_genesis_config() -> GenesisConfig<Runtime> {
    // IMPORTANT: These are EXAMPLE keys only!
    // Generate real keys using the generate-attester-keys.sh script

    GenesisConfig {
        attesters: vec![
            GenesisAttester {
                public_key: hex::decode(
                    "02a1b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef012345678"
                )
                .unwrap(),
                name: b"Attester-Alpha".to_vec(),
                enabled: true,
            },
            GenesisAttester {
                public_key: hex::decode(
                    "03b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789a"
                )
                .unwrap(),
                name: b"Attester-Beta".to_vec(),
                enabled: true,
            },
            GenesisAttester {
                public_key: hex::decode(
                    "02c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789ab1"
                )
                .unwrap(),
                name: b"Attester-Gamma".to_vec(),
                enabled: true,
            },
            GenesisAttester {
                public_key: hex::decode(
                    "03d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789ab12c"
                )
                .unwrap(),
                name: b"Attester-Delta".to_vec(),
                enabled: true,
            },
            GenesisAttester {
                public_key: hex::decode(
                    "02e5f6789abcdef0123456789abcdef0123456789abcdef0123456789ab12cd3"
                )
                .unwrap(),
                name: b"Attester-Epsilon".to_vec(),
                enabled: true,
            },
        ],
        // Configure 3-of-5 threshold
        threshold: GenesisThreshold {
            min_signatures: 3,
            total_attesters: 5,
        },
        // Domain-specific thresholds (optional)
        domain_thresholds: vec![
            // Higher security for Ethereum mainnet (domain 1)
            (
                1,
                GenesisThreshold {
                    min_signatures: 4, // 4-of-5 for Ethereum
                    total_attesters: 5,
                },
            ),
            // Standard security for testnets (domain 2)
            (
                2,
                GenesisThreshold {
                    min_signatures: 2, // 2-of-5 for testnet
                    total_attesters: 5,
                },
            ),
        ],
        _phantom: Default::default(),
    }
}

/// Chain Spec Genesis Configuration
///
/// This is how you would integrate it into your chain spec
pub fn testnet_genesis() -> serde_json::Value {
    use serde_json::json;

    json!({
        "bridgeAttestation": {
            "attesters": [
                {
                    "publicKey": "0x02a1b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef012345678",
                    "name": "0x4174746573746572414c706861", // "AttesterAlpha" in hex
                    "enabled": true
                },
                {
                    "publicKey": "0x03b2c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789a",
                    "name": "0x417474657374657242657461", // "AttesterBeta" in hex
                    "enabled": true
                },
                {
                    "publicKey": "0x02c3d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789ab1",
                    "name": "0x417474657374657247616d6d61", // "AttesterGamma" in hex
                    "enabled": true
                },
                {
                    "publicKey": "0x03d4e5f6789abcdef0123456789abcdef0123456789abcdef0123456789ab12c",
                    "name": "0x4174746573746572446574616c6b61", // "AttesterDelta" in hex
                    "enabled": true
                },
                {
                    "publicKey": "0x02e5f6789abcdef0123456789abcdef0123456789abcdef0123456789ab12cd3",
                    "name": "0x4174746573746572457073696c6f6e", // "AttesterEpsilon" in hex
                    "enabled": true
                }
            ],
            "threshold": {
                "minSignatures": 3,
                "totalAttesters": 5
            },
            "domainThresholds": [
                [1, { "minSignatures": 4, "totalAttesters": 5 }],
                [2, { "minSignatures": 2, "totalAttesters": 5 }]
            ]
        }
    })
}

/// Load Genesis from JSON File
///
/// Load attester configuration from the generated genesis JSON
#[cfg(feature = "std")]
pub fn load_genesis_from_file(path: &str) -> Result<GenesisConfig<Runtime>, Box<dyn std::error::Error>> {
    use std::fs;

    #[derive(serde::Deserialize)]
    struct GenesisFile {
        attesters: Vec<AttesterDef>,
        threshold_config: ThresholdDef,
    }

    #[derive(serde::Deserialize)]
    struct AttesterDef {
        id: u32,
        name: String,
        public_key: String,
        enabled: bool,
    }

    #[derive(serde::Deserialize)]
    struct ThresholdDef {
        min_signatures: u32,
        total_attesters: u32,
    }

    let content = fs::read_to_string(path)?;
    let genesis_file: GenesisFile = serde_json::from_str(&content)?;

    let attesters: Vec<GenesisAttester> = genesis_file
        .attesters
        .into_iter()
        .map(|a| {
            let pubkey = if a.public_key.starts_with("0x") {
                hex::decode(&a.public_key[2..])
            } else {
                hex::decode(&a.public_key)
            }
            .expect("Invalid hex public key");

            GenesisAttester {
                public_key: pubkey,
                name: a.name.as_bytes().to_vec(),
                enabled: a.enabled,
            }
        })
        .collect();

    Ok(GenesisConfig {
        attesters,
        threshold: GenesisThreshold {
            min_signatures: genesis_file.threshold_config.min_signatures,
            total_attesters: genesis_file.threshold_config.total_attesters,
        },
        domain_thresholds: vec![],
        _phantom: Default::default(),
    })
}

/// Example: Query Attester Info
#[cfg(feature = "std")]
pub async fn query_attester_info(attester_id: u32) {
    use subxt::{OnlineClient, PolkadotConfig};

    // Connect to node
    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    // Query attester
    // Note: Actual implementation depends on your RPC setup
    println!("Querying attester {}", attester_id);

    // Example query (pseudocode):
    // let attester = api
    //     .storage()
    //     .fetch(&pallet_bridge_attestation::Attesters(attester_id))
    //     .await?;
    //
    // println!("Attester Info: {:?}", attester);
}

/// Example: Register Attester via Governance
#[cfg(feature = "std")]
pub async fn register_attester_via_governance(public_key: Vec<u8>) {
    // Pseudocode for governance proposal
    println!("Creating governance proposal to register attester");
    println!("Public Key: 0x{}", hex::encode(&public_key));

    // In practice, this would:
    // 1. Create a democracy/council proposal
    // 2. Call pallet_bridge_attestation::register_attester
    // 3. Wait for voting period
    // 4. Execute if passed
}

/// Example: Monitor Attestation Events
#[cfg(feature = "std")]
pub async fn monitor_attestation_events() {
    use subxt::{OnlineClient, PolkadotConfig};

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    println!("Monitoring bridge attestation events...");

    // Subscribe to new blocks and watch for events
    // let mut blocks = api.blocks().subscribe_finalized().await.unwrap();
    //
    // while let Some(block) = blocks.next().await {
    //     let block = block.unwrap();
    //     let events = block.events().await.unwrap();
    //
    //     for event in events.iter() {
    //         if let Ok(Some(attestation_event)) = event.as_event::<pallet_bridge_attestation::Event>() {
    //             println!("Attestation Event: {:?}", attestation_event);
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_config_valid() {
        let genesis = example_genesis_config();
        assert_eq!(genesis.attesters.len(), 5);
        assert_eq!(genesis.threshold.min_signatures, 3);
        assert_eq!(genesis.threshold.total_attesters, 5);
    }

    #[test]
    fn test_domain_thresholds() {
        let genesis = example_genesis_config();
        assert_eq!(genesis.domain_thresholds.len(), 2);

        // Check Ethereum domain (higher security)
        let eth_threshold = &genesis.domain_thresholds[0];
        assert_eq!(eth_threshold.0, 1);
        assert_eq!(eth_threshold.1.min_signatures, 4);

        // Check testnet domain (lower security)
        let test_threshold = &genesis.domain_thresholds[1];
        assert_eq!(test_threshold.0, 2);
        assert_eq!(test_threshold.1.min_signatures, 2);
    }

    #[test]
    fn test_all_attesters_enabled() {
        let genesis = example_genesis_config();
        for attester in genesis.attesters.iter() {
            assert!(attester.enabled);
        }
    }

    #[test]
    fn test_public_key_lengths() {
        let genesis = example_genesis_config();
        for attester in genesis.attesters.iter() {
            // ECDSA compressed keys should be 33 bytes
            assert_eq!(attester.public_key.len(), 33);
        }
    }

    #[test]
    fn test_unique_public_keys() {
        let genesis = example_genesis_config();
        let mut keys = std::collections::HashSet::new();

        for attester in genesis.attesters.iter() {
            assert!(
                keys.insert(attester.public_key.clone()),
                "Duplicate public key found"
            );
        }
    }
}
