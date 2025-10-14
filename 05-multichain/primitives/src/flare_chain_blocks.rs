//! FlareChain Block Structures (From Ëtrid Ivory Papers)
//!
//! FlareChain stores aggregated transactions using Patricia Merkle Trees.
//! It stores the world state of the Ëtrid Multichain - specifically,
//! hashes of Partitioned Burst Chain block headers aggregated into a Flare Chain block.

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::{AccountId, BlockNumber, Hash, VMw};

/// FlareChain ID (1M/4q/F from Ivory Papers)
pub const FLARE_CHAIN_ID: &str = "1M/4q/F";

/// FlareChain Block Header
/// 
/// Structure from Ivory Papers:
/// - Chain ID – Identifier of the Flare Chain (1M/4q/F)
/// - PPFA – Proposing Panel for Attestation Epoch rotation
/// - PPFA Index – Index of selected Validity Node for attestation
/// - Parent Root – Keccak-256 hash of previous block's root node
/// - Multichain State Root – Keccak-256 hash of aggregate PBC state tree
/// - Body Root – Keccak-256 hash of the block body's root node
/// - VM Wattage – Limit set on VM watts per block
/// - Timestamp – Epoch Unix time when block was virtualized
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FlareChainHeader {
    /// Chain ID (1M/4q/F)
    pub chain_id: Vec<u8>,
    
    /// PPFA - Proposing Panel for Attestation (committee epoch)
    pub ppfa_epoch: u32,
    
    /// PPFA Index - Index of selected Validity Node for attestation
    pub ppfa_index: u32,
    
    /// Parent Root - Keccak-256 hash of previous block's root node
    pub parent_root: Hash,
    
    /// Multichain State Root - Keccak-256 hash of aggregate PBC state tree
    pub multichain_state_root: Hash,
    
    /// Body Root - Keccak-256 hash of the block body's root node
    pub body_root: Hash,
    
    /// VM Wattage Limit - Max VM watts per block
    pub vm_wattage_limit: VMw,
    
    /// Timestamp - Epoch Unix time when block was virtualized
    pub timestamp: u64,
    
    /// Block number
    pub number: BlockNumber,
}

impl Default for FlareChainHeader {
    fn default() -> Self {
        Self {
            chain_id: FLARE_CHAIN_ID.as_bytes().to_vec(),
            ppfa_epoch: 0,
            ppfa_index: 0,
            parent_root: Hash::default(),
            multichain_state_root: Hash::default(),
            body_root: Hash::default(),
            vm_wattage_limit: VMw::new(30_000_000),
            timestamp: 0,
            number: 0,
        }
    }
}

/// FlareChain Block Body
///
/// Structure from Ivory Papers:
/// - Block Signature – Decentralized Director Flare Node Attestation Certificate
/// - PBC Data – Partitioned Burst Chain data
/// - Trunk – Arbitrary block-related data (up to 32 bytes)
/// - Flare Penalty List – Penalized Flare Nodes
/// - Validity Penalty List – Penalized Validity Nodes
/// - Attestations List – Attestations within the block
/// - Stake List – Stake deposits within the block
/// - Exited Node List – Nodes with exited status
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FlareChainBody {
    /// Block Signature - Decentralized Director attestation certificate
    pub block_signature: Option<Vec<u8>>,
    
    /// PBC Data - Aggregated Partitioned Burst Chain state roots
    pub pbc_data: Vec<PbcStateSubmission>,
    
    /// Trunk - Arbitrary block-related data (up to 32 bytes)
    pub trunk: [u8; 32],
    
    /// Flare Penalty List - Penalized Flare Nodes
    pub flare_penalty_list: Vec<PenalizedNode>,
    
    /// Validity Penalty List - Penalized Validity Nodes
    pub validity_penalty_list: Vec<PenalizedNode>,
    
    /// Attestations List - Validity certificates within the block
    pub attestations_list: Vec<AttestationRecord>,
    
    /// Stake List - Stake deposits within the block
    pub stake_list: Vec<StakeDeposit>,
    
    /// Exited Node List - Nodes with exited status
    pub exited_node_list: Vec<AccountId>,
}

impl Default for FlareChainBody {
    fn default() -> Self {
        Self {
            block_signature: None,
            pbc_data: Vec::new(),
            trunk: [0u8; 32],
            flare_penalty_list: Vec::new(),
            validity_penalty_list: Vec::new(),
            attestations_list: Vec::new(),
            stake_list: Vec::new(),
            exited_node_list: Vec::new(),
        }
    }
}

/// PBC State Submission (part of FlareChain body)
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PbcStateSubmission {
    /// PBC Chain ID
    pub pbc_chain_id: Vec<u8>,
    
    /// PBC block number
    pub block_number: BlockNumber,
    
    /// PBC state root hash
    pub state_root: Hash,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Penalized Node record
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PenalizedNode {
    /// Node account
    pub account: AccountId,
    
    /// Penalty reason
    pub reason: Vec<u8>,
    
    /// Penalty amount
    pub amount: u128,
    
    /// Block number when penalized
    pub block_number: BlockNumber,
}

/// Attestation record
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AttestationRecord {
    /// Block hash being attested
    pub block_hash: Hash,
    
    /// Validator account
    pub validator: AccountId,
    
    /// Signature
    pub signature: Vec<u8>,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Stake deposit record
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StakeDeposit {
    /// Depositor account
    pub depositor: AccountId,
    
    /// Amount staked
    pub amount: u128,
    
    /// Validator being staked to (if applicable)
    pub validator: Option<AccountId>,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Complete FlareChain Block
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FlareChainBlock {
    /// Block header
    pub header: FlareChainHeader,
    
    /// Block body
    pub body: FlareChainBody,
}

impl FlareChainBlock {
    /// Create new FlareChain block
    pub fn new(header: FlareChainHeader, body: FlareChainBody) -> Self {
        Self { header, body }
    }
    
    /// Get block number
    pub fn number(&self) -> BlockNumber {
        self.header.number
    }
    
    /// Get parent hash
    pub fn parent_hash(&self) -> Hash {
        self.header.parent_root
    }
    
    /// Get multichain state root
    pub fn multichain_state_root(&self) -> Hash {
        self.header.multichain_state_root
    }
    
    /// Get PPFA information
    pub fn ppfa_info(&self) -> (u32, u32) {
        (self.header.ppfa_epoch, self.header.ppfa_index)
    }
    
    /// Get PBC count
    pub fn pbc_count(&self) -> usize {
        self.body.pbc_data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flare_chain_header_default_works() {
        let header = FlareChainHeader::default();
        assert_eq!(header.chain_id, FLARE_CHAIN_ID.as_bytes());
        assert_eq!(header.vm_wattage_limit.get(), 30_000_000);
    }

    #[test]
    fn flare_chain_block_creation_works() {
        let header = FlareChainHeader::default();
        let body = FlareChainBody::default();
        let block = FlareChainBlock::new(header, body);
        
        assert_eq!(block.number(), 0);
        assert_eq!(block.pbc_count(), 0);
    }
}
