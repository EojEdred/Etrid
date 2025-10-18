use flare_chain_runtime::WASM_BINARY;
use sc_service::ChainType;

/// Specialized `ChainSpec` for FlareChain
pub type ChainSpec = sc_service::GenericChainSpec;

/// Development config (single validator - Alice)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Development")
    .with_id("flarechain_dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .build())
}

/// Local testnet config (two validators - Alice & Bob)
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Local Testnet")
    .with_id("flarechain_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("flarechain")
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// FlareChain mainnet config
pub fn flarechain_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/flarechain.json")[..])
}