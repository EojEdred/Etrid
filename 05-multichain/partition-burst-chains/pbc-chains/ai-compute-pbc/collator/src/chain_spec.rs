//! Chain specification for AI-Compute-PBC

use ai_compute_pbc_runtime::{
    AccountId, AuraConfig, BalancesConfig, GrandpaConfig, RuntimeGenesisConfig,
    Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for AI-Compute-PBC
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

/// Development chain configuration
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "AI Compute PBC Development",
        // ID
        "ai_compute_pbc_dev",
        // Chain type
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial authorities (Aura, GRANDPA)
                vec![authority_keys_from_seed("Alice")],
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
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("ai-compute-pbc-dev"),
        // Fork ID
        None,
        // Properties
        Some(chain_properties()),
        // Extensions
        None,
    ))
}

/// Local testnet configuration
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "AI Compute PBC Local Testnet",
        // ID
        "ai_compute_pbc_local",
        // Chain type
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("ai-compute-pbc-local"),
        // Fork ID
        None,
        // Properties
        Some(chain_properties()),
        // Extensions
        None,
    ))
}

/// Mainnet configuration
pub fn mainnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/mainnet.json")[..])
}

/// Chain properties (for UI)
fn chain_properties() -> serde_json::Value {
    serde_json::json!({
        "tokenSymbol": "ËDSC",
        "tokenDecimals": 18,
        "ss58Format": 42,
    })
}

/// Configure initial genesis state
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
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
                .map(|k| (k, 1_000_000 * 10u128.pow(18))) // 1M ËDSC per account
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
        sudo: SudoConfig {
            key: Some(root_key),
        },
        transaction_payment: Default::default(),
        // AI Compute pallets start with empty state
        // GPUs will be registered via extrinsics after launch
    }
}
