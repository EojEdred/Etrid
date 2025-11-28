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

/// Production mainnet config with 20 validators
pub fn production_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Default::default(),
    )
    .with_name("SC-USDT-PBC Mainnet")
    .with_id("sc_usdt_pbc_mainnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("sc-usdt-pbc")
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