use alloc::vec::Vec;
use sp_core::{H160, U256};

use evm_adapter::{AbiTranslator, AdapterResult, CallContext, EvmInterpreter, ExecutionOutcome, GasConverter, TranslatedCall};

/// Executes bytecode using the shared adapter utilities.
pub struct EvmRuntime;

impl EvmRuntime {
    pub fn execute(
        bytecode: &[u8],
        calldata: &[u8],
        caller: H160,
        contract: H160,
        gas_limit: u64,
    ) -> AdapterResult<(ExecutionOutcome, TranslatedCall, u128)> {
        let translated = AbiTranslator::translate(calldata)?;
        let gas_converter = GasConverter::new();
        let vmw = gas_converter.gwei_to_vmw(gas_limit)?;

        let context = CallContext::new(contract, caller, U256::zero(), calldata.to_vec());
        let mut interpreter = EvmInterpreter::new(context, gas_limit);
        let outcome = interpreter.execute(bytecode)?;
        Ok((outcome, translated, vmw))
    }
}
