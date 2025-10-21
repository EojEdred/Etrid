//! Chain specification for SC-USDT-PBC collator

use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// SC-USDT-PBC Runtime
use sc_usdt_pbc_runtime::{AccountId, RuntimeGenesisConfig, WASM_BINARY};

/// Specialized `ChainSpec` for SC-USDT-PBC
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    TPublic: From<sr25519::Public>,
{
    get_from_seed::<sr25519::Public>(seed).into()
}

/// Development config (single collator)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Default::default(),
    )
    .with_name("SC-USDT-PBC Development")
    .with_id("sc_usdt_pbc_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis())
    .build())
}

/// Local testnet config (two collators)
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Default::default(),
    )
    .with_name("SC-USDT-PBC Local Testnet")
    .with_id("sc_usdt_pbc_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("sc-usdt-pbc")
    .with_genesis_config_patch(testnet_genesis())
    .build())
}

/// Generate genesis configuration for SC-USDT-PBC
fn testnet_genesis() -> serde_json::Value {
    // Development accounts
    let endowed_accounts: Vec<AccountId> = vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    ];

    let balances: Vec<(AccountId, u128)> = endowed_accounts
        .iter()
        .cloned()
        .map(|k| (k, 1_000_000_000_000_000_000_000u128))
        .collect();

    serde_json::json!({
        "balances": {
            "balances": balances,
        }
    })
}