// ETH-PBC Precompiles
// Combines standard Ethereum precompiles with custom Ëtrid precompiles

use core::marker::PhantomData;
use pallet_evm::{
	IsPrecompileResult, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use sp_core::H160;

// Standard Ethereum precompiles
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, Identity, Ripemd160, Sha256};

// Custom Ëtrid precompiles
mod xcm_bridge;
mod oracle;
mod governance;
mod staking;
mod lightning;
mod native_eth_wrap;
mod state_proof;
mod token_registry;

pub use xcm_bridge::{MockXcmBridge, XcmBridge};
#[allow(unused_imports)]
pub use xcm_bridge::{FlareChainQuery, FlareChainResponse};
pub use oracle::EtridOraclePrecompile;
pub use governance::EtridGovernancePrecompile;
pub use staking::EtridStakingPrecompile;
pub use lightning::EtridLightningPrecompile;
pub use native_eth_wrap::NativeETHWrapPrecompile;
pub use state_proof::StateProofPrecompile;
pub use token_registry::TokenRegistryPrecompile;

/// Combined precompile set for ETH-PBC
/// Includes both standard Ethereum precompiles and custom Ëtrid precompiles
pub struct EtridPrecompiles<R, XCM = MockXcmBridge>(PhantomData<(R, XCM)>);

impl<R, XCM> EtridPrecompiles<R, XCM>
where
	R: pallet_evm::Config,
	XCM: XcmBridge,
{
	pub fn new() -> Self {
		Self(Default::default())
	}

	/// All precompile addresses (standard + custom)
	pub fn used_addresses() -> [H160; 13] {
		[
			// Standard Ethereum precompiles
			hash(1),    // ECRecover
			hash(2),    // SHA256
			hash(3),    // RIPEMD160
			hash(4),    // Identity
			hash(5),    // Modexp
			hash(8),    // SHA3FIPS256
			// Custom Ëtrid precompiles
			hash(0x800), // Oracle
			hash(0x801), // Governance
			hash(0x802), // Staking
			hash(0x803), // Native ETH Wrapping
			hash(0x804), // State Proof Verification
			hash(0x805), // Token Registry
			hash(0x808), // Lightning Channels
		]
	}
}

impl<R, XCM> PrecompileSet for EtridPrecompiles<R, XCM>
where
	R: pallet_evm::Config,
	XCM: XcmBridge,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Standard Ethereum precompiles (0x01 - 0x08)
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			a if a == hash(8) => Some(Sha3FIPS256::execute(handle)),

			// Custom Ëtrid precompiles (0x800+)
			a if a == hash(0x800) => Some(EtridOraclePrecompile::<XCM>::execute(handle)),
			a if a == hash(0x801) => Some(EtridGovernancePrecompile::<XCM>::execute(handle)),
			a if a == hash(0x802) => Some(EtridStakingPrecompile::<XCM>::execute(handle)),
			a if a == hash(0x803) => Some(NativeETHWrapPrecompile::<R>::execute(handle)),
			a if a == hash(0x804) => Some(StateProofPrecompile::<R>::execute(handle)),
			a if a == hash(0x805) => Some(TokenRegistryPrecompile::<R>::execute(handle)),
			a if a == hash(0x808) => Some(EtridLightningPrecompile::<R>::execute(handle)),

			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().contains(&address),
			extra_cost: 0,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

// Re-export for backward compatibility
pub type FrontierPrecompiles<R> = EtridPrecompiles<R, MockXcmBridge>;
