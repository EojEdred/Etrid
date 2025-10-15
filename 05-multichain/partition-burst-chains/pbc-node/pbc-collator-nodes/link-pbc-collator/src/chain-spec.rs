//! Chain specification for LINK-PBC collator
//!
//! Chainlink Partition Burst Chain - Oracle services and price feeds

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// LINK-PBC Runtime
use link_pbc_runtime::{AccountId, WASM_BINARY};

/// Specialized `ChainSpec` for LINK-PBC
/// NOTE: Update `GenesisConfig` type when implementing genesis
pub type ChainSpec = sc_service::GenericChainSpec<()>; // TODO: Replace () with actual GenesisConfig

/// Generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> sp_runtime::AccountId32
where
    sp_runtime::MultiSignature: From<<TPublic::Pair as Pair>::Signature>,
{
    sp_runtime::MultiSigner::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate authority keys
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

/// Development config (single collator)
pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::from_genesis(
        // Name
        "LINK-PBC Development",
        // ID
        "link_pbc_dev",
        ChainType::Development,
        move || {
            // NOTE: Implement testnet_genesis for LINK-PBC runtime
            panic!("Genesis config not implemented - update for specific LINK-PBC runtime")
        },
        vec![],
        None,
        None,
        None,
        None,
        None,
    ))
}

/// Local testnet config (two collators)
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::from_genesis(
        // Name
        "LINK-PBC Local Testnet",
        // ID
        "link_pbc_local",
        ChainType::Local,
        move || {
            // NOTE: Implement testnet_genesis for LINK-PBC runtime
            panic!("Genesis config not implemented - update for specific LINK-PBC runtime")
        },
        vec![],
        None,
        Some("link-pbc"),
        None,
        None,
        None,
    ))
}

// TODO: Implement testnet_genesis for LINK-PBC
// Should include:
// - System config with WASM binary
// - Balances for endowed accounts
// - Aura authorities for block production
// - Grandpa authorities for finalization
// - ChainlinkBridge config with oracle nodes and data feeds
/*
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1_000_000_000_000_000_000_000u128))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
            ..Default::default()
        },
        chainlink_bridge: ChainlinkBridgeConfig {
            // TODO: Configure oracle nodes, data feeds, VRF
            ..Default::default()
        },
        ..Default::default()
    }
}
*/