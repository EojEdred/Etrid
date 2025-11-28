#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub use etrid_economics::{Balance, ONE_ETRID};

/// VMw (Virtual Machine Watts) - non-tradable gas units for computation
pub type VMw = u64;

/// ============================================================================
/// GAS OPERATION COSTS
/// ============================================================================
/// 
/// Each operation on the blockchain consumes a specific amount of VMw (gas).
/// These costs are calibrated to prevent network abuse while remaining affordable.

/// Cost to initialize (deploy) a new smart contract
/// - Includes verification, storage initialization, execution of init code
/// - Approximately: 2000 VMw = 0.002 ÉTR at price 1
pub const VMW_CONTRACT_INIT: VMw = 2_000;

/// Cost to call an existing smart contract
/// - Includes contract lookup, ABI validation, basic execution
/// - Approximately: 500 VMw = 0.0005 ÉTR at price 1
pub const VMW_CONTRACT_CALL: VMw = 500;

/// Cost to read a value from storage
/// - Includes disk I/O, verification, state proof
/// - Approximately: 100 VMw = 0.0001 ÉTR at price 1
pub const VMW_STORAGE_READ: VMw = 100;

/// Cost to write a value to storage
/// - Includes disk I/O, verification, state update
/// - Approximately: 300 VMw = 0.0003 ÉTR at price 1
pub const VMW_STORAGE_WRITE: VMw = 300;

/// Cost to verify state (consensus validation)
/// - Includes signature verification, state proof validation
/// - Approximately: 150 VMw = 0.00015 ÉTR at price 1
pub const VMW_STATE_VERIFY: VMw = 150;

/// Cost to verify/check an address
/// - Includes address validation, DID lookup
/// - Approximately: 50 VMw = 0.00005 ÉTR at price 1
pub const VMW_ADDRESS_CHECK: VMw = 50;

/// ============================================================================
/// BLOCK AND TRANSACTION LIMITS
/// ============================================================================

/// Maximum VMw allowed per block
/// - Allows roughly 5,000 contract calls per block (10M / 2000)
/// - Or 100,000 storage reads per block (10M / 100)
pub const VMW_BLOCK_LIMIT: VMw = 10_000_000;

/// Maximum VMw allowed per transaction
/// - Prevents single transaction from monopolizing block
/// - Allows 2,000 contract calls per transaction
pub const VMW_TX_LIMIT: VMw = 1_000_000;

/// Maximum transactions per block
pub const MAX_TRANSACTIONS_PER_BLOCK: u32 = 1_000;

/// Maximum block size in bytes
pub const MAX_BLOCK_SIZE_BYTES: u32 = 5_000_000; // 5 MB

/// ============================================================================
/// GAS PRICE AND CONVERSION
/// ============================================================================

/// Conversion rate: 1 ÉTR = 1,000,000 VMw
/// This means: Fee (ÉTR) = (VMw_Used × Op_Price) / 1,000,000
pub const WATTS_PER_ETRID: VMw = 1_000_000;

/// Minimum operation price (in units, not ÉTR)
/// Prevents spam by ensuring each operation costs something
pub const MIN_OP_PRICE: u32 = 1;

/// Maximum operation price (prevents fee spike attacks)
pub const MAX_OP_PRICE: u32 = 1000;

/// Default operation price at genesis
pub const DEFAULT_OP_PRICE: u32 = 1;

/// ============================================================================
/// GAS OPERATION TYPE
/// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum GasOperation {
    /// Deploy a new smart contract
    ContractInit,
    /// Call an existing smart contract
    ContractCall,
    /// Read from storage
    StorageRead,
    /// Write to storage
    StorageWrite,
    /// Verify state (consensus)
    StateVerify,
    /// Verify/check an address
    AddressCheck,
}

impl GasOperation {
    /// Get the base VMw cost for this operation
    pub fn base_cost(&self) -> VMw {
        match self {
            GasOperation::ContractInit => VMW_CONTRACT_INIT,
            GasOperation::ContractCall => VMW_CONTRACT_CALL,
            GasOperation::StorageRead => VMW_STORAGE_READ,
            GasOperation::StorageWrite => VMW_STORAGE_WRITE,
            GasOperation::StateVerify => VMW_STATE_VERIFY,
            GasOperation::AddressCheck => VMW_ADDRESS_CHECK,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            GasOperation::ContractInit => "Contract Initialization",
            GasOperation::ContractCall => "Contract Call",
            GasOperation::StorageRead => "Storage Read",
            GasOperation::StorageWrite => "Storage Write",
            GasOperation::StateVerify => "State Verification",
            GasOperation::AddressCheck => "Address Check",
        }
    }

    /// Calculate cost at given operation price
    pub fn cost_at_price(&self, op_price: u32) -> VMw {
        self.base_cost().saturating_mul(op_price as u64)
    }
}

/// ============================================================================
/// GAS METERING STATE
/// ============================================================================

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GasMeter {
    /// VMw used in current block
    pub vmw_used_block: VMw,
    /// VMw used in current transaction
    pub vmw_used_tx: VMw,
    /// Current operation price (dynamic)
    pub op_price: u32,
    /// Total transactions in current block
    pub tx_count: u32,
    /// Total block size in bytes so far
    pub block_size_bytes: u32,
}

impl GasMeter {
    /// Create new gas meter
    pub fn new(op_price: u32) -> Self {
        Self {
            vmw_used_block: 0,
            vmw_used_tx: 0,
            op_price: op_price.max(MIN_OP_PRICE).min(MAX_OP_PRICE),
            tx_count: 0,
            block_size_bytes: 0,
        }
    }

    /// Check if operation would exceed transaction limit
    pub fn can_consume_vmw_tx(&self, vmw: VMw) -> bool {
        self.vmw_used_tx.saturating_add(vmw) <= VMW_TX_LIMIT
    }

    /// Check if operation would exceed block limit
    pub fn can_consume_vmw_block(&self, vmw: VMw) -> bool {
        self.vmw_used_block.saturating_add(vmw) <= VMW_BLOCK_LIMIT
    }

    /// Check if we can add another transaction to block
    pub fn can_add_transaction(&self) -> bool {
        self.tx_count < MAX_TRANSACTIONS_PER_BLOCK
    }

    /// Check if adding bytes would exceed block size limit
    pub fn can_add_bytes(&self, bytes: u32) -> bool {
        self.block_size_bytes.saturating_add(bytes) <= MAX_BLOCK_SIZE_BYTES
    }

    /// Consume VMw for a transaction
    /// Returns error if limit exceeded
    pub fn consume_vmw_tx(&mut self, vmw: VMw) -> Result<(), &'static str> {
        if self.can_consume_vmw_tx(vmw) {
            self.vmw_used_tx = self.vmw_used_tx.saturating_add(vmw);
            Ok(())
        } else {
            Err("Transaction VMw limit exceeded")
        }
    }

    /// Consume VMw for block
    /// Returns error if limit exceeded
    pub fn consume_vmw_block(&mut self, vmw: VMw) -> Result<(), &'static str> {
        if self.can_consume_vmw_block(vmw) {
            self.vmw_used_block = self.vmw_used_block.saturating_add(vmw);
            Ok(())
        } else {
            Err("Block VMw limit exceeded")
        }
    }

    /// Finalize transaction - commits TX gas to block, resets TX meter
    pub fn finalize_transaction(&mut self) -> Result<(), &'static str> {
        // TX gas already counted toward block, just reset TX meter
        self.vmw_used_tx = 0;
        self.tx_count = self.tx_count.saturating_add(1);
        
        if self.tx_count > MAX_TRANSACTIONS_PER_BLOCK {
            Err("Max transactions per block exceeded")
        } else {
            Ok(())
        }
    }

    /// Add bytes to block size tracker
    pub fn add_block_bytes(&mut self, bytes: u32) -> Result<(), &'static str> {
        if self.can_add_bytes(bytes) {
            self.block_size_bytes = self.block_size_bytes.saturating_add(bytes);
            Ok(())
        } else {
            Err("Block size limit exceeded")
        }
    }

    /// Reset for new block
    pub fn reset_block(&mut self) {
        self.vmw_used_block = 0;
        self.vmw_used_tx = 0;
        self.tx_count = 0;
        self.block_size_bytes = 0;
    }

    /// Update operation price
    pub fn set_op_price(&mut self, new_price: u32) {
        self.op_price = new_price.max(MIN_OP_PRICE).min(MAX_OP_PRICE);
    }

    /// Get percentage of block VMw used
    pub fn block_vmw_percentage(&self) -> u32 {
        if VMW_BLOCK_LIMIT == 0 {
            return 0;
        }
        ((self.vmw_used_block as u64 * 100) / (VMW_BLOCK_LIMIT as u64)) as u32
    }

    /// Get percentage of block size used
    pub fn block_size_percentage(&self) -> u32 {
        if MAX_BLOCK_SIZE_BYTES == 0 {
            return 0;
        }
        ((self.block_size_bytes as u64 * 100) / (MAX_BLOCK_SIZE_BYTES as u64)) as u32
    }
}

impl Default for GasMeter {
    fn default() -> Self {
        Self::new(DEFAULT_OP_PRICE)
    }
}

/// ============================================================================
/// FEE CALCULATOR
/// ============================================================================

#[derive(Debug, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct FeeCalculator {
    /// Current operation price
    pub op_price: u32,
}

impl FeeCalculator {
    /// Create new fee calculator
    pub fn new(op_price: u32) -> Self {
        Self {
            op_price: op_price.max(MIN_OP_PRICE).min(MAX_OP_PRICE),
        }
    }

    /// Calculate cost in ÉTR for an operation
    /// Formula: (VMw_Used × Op_Price) / 1,000,000 = Cost in ÉTR (atomic units)
    /// 
    /// # Arguments
    /// * `vmw_used` - Amount of VMw consumed
    /// 
    /// # Returns
    /// Cost in ÉTR (as u128, matches Balance type)
    pub fn calculate_fee(&self, vmw_used: VMw) -> Balance {
        let cost = (vmw_used as u128).saturating_mul(self.op_price as u128);
        cost.saturating_div(WATTS_PER_ETRID as u128)
    }

    /// Calculate cost for a specific operation
    pub fn cost_for_operation(&self, op: GasOperation) -> Balance {
        let vmw = op.cost_at_price(self.op_price);
        self.calculate_fee(vmw)
    }

    /// Calculate multiple operations
    pub fn cost_for_operations(&self, ops: &[GasOperation]) -> Balance {
        ops.iter()
            .map(|op| op.cost_at_price(self.op_price) as u128)
            .fold(0u128, |acc, x| acc.saturating_add(x))
            .saturating_div(WATTS_PER_ETRID as u128)
    }

    /// Update operation price
    pub fn set_op_price(&mut self, new_price: u32) {
        self.op_price = new_price.max(MIN_OP_PRICE).min(MAX_OP_PRICE);
    }
}

impl Default for FeeCalculator {
    fn default() -> Self {
        Self::new(DEFAULT_OP_PRICE)
    }
}

/// ============================================================================
/// UTILITY FUNCTIONS
/// ============================================================================

/// Convert VMw amount to ÉTR at given operation price
pub fn vmw_to_etrid(vmw: VMw, op_price: u32) -> Balance {
    let cost = (vmw as u128).saturating_mul(op_price as u128);
    cost.saturating_div(WATTS_PER_ETRID as u128)
}

/// Convert ÉTR amount to VMw at given operation price
pub fn etrid_to_vmw(etrid: Balance, op_price: u32) -> VMw {
    if op_price == 0 {
        return 0;
    }
    ((etrid.saturating_mul(WATTS_PER_ETRID as u128)) / (op_price as u128)) as VMw
}

/// ============================================================================
/// TESTS
/// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_costs() {
        assert_eq!(GasOperation::ContractInit.base_cost(), 2_000);
        assert_eq!(GasOperation::ContractCall.base_cost(), 500);
        assert_eq!(GasOperation::StorageRead.base_cost(), 100);
        assert_eq!(GasOperation::StorageWrite.base_cost(), 300);
        assert_eq!(GasOperation::StateVerify.base_cost(), 150);
        assert_eq!(GasOperation::AddressCheck.base_cost(), 50);
    }

    #[test]
    fn test_operation_cost_with_price() {
        let op = GasOperation::ContractCall;
        
        // At price 1
        assert_eq!(op.cost_at_price(1), 500);
        
        // At price 2
        assert_eq!(op.cost_at_price(2), 1_000);
        
        // At price 10
        assert_eq!(op.cost_at_price(10), 5_000);
    }

    #[test]
    fn test_fee_calculation() {
        let calc = FeeCalculator::new(1);

        // 500 VMw at price 1 = 0.0005 ÉTR
        let fee = calc.calculate_fee(500);
        assert_eq!(fee, 0);  // Less than 1 atomic unit

        // 1,000,000 VMw at price 1 = 1 ÉTR
        let fee = calc.calculate_fee(1_000_000);
        assert_eq!(fee, 1);

        // 2,000,000 VMw at price 2 = 4 ÉTR
        let calc2 = FeeCalculator::new(2);
        let fee = calc2.calculate_fee(2_000_000);
        assert_eq!(fee, 4);
    }

    #[test]
    fn test_operation_fee() {
        let calc = FeeCalculator::new(1);

        // Contract call at price 1
        let fee = calc.cost_for_operation(GasOperation::ContractCall);
        assert_eq!(fee, 0);  // 500 VMw < 1 ÉTR

        // Contract init at price 2
        let calc2 = FeeCalculator::new(2);
        let fee = calc2.cost_for_operation(GasOperation::ContractInit);
        assert_eq!(fee, 0);  // 4,000 VMw = 0.004 ÉTR

        // 500 contract calls at price 1
        let fee = calc.calculate_fee(500 * 500);
        assert_eq!(fee, 0);  // 250,000 VMw < 1 ÉTR
    }

    #[test]
    fn test_gas_meter_tx_limit() {
        let mut meter = GasMeter::new(1);

        // Can consume 500k VMw
        assert!(meter.can_consume_vmw_tx(500_000));
        meter.consume_vmw_tx(500_000).unwrap();

        // Can consume another 500k VMw (total = 1M)
        assert!(meter.can_consume_vmw_tx(500_000));
        meter.consume_vmw_tx(500_000).unwrap();
        assert_eq!(meter.vmw_used_tx, 1_000_000);

        // Cannot consume more
        assert!(!meter.can_consume_vmw_tx(1));
        assert!(meter.consume_vmw_tx(1).is_err());
    }

    #[test]
    fn test_gas_meter_block_limit() {
        let mut meter = GasMeter::new(1);

        // Consume 9M VMw toward block
        assert!(meter.can_consume_vmw_block(9_000_000));
        meter.consume_vmw_block(9_000_000).unwrap();

        // Can consume 1M more (total = 10M)
        assert!(meter.can_consume_vmw_block(1_000_000));
        meter.consume_vmw_block(1_000_000).unwrap();
        assert_eq!(meter.vmw_used_block, 10_000_000);

        // Cannot consume more
        assert!(!meter.can_consume_vmw_block(1));
        assert!(meter.consume_vmw_block(1).is_err());
    }

    #[test]
    fn test_gas_meter_percentages() {
        let mut meter = GasMeter::new(1);

        // 0%
        assert_eq!(meter.block_vmw_percentage(), 0);

        // 50%
        meter.consume_vmw_block(5_000_000).unwrap();
        assert_eq!(meter.block_vmw_percentage(), 50);

        // 100%
        meter.consume_vmw_block(5_000_000).unwrap();
        assert_eq!(meter.block_vmw_percentage(), 100);
    }

    #[test]
    fn test_gas_meter_transaction_count() {
        let mut meter = GasMeter::new(1);

        assert_eq!(meter.tx_count, 0);
        meter.finalize_transaction().unwrap();
        assert_eq!(meter.tx_count, 1);

        for _ in 0..998 {
            meter.finalize_transaction().unwrap();
        }
        assert_eq!(meter.tx_count, 999);

        // Can add one more
        meter.finalize_transaction().unwrap();
        assert_eq!(meter.tx_count, 1000);

        // Cannot add more
        assert!(meter.finalize_transaction().is_err());
    }

    #[test]
    fn test_gas_meter_block_size() {
        let mut meter = GasMeter::new(1);

        assert!(meter.can_add_bytes(2_500_000));
        meter.add_block_bytes(2_500_000).unwrap();

        assert!(meter.can_add_bytes(2_500_000));
        meter.add_block_bytes(2_500_000).unwrap();
        assert_eq!(meter.block_size_bytes, 5_000_000);

        // Cannot add more
        assert!(!meter.can_add_bytes(1));
        assert!(meter.add_block_bytes(1).is_err());
    }

    #[test]
    fn test_gas_meter_reset() {
        let mut meter = GasMeter::new(1);

        meter.consume_vmw_block(5_000_000).unwrap();
        meter.consume_vmw_tx(500_000).unwrap();
        meter.tx_count = 10;
        meter.block_size_bytes = 1_000_000;

        meter.reset_block();

        assert_eq!(meter.vmw_used_block, 0);
        assert_eq!(meter.vmw_used_tx, 0);
        assert_eq!(meter.tx_count, 0);
        assert_eq!(meter.block_size_bytes, 0);
    }

    #[test]
    fn test_vmw_to_etrid_conversion() {
        // 1M VMw at price 1 = 1 ÉTR
        assert_eq!(vmw_to_etrid(1_000_000, 1), 1);

        // 2M VMw at price 1 = 2 ÉTR
        assert_eq!(vmw_to_etrid(2_000_000, 1), 2);

        // 1M VMw at price 2 = 2 ÉTR
        assert_eq!(vmw_to_etrid(1_000_000, 2), 2);

        // 500 VMw at price 1 = 0 ÉTR (too small)
        assert_eq!(vmw_to_etrid(500, 1), 0);
    }

    #[test]
    fn test_etrid_to_vmw_conversion() {
        // 1 ÉTR at price 1 = 1M VMw
        assert_eq!(etrid_to_vmw(1, 1), 1_000_000);

        // 2 ÉTR at price 1 = 2M VMw
        assert_eq!(etrid_to_vmw(2, 1), 2_000_000);

        // 1 ÉTR at price 2 = 500k VMw
        assert_eq!(etrid_to_vmw(1, 2), 500_000);

        // Edge case: price 0 returns 0
        assert_eq!(etrid_to_vmw(1, 0), 0);
    }

    #[test]
    fn test_operation_price_bounds() {
        // Min price clamped to 1
        let calc = FeeCalculator::new(0);
        assert_eq!(calc.op_price, MIN_OP_PRICE);

        // Max price clamped to 1000
        let calc = FeeCalculator::new(10_000);
        assert_eq!(calc.op_price, MAX_OP_PRICE);

        // Valid price unchanged
        let calc = FeeCalculator::new(50);
        assert_eq!(calc.op_price, 50);
    }

    #[test]
    fn test_multiple_operations_cost() {
        let calc = FeeCalculator::new(1);

        let ops = vec![
            GasOperation::ContractInit,
            GasOperation::ContractCall,
            GasOperation::StorageWrite,
        ];

        // Total: 2000 + 500 + 300 = 2800 VMw
        // At price 1: 2800 / 1,000,000 = 0 ÉTR
        let fee = calc.cost_for_operations(&ops);
        assert_eq!(fee, 0);
    }

    #[test]
    fn test_constants_sanity() {
        // Block limit > TX limit
        assert!(VMW_BLOCK_LIMIT > VMW_TX_LIMIT);

        // TX limit > any single operation
        assert!(VMW_TX_LIMIT > VMW_CONTRACT_INIT);

        // Conversion rate is sensible
        assert!(WATTS_PER_ETRID > 0);

        // Operation prices have valid range
        assert!(MIN_OP_PRICE <= DEFAULT_OP_PRICE);
        assert!(DEFAULT_OP_PRICE <= MAX_OP_PRICE);
    }
}
