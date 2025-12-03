//! Chain specification for ADA-PBC collator

use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};

// ADA-PBC Runtime imports
use ada_pbc_runtime::{AccountId, WASM_BINARY};

/// Specialized `ChainSpec` for ADA-PBC
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
    TPublic: From<sp_core::sr25519::Public>,
{
    let public = get_from_seed::<sr25519::Public>(seed);
    public.into()
}

fn production_genesis() -> serde_json::Value {
    let endowed_accounts: Vec<AccountId> = vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
    ];

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1_000_000_000_000_000_000_000u128)).collect::<Vec<_>>(),
        }
    })
}

/// Development config (single collator)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("ADA-PBC Development")
    .with_id("ada_pbc_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .build())
}

/// Local testnet config (two collators)
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("ADA-PBC Local Testnet")
    .with_id("ada_pbc_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("ada-pbc")
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// Production mainnet config
pub fn production_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("ADA-PBC Mainnet")
    .with_id("ada_pbc_mainnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("ada-pbc")
    .with_genesis_config_patch(production_genesis())
    .with_properties({
        let mut props = serde_json::Map::new();
        props.insert("tokenSymbol".into(), "ETRID".into());
        props.insert("tokenDecimals".into(), 18.into());
        props
    })
    .build())
}
