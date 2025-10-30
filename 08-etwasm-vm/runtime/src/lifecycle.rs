//! Contract Lifecycle Management
//!
//! This module handles the complete lifecycle of smart contracts:
//! - Deployment (CREATE, CREATE2)
//! - Upgrades (proxy patterns, storage migration)
//! - Destruction (SELFDESTRUCT)
//! - Metadata tracking (owner, version, code hash)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_io::hashing::blake2_256;
use sp_std::prelude::*;
use etwasm_gas_metering::VMw;

/// Contract state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum ContractState {
    /// Contract is active and can be called
    Active,
    /// Contract is paused (no execution allowed)
    Paused,
    /// Contract is marked for destruction
    PendingDestruction,
    /// Contract has been destroyed
    Destroyed,
    /// Contract is being upgraded
    Upgrading,
}

/// Contract metadata
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ContractMetadata {
    /// Contract owner address
    pub owner: [u8; 32],
    /// Contract creation block
    pub created_at_block: u64,
    /// Contract creation timestamp
    pub created_at_timestamp: u64,
    /// Code hash
    pub code_hash: H256,
    /// Contract state
    pub state: ContractState,
    /// Version number (for upgrades)
    pub version: u32,
    /// Total number of calls
    pub call_count: u64,
    /// Last interaction block
    pub last_interaction_block: u64,
    /// Contract balance (in wei)
    pub balance: u128,
    /// Whether the contract is upgradeable
    pub is_upgradeable: bool,
    /// Previous code hash (for rollback)
    pub previous_code_hash: Option<H256>,
}

impl ContractMetadata {
    /// Create new contract metadata
    pub fn new(
        owner: [u8; 32],
        code_hash: H256,
        block_number: u64,
        timestamp: u64,
        is_upgradeable: bool,
    ) -> Self {
        Self {
            owner,
            created_at_block: block_number,
            created_at_timestamp: timestamp,
            code_hash,
            state: ContractState::Active,
            version: 1,
            call_count: 0,
            last_interaction_block: block_number,
            balance: 0,
            is_upgradeable,
            previous_code_hash: None,
        }
    }

    /// Check if contract can be called
    pub fn is_callable(&self) -> bool {
        matches!(self.state, ContractState::Active)
    }

    /// Check if contract is destroyed
    pub fn is_destroyed(&self) -> bool {
        matches!(self.state, ContractState::Destroyed)
    }

    /// Record a call to the contract
    pub fn record_call(&mut self, block_number: u64) {
        self.call_count += 1;
        self.last_interaction_block = block_number;
    }

    /// Pause the contract
    pub fn pause(&mut self) {
        self.state = ContractState::Paused;
    }

    /// Resume the contract
    pub fn resume(&mut self) {
        if matches!(self.state, ContractState::Paused) {
            self.state = ContractState::Active;
        }
    }
}

/// Contract deployment parameters
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct DeploymentParams {
    /// Deployer address
    pub deployer: [u8; 32],
    /// Contract bytecode
    pub bytecode: Vec<u8>,
    /// Constructor arguments
    pub constructor_args: Vec<u8>,
    /// Initial balance
    pub initial_balance: u128,
    /// Gas limit
    pub gas_limit: VMw,
    /// Whether contract is upgradeable
    pub is_upgradeable: bool,
    /// Salt for CREATE2 (deterministic deployment)
    pub salt: Option<H256>,
}

impl DeploymentParams {
    /// Create deployment parameters
    pub fn new(
        deployer: [u8; 32],
        bytecode: Vec<u8>,
        gas_limit: VMw,
    ) -> Self {
        Self {
            deployer,
            bytecode,
            constructor_args: Vec::new(),
            initial_balance: 0,
            gas_limit,
            is_upgradeable: false,
            salt: None,
        }
    }

    /// Make the contract upgradeable
    pub fn with_upgradeable(mut self) -> Self {
        self.is_upgradeable = true;
        self
    }

    /// Add constructor arguments
    pub fn with_constructor_args(mut self, args: Vec<u8>) -> Self {
        self.constructor_args = args;
        self
    }

    /// Add initial balance
    pub fn with_initial_balance(mut self, balance: u128) -> Self {
        self.initial_balance = balance;
        self
    }

    /// Add salt for CREATE2
    pub fn with_salt(mut self, salt: H256) -> Self {
        self.salt = Some(salt);
        self
    }
}

/// Contract upgrade parameters
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct UpgradeParams {
    /// Contract address to upgrade
    pub contract_address: [u8; 32],
    /// New bytecode
    pub new_bytecode: Vec<u8>,
    /// Storage migration script (optional)
    pub migration_script: Option<Vec<u8>>,
    /// Gas limit for upgrade
    pub gas_limit: VMw,
}

/// Destruction parameters
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct DestructionParams {
    /// Contract address to destroy
    pub contract_address: [u8; 32],
    /// Beneficiary address (receives remaining balance)
    pub beneficiary: [u8; 32],
}

/// Deployment result
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct DeploymentResult {
    /// Whether deployment succeeded
    pub success: bool,
    /// Deployed contract address
    pub contract_address: Option<[u8; 32]>,
    /// Gas used
    pub gas_used: VMw,
    /// Error message (if failed)
    pub error: Option<Vec<u8>>,
}

impl DeploymentResult {
    /// Create successful deployment result
    pub fn success(address: [u8; 32], gas_used: VMw) -> Self {
        Self {
            success: true,
            contract_address: Some(address),
            gas_used,
            error: None,
        }
    }

    /// Create failed deployment result
    pub fn failure(gas_used: VMw, error: Vec<u8>) -> Self {
        Self {
            success: false,
            contract_address: None,
            gas_used,
            error: Some(error),
        }
    }
}

/// Contract lifecycle manager
#[derive(Debug, Clone)]
pub struct ContractLifecycleManager {
    /// Contract metadata by address
    contracts: BTreeMap<[u8; 32], ContractMetadata>,
    /// Contract bytecode by code hash
    bytecode_storage: BTreeMap<H256, Vec<u8>>,
    /// Address by code hash (for deduplication)
    code_hash_to_address: BTreeMap<H256, Vec<[u8; 32]>>,
}

impl ContractLifecycleManager {
    /// Create a new lifecycle manager
    pub fn new() -> Self {
        Self {
            contracts: BTreeMap::new(),
            bytecode_storage: BTreeMap::new(),
            code_hash_to_address: BTreeMap::new(),
        }
    }

    /// Deploy a new contract
    pub fn deploy_contract(
        &mut self,
        params: DeploymentParams,
        block_number: u64,
        timestamp: u64,
    ) -> DeploymentResult {
        // Calculate code hash
        let code_hash = H256::from(blake2_256(&params.bytecode));

        // Generate contract address
        let contract_address = if let Some(salt) = params.salt {
            // CREATE2: deterministic address
            self.generate_create2_address(&params.deployer, &params.bytecode, &salt)
        } else {
            // CREATE: nonce-based address
            self.generate_create_address(&params.deployer, block_number)
        };

        // Check if address already exists
        if self.contracts.contains_key(&contract_address) {
            return DeploymentResult::failure(0, b"Address already exists".to_vec());
        }

        // Store bytecode (deduplicated by hash)
        if !self.bytecode_storage.contains_key(&code_hash) {
            self.bytecode_storage.insert(code_hash, params.bytecode.clone());
        }

        // Create metadata
        let metadata = ContractMetadata::new(
            params.deployer,
            code_hash,
            block_number,
            timestamp,
            params.is_upgradeable,
        );

        // Store contract
        self.contracts.insert(contract_address, metadata);

        // Update code hash index
        self.code_hash_to_address
            .entry(code_hash)
            .or_insert_with(Vec::new)
            .push(contract_address);

        // Estimate gas (simplified)
        let gas_used = self.estimate_deployment_gas(&params.bytecode);

        DeploymentResult::success(contract_address, gas_used)
    }

    /// Upgrade a contract
    pub fn upgrade_contract(
        &mut self,
        params: UpgradeParams,
        upgrader: [u8; 32],
        block_number: u64,
    ) -> Result<VMw, LifecycleError> {
        // Get contract metadata
        let metadata = self.contracts
            .get_mut(&params.contract_address)
            .ok_or(LifecycleError::ContractNotFound)?;

        // Check if upgradeable
        if !metadata.is_upgradeable {
            return Err(LifecycleError::NotUpgradeable);
        }

        // Check permissions
        if metadata.owner != upgrader {
            return Err(LifecycleError::Unauthorized);
        }

        // Check state
        if !matches!(metadata.state, ContractState::Active | ContractState::Paused) {
            return Err(LifecycleError::InvalidState);
        }

        // Mark as upgrading
        metadata.state = ContractState::Upgrading;

        // Calculate new code hash
        let new_code_hash = H256::from(blake2_256(&params.new_bytecode));

        // Save previous code hash for rollback
        metadata.previous_code_hash = Some(metadata.code_hash);

        // Store new bytecode
        if !self.bytecode_storage.contains_key(&new_code_hash) {
            self.bytecode_storage.insert(new_code_hash, params.new_bytecode.clone());
        }

        // Update metadata
        metadata.code_hash = new_code_hash;
        metadata.version += 1;
        metadata.last_interaction_block = block_number;

        // Execute migration script if provided
        if let Some(_migration) = params.migration_script {
            // In real implementation, execute migration
            // For now, just acknowledge it
        }

        // Mark as active again
        metadata.state = ContractState::Active;

        // Update code hash index
        self.code_hash_to_address
            .entry(new_code_hash)
            .or_insert_with(Vec::new)
            .push(params.contract_address);

        // Estimate gas
        let gas_used = self.estimate_upgrade_gas(&params.new_bytecode);

        Ok(gas_used)
    }

    /// Rollback to previous version
    pub fn rollback_upgrade(
        &mut self,
        contract_address: [u8; 32],
        caller: [u8; 32],
    ) -> Result<(), LifecycleError> {
        let metadata = self.contracts
            .get_mut(&contract_address)
            .ok_or(LifecycleError::ContractNotFound)?;

        // Check permissions
        if metadata.owner != caller {
            return Err(LifecycleError::Unauthorized);
        }

        // Get previous code hash
        let previous_hash = metadata.previous_code_hash
            .ok_or(LifecycleError::NoPreviousVersion)?;

        // Rollback
        metadata.code_hash = previous_hash;
        metadata.version = metadata.version.saturating_sub(1);
        metadata.previous_code_hash = None;

        Ok(())
    }

    /// Destroy a contract (SELFDESTRUCT)
    pub fn destroy_contract(
        &mut self,
        params: DestructionParams,
        caller: [u8; 32],
    ) -> Result<u128, LifecycleError> {
        let metadata = self.contracts
            .get_mut(&params.contract_address)
            .ok_or(LifecycleError::ContractNotFound)?;

        // Only owner can destroy
        if metadata.owner != caller {
            return Err(LifecycleError::Unauthorized);
        }

        // Cannot destroy if already destroyed
        if metadata.is_destroyed() {
            return Err(LifecycleError::AlreadyDestroyed);
        }

        // Mark as pending destruction
        metadata.state = ContractState::PendingDestruction;

        // Get balance to transfer
        let balance = metadata.balance;
        metadata.balance = 0;

        // Mark as destroyed
        metadata.state = ContractState::Destroyed;

        // In real implementation, transfer balance to beneficiary
        // For now, just return the balance

        Ok(balance)
    }

    /// Get contract metadata
    pub fn get_contract(&self, address: &[u8; 32]) -> Option<&ContractMetadata> {
        self.contracts.get(address)
    }

    /// Get contract bytecode
    pub fn get_bytecode(&self, address: &[u8; 32]) -> Option<&Vec<u8>> {
        let metadata = self.contracts.get(address)?;
        self.bytecode_storage.get(&metadata.code_hash)
    }

    /// Get contracts by code hash (finds all instances of same code)
    pub fn get_contracts_by_code_hash(&self, code_hash: &H256) -> Vec<[u8; 32]> {
        self.code_hash_to_address
            .get(code_hash)
            .cloned()
            .unwrap_or_default()
    }

    /// Pause a contract
    pub fn pause_contract(
        &mut self,
        address: [u8; 32],
        caller: [u8; 32],
    ) -> Result<(), LifecycleError> {
        let metadata = self.contracts
            .get_mut(&address)
            .ok_or(LifecycleError::ContractNotFound)?;

        if metadata.owner != caller {
            return Err(LifecycleError::Unauthorized);
        }

        metadata.pause();
        Ok(())
    }

    /// Resume a contract
    pub fn resume_contract(
        &mut self,
        address: [u8; 32],
        caller: [u8; 32],
    ) -> Result<(), LifecycleError> {
        let metadata = self.contracts
            .get_mut(&address)
            .ok_or(LifecycleError::ContractNotFound)?;

        if metadata.owner != caller {
            return Err(LifecycleError::Unauthorized);
        }

        metadata.resume();
        Ok(())
    }

    /// Transfer ownership
    pub fn transfer_ownership(
        &mut self,
        address: [u8; 32],
        old_owner: [u8; 32],
        new_owner: [u8; 32],
    ) -> Result<(), LifecycleError> {
        let metadata = self.contracts
            .get_mut(&address)
            .ok_or(LifecycleError::ContractNotFound)?;

        if metadata.owner != old_owner {
            return Err(LifecycleError::Unauthorized);
        }

        metadata.owner = new_owner;
        Ok(())
    }

    /// Generate CREATE address
    fn generate_create_address(&self, creator: &[u8; 32], nonce: u64) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(creator);
        data.extend_from_slice(&nonce.to_le_bytes());
        let hash = H256::from(blake2_256(&data));
        let mut address = [0u8; 32];
        address.copy_from_slice(hash.as_bytes());
        address
    }

    /// Generate CREATE2 address
    fn generate_create2_address(&self, creator: &[u8; 32], bytecode: &[u8], salt: &H256) -> [u8; 32] {
        let mut data = Vec::new();
        data.push(0xff);
        data.extend_from_slice(creator);
        data.extend_from_slice(salt.as_bytes());
        data.extend_from_slice(&H256::from(blake2_256(bytecode)).as_bytes());
        let hash = H256::from(blake2_256(&data));
        let mut address = [0u8; 32];
        address.copy_from_slice(hash.as_bytes());
        address
    }

    /// Estimate deployment gas
    fn estimate_deployment_gas(&self, bytecode: &[u8]) -> VMw {
        32000 + (bytecode.len() as VMw * 200)
    }

    /// Estimate upgrade gas
    fn estimate_upgrade_gas(&self, bytecode: &[u8]) -> VMw {
        20000 + (bytecode.len() as VMw * 100)
    }
}

impl Default for ContractLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Lifecycle errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleError {
    /// Contract not found
    ContractNotFound,
    /// Not authorized to perform operation
    Unauthorized,
    /// Contract is not upgradeable
    NotUpgradeable,
    /// Invalid contract state for operation
    InvalidState,
    /// Contract already destroyed
    AlreadyDestroyed,
    /// No previous version to rollback to
    NoPreviousVersion,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_deployment() {
        let mut manager = ContractLifecycleManager::new();
        let deployer = [1u8; 32];
        let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];

        let params = DeploymentParams::new(deployer, bytecode, 100000);
        let result = manager.deploy_contract(params, 100, 1000);

        assert!(result.success);
        assert!(result.contract_address.is_some());

        let address = result.contract_address.unwrap();
        let metadata = manager.get_contract(&address).unwrap();
        assert_eq!(metadata.owner, deployer);
        assert_eq!(metadata.version, 1);
        assert!(metadata.is_callable());
    }

    #[test]
    fn test_deterministic_deployment() {
        let mut manager = ContractLifecycleManager::new();
        let deployer = [1u8; 32];
        let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xf3];
        let salt = H256::from_low_u64_be(42);

        let params1 = DeploymentParams::new(deployer, bytecode.clone(), 100000)
            .with_salt(salt);
        let result1 = manager.deploy_contract(params1, 100, 1000);

        let params2 = DeploymentParams::new(deployer, bytecode, 100000)
            .with_salt(salt);
        let result2 = manager.deploy_contract(params2, 101, 1001);

        // Second deployment should fail (address collision)
        assert!(!result2.success);
    }

    #[test]
    fn test_contract_upgrade() {
        let mut manager = ContractLifecycleManager::new();
        let deployer = [1u8; 32];
        let bytecode = vec![0x60, 0x00];

        let params = DeploymentParams::new(deployer, bytecode, 100000)
            .with_upgradeable();
        let result = manager.deploy_contract(params, 100, 1000);

        let address = result.contract_address.unwrap();

        // Upgrade
        let new_bytecode = vec![0x60, 0x01];
        let upgrade_params = UpgradeParams {
            contract_address: address,
            new_bytecode,
            migration_script: None,
            gas_limit: 100000,
        };

        let gas_used = manager.upgrade_contract(upgrade_params, deployer, 101);
        assert!(gas_used.is_ok());

        let metadata = manager.get_contract(&address).unwrap();
        assert_eq!(metadata.version, 2);
        assert!(metadata.previous_code_hash.is_some());
    }

    #[test]
    fn test_upgrade_rollback() {
        let mut manager = ContractLifecycleManager::new();
        let deployer = [1u8; 32];
        let original_bytecode = vec![0x60, 0x00];

        let params = DeploymentParams::new(deployer, original_bytecode.clone(), 100000)
            .with_upgradeable();
        let result = manager.deploy_contract(params, 100, 1000);
        let address = result.contract_address.unwrap();

        let original_code_hash = H256::from(blake2_256(&original_bytecode));

        // Upgrade
        let upgrade_params = UpgradeParams {
            contract_address: address,
            new_bytecode: vec![0x60, 0x01],
            migration_script: None,
            gas_limit: 100000,
        };
        manager.upgrade_contract(upgrade_params, deployer, 101).unwrap();

        // Rollback
        manager.rollback_upgrade(address, deployer).unwrap();

        let metadata = manager.get_contract(&address).unwrap();
        assert_eq!(metadata.code_hash, original_code_hash);
        assert_eq!(metadata.version, 1);
    }

    #[test]
    fn test_contract_destruction() {
        let mut manager = ContractLifecycleManager::new();
        let deployer = [1u8; 32];
        let beneficiary = [2u8; 32];

        let params = DeploymentParams::new(deployer, vec![0x60, 0x00], 100000);
        let result = manager.deploy_contract(params, 100, 1000);
        let address = result.contract_address.unwrap();

        let destruction_params = DestructionParams {
            contract_address: address,
            beneficiary,
        };

        let balance = manager.destroy_contract(destruction_params, deployer);
        assert!(balance.is_ok());

        let metadata = manager.get_contract(&address).unwrap();
        assert!(metadata.is_destroyed());
    }

    #[test]
    fn test_pause_resume() {
        let mut manager = ContractLifecycleManager::new();
        let deployer = [1u8; 32];

        let params = DeploymentParams::new(deployer, vec![0x60, 0x00], 100000);
        let result = manager.deploy_contract(params, 100, 1000);
        let address = result.contract_address.unwrap();

        // Pause
        manager.pause_contract(address, deployer).unwrap();
        let metadata = manager.get_contract(&address).unwrap();
        assert_eq!(metadata.state, ContractState::Paused);
        assert!(!metadata.is_callable());

        // Resume
        manager.resume_contract(address, deployer).unwrap();
        let metadata = manager.get_contract(&address).unwrap();
        assert_eq!(metadata.state, ContractState::Active);
        assert!(metadata.is_callable());
    }

    #[test]
    fn test_ownership_transfer() {
        let mut manager = ContractLifecycleManager::new();
        let old_owner = [1u8; 32];
        let new_owner = [2u8; 32];

        let params = DeploymentParams::new(old_owner, vec![0x60, 0x00], 100000);
        let result = manager.deploy_contract(params, 100, 1000);
        let address = result.contract_address.unwrap();

        manager.transfer_ownership(address, old_owner, new_owner).unwrap();

        let metadata = manager.get_contract(&address).unwrap();
        assert_eq!(metadata.owner, new_owner);
    }

    #[test]
    fn test_unauthorized_operations() {
        let mut manager = ContractLifecycleManager::new();
        let owner = [1u8; 32];
        let attacker = [2u8; 32];

        let params = DeploymentParams::new(owner, vec![0x60, 0x00], 100000)
            .with_upgradeable();
        let result = manager.deploy_contract(params, 100, 1000);
        let address = result.contract_address.unwrap();

        // Try to upgrade as non-owner
        let upgrade_params = UpgradeParams {
            contract_address: address,
            new_bytecode: vec![0x60, 0x01],
            migration_script: None,
            gas_limit: 100000,
        };
        let result = manager.upgrade_contract(upgrade_params, attacker, 101);
        assert_eq!(result, Err(LifecycleError::Unauthorized));

        // Try to pause as non-owner
        let result = manager.pause_contract(address, attacker);
        assert_eq!(result, Err(LifecycleError::Unauthorized));
    }
}
