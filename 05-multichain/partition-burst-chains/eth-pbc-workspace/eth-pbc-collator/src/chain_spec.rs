//! Chain specification for ETH-PBC collator

use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public, H160};
use sp_runtime::traits::{IdentifyAccount, Verify};

// ETH-PBC Runtime imports
use eth_pbc_runtime::{AccountId, WASM_BINARY};

/// Specialized `ChainSpec` for ETH-PBC
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Generate an Ethereum-compatible account ID from seed
///
/// Converts Sr25519 public key to Ethereum address by:
/// 1. Hashing the public key with Keccak-256
/// 2. Taking the last 20 bytes (160 bits) as the Ethereum address
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    TPublic: From<sp_core::sr25519::Public>,
{
    let public = get_from_seed::<sr25519::Public>(seed);
    let public_bytes = public.as_ref();

    // Hash the Sr25519 public key using Keccak-256
    let hash = sp_core::keccak_256(public_bytes);

    // Take the last 20 bytes to create an Ethereum address
    let mut address_bytes = [0u8; 20];
    address_bytes.copy_from_slice(&hash[12..32]);

    H160::from(address_bytes).into()
}

/// Development config (single collator)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("ETH-PBC Development")
    .with_id("eth_pbc_dev")
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
    .with_name("ETH-PBC Local Testnet")
    .with_id("eth_pbc_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("eth-pbc")
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}
