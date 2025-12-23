use alloc::collections::BTreeMap;

use crate::abi_translator::FunctionSelector;

/// Represents a registered precompile or function bridge target.
#[derive(Clone, Debug)]
pub struct PrecompileTarget {
    pub name: &'static str,
    pub selector: FunctionSelector,
    pub wasm_entrypoint: &'static str,
}

/// Lightweight registry that helps translating EVM function selectors into
/// Wasm entrypoints. During execution we consult this registry before falling
/// back to raw SCALE dispatch.
#[derive(Default)]
pub struct PrecompileRegistry {
    entries: BTreeMap<[u8; 4], PrecompileTarget>,
}

impl PrecompileRegistry {
    pub fn new() -> Self {
        let mut registry = Self { entries: BTreeMap::new() };
        // Pre-register a few critical targets so developers can test the flow
        // before wiring production contracts.
        registry.register(PrecompileTarget {
            name: "PrimeSwap::quote",
            selector: FunctionSelector([0x12, 0x34, 0x56, 0x78]),
            wasm_entrypoint: "primeswap::quote",
        });
        registry.register(PrecompileTarget {
            name: "PrimeSwap::swap",
            selector: FunctionSelector([0xaa, 0xbb, 0xcc, 0xdd]),
            wasm_entrypoint: "primeswap::swap",
        });
        registry
    }

    pub fn register(&mut self, target: PrecompileTarget) {
        self.entries.insert(target.selector.0, target);
    }

    pub fn resolve(&self, selector: &FunctionSelector) -> Option<&PrecompileTarget> {
        self.entries.get(&selector.0)
    }
}
