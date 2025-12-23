#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod abi_translator;
pub mod bytecode_interpreter;
pub mod error;
pub mod gas_converter;
pub mod precompiles;
pub mod storage_mapper;

pub use abi_translator::{AbiTranslator, FunctionSelector, TranslatedCall};
pub use bytecode_interpreter::{CallContext, EvmInterpreter, ExecutionOutcome, ExitReason};
pub use error::{AdapterError, AdapterResult};
pub use gas_converter::GasConverter;
pub use precompiles::{PrecompileRegistry, PrecompileTarget};
pub use storage_mapper::{AccountMapping, StorageMapper};
