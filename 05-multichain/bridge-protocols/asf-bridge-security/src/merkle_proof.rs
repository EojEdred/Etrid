//! # Merkle Proof Verification
//!
//! Verifies that a transfer exists in a source block using Merkle proofs.

use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

use asf_collator::Hash;

/// Merkle proof for transfer inclusion
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct MerkleProof {
    /// Merkle root (from block header)
    pub root: Hash,
    /// Proof path (hashes to combine with leaf)
    pub proof: Vec<Hash>,
    /// Leaf index in tree
    pub index: u64,
}

impl MerkleProof {
    /// Create new merkle proof
    pub fn new(root: Hash, proof: Vec<Hash>, index: u64) -> Self {
        Self { root, proof, index }
    }

    /// Verify that leaf is in tree
    pub fn verify(&self, leaf: Hash, expected_root: Hash) -> bool {
        if self.root != expected_root {
            return false;
        }

        let mut current = leaf;
        let mut index = self.index;

        for sibling in &self.proof {
            let mut data = Vec::new();

            if index % 2 == 0 {
                // Current is left child
                data.extend_from_slice(current.as_ref());
                data.extend_from_slice(sibling.as_ref());
            } else {
                // Current is right child
                data.extend_from_slice(sibling.as_ref());
                data.extend_from_slice(current.as_ref());
            }

            current = BlakeTwo256::hash(&data);
            index /= 2;
        }

        current == self.root
    }
}

/// Merkle tree builder for transfer batches
pub struct MerkleTreeBuilder {
    leaves: Vec<Hash>,
}

impl MerkleTreeBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self { leaves: Vec::new() }
    }

    /// Add leaf
    pub fn add_leaf(&mut self, leaf: Hash) {
        self.leaves.push(leaf);
    }

    /// Build tree and get root
    pub fn build(&self) -> Hash {
        if self.leaves.is_empty() {
            return Hash::zero();
        }

        let mut layer = self.leaves.clone();

        while layer.len() > 1 {
            let mut next_layer = Vec::new();

            for i in (0..layer.len()).step_by(2) {
                if i + 1 < layer.len() {
                    // Pair exists
                    let mut data = Vec::new();
                    data.extend_from_slice(layer[i].as_ref());
                    data.extend_from_slice(layer[i + 1].as_ref());
                    next_layer.push(BlakeTwo256::hash(&data));
                } else {
                    // Odd node, promote to next layer
                    next_layer.push(layer[i]);
                }
            }

            layer = next_layer;
        }

        layer[0]
    }

    /// Build proof for leaf at index
    pub fn build_proof(&self, index: usize) -> Option<MerkleProof> {
        if index >= self.leaves.len() {
            return None;
        }

        let mut proof = Vec::new();
        let mut layer = self.leaves.clone();
        let mut current_index = index;

        while layer.len() > 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < layer.len() {
                proof.push(layer[sibling_index]);
            }

            // Build next layer
            let mut next_layer = Vec::new();
            for i in (0..layer.len()).step_by(2) {
                if i + 1 < layer.len() {
                    let mut data = Vec::new();
                    data.extend_from_slice(layer[i].as_ref());
                    data.extend_from_slice(layer[i + 1].as_ref());
                    next_layer.push(BlakeTwo256::hash(&data));
                } else {
                    next_layer.push(layer[i]);
                }
            }

            layer = next_layer;
            current_index /= 2;
        }

        let root = self.build();
        Some(MerkleProof::new(root, proof, index as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_single_leaf() {
        let mut builder = MerkleTreeBuilder::new();
        let leaf = H256::random();
        builder.add_leaf(leaf);

        let root = builder.build();
        assert_eq!(root, leaf);
    }

    #[test]
    fn test_merkle_tree_two_leaves() {
        let mut builder = MerkleTreeBuilder::new();
        let leaf1 = H256::random();
        let leaf2 = H256::random();

        builder.add_leaf(leaf1);
        builder.add_leaf(leaf2);

        let root = builder.build();

        // Manually compute expected root
        let mut data = Vec::new();
        data.extend_from_slice(leaf1.as_ref());
        data.extend_from_slice(leaf2.as_ref());
        let expected = BlakeTwo256::hash(&data);

        assert_eq!(root, expected);
    }

    #[test]
    fn test_merkle_proof_verification() {
        let mut builder = MerkleTreeBuilder::new();

        // Add 4 leaves
        let leaves: Vec<_> = (0..4).map(|_| H256::random()).collect();
        for leaf in &leaves {
            builder.add_leaf(*leaf);
        }

        let root = builder.build();

        // Build proof for leaf 2
        let proof = builder.build_proof(2).unwrap();
        assert!(proof.verify(leaves[2], root));

        // Verify fails with wrong leaf
        assert!(!proof.verify(leaves[0], root));
    }

    #[test]
    fn test_merkle_proof_odd_leaves() {
        let mut builder = MerkleTreeBuilder::new();

        // Add 5 leaves (odd number)
        let leaves: Vec<_> = (0..5).map(|_| H256::random()).collect();
        for leaf in &leaves {
            builder.add_leaf(*leaf);
        }

        let root = builder.build();

        // All proofs should verify
        for i in 0..5 {
            let proof = builder.build_proof(i).unwrap();
            assert!(proof.verify(leaves[i], root));
        }
    }
}
