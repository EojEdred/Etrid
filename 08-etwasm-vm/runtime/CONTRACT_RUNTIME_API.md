# ETWasm Contract Runtime API Documentation

**Component:** 08-etwasm-vm/runtime
**Version:** 2.0.0
**Last Updated:** October 30, 2025
**Status:** Production Ready

---

## Table of Contents

1. [Overview](#overview)
2. [Storage System](#storage-system)
3. [Cross-Contract Calls](#cross-contract-calls)
4. [Event System](#event-system)
5. [Contract Lifecycle](#contract-lifecycle)
6. [Usage Examples](#usage-examples)
7. [API Reference](#api-reference)
8. [Best Practices](#best-practices)

---

## Overview

The ETWasm Contract Runtime provides a complete execution environment for smart contracts with:

- **Advanced Storage**: Persistent key-value storage with merkle proofs and rent mechanism
- **Cross-Contract Calls**: Full support for CALL, DELEGATECALL, STATICCALL, CREATE, CREATE2
- **Event System**: EVM-compatible event emission with indexing and bloom filters
- **Lifecycle Management**: Deploy, upgrade, pause, resume, and destroy contracts

All features include comprehensive reentrancy protection and gas accounting.

---

## Storage System

### Overview

The advanced storage system provides:
- Persistent key-value storage (256-bit keys and values)
- Merkle tree commitments for proof generation
- Storage rent mechanism to prevent state bloat
- Cold/warm access tracking (EIP-2929)
- Storage deposits for anti-spam

### Key Types

```rust
// Storage key and value types
pub type StorageKey = H256;
pub type StorageValue = H256;

// Storage entry with metadata
pub struct StorageEntry {
    pub value: StorageValue,
    pub original_value: StorageValue,  // For gas refunds
    pub deposit: u128,                  // Deposit paid
    pub last_rent_block: u64,           // Last rent payment
    pub access_count: u32,              // Transaction access count
}

// Storage commitment (merkle root)
pub struct StorageCommitment {
    pub root: H256,                     // Merkle root hash
    pub slot_count: u64,                // Number of storage slots
    pub total_size: u64,                // Total size in bytes
    pub block_number: u64,              // Block when created
}
```

### Core Trait

```rust
pub trait AdvancedStorage {
    // Basic operations
    fn read(&self, key: &StorageKey) -> Option<StorageValue>;
    fn write(&mut self, key: StorageKey, value: StorageValue);

    // Access tracking (for EIP-2929 gas optimization)
    fn read_with_access(&mut self, key: &StorageKey)
        -> (Option<StorageValue>, StorageAccessMode);

    // Write with deposit requirement
    fn write_with_deposit(
        &mut self,
        key: StorageKey,
        value: StorageValue,
        deposit: u128
    ) -> Result<(), StorageError>;

    // Delete and recover deposit
    fn delete(&mut self, key: &StorageKey) -> Option<u128>;

    // Merkle proofs
    fn get_commitment(&self) -> StorageCommitment;
    fn generate_proof(&self, key: &StorageKey) -> Option<StorageMerkleProof>;

    // Rent system
    fn calculate_total_rent(&self, current_block: u64) -> u128;
    fn pay_rent(&mut self, current_block: u64) -> Result<u128, StorageError>;
}
```

### Implementation

```rust
// In-memory storage with full features
pub struct AdvancedInMemoryStorage {
    entries: BTreeMap<StorageKey, StorageEntry>,
    accessed_keys: BTreeMap<StorageKey, u32>,
    current_block: u64,
    rent_threshold: u64,
}

impl AdvancedInMemoryStorage {
    pub fn new(current_block: u64) -> Self;
    pub fn set_current_block(&mut self, block: u64);
    pub fn clear_access_cache(&mut self);  // Call at tx end
}
```

### Usage Example

```rust
use etwasm_runtime::*;

// Create storage
let mut storage = AdvancedInMemoryStorage::new(100);

// Write with deposit
let key = H256::from_low_u64_be(1);
let value = H256::from_low_u64_be(42);
storage.write_with_deposit(key, value, MINIMUM_STORAGE_DEPOSIT)?;

// Read with access tracking
let (value, access_mode) = storage.read_with_access(&key);
assert_eq!(access_mode, StorageAccessMode::Cold);  // First access

let (value, access_mode) = storage.read_with_access(&key);
assert_eq!(access_mode, StorageAccessMode::Warm);  // Subsequent access

// Get merkle commitment
let commitment = storage.get_commitment();
println!("Storage root: {:?}", commitment.root);

// Generate proof
let proof = storage.generate_proof(&key)?;
assert!(proof.verify());

// Handle rent
storage.set_current_block(200);
let rent = storage.calculate_total_rent(200);
storage.pay_rent(200)?;

// Clear access cache at transaction end
storage.clear_access_cache();
```

### Storage Rent

**Economics:**
- Rent: 1 VMw per byte per block
- Minimum deposit: 1,000,000 wei (1 ETR)
- Default rent threshold: 100 blocks

**Formulas:**
```rust
// Calculate rent owed
rent = (blocks_elapsed * storage_size * STORAGE_RENT_PER_BYTE_PER_BLOCK)

// For 32-byte slot over 100 blocks
rent = 100 * 32 * 1 = 3,200 VMw
```

---

## Cross-Contract Calls

### Overview

Full support for all EVM call types:
- **CALL**: Regular call with value transfer
- **DELEGATECALL**: Execute in caller's context (library pattern)
- **STATICCALL**: Read-only call (no state modification)
- **CREATE**: Deploy new contract with nonce-based address
- **CREATE2**: Deploy with deterministic address

All calls include reentrancy protection and proper gas accounting.

### Call Types

```rust
pub enum CallType {
    Call,
    DelegateCall,
    StaticCall,
    Create,
    Create2,
}

// Call parameters
pub struct CallParams {
    pub call_type: CallType,
    pub caller: [u8; 32],
    pub target: Option<[u8; 32]>,
    pub value: u128,
    pub gas_limit: VMw,
    pub input_data: Vec<u8>,
    pub salt: Option<H256>,          // For CREATE2
    pub bytecode: Option<Vec<u8>>,    // For CREATE/CREATE2
}
```

### Call Result

```rust
pub struct CallResult {
    pub success: bool,
    pub gas_used: VMw,
    pub return_data: Vec<u8>,
    pub created_address: Option<[u8; 32]>,  // For CREATE/CREATE2
}
```

### Call Context

```rust
pub struct CallContext {
    pub context: ExecutionContext,
    pub state_lock: StateLock,
    pub is_static: bool,              // Read-only mode
    pub return_data: Vec<u8>,         // Last call's return data
}

impl CallContext {
    pub fn new(context: ExecutionContext) -> Self;
    pub fn can_modify_state(&self) -> bool;
}
```

### Usage Examples

#### Regular CALL

```rust
use etwasm_runtime::*;

let caller = [1u8; 32];
let target = [2u8; 32];

// Create call parameters
let params = CallParams::new_call(
    caller,
    target,
    100,        // value (wei)
    100_000,    // gas limit
    vec![1, 2, 3]  // input data
);

// Execute call
let mut call_context = CallContext::new(ExecutionContext::default());
let result = CallExecutor::execute_call(params, &mut call_context, load_code);

if result.success {
    println!("Call succeeded!");
    println!("Gas used: {}", result.gas_used);
    println!("Return data: {:?}", result.return_data);
} else {
    println!("Call failed!");
}
```

#### DELEGATECALL (Library Pattern)

```rust
// Execute target's code in caller's context
let params = CallParams::new_delegatecall(
    caller,
    library_address,
    100_000,
    input_data
);

let result = CallExecutor::execute_call(params, &mut call_context, load_code);
// Executes library code, but uses caller's storage and balance
```

#### STATICCALL (Read-Only)

```rust
// Read-only call (no state modification allowed)
let params = CallParams::new_staticcall(
    caller,
    target,
    100_000,
    input_data
);

let result = CallExecutor::execute_call(params, &mut call_context, load_code);
// Any state modification will fail
```

#### CREATE (Deploy Contract)

```rust
let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];  // Contract bytecode

let params = CallParams::new_create(
    deployer,
    1000,       // initial balance
    100_000,    // gas limit
    bytecode
);

let result = CallExecutor::execute_call(params, &mut call_context, load_code);

if result.success {
    let new_address = result.created_address.unwrap();
    println!("Contract deployed at: {:?}", new_address);
}
```

#### CREATE2 (Deterministic Deployment)

```rust
let salt = H256::from_low_u64_be(42);

let params = CallParams::new_create2(
    deployer,
    0,
    100_000,
    bytecode.clone(),
    salt
);

let result = CallExecutor::execute_call(params, &mut call_context, load_code);
// Address is deterministic based on deployer, bytecode, and salt
```

---

## Event System

### Overview

EVM-compatible event system with:
- LOG0-LOG4 support (0-4 indexed topics)
- Efficient indexing by address and topic
- Bloom filters for fast existence checks
- Block-range queries with filters
- Event pruning for state management

### Event Types

```rust
// Event log entry
pub struct EventLog {
    pub address: [u8; 32],           // Contract address
    pub topics: Vec<EventTopic>,     // Indexed parameters (max 4)
    pub data: Vec<u8>,               // Non-indexed parameters
    pub block_number: u64,
    pub transaction_index: u32,
    pub log_index: u32,
}

// Event filter for queries
pub struct EventFilter {
    pub addresses: Vec<[u8; 32]>,    // Filter by addresses
    pub topics: Vec<Option<EventTopic>>,  // Filter by topics (None = any)
    pub from_block: u64,
    pub to_block: u64,
    pub limit: Option<u32>,
}
```

### Event Store

```rust
pub struct EventStore {
    // Implementation details hidden
}

impl EventStore {
    pub fn new() -> Self;

    // Add event
    pub fn add_event(&mut self, event: EventLog) -> Result<(), EventError>;

    // Query events
    pub fn query_events(&self, filter: &EventFilter) -> Vec<EventLog>;
    pub fn get_events_by_address(&self, address: &[u8; 32]) -> Vec<EventLog>;
    pub fn get_events_by_topic(&self, topic: &EventTopic) -> Vec<EventLog>;

    // Statistics
    pub fn event_count(&self) -> u64;

    // Pruning
    pub fn prune_before_block(&mut self, block_number: u64);
}
```

### Usage Examples

#### Emitting Events

```rust
use etwasm_runtime::*;

let mut event_store = EventStore::new();

// Create event (like Solidity: event Transfer(address indexed from, address indexed to, uint256 value))
let contract_address = [1u8; 32];
let topic_transfer = H256::from_low_u64_be(0xddf252ad);  // keccak256("Transfer(address,address,uint256)")
let topic_from = H256::from_slice(&from_address);
let topic_to = H256::from_slice(&to_address);

let event = EventLog::new(
    contract_address,
    vec![topic_transfer, topic_from, topic_to],  // Indexed parameters
    amount.to_be_bytes().to_vec(),                // Non-indexed data
    block_number,
    tx_index,
    log_index
)?;

event_store.add_event(event)?;
```

#### Querying Events

```rust
// Query all Transfer events from a specific contract
let events = event_store.get_events_by_address(&contract_address);

// Query with filter
let filter = EventFilter::new(100, 200)        // Block range
    .with_address(contract_address)             // From specific contract
    .with_topic(Some(topic_transfer))           // Transfer events only
    .with_limit(100);                           // Max 100 results

let events = event_store.query_events(&filter);

for event in events {
    println!("Block {}: {:?}", event.block_number, event.data);
}
```

#### Bloom Filter Optimization

```rust
// Get bloom filter for fast existence checks
if let Some(bloom) = event_store.get_block_bloom(block_number) {
    // Quick check before expensive query
    if bloom.contains(&contract_address) {
        // Contract emitted events in this block
        let events = event_store.get_events_by_address(&contract_address);
    }
}
```

#### Event Pruning

```rust
// Prune old events to save space
event_store.prune_before_block(1000);
// Events before block 1000 are removed
```

---

## Contract Lifecycle

### Overview

Complete contract lifecycle management:
- **Deploy**: CREATE and CREATE2 with nonce or deterministic addresses
- **Upgrade**: Change contract code with storage migration
- **Pause/Resume**: Temporarily disable contract execution
- **Destroy**: SELFDESTRUCT with balance transfer
- **Ownership**: Transfer contract ownership

### Contract States

```rust
pub enum ContractState {
    Active,             // Normal operation
    Paused,             // Execution disabled
    PendingDestruction, // Marked for deletion
    Destroyed,          // Deleted
    Upgrading,          // During upgrade
}
```

### Contract Metadata

```rust
pub struct ContractMetadata {
    pub owner: [u8; 32],
    pub created_at_block: u64,
    pub created_at_timestamp: u64,
    pub code_hash: H256,
    pub state: ContractState,
    pub version: u32,              // Increments with upgrades
    pub call_count: u64,
    pub last_interaction_block: u64,
    pub balance: u128,
    pub is_upgradeable: bool,
    pub previous_code_hash: Option<H256>,  // For rollback
}
```

### Lifecycle Manager

```rust
pub struct ContractLifecycleManager;

impl ContractLifecycleManager {
    pub fn new() -> Self;

    // Deployment
    pub fn deploy_contract(
        &mut self,
        params: DeploymentParams,
        block_number: u64,
        timestamp: u64
    ) -> DeploymentResult;

    // Upgrades
    pub fn upgrade_contract(
        &mut self,
        params: UpgradeParams,
        upgrader: [u8; 32],
        block_number: u64
    ) -> Result<VMw, LifecycleError>;

    pub fn rollback_upgrade(
        &mut self,
        contract_address: [u8; 32],
        caller: [u8; 32]
    ) -> Result<(), LifecycleError>;

    // State management
    pub fn pause_contract(&mut self, address: [u8; 32], caller: [u8; 32])
        -> Result<(), LifecycleError>;
    pub fn resume_contract(&mut self, address: [u8; 32], caller: [u8; 32])
        -> Result<(), LifecycleError>;

    // Destruction
    pub fn destroy_contract(
        &mut self,
        params: DestructionParams,
        caller: [u8; 32]
    ) -> Result<u128, LifecycleError>;

    // Ownership
    pub fn transfer_ownership(
        &mut self,
        address: [u8; 32],
        old_owner: [u8; 32],
        new_owner: [u8; 32]
    ) -> Result<(), LifecycleError>;

    // Queries
    pub fn get_contract(&self, address: &[u8; 32]) -> Option<&ContractMetadata>;
    pub fn get_bytecode(&self, address: &[u8; 32]) -> Option<&Vec<u8>>;
}
```

### Usage Examples

#### Deploy Contract

```rust
use etwasm_runtime::*;

let mut manager = ContractLifecycleManager::new();
let deployer = [1u8; 32];
let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];

// Create deployment parameters
let params = DeploymentParams::new(deployer, bytecode, 100_000)
    .with_upgradeable()                      // Make it upgradeable
    .with_initial_balance(1000)              // Initial balance
    .with_constructor_args(vec![1, 2, 3]);   // Constructor arguments

// Deploy
let result = manager.deploy_contract(params, block_number, timestamp);

if result.success {
    let address = result.contract_address.unwrap();
    println!("Contract deployed at: {:?}", address);
    println!("Gas used: {}", result.gas_used);
}
```

#### Deploy with CREATE2 (Deterministic)

```rust
let salt = H256::from_low_u64_be(42);

let params = DeploymentParams::new(deployer, bytecode, 100_000)
    .with_salt(salt);  // Deterministic address

let result = manager.deploy_contract(params, block_number, timestamp);
// Address is computed from: keccak256(0xff ++ deployer ++ salt ++ keccak256(bytecode))
```

#### Upgrade Contract

```rust
let new_bytecode = vec![0x60, 0x01, 0x60, 0x00, 0xf3];

let upgrade_params = UpgradeParams {
    contract_address,
    new_bytecode,
    migration_script: Some(migration_code),  // Optional storage migration
    gas_limit: 200_000,
};

let gas_used = manager.upgrade_contract(upgrade_params, owner, block_number)?;

// Contract is now at version 2
let metadata = manager.get_contract(&contract_address).unwrap();
assert_eq!(metadata.version, 2);
```

#### Rollback Upgrade

```rust
// Rollback to previous version
manager.rollback_upgrade(contract_address, owner)?;

let metadata = manager.get_contract(&contract_address).unwrap();
assert_eq!(metadata.version, 1);  // Back to version 1
```

#### Pause and Resume

```rust
// Pause contract (no execution allowed)
manager.pause_contract(contract_address, owner)?;

let metadata = manager.get_contract(&contract_address).unwrap();
assert!(!metadata.is_callable());

// Resume contract
manager.resume_contract(contract_address, owner)?;

let metadata = manager.get_contract(&contract_address).unwrap();
assert!(metadata.is_callable());
```

#### Destroy Contract

```rust
let beneficiary = [2u8; 32];

let params = DestructionParams {
    contract_address,
    beneficiary,
};

// Destroy and transfer balance
let balance = manager.destroy_contract(params, owner)?;
println!("Transferred {} wei to beneficiary", balance);

// Contract is now destroyed
let metadata = manager.get_contract(&contract_address).unwrap();
assert!(metadata.is_destroyed());
```

---

## Usage Examples

### Complete Contract Workflow

```rust
use etwasm_runtime::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize components
    let mut manager = ContractLifecycleManager::new();
    let mut storage = AdvancedInMemoryStorage::new(100);
    let mut event_store = EventStore::new();

    let owner = [1u8; 32];
    let bytecode = compile_contract("MyContract.sol")?;

    // 2. Deploy contract
    let params = DeploymentParams::new(owner, bytecode, 500_000)
        .with_upgradeable()
        .with_initial_balance(10_000);

    let result = manager.deploy_contract(params, 100, timestamp);
    let contract_address = result.contract_address.unwrap();

    // 3. Emit deployment event
    let event = EventLog::new(
        contract_address,
        vec![H256::from_low_u64_be(0xDEAD)],  // Deployment topic
        owner.to_vec(),
        100,
        0,
        0
    )?;
    event_store.add_event(event)?;

    // 4. Use storage
    let key = H256::from_low_u64_be(1);
    let value = H256::from_low_u64_be(42);
    storage.write_with_deposit(key, value, MINIMUM_STORAGE_DEPOSIT)?;

    // 5. Make cross-contract call
    let target = [2u8; 32];
    let params = CallParams::new_call(contract_address, target, 0, 100_000, vec![]);
    let mut call_context = CallContext::new(ExecutionContext::default());
    let call_result = CallExecutor::execute_call(params, &mut call_context, load_code);

    // 6. Emit event for call
    let event = EventLog::new(
        contract_address,
        vec![H256::from_low_u64_be(0xCALL)],
        call_result.return_data,
        101,
        0,
        0
    )?;
    event_store.add_event(event)?;

    // 7. Upgrade contract
    let new_bytecode = compile_contract("MyContractV2.sol")?;
    let upgrade_params = UpgradeParams {
        contract_address,
        new_bytecode,
        migration_script: None,
        gas_limit: 300_000,
    };
    manager.upgrade_contract(upgrade_params, owner, 102)?;

    // 8. Query events
    let filter = EventFilter::new(100, 102)
        .with_address(contract_address);
    let events = event_store.query_events(&filter);

    println!("Contract has {} events", events.len());

    // 9. Handle storage rent
    storage.set_current_block(200);
    let rent = storage.calculate_total_rent(200);
    storage.pay_rent(200)?;

    Ok(())
}
```

---

## API Reference

### Storage Module (`storage.rs`)

| Item | Type | Description |
|------|------|-------------|
| `AdvancedStorage` | Trait | Storage operations interface |
| `AdvancedInMemoryStorage` | Struct | In-memory implementation |
| `StorageEntry` | Struct | Storage slot with metadata |
| `StorageCommitment` | Struct | Merkle root commitment |
| `StorageMerkleProof` | Struct | Merkle inclusion proof |
| `StorageAccessMode` | Enum | Cold/Warm access tracking |
| `MINIMUM_STORAGE_DEPOSIT` | Const | 1,000,000 wei |

### Calls Module (`calls.rs`)

| Item | Type | Description |
|------|------|-------------|
| `CallExecutor` | Struct | Cross-contract call executor |
| `CallParams` | Struct | Call parameters |
| `CallResult` | Struct | Call execution result |
| `CallContext` | Struct | Call tracking context |
| `CallType` | Enum | CALL/DELEGATECALL/STATICCALL/CREATE/CREATE2 |

### Events Module (`events.rs`)

| Item | Type | Description |
|------|------|-------------|
| `EventStore` | Struct | Event storage and indexing |
| `EventLog` | Struct | Single event log entry |
| `EventFilter` | Struct | Event query filter |
| `EventBloomFilter` | Struct | Bloom filter for fast checks |
| `MAX_TOPICS` | Const | 4 (EVM standard) |

### Lifecycle Module (`lifecycle.rs`)

| Item | Type | Description |
|------|------|-------------|
| `ContractLifecycleManager` | Struct | Lifecycle management |
| `ContractMetadata` | Struct | Contract metadata |
| `DeploymentParams` | Struct | Deployment parameters |
| `UpgradeParams` | Struct | Upgrade parameters |
| `ContractState` | Enum | Active/Paused/Destroyed/etc. |

---

## Best Practices

### Storage

1. **Always use deposits**: Prevents storage spam
   ```rust
   storage.write_with_deposit(key, value, MINIMUM_STORAGE_DEPOSIT)?;
   ```

2. **Track access for gas optimization**: Use `read_with_access` for EIP-2929 benefits
   ```rust
   let (value, mode) = storage.read_with_access(&key);
   let gas_cost = match mode {
       Cold => 2100,
       Warm => 100,
   };
   ```

3. **Clear access cache**: Call at end of each transaction
   ```rust
   storage.clear_access_cache();
   ```

4. **Pay rent periodically**: Prevent deposit exhaustion
   ```rust
   if block % 100 == 0 {
       storage.pay_rent(block)?;
   }
   ```

### Cross-Contract Calls

1. **Check reentrancy**: Always handled automatically, but be aware
   ```rust
   // Reentrancy is automatically detected and blocked
   let result = CallExecutor::execute_call(params, &mut context, load_code);
   ```

2. **Use STATICCALL for read-only**: Prevents accidental state modification
   ```rust
   let params = CallParams::new_staticcall(caller, target, gas, data);
   ```

3. **Handle call failures**: Check result.success
   ```rust
   if !result.success {
       // Handle failure, maybe revert or log
   }
   ```

### Events

1. **Index important parameters**: Max 4 topics for efficient querying
   ```rust
   // Index: from, to (for fast filtering)
   // Data: amount (non-indexed)
   EventLog::new(addr, vec![topic_from, topic_to], amount_bytes, ...)
   ```

2. **Prune old events**: Save storage space
   ```rust
   event_store.prune_before_block(current_block - 10000);
   ```

3. **Use bloom filters**: Fast existence checks before expensive queries
   ```rust
   if bloom.contains(&address) {
       let events = store.get_events_by_address(&address);
   }
   ```

### Lifecycle

1. **Make contracts upgradeable from start**: Can't be changed later
   ```rust
   let params = DeploymentParams::new(owner, bytecode, gas)
       .with_upgradeable();
   ```

2. **Test upgrades thoroughly**: Use rollback if needed
   ```rust
   manager.upgrade_contract(upgrade_params, owner, block)?;
   // If issues detected:
   manager.rollback_upgrade(address, owner)?;
   ```

3. **Use pause for emergencies**: Quick response to issues
   ```rust
   manager.pause_contract(address, owner)?;
   // Fix issue, then resume
   manager.resume_contract(address, owner)?;
   ```

---

**ETWasm Contract Runtime**
Version 2.0.0 | Etrid Blockchain
Last Updated: October 30, 2025
