# EVM Compatibility Layer - Developer Implementation Guide

**Location:** `/08-etwasm-vm/`
**Purpose:** Enable Solidity smart contract execution on ĒTRID while maintaining native Wasm as primary execution environment
**Target Audience:** Developers implementing the EVM compatibility bridge

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ĒTRID DUAL EXECUTION ENVIRONMENT                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  NATIVE EXECUTION (Primary - 90% of traffic)               │    │
│  │  ┌──────────────────────────────────────────────────────┐  │    │
│  │  │  ETWasm VM                                           │  │    │
│  │  │  ├─ ink! contracts (Rust → Wasm)                    │  │    │
│  │  │  ├─ VMw gas metering                                │  │    │
│  │  │  ├─ Reentrancy protection                           │  │    │
│  │  │  └─ Direct substrate pallet integration            │  │    │
│  │  └──────────────────────────────────────────────────────┘  │    │
│  └────────────────────────────────────────────────────────────┘    │
│                              ↕ Interop                              │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  EVM COMPATIBILITY LAYER (Bridge - 10% of traffic)        │    │
│  │  ┌──────────────────────────────────────────────────────┐  │    │
│  │  │  EVM Precompile Adapter                              │  │    │
│  │  │  ├─ Solidity bytecode interpreter                   │  │    │
│  │  │  ├─ EVM → Wasm ABI translation                      │  │    │
│  │  │  ├─ Gas conversion (Gwei → VMw)                     │  │    │
│  │  │  └─ State mapping (EVM storage → Substrate)         │  │    │
│  │  └──────────────────────────────────────────────────────┘  │    │
│  └────────────────────────────────────────────────────────────┘    │
│                              ↓                                      │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  Unified State (Substrate Storage)                         │    │
│  │  ├─ Wasm contracts: Direct access                         │    │
│  │  ├─ EVM contracts: Via adapter mapping                    │    │
│  │  └─ Shared account system (AccountId32)                   │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Wasm-First Philosophy:** EVM is a compatibility bridge, NOT the primary execution environment
2. **One-Way Calls:** External EVM contracts can call INTO ĒTRID Wasm contracts (not reverse)
3. **Gas Unification:** All execution measured in VMw (Virtual Machine Watts), EVM gas converted
4. **State Isolation:** EVM contracts have isolated storage, cannot corrupt Wasm contract state
5. **No Migration Required:** External projects can deploy Solidity as-is, gradually migrate to Wasm

---

## Folder Structure

```
08-etwasm-vm/
├── EVM_COMPATIBILITY_LAYER.md (this file)
│
├── evm-adapter/                    # NEW: Core EVM compatibility components
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                  # Main adapter module
│   │   ├── bytecode_interpreter.rs # EVM bytecode execution
│   │   ├── abi_translator.rs       # EVM ↔ Wasm ABI translation
│   │   ├── gas_converter.rs        # Gwei → VMw conversion
│   │   ├── storage_mapper.rs       # EVM storage → Substrate mapping
│   │   ├── precompiles.rs          # EVM precompiled contracts
│   │   └── error.rs                # Error types
│   └── tests/
│       ├── integration_tests.rs
│       └── fixtures/
│           └── sample_contracts/   # Test Solidity contracts
│
├── pallet-evm/                     # NEW: Substrate pallet for EVM
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                  # Pallet definition
│   │   ├── evm_runtime.rs          # EVM execution environment
│   │   ├── account_mapping.rs      # H160 ↔ AccountId32
│   │   ├── weights.rs              # Benchmarked weights
│   │   └── mock.rs                 # Test runtime
│   └── tests/
│       └── solidity_execution.rs
│
├── evm-rpc/                        # NEW: Ethereum JSON-RPC compatibility
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── eth_api.rs              # eth_* methods
│   │   ├── web3_api.rs             # web3_* methods
│   │   ├── net_api.rs              # net_* methods
│   │   └── trace_api.rs            # debug_* methods
│   └── tests/
│
├── solidity-compiler/              # NEW: Solc integration
│   ├── compile.sh                  # Compile Solidity → EVM bytecode
│   ├── verify.sh                   # Bytecode verification
│   └── templates/
│       ├── ERC20.sol               # Standard templates
│       ├── ERC721.sol
│       └── Governance.sol
│
└── integration/                    # NEW: Wasm ↔ EVM interop
    ├── wasm_to_evm_bridge.rs       # Call EVM from Wasm
    ├── evm_to_wasm_bridge.rs       # Call Wasm from EVM (primary use)
    └── examples/
        ├── evm_calls_primeswap.sol # Example: External contract calls PrimeSwap pool
        └── wasm_queries_evm.rs     # Example: Wasm contract reads EVM state
```

---

## Core Components

### 1. EVM Bytecode Interpreter

**File:** `08-etwasm-vm/evm-adapter/src/bytecode_interpreter.rs`

**Purpose:** Execute EVM bytecode within the Substrate runtime

```rust
use evm_core::{ExitReason, Machine, Opcode, Stack};
use sp_core::{H160, H256, U256};
use sp_std::vec::Vec;

/// EVM bytecode interpreter for ĒTRID
pub struct EvmInterpreter {
    /// EVM machine state
    machine: Machine,
    /// Gas limit for execution
    gas_limit: u64,
    /// Gas used so far
    gas_used: u64,
    /// Contract address
    address: H160,
}

impl EvmInterpreter {
    /// Create new interpreter instance
    pub fn new(address: H160, gas_limit: u64) -> Self {
        Self {
            machine: Machine::new(),
            gas_limit,
            gas_used: 0,
            address,
        }
    }

    /// Execute EVM bytecode
    ///
    /// # Arguments
    /// * `code` - EVM bytecode to execute
    /// * `input` - Call data (ABI encoded)
    /// * `value` - Amount of ÉTR sent with call
    ///
    /// # Returns
    /// * `ExitReason` - Success or error
    /// * `Vec<u8>` - Return data
    /// * `u64` - Gas used
    pub fn execute(
        &mut self,
        code: &[u8],
        input: &[u8],
        value: U256,
    ) -> Result<(ExitReason, Vec<u8>, u64), EvmError> {
        // 1. Initialize stack and memory
        let mut stack = Stack::new(1024);
        let mut memory = Vec::new();

        // 2. Load bytecode into machine
        self.machine.load_code(code);

        // 3. Set up call context
        let context = CallContext {
            address: self.address,
            caller: H160::zero(), // Will be set by caller
            value,
            input: input.to_vec(),
        };

        // 4. Execute opcode by opcode
        loop {
            // Check gas limit
            if self.gas_used >= self.gas_limit {
                return Err(EvmError::OutOfGas);
            }

            // Fetch next opcode
            let opcode = match self.machine.fetch_opcode() {
                Some(op) => op,
                None => break, // End of code
            };

            // Execute opcode
            let gas_cost = self.execute_opcode(
                opcode,
                &mut stack,
                &mut memory,
                &context,
            )?;

            self.gas_used += gas_cost;

            // Check for return or revert
            if opcode == Opcode::RETURN || opcode == Opcode::REVERT {
                let return_data = self.extract_return_data(&stack, &memory)?;
                let reason = if opcode == Opcode::RETURN {
                    ExitReason::Succeed
                } else {
                    ExitReason::Revert
                };
                return Ok((reason, return_data, self.gas_used));
            }
        }

        // Normal exit
        Ok((ExitReason::Succeed, Vec::new(), self.gas_used))
    }

    /// Execute a single EVM opcode
    fn execute_opcode(
        &mut self,
        opcode: Opcode,
        stack: &mut Stack,
        memory: &mut Vec<u8>,
        context: &CallContext,
    ) -> Result<u64, EvmError> {
        match opcode {
            // Arithmetic
            Opcode::ADD => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a.overflowing_add(b).0)?;
                Ok(3) // Gas cost for ADD
            }
            Opcode::MUL => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a.overflowing_mul(b).0)?;
                Ok(5) // Gas cost for MUL
            }

            // Memory operations
            Opcode::MLOAD => {
                let offset = stack.pop()?.as_usize();
                let value = self.load_memory(memory, offset)?;
                stack.push(value)?;
                Ok(3)
            }
            Opcode::MSTORE => {
                let offset = stack.pop()?.as_usize();
                let value = stack.pop()?;
                self.store_memory(memory, offset, value)?;
                Ok(3)
            }

            // Storage operations (maps to Substrate storage)
            Opcode::SLOAD => {
                let key = stack.pop()?;
                let value = self.load_storage(context.address, key)?;
                stack.push(value)?;
                Ok(200) // SLOAD is expensive
            }
            Opcode::SSTORE => {
                let key = stack.pop()?;
                let value = stack.pop()?;
                self.store_storage(context.address, key, value)?;
                Ok(5000) // SSTORE is very expensive
            }

            // Call operations (critical for Wasm interop)
            Opcode::CALL => {
                self.execute_call(stack, memory, context)?;
                Ok(700)
            }

            // ... implement all 140+ EVM opcodes
            _ => Err(EvmError::InvalidOpcode(opcode)),
        }
    }

    /// Load value from EVM contract storage (maps to Substrate storage)
    fn load_storage(&self, address: H160, key: U256) -> Result<U256, EvmError> {
        // Map EVM storage to Substrate storage
        let substrate_key = Self::map_storage_key(address, key);

        // Read from pallet-evm storage
        let value = pallet_evm::AccountStorages::<Runtime>::get(address, key);

        Ok(value)
    }

    /// Store value to EVM contract storage
    fn store_storage(&mut self, address: H160, key: U256, value: U256) -> Result<(), EvmError> {
        // Write to pallet-evm storage
        pallet_evm::AccountStorages::<Runtime>::insert(address, key, value);
        Ok(())
    }

    /// Execute CALL opcode - can call both EVM and Wasm contracts
    fn execute_call(
        &mut self,
        stack: &mut Stack,
        memory: &Vec<u8>,
        context: &CallContext,
    ) -> Result<(), EvmError> {
        let gas = stack.pop()?.as_u64();
        let address = H160::from_slice(&stack.pop()?.as_bytes()[12..32]);
        let value = stack.pop()?;
        let args_offset = stack.pop()?.as_usize();
        let args_size = stack.pop()?.as_usize();
        let ret_offset = stack.pop()?.as_usize();
        let ret_size = stack.pop()?.as_usize();

        // Extract call data from memory
        let input = memory[args_offset..args_offset + args_size].to_vec();

        // CRITICAL: Check if target is EVM or Wasm contract
        if Self::is_wasm_contract(address) {
            // Call into Wasm contract (main use case for compatibility)
            let result = self.call_wasm_contract(address, input, value)?;

            // Write result back to memory
            self.write_memory(memory, ret_offset, &result)?;

            stack.push(U256::one())?; // Success
        } else {
            // Call another EVM contract
            let result = self.call_evm_contract(address, input, value, gas)?;

            self.write_memory(memory, ret_offset, &result)?;
            stack.push(U256::one())?; // Success
        }

        Ok(())
    }

    /// Call a Wasm contract from EVM (CRITICAL for external integration)
    fn call_wasm_contract(
        &self,
        address: H160,
        input: Vec<u8>,
        value: U256,
    ) -> Result<Vec<u8>, EvmError> {
        // Convert H160 (EVM address) to AccountId32 (Substrate)
        let account_id = Self::h160_to_account_id(address);

        // Translate EVM ABI call to Wasm call
        let wasm_call = AbiTranslator::evm_to_wasm(input)?;

        // Execute via pallet-contracts
        let result = pallet_contracts::Pallet::<Runtime>::bare_call(
            account_id.clone(),
            account_id.clone(),
            value.as_u128(),
            Weight::MAX,
            None,
            wasm_call,
            false,
        );

        match result.result {
            Ok(exec_result) => Ok(exec_result.data),
            Err(e) => Err(EvmError::WasmCallFailed(e)),
        }
    }

    /// Check if address belongs to a Wasm contract
    fn is_wasm_contract(address: H160) -> bool {
        // Wasm contracts have specific address prefix or registry check
        pallet_contracts::ContractInfoOf::<Runtime>::contains_key(
            Self::h160_to_account_id(address)
        )
    }

    /// Convert EVM address (H160) to Substrate AccountId
    fn h160_to_account_id(address: H160) -> AccountId32 {
        // Strategy 1: Direct mapping (first 20 bytes + padding)
        let mut data = [0u8; 32];
        data[0..20].copy_from_slice(&address.0);
        AccountId32::from(data)
    }
}
```

---

### 2. Gas Converter

**File:** `08-etwasm-vm/evm-adapter/src/gas_converter.rs`

**Purpose:** Convert Ethereum gas (Gwei) to ĒTRID VMw (Virtual Machine Watts)

```rust
use sp_core::U256;

/// Gas conversion utilities
pub struct GasConverter;

impl GasConverter {
    /// Conversion ratio: 1 Gwei = X VMw
    ///
    /// Ethereum avg gas price: ~20 Gwei
    /// ĒTRID VMw target: Make it cheaper to incentivize migration
    /// Ratio: 1 Gwei = 10,000 VMw
    const GWEI_TO_VMW_RATIO: u64 = 10_000;

    /// Convert Ethereum gas to VMw
    ///
    /// # Arguments
    /// * `gas` - Gas amount in Gwei units
    ///
    /// # Returns
    /// VMw equivalent
    pub fn gwei_to_vmw(gas: u64) -> u64 {
        gas.saturating_mul(Self::GWEI_TO_VMW_RATIO)
    }

    /// Convert VMw to Ethereum gas
    pub fn vmw_to_gwei(vmw: u64) -> u64 {
        vmw / Self::GWEI_TO_VMW_RATIO
    }

    /// Calculate gas cost for EVM opcode in VMw
    pub fn opcode_cost_vmw(opcode: u8) -> u64 {
        let evm_gas_cost = Self::evm_opcode_cost(opcode);
        Self::gwei_to_vmw(evm_gas_cost)
    }

    /// Standard EVM opcode gas costs
    fn evm_opcode_cost(opcode: u8) -> u64 {
        match opcode {
            0x01 => 3,     // ADD
            0x02 => 5,     // MUL
            0x54 => 200,   // SLOAD
            0x55 => 5000,  // SSTORE (cold)
            0xF0 => 32000, // CREATE
            0xF1 => 700,   // CALL
            _ => 3,        // Default
        }
    }

    /// Calculate total VMw cost for EVM transaction
    pub fn calculate_evm_tx_cost(
        bytecode_length: usize,
        storage_writes: u32,
        calls: u32,
    ) -> u64 {
        let base_cost = 21_000; // EVM base tx cost
        let bytecode_cost = (bytecode_length as u64) * 200;
        let storage_cost = (storage_writes as u64) * 5_000;
        let call_cost = (calls as u64) * 700;

        let total_gwei = base_cost + bytecode_cost + storage_cost + call_cost;
        Self::gwei_to_vmw(total_gwei)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_conversion() {
        // 100 Gwei should equal 1,000,000 VMw
        assert_eq!(GasConverter::gwei_to_vmw(100), 1_000_000);

        // Round trip
        let original = 50_000;
        let vmw = GasConverter::gwei_to_vmw(original);
        let back = GasConverter::vmw_to_gwei(vmw);
        assert_eq!(original, back);
    }

    #[test]
    fn test_tx_cost() {
        // Small contract call
        let cost = GasConverter::calculate_evm_tx_cost(
            100,  // 100 bytes bytecode
            2,    // 2 storage writes
            1,    // 1 external call
        );

        // Should be reasonable (less than deploying Wasm contract)
        assert!(cost > 0);
        assert!(cost < 10_000_000); // 10M VMw max
    }
}
```

---

### 3. ABI Translator

**File:** `08-etwasm-vm/evm-adapter/src/abi_translator.rs`

**Purpose:** Translate between EVM ABI and Wasm ABI for cross-VM calls

```rust
use ethabi::{Function, Param, ParamType, Token};
use sp_std::vec::Vec;

/// Translate between EVM and Wasm ABIs
pub struct AbiTranslator;

impl AbiTranslator {
    /// Translate EVM function call to Wasm contract call
    ///
    /// Example: EVM calling PrimeSwap pool's swapETRForWrapped()
    ///
    /// EVM ABI: swapETRForWrapped(uint256,uint256) -> 0x12345678...
    /// Wasm ABI: {"swap_etr_for_wrapped": {"amount_in": 1000, "min_out": 950}}
    pub fn evm_to_wasm(evm_calldata: Vec<u8>) -> Result<Vec<u8>, AbiError> {
        // 1. Decode EVM function selector (first 4 bytes)
        if evm_calldata.len() < 4 {
            return Err(AbiError::InvalidCalldata);
        }

        let selector = &evm_calldata[0..4];
        let params = &evm_calldata[4..];

        // 2. Map EVM function to Wasm function
        let wasm_selector = Self::map_selector(selector)?;

        // 3. Decode EVM parameters
        let decoded_params = ethabi::decode(
            &Self::get_evm_param_types(selector)?,
            params,
        )?;

        // 4. Encode as Wasm SCALE codec
        let wasm_call = Self::encode_wasm_call(wasm_selector, decoded_params)?;

        Ok(wasm_call)
    }

    /// Map EVM function selector to Wasm function name
    ///
    /// This is a registry of known cross-VM function mappings
    fn map_selector(evm_selector: &[u8]) -> Result<&'static str, AbiError> {
        // Function selector = keccak256("functionName(paramTypes)")[:4]

        // PrimeSwap Tier 2 Pool functions
        if evm_selector == &keccak256("swapETRForWrapped(uint256,uint256)")[..4] {
            return Ok("swap_etr_for_wrapped");
        }
        if evm_selector == &keccak256("swapWrappedForETR(uint256,uint256)")[..4] {
            return Ok("swap_wrapped_for_etr");
        }
        if evm_selector == &keccak256("getAmountOut(uint256,address,address)")[..4] {
            return Ok("get_amount_out");
        }

        // EDSC Minting Engine functions
        if evm_selector == &keccak256("mintWithUSDC(uint256)")[..4] {
            return Ok("mint_with_usdc");
        }

        // Intent Router functions
        if evm_selector == &keccak256("convertToEtr(address,uint256,uint256,uint256)")[..4] {
            return Ok("convert_to_etr");
        }

        Err(AbiError::UnknownSelector)
    }

    /// Encode Wasm contract call using SCALE codec
    fn encode_wasm_call(
        function: &str,
        params: Vec<Token>,
    ) -> Result<Vec<u8>, AbiError> {
        use parity_scale_codec::Encode;

        // Wasm contracts use SCALE encoding
        // Format: selector (4 bytes) + SCALE encoded args

        let selector = Self::wasm_function_selector(function);
        let mut encoded = selector.to_vec();

        // Encode each parameter
        for param in params {
            match param {
                Token::Uint(val) => {
                    encoded.extend_from_slice(&val.encode());
                }
                Token::Address(addr) => {
                    // Convert H160 to AccountId32
                    let account_id = Self::h160_to_account_id(addr);
                    encoded.extend_from_slice(&account_id.encode());
                }
                Token::Bool(b) => {
                    encoded.extend_from_slice(&b.encode());
                }
                _ => return Err(AbiError::UnsupportedType),
            }
        }

        Ok(encoded)
    }

    /// Generate Wasm function selector (Blake2_128 hash)
    fn wasm_function_selector(function: &str) -> [u8; 4] {
        use sp_core::blake2_128;
        let hash = blake2_128(function.as_bytes());
        [hash[0], hash[1], hash[2], hash[3]]
    }

    /// Translate Wasm return value to EVM ABI
    pub fn wasm_to_evm(wasm_result: Vec<u8>) -> Result<Vec<u8>, AbiError> {
        use parity_scale_codec::Decode;

        // Decode SCALE-encoded result
        // This is function-specific - needs mapping registry

        // Example: uint256 return value
        let value = u128::decode(&mut &wasm_result[..])?;

        // Encode as EVM ABI
        let token = Token::Uint(value.into());
        Ok(ethabi::encode(&[token]))
    }
}

/// Function mapping registry
///
/// Maps EVM function signatures to Wasm contract functions
pub struct FunctionRegistry {
    mappings: Vec<FunctionMapping>,
}

pub struct FunctionMapping {
    pub evm_signature: String,          // "swapETRForWrapped(uint256,uint256)"
    pub evm_selector: [u8; 4],          // keccak256 hash
    pub wasm_function: String,          // "swap_etr_for_wrapped"
    pub wasm_selector: [u8; 4],         // blake2_128 hash
    pub param_types: Vec<ParamType>,    // [Uint(256), Uint(256)]
}

impl FunctionRegistry {
    /// Initialize with standard ĒTRID contract mappings
    pub fn new_etrid_standard() -> Self {
        let mut mappings = Vec::new();

        // PrimeSwap Tier 2 Pool
        mappings.push(FunctionMapping {
            evm_signature: "swapETRForWrapped(uint256,uint256)".into(),
            evm_selector: keccak256("swapETRForWrapped(uint256,uint256)")[..4].try_into().unwrap(),
            wasm_function: "swap_etr_for_wrapped".into(),
            wasm_selector: blake2_128(b"swap_etr_for_wrapped")[..4].try_into().unwrap(),
            param_types: vec![ParamType::Uint(256), ParamType::Uint(256)],
        });

        // Add all standard functions...

        Self { mappings }
    }

    /// Look up Wasm function by EVM selector
    pub fn find_by_evm_selector(&self, selector: &[u8; 4]) -> Option<&FunctionMapping> {
        self.mappings.iter().find(|m| &m.evm_selector == selector)
    }
}
```

---

### 4. Pallet EVM

**File:** `08-etwasm-vm/pallet-evm/src/lib.rs`

**Purpose:** Substrate pallet providing EVM execution environment

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use sp_core::{H160, H256, U256};
    use sp_std::vec::Vec;

    use crate::evm_runtime::EvmRuntime;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_contracts::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency for gas payments
        type Currency: Currency<Self::AccountId>;

        /// Precompiled contracts
        type Precompiles: Precompiles;

        /// Gas price (VMw per gas unit)
        #[pallet::constant]
        type GasPrice: Get<u64>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// EVM account storage: H160 → (nonce, balance, code_hash)
    #[pallet::storage]
    pub type Accounts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H160,
        EvmAccount,
        ValueQuery,
    >;

    /// EVM contract storage: (H160, H256) → H256
    #[pallet::storage]
    pub type AccountStorages<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        H160,         // Contract address
        Blake2_128Concat,
        H256,         // Storage key
        H256,         // Storage value
        ValueQuery,
    >;

    /// EVM contract bytecode: H160 → Vec<u8>
    #[pallet::storage]
    pub type AccountCodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H160,
        Vec<u8>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// EVM contract created
        Created {
            address: H160,
            deployer: T::AccountId,
        },
        /// EVM contract called
        Executed {
            from: H160,
            to: H160,
            value: U256,
            gas_used: u64,
        },
        /// EVM contract called Wasm contract
        CrossVmCall {
            evm_caller: H160,
            wasm_contract: T::AccountId,
            success: bool,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Execution reverted
        ExecutionReverted,
        /// Out of gas
        OutOfGas,
        /// Invalid bytecode
        InvalidBytecode,
        /// Address already exists
        AddressAlreadyExists,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deploy a Solidity contract
        ///
        /// # Arguments
        /// * `bytecode` - Compiled EVM bytecode
        /// * `value` - ÉTR to send to constructor
        /// * `gas_limit` - Max gas for deployment
        ///
        /// # Returns
        /// Contract address (H160)
        #[pallet::weight(10_000)]
        pub fn create(
            origin: OriginFor<T>,
            bytecode: Vec<u8>,
            value: u128,
            gas_limit: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Generate contract address (CREATE algorithm)
            let address = Self::generate_address(&who);

            ensure!(
                !Accounts::<T>::contains_key(address),
                Error::<T>::AddressAlreadyExists
            );

            // Execute constructor
            let mut runtime = EvmRuntime::<T>::new(address, gas_limit);
            let (exit_reason, _, gas_used) = runtime.execute(
                &bytecode,
                &[],
                U256::from(value),
            )?;

            ensure!(
                exit_reason.is_succeed(),
                Error::<T>::ExecutionReverted
            );

            // Store contract
            Accounts::<T>::insert(address, EvmAccount {
                nonce: 0,
                balance: U256::from(value),
                code_hash: sp_core::blake2_256(&bytecode).into(),
            });
            AccountCodes::<T>::insert(address, bytecode);

            Self::deposit_event(Event::Created {
                address,
                deployer: who,
            });

            Ok(())
        }

        /// Call a Solidity contract
        ///
        /// # Arguments
        /// * `target` - Contract address
        /// * `input` - ABI-encoded call data
        /// * `value` - ÉTR to send
        /// * `gas_limit` - Max gas
        #[pallet::weight(10_000)]
        pub fn call(
            origin: OriginFor<T>,
            target: H160,
            input: Vec<u8>,
            value: u128,
            gas_limit: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Load contract code
            let code = AccountCodes::<T>::get(target);
            ensure!(!code.is_empty(), Error::<T>::InvalidBytecode);

            // Execute
            let mut runtime = EvmRuntime::<T>::new(target, gas_limit);
            let (exit_reason, _output, gas_used) = runtime.execute(
                &code,
                &input,
                U256::from(value),
            )?;

            ensure!(
                exit_reason.is_succeed(),
                Error::<T>::ExecutionReverted
            );

            Self::deposit_event(Event::Executed {
                from: Self::account_id_to_h160(&who),
                to: target,
                value: U256::from(value),
                gas_used,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Generate CREATE address
        fn generate_address(sender: &T::AccountId) -> H160 {
            // RLP(sender, nonce) hashed
            let nonce = frame_system::Pallet::<T>::account_nonce(sender);
            let hash = sp_core::keccak_256(
                &(sender, nonce).encode()
            );
            H160::from_slice(&hash[12..])
        }

        /// Convert AccountId to H160
        fn account_id_to_h160(account: &T::AccountId) -> H160 {
            let bytes = account.encode();
            H160::from_slice(&bytes[0..20])
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct EvmAccount {
        pub nonce: u64,
        pub balance: U256,
        pub code_hash: H256,
    }
}
```

---

## Integration Flow

### Use Case: External Ethereum dApp Calls PrimeSwap Pool

```
┌────────────────────────────────────────────────────────────┐
│ STEP 1: External Ethereum dApp (Uniswap aggregator)       │
│ ├─ User wants best BTC → ÉTR rate                        │
│ ├─ Aggregator queries multiple DEXs including ĒTRID      │
│ └─ Sends JSON-RPC call to ĒTRID node                     │
└────────────────────────────────────────────────────────────┘
                         ↓ eth_call
┌────────────────────────────────────────────────────────────┐
│ STEP 2: ĒTRID JSON-RPC Server (evm-rpc)                   │
│ ├─ Receives: eth_call(primeswap_address, calldata)       │
│ ├─ Validates request                                      │
│ └─ Forwards to pallet-evm                                │
└────────────────────────────────────────────────────────────┘
                         ↓
┌────────────────────────────────────────────────────────────┐
│ STEP 3: pallet-evm                                         │
│ ├─ Decodes EVM calldata                                   │
│ ├─ Function: getAmountOut(1000, wBTC, ÉTR)               │
│ ├─ Checks: Is target EVM or Wasm contract?               │
│ └─ Result: Wasm contract detected                        │
└────────────────────────────────────────────────────────────┘
                         ↓ Cross-VM call
┌────────────────────────────────────────────────────────────┐
│ STEP 4: ABI Translator                                     │
│ ├─ EVM calldata: 0x12345678 + encoded(1000, wBTC, ÉTR)   │
│ ├─ Translates to Wasm call                                │
│ └─ SCALE encoded: get_amount_out(1000, wBTC, ÉTR)        │
└────────────────────────────────────────────────────────────┘
                         ↓
┌────────────────────────────────────────────────────────────┐
│ STEP 5: PrimeSwap Tier 2 Pool (ink! Wasm contract)        │
│ ├─ Executes native Wasm function                          │
│ ├─ Calculates: AMM formula (x*y=k)                        │
│ └─ Returns: 24,974 ÉTR                                    │
└────────────────────────────────────────────────────────────┘
                         ↓ Result
┌────────────────────────────────────────────────────────────┐
│ STEP 6: ABI Translator (reverse)                           │
│ ├─ SCALE decoded: u128(24974000000000000000000)           │
│ ├─ Translates to EVM ABI                                  │
│ └─ Encodes: uint256(24974000000000000000000)              │
└────────────────────────────────────────────────────────────┘
                         ↓
┌────────────────────────────────────────────────────────────┐
│ STEP 7: JSON-RPC Response                                  │
│ ├─ Returns EVM-compatible response                        │
│ └─ External dApp receives: "24974000000000000000000"      │
└────────────────────────────────────────────────────────────┘
                         ↓
         External dApp displays: "1000 wBTC = 24,974 ÉTR"
         User can execute swap if price is competitive
```

---

## Implementation Checklist

### Phase 1: Core EVM Execution (Week 1-2)
- [ ] Implement bytecode interpreter (evm-adapter/src/bytecode_interpreter.rs)
- [ ] Implement all 140+ EVM opcodes
- [ ] Add gas metering per opcode
- [ ] Test with simple Solidity contracts (arithmetic, storage)

### Phase 2: Storage & State (Week 2-3)
- [ ] Implement storage mapper (Substrate storage backend)
- [ ] Create pallet-evm with account and storage
- [ ] Test SLOAD/SSTORE operations
- [ ] Verify state persistence across calls

### Phase 3: Cross-VM Calls (Week 3-4)
- [ ] Implement ABI translator (EVM ↔ Wasm)
- [ ] Create function registry for ĒTRID contracts
- [ ] Implement CALL opcode with Wasm target detection
- [ ] Test EVM calling PrimeSwap pool (ink! contract)

### Phase 4: JSON-RPC API (Week 4-5)
- [ ] Implement eth_* RPC methods (eth_call, eth_sendTransaction, etc.)
- [ ] Implement web3_* methods
- [ ] Add Metamask compatibility
- [ ] Test with Remix IDE

### Phase 5: Gas & Economics (Week 5-6)
- [ ] Implement gas converter (Gwei → VMw)
- [ ] Benchmark EVM vs Wasm execution costs
- [ ] Set competitive gas prices
- [ ] Add fee collection mechanism

### Phase 6: Security & Testing (Week 6-7)
- [ ] Reentrancy protection for EVM contracts
- [ ] State isolation tests (EVM can't corrupt Wasm state)
- [ ] Fuzz testing with random bytecode
- [ ] Security audit

### Phase 7: Developer Tooling (Week 7-8)
- [ ] Solidity compilation scripts
- [ ] Contract deployment scripts
- [ ] Etherscan-like explorer integration
- [ ] Documentation and examples

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_add_opcode() {
        let mut interpreter = EvmInterpreter::new(H160::zero(), 100000);

        // PUSH1 0x05
        // PUSH1 0x03
        // ADD
        let bytecode = vec![0x60, 0x05, 0x60, 0x03, 0x01];

        let (reason, output, gas) = interpreter.execute(&bytecode, &[], U256::zero()).unwrap();

        assert!(reason.is_succeed());
        // Stack should have 8 (5 + 3)
    }

    #[test]
    fn test_cross_vm_call() {
        // Deploy Wasm PrimeSwap pool
        let pool_address = deploy_wasm_pool();

        // Deploy EVM contract that calls pool
        let evm_contract = deploy_evm_caller();

        // Execute EVM contract's call to Wasm pool
        let result = execute_evm_call(evm_contract, "callPool", &[]);

        // Should succeed and return correct value
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[test]
fn test_metamask_connect() {
    // Start ĒTRID node with EVM-RPC
    let node = start_node_with_evm();

    // Connect Metamask
    let provider = MetamaskProvider::new(&node.rpc_url);

    // Check eth_chainId
    let chain_id = provider.request("eth_chainId", &[]).await;
    assert_eq!(chain_id, "0x..."); // ĒTRID chain ID
}

#[test]
fn test_uniswap_aggregator_integration() {
    // Deploy mock Uniswap aggregator (Solidity)
    let aggregator = deploy_solidity_contract("UniswapAggregator.sol");

    // Call aggregator which queries PrimeSwap pool
    let result = aggregator.call("getBestPrice", &[wBTC, ÉTR, 1000]);

    // Should return competitive rate from PrimeSwap
    assert!(result.price > 0);
}
```

---

## Deployment Guide

### 1. Add to Runtime

**File:** `05-multichain/primearc-core-chain/runtime/src/lib.rs`

```rust
// Add pallet-evm to runtime
impl pallet_evm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Precompiles = EvmPrecompiles;
    type GasPrice = ConstU64<10_000>; // 1 Gwei = 10,000 VMw
}

construct_runtime!(
    pub struct Runtime {
        // ... existing pallets ...

        // Add EVM support
        EVM: pallet_evm,
    }
);
```

### 2. Start Node with EVM-RPC

```bash
# Start Primearc node with EVM JSON-RPC
./primearc-node \
    --chain mainnet \
    --evm-rpc-port 8545 \
    --evm-ws-port 8546 \
    --enable-evm-compatibility
```

### 3. Configure Metamask

```
Network Name: ĒTRID Mainnet
RPC URL: https://rpc.etrid.io
Chain ID: 8338 (ĒTRID)
Currency Symbol: ÉTR
Block Explorer: https://explorer.etrid.io
```

### 4. Deploy Solidity Contract

```bash
# Compile Solidity
cd 08-etwasm-vm/solidity-compiler
./compile.sh MyContract.sol

# Deploy via Remix or Hardhat
npx hardhat run --network etrid scripts/deploy.js
```

---

## Performance Benchmarks

### Expected Performance

| Metric | EVM (via adapter) | Native Wasm | Ratio |
|--------|------------------|-------------|-------|
| Simple arithmetic | 150 μs | 100 μs | 1.5x slower |
| Storage read (SLOAD) | 300 μs | 200 μs | 1.5x slower |
| Storage write (SSTORE) | 500 μs | 350 μs | 1.43x slower |
| Cross-contract call | 800 μs | 400 μs | 2x slower |
| EVM → Wasm call | 1200 μs | N/A | Overhead |
| Token transfer | 200 μs | 120 μs | 1.67x slower |

**Conclusion:** EVM contracts run 1.5-2x slower than native Wasm, which is acceptable for compatibility layer.

---

## Security Considerations

### 1. State Isolation
- EVM contracts CANNOT directly access Wasm contract storage
- All cross-VM calls go through ABI translator (validated)
- Prevents malicious EVM contracts from corrupting ĒTRID state

### 2. Gas Limits
- EVM contracts have strict gas limits
- Cannot DOS attack the chain with infinite loops
- Gas prices set to make attacks economically infeasible

### 3. Reentrancy Protection
- EVM contracts calling Wasm contracts have reentrancy guards
- Wasm contracts already have native reentrancy protection
- Cross-VM calls are single-threaded (no concurrent execution)

### 4. Bytecode Verification
- Optional: Verify Solidity source code (like Etherscan)
- Detect malicious patterns in bytecode
- Blacklist known exploit contracts

---

## FAQ for Developers

**Q: Can I deploy existing Solidity contracts without modification?**
A: Yes, most contracts work as-is. Complex contracts using CREATE2 or delegatecall may need adjustments.

**Q: How do I call a Wasm contract from Solidity?**
A: Use the standard `call()` opcode. The EVM adapter auto-detects Wasm targets and translates the call.

**Q: Is Metamask supported?**
A: Yes, full Metamask support via JSON-RPC compatibility layer.

**Q: What's the gas cost difference?**
A: EVM contracts cost ~1.5-2x more gas than equivalent Wasm contracts. Still cheaper than Ethereum mainnet.

**Q: Can Wasm contracts call EVM contracts?**
A: Technically yes, but not recommended. Design pattern is: External EVM → ĒTRID Wasm (one-way).

---

**Status:** Architecture defined, ready for implementation
**Estimated Effort:** 6-8 weeks (1 senior Rust developer)
**Priority:** Medium (nice-to-have for external integration)
**Dependencies:** ETWasm VM, pallet-contracts, ink! contracts
