use alloc::vec::Vec;
use codec::Encode;
use sp_core::{crypto::AccountId32, H160, U256};
use sp_core::hashing::blake2_128;
use tiny_keccak::{Hasher, Keccak};

use crate::error::{AdapterError, AdapterResult};

#[derive(Clone, Debug, PartialEq, Eq, Encode)]
pub struct FunctionSelector(pub [u8; 4]);

impl FunctionSelector {
    pub fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbiValue {
    Uint(U256),
    Address(H160),
    Bool(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamKind {
    Uint256,
    Address,
    Bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslatedCall {
    pub evm_selector: FunctionSelector,
    pub wasm_selector: [u8; 4],
    param_kinds: Vec<ParamKind>,
    params: Vec<AbiValue>,
}

impl TranslatedCall {
    pub fn encode_wasm_call<F>(&self, mut convert_address: F) -> AdapterResult<Vec<u8>>
    where
        F: FnMut(&H160) -> AdapterResult<AccountId32>,
    {
        if self.param_kinds.len() != self.params.len() {
            return Err(AdapterError::MalformedCalldata);
        }

        let mut encoded = Vec::with_capacity(4 + self.params.len() * 32);
        encoded.extend_from_slice(&self.wasm_selector);

        for (kind, value) in self.param_kinds.iter().zip(self.params.iter()) {
            match (kind, value) {
                (ParamKind::Uint256, AbiValue::Uint(val)) => {
                    if val.bit(128) {
                        return Err(AdapterError::ValueOverflow);
                    }
                    let v: u128 = val.low_u128();
                    encoded.extend_from_slice(&v.encode());
                }
                (ParamKind::Address, AbiValue::Address(addr)) => {
                    let account = convert_address(addr)?;
                    encoded.extend_from_slice(&account.encode());
                }
                (ParamKind::Bool, AbiValue::Bool(flag)) => {
                    encoded.extend_from_slice(&flag.encode());
                }
                _ => return Err(AdapterError::UnsupportedType),
            }
        }

        Ok(encoded)
    }
}

pub struct AbiTranslator;

impl AbiTranslator {
    pub fn translate(calldata: &[u8]) -> AdapterResult<TranslatedCall> {
        let (selector, params) = Self::split_calldata(calldata)?;
        let registry = FunctionRegistry::new();
        let mapping = registry
            .find_by_selector(&selector)
            .ok_or(AdapterError::UnknownSelector)?;

        let decoded_params = Self::decode_params(mapping.param_types.as_slice(), params)?;

        Ok(TranslatedCall {
            evm_selector: selector,
            wasm_selector: mapping.wasm_selector,
            param_kinds: mapping.param_types.clone(),
            params: decoded_params,
        })
    }

    pub fn split_calldata(calldata: &[u8]) -> AdapterResult<(FunctionSelector, &[u8])> {
        if calldata.len() < 4 {
            return Err(AdapterError::MalformedCalldata);
        }
        let mut selector = [0u8; 4];
        selector.copy_from_slice(&calldata[..4]);
        Ok((FunctionSelector(selector), &calldata[4..]))
    }

    fn decode_params(kinds: &[ParamKind], data: &[u8]) -> AdapterResult<Vec<AbiValue>> {
        if data.len() < kinds.len() * 32 {
            return Err(AdapterError::MalformedCalldata);
        }
        let mut values = Vec::with_capacity(kinds.len());
        for (idx, kind) in kinds.iter().enumerate() {
            let start = idx * 32;
            let end = start + 32;
            let word = &data[start..end];
            match kind {
                ParamKind::Uint256 => {
                    let value = U256::from_big_endian(word);
                    values.push(AbiValue::Uint(value));
                }
                ParamKind::Address => {
                    let mut bytes = [0u8; 20];
                    bytes.copy_from_slice(&word[12..]);
                    values.push(AbiValue::Address(H160::from(bytes)));
                }
                ParamKind::Bool => {
                    let flag = word[31] & 1 == 1;
                    values.push(AbiValue::Bool(flag));
                }
            }
        }
        Ok(values)
    }
}

struct FunctionDefinition {
    signature: &'static str,
    wasm_function: &'static str,
    param_types: &'static [ParamKind],
}

struct FunctionMapping {
    wasm_selector: [u8; 4],
    param_types: Vec<ParamKind>,
    evm_selector: FunctionSelector,
}

struct FunctionRegistry {
    mappings: Vec<FunctionMapping>,
}

impl FunctionRegistry {
    fn new() -> Self {
        let mut mappings = Vec::new();
        for def in Self::definitions() {
            mappings.push(FunctionMapping {
                wasm_selector: wasm_selector(def.wasm_function),
                param_types: def.param_types.to_vec(),
                evm_selector: FunctionSelector(keccak_selector(def.signature)),
            });
        }
        Self { mappings }
    }

    fn find_by_selector(&self, selector: &FunctionSelector) -> Option<&FunctionMapping> {
        self.mappings.iter().find(|m| m.evm_selector == *selector)
    }

    fn definitions() -> &'static [FunctionDefinition] {
        &[
            FunctionDefinition {
                signature: "swapETRForWrapped(uint256,uint256)",
                wasm_function: "swap_etr_for_wrapped",
                param_types: &[ParamKind::Uint256, ParamKind::Uint256],
            },
            FunctionDefinition {
                signature: "swapWrappedForETR(uint256,uint256)",
                wasm_function: "swap_wrapped_for_etr",
                param_types: &[ParamKind::Uint256, ParamKind::Uint256],
            },
            FunctionDefinition {
                signature: "getAmountOut(uint256,address,address)",
                wasm_function: "get_amount_out",
                param_types: &[ParamKind::Uint256, ParamKind::Address, ParamKind::Address],
            },
            FunctionDefinition {
                signature: "mintWithUSDC(uint256)",
                wasm_function: "mint_with_usdc",
                param_types: &[ParamKind::Uint256],
            },
            FunctionDefinition {
                signature: "convertToEtr(address,uint256,uint256,uint256)",
                wasm_function: "convert_to_etr",
                param_types: &[
                    ParamKind::Address,
                    ParamKind::Uint256,
                    ParamKind::Uint256,
                    ParamKind::Uint256,
                ],
            },
        ]
    }
}

fn keccak_selector(signature: &str) -> [u8; 4] {
    let mut hasher = Keccak::v256();
    hasher.update(signature.as_bytes());
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    [output[0], output[1], output[2], output[3]]
}

fn wasm_selector(function: &str) -> [u8; 4] {
    let hash = blake2_128(function.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}
