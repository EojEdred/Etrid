//! Chain specification for XRP PBC collator
//!
//! This defines the chain configuration for the XRP Ledger Partition Burst Chain

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// XRP PBC Runtime imports
use xrp_pbc_runtime::{AccountId, RuntimeGenesisConfig, WASM_BINARY};

/// Specialized `ChainSpec` for XRP PBC
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

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
        "XRP PBC Development",
        // ID
        "xrp_pbc_dev",
        ChainType::Development,
        move || {
            // NOTE: Implement testnet_genesis for your specific XRP PBC runtime
            panic!("Genesis config not implemented - update for specific XRP PBC runtime")
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
        "XRP PBC Local Testnet",
        // ID
        "xrp_pbc_local",
        ChainType::Local,
        move || {
            // NOTE: Implement testnet_genesis for your specific XRP PBC runtime
            panic!("Genesis config not implemented - update for specific XRP PBC runtime")
        },
        vec![],
        None,
        Some("xrp-pbc"),
        None,
        None,
        None,
    ))
}

// NOTE: When implementing for XRP PBC, add a testnet_genesis function like this:
/*
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
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
        // XRP-specific bridge pallet config
        xrp_bridge: XrpBridgeConfig {
            bridge_account: root_key.clone(),
            min_confirmations: 1,
            initial_validators: initial_authorities.iter().map(|x| x.0.clone().into()).collect(),
        },
        ..Default::default()
    }
}
*/