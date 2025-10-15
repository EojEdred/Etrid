//! Chain specification for XLM-PBC (Stellar) collator

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// XLM-PBC Runtime imports
use xlm_pbc_runtime::{AccountId, AuraConfig, BalancesConfig, RuntimeGenesisConfig, SystemConfig};

/// Specialized `ChainSpec` for XLM-PBC
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
        "XLM-PBC Development",
        // ID
        "xlm_pbc_dev",
        ChainType::Development,
        move || testnet_genesis(),
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
        "XLM-PBC Local Testnet",
        // ID
        "xlm_pbc_local",
        ChainType::Local,
        move || testnet_genesis(),
        vec![],
        None,
        Some("xlm-pbc"),
        None,
        None,
        None,
    ))
}

/// Genesis configuration for XLM-PBC testnet
fn testnet_genesis() -> RuntimeGenesisConfig {
    // Get development accounts
    let endowed_accounts: Vec<AccountId> = vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    ];

    let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![
        authority_keys_from_seed("Alice"),
    ];

    RuntimeGenesisConfig {
        system: SystemConfig {
            code: xlm_pbc_runtime::WASM_BINARY
                .expect("WASM binary not available")
                .to_vec(),
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
            authorities: initial_authorities.iter().map(|x| x.0.clone()).collect(),
        },
        grandpa: xlm_pbc_runtime::GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
            ..Default::default()
        },
        ..Default::default()
    }
}