//! Integration Tests for Ëtrid Custom Block Structures
//!
//! Tests FlareChain blocks, PBC blocks, and Ants (secondary blocks)

#[cfg(test)]
mod tests {
    use etrid_primitives::{
        // FlareChain types
        FlareChainBlock, FlareChainHeader, FlareChainBody,
        PbcStateSubmission, AttestationRecord, StakeDeposit,
        FLARE_CHAIN_ID,
        
        // PBC types
        PbcBlock, PbcBlockHeader, PbcBlockBody,
        AntBlock, TransactionRecord,
        MAX_ANTS_DEPTH, MAX_ANTS_PER_BLOCK,
        
        // Common types
        VMw, VMwMetering, BlockVMwLimit,
        AccountId, Hash, BlockNumber,
    };
    use sp_core::H256;
    use sp_runtime::AccountId32;

    // ═══════════════════════════════════════════════════════════════════════
    // FLARECHAIN BLOCK TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn flare_chain_block_creation_works() {
        // Create FlareChain block with default values
        let header = FlareChainHeader::default();
        let body = FlareChainBody::default();
        let block = FlareChainBlock::new(header, body);

        // Verify basic properties
        assert_eq!(block.number(), 0);
        assert_eq!(block.pbc_count(), 0);
        
        // Verify chain ID
        let (epoch, index) = block.ppfa_info();
        assert_eq!(epoch, 0);
        assert_eq!(index, 0);
    }

    #[test]
    fn flare_chain_header_has_correct_defaults() {
        let header = FlareChainHeader::default();
        
        // Verify chain ID matches Ivory Papers
        assert_eq!(header.chain_id, FLARE_CHAIN_ID.as_bytes());
        
        // Verify VMw limit
        assert_eq!(header.vm_wattage_limit.get(), 30_000_000);
        
        // Verify PPFA
        assert_eq!(header.ppfa_epoch, 0);
        assert_eq!(header.ppfa_index, 0);
    }

    #[test]
    fn flare_chain_can_aggregate_pbc_states() {
        let mut body = FlareChainBody::default();
        
        // Add PBC state submissions
        let pbc_submission = PbcStateSubmission {
            pbc_chain_id: b"BTC".to_vec(),
            block_number: 100,
            state_root: H256::random(),
            timestamp: 1234567890,
        };
        
        body.pbc_data.push(pbc_submission);
        
        assert_eq!(body.pbc_data.len(), 1);
    }

    #[test]
    fn flare_chain_can_record_attestations() {
        let mut body = FlareChainBody::default();
        
        // Add attestation
        let attestation = AttestationRecord {
            block_hash: H256::random(),
            validator: AccountId32::new([1u8; 32]),
            signature: vec![0u8; 64],
            timestamp: 1234567890,
        };
        
        body.attestations_list.push(attestation);
        
        assert_eq!(body.attestations_list.len(), 1);
    }

    #[test]
    fn flare_chain_can_record_stakes() {
        let mut body = FlareChainBody::default();
        
        // Add stake deposit
        let stake = StakeDeposit {
            depositor: AccountId32::new([2u8; 32]),
            amount: 64_000_000_000_000_000_000_000, // 64 ËTR
            validator: Some(AccountId32::new([3u8; 32])),
            timestamp: 1234567890,
        };
        
        body.stake_list.push(stake);
        
        assert_eq!(body.stake_list.len(), 1);
        assert_eq!(body.stake_list[0].amount, 64_000_000_000_000_000_000_000);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // PBC BLOCK TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn pbc_block_creation_works() {
        let header = PbcBlockHeader::default();
        let body = PbcBlockBody::default();
        let block = PbcBlock::new(header, body);

        assert_eq!(block.number(), 0);
        assert_eq!(block.transaction_count(), 0);
        assert_eq!(block.ant_count(), 0);
        assert!(block.validate());
    }

    #[test]
    fn pbc_block_header_has_correct_defaults() {
        let header = PbcBlockHeader::default();
        
        // Verify VMw limits
        assert_eq!(header.vm_wattage_limit.get(), 30_000_000);
        assert_eq!(header.vm_watts_used.get(), 0);
        
        // Verify genesis count
        assert_eq!(header.genesis_count, 0);
        
        // Verify trunk size
        assert_eq!(header.trunk.len(), 32);
    }

    #[test]
    fn pbc_block_can_add_transactions() {
        let mut body = PbcBlockBody::default();
        
        // Add transaction
        let tx = TransactionRecord {
            hash: H256::random(),
            from: AccountId32::new([4u8; 32]),
            to: Some(AccountId32::new([5u8; 32])),
            value: 1_000_000_000_000_000_000, // 1 ËTR
            data: vec![],
            vmw_limit: VMw::new(21_000),
            vmw_used: VMw::new(21_000),
            nonce: 0,
            signature: vec![0u8; 64],
        };
        
        body.transaction_list.push(tx);
        
        assert_eq!(body.transaction_list.len(), 1);
    }

    #[test]
    fn pbc_block_tracks_vmw_usage() {
        let mut header = PbcBlockHeader::default();
        header.vm_watts_used = VMw::new(5_000_000);
        
        let body = PbcBlockBody::default();
        let block = PbcBlock::new(header, body);
        
        let total_vmw = block.total_vmw_used();
        assert_eq!(total_vmw.get(), 5_000_000);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ANT (SECONDARY BLOCK) TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn ant_block_creation_works() {
        let ant = AntBlock::new(
            H256::random(),
            H256::random(),
            1, // depth
            AccountId32::new([6u8; 32]),
        );
        
        assert_eq!(ant.depth, 1);
        assert!(ant.is_valid_depth());
        assert!(ant.validate_tree());
    }

    #[test]
    fn ant_enforces_max_depth() {
        let ant = AntBlock::new(
            H256::random(),
            H256::random(),
            MAX_ANTS_DEPTH, // depth = 6 (max)
            AccountId32::new([7u8; 32]),
        );
        
        assert!(ant.is_valid_depth());
        
        // Depth 7 should be invalid
        let mut invalid_ant = ant.clone();
        invalid_ant.depth = 7;
        assert!(!invalid_ant.is_valid_depth());
    }

    #[test]
    fn ant_tree_validation_works() {
        let mut parent_ant = AntBlock::new(
            H256::random(),
            H256::random(),
            1,
            AccountId32::new([8u8; 32]),
        );
        
        // Add child ant (depth = parent + 1)
        let child_ant = AntBlock::new(
            H256::random(),
            H256::random(),
            2, // parent depth + 1
            AccountId32::new([9u8; 32]),
        );
        
        parent_ant.child_ants.push(child_ant);
        assert!(parent_ant.validate_tree());
        
        // Add second child (still valid, max is 2)
        let child_ant2 = AntBlock::new(
            H256::random(),
            H256::random(),
            2,
            AccountId32::new([10u8; 32]),
        );
        
        parent_ant.child_ants.push(child_ant2);
        assert!(parent_ant.validate_tree());
        assert_eq!(parent_ant.child_ants.len(), 2);
    }

    #[test]
    fn ant_enforces_max_children() {
        let mut parent_ant = AntBlock::new(
            H256::random(),
            H256::random(),
            1,
            AccountId32::new([11u8; 32]),
        );
        
        // Add 3 children (exceeds max of 2)
        for i in 0..3 {
            let child = AntBlock::new(
                H256::random(),
                H256::random(),
                2,
                AccountId32::new([(12 + i) as u8; 32]),
            );
            parent_ant.child_ants.push(child);
        }
        
        // Should fail validation
        assert!(!parent_ant.validate_tree());
    }

    #[test]
    fn ant_counts_total_ants_recursively() {
        let mut parent_ant = AntBlock::new(
            H256::random(),
            H256::random(),
            1,
            AccountId32::new([15u8; 32]),
        );
        
        // Add child with its own child
        let mut child_ant = AntBlock::new(
            H256::random(),
            H256::random(),
            2,
            AccountId32::new([16u8; 32]),
        );
        
        let grandchild_ant = AntBlock::new(
            H256::random(),
            H256::random(),
            3,
            AccountId32::new([17u8; 32]),
        );
        
        child_ant.child_ants.push(grandchild_ant);
        parent_ant.child_ants.push(child_ant);
        
        // Total: parent (1) + child (1) + grandchild (1) = 3
        assert_eq!(parent_ant.count_total_ants(), 3);
    }

    #[test]
    fn pbc_block_with_ants_validates() {
        let mut header = PbcBlockHeader::default();
        header.vm_watts_used = VMw::new(1_000_000);
        
        let mut body = PbcBlockBody::default();
        
        // Add ant with VMw usage
        let mut ant = AntBlock::new(
            H256::random(),
            H256::random(),
            1,
            AccountId32::new([18u8; 32]),
        );
        ant.vmw_used = VMw::new(500_000);
        
        body.ant_block_list.push(ant);
        
        let block = PbcBlock::new(header, body);
        
        // Block should validate (VMw within limit)
        assert!(block.validate());
        
        // Total VMw = header VMw + ant VMw
        assert_eq!(block.total_vmw_used().get(), 1_500_000);
    }

    #[test]
    fn pbc_block_rejects_too_many_ants() {
        let header = PbcBlockHeader::default();
        let mut body = PbcBlockBody::default();
        
        // Add 3 ants (exceeds max of 2)
        for i in 0..3 {
            let ant = AntBlock::new(
                H256::random(),
                H256::random(),
                1,
                AccountId32::new([(19 + i) as u8; 32]),
            );
            body.ant_block_list.push(ant);
        }
        
        let block = PbcBlock::new(header, body);
        
        // Should fail validation
        assert!(!block.validate());
    }

    // ═══════════════════════════════════════════════════════════════════════
    // VMW METERING TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn vmw_arithmetic_works() {
        let vmw1 = VMw::new(1000);
        let vmw2 = VMw::new(500);
        
        assert_eq!((vmw1 + vmw2).get(), 1500);
        assert_eq!((vmw1 - vmw2).get(), 500);
        assert_eq!((vmw1 * 2).get(), 2000);
    }

    #[test]
    fn vmw_metering_calculates_refunds() {
        let metering = VMwMetering::new(
            VMw::new(100_000),  // allocated
            VMw::new(75_000),   // used
            1_000_000_000_000,  // price per unit
        );
        
        assert_eq!(metering.vmw_refund.get(), 25_000);
        assert!(metering.is_success());
        assert_eq!(metering.total_cost, 75_000_000_000_000_000);
    }

    #[test]
    fn vmw_metering_detects_over_usage() {
        let metering = VMwMetering::new(
            VMw::new(50_000),   // allocated
            VMw::new(75_000),   // used (exceeds allocation!)
            1_000_000_000_000,
        );
        
        // Should fail (used > available)
        assert!(!metering.is_success());
    }

    #[test]
    fn block_vmw_limit_defaults() {
        let limit = BlockVMwLimit::default();
        
        assert_eq!(limit.max_vmw.get(), 30_000_000);
        assert_eq!(limit.target_vmw.get(), 15_000_000);
    }
}
