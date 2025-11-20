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

/// FlareChain mainnet config (using runtime preset with 21 validators)
pub fn flarechain_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Mainnet")
    .with_id("flarechain_mainnet")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("flarechain_mainnet")
    .build())
}

/// 2-validator test config (Alice & Bob) for debugging
pub fn test_2validator_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain 2-Validator Test")
    .with_id("flarechain_test_2val")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("flarechain_test")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("test_2validator")
    .build())
}

/// 21-validator test config (9 DecentralizedDirector + 12 ValidityNode) for production testing
pub fn test_21validator_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain 21-Validator Test")
    .with_id("flarechain_test_21val")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("flarechain_test_21")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 18.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("test_21val")
    .build())
}

/// 9-Director mainnet config (production restart with GRANDPA fix)
pub fn directors_9_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Mainnet (9 Directors)")
    .with_id("flarechain_mainnet_9directors")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("flarechain_mainnet_restart_final")
    .build())
}

/// 11-validator test config for Pure ASF quorum testing
pub fn test_11validator_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain 11-Validator Test")
    .with_id("flarechain_test_11val")
    .with_chain_type(ChainType::Local)
    .with_protocol_id("flarechain_test_11")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 18.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("test_11val")
    .build())
}

/// Session-fixed mainnet config (GRANDPA finality fix with session pallet)
pub fn session_fixed_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Mainnet (Session Fixed)")
    .with_id("flarechain_mainnet_session_fixed")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("flarechain_mainnet_session_fixed")
    .build())
}

/// ASF mainnet config (21 validators with stake-weighted GRANDPA)
pub fn asf_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Mainnet (ASF)")
    .with_id("flarechain_mainnet_asf")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties
    })
    .with_genesis_config_preset_name("flarechain_mainnet_asf")
    .build())
}

/// Hybrid mainnet config (Phase 2: GRANDPA + ASF dual finality) - DEPRECATED in v108
pub fn hybrid_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Mainnet (Hybrid)")
    .with_id("flarechain_mainnet_hybrid")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ETR".into());
        properties.insert("tokenDecimals".into(), 12.into());
        properties.insert("ss58Format".into(), 42.into());
        properties.insert("consensusMode".into(), "hybrid".into());
        properties.insert("blockProduction".into(), "PPFA".into());
        properties.insert("finality".into(), "Pure ASF".into());
        properties
    })
    .with_genesis_config_preset_name("mainnet_hybrid")
    .build())
}

/// Pure ASF mainnet config (v1 - Pure ASF, no GRANDPA)
pub fn pure_asf_mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Mainnet (Pure ASF)")
    .with_id("flarechain_mainnet_v1")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
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
    .with_genesis_config_preset_name("mainnet_v108_pure_asf")
    .build())
}

/// Development config for Pure ASF (single validator - Alice, NO GRANDPA)
pub fn development_asf_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain Development (Pure ASF)")
    .with_id("flarechain_dev_asf")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_preset_name("dev_pure_asf")
    .build())
}