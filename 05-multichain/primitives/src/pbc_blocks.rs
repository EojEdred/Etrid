//! PBC (Partitioned Burst Chain) Block Structures (From Ëtrid Ivory Papers)
//!
//! PBCs are loosely coupled partitions of the Ëtrid Multichain operating as
//! separate, parallel side chains. Each records chain-specific data transaction
//! types representing state transitions.

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::{AccountId, BlockNumber, Hash, VMw};

/// Maximum ants depth (from Ivory Papers)
pub const MAX_ANTS_DEPTH: u32 = 6;

/// Maximum ants per block (from Ivory Papers)
pub const MAX_ANTS_PER_BLOCK: u32 = 2;

/// PBC Block Header
///
/// Structure from Ivory Papers:
/// - Chain ID – Identifier of the PBC
/// - Parent Reference Hash – Keccak-256 hash of previous block header
/// - Ant Hash – Keccak-256 hash of Ant blocks included
/// - Block Distribution Node – RCUK address for reward distribution
/// - State Root – Keccak-256 hash of state tree root node
/// - Transaction Root – Keccak-256 hash of transaction tree root node
/// - Post Meta State Root – Keccak-256 hash of post-meta state tree
/// - Logs Bloom – Bloom filter from log entries
/// - Genesis Count – Number of blocks since genesis
/// - VM Wattage Limit – Max VM watts per block
/// - VM Watts Used – Total consumed VM watts
/// - Timestamp – Unix time of block init
/// - Trunk – Arbitrary block data (up to 32 bytes)
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PbcBlockHeader {
    /// Chain ID - Identifier of the PBC
    pub chain_id: Vec<u8>,
    
    /// Parent Reference Hash - Keccak-256 hash of previous block header
    pub parent_reference_hash: Hash,
    
    /// Ant Hash - Keccak-256 hash of Ant blocks included
    pub ant_hash: Hash,
    
    /// Block Distribution Node - RCUK address for reward distribution
    pub block_distribution_node: AccountId,
    
    /// State Root - Keccak-256 hash of state tree root node
    pub state_root: Hash,
    
    /// Transaction Root - Keccak-256 hash of transaction tree root node
    pub transaction_root: Hash,
    
    /// Post Meta State Root - Keccak-256 hash of post-meta state tree
    pub post_meta_state_root: Hash,
    
    /// Logs Bloom - Bloom filter from log entries
    pub logs_bloom: [u8; 256],
    
    /// Genesis Count - Number of blocks since genesis
    pub genesis_count: BlockNumber,
    
    /// VM Wattage Limit - Max VM watts per block
    pub vm_wattage_limit: VMw,
    
    /// VM Watts Used - Total consumed VM watts
    pub vm_watts_used: VMw,
    
    /// Timestamp - Unix time of block init
    pub timestamp: u64,
    
    /// Trunk - Arbitrary block data (up to 32 bytes)
    pub trunk: [u8; 32],
}

impl Default for PbcBlockHeader {
    fn default() -> Self {
        use sp_core::crypto::AccountId32;
        Self {
            chain_id: Vec::new(),
            parent_reference_hash: Hash::default(),
            ant_hash: Hash::default(),
            block_distribution_node: AccountId32::new([0u8; 32]),
            state_root: Hash::default(),
            transaction_root: Hash::default(),
            post_meta_state_root: Hash::default(),
            logs_bloom: [0u8; 256],
            genesis_count: 0,
            vm_wattage_limit: VMw::new(30_000_000), // 30M VMw default
            vm_watts_used: VMw::zero(),
            timestamp: 0,
            trunk: [0u8; 32],
        }
    }
}

/// PBC Block Body
///
/// Structure from Ivory Papers:
/// - Transaction List – All recorded transactions
/// - Ant Block List – Valid but stale blocks (secondary blocks)
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PbcBlockBody {
    /// Transaction List - All recorded transactions
    pub transaction_list: Vec<TransactionRecord>,
    
    /// Ant Block List - Valid but stale blocks (secondary blocks)
    pub ant_block_list: Vec<AntBlock>,
}

impl Default for PbcBlockBody {
    fn default() -> Self {
        Self {
            transaction_list: Vec::new(),
            ant_block_list: Vec::new(),
        }
    }
}

/// Transaction Record (simplified for now)
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TransactionRecord {
    /// Transaction hash
    pub hash: Hash,
    
    /// From address
    pub from: AccountId,
    
    /// To address (None for contract creation)
    pub to: Option<AccountId>,
    
    /// Value transferred
    pub value: u128,
    
    /// Data payload
    pub data: Vec<u8>,
    
    /// VMw limit for this transaction
    pub vmw_limit: VMw,
    
    /// VMw used by this transaction
    pub vmw_used: VMw,
    
    /// Nonce
    pub nonce: u64,
    
    /// Signature
    pub signature: Vec<u8>,
}

/// Ant Block (Secondary Block from Ivory Papers)
///
/// "Ants" are secondary blocks that can be attached to primary blocks.
/// - Maximum depth: 6 levels
/// - Maximum per block: 2 ants
/// - Used for parallel transaction processing and increased throughput
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AntBlock {
    /// Ant hash (unique identifier)
    pub ant_hash: Hash,
    
    /// Parent block hash (which block this ant is attached to)
    pub parent_hash: Hash,
    
    /// Depth level (1-6)
    pub depth: u32,
    
    /// Producer (validator who created this ant)
    pub producer: AccountId,
    
    /// Transactions in this ant
    pub transactions: Vec<TransactionRecord>,
    
    /// State root after ant execution
    pub state_root: Hash,
    
    /// VMw used by this ant
    pub vmw_used: VMw,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Child ants (recursive structure, max depth 6)
    pub child_ants: Vec<AntBlock>,
}

impl AntBlock {
    /// Create new ant block
    pub fn new(
        ant_hash: Hash,
        parent_hash: Hash,
        depth: u32,
        producer: AccountId,
    ) -> Self {
        Self {
            ant_hash,
            parent_hash,
            depth,
            producer,
            transactions: Vec::new(),
            state_root: Hash::default(),
            vmw_used: VMw::zero(),
            timestamp: 0,
            child_ants: Vec::new(),
        }
    }
    
    /// Validate ant depth
    pub fn is_valid_depth(&self) -> bool {
        self.depth <= MAX_ANTS_DEPTH
    }
    
    /// Count total ants in tree (recursive)
    pub fn count_total_ants(&self) -> u32 {
        1 + self.child_ants.iter().map(|a| a.count_total_ants()).sum::<u32>()
    }
    
    /// Validate entire ant tree structure
    pub fn validate_tree(&self) -> bool {
        // Check depth limit
        if !self.is_valid_depth() {
            return false;
        }
        
        // Check child count
        if self.child_ants.len() > MAX_ANTS_PER_BLOCK as usize {
            return false;
        }
        
        // Recursively validate children
        for child in &self.child_ants {
            if !child.validate_tree() {
                return false;
            }
            
            // Ensure child depth is parent depth + 1
            if child.depth != self.depth + 1 {
                return false;
            }
        }
        
        true
    }
}

/// Complete PBC Block
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct PbcBlock {
    /// Block header
    pub header: PbcBlockHeader,
    
    /// Block body
    pub body: PbcBlockBody,
}

impl PbcBlock {
    /// Create new PBC block
    pub fn new(header: PbcBlockHeader, body: PbcBlockBody) -> Self {
        Self { header, body }
    }
    
    /// Get block number
    pub fn number(&self) -> BlockNumber {
        self.header.genesis_count
    }
    
    /// Get parent hash
    pub fn parent_hash(&self) -> Hash {
        self.header.parent_reference_hash
    }
    
    /// Get chain ID
    pub fn chain_id(&self) -> &[u8] {
        &self.header.chain_id
    }
    
    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.body.transaction_list.len()
    }
    
    /// Get ant count
    pub fn ant_count(&self) -> usize {
        self.body.ant_block_list.len()
    }
    
    /// Get total VMw used (including ants)
    pub fn total_vmw_used(&self) -> VMw {
        let mut total = self.header.vm_watts_used;
        
        for ant in &self.body.ant_block_list {
            total = total.saturating_add(ant.vmw_used);
        }
        
        total
    }
    
    /// Validate block structure
    pub fn validate(&self) -> bool {
        // Check VMw limit
        if self.total_vmw_used() > self.header.vm_wattage_limit {
            return false;
        }
        
        // Check ant count
        if self.ant_count() > MAX_ANTS_PER_BLOCK as usize {
            return false;
        }
        
        // Validate each ant tree
        for ant in &self.body.ant_block_list {
            if !ant.validate_tree() {
                return false;
            }
        }
        
        true
    }
}

/// Block validation result (from Ivory Papers finalization process)
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BlockValidationResult {
    /// Block is valid
    pub is_valid: bool,
    
    /// Ants validated
    pub ants_valid: bool,
    
    /// VMw usage within limits
    pub vmw_valid: bool,
    
    /// State roots match
    pub state_valid: bool,
    
    /// Post-meta state valid
    pub post_meta_valid: bool,
}

impl BlockValidationResult {
    /// Check if block fully validated
    pub fn is_fully_valid(&self) -> bool {
        self.is_valid && self.ants_valid && self.vmw_valid && self.state_valid && self.post_meta_valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::crypto::AccountId32;

    fn default_account_id() -> AccountId {
        AccountId32::new([0u8; 32])
    }

    #[test]
    fn pbc_header_default_works() {
        let header = PbcBlockHeader::default();
        assert_eq!(header.vm_wattage_limit.get(), 30_000_000);
        assert_eq!(header.vm_watts_used.get(), 0);
    }

    #[test]
    fn ant_block_validation_works() {
        let mut ant = AntBlock::new(
            Hash::default(),
            Hash::default(),
            1,
            default_account_id(),
        );

        assert!(ant.is_valid_depth());
        assert!(ant.validate_tree());

        // Test depth limit
        ant.depth = 7;
        assert!(!ant.is_valid_depth());
    }

    #[test]
    fn ant_tree_validation_works() {
        let mut parent_ant = AntBlock::new(
            Hash::default(),
            Hash::default(),
            1,
            default_account_id(),
        );

        // Add valid child
        let child1 = AntBlock::new(
            Hash::default(),
            Hash::default(),
            2, // depth = parent + 1
            default_account_id(),
        );

        parent_ant.child_ants.push(child1);
        assert!(parent_ant.validate_tree());

        // Add too many children
        let child2 = AntBlock::new(
            Hash::default(),
            Hash::default(),
            2,
            default_account_id(),
        );
        let child3 = AntBlock::new(
            Hash::default(),
            Hash::default(),
            2,
            default_account_id(),
        );

        parent_ant.child_ants.push(child2);
        parent_ant.child_ants.push(child3);

        assert!(!parent_ant.validate_tree()); // More than 2 children
    }

    #[test]
    fn pbc_block_validation_works() {
        let header = PbcBlockHeader::default();
        let body = PbcBlockBody::default();
        let block = PbcBlock::new(header, body);

        assert!(block.validate());
        assert_eq!(block.ant_count(), 0);
        assert_eq!(block.transaction_count(), 0);
    }

    #[test]
    fn pbc_block_vmw_calculation_works() {
        let mut header = PbcBlockHeader::default();
        header.vm_watts_used = VMw::new(1_000_000);

        let mut body = PbcBlockBody::default();

        // Add ant with VMw usage
        let mut ant = AntBlock::new(
            Hash::default(),
            Hash::default(),
            1,
            default_account_id(),
        );
        ant.vmw_used = VMw::new(500_000);
        body.ant_block_list.push(ant);

        let block = PbcBlock::new(header, body);
        assert_eq!(block.total_vmw_used().get(), 1_500_000);
    }
}
