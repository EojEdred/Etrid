//! Chain specification for DOGE-PBC collator

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// DOGE-PBC Runtime imports
use doge_pbc_runtime::{AccountId, RuntimeGenesisConfig, WASM_BINARY};

/// Specialized `ChainSpec` for DOGE-PBC
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
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("DOGE-PBC Development")
    .with_id("doge_pbc_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
        ],
    ))
    .build())
}

/// Local testnet config (two collators)
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("DOGE-PBC Local Testnet")
    .with_id("doge_pbc_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("doge_pbc")
    .with_genesis_config_patch(testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
        ],
    ))
    .build())
}

/// Generate the genesis configuration for DOGE-PBC testnet
fn testnet_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    _root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> serde_json::Value {
    use serde_json::json;

    json!({
        "balances": {
            "balances": endowed_accounts
                .iter()
                .map(|k| (k.clone(), 1_000_000_000_000_000_000_000u128))
                .collect::<Vec<_>>(),
        },
    })
}