//! Chain specification for BNB-PBC collator

use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use bnb_pbc_runtime::{AccountId, Balance};

/// Specialized `ChainSpec` for BNB-PBC
pub type ChainSpec = sc_service::GenericChainSpec<bnb_pbc_runtime::RuntimeGenesisConfig>;

/// Generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    sp_runtime::MultiSignature: From<<TPublic::Pair as Pair>::Signature>,
{
    sp_runtime::MultiSigner::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Development config (single collator)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = bnb_pbc_runtime::WASM_BINARY
        .ok_or_else(|| "BNB-PBC wasm binary not available".to_string())?;

    Ok(ChainSpec::builder(wasm_binary, None)
        .with_name("BNB-PBC Development")
        .with_id("bnb_pbc_dev")
        .with_chain_type(ChainType::Development)
        .with_genesis_config_patch(testnet_genesis(
            vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
            true,
        ))
        .build())
}

/// Local testnet config (two collators)
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = bnb_pbc_runtime::WASM_BINARY
        .ok_or_else(|| "BNB-PBC wasm binary not available".to_string())?;

    Ok(ChainSpec::builder(wasm_binary, None)
        .with_name("BNB-PBC Local Testnet")
        .with_id("bnb_pbc_local")
        .with_chain_type(ChainType::Local)
        .with_genesis_config_patch(testnet_genesis(
            vec![
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
            ],
            true,
        ))
        .with_protocol_id("bnb-pbc")
        .build())
}

fn testnet_genesis(
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1_000_000_000_000_000_000u128)).collect::<Vec<_>>(),
        },
    })
}