use evm_adapter::{
    AbiTranslator, CallContext, EvmInterpreter, GasConverter,
};
use sp_core::{crypto::AccountId32, H160, U256};
use tiny_keccak::{Hasher, Keccak};

#[test]
fn gas_conversion_roundtrip() {
    let converter = GasConverter::new();
    let vmw = converter.gwei_to_vmw(2).expect("conversion");
    assert_eq!(vmw, 20_000);
    let gwei = converter.vmw_to_gwei(vmw).expect("reverse");
    assert_eq!(gwei, 2);
}

#[test]
fn abi_translation_decodes_swap() {
    let selector = keccak_selector(\"swapETRForWrapped(uint256,uint256)\");
    let mut calldata = Vec::new();
    calldata.extend_from_slice(&selector);
    calldata.extend_from_slice(&[0u8; 32]); // amount in
    calldata.extend_from_slice(&[0u8; 32]); // min out

    let translated = AbiTranslator::translate(&calldata).expect(\"translate\");
    assert_eq!(translated.encode_wasm_call(|_| Ok(AccountId32::new([0u8; 32]))).is_ok(), true);
}

#[test]
fn interpreter_adds_numbers() {
    let context = CallContext::new(H160::repeat_byte(0x11), H160::repeat_byte(0x22), U256::zero(), Vec::new());
    let mut interpreter = EvmInterpreter::new(context, 1_000);

    // PUSH1 0x01 PUSH1 0x02 ADD STOP
    let bytecode = [0x60, 0x01, 0x60, 0x02, 0x01, 0x00];
    let outcome = interpreter.execute(&bytecode).expect("execution");
    assert_eq!(outcome.exit_reason, evm_adapter::ExitReason::Succeed);
    assert!(outcome.return_data.is_empty());
}

fn keccak_selector(sig: &str) -> [u8; 4] {
    let mut hasher = Keccak::v256();
    hasher.update(sig.as_bytes());
    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    [out[0], out[1], out[2], out[3]]
}
