//! ETRID Attester Registry for Solana
//!
//! This Anchor program manages the 9 Director attestation system on Solana.
//! It stores attester public keys and validates attestations for cross-chain operations.
//!
//! Threshold: 5-of-9 attesters required for valid attestation

use anchor_lang::prelude::*;

declare_id!("ETRi11111111111111111111111111111111111111");

/// Maximum number of attesters (9 Directors)
pub const MAX_ATTESTERS: usize = 9;

/// Required threshold for valid attestation (5-of-9)
pub const ATTESTATION_THRESHOLD: u8 = 5;

/// Size of ECDSA compressed public key (33 bytes)
pub const ECDSA_PUBKEY_SIZE: usize = 33;

/// Size of ECDSA signature (64 bytes: r + s)
pub const ECDSA_SIGNATURE_SIZE: usize = 64;

#[program]
pub mod etrid_attester_registry {
    use super::*;

    /// Initialize the attester registry with 9 Director public keys
    pub fn initialize(
        ctx: Context<Initialize>,
        attesters: [[u8; ECDSA_PUBKEY_SIZE]; MAX_ATTESTERS],
        threshold: u8,
    ) -> Result<()> {
        require!(
            threshold >= 1 && threshold <= MAX_ATTESTERS as u8,
            AttesterError::InvalidThreshold
        );

        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.attesters = attesters;
        registry.active_count = MAX_ATTESTERS as u8;
        registry.threshold = threshold;
        registry.nonce = 0;
        registry.bump = ctx.bumps.registry;

        emit!(RegistryInitialized {
            authority: registry.authority,
            attester_count: registry.active_count,
            threshold: registry.threshold,
        });

        Ok(())
    }

    /// Update an attester's public key (authority only)
    pub fn update_attester(
        ctx: Context<UpdateAttester>,
        attester_index: u8,
        new_pubkey: [u8; ECDSA_PUBKEY_SIZE],
    ) -> Result<()> {
        require!(
            (attester_index as usize) < MAX_ATTESTERS,
            AttesterError::InvalidAttesterIndex
        );

        let registry = &mut ctx.accounts.registry;
        let old_pubkey = registry.attesters[attester_index as usize];
        registry.attesters[attester_index as usize] = new_pubkey;

        emit!(AttesterUpdated {
            index: attester_index,
            old_pubkey,
            new_pubkey,
        });

        Ok(())
    }

    /// Update the attestation threshold (authority only)
    pub fn update_threshold(ctx: Context<UpdateThreshold>, new_threshold: u8) -> Result<()> {
        require!(
            new_threshold >= 1 && new_threshold <= MAX_ATTESTERS as u8,
            AttesterError::InvalidThreshold
        );

        let registry = &mut ctx.accounts.registry;
        let old_threshold = registry.threshold;
        registry.threshold = new_threshold;

        emit!(ThresholdUpdated {
            old_threshold,
            new_threshold,
        });

        Ok(())
    }

    /// Verify an attestation (cross-chain bridge operation)
    /// Returns true if enough valid signatures are provided
    pub fn verify_attestation(
        ctx: Context<VerifyAttestation>,
        message_hash: [u8; 32],
        signatures: Vec<AttesterSignature>,
    ) -> Result<bool> {
        let registry = &ctx.accounts.registry;

        // Count valid signatures
        let mut valid_count = 0u8;
        let mut used_attesters = [false; MAX_ATTESTERS];

        for sig in signatures.iter() {
            let idx = sig.attester_index as usize;

            // Check index bounds and not already used
            if idx >= MAX_ATTESTERS || used_attesters[idx] {
                continue;
            }

            // Get attester pubkey
            let pubkey = &registry.attesters[idx];

            // Verify ECDSA signature using secp256k1
            if verify_ecdsa_signature(pubkey, &message_hash, &sig.signature) {
                valid_count += 1;
                used_attesters[idx] = true;

                if valid_count >= registry.threshold {
                    break;
                }
            }
        }

        let is_valid = valid_count >= registry.threshold;

        emit!(AttestationVerified {
            message_hash,
            valid_signatures: valid_count,
            threshold: registry.threshold,
            is_valid,
        });

        Ok(is_valid)
    }

    /// Submit and process a bridge attestation
    pub fn process_bridge_attestation(
        ctx: Context<ProcessBridgeAttestation>,
        bridge_request: BridgeRequest,
        signatures: Vec<AttesterSignature>,
    ) -> Result<()> {
        let registry = &ctx.accounts.registry;
        let attestation = &mut ctx.accounts.attestation;

        // Create message hash from bridge request
        let message_hash = bridge_request.compute_hash();

        // Verify attestation
        let mut valid_count = 0u8;
        let mut used_attesters = [false; MAX_ATTESTERS];
        let mut signers = Vec::new();

        for sig in signatures.iter() {
            let idx = sig.attester_index as usize;

            if idx >= MAX_ATTESTERS || used_attesters[idx] {
                continue;
            }

            let pubkey = &registry.attesters[idx];

            if verify_ecdsa_signature(pubkey, &message_hash, &sig.signature) {
                valid_count += 1;
                used_attesters[idx] = true;
                signers.push(sig.attester_index);

                if valid_count >= registry.threshold {
                    break;
                }
            }
        }

        require!(
            valid_count >= registry.threshold,
            AttesterError::InsufficientSignatures
        );

        // Store attestation
        attestation.message_hash = message_hash;
        attestation.bridge_request = bridge_request.clone();
        attestation.signatures_count = valid_count;
        attestation.is_processed = false;
        attestation.timestamp = Clock::get()?.unix_timestamp;
        attestation.bump = ctx.bumps.attestation;

        emit!(BridgeAttestationSubmitted {
            request_id: bridge_request.request_id,
            source_chain: bridge_request.source_chain,
            dest_chain: bridge_request.dest_chain,
            amount: bridge_request.amount,
            signatures_count: valid_count,
        });

        Ok(())
    }

    /// Mark a bridge attestation as processed (executor only)
    pub fn mark_attestation_processed(ctx: Context<MarkProcessed>) -> Result<()> {
        let attestation = &mut ctx.accounts.attestation;

        require!(!attestation.is_processed, AttesterError::AlreadyProcessed);

        attestation.is_processed = true;

        emit!(AttestationProcessed {
            message_hash: attestation.message_hash,
            request_id: attestation.bridge_request.request_id,
        });

        Ok(())
    }

    /// Transfer registry authority (authority only)
    pub fn transfer_authority(ctx: Context<TransferAuthority>, new_authority: Pubkey) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let old_authority = registry.authority;
        registry.authority = new_authority;

        emit!(AuthorityTransferred {
            old_authority,
            new_authority,
        });

        Ok(())
    }
}

// === Account Structures ===

#[account]
#[derive(Default)]
pub struct AttesterRegistry {
    /// Authority that can modify the registry
    pub authority: Pubkey,
    /// Array of 9 Director ECDSA compressed public keys (33 bytes each)
    pub attesters: [[u8; ECDSA_PUBKEY_SIZE]; MAX_ATTESTERS],
    /// Number of active attesters
    pub active_count: u8,
    /// Required threshold for valid attestation
    pub threshold: u8,
    /// Nonce for replay protection
    pub nonce: u64,
    /// PDA bump seed
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct BridgeAttestation {
    /// Hash of the attested message
    pub message_hash: [u8; 32],
    /// The bridge request that was attested
    pub bridge_request: BridgeRequest,
    /// Number of valid signatures
    pub signatures_count: u8,
    /// Whether this attestation has been processed
    pub is_processed: bool,
    /// Timestamp of attestation
    pub timestamp: i64,
    /// PDA bump seed
    pub bump: u8,
}

// === Data Structures ===

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AttesterSignature {
    /// Index of the attester (0-8)
    pub attester_index: u8,
    /// ECDSA signature (64 bytes: r + s)
    pub signature: [u8; ECDSA_SIGNATURE_SIZE],
    /// Recovery ID (v value)
    pub recovery_id: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct BridgeRequest {
    /// Unique request ID
    pub request_id: [u8; 32],
    /// Source chain ID
    pub source_chain: u32,
    /// Destination chain ID (Solana = 1399811149)
    pub dest_chain: u32,
    /// Sender address on source chain
    pub sender: [u8; 32],
    /// Recipient address on destination chain
    pub recipient: Pubkey,
    /// Amount to bridge (in smallest unit)
    pub amount: u64,
    /// Token address (zero for native)
    pub token: Pubkey,
    /// Nonce for replay protection
    pub nonce: u64,
}

impl BridgeRequest {
    pub fn compute_hash(&self) -> [u8; 32] {
        use anchor_lang::solana_program::keccak;

        let mut data = Vec::new();
        data.extend_from_slice(&self.request_id);
        data.extend_from_slice(&self.source_chain.to_le_bytes());
        data.extend_from_slice(&self.dest_chain.to_le_bytes());
        data.extend_from_slice(&self.sender);
        data.extend_from_slice(self.recipient.as_ref());
        data.extend_from_slice(&self.amount.to_le_bytes());
        data.extend_from_slice(self.token.as_ref());
        data.extend_from_slice(&self.nonce.to_le_bytes());

        keccak::hash(&data).to_bytes()
    }
}

// === Contexts ===

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<AttesterRegistry>(),
        seeds = [b"attester_registry"],
        bump
    )]
    pub registry: Account<'info, AttesterRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAttester<'info> {
    #[account(
        mut,
        seeds = [b"attester_registry"],
        bump = registry.bump,
        has_one = authority
    )]
    pub registry: Account<'info, AttesterRegistry>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateThreshold<'info> {
    #[account(
        mut,
        seeds = [b"attester_registry"],
        bump = registry.bump,
        has_one = authority
    )]
    pub registry: Account<'info, AttesterRegistry>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyAttestation<'info> {
    #[account(
        seeds = [b"attester_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AttesterRegistry>,
}

#[derive(Accounts)]
#[instruction(bridge_request: BridgeRequest)]
pub struct ProcessBridgeAttestation<'info> {
    #[account(
        seeds = [b"attester_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AttesterRegistry>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<BridgeAttestation>(),
        seeds = [b"attestation", bridge_request.request_id.as_ref()],
        bump
    )]
    pub attestation: Account<'info, BridgeAttestation>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MarkProcessed<'info> {
    #[account(
        seeds = [b"attester_registry"],
        bump = registry.bump,
        has_one = authority
    )]
    pub registry: Account<'info, AttesterRegistry>,

    #[account(mut)]
    pub attestation: Account<'info, BridgeAttestation>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferAuthority<'info> {
    #[account(
        mut,
        seeds = [b"attester_registry"],
        bump = registry.bump,
        has_one = authority
    )]
    pub registry: Account<'info, AttesterRegistry>,

    pub authority: Signer<'info>,
}

// === Events ===

#[event]
pub struct RegistryInitialized {
    pub authority: Pubkey,
    pub attester_count: u8,
    pub threshold: u8,
}

#[event]
pub struct AttesterUpdated {
    pub index: u8,
    pub old_pubkey: [u8; ECDSA_PUBKEY_SIZE],
    pub new_pubkey: [u8; ECDSA_PUBKEY_SIZE],
}

#[event]
pub struct ThresholdUpdated {
    pub old_threshold: u8,
    pub new_threshold: u8,
}

#[event]
pub struct AttestationVerified {
    pub message_hash: [u8; 32],
    pub valid_signatures: u8,
    pub threshold: u8,
    pub is_valid: bool,
}

#[event]
pub struct BridgeAttestationSubmitted {
    pub request_id: [u8; 32],
    pub source_chain: u32,
    pub dest_chain: u32,
    pub amount: u64,
    pub signatures_count: u8,
}

#[event]
pub struct AttestationProcessed {
    pub message_hash: [u8; 32],
    pub request_id: [u8; 32],
}

#[event]
pub struct AuthorityTransferred {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
}

// === Errors ===

#[error_code]
pub enum AttesterError {
    #[msg("Invalid threshold value")]
    InvalidThreshold,
    #[msg("Invalid attester index")]
    InvalidAttesterIndex,
    #[msg("Insufficient signatures for threshold")]
    InsufficientSignatures,
    #[msg("Invalid ECDSA signature")]
    InvalidSignature,
    #[msg("Attestation already processed")]
    AlreadyProcessed,
    #[msg("Unauthorized")]
    Unauthorized,
}

// === Helper Functions ===

/// Verify an ECDSA signature using secp256k1
/// Uses Solana's native secp256k1 instruction
fn verify_ecdsa_signature(
    pubkey: &[u8; ECDSA_PUBKEY_SIZE],
    message_hash: &[u8; 32],
    signature: &[u8; ECDSA_SIGNATURE_SIZE],
) -> bool {
    // In production, this would use secp256k1_recover or
    // call the native secp256k1 program
    // For now, we'll use a simplified check

    use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;

    // The signature format is r (32 bytes) + s (32 bytes)
    // Recovery ID would be passed separately
    let recovery_id = 0u8; // Would come from AttesterSignature

    match secp256k1_recover(message_hash, recovery_id, signature) {
        Ok(recovered_pubkey) => {
            // Compare first 32 bytes of recovered key with our stored key
            // (Compressed pubkeys are 33 bytes: prefix + x-coordinate)
            let recovered_bytes = recovered_pubkey.to_bytes();

            // Simple comparison (in production, decompress and compare properly)
            pubkey[1..33] == recovered_bytes[0..32]
        }
        Err(_) => false,
    }
}
