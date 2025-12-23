#![allow(dead_code)]
use primearc_core_runtime::WASM_BINARY;
use sc_service::ChainType;

/// Specialized `ChainSpec` for Primearc Core
pub type ChainSpec = sc_service::GenericChainSpec;

// =============================================================================
// AVAILABLE PRESETS:
//   - "development" (sp_genesis_builder::DEV_RUNTIME_PRESET)
//   - "local_testnet" (sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
//   - "ember_testnet"
//   - "primearc_core_mainnet_v1_pure_asf" (PRODUCTION)
// =============================================================================

/// Development config (single validator - Alice)
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Development")
    .with_id("primearc_core_dev")
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
    .with_name("Ëtrid Primearc Core Local Testnet")
    .with_id("primearc_core_local")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("primearc")
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// Ember staging testnet config (public testnet)
pub fn staging_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Staging wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Ember Testnet")
    .with_id("ember_testnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("ember")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("ember_testnet")
    .build())
}

/// Primearc Core mainnet config (PRODUCTION - Pure ASF)
pub fn primearc_core_chain_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Mainnet")
    .with_id("primearc_core_mainnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("primearc")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties.insert("consensusMode".into(), "pure_asf".into());
        properties.insert("blockProduction".into(), "PPFA".into());
        properties.insert("finality".into(), "ASF".into());
        properties
    })
    .with_genesis_config_preset_name("primearc_core_mainnet_v1_pure_asf")
    .build())
}

/// 2-validator test config (uses local_testnet preset)
pub fn test_2validator_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core 2-Validator Test")
    .with_id("primearc_core_test_2val")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("primearc_test")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// 21-validator test config (uses local_testnet preset)
pub fn test_21validator_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core 21-Validator Test")
    .with_id("primearc_core_test_21val")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("primearc_test_21")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 18.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// 9-Director mainnet config (uses production preset)
pub fn directors_9_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Mainnet (9 Directors)")
    .with_id("primearc_core_mainnet_9directors")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("primearc")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("primearc_core_mainnet_v1_pure_asf")
    .build())
}

/// 11-validator test config (uses local_testnet preset)
pub fn test_11validator_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core 11-Validator Test")
    .with_id("primearc_core_test_11val")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("primearc_test_11")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 18.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
    .build())
}

/// Session-fixed mainnet config (uses production preset)
pub fn session_fixed_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Mainnet (Session Fixed)")
    .with_id("primearc_core_mainnet_session_fixed")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("primearc")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("primearc_core_mainnet_v1_pure_asf")
    .build())
}

/// ASF mainnet config (uses production preset)
pub fn asf_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Mainnet (ASF)")
    .with_id("primearc_core_mainnet_asf")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("primearc")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties.insert("consensusMode".into(), "pure_asf".into());
        properties
    })
    .with_genesis_config_preset_name("primearc_core_mainnet_v1_pure_asf")
    .build())
}

/// Hybrid mainnet config (uses production preset - hybrid mode deprecated)
pub fn hybrid_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Mainnet (Hybrid)")
    .with_id("primearc_core_mainnet_hybrid")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("primearc")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties.insert("consensusMode".into(), "pure_asf".into());
        properties.insert("blockProduction".into(), "PPFA".into());
        properties.insert("finality".into(), "Pure ASF".into());
        properties
    })
    .with_genesis_config_preset_name("primearc_core_mainnet_v1_pure_asf")
    .build())
}

/// Pure ASF mainnet config (PRODUCTION - v1 Pure ASF)
pub fn pure_asf_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Mainnet (Pure ASF)")
    .with_id("primearc_core_mainnet_v1")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("primearc")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties.insert("runtimeVersion".into(), 108.into());
        properties.insert("consensusMode".into(), "pure_asf".into());
        properties.insert("blockProduction".into(), "PPFA".into());
        properties.insert("finality".into(), "ASF".into());
        properties
    })
    .with_genesis_config_preset_name("primearc_core_mainnet_v1_pure_asf")
    .build())
}

/// Development config for Pure ASF (uses development preset)
pub fn development_asf_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid Primearc Core Development (Pure ASF)")
    .with_id("primearc_core_dev_asf")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
    .build())
}
