# EDSC-PBT Integration Gameplan

## Executive Summary

Based on the comprehensive `eddc-pbt:update.md` specification, this gameplan outlines the integration of the **Ëtrid Dollar Stablecoin (EDSC)** system with **Partition Burst Token (PBT)** architecture into the Ëtrid multichain ecosystem.

**Current Status**:
- ✅ **6 Core EDSC pallets** built and integrated into FlareChain runtime
- ⚠️ **EDSC-PBC dedicated chain** - structure exists but pallets not integrated
- ❌ **Additional modules** - checkpoint, circuit-breaker, reserve-oracle need to be built

---

## What is EDSC-PBT?

### EDSC (Ëtrid Dollar Stablecoin)
A **1:1 USD-pegged stablecoin** with:
- Multi-path redemption system (3 paths based on proof type)
- TWAP oracle-based pricing
- Dynamic fee mechanism to prevent arbitrage attacks
- 110-130% overcollateralization requirement
- Circuit breakers and safety controls

### PBT (Partition Burst Token)
Represents **partitioned assets on Partition Burst Chains (PBCs)**:
- EDSC operations isolated to dedicated PBC-EDSC chain
- Checkpoints sync state to FlareChain main chain
- ERC-1155 wrapped representations for institutional DeFi
- Proof-of-reserves anchored cross-chain

### Architecture Vision

```
┌─────────────────────────────────────────────────────────────┐
│                     FlareChain (Main Chain)                  │
│  ┌────────────────┐  ┌──────────────────┐  ┌──────────────┐ │
│  │ Reserve Vault  │  │ Custodian Registry│  │ Reserve      │ │
│  │ (On-chain      │  │ (Off-chain        │  │ Oracle       │ │
│  │  collateral)   │  │  reserves)        │  │ (Aggregator) │ │
│  └────────────────┘  └──────────────────┘  └──────────────┘ │
│           ▲                    ▲                    ▲         │
└───────────┼────────────────────┼────────────────────┼─────────┘
            │                    │                    │
            │ Checkpoint Proofs  │                    │
            │                    │                    │
┌───────────▼────────────────────▼────────────────────▼─────────┐
│                    PBC-EDSC (Dedicated Chain)                  │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │ EDSC Token   │  │ Receipts     │  │ Redemption Engine  │  │
│  │ (Mint/Burn)  │  │ (SBT)        │  │ (3-path system)    │  │
│  └──────────────┘  └──────────────┘  └────────────────────┘  │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │ Oracle       │  │ Checkpoint   │  │ Circuit Breaker    │  │
│  │ (TWAP)       │  │ Module       │  │ (Safety)           │  │
│  └──────────────┘  └──────────────┘  └────────────────────┘  │
└────────────────────────────────────────────────────────────────┘
            │
            │ Bridges
            ▼
┌───────────────────────────────────────────────────────────────┐
│          External Chains (HyperEVM, Ethereum, etc.)            │
│  ┌──────────────────────┐  ┌──────────────────────────────┐  │
│  │ Wrapped EDSC         │  │ PBT ERC-1155 Representation  │  │
│  │ (CCTP-style bridge)  │  │ (Institutional liquidity)    │  │
│  └──────────────────────┘  └──────────────────────────────┘  │
└───────────────────────────────────────────────────────────────┘
```

---

## Current State Assessment

### ✅ Completed Components

| Component | Location | Status |
|-----------|----------|--------|
| **pallet-edsc-token** | `/pallets/pallet-edsc-token/` | ✅ Built, tested, integrated in FlareChain |
| **pallet-edsc-receipts** | `/pallets/pallet-edsc-receipts/` | ✅ Built, tested, integrated in FlareChain |
| **pallet-edsc-redemption** | `/pallets/pallet-edsc-redemption/` | ✅ Built, tested, integrated in FlareChain |
| **pallet-edsc-oracle** | `/pallets/pallet-edsc-oracle/` | ✅ Built, tested, integrated in FlareChain |
| **pallet-reserve-vault** | `/pallets/pallet-reserve-vault/` | ✅ Built, tested, integrated in FlareChain |
| **pallet-custodian-registry** | `/pallets/pallet-custodian-registry/` | ✅ Built, tested, integrated in FlareChain |
| **FlareChain Runtime** | `/05-multichain/flare-chain/runtime/` | ✅ All EDSC pallets configured and compiling |
| **EDSC Architecture Docs** | `/EDSC_PALLET_ARCHITECTURE.md` | ✅ Complete documentation created |

### ⚠️ Partially Complete

| Component | Location | Status |
|-----------|----------|--------|
| **EDSC-PBC Chain** | `/05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/` | ⚠️ Directory structure exists, runtime skeleton created, pallets NOT integrated |

### ❌ Missing Components

| Component | Purpose | Priority |
|-----------|---------|----------|
| **pallet-edsc-checkpoint** | Posts state summaries to FlareChain every N blocks | HIGH |
| **pallet-circuit-breaker** | Emergency pause system for redemptions | HIGH |
| **pallet-reserve-oracle** | Aggregates reserve data from vault + custodians | MEDIUM |
| **PBT Bridge Contracts** | ERC-1155 wrapper for institutional liquidity | LOW |
| **HyperEVM Integration** | BitGo/Anchorage custody integration | LOW |
| **CCTP-style Bridge** | Cross-chain EDSC transfers | MEDIUM |

---

## Integration Phases

### Phase 1: Complete EDSC-PBC Dedicated Chain [PRIORITY: HIGH]

**Goal**: Get the dedicated PBC-EDSC chain running with all core pallets

**Tasks**:

1. **Integrate existing EDSC pallets into EDSC-PBC runtime**
   - Location: `/05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/runtime/`
   - Add dependencies to `Cargo.toml`:
     - pallet-edsc-token
     - pallet-edsc-receipts
     - pallet-edsc-redemption
     - pallet-edsc-oracle
   - Configure pallets in `runtime/src/lib.rs`
   - Add to `construct_runtime!` macro

2. **Build pallet-edsc-checkpoint**
   - Purpose: Periodically posts Merkle roots of EDSC state to FlareChain
   - Key features:
     - Checkpoint every N blocks (configurable, ~100 blocks = ~10 minutes)
     - Sign checkpoints with validator quorum (⅔ threshold)
     - Include: total supply, reserve ratio, redemption queue hash
     - Submit to FlareChain via XCM or custom bridge pallet
   - Dependencies: frame-system, sp-runtime, XCM
   - Location: `/pallets/pallet-edsc-checkpoint/`

3. **Build pallet-circuit-breaker**
   - Purpose: Automatic safety controls for redemptions
   - Key features:
     - Monitor redemption volume (per-hour, per-day caps)
     - Pause redemptions if reserve ratio < 105%
     - Throttle redemptions (50% capacity) if ratio < 110%
     - Pause if oracle variance > 5%
     - Governance override capability
   - Dependencies: pallet-edsc-redemption, pallet-edsc-oracle
   - Location: `/pallets/pallet-circuit-breaker/`

4. **Configure EDSC-PBC runtime parameters**
   ```rust
   // Redemption parameters
   MinRedemptionFee: 0.25%
   SafetyMultiplier: 1.2
   PerTxCap: 50,000 EDSC
   DailyCap: 0.5% of supply

   // Oracle parameters
   PrimaryTwapWindow: 24 hours (14,400 blocks)
   FallbackTwapWindow: 7 days (100,800 blocks)
   MinPriceSources: 5
   OutlierThreshold: 2%

   // Circuit breaker thresholds
   HourlyVolumeCap: 0.5% of supply
   ReserveRatioPause: 105%
   ReserveRatioThrottle: 110%
   OracleVarianceLimit: 5%
   ```

5. **Build and test EDSC-PBC runtime**
   ```bash
   cargo build --release -p edsc-pbc-runtime
   cargo test -p edsc-pbc-runtime
   ```

6. **Create EDSC-PBC collator node**
   - Location: `/05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/node/`
   - Based on existing PBC collator template
   - Configure as parachain to FlareChain relay

**Deliverables**:
- ✅ EDSC-PBC runtime compiles with all 6 pallets
- ✅ pallet-edsc-checkpoint functional
- ✅ pallet-circuit-breaker functional
- ✅ EDSC-PBC can run as standalone dev chain
- ✅ Unit tests pass for all pallets

**Estimated Time**: 2-3 weeks

---

### Phase 2: FlareChain Reserve Integration [PRIORITY: HIGH]

**Goal**: Complete the reserve management infrastructure on FlareChain

**Tasks**:

1. **Build pallet-reserve-oracle**
   - Purpose: Aggregates reserve data from multiple sources
   - Key features:
     - Queries pallet-reserve-vault for on-chain collateral value
     - Queries pallet-custodian-registry for off-chain attestations
     - Calculates total reserve value
     - Computes reserve ratio = total_reserves / edsc_supply
     - Publishes to pallet-edsc-redemption (via XCM to PBC-EDSC)
   - Location: `/pallets/pallet-reserve-oracle/`
   - Dependencies:
     - pallet-reserve-vault
     - pallet-custodian-registry
     - XCM (to send updates to PBC-EDSC)

2. **Integrate reserve-oracle into FlareChain runtime**
   - Add to FlareChain `Cargo.toml`
   - Configure in `runtime/src/lib.rs`
   - Set update frequency (every 600 blocks = ~1 hour)

3. **Set up XCM/checkpoint verification**
   - FlareChain receives checkpoints from PBC-EDSC
   - Validates signatures (⅔ quorum)
   - Stores checkpoint data for auditability
   - FlareChain sends reserve ratio updates to PBC-EDSC

4. **Create proof-of-reserves dashboard data**
   - Expose RPC endpoints:
     - `/reserve_vault/total_value`
     - `/custodian_registry/total_attested`
     - `/reserve_oracle/reserve_ratio`
     - `/edsc_checkpoint/latest`
   - JSON export for public dashboard

**Deliverables**:
- ✅ pallet-reserve-oracle functional
- ✅ Bi-directional communication: FlareChain ↔ PBC-EDSC
- ✅ Reserve ratio auto-updates on both chains
- ✅ RPC endpoints for dashboard

**Estimated Time**: 2-3 weeks

---

### Phase 3: Cross-Chain Bridge Infrastructure [PRIORITY: MEDIUM]

**Goal**: Enable EDSC to move between chains (PBC-EDSC ↔ FlareChain ↔ External)

**Tasks**:

1. **PBC-EDSC ↔ FlareChain bridge**
   - Built on existing pallet-bridge infrastructure
   - Lock EDSC on PBC-EDSC → mint on FlareChain
   - Burn on FlareChain → unlock on PBC-EDSC
   - 15-block confirmation delay for security
   - Merkle proof verification

2. **CCTP-style external bridges** (for HyperEVM, Ethereum, etc.)
   - Implement burn-and-mint pattern (like Circle CCTP V2)
   - Burn EDSC on source chain
   - Generate cryptographic proof
   - Mint equivalent EDSC on destination chain
   - Verify via on-chain proof validation
   - Reference: Circle CCTP architecture from document

3. **Bridge quotas and caps**
   - Max 10% of total EDSC supply bridged to any single external chain
   - Per-transaction caps (e.g., 100k EDSC)
   - Delay period before bridged EDSC becomes redeemable
   - Emergency pause capability

4. **Multi-sig bridge validators**
   - 5-of-9 multi-sig control
   - Monitoring alerts for unusual activity
   - DAO can freeze bridge if exploit detected

**Deliverables**:
- ✅ PBC-EDSC ↔ FlareChain bridge operational
- ✅ CCTP-style bridge contracts deployed
- ✅ Bridge monitoring and alerts set up

**Estimated Time**: 3-4 weeks

---

### Phase 4: Institutional Integration (HyperEVM) [PRIORITY: LOW]

**Goal**: Enable institutional custody and DeFi access via HyperEVM/Hyperliquid

**Tasks**:

1. **Deploy EDSC mirror on HyperEVM**
   - Solidity contract for EDSC representation
   - Integrated with Anchorage Digital custody
   - BitGo MPC attestation for reserves

2. **PBT ERC-1155 wrapper**
   - Wrap PBC-EDSC tokens as ERC-1155 on HyperEVM
   - Enable composability with institutional DeFi
   - Liquidity mining incentives

3. **Hyperliquid DEX integration**
   - Create EDSC/USDC liquidity pool
   - Create EDSC/ÉTR liquidity pool
   - Integrate Hyperliquid price feeds into pallet-edsc-oracle
   - Incentivize LPs with PBT emissions

4. **Regulatory compliance setup**
   - Payment stablecoin classification (U.S. definition)
   - KYC/AML for institutional on-ramps
   - Reserve attestation API for regulators
   - Quarterly third-party audits

**Deliverables**:
- ✅ EDSC available on HyperEVM
- ✅ Institutional custody via Anchorage
- ✅ Liquidity on Hyperliquid DEX
- ✅ Compliance documentation

**Estimated Time**: 4-6 weeks

---

### Phase 5: Testing & Stress Testing [PRIORITY: HIGH]

**Goal**: Validate system under adversarial and stress conditions

**Tasks**:

1. **Unit testing**
   - All pallet extrinsics
   - Edge cases (zero amounts, overflows, etc.)
   - Access control (unauthorized calls)
   - State transitions

2. **Integration testing**
   - Cross-pallet interactions (oracle → redemption, vault → redemption)
   - Path switching (reserve ratio triggers)
   - Circuit breaker activation
   - Checkpoint verification

3. **Stress testing scenarios** (from Mo11 in document)
   - **Bank run simulation**: 20% of supply redeemed in 1 hour
   - **Oracle manipulation**: Flash crash to $0.85
   - **Reserve depletion**: Reserve ratio drops to 95%
   - **Bridge attack**: Attempt double-spend across chains
   - **Custodian failure**: 2 of 5 custodians miss attestations
   - **Arbitrage loop**: Attempt to profit from peg deviation

4. **Security audit**
   - Third-party audit of all pallets
   - Focus on:
     - Arithmetic overflow/underflow
     - Access control
     - Oracle manipulation
     - Reentrancy (cross-pallet calls)
     - Economic attacks (fee bypasses, arbitrage loops)

5. **Public bug bounty**
   - Offer rewards for critical vulnerabilities
   - Testnet for white-hat testing

**Deliverables**:
- ✅ Full test suite passing
- ✅ Stress test report
- ✅ Security audit report
- ✅ Bug bounty program live

**Estimated Time**: 4-6 weeks

---

### Phase 6: Governance & Transparency [PRIORITY: MEDIUM]

**Goal**: Enable DAO governance and public transparency

**Tasks**:

1. **DAO governance integration**
   - Consensus Day parameter updates
   - Emergency governance multi-sig (3-of-5 Foundation members)
   - 72-hour timelock for non-emergency upgrades
   - Parameter vote: reserve ratio targets, fee ranges, caps

2. **Public dashboard**
   - Real-time reserve ratio display
   - Collateral composition breakdown
   - Redemption volume charts
   - Oracle price feeds
   - Custodian attestation status
   - Total supply and circulation

3. **Proof-of-reserves explorer**
   - Merkle proof verification UI
   - Per-vault balance verification
   - Custodian attestation history
   - Checkpoint audit trail

4. **Transparency reports**
   - Quarterly reserve attestations
   - Third-party audit reports (hash anchored on-chain)
   - Fee revenue breakdown
   - Redemption statistics

**Deliverables**:
- ✅ DAO controls operational
- ✅ Public dashboard live
- ✅ Proof-of-reserves explorer
- ✅ Quarterly reporting cadence

**Estimated Time**: 3-4 weeks

---

### Phase 7: AI Governance Agents [PRIORITY: MEDIUM-HIGH]

**Goal**: Implement AI-driven governance and monitoring with verifiable off-chain compute

**Background**: Based on Mo14 (AI Interoperability & Cross-Chain Protocol) and Mo16 (Protocol Operations & AI Governance), Ëtrid will integrate AI agents as active participants in governance, monitoring, and protocol operations—with cryptographic proof of their actions.

**Tasks**:

1. **AICP (AI Interoperability & Cross-Chain Protocol) Infrastructure**
   - Extend OpenDID to **OpenAIDID** (AI identity tokens)
   - Build **AI-Wallet Interface (AIWI)** for GPT-class agents
   - Enable AI agents to hold and transact ÉTR/EDSC
   - Implement **DETR-AI transport layer** for AI-to-AI communication
   - Create **Cross-Chain AI Bridge (CAIB)** for multi-chain AI operations

2. **Verifiable Off-Chain Compute Framework**

   Three proof classes for AI actions:

   **A) TEE Attested Compute** (Production-ready now)
   - Deploy **Trusted Execution Environments** (Intel SGX, AMD SEV)
   - AI logic runs inside secure enclaves
   - Generate remote attestations with signatures
   - Use for: monitoring, transaction routing, classification

   **B) zkVM / zkML Proofs** (2025-2026 deployment)
   - Use zero-knowledge VMs (Risc-Zero, SP1)
   - Generate succinct proofs for deterministic logic
   - Use for: policy checks, threshold logic, aggregations

   **C) MPC-zk Hybrid** (Privacy-preserving analytics)
   - Multi-party computation for sensitive data
   - Aggregate via zk to compact proof
   - Use for: compliance scoring, AML flags, custody reconciliations

3. **Build Core AI Governance Pallets**

   **pallet-ai-authority** (Substrate)
   - Location: `/pallets/pallet-ai-authority/`
   - Purpose: Register and manage AI agents
   - Features:
     - AI agent registration with staking requirement
     - Delegation management (human → AI)
     - Permission scoping (spend limits, function allowlists)
     - Slashing for malicious behavior

   **pallet-attestation-verifier** (Substrate)
   - Location: `/pallets/pallet-attestation-verifier/`
   - Purpose: Verify TEE/zk proofs from AI agents
   - Features:
     - TEE remote attestation verification (SGX, SEV)
     - zkSNARK/zkSTARK proof verification
     - Proof registry for audit trail
     - Multi-TEE quorum support

   **pallet-poc-oracle** (Proof-of-Computation Oracle)
   - Location: `/pallets/pallet-poc-oracle/`
   - Purpose: Aggregate AI computation results and feed to other pallets
   - Features:
     - Receive verified AI outputs
     - Feed results to redemption engine, circuit breaker, etc.
     - Rate limiting and spam prevention
     - DAO-controlled oracle source management

4. **AI Governance Integration with Consensus Day**

   **AI Director Seats**:
   - Reserve 3-5 seats on Consensus Day council for AI agents
   - AI agents must:
     - Post significant ÉTR stake (10M+ ÉTR)
     - Provide TEE attestations for all votes
     - Submit zk proofs for policy proposals
     - Operate under human oversight (2-key execution)

   **Two-Key Execution Pattern**:
   ```rust
   // For high-impact operations, require AI attestation + human co-signature
   pub fn ai_gated_parameter_update(
       origin: OriginFor<T>,
       ai_attestation: ProofRecord<T>,
       human_cosig: MultiSignature,
       new_param: ParameterValue,
   ) -> DispatchResult {
       // Verify AI proof
       ensure!(
           pallet_attestation_verifier::Pallet::<T>::is_verified(&ai_attestation),
           Error::<T>::InvalidAIAttestation
       );

       // Verify human co-signature from council
       ensure!(
           Self::verify_council_cosig(&human_cosig),
           Error::<T>::InvalidHumanCosig
       );

       // Apply timelock (72 hours)
       Self::schedule_parameter_update(new_param, 72 * 3600)?;

       Ok(())
   }
   ```

5. **AI Monitoring and Analytics Agents**

   Deploy AI agents for:

   **Reserve Monitoring Agent**:
   - Continuously monitors reserve ratio
   - Alerts when ratio < 115%
   - Proposes buyback actions when EDSC < $1
   - Provides TEE attestations for all alerts

   **Oracle Health Agent**:
   - Monitors price feed variance
   - Detects outliers and manipulation attempts
   - Auto-triggers circuit breakers if variance > 5%
   - Generates zk proofs of oracle health metrics

   **Redemption Analytics Agent**:
   - Analyzes redemption patterns
   - Predicts bank run scenarios
   - Recommends dynamic fee adjustments
   - All recommendations backed by MPC-zk proofs

   **Governance Proposal Agent**:
   - Analyzes treasury health
   - Proposes parameter optimizations
   - Simulates impact of proposals before submission
   - Publishes simulation results with zk proofs

6. **DETR-AI Transport Layer**

   Extend DETRP2P with AI-specific features:
   ```rust
   // AI-to-AI messaging over DETR-AI
   pub struct AIMessage {
       pub from_aidid: Vec<u8>,
       pub to_aidid: Vec<u8>,
       pub intent_hash: [u8; 32],
       pub payload: Vec<u8>,
       pub proof: ProofRecord,
       pub signature: Signature,
   }

   // Cross-chain AI agent routing
   pub trait AITransport {
       fn send_ai_message(msg: AIMessage) -> Result<(), Error>;
       fn receive_ai_message() -> Result<AIMessage, Error>;
       fn verify_ai_intent(msg: &AIMessage) -> bool;
   }
   ```

7. **Safety and Governance Rules**

   **Stake & Slash**:
   - Minimum stake: 1M ÉTR for operational agents, 10M ÉTR for governance agents
   - Slashing: 10% for invalid proofs, 50% for malicious actions, 100% for critical exploits

   **Timelock & Veto**:
   - All AI-proposed parameter changes: 72-hour timelock
   - DAO can veto any AI action within timelock period
   - Emergency actions require 3-of-5 multi-sig from human council

   **Quorum Diversity**:
   - Critical operations require 2+ distinct TEE vendors OR 1 TEE + 1 zk proof
   - Prevents single-vendor trust dependency

   **Human Oversight Sunset**:
   - Initial phase: All AI actions require human co-signature
   - Phase 2 (Q2 2026): Reduce to 50% human oversight for low-risk ops
   - Phase 3 (Q4 2026): Reduce to 10% for proven AI agents
   - Never eliminate human oversight for critical ops (treasury, emergency, upgrades)

**Deliverables**:
- ✅ pallet-ai-authority functional
- ✅ pallet-attestation-verifier supporting TEE and zk proofs
- ✅ pallet-poc-oracle aggregating AI outputs
- ✅ DETR-AI transport layer operational
- ✅ 3+ AI monitoring agents deployed with TEE attestations
- ✅ AI Director seats on Consensus Day
- ✅ Two-key execution pattern implemented
- ✅ Safety rules and slashing mechanisms active

**Estimated Time**: 6-8 weeks

---

## Technical Modules to Build

### Priority 1: Core Infrastructure

#### 1. pallet-edsc-checkpoint

**Location**: `/pallets/pallet-edsc-checkpoint/`

**Purpose**: Posts state commitments from PBC-EDSC to FlareChain

**Key Features**:
- Merkle tree of EDSC state (supply, reserves, redemptions)
- Validator signatures (⅔ quorum)
- Periodic checkpoints (every 100 blocks)
- XCM integration for cross-chain posting

**Storage**:
```rust
#[pallet::storage]
pub type LatestCheckpoint<T> = StorageValue<_, Checkpoint<T::BlockNumber>>;

#[pallet::storage]
pub type CheckpointHistory<T> = StorageMap<_, Blake2_128Concat, T::BlockNumber, Checkpoint<T::BlockNumber>>;
```

**Extrinsics**:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn submit_checkpoint(
        origin: OriginFor<T>,
        merkle_root: [u8; 32],
        total_supply: u128,
        reserve_ratio: FixedU128,
        signatures: Vec<ValidatorSignature>,
    ) -> DispatchResult;

    pub fn verify_checkpoint(
        origin: OriginFor<T>,
        checkpoint_height: T::BlockNumber,
    ) -> DispatchResult;
}
```

**Config Parameters**:
```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type CheckpointFrequency: Get<u32>;  // blocks between checkpoints
    type ValidatorQuorum: Get<Permill>;  // 66% = 660_000
    type MaxSignatures: Get<u32>;        // max validators
}
```

---

#### 2. pallet-circuit-breaker

**Location**: `/pallets/pallet-circuit-breaker/`

**Purpose**: Automatic safety controls for EDSC redemptions

**Key Features**:
- Volume-based caps (hourly, daily)
- Reserve ratio thresholds
- Oracle variance monitoring
- Automatic pause/throttle
- Governance override

**Storage**:
```rust
#[pallet::storage]
pub type RedemptionVolume<T> = StorageValue<_, VolumeTracker<T::BlockNumber>>;

#[pallet::storage]
pub type CircuitBreakerStatus<T> = StorageValue<_, BreakerStatus>;

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum BreakerStatus {
    Normal,
    Throttled,  // 50% capacity
    Paused,     // no redemptions
}
```

**Hooks**:
```rust
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_finalize(n: BlockNumberFor<T>) {
        // Check volume caps
        Self::check_hourly_cap(n);
        Self::check_daily_cap(n);

        // Check reserve ratio
        Self::check_reserve_thresholds();

        // Check oracle health
        Self::check_oracle_variance();

        // Update breaker status
        Self::update_breaker_status();
    }
}
```

**Extrinsics**:
```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn emergency_pause(origin: OriginFor<T>) -> DispatchResult;
    pub fn resume_normal(origin: OriginFor<T>) -> DispatchResult;
    pub fn override_throttle(origin: OriginFor<T>) -> DispatchResult;
}
```

**Integration with pallet-edsc-redemption**:
```rust
// In pallet-edsc-redemption, before processing redemption:
if pallet_circuit_breaker::Pallet::<T>::is_paused() {
    return Err(Error::<T>::RedemptionsPaused.into());
}

if pallet_circuit_breaker::Pallet::<T>::is_throttled() {
    // Apply 50% reduction to redemption amount
    amount = amount / 2;
}

// Record redemption volume
pallet_circuit_breaker::Pallet::<T>::record_redemption(amount)?;
```

---

#### 3. pallet-reserve-oracle

**Location**: `/pallets/pallet-reserve-oracle/`

**Purpose**: Aggregates reserve data from vault + custodians

**Key Features**:
- Queries on-chain vault balances
- Aggregates custodian attestations
- Calculates total reserve value
- Computes reserve ratio
- Publishes to PBC-EDSC

**Storage**:
```rust
#[pallet::storage]
pub type TotalReserveValue<T> = StorageValue<_, u128>;

#[pallet::storage]
pub type ReserveRatio<T> = StorageValue<_, FixedU128>;

#[pallet::storage]
pub type LastUpdate<T> = StorageValue<_, T::BlockNumber>;
```

**Off-chain Worker**:
```rust
impl<T: Config> Pallet<T> {
    fn offchain_worker(n: BlockNumberFor<T>) {
        if (n % T::UpdateFrequency::get()).is_zero() {
            // Query vault
            let vault_value = pallet_reserve_vault::Pallet::<T>::total_value();

            // Query custodians
            let custodian_value = pallet_custodian_registry::Pallet::<T>::total_attested();

            // Get EDSC supply from PBC-EDSC (via XCM query)
            let edsc_supply = Self::query_edsc_supply();

            // Calculate ratio
            let total = vault_value + custodian_value;
            let ratio = FixedU128::from_rational(total, edsc_supply);

            // Submit transaction to update on-chain
            Self::submit_reserve_update(total, ratio);
        }
    }
}
```

**XCM Integration**:
```rust
// Send reserve ratio to PBC-EDSC
pub fn send_reserve_ratio_to_pbc(ratio: FixedU128) -> DispatchResult {
    let message = Xcm(vec![
        Transact {
            origin_kind: OriginKind::Native,
            require_weight_at_most: Weight::from_parts(1_000_000_000, 64 * 1024),
            call: pallet_edsc_redemption::Call::do_update_reserve_ratio { ratio }.encode().into(),
        },
    ]);

    send_xcm::<T::XcmRouter>(
        (Parent, Parachain(EDSC_PBC_PARA_ID)).into(),
        message,
    )?;

    Ok(())
}
```

---

## Deployment Roadmap

### Testnet Deployment (Weeks 1-8)

1. **Week 1-2**: Integrate EDSC pallets into EDSC-PBC runtime
2. **Week 3-4**: Build pallet-edsc-checkpoint and pallet-circuit-breaker
3. **Week 5-6**: Deploy EDSC-PBC testnet, connect to FlareChain testnet
4. **Week 7-8**: Build pallet-reserve-oracle, test bi-directional communication

### Integration Testing (Weeks 9-12)

1. **Week 9**: Integration tests for all cross-pallet interactions
2. **Week 10**: Stress testing (bank run, oracle manipulation, etc.)
3. **Week 11**: Security audit preparation
4. **Week 12**: Third-party security audit

### Mainnet Preparation (Weeks 13-16)

1. **Week 13**: Fix audit findings, final testing
2. **Week 14**: Deploy to mainnet staging environment
3. **Week 15**: Initialize reserves, register custodians
4. **Week 16**: Gradual rollout (10% → 50% → 100% capacity)

### Post-Launch (Weeks 17+)

1. **Week 17-18**: Monitor system health, adjust parameters
2. **Week 19-20**: Bridge integration (HyperEVM, Ethereum)
3. **Week 21-22**: Institutional gateway enablement
4. **Week 23-24**: Public dashboard and proof-of-reserves explorer

---

## Risk Mitigation

### Technical Risks

| Risk | Mitigation |
|------|------------|
| **Oracle manipulation** | Multi-source TWAP, outlier removal, 24h windows |
| **Arbitrage attacks** | Dynamic fees, redemption caps, circuit breakers |
| **Reserve depletion** | 110-130% overcollateralization, daily caps |
| **Bridge exploits** | Multi-sig control, 15-block delay, chain quotas |
| **Smart contract bugs** | Third-party audits, bug bounty, gradual rollout |

### Economic Risks

| Risk | Mitigation |
|------|------------|
| **Bank run** | Redemption queues, circuit breakers, reserve buffers |
| **Peg deviation** | Automated buybacks, dynamic fees, liquidity incentives |
| **Collateral volatility** | Diversified assets, conservative haircuts |
| **Custodian failures** | Bonding requirements, slashing, redundancy |

### Regulatory Risks

| Risk | Mitigation |
|------|------------|
| **Stablecoin classification** | Position as "payment stablecoin" (U.S. definition) |
| **Reserve requirements** | Maintain 110%+ ratio, quarterly audits |
| **AML/KYC requirements** | Institutional gateway with compliance |
| **Securities classification** | Governance token (ÉTR) separate from EDSC |

---

## Success Metrics

### Phase 1 (EDSC-PBC Launch)
- ✅ EDSC-PBC runtime compiles and runs
- ✅ All 6+ pallets functional
- ✅ Checkpoints posting to FlareChain every 100 blocks
- ✅ Circuit breakers activate correctly during stress tests

### Phase 2 (Reserve Integration)
- ✅ Reserve ratio auto-updates every hour
- ✅ Ratio stays above 110% for 30 consecutive days
- ✅ Custodian attestations 100% compliant

### Phase 3 (Cross-Chain Bridges)
- ✅ EDSC bridges to 3+ external chains
- ✅ Zero bridge exploits in first 90 days
- ✅ <1% of total supply on any single external chain

### Phase 4 (Institutional Integration)
- ✅ EDSC available on HyperEVM
- ✅ $10M+ liquidity on Hyperliquid DEX
- ✅ 2+ institutional custodians onboarded

### Phase 5 (Testing & Audit)
- ✅ Zero critical vulnerabilities in audit
- ✅ All stress test scenarios pass
- ✅ Bug bounty program yields no critical bugs

### Phase 6 (Governance & Transparency)
- ✅ Public dashboard live with <5s latency
- ✅ Quarterly audit reports published
- ✅ DAO successfully votes on parameter update

---

## Resource Requirements

### Development Team

| Role | Time Commitment | Duration |
|------|----------------|----------|
| **Substrate Runtime Engineer** | Full-time | 16 weeks |
| **Smart Contract Developer** (Solidity) | Part-time (50%) | 8 weeks |
| **DevOps Engineer** | Part-time (50%) | 16 weeks |
| **Security Auditor** (external) | Full-time | 2 weeks |
| **QA/Testing Engineer** | Full-time | 8 weeks |

### Infrastructure

| Component | Cost | Purpose |
|-----------|------|---------|
| **Testnet Nodes** (5 validators) | Cloud hosting | Testing EDSC-PBC |
| **Mainnet Nodes** (10+ validators) | Cloud hosting | Production EDSC-PBC |
| **Oracle Infrastructure** | API costs | Price feeds (Binance, Coinbase, etc.) |
| **Dashboard Hosting** | Cloud hosting | Public transparency portal |
| **Security Audit** | $50k-$100k | Third-party audit |

### Initial Reserves

| Asset Type | Amount | Purpose |
|------------|--------|---------|
| **USDC/USDT** | $1.1M - $1.3M | 110-130% backing for initial EDSC supply |
| **BTC/ETH** | $200k - $500k | Diversified collateral |
| **ÉTR Treasury** | 10M ÉTR | Buyback fund for peg defense |

---

## Implementation Priority

### Immediate (Weeks 1-4)
1. ✅ Integrate EDSC pallets into EDSC-PBC runtime
2. ✅ Build pallet-edsc-checkpoint
3. ✅ Build pallet-circuit-breaker
4. ✅ Deploy EDSC-PBC testnet

### High Priority (Weeks 5-12)
1. Build pallet-reserve-oracle
2. Set up XCM communication FlareChain ↔ PBC-EDSC
3. Integration testing
4. Stress testing
5. Security audit

### Medium Priority (Weeks 13-20)
1. Mainnet deployment
2. Bridge infrastructure
3. Public dashboard
4. Governance integration

### Low Priority (Weeks 21+)
1. HyperEVM integration
2. Institutional custody (Anchorage/BitGo)
3. External DEX integrations
4. Additional compliance features

---

## Next Steps

### This Week
1. ✅ Review and approve this gameplan
2. ⬜ Set up development branch: `feat/edsc-pbc-integration`
3. ⬜ Create GitHub issues for each pallet to build
4. ⬜ Assign resources and timeline

### Next 2 Weeks
1. ⬜ Integrate 6 EDSC pallets into EDSC-PBC runtime
2. ⬜ Start building pallet-edsc-checkpoint
3. ⬜ Start building pallet-circuit-breaker
4. ⬜ Set up testnet infrastructure

### Month 1 Goal
- ✅ EDSC-PBC runtime compiling with all pallets
- ✅ Checkpoints posting to FlareChain testnet
- ✅ Circuit breakers functional
- ✅ Begin integration testing

---

## Questions Resolved ✅

1. **Parachain vs Standalone**: ✅ **RESOLVED**
   - EDSC-PBC will run **like other PBC chains** but with native integration to Ëtrid
   - Similar architecture to existing PBCs (BTC-PBC, ETH-PBC, etc.)
   - Custom bridge/checkpoint system for FlareChain communication

2. **XCM Version**: ✅ **RESOLVED**
   - XCM should be **in coherence with DETRP2P** (Ëtrid's native peer-to-peer protocol)
   - Integrate DETR-AI transport layer for AI agent interoperability
   - Use DETRP2P messaging infrastructure where applicable

3. **Oracle Sources**: ✅ **RESOLVED**
   - Use **authoritative status oracles** (those that make sense for production)
   - Prioritize: Binance, Coinbase, Kraken, Bitstamp, Gemini (5 sources minimum)
   - Add Uniswap/PancakeSwap/Curve DEX TWAPs as secondary sources
   - Integrate Hyperliquid price feeds once HyperEVM is live

4. **Custodian Onboarding**: ✅ **RESOLVED**
   - **HyperEVM/Anchorage Digital** as primary institutional custodian
   - BitGo MPC attestation for reserve proofs
   - Target 3-5 custodians for diversification

5. **Initial Supply**: ✅ **RESOLVED**
   - **Total supply: 50 billion EDSC**
   - **Initial circulation: 5 billion EDSC** (10%)
   - **Locked reserve: 45 billion EDSC** (90%)
   - Locked supply released **only when capital market cap demands** additional issuance
   - Release mechanism controlled by Consensus Day governance
   - Requires proof of adequate reserve backing before unlock

6. **Governance**: ✅ **RESOLVED**
   - **Consensus Day controls all** final decisions
   - EDSC can have its own **discussions and votes** (community governance)
   - All EDSC parameter changes must be **ratified by Consensus Day**
   - EDSC governance acts as a **sub-council under Consensus Day**
   - Emergency actions require Consensus Day multi-sig approval

---

## Conclusion

The EDSC-PBT integration represents a **comprehensive stablecoin infrastructure** with:
- ✅ **6 core pallets already built** (token, receipts, redemption, oracle, vault, custodian)
- ⚠️ **6 additional pallets needed** (checkpoint, circuit-breaker, reserve-oracle, ai-authority, attestation-verifier, poc-oracle)
- 🎯 **Clear 24-week roadmap** to mainnet deployment with AI governance
- 🔒 **Robust safety mechanisms** (dynamic fees, circuit breakers, overcollateralization)
- 🌐 **Cross-chain capable** (PBC-EDSC ↔ FlareChain ↔ External chains)
- 🤖 **AI-driven governance** (verifiable off-chain compute, TEE attestations, zk proofs)

**Key Innovations**:
1. **50 billion total supply** with 45 billion locked (demand-controlled release)
2. **Consensus Day supreme governance** with EDSC sub-council
3. **DETRP2P-coherent cross-chain** messaging
4. **AI Director seats** on Consensus Day with verifiable compute proofs
5. **Multi-path redemption** system preventing arbitrage attacks

**Recommendation**: Start with **Phase 1** immediately to get EDSC-PBC operational, then proceed sequentially through phases based on priority.

**Estimated Total Timeline**:
- **Core EDSC functionality**: 16-20 weeks
- **With AI governance**: 24-28 weeks
- **Full production deployment**: 30-32 weeks (including audits and gradual rollout)

**Estimated Development Cost**:
- **EDSC core**: $150k-$250k (team + infrastructure + audits)
- **AI governance**: $100k-$150k (AI infrastructure + TEE/zk setup + additional audits)
- **Total**: $250k-$400k

---

**Document Version**: 1.0
**Created**: 2025-10-20
**Author**: Claude Code
**Status**: Ready for Eoj's review and approval
