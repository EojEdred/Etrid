#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use core::marker::PhantomData;
use evm_adapter::TranslatedCall;
use frame_support::{dispatch::{DispatchResult}, pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32, H160};
use sp_runtime::DispatchError;
use etwasm_gas_metering::VMw;
use pallet_etwasm_vm as etwasm;

pub mod account_mapping;
pub mod evm_runtime;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use account_mapping::account_to_h160;
use evm_runtime::EvmRuntime;
use weights::WeightInfo;

pub trait WasmCallBridge<AccountId> {
    fn dispatch(caller: &AccountId, call: &TranslatedCall) -> Result<VMw, DispatchError>;
}

impl<AccountId> WasmCallBridge<AccountId> for () {
    fn dispatch(_caller: &AccountId, _call: &TranslatedCall) -> Result<VMw, DispatchError> {
        Ok(0)
    }
}

#[derive(Clone, Copy, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, RuntimeDebug)]
pub enum FailureReason {
    MalformedCalldata,
    ExecutionFailed,
    OutOfGas,
}

#[derive(
	Clone, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, RuntimeDebug,
	serde::Serialize, serde::Deserialize
)]
pub struct BridgeTarget<AccountId> {
    pub contract: AccountId,
    pub gas_limit: VMw,
}

impl From<FailureReason> for u8 {
    fn from(reason: FailureReason) -> Self {
        match reason {
            FailureReason::MalformedCalldata => 0,
            FailureReason::ExecutionFailed => 1,
            FailureReason::OutOfGas => 2,
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use evm_adapter::AdapterError;

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config<AccountId = AccountId32> + pallet_timestamp::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WasmBridge: WasmCallBridge<Self::AccountId>;
        type WeightInfo: WeightInfo;
		type Currency: frame_support::traits::Currency<Self::AccountId>;
		type Treasury: pallet_etwasm_vm::TreasuryInterface<Self::Balance>;
		type MaxCodeSize: Get<u32>;
		type DefaultGasLimit: Get<VMw>;
		type MaxGasLimit: Get<VMw>;
		type VmwOperationPrice: Get<u32>;
		type TreasuryFeePercent: Get<u32>;
		type Balance: frame_support::traits::tokens::Balance;
        type EtwasmConfig: pallet_etwasm_vm::pallet::Config<AccountId = Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn known_accounts)]
    pub type KnownAccounts<T: Config> = StorageMap<_, Blake2_128Concat, H160, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn bridge_targets)]
    pub type BridgeTargets<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        [u8; 4],
        BridgeTarget<T::AccountId>,
        OptionQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub known_accounts: Vec<(H160, T::AccountId)>,
        pub bridge_targets: Vec<([u8; 4], BridgeTarget<T::AccountId>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                known_accounts: Vec::new(),
                bridge_targets: Vec::new(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            for (evm_address, account_id) in &self.known_accounts {
                KnownAccounts::<T>::insert(evm_address, account_id);
            }
            for (selector, target) in &self.bridge_targets {
                BridgeTargets::<T>::insert(selector, target);
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CallDispatched { caller: T::AccountId, selector: [u8; 4], gas_used: u64 },
        CallFailed { caller: T::AccountId, reason: u8 },
        BridgeTargetRegistered { selector: [u8; 4], contract: T::AccountId, gas_limit: VMw },
        KnownAccountRegistered { evm_address: H160, account_id: T::AccountId },
        CrossVmCall { caller: T::AccountId, contract: T::AccountId, selector: [u8; 4], gas_used: VMw, success: bool },
    }

    #[pallet::error]
    pub enum Error<T> {
        MalformedCalldata,
        ExecutionFailed,
        OutOfGas,
        BridgeTargetMissing,
        BridgeEncodingFailed,
        KnownAccountMissing,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::call_evm())]
        pub fn call_evm(origin: OriginFor<T>, bytecode: Vec<u8>, calldata: Vec<u8>, gas_limit: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let caller_h160 = account_to_h160(&who);

            match EvmRuntime::execute(&bytecode, &calldata, caller_h160, caller_h160, gas_limit) {
                Ok((outcome, translated, _)) => {
                    let _ = outcome;
                    let gas_used = T::WasmBridge::dispatch(&who, &translated)
                        .map_err(|_| Error::<T>::ExecutionFailed)?;
                    Self::deposit_event(Event::CallDispatched {
                        caller: who.clone(),
                        selector: translated.evm_selector.0,
                        gas_used: gas_used as u64,
                    });
                    Ok(())
                }
                Err(err) => {
                    let (pallet_error, reason) = Self::map_error(err);
                    Self::deposit_event(Event::CallFailed { caller: who.clone(), reason: reason.into() });
                    Err(pallet_error.into())
                }
            }
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::register_bridge_target())]
        pub fn register_bridge_target(
            origin: OriginFor<T>,
            selector: [u8; 4],
            contract: T::AccountId,
            gas_limit: VMw,
        ) -> DispatchResult {
            ensure_root(origin)?;
            BridgeTargets::<T>::insert(selector, BridgeTarget { contract: contract.clone(), gas_limit });
            Self::deposit_event(Event::BridgeTargetRegistered { selector, contract, gas_limit });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::register_known_account())]
        pub fn register_known_account(
            origin: OriginFor<T>,
            evm_address: H160,
            account_id: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            KnownAccounts::<T>::insert(evm_address, account_id.clone());
            Self::deposit_event(Event::KnownAccountRegistered { evm_address, account_id });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn h160_to_account(address: &H160) -> Result<T::AccountId, Error<T>> {
            Self::known_accounts(address).ok_or(Error::<T>::KnownAccountMissing)
        }

        fn map_error(err: AdapterError) -> (Error<T>, FailureReason) {
            match err {
                AdapterError::MalformedCalldata => (Error::<T>::MalformedCalldata, FailureReason::MalformedCalldata),
                AdapterError::OutOfGas | AdapterError::GasOverflow => (Error::<T>::OutOfGas, FailureReason::OutOfGas),
                AdapterError::UnknownSelector => (Error::<T>::MalformedCalldata, FailureReason::MalformedCalldata),
                AdapterError::UnsupportedType | AdapterError::ValueOverflow => (Error::<T>::ExecutionFailed, FailureReason::ExecutionFailed),
                _ => (Error::<T>::ExecutionFailed, FailureReason::ExecutionFailed),
            }
        }
    }
}


pub struct EtwasmBridge<T: pallet::Config>(PhantomData<T>);

impl<T: pallet::Config> WasmCallBridge<T::AccountId> for EtwasmBridge<T>
{
    fn dispatch(caller: &T::AccountId, call: &TranslatedCall) -> Result<VMw, DispatchError> {
        let bridge_target = pallet::BridgeTargets::<T>::get(call.evm_selector.0)
            .ok_or(pallet::Error::<T>::BridgeTargetMissing)?;

        let payload = call
            .encode_wasm_call(|addr| pallet::Pallet::<T>::h160_to_account(addr).map_err(|_| evm_adapter::AdapterError::MalformedCalldata))
            .map_err(|_| pallet::Error::<T>::BridgeEncodingFailed)?;

        let result = etwasm::pallet::Pallet::<T::EtwasmConfig>::call_from_bridge(
            caller,
            &bridge_target.contract,
            payload,
            bridge_target.gas_limit,
        );

        match result {
            Ok(gas_used) => {
                pallet::Pallet::<T>::deposit_event(pallet::Event::CrossVmCall {
                    caller: caller.clone(),
                    contract: bridge_target.contract.clone(),
                    selector: call.evm_selector.0,
                    gas_used,
                    success: true,
                });
                Ok(gas_used)
            }
            Err(e) => {
                pallet::Pallet::<T>::deposit_event(pallet::Event::CrossVmCall {
                    caller: caller.clone(),
                    contract: bridge_target.contract.clone(),
                    selector: call.evm_selector.0,
                    gas_used: 0,
                    success: false,
                });
                Err(e)
            }
        }
    }
}
