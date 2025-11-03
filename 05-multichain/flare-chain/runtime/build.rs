fn main() {
    #[cfg(feature = "std")]
    {
        substrate_wasm_builder::WasmBuilder::new()
            .with_current_project()
            .export_heap_base()
            .import_memory()
            .build();
    }

    // Force rebuild when preset files change
    println!("cargo:rerun-if-changed=presets/flarechain_mainnet.json");
    println!("cargo:rerun-if-changed=presets/development.json");
    println!("cargo:rerun-if-changed=presets/local_testnet.json");
    println!("cargo:rerun-if-changed=presets/ember_testnet.json");
    println!("cargo:rerun-if-changed=presets/test_2validator.json");
}
