//! Chain specifications for DOGE-PBC

use doge_pbc_runtime::{
    AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig, SystemConfig,
    WASM_BINARY, ConsensusConfig,
};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <sp_runtime::MultiSignature as Verify>::Signer;

/// Generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Development chain configuration
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Default::default(),
    )
    .with_name("DOGE-PBC Development")
    .with_id("doge_pbc_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis(
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        ],
        // Initial validators
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
        ],
        true,
    ))
    .build())
}

/// Local testnet configuration
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Default::default(),
    )
    .with_name("DOGE-PBC Local Testnet")
    .with_id("doge_pbc_local")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(testnet_genesis(
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        // Initial validators
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
        ],
        true,
    ))
    .build())
}

/// Main DOGE-PBC configuration
pub fn doge_pbc_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        Default::default(),
    )
    .with_name("DOGE-PBC Mainnet")
    .with_id("doge_pbc")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(testnet_genesis(
        // Sudo account (should be removed in production)
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
        ],
        // Initial validators
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
        ],
        false,
    ))
    .build())
}

/// Generate genesis configuration
fn testnet_genesis(
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    initial_validators: Vec<AccountId>,
    _enable_println: bool,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u128 << 60)).collect::<Vec<_>>(),
        },
        "sudo": {
            "key": Some(root_key),
        },
        "consensus": {
            "validators": initial_validators,
        }
    })
}

/// Load chain specification from ID
pub fn load_spec(id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(development_config()?),
        "local" => Box::new(local_testnet_config()?),
        "" | "doge-pbc" => Box::new(doge_pbc_config()?),
        path => Box::new(ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),
    })
}
