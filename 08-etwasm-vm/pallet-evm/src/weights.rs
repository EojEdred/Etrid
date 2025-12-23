use frame_support::weights::Weight;

pub trait WeightInfo {
    fn call_evm() -> Weight;
    fn register_bridge_target() -> Weight;
    fn register_known_account() -> Weight;
}

pub struct SubstrateWeight;

impl WeightInfo for SubstrateWeight {
    fn call_evm() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn register_bridge_target() -> Weight {
        Weight::from_parts(10_000, 0)
    }

    fn register_known_account() -> Weight {
        Weight::from_parts(10_000, 0)
    }
}
