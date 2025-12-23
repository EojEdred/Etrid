//! # Runtime Migration v109: Sudo Key Recovery + Bridge Attesters
//!
//! This migration fixes the lost sudo key and registers all 9 Director attesters.
//!
//! ## What Changes
//!
//! 1. **Sudo Key**: Migrated from unknown key to Gizzi Payment account
//! 2. **Bridge Attesters**: Register all 9 Director ECDSA public keys
//! 3. **Threshold Config**: Set 5-of-9 threshold for attestations
//!
//! ## Migration Safety
//!
//! - Preserves all balances
//! - Preserves all pallet state
//! - Maintains chain continuity
//! - Genesis hash unchanged
//!
//! ## Background
//!
//! The original sudo key (5HCvaHrCfXDasyQNRCdJ4jRtcwMmdkPDZEAF3LqF77qf5JtP) was
//! set during genesis but its seed phrase was lost. This migration restores
//! sudo access using the Gizzi Payment account which has a known seed phrase.
//!
//! Bridge attesters are registered for cross-chain message verification.

use frame_support::{
    traits::OnRuntimeUpgrade,
    weights::Weight,
    pallet_prelude::*,
    BoundedVec,
};
use crate::Runtime;
use sp_std::vec::Vec;
use sp_std::vec;

// Gizzi Payment account - raw bytes for the AccountId
// Address: 5HQMqpWrZU1AdN2WumX2Fv8EphJUgiF6fmyMZr94HH31kVQd
// Seed: "hope inject assume uniform attack stereo joke order few couch educate human"
const NEW_SUDO_KEY_BYTES: [u8; 32] = [
    0xec, 0x2a, 0xb3, 0x39, 0x31, 0xd3, 0x0b, 0x3a,
    0x00, 0xcf, 0x66, 0x4c, 0x82, 0xb6, 0x8b, 0x6f,
    0x42, 0x39, 0x77, 0x88, 0xe8, 0xb6, 0xd1, 0x5b,
    0x31, 0x90, 0x9b, 0x94, 0x11, 0xba, 0x35, 0x63
];

/// Director ECDSA compressed public keys (33 bytes each)
/// Source: services/bridge-monitor-service/src/config/production.ts
///
/// These are the actual production public keys matching the EVM addresses:
/// Director-1: 0xA12d48dB2589cfe7ff11a595b80013CffFf5eE3d
/// Director-2: 0x18a6034995CC0c6Db7fC9Ee53E535f5b1984f83e
/// Director-3: 0x574B03172d7e637e2aA645eA9789Fe1E36DdBE33
/// Director-4: 0x698AEdAd3550e716eDA5C923950caC3aA163883F
/// Director-5: 0xa27f49Bf5a5daa961fECF86526bDa0FD315bE988
/// Director-6: 0x6250F01Ca6fcCeB81a1c7E5c2f8A114511188934
/// Director-7: 0x56824F247Bbb54c353025306E860E8edA8877c7b
/// Director-8: 0x64810209643c663D0505806e66Fe5dc0C5cEdB37
/// Director-9: 0x3D9f108A558f9DDDc3c0881d6eafF7292d64dF92
const DIRECTOR_PUBKEYS: [[u8; 33]; 9] = [
    // Director-1: 0x03974b3a6408b7cec959215fbbbbf19af7c34eaa506b1420d89850662ef9af7d1f
    [0x03, 0x97, 0x4b, 0x3a, 0x64, 0x08, 0xb7, 0xce, 0xc9, 0x59, 0x21, 0x5f, 0xbb, 0xbb, 0xf1, 0x9a,
     0xf7, 0xc3, 0x4e, 0xaa, 0x50, 0x6b, 0x14, 0x20, 0xd8, 0x98, 0x50, 0x66, 0x2e, 0xf9, 0xaf, 0x7d, 0x1f],
    // Director-2: 0x02bb133f8096effc1a2d6671f3797986fba0be082a978c5d15eafe2742f369644f
    [0x02, 0xbb, 0x13, 0x3f, 0x80, 0x96, 0xef, 0xfc, 0x1a, 0x2d, 0x66, 0x71, 0xf3, 0x79, 0x79, 0x86,
     0xfb, 0xa0, 0xbe, 0x08, 0x2a, 0x97, 0x8c, 0x5d, 0x15, 0xea, 0xfe, 0x27, 0x42, 0xf3, 0x69, 0x64, 0x4f],
    // Director-3: 0x024e73743d02d4aedc8d08d444af3e608ba7168836a17b3126d1909432f757b41b
    [0x02, 0x4e, 0x73, 0x74, 0x3d, 0x02, 0xd4, 0xae, 0xdc, 0x8d, 0x08, 0xd4, 0x44, 0xaf, 0x3e, 0x60,
     0x8b, 0xa7, 0x16, 0x88, 0x36, 0xa1, 0x7b, 0x31, 0x26, 0xd1, 0x90, 0x94, 0x32, 0xf7, 0x57, 0xb4, 0x1b],
    // Director-4: 0x03e5b448f4d78125a005b5cbdee8cf5acfcc99905f9e352f27cdf36b20a275b3b4
    [0x03, 0xe5, 0xb4, 0x48, 0xf4, 0xd7, 0x81, 0x25, 0xa0, 0x05, 0xb5, 0xcb, 0xde, 0xe8, 0xcf, 0x5a,
     0xcf, 0xcc, 0x99, 0x90, 0x5f, 0x9e, 0x35, 0x2f, 0x27, 0xcd, 0xf3, 0x6b, 0x20, 0xa2, 0x75, 0xb3, 0xb4],
    // Director-5: 0x02c8c32311ae5480b65e9db6dd1059d53747117380b63aba243c4d6b8521795171
    [0x02, 0xc8, 0xc3, 0x23, 0x11, 0xae, 0x54, 0x80, 0xb6, 0x5e, 0x9d, 0xb6, 0xdd, 0x10, 0x59, 0xd5,
     0x37, 0x47, 0x11, 0x73, 0x80, 0xb6, 0x3a, 0xba, 0x24, 0x3c, 0x4d, 0x6b, 0x85, 0x21, 0x79, 0x51, 0x71],
    // Director-6: 0x022d2477d1388e282fa0c2987d7f16e68cf57ba15156f321533fb4b31eb79f6a9b
    [0x02, 0x2d, 0x24, 0x77, 0xd1, 0x38, 0x8e, 0x28, 0x2f, 0xa0, 0xc2, 0x98, 0x7d, 0x7f, 0x16, 0xe6,
     0x8c, 0xf5, 0x7b, 0xa1, 0x51, 0x56, 0xf3, 0x21, 0x53, 0x3f, 0xb4, 0xb3, 0x1e, 0xb7, 0x9f, 0x6a, 0x9b],
    // Director-7: 0x02886bbf04d83c7efcba263123da9c4e502b205f0ccf348dde95e6f81e80a80c72
    [0x02, 0x88, 0x6b, 0xbf, 0x04, 0xd8, 0x3c, 0x7e, 0xfc, 0xba, 0x26, 0x31, 0x23, 0xda, 0x9c, 0x4e,
     0x50, 0x2b, 0x20, 0x5f, 0x0c, 0xcf, 0x34, 0x8d, 0xde, 0x95, 0xe6, 0xf8, 0x1e, 0x80, 0xa8, 0x0c, 0x72],
    // Director-8: 0x0295ff6d4a61254daa701439609e8dbfe99fd541f6e7d609a1229fc449d42209c6
    [0x02, 0x95, 0xff, 0x6d, 0x4a, 0x61, 0x25, 0x4d, 0xaa, 0x70, 0x14, 0x39, 0x60, 0x9e, 0x8d, 0xbf,
     0xe9, 0x9f, 0xd5, 0x41, 0xf6, 0xe7, 0xd6, 0x09, 0xa1, 0x22, 0x9f, 0xc4, 0x49, 0xd4, 0x22, 0x09, 0xc6],
    // Director-9: 0x0280cf1c15ea6b78f6a57b449016a0bd0c24bee322afbc5dcba44ea21e9c66abce
    [0x02, 0x80, 0xcf, 0x1c, 0x15, 0xea, 0x6b, 0x78, 0xf6, 0xa5, 0x7b, 0x44, 0x90, 0x16, 0xa0, 0xbd,
     0x0c, 0x24, 0xbe, 0xe3, 0x22, 0xaf, 0xbc, 0x5d, 0xcb, 0xa4, 0x4e, 0xa2, 0x1e, 0x9c, 0x66, 0xab, 0xce],
];

/// Migration to restore sudo access and register bridge attesters
pub struct MigrateSudoAndEdscMinter;

impl OnRuntimeUpgrade for MigrateSudoAndEdscMinter {
    fn on_runtime_upgrade() -> Weight {
        // log::info!("🔄 Runtime Migration v109: Sudo Key Recovery + Bridge Attesters");

        // ═══════════════════════════════════════════════════════════════════════════
        // PART 1: Change Sudo Key to Gizzi Payment Account
        // ═══════════════════════════════════════════════════════════════════════════

        // Create the new sudo account from raw bytes
        let new_sudo_account = sp_runtime::AccountId32::new(NEW_SUDO_KEY_BYTES);

        // Set the new sudo key
        pallet_sudo::Key::<Runtime>::put(new_sudo_account);

        // ═══════════════════════════════════════════════════════════════════════════
        // PART 2: Register All 9 Director Attesters for Bridge Protocol
        // ═══════════════════════════════════════════════════════════════════════════

        use pallet_bridge_attestation::{
            Attesters, AttesterByPubkey, NextAttesterId, ActiveAttesterCount,
            GlobalThreshold, AttesterInfo, AttesterStatus, ThresholdConfig,
        };
        use frame_system::pallet_prelude::BlockNumberFor;

        // Get current block number for registration timestamp
        let current_block = frame_system::Pallet::<Runtime>::block_number();

        // Register each Director attester
        for (idx, pubkey_bytes) in DIRECTOR_PUBKEYS.iter().enumerate() {
            let attester_id = idx as u32;

            // Create bounded pubkey from bytes
            let pubkey: BoundedVec<u8, ConstU32<64>> = pubkey_bytes.to_vec()
                .try_into()
                .expect("pubkey is 33 bytes, fits in 64");

            // Create attester info
            let attester_info = AttesterInfo::<Runtime> {
                public_key: pubkey.clone(),
                status: AttesterStatus::Active,
                registered_at: current_block,
                messages_signed: 0,
                last_signed_at: current_block,
            };

            // Insert into Attesters storage
            Attesters::<Runtime>::insert(attester_id, attester_info);

            // Insert pubkey → attester_id mapping
            AttesterByPubkey::<Runtime>::insert(pubkey, attester_id);
        }

        // Set next attester ID to 9 (after the 9 directors: 0-8)
        NextAttesterId::<Runtime>::put(9u32);

        // Set active attester count
        ActiveAttesterCount::<Runtime>::put(9u32);

        // ═══════════════════════════════════════════════════════════════════════════
        // PART 3: Configure Global Threshold (5-of-9)
        // ═══════════════════════════════════════════════════════════════════════════

        let threshold_config = ThresholdConfig {
            min_signatures: 5,      // M = 5 signatures required
            total_attesters: 9,     // N = 9 total attesters
            enabled: true,
        };
        GlobalThreshold::<Runtime>::put(threshold_config);

        // Weight calculation:
        // - Sudo: 1 write
        // - Attesters: 9 writes to Attesters map
        // - AttesterByPubkey: 9 writes to pubkey map
        // - NextAttesterId: 1 write
        // - ActiveAttesterCount: 1 write
        // - GlobalThreshold: 1 write
        // Total: 22 writes
        Weight::from_parts(100_000_000, 0)
    }

    #[cfg(feature = "try-runtime")]
    fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
        use codec::Encode;

        // Record current sudo key
        let current_sudo = pallet_sudo::Key::<Runtime>::get();
        Ok(current_sudo.encode())
    }

    #[cfg(feature = "try-runtime")]
    fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
        use pallet_edsc_bridge_attestation::{ActiveAttesterCount, GlobalThreshold};

        // Verify sudo key was changed
        let new_sudo = pallet_sudo::Key::<Runtime>::get();
        let expected = sp_runtime::AccountId32::new(NEW_SUDO_KEY_BYTES);

        if new_sudo != Some(expected) {
            return Err(sp_runtime::DispatchError::Other("Sudo key not updated"));
        }

        // Verify 9 attesters were registered
        let attester_count = ActiveAttesterCount::<Runtime>::get();
        if attester_count != 9 {
            return Err(sp_runtime::DispatchError::Other("Expected 9 attesters"));
        }

        // Verify global threshold is set
        let threshold = GlobalThreshold::<Runtime>::get();
        if threshold.is_none() {
            return Err(sp_runtime::DispatchError::Other("Global threshold not set"));
        }

        let config = threshold.unwrap();
        if config.min_signatures != 5 || config.total_attesters != 9 {
            return Err(sp_runtime::DispatchError::Other("Invalid threshold config"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_weight_is_reasonable() {
        let weight = MigrateSudoAndEdscMinter::on_runtime_upgrade();
        // Migration does 22 writes, so allow up to 200M
        assert!(weight.ref_time() < 200_000_000, "Migration should be lightweight");
    }

    #[test]
    fn director_pubkeys_are_valid() {
        // Verify all pubkeys are 33 bytes
        for (idx, pubkey) in DIRECTOR_PUBKEYS.iter().enumerate() {
            assert_eq!(pubkey.len(), 33, "Director {} pubkey should be 33 bytes", idx + 1);
            // First byte should be 0x02 or 0x03 for compressed ECDSA
            assert!(
                pubkey[0] == 0x02 || pubkey[0] == 0x03,
                "Director {} pubkey should start with 0x02 or 0x03", idx + 1
            );
        }
    }

    #[test]
    fn verify_production_pubkeys_match() {
        // Verify Director-1 pubkey matches expected hex
        // Source: 0x03974b3a6408b7cec959215fbbbbf19af7c34eaa506b1420d89850662ef9af7d1f
        assert_eq!(DIRECTOR_PUBKEYS[0][0], 0x03);
        assert_eq!(DIRECTOR_PUBKEYS[0][1], 0x97);
        assert_eq!(DIRECTOR_PUBKEYS[0][32], 0x1f);

        // Verify Director-9 pubkey matches expected hex
        // Source: 0x0280cf1c15ea6b78f6a57b449016a0bd0c24bee322afbc5dcba44ea21e9c66abce
        assert_eq!(DIRECTOR_PUBKEYS[8][0], 0x02);
        assert_eq!(DIRECTOR_PUBKEYS[8][1], 0x80);
        assert_eq!(DIRECTOR_PUBKEYS[8][32], 0xce);
    }
}
