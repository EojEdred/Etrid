//! Chain specification for ADA-PBC collator
//!
//! Configures the Cardano Partition Burst Chain genesis state

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// ADA-PBC Runtime
use ada_pbc_runtime::{AccountId, RuntimeGenesisConfig, WASM_BINARY};

/// Specialized `ChainSpec` for ADA-PBC
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

    Ok(ChainSpec::from_genesis(
        "ADA-PBC Development",
        "ada_pbc_dev",
        ChainType::Development,
        move || testnet_genesis(wasm_binary),
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
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "ADA-PBC Local Testnet",
        "ada_pbc_local",
        ChainType::Local,
        move || testnet_genesis(wasm_binary),
        vec![],
        None,
        Some("ada-pbc"),
        None,
        None,
        None,
    ))
}

/// Generate genesis configuration for testnet
fn testnet_genesis(wasm_binary: &'static [u8]) -> RuntimeGenesisConfig {
    use ada_pbc_runtime::{BalancesConfig, SystemConfig};

    // Endowed accounts for testing
    let endowed_accounts: Vec<AccountId> = vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
    ];

    RuntimeGenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1_000_000_000_000_000_000_000u128))
                .collect(),
        },
        // Add ADA-PBC specific configuration here as needed
        // cardano_bridge: CardanoBridgeConfig { ... },
        // accounts: AccountsConfig { ... },
        // consensus: ConsensusConfig { ... },
    }
}