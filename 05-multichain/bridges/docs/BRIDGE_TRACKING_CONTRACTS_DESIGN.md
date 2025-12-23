# Bridge Tracking Contracts Design

## Executive Summary

This document presents a comprehensive design for bridge tracking contracts that monitor external chain swaps flowing into ETRID. The system focuses on **tracking and verification** of external chain state while keeping all wrapped tokens native to ETRID Partition Burst Chains (PBCs). No external deployments are required.

**Last Updated**: December 8, 2025
**Status**: Design Phase
**Author**: Bridge Architecture Team

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [External Chain State Tracking](#external-chain-state-tracking)
3. [Event Monitoring and Verification](#event-monitoring-and-verification)
4. [Accounting and Reconciliation](#accounting-and-reconciliation)
5. [Security Considerations](#security-considerations)
6. [Contract Specifications](#contract-specifications)
7. [Implementation Roadmap](#implementation-roadmap)
8. [Integration with Existing Systems](#integration-with-existing-systems)

---

## Architecture Overview

### Design Principles

1. **Native Token Approach**: All wrapped tokens exist ONLY on ETRID PBCs
2. **External Monitoring**: Track external chain events without deploying contracts externally
3. **Multi-Layer Verification**: Combine multiple proof systems for security
4. **Custodian Security**: Leverage existing M-of-N multisig infrastructure
5. **Comprehensive Accounting**: Full audit trail for all cross-chain flows

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    External Blockchains                          │
│  (Bitcoin, Ethereum, Solana, Tron, BNB Chain, etc.)             │
└───────────────────┬─────────────────────────────────────────────┘
                    │
                    │ Events, State Proofs
                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                Relayer Network + Oracle Layer                    │
│  ┌──────────────┬──────────────┬──────────────┬──────────────┐ │
│  │  SPV Proofs  │  Light Client│   Relayers   │  Event Watch │ │
│  │  Validator   │  Verifier    │   (Trusted)  │  Service     │ │
│  └──────────────┴──────────────┴──────────────┴──────────────┘ │
└───────────────────┬─────────────────────────────────────────────┘
                    │
                    │ Verified State + Proofs
                    ▼
┌─────────────────────────────────────────────────────────────────┐
│           Bridge Tracking Contracts (ETRID Native)               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  State Verification Contract                              │  │
│  │  - Merkle proof validation                                │  │
│  │  - Block header validation                                │  │
│  │  - Finality confirmation                                  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Event Tracking Contract                                  │  │
│  │  - External deposit detection                             │  │
│  │  - Burn event monitoring                                  │  │
│  │  - Cross-chain transaction indexing                       │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Accounting & Reconciliation Contract                     │  │
│  │  - Total value locked (TVL) tracking                      │  │
│  │  - Per-chain balance reconciliation                       │  │
│  │  - Audit trail generation                                 │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Custodian Approval Contract (M-of-N Multisig)           │  │
│  │  - Withdrawal verification                                │  │
│  │  - Emergency pause mechanism                              │  │
│  │  - Rate limit enforcement                                 │  │
│  └──────────────────────────────────────────────────────────┘  │
└───────────────────┬─────────────────────────────────────────────┘
                    │
                    │ Mint/Burn Operations
                    ▼
┌─────────────────────────────────────────────────────────────────┐
│              Partition Burst Chains (PBCs)                       │
│  - BTC PBC (Wrapped BTC)                                         │
│  - ETH PBC (Wrapped ETH + ERC-20)                               │
│  - SOL PBC (Wrapped SOL + SPL tokens)                           │
│  - TRX PBC (Wrapped TRX + TRC-20)                               │
│  - ... (all wrapped tokens stay native on ETRID)                │
└─────────────────────────────────────────────────────────────────┘
```

---

## External Chain State Tracking

### Challenge: Monitoring External Chains Without Deployments

Since ETRID does not deploy contracts to external chains, we must track external chain state through alternative mechanisms.

### Solution: Multi-Proof Verification System

#### 1. SPV (Simplified Payment Verification) Proofs

**Concept**: Light client verification using block headers and Merkle proofs.

**Implementation for Bitcoin Bridge**:
```rust
pub struct SPVProof {
    /// Bitcoin block header containing the transaction
    pub block_header: BitcoinBlockHeader,
    /// Merkle branch proving transaction inclusion
    pub merkle_branch: Vec<H256>,
    /// Transaction data
    pub transaction: BitcoinTransaction,
    /// Transaction index in block
    pub tx_index: u32,
}

impl SPVProof {
    /// Verify that transaction is included in block
    pub fn verify(&self) -> Result<(), SPVError> {
        // 1. Validate block header PoW
        ensure!(self.block_header.meets_difficulty_target(), SPVError::InvalidPoW);

        // 2. Reconstruct merkle root from branch
        let computed_root = self.compute_merkle_root();

        // 3. Compare with header merkle root
        ensure!(computed_root == self.block_header.merkle_root, SPVError::InvalidMerkleProof);

        Ok(())
    }
}
```

**For Bitcoin**: SPV proofs validate transaction inclusion in blocks
**For Ethereum**: Merkle Patricia trie proofs validate state/receipt inclusion
**For Solana**: Account state proofs with validator signatures

#### 2. Block Header Chain Verification

**Concept**: Maintain a verified chain of block headers on ETRID to validate proofs against.

**Implementation**:
```rust
pub trait HeaderChainVerifier {
    /// Submit new block header to chain
    fn submit_header(
        origin: OriginFor<T>,
        header: BlockHeader,
        proof: ChainProof,
    ) -> DispatchResult;

    /// Verify header is part of canonical chain
    fn verify_header(&self, header_hash: Hash) -> bool;

    /// Get finalized header at height
    fn get_finalized_header(&self, height: BlockNumber) -> Option<BlockHeader>;

    /// Check if transaction is confirmed with sufficient depth
    fn is_confirmed(&self, tx_hash: Hash, min_confirmations: u32) -> bool;
}
```

**Storage Design**:
```rust
/// Verified block headers for external chain
#[pallet::storage]
pub type VerifiedHeaders<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ChainId, // e.g., "bitcoin", "ethereum"
    BTreeMap<BlockNumber, BlockHeader>,
    ValueQuery,
>;

/// Finalized block heights per chain
#[pallet::storage]
pub type FinalizedHeight<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ChainId,
    BlockNumber,
    ValueQuery,
>;
```

#### 3. Relayer Network Architecture

**Trusted Relayers with Economic Security**:

```rust
pub struct RelayerRegistration<AccountId, Balance> {
    /// Relayer account
    pub relayer: AccountId,
    /// Staked collateral
    pub stake: Balance,
    /// Chains this relayer monitors
    pub chains: Vec<ChainId>,
    /// Reputation score (0-100)
    pub reputation: u8,
    /// Number of successful submissions
    pub successful_submissions: u64,
    /// Number of slashed submissions
    pub slashed_submissions: u64,
}

#[pallet::storage]
pub type RegisteredRelayers<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    RelayerRegistration<T::AccountId, BalanceOf<T>>,
    OptionQuery,
>;
```

**Slashing Conditions**:
1. Submitting invalid proofs
2. Submitting proofs for reorganized blocks
3. Excessive delays in submission
4. Attempting to submit replay attacks

#### 4. Finality Handling Per Chain

Different chains have different finality models:

```rust
pub enum FinalityType {
    /// Probabilistic finality (Bitcoin, Ethereum PoW)
    Probabilistic { confirmations: u32 },
    /// Deterministic finality (Ethereum PoS, Cosmos)
    Deterministic { checkpoint_delay: u32 },
    /// Instant finality (Solana, Avalanche)
    Instant,
}

pub struct ChainConfig {
    pub chain_id: ChainId,
    pub finality_type: FinalityType,
    pub block_time: u64, // seconds
    pub reorg_resistance: u32, // blocks
}

impl ChainConfig {
    /// Calculate required confirmations for safety
    pub fn required_confirmations(&self) -> u32 {
        match self.finality_type {
            FinalityType::Probabilistic { confirmations } => confirmations,
            FinalityType::Deterministic { checkpoint_delay } => checkpoint_delay,
            FinalityType::Instant => 1,
        }
    }

    /// Check if transaction can be considered final
    pub fn is_finalized(&self, tx_height: u64, current_height: u64) -> bool {
        let depth = current_height.saturating_sub(tx_height);
        depth >= self.required_confirmations() as u64
    }
}
```

**Recommended Confirmation Depths**:
- Bitcoin: 6 confirmations (probabilistic, ~1 hour)
- Ethereum: 12 confirmations (probabilistic pre-merge, ~2.5 minutes)
- Ethereum PoS: 2 epochs (deterministic, ~13 minutes)
- Solana: 32 slots (optimistic, ~12 seconds)
- BNB Chain: 15 confirmations (~45 seconds)
- Tron: 19 confirmations (~57 seconds)

---

## Event Monitoring and Verification

### External Chain Event Types

#### 1. Deposit Events

**What to Track**:
```rust
pub struct ExternalDeposit {
    /// External chain identifier
    pub chain_id: ChainId,
    /// Transaction hash on external chain
    pub tx_hash: Hash,
    /// Block number on external chain
    pub block_number: u64,
    /// Sender address on external chain
    pub from_address: Vec<u8>,
    /// Destination address on ETRID
    pub to_address: AccountId,
    /// Asset identifier (native or token contract)
    pub asset: AssetId,
    /// Amount deposited
    pub amount: u128,
    /// Timestamp of deposit
    pub timestamp: u64,
    /// Current confirmations
    pub confirmations: u32,
    /// Proof of inclusion
    pub proof: DepositProof,
}

pub enum DepositProof {
    /// SPV proof for Bitcoin-like chains
    SPV(SPVProof),
    /// Merkle Patricia proof for Ethereum
    EthereumReceipt(ReceiptProof),
    /// Validator signature for Solana
    SolanaAccount(AccountProof),
    /// Generic relayer attestation
    RelayerAttestation(Vec<RelayerSignature>),
}
```

**Event Monitoring Flow**:
```
External Chain → Relayer Detects Deposit → Submit to ETRID
                                              ↓
                                    Verify Proof + Finality
                                              ↓
                                    Store Pending Deposit
                                              ↓
                      Wait for Required Confirmations
                                              ↓
                          Custodian Approval (M-of-N)
                                              ↓
                                    Mint Wrapped Tokens
```

#### 2. Burn Events (Withdrawal from ETRID)

**What to Track**:
```rust
pub struct BurnEvent {
    /// ETRID chain identifier (which PBC)
    pub pbc_id: ChainId,
    /// Burn transaction hash on ETRID
    pub etrid_tx_hash: Hash,
    /// User burning tokens
    pub burner: AccountId,
    /// Destination address on external chain
    pub destination: Vec<u8>,
    /// Asset being burned
    pub asset: AssetId,
    /// Amount burned
    pub amount: u128,
    /// Burn block number on ETRID
    pub block_number: u64,
    /// Status of external release
    pub status: WithdrawalStatus,
}

pub enum WithdrawalStatus {
    Pending,
    AwaitingCustodianApproval,
    Approved,
    Executed { external_tx_hash: Hash },
    Failed { reason: Vec<u8> },
}
```

#### 3. External State Change Detection

**Monitoring External Balances**:
```rust
pub struct ExternalBalance {
    pub chain_id: ChainId,
    pub asset: AssetId,
    pub address: Vec<u8>, // Bridge custodian address on external chain
    pub balance: u128,
    pub last_updated: u64,
    pub last_updated_block: u64,
}

#[pallet::storage]
pub type ExternalBalances<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    ChainId,
    Blake2_128Concat,
    AssetId,
    ExternalBalance,
    OptionQuery,
>;
```

### Event Verification Pipeline

```rust
pub trait EventVerifier {
    /// Verify deposit event with proof
    fn verify_deposit(
        &self,
        deposit: ExternalDeposit,
    ) -> Result<VerifiedDeposit, VerificationError>;

    /// Verify sufficient confirmations
    fn verify_confirmations(
        &self,
        chain_id: ChainId,
        block_number: u64,
        required: u32,
    ) -> Result<(), VerificationError>;

    /// Verify no replay attack
    fn verify_not_processed(
        &self,
        tx_hash: Hash,
    ) -> Result<(), VerificationError>;
}

impl<T: Config> EventVerifier for Pallet<T> {
    fn verify_deposit(&self, deposit: ExternalDeposit) -> Result<VerifiedDeposit, VerificationError> {
        // 1. Verify proof validity
        deposit.proof.verify()?;

        // 2. Check finality
        self.verify_confirmations(
            deposit.chain_id.clone(),
            deposit.block_number,
            self.get_required_confirmations(&deposit.chain_id),
        )?;

        // 3. Check not already processed
        self.verify_not_processed(deposit.tx_hash)?;

        // 4. Validate destination address
        self.verify_destination(&deposit.to_address)?;

        // 5. Validate amount within limits
        self.verify_amount_limits(&deposit.asset, deposit.amount)?;

        Ok(VerifiedDeposit {
            deposit,
            verified_at: Self::current_timestamp(),
            verified_by: Self::current_relayer(),
        })
    }
}
```

---

## Accounting and Reconciliation

### Comprehensive Accounting System

#### 1. Total Value Locked (TVL) Tracking

**Per-Chain TVL**:
```rust
pub struct ChainTVL {
    /// Chain identifier
    pub chain_id: ChainId,
    /// Total value locked on this chain (in USD equivalent)
    pub total_usd: u128,
    /// Assets and their amounts
    pub assets: BTreeMap<AssetId, AssetTVL>,
    /// Last reconciliation timestamp
    pub last_reconciled: u64,
    /// Reconciliation status
    pub status: ReconciliationStatus,
}

pub struct AssetTVL {
    pub asset_id: AssetId,
    /// Amount locked on external chain
    pub external_locked: u128,
    /// Amount minted on ETRID
    pub etrid_minted: u128,
    /// Difference (should be 0)
    pub discrepancy: i128,
    /// Last updated
    pub last_updated: u64,
}

#[pallet::storage]
pub type ChainTVLs<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ChainId,
    ChainTVL,
    OptionQuery,
>;
```

#### 2. Transaction Tracking and Audit Trail

**Cross-Chain Transaction Record**:
```rust
pub struct CrossChainTransaction {
    /// Unique transaction ID (ETRID-generated)
    pub id: TransactionId,
    /// Direction of flow
    pub direction: TransactionDirection,
    /// Source chain
    pub source_chain: ChainId,
    /// Destination chain
    pub dest_chain: ChainId,
    /// Asset transferred
    pub asset: AssetId,
    /// Amount
    pub amount: u128,
    /// User address on source chain
    pub from: Vec<u8>,
    /// User address on destination chain
    pub to: Vec<u8>,
    /// Source chain transaction hash
    pub source_tx: Option<Hash>,
    /// Destination chain transaction hash
    pub dest_tx: Option<Hash>,
    /// Timestamp initiated
    pub initiated_at: u64,
    /// Timestamp completed
    pub completed_at: Option<u64>,
    /// Current status
    pub status: TransactionStatus,
    /// Fee paid
    pub fee: u128,
}

pub enum TransactionDirection {
    /// External chain → ETRID
    Inbound,
    /// ETRID → External chain
    Outbound,
}

pub enum TransactionStatus {
    Initiated,
    PendingConfirmations { current: u32, required: u32 },
    ConfirmedSource,
    AwaitingCustodianApproval,
    Approved,
    ExecutingDestination,
    Completed,
    Failed { reason: Vec<u8> },
    Cancelled,
}

#[pallet::storage]
pub type CrossChainTransactions<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    TransactionId,
    CrossChainTransaction,
    OptionQuery,
>;
```

#### 3. Reconciliation Engine

**Periodic Reconciliation**:
```rust
pub trait ReconciliationEngine {
    /// Perform reconciliation for a chain
    fn reconcile_chain(&mut self, chain_id: ChainId) -> ReconciliationReport;

    /// Check if balances match
    fn verify_balance_match(
        &self,
        chain_id: ChainId,
        asset: AssetId,
    ) -> Result<(), DiscrepancyError>;

    /// Get reconciliation report
    fn get_report(&self, chain_id: ChainId) -> Option<ReconciliationReport>;
}

pub struct ReconciliationReport {
    pub chain_id: ChainId,
    pub timestamp: u64,
    pub assets_checked: u32,
    pub discrepancies_found: u32,
    pub total_value_locked: u128,
    pub total_value_minted: u128,
    pub status: ReconciliationStatus,
    pub issues: Vec<ReconciliationIssue>,
}

pub enum ReconciliationStatus {
    Balanced,
    MinorDiscrepancy, // < 0.1%
    MajorDiscrepancy, // >= 0.1%
    Critical,         // >= 1% or > $10k
}

pub struct ReconciliationIssue {
    pub asset: AssetId,
    pub expected: u128,
    pub actual: u128,
    pub difference: i128,
    pub severity: IssueSeverity,
}
```

**Automated Reconciliation Schedule**:
- Real-time: After each transaction
- Hourly: Quick balance check
- Daily: Full reconciliation with discrepancy detection
- Weekly: Deep audit with external API verification

#### 4. Discrepancy Handling

```rust
impl<T: Config> Pallet<T> {
    /// Handle detected discrepancy
    pub fn handle_discrepancy(
        chain_id: ChainId,
        asset: AssetId,
        discrepancy: i128,
    ) -> DispatchResult {
        let severity = Self::calculate_severity(discrepancy);

        match severity {
            IssueSeverity::Low => {
                // Log and monitor
                Self::log_discrepancy(chain_id, asset, discrepancy);
            }
            IssueSeverity::Medium => {
                // Alert operators
                Self::emit_alert(chain_id, asset, discrepancy);
                Self::schedule_investigation(chain_id, asset);
            }
            IssueSeverity::High => {
                // Pause affected bridge
                Self::emergency_pause_bridge(chain_id)?;
                Self::notify_custodians(chain_id, asset, discrepancy);
            }
            IssueSeverity::Critical => {
                // Pause all bridges, require governance intervention
                Self::emergency_pause_all()?;
                Self::escalate_to_governance(chain_id, asset, discrepancy);
            }
        }

        Ok(())
    }
}
```

---

## Security Considerations

### 1. Replay Attack Prevention

**Problem**: Attacker resubmits valid proof multiple times to mint tokens repeatedly.

**Solution**:
```rust
/// Processed external transactions (prevents replay)
#[pallet::storage]
pub type ProcessedTransactions<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    ChainId,
    Blake2_128Concat,
    Hash, // External chain tx hash
    ProcessedTxInfo,
    OptionQuery,
>;

pub struct ProcessedTxInfo {
    pub processed_at: u64,
    pub processed_block: u64,
    pub amount: u128,
    pub recipient: AccountId,
}

impl<T: Config> Pallet<T> {
    fn check_replay_attack(chain_id: ChainId, tx_hash: Hash) -> DispatchResult {
        ensure!(
            !ProcessedTransactions::<T>::contains_key(&chain_id, &tx_hash),
            Error::<T>::TransactionAlreadyProcessed
        );
        Ok(())
    }
}
```

### 2. Reorganization Handling

**Problem**: Block reorganizations on external chains could invalidate processed transactions.

**Solution**:
```rust
pub struct ReorgProtection {
    /// Minimum confirmations before processing
    pub min_confirmations: u32,
    /// Reorg detection threshold
    pub reorg_threshold: u32,
    /// Maximum reorg depth to handle
    pub max_reorg_depth: u32,
}

impl<T: Config> Pallet<T> {
    /// Handle detected reorganization
    pub fn handle_reorg(
        chain_id: ChainId,
        forked_block: u64,
        new_chain_tip: u64,
    ) -> DispatchResult {
        // 1. Find all transactions processed after fork point
        let affected_txs = Self::get_transactions_after_block(chain_id.clone(), forked_block);

        // 2. Mark as pending reverification
        for tx in affected_txs {
            Self::mark_for_reverification(tx)?;
        }

        // 3. Pause deposits until chain stabilizes
        Self::pause_deposits(chain_id.clone())?;

        // 4. Emit reorg event
        Self::deposit_event(Event::ReorgDetected {
            chain_id,
            forked_block,
            affected_count: affected_txs.len() as u32,
        });

        Ok(())
    }
}
```

### 3. Rate Limiting and Circuit Breakers

**Problem**: Large sudden volumes could indicate exploit or attack.

**Solution**:
```rust
pub struct RateLimits {
    /// Maximum per transaction
    pub max_per_transaction: u128,
    /// Maximum per hour
    pub max_per_hour: u128,
    /// Maximum per day
    pub max_per_day: u128,
    /// Maximum TVL change per hour (%)
    pub max_tvl_change_pct: u8,
}

#[pallet::storage]
pub type RateLimitConfig<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ChainId,
    RateLimits,
    ValueQuery,
>;

impl<T: Config> Pallet<T> {
    fn check_rate_limits(
        chain_id: ChainId,
        amount: u128,
    ) -> DispatchResult {
        let limits = RateLimitConfig::<T>::get(&chain_id);

        // Check per-transaction limit
        ensure!(amount <= limits.max_per_transaction, Error::<T>::ExceedsMaxPerTransaction);

        // Check hourly volume
        let hourly_volume = Self::get_hourly_volume(&chain_id);
        ensure!(
            hourly_volume + amount <= limits.max_per_hour,
            Error::<T>::HourlyLimitExceeded
        );

        // Check daily volume
        let daily_volume = Self::get_daily_volume(&chain_id);
        ensure!(
            daily_volume + amount <= limits.max_per_day,
            Error::<T>::DailyLimitExceeded
        );

        Ok(())
    }

    /// Circuit breaker for anomalous activity
    pub fn check_circuit_breaker(chain_id: ChainId) -> DispatchResult {
        let current_tvl = Self::get_tvl(&chain_id);
        let previous_tvl = Self::get_historical_tvl(&chain_id, 3600); // 1 hour ago

        let change_pct = Self::calculate_change_percentage(previous_tvl, current_tvl);
        let threshold = RateLimitConfig::<T>::get(&chain_id).max_tvl_change_pct;

        if change_pct > threshold as u128 {
            // Trigger circuit breaker
            Self::pause_bridge(chain_id)?;
            Self::deposit_event(Event::CircuitBreakerTriggered {
                chain_id,
                change_pct,
                threshold,
            });
            return Err(Error::<T>::CircuitBreakerActive.into());
        }

        Ok(())
    }
}
```

### 4. Custodian Multisig Security

**Leverage existing multisig infrastructure**:
```rust
use etrid_bridge_common::multisig::{MultiSigCustodian, PendingApproval};

impl<T: Config> Pallet<T> {
    /// Require custodian approval for large withdrawals
    pub fn process_withdrawal(
        origin: OriginFor<T>,
        withdrawal_id: WithdrawalId,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        let withdrawal = Withdrawals::<T>::get(withdrawal_id)
            .ok_or(Error::<T>::WithdrawalNotFound)?;

        // Check if amount requires custodian approval
        if withdrawal.amount >= T::CustodianApprovalThreshold::get() {
            // Use multisig approval workflow
            let custodian_set = CustodianSet::<T>::get()
                .ok_or(Error::<T>::NoCustodianSet)?;

            ensure!(custodian_set.is_custodian(&who), Error::<T>::NotCustodian);

            // Add approval and check threshold
            Self::add_custodian_approval(withdrawal_id, who)?;
        } else {
            // Small withdrawal, can be processed immediately
            Self::execute_withdrawal(withdrawal_id)?;
        }

        Ok(())
    }
}
```

### 5. Oracle Price Feed Security

**Multiple oracle sources for exchange rates**:
```rust
use etrid_bridge_common::oracle_adapter::{OracleAggregator, ExchangeRate};

impl<T: Config> Pallet<T> {
    /// Get verified exchange rate from multiple sources
    pub fn get_exchange_rate(
        from_chain: ChainId,
        to_asset: AssetId,
    ) -> Result<ExchangeRate, Error<T>> {
        let oracle = Self::oracle_aggregator();

        // Requires minimum 2 sources with <5% deviation
        let rate = oracle.get_aggregated_rate(
            &from_chain,
            &to_asset,
            Self::current_timestamp(),
        ).map_err(|_| Error::<T>::OracleUnavailable)?;

        // Verify rate is not stale (max 5 minutes old)
        ensure!(!rate.is_stale(300, Self::current_timestamp()), Error::<T>::StaleOracleRate);

        // Verify confidence is sufficient (min 80%)
        ensure!(rate.is_reliable(80), Error::<T>::LowOracleConfidence);

        Ok(rate)
    }
}
```

---

## Contract Specifications

### 1. State Verification Contract

```rust
#[frame_support::pallet]
pub mod state_verification {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of headers to store per chain
        #[pallet::constant]
        type MaxHeadersPerChain: Get<u32>;

        /// Maximum proof size
        #[pallet::constant]
        type MaxProofSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Verified block headers
    #[pallet::storage]
    pub type VerifiedHeaders<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ChainId,
        Blake2_128Concat,
        BlockNumber,
        BlockHeader,
        OptionQuery,
    >;

    /// Latest finalized block per chain
    #[pallet::storage]
    pub type FinalizedBlocks<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ChainId,
        BlockNumber,
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit block header with proof
        #[pallet::call_index(0)]
        #[pallet::weight(50_000)]
        pub fn submit_header(
            origin: OriginFor<T>,
            chain_id: ChainId,
            header: BlockHeader,
            proof: HeaderProof,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;

            // Verify relayer is authorized
            ensure!(
                Self::is_authorized_relayer(&relayer),
                Error::<T>::NotAuthorizedRelayer
            );

            // Verify proof
            Self::verify_header_proof(&chain_id, &header, &proof)?;

            // Store header
            VerifiedHeaders::<T>::insert(&chain_id, header.number, header.clone());

            // Update finalized if applicable
            Self::try_update_finalized(&chain_id, header.number)?;

            Self::deposit_event(Event::HeaderSubmitted {
                chain_id,
                block_number: header.number,
                relayer,
            });

            Ok(())
        }

        /// Verify transaction inclusion proof
        #[pallet::call_index(1)]
        #[pallet::weight(100_000)]
        pub fn verify_transaction(
            origin: OriginFor<T>,
            chain_id: ChainId,
            tx_hash: Hash,
            proof: TransactionProof,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get header for block containing transaction
            let header = VerifiedHeaders::<T>::get(&chain_id, proof.block_number)
                .ok_or(Error::<T>::HeaderNotFound)?;

            // Verify inclusion proof
            Self::verify_merkle_proof(&header, &tx_hash, &proof)?;

            // Check finality
            let finalized = FinalizedBlocks::<T>::get(&chain_id);
            ensure!(
                proof.block_number <= finalized,
                Error::<T>::InsufficientConfirmations
            );

            Self::deposit_event(Event::TransactionVerified {
                chain_id,
                tx_hash,
                block_number: proof.block_number,
            });

            Ok(())
        }
    }
}
```

### 2. Event Tracking Contract

```rust
#[frame_support::pallet]
pub mod event_tracking {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + state_verification::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Minimum confirmations per chain
        type MinConfirmations: Get<BTreeMap<ChainId, u32>>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Tracked deposit events
    #[pallet::storage]
    pub type TrackedDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        TransactionId,
        ExternalDeposit,
        OptionQuery,
    >;

    /// Processed transaction hashes (replay prevention)
    #[pallet::storage]
    pub type ProcessedTransactions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ChainId,
        Blake2_128Concat,
        Hash,
        ProcessedTxInfo,
        OptionQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Submit external deposit with proof
        #[pallet::call_index(0)]
        #[pallet::weight(150_000)]
        pub fn submit_deposit(
            origin: OriginFor<T>,
            deposit: ExternalDeposit,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;

            // Check replay
            Self::check_replay_attack(deposit.chain_id.clone(), deposit.tx_hash)?;

            // Verify proof using state verification contract
            state_verification::Pallet::<T>::verify_transaction(
                frame_system::RawOrigin::Signed(relayer.clone()).into(),
                deposit.chain_id.clone(),
                deposit.tx_hash,
                deposit.proof.clone(),
            )?;

            // Check confirmations
            let min_confirmations = T::MinConfirmations::get()
                .get(&deposit.chain_id)
                .cloned()
                .unwrap_or(6);

            ensure!(
                deposit.confirmations >= min_confirmations,
                Error::<T>::InsufficientConfirmations
            );

            // Generate transaction ID
            let tx_id = Self::generate_transaction_id(&deposit);

            // Store deposit
            TrackedDeposits::<T>::insert(tx_id, deposit.clone());

            // Mark as processed
            ProcessedTransactions::<T>::insert(
                &deposit.chain_id,
                &deposit.tx_hash,
                ProcessedTxInfo {
                    processed_at: Self::current_timestamp(),
                    processed_block: frame_system::Pallet::<T>::block_number().saturated_into(),
                    amount: deposit.amount,
                    recipient: deposit.to_address.clone(),
                },
            );

            Self::deposit_event(Event::DepositTracked {
                tx_id,
                chain_id: deposit.chain_id,
                amount: deposit.amount,
                recipient: deposit.to_address,
            });

            Ok(())
        }

        /// Track burn event (withdrawal from ETRID)
        #[pallet::call_index(1)]
        #[pallet::weight(100_000)]
        pub fn track_burn(
            origin: OriginFor<T>,
            burn: BurnEvent,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Verify burn actually occurred on ETRID
            Self::verify_burn_event(&burn)?;

            // Store for custodian processing
            PendingBurns::<T>::insert(burn.etrid_tx_hash, burn.clone());

            Self::deposit_event(Event::BurnTracked {
                etrid_tx_hash: burn.etrid_tx_hash,
                destination: burn.destination,
                amount: burn.amount,
            });

            Ok(())
        }
    }
}
```

### 3. Accounting & Reconciliation Contract

```rust
#[frame_support::pallet]
pub mod accounting {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + event_tracking::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Reconciliation frequency (in blocks)
        #[pallet::constant]
        type ReconciliationPeriod: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Total value locked per chain
    #[pallet::storage]
    pub type ChainTVLs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ChainId,
        ChainTVL,
        OptionQuery,
    >;

    /// Cross-chain transaction records
    #[pallet::storage]
    pub type CrossChainTransactions<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        TransactionId,
        CrossChainTransaction,
        OptionQuery,
    >;

    /// Hourly volume tracking
    #[pallet::storage]
    pub type HourlyVolume<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ChainId,
        Blake2_128Concat,
        u64, // Hour timestamp
        u128, // Volume
        ValueQuery,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Perform reconciliation for chain
        #[pallet::call_index(0)]
        #[pallet::weight(200_000)]
        pub fn reconcile_chain(
            origin: OriginFor<T>,
            chain_id: ChainId,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get external balance from oracle/relayer
            let external_balance = Self::fetch_external_balance(&chain_id)?;

            // Get ETRID minted amount
            let minted_balance = Self::get_minted_balance(&chain_id);

            // Calculate discrepancy
            let discrepancy = external_balance as i128 - minted_balance as i128;

            // Handle based on severity
            if discrepancy.abs() > 0 {
                Self::handle_discrepancy(chain_id.clone(), AssetId::Native, discrepancy)?;
            }

            // Update TVL
            ChainTVLs::<T>::mutate(&chain_id, |tvl| {
                if let Some(tvl) = tvl {
                    tvl.last_reconciled = Self::current_timestamp();
                    tvl.status = if discrepancy.abs() == 0 {
                        ReconciliationStatus::Balanced
                    } else {
                        ReconciliationStatus::MinorDiscrepancy
                    };
                }
            });

            Self::deposit_event(Event::ReconciliationCompleted {
                chain_id,
                external_balance,
                minted_balance,
                discrepancy,
            });

            Ok(())
        }

        /// Generate audit report
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn generate_audit_report(
            origin: OriginFor<T>,
            chain_id: ChainId,
            start_time: u64,
            end_time: u64,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            let report = Self::build_audit_report(&chain_id, start_time, end_time)?;

            Self::deposit_event(Event::AuditReportGenerated {
                chain_id,
                period_start: start_time,
                period_end: end_time,
                total_inbound: report.total_inbound,
                total_outbound: report.total_outbound,
                net_flow: report.net_flow,
            });

            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Automatic periodic reconciliation
        fn on_initialize(n: BlockNumberFor<T>) -> Weight {
            if (n % T::ReconciliationPeriod::get().into()).is_zero() {
                // Trigger reconciliation for all chains
                for (chain_id, _) in ChainTVLs::<T>::iter() {
                    let _ = Self::reconcile_chain(
                        frame_system::RawOrigin::Root.into(),
                        chain_id,
                    );
                }
            }
            Weight::zero()
        }
    }
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Months 1-2)

**Objective**: Build core infrastructure for external chain monitoring

#### Milestone 1.1: State Verification Infrastructure
- Implement block header submission and verification
- Build SPV proof validator for Bitcoin
- Build Merkle Patricia proof validator for Ethereum
- Create relayer registration and management system
- Estimated: 3 weeks

#### Milestone 1.2: Event Tracking System
- Implement deposit event detection and storage
- Build transaction proof verification pipeline
- Create replay attack prevention mechanism
- Implement confirmation depth tracking
- Estimated: 3 weeks

#### Milestone 1.3: Testing and Security Audit
- Comprehensive unit tests for state verification
- Integration tests with mock external chains
- Security audit of proof verification logic
- Estimated: 2 weeks

**Deliverables**:
- State verification pallet (fully tested)
- Event tracking pallet (fully tested)
- Relayer infrastructure
- Security audit report

### Phase 2: Accounting & Safety (Months 3-4)

**Objective**: Implement comprehensive accounting and safety mechanisms

#### Milestone 2.1: Accounting System
- Build TVL tracking system
- Implement cross-chain transaction ledger
- Create reconciliation engine
- Build discrepancy detection and handling
- Estimated: 3 weeks

#### Milestone 2.2: Safety Mechanisms
- Implement rate limiting
- Build circuit breaker system
- Add reorg detection and handling
- Create emergency pause mechanism
- Estimated: 3 weeks

#### Milestone 2.3: Custodian Integration
- Integrate with existing multisig system
- Add custodian approval workflow for large transactions
- Implement slashing for malicious relayers
- Estimated: 2 weeks

**Deliverables**:
- Accounting pallet (fully tested)
- Safety mechanisms (circuit breakers, rate limits)
- Custodian integration
- Reconciliation dashboard

### Phase 3: Multi-Chain Expansion (Months 5-6)

**Objective**: Extend support to multiple external chains

#### Milestone 3.1: Ethereum Support
- Ethereum receipt proof verification
- ERC-20 token tracking
- Smart contract event monitoring
- Estimated: 3 weeks

#### Milestone 3.2: Additional Chains
- Solana account state proofs
- Tron TRC-20 tracking
- BNB Chain support
- Polygon support
- Estimated: 4 weeks

#### Milestone 3.3: Oracle Integration
- Multi-source oracle aggregator
- Price feed validation
- Chainlink integration
- Estimated: 1 week

**Deliverables**:
- Support for 5+ external chains
- Oracle price feed system
- Comprehensive chain configuration

### Phase 4: Optimization & Production (Months 7-8)

**Objective**: Optimize for production and mainnet deployment

#### Milestone 4.1: Performance Optimization
- Optimize storage structures
- Reduce proof verification costs
- Batch processing for relayer submissions
- Estimated: 2 weeks

#### Milestone 4.2: Monitoring & Observability
- Real-time monitoring dashboard
- Alert system for custodians
- Metrics and analytics
- Estimated: 2 weeks

#### Milestone 4.3: Mainnet Preparation
- Final security audit
- Stress testing
- Documentation and runbooks
- Mainnet deployment plan
- Estimated: 4 weeks

**Deliverables**:
- Production-ready bridge tracking system
- Monitoring infrastructure
- Security audit report (final)
- Mainnet deployment guide

---

## Integration with Existing Systems

### Integration with Bitcoin Bridge

**Current Implementation** (`/Users/macbook/Desktop/etrid/05-multichain/bridges/protocols/bitcoin-bridge/src/lib.rs`):
- Already has deposit/withdrawal tracking
- Uses multisig custodian approval
- Has relayer authorization

**Enhancements**:
1. Add state verification contract for SPV proofs
2. Integrate accounting pallet for TVL tracking
3. Add comprehensive reconciliation
4. Implement circuit breakers

**Integration Points**:
```rust
impl<T: Config> pallet_bitcoin_bridge::Pallet<T> {
    /// Enhanced deposit submission with state verification
    pub fn deposit_btc_enhanced(
        origin: OriginFor<T>,
        deposit: ExternalDeposit,
    ) -> DispatchResult {
        // 1. Use state verification contract
        state_verification::Pallet::<T>::verify_transaction(
            origin.clone(),
            deposit.chain_id.clone(),
            deposit.tx_hash,
            deposit.proof.clone(),
        )?;

        // 2. Track in event tracking system
        event_tracking::Pallet::<T>::submit_deposit(
            origin.clone(),
            deposit.clone(),
        )?;

        // 3. Update accounting
        accounting::Pallet::<T>::record_inbound_transaction(
            deposit.chain_id,
            deposit.amount,
        )?;

        // 4. Execute existing mint logic
        Self::deposit_btc(origin, deposit.depositor, /* ... */)?;

        Ok(())
    }
}
```

### Integration with Ethereum Bridge

**Current Implementation** (`/Users/macbook/Desktop/etrid/05-multichain/bridges/protocols/ethereum-bridge/src/lib.rs`):
- Handles ETH and ERC-20 deposits
- Exchange rate management
- Fee collection

**Enhancements**:
1. Add Merkle Patricia proof verification
2. Track smart contract events
3. Multi-token reconciliation
4. Oracle price feed integration

### Integration with Existing Multisig System

**Existing System** (`/Users/macbook/Desktop/etrid/05-multichain/bridges/protocols/common/src/multisig.rs`):
- M-of-N custodian approval
- Duplicate prevention
- Threshold-based execution

**Integration**:
```rust
use etrid_bridge_common::multisig::{MultiSigCustodian, PendingApproval};

impl<T: Config> Pallet<T> {
    /// Use existing multisig for large withdrawals
    pub fn process_large_withdrawal(
        origin: OriginFor<T>,
        withdrawal_id: WithdrawalId,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Get custodian set from common module
        let custodian_set = CustodianSet::<T>::get()
            .ok_or(Error::<T>::NoCustodianSet)?;

        // Verify custodian
        ensure!(custodian_set.is_custodian(&who), Error::<T>::NotCustodian);

        // Add approval
        // ... (use existing workflow)

        Ok(())
    }
}
```

### Integration with Oracle System

**Existing System** (`/Users/macbook/Desktop/etrid/05-multichain/bridges/protocols/common/src/oracle_adapter.rs`):
- Multi-source price aggregation
- Exchange rate management
- Static rate oracle for stablecoins

**Integration**:
```rust
use etrid_bridge_common::oracle_adapter::{OracleAggregator, ExchangeRate};

impl<T: Config> Pallet<T> {
    /// Get exchange rate for cross-chain conversion
    pub fn convert_amount(
        from_chain: ChainId,
        to_chain: ChainId,
        amount: u128,
    ) -> Result<u128, Error<T>> {
        let oracle = T::OracleAggregator::get();

        let rate = oracle.get_aggregated_rate(
            &from_chain,
            &to_chain,
            Self::current_timestamp(),
        ).map_err(|_| Error::<T>::OracleUnavailable)?;

        Ok(rate.convert(amount))
    }
}
```

---

## Conclusion

This design provides a comprehensive bridge tracking system for ETRID that:

1. **Monitors External Chains**: Through relayers, SPV proofs, and light client verification
2. **Ensures Security**: Multi-layer verification, replay prevention, reorg handling
3. **Maintains Accuracy**: Real-time accounting, periodic reconciliation, discrepancy detection
4. **Native Implementation**: All wrapped tokens stay on ETRID PBCs
5. **Integrates Seamlessly**: Leverages existing multisig, oracle, and bridge infrastructure

### Key Advantages

- **No External Deployments**: All tracking happens on ETRID
- **Comprehensive Verification**: Multiple proof systems for different chains
- **Real-Time Accounting**: Immediate tracking of all cross-chain flows
- **Custodian Security**: M-of-N multisig for critical operations
- **Automated Reconciliation**: Regular balance checks and discrepancy detection

### Next Steps

1. Review and approve this design document
2. Begin Phase 1 implementation (Foundation)
3. Conduct security audit of state verification logic
4. Integrate with existing Bitcoin and Ethereum bridges
5. Expand to additional chains in Phase 3

---

**Document Version**: 1.0
**Created**: December 8, 2025
**Status**: Awaiting Review
**Reviewers**: Bridge Architecture Team, Security Team, Core Developers
