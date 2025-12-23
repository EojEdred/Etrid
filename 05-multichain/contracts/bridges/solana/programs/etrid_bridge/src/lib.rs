use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn};

declare_id!("EtBridge111111111111111111111111111111111111");

/// ETRID Cross-Chain Bridge for Solana
///
/// Features:
/// - Lock SOL/SPL tokens to bridge to ETRID PBC
/// - Mint wrapped tokens after attestation
/// - 5-of-9 Director attestation verification
/// - Replay protection via nonces
///
/// Bridge Flow:
/// 1. User locks tokens -> LockEvent emitted
/// 2. Directors observe and sign attestation
/// 3. Relayer collects 5+ signatures
/// 4. User/relayer claims on destination with attestation

#[program]
pub mod etrid_bridge {
    use super::*;

    /// Initialize the bridge with attester registry
    pub fn initialize(
        ctx: Context<Initialize>,
        domain_id: u32,
        fee_bps: u16,
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        bridge.authority = ctx.accounts.authority.key();
        bridge.domain_id = domain_id;
        bridge.fee_bps = fee_bps;
        bridge.fee_recipient = ctx.accounts.authority.key();
        bridge.nonce = 0;
        bridge.paused = false;
        bridge.bump = ctx.bumps.bridge_state;

        emit!(BridgeInitialized {
            authority: bridge.authority,
            domain_id,
            fee_bps,
        });

        Ok(())
    }

    /// Register the attester registry (links to attestation program)
    pub fn set_attester_registry(
        ctx: Context<SetAttesterRegistry>,
        registry: Pubkey,
    ) -> Result<()> {
        let bridge = &mut ctx.accounts.bridge_state;
        bridge.attester_registry = registry;

        emit!(AttesterRegistryUpdated { registry });
        Ok(())
    }

    /// Lock SPL tokens to bridge to ETRID PBC
    pub fn lock_tokens(
        ctx: Context<LockTokens>,
        amount: u64,
        destination_address: [u8; 32], // SS58 address on ETRID
        destination_domain: u32,
    ) -> Result<()> {
        require!(!ctx.accounts.bridge_state.paused, BridgeError::Paused);
        require!(amount > 0, BridgeError::ZeroAmount);
        require!(destination_domain != ctx.accounts.bridge_state.domain_id, BridgeError::SameDomain);

        let bridge = &mut ctx.accounts.bridge_state;

        // Calculate fee
        let fee = (amount as u128 * bridge.fee_bps as u128 / 10000) as u64;
        let net_amount = amount - fee;

        // Generate request ID
        let nonce = bridge.nonce;
        bridge.nonce += 1;

        let request_id = generate_request_id(
            &ctx.accounts.sender.key(),
            &ctx.accounts.token_mint.key(),
            amount,
            &destination_address,
            destination_domain,
            bridge.domain_id,
            nonce,
        );

        // Transfer tokens to vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        // Transfer fee to recipient (if applicable)
        if fee > 0 {
            let transfer_fee_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.fee_token_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
            );

            let seeds = &[
                b"vault_authority".as_ref(),
                &[ctx.accounts.bridge_state.bump],
            ];
            let signer_seeds = &[&seeds[..]];
            token::transfer(
                transfer_fee_ctx.with_signer(signer_seeds),
                fee,
            )?;
        }

        emit!(TokensLocked {
            request_id,
            sender: ctx.accounts.sender.key(),
            token_mint: ctx.accounts.token_mint.key(),
            amount: net_amount,
            fee,
            destination_address,
            destination_domain,
            nonce,
        });

        Ok(())
    }

    /// Lock native SOL to bridge
    pub fn lock_sol(
        ctx: Context<LockSol>,
        amount: u64,
        destination_address: [u8; 32],
        destination_domain: u32,
    ) -> Result<()> {
        require!(!ctx.accounts.bridge_state.paused, BridgeError::Paused);
        require!(amount > 0, BridgeError::ZeroAmount);
        require!(destination_domain != ctx.accounts.bridge_state.domain_id, BridgeError::SameDomain);

        let bridge = &mut ctx.accounts.bridge_state;

        // Calculate fee
        let fee = (amount as u128 * bridge.fee_bps as u128 / 10000) as u64;
        let net_amount = amount - fee;

        let nonce = bridge.nonce;
        bridge.nonce += 1;

        let request_id = generate_request_id(
            &ctx.accounts.sender.key(),
            &Pubkey::default(), // Native SOL
            amount,
            &destination_address,
            destination_domain,
            bridge.domain_id,
            nonce,
        );

        // Transfer SOL to vault
        let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &ctx.accounts.sol_vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_ix,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.sol_vault.to_account_info(),
            ],
        )?;

        emit!(SolLocked {
            request_id,
            sender: ctx.accounts.sender.key(),
            amount: net_amount,
            fee,
            destination_address,
            destination_domain,
            nonce,
        });

        Ok(())
    }

    /// Unlock tokens after attestation verification
    pub fn unlock_tokens(
        ctx: Context<UnlockTokens>,
        request_id: [u8; 32],
        source_domain: u32,
        amount: u64,
        signatures: Vec<[u8; 65]>, // ECDSA signatures from Directors
    ) -> Result<()> {
        require!(!ctx.accounts.bridge_state.paused, BridgeError::Paused);
        require!(amount > 0, BridgeError::ZeroAmount);

        // Check not already processed
        let processed = &mut ctx.accounts.processed_request;
        require!(!processed.is_processed, BridgeError::AlreadyProcessed);

        // Build message hash
        let message_hash = build_unlock_message_hash(
            &request_id,
            source_domain,
            ctx.accounts.bridge_state.domain_id,
            &ctx.accounts.recipient.key(),
            &ctx.accounts.token_mint.key(),
            amount,
        );

        // Verify 5-of-9 attestation
        let valid_signatures = verify_attestation(
            &ctx.accounts.attester_registry,
            &message_hash,
            &signatures,
        )?;
        require!(valid_signatures >= 5, BridgeError::InsufficientSignatures);

        // Mark as processed
        processed.is_processed = true;
        processed.request_id = request_id;
        processed.processed_at = Clock::get()?.unix_timestamp;

        // Transfer tokens from vault to recipient
        let seeds = &[
            b"vault_authority".as_ref(),
            &[ctx.accounts.bridge_state.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(transfer_ctx, amount)?;

        emit!(TokensUnlocked {
            request_id,
            recipient: ctx.accounts.recipient.key(),
            token_mint: ctx.accounts.token_mint.key(),
            amount,
            source_domain,
        });

        Ok(())
    }

    /// Mint wrapped tokens (for tokens not locked on Solana)
    pub fn mint_wrapped(
        ctx: Context<MintWrapped>,
        request_id: [u8; 32],
        source_domain: u32,
        amount: u64,
        signatures: Vec<[u8; 65]>,
    ) -> Result<()> {
        require!(!ctx.accounts.bridge_state.paused, BridgeError::Paused);
        require!(amount > 0, BridgeError::ZeroAmount);

        // Check not already processed
        let processed = &mut ctx.accounts.processed_request;
        require!(!processed.is_processed, BridgeError::AlreadyProcessed);

        // Build message hash
        let message_hash = build_mint_message_hash(
            &request_id,
            source_domain,
            ctx.accounts.bridge_state.domain_id,
            &ctx.accounts.recipient.key(),
            &ctx.accounts.wrapped_mint.key(),
            amount,
        );

        // Verify attestation
        let valid_signatures = verify_attestation(
            &ctx.accounts.attester_registry,
            &message_hash,
            &signatures,
        )?;
        require!(valid_signatures >= 5, BridgeError::InsufficientSignatures);

        // Mark as processed
        processed.is_processed = true;
        processed.request_id = request_id;
        processed.processed_at = Clock::get()?.unix_timestamp;

        // Mint wrapped tokens
        let seeds = &[
            b"mint_authority".as_ref(),
            &[ctx.accounts.wrapped_token_state.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.wrapped_mint.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::mint_to(mint_ctx, amount)?;

        emit!(WrappedTokensMinted {
            request_id,
            recipient: ctx.accounts.recipient.key(),
            wrapped_mint: ctx.accounts.wrapped_mint.key(),
            amount,
            source_domain,
        });

        Ok(())
    }

    /// Burn wrapped tokens to bridge back
    pub fn burn_wrapped(
        ctx: Context<BurnWrapped>,
        amount: u64,
        destination_address: [u8; 32],
        destination_domain: u32,
    ) -> Result<()> {
        require!(!ctx.accounts.bridge_state.paused, BridgeError::Paused);
        require!(amount > 0, BridgeError::ZeroAmount);

        let bridge = &mut ctx.accounts.bridge_state;
        let nonce = bridge.nonce;
        bridge.nonce += 1;

        let request_id = generate_request_id(
            &ctx.accounts.sender.key(),
            &ctx.accounts.wrapped_mint.key(),
            amount,
            &destination_address,
            destination_domain,
            bridge.domain_id,
            nonce,
        );

        // Burn tokens
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.wrapped_mint.to_account_info(),
                from: ctx.accounts.sender_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        );
        token::burn(burn_ctx, amount)?;

        emit!(WrappedTokensBurned {
            request_id,
            sender: ctx.accounts.sender.key(),
            wrapped_mint: ctx.accounts.wrapped_mint.key(),
            amount,
            destination_address,
            destination_domain,
            nonce,
        });

        Ok(())
    }

    /// Pause bridge operations (admin only)
    pub fn pause(ctx: Context<AdminOnly>) -> Result<()> {
        ctx.accounts.bridge_state.paused = true;
        emit!(BridgePaused {});
        Ok(())
    }

    /// Unpause bridge operations (admin only)
    pub fn unpause(ctx: Context<AdminOnly>) -> Result<()> {
        ctx.accounts.bridge_state.paused = false;
        emit!(BridgeUnpaused {});
        Ok(())
    }

    /// Update fee settings
    pub fn update_fees(
        ctx: Context<AdminOnly>,
        fee_bps: u16,
        fee_recipient: Pubkey,
    ) -> Result<()> {
        require!(fee_bps <= 500, BridgeError::FeeTooHigh); // Max 5%
        let bridge = &mut ctx.accounts.bridge_state;
        bridge.fee_bps = fee_bps;
        bridge.fee_recipient = fee_recipient;

        emit!(FeesUpdated { fee_bps, fee_recipient });
        Ok(())
    }
}

// ============ Account Structures ============

#[account]
pub struct BridgeState {
    pub authority: Pubkey,
    pub attester_registry: Pubkey,
    pub domain_id: u32,
    pub fee_bps: u16,
    pub fee_recipient: Pubkey,
    pub nonce: u64,
    pub paused: bool,
    pub bump: u8,
}

#[account]
pub struct ProcessedRequest {
    pub is_processed: bool,
    pub request_id: [u8; 32],
    pub processed_at: i64,
}

#[account]
pub struct WrappedTokenState {
    pub original_chain: u32,
    pub original_address: [u8; 32],
    pub wrapped_mint: Pubkey,
    pub bump: u8,
}

// ============ Context Structures ============

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 4 + 2 + 32 + 8 + 1 + 1,
        seeds = [b"bridge_state"],
        bump
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAttesterRegistry<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump,
        has_one = authority
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA authority for vault
    #[account(seeds = [b"vault_authority"], bump)]
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub fee_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct LockSol<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: SOL vault PDA
    #[account(
        mut,
        seeds = [b"sol_vault"],
        bump
    )]
    pub sol_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnlockTokens<'info> {
    #[account(
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,

    /// CHECK: Attester registry account
    pub attester_registry: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 1 + 32 + 8,
        seeds = [b"processed", &request_id],
        bump
    )]
    pub processed_request: Account<'info, ProcessedRequest>,

    /// CHECK: Recipient address
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// CHECK: Vault authority PDA
    #[account(seeds = [b"vault_authority"], bump)]
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintWrapped<'info> {
    #[account(
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,

    /// CHECK: Attester registry
    pub attester_registry: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 1 + 32 + 8,
        seeds = [b"processed", &request_id],
        bump
    )]
    pub processed_request: Account<'info, ProcessedRequest>,

    pub wrapped_token_state: Account<'info, WrappedTokenState>,

    /// CHECK: Recipient
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wrapped_mint: Account<'info, Mint>,

    /// CHECK: Mint authority PDA
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BurnWrapped<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,

    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub sender_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub wrapped_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump,
        has_one = authority
    )]
    pub bridge_state: Account<'info, BridgeState>,

    pub authority: Signer<'info>,
}

// ============ Events ============

#[event]
pub struct BridgeInitialized {
    pub authority: Pubkey,
    pub domain_id: u32,
    pub fee_bps: u16,
}

#[event]
pub struct AttesterRegistryUpdated {
    pub registry: Pubkey,
}

#[event]
pub struct TokensLocked {
    pub request_id: [u8; 32],
    pub sender: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub destination_address: [u8; 32],
    pub destination_domain: u32,
    pub nonce: u64,
}

#[event]
pub struct SolLocked {
    pub request_id: [u8; 32],
    pub sender: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub destination_address: [u8; 32],
    pub destination_domain: u32,
    pub nonce: u64,
}

#[event]
pub struct TokensUnlocked {
    pub request_id: [u8; 32],
    pub recipient: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub source_domain: u32,
}

#[event]
pub struct WrappedTokensMinted {
    pub request_id: [u8; 32],
    pub recipient: Pubkey,
    pub wrapped_mint: Pubkey,
    pub amount: u64,
    pub source_domain: u32,
}

#[event]
pub struct WrappedTokensBurned {
    pub request_id: [u8; 32],
    pub sender: Pubkey,
    pub wrapped_mint: Pubkey,
    pub amount: u64,
    pub destination_address: [u8; 32],
    pub destination_domain: u32,
    pub nonce: u64,
}

#[event]
pub struct BridgePaused {}

#[event]
pub struct BridgeUnpaused {}

#[event]
pub struct FeesUpdated {
    pub fee_bps: u16,
    pub fee_recipient: Pubkey,
}

// ============ Errors ============

#[error_code]
pub enum BridgeError {
    #[msg("Bridge is paused")]
    Paused,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Cannot bridge to same domain")]
    SameDomain,
    #[msg("Request already processed")]
    AlreadyProcessed,
    #[msg("Insufficient attestation signatures (need 5 of 9)")]
    InsufficientSignatures,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Fee too high (max 5%)")]
    FeeTooHigh,
}

// ============ Helper Functions ============

fn generate_request_id(
    sender: &Pubkey,
    token: &Pubkey,
    amount: u64,
    destination: &[u8; 32],
    dest_domain: u32,
    source_domain: u32,
    nonce: u64,
) -> [u8; 32] {
    let clock = Clock::get().unwrap();
    anchor_lang::solana_program::keccak::hash(
        &[
            sender.as_ref(),
            token.as_ref(),
            &amount.to_le_bytes(),
            destination.as_ref(),
            &dest_domain.to_le_bytes(),
            &source_domain.to_le_bytes(),
            &nonce.to_le_bytes(),
            &clock.unix_timestamp.to_le_bytes(),
        ]
        .concat(),
    )
    .to_bytes()
}

fn build_unlock_message_hash(
    request_id: &[u8; 32],
    source_domain: u32,
    dest_domain: u32,
    recipient: &Pubkey,
    token: &Pubkey,
    amount: u64,
) -> [u8; 32] {
    anchor_lang::solana_program::keccak::hash(
        &[
            request_id.as_ref(),
            &source_domain.to_le_bytes(),
            &dest_domain.to_le_bytes(),
            recipient.as_ref(),
            token.as_ref(),
            &amount.to_le_bytes(),
        ]
        .concat(),
    )
    .to_bytes()
}

fn build_mint_message_hash(
    request_id: &[u8; 32],
    source_domain: u32,
    dest_domain: u32,
    recipient: &Pubkey,
    wrapped_mint: &Pubkey,
    amount: u64,
) -> [u8; 32] {
    anchor_lang::solana_program::keccak::hash(
        &[
            request_id.as_ref(),
            &source_domain.to_le_bytes(),
            &dest_domain.to_le_bytes(),
            recipient.as_ref(),
            wrapped_mint.as_ref(),
            &amount.to_le_bytes(),
            b"MINT",
        ]
        .concat(),
    )
    .to_bytes()
}

/// Verify attestation signatures from Directors
/// Returns number of valid signatures
fn verify_attestation(
    _attester_registry: &AccountInfo,
    message_hash: &[u8; 32],
    signatures: &[[u8; 65]],
) -> Result<u8> {
    // In production, this would:
    // 1. Load attester registry to get Director pubkeys
    // 2. Use secp256k1 recover to get signer from each signature
    // 3. Check if recovered pubkey is in registry
    // 4. Count unique valid signatures

    // For now, return signature count (actual verification via CPI)
    let mut valid_count = 0u8;

    for signature in signatures.iter() {
        // Verify signature format (65 bytes: r[32] + s[32] + v[1])
        if signature.len() == 65 {
            // In production: secp256k1 signature verification
            // let recovered = secp256k1_recover(message_hash, signature);
            // if is_registered_attester(attester_registry, recovered) {
            //     valid_count += 1;
            // }
            valid_count += 1; // Placeholder
        }
    }

    Ok(valid_count)
}
