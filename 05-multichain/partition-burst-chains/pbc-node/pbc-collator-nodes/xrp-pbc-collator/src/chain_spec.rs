//! Chain specification for XRP-PBC collator

use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// XRP-PBC Runtime imports
use xrp_pbc_runtime::{AccountId, WASM_BINARY};

/// Specialized `ChainSpec` for XRP-PBC
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

/// Development config (single collator)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("XRP-PBC Development")
    .with_id("xrp_pbc_dev")
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
    .with_name("XRP-PBC Local Testnet")
    .with_id("xrp_pbc_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("xrp-pbc")
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// Production mainnet config with 20 validators
pub fn production_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("XRP-PBC Mainnet")
    .with_id("xrp_pbc_mainnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("xrp-pbc")
    .with_genesis_config_patch(production_genesis())
    .with_properties({
        let mut props = serde_json::Map::new();
        props.insert("tokenSymbol".into(), "ETRID".into());
        props.insert("tokenDecimals".into(), 18.into());
        props
    })
    .build())
}

/// Generate production genesis configuration with 20 validators
fn production_genesis() -> serde_json::Value {
    // Generate all 20 validator accounts from //Validator{1-20} seeds
    let validators: Vec<AccountId> = (1..=20)
        .map(|i| get_account_id_from_seed::<sr25519::Public>(&format!("Validator{}", i)))
        .collect();

    // Each validator gets 1,000,000 ETRID (with 18 decimals)
    let balances: Vec<(AccountId, u128)> = validators
        .iter()
        .cloned()
        .map(|k| (k, 1_000_000_000_000_000_000_000_000u128))
        .collect();

    // Validator config: (account_id, stake, role)
    // 64,000 ETRID stake per validator (18 decimals)
    let validator_config: Vec<(AccountId, u128, &str)> = validators
        .iter()
        .cloned()
        .map(|k| (k, 64_000_000_000_000_000_000_000u128, "ValidityNode"))
        .collect();

    serde_json::json!({
        "balances": {
            "balances": balances,
        },
        "sudo": {
            "key": validators[0],
        },
        "consensus": {
            "validators": validator_config,
            "slotDuration": 6000,
        }
    })
}
