# ASF Consensus Service Layer Design

## Architecture Overview

The ASF service layer will integrate Ëtrid's FODDoS ASF consensus with Substrate's service architecture, replacing AURA while maintaining compatibility with GRANDPA finality.

## Components to Implement

### 1. Runtime API (`sp-consensus-asf`)
**Purpose:** Define the runtime interface for ASF consensus queries

**Traits:**
```rust
sp_api::decl_runtime_apis! {
    pub trait AsfApi<AuthorityId: Codec> {
        /// Get current PPFA committee
        fn committee() -> Vec<AuthorityId>;

        /// Get current PPFA index
        fn ppfa_index() -> u32;

        /// Get adaptive slot duration
        fn slot_duration() -> SlotDuration;

        /// Check if validator should propose
        fn should_propose(validator: AuthorityId) -> bool;
    }
}
```

### 2. Client Service (`sc-consensus-asf`)
**Purpose:** Substrate client-side consensus implementation

**Key Functions:**
- `import_queue()` - Creates ASF-compatible block import queue
- `start_asf()` - Starts ASF block authoring worker
- `AsfWorker` - Background task for PPFA block production

**Architecture:**
```
┌─────────────────────────────────────────────────┐
│          Substrate Service Layer                │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌──────────────┐        ┌──────────────┐     │
│  │ Import Queue │───────▶│ Block Import │     │
│  └──────────────┘        └──────────────┘     │
│         │                        │             │
│         │                        │             │
│         ▼                        ▼             │
│  ┌──────────────┐        ┌──────────────┐     │
│  │   Verifier   │        │   Executor   │     │
│  └──────────────┘        └──────────────┘     │
│         │                        │             │
│         └────────┬───────────────┘             │
│                  │                             │
│                  ▼                             │
│          ┌──────────────┐                      │
│          │   Runtime    │                      │
│          │ (pallet-     │                      │
│          │  consensus)  │                      │
│          └──────────────┘                      │
│                                                 │
│  ┌──────────────────────────────────────────┐  │
│  │         ASF Worker (Block Authoring)     │  │
│  ├──────────────────────────────────────────┤  │
│  │                                          │  │
│  │  1. Query PPFA index from runtime       │  │
│  │  2. Check if we're current proposer     │  │
│  │  3. Wait for slot timing                │  │
│  │  4. Build block with transactions       │  │
│  │  5. Sign and propose                    │  │
│  │  6. Handle Ant blocks if needed         │  │
│  │                                          │  │
│  └──────────────────────────────────────────┘  │
│                                                 │
└─────────────────────────────────────────────────┘
```

### 3. Inherent Data Providers
**Purpose:** Provide slot timing and proposer info to blocks

**Providers:**
- `AsfInherentDataProvider` - Slot number and duration
- `ProposerInherentDataProvider` - PPFA proposer information

### 4. Block Verifier
**Purpose:** Validate blocks against ASF rules

**Checks:**
- Proposer is in current PPFA committee
- Proposer matches expected PPFA index for slot
- Block signature is valid
- Slot timing is correct
- Committee rotation at epoch boundaries

## Integration Points

### With Existing Ëtrid Crates

**`pallet-consensus`:**
- Runtime storage queries for committee, PPFA index
- On-chain state for slot duration, network health
- Validator registration and stake tracking

**`block-production` crate:**
- Reuse `ProposerSelector` for committee logic
- Reuse `SlotTiming` for adaptive slots
- Reuse `BlockValidator` for validation

**`asf-algorithm` crate:**
- HotStuff consensus phases
- Certificate management
- Finality calculations

**`validator-management` crate:**
- Committee management
- Health monitoring
- Reward distribution

### With Substrate

**`sc_consensus::BlockImport`:**
- Wrap GRANDPA block import
- Add ASF-specific verification

**`sc_consensus::ImportQueue`:**
- Use manual or basic queue
- Add ASF block verifier

**`sp_consensus::Environment`:**
- Proposer factory
- Transaction pool access
- Block builder

## Data Flow

### Block Import Flow

```
Network Block Received
         │
         ▼
  ┌─────────────┐
  │Import Queue │
  └─────────────┘
         │
         ▼
  ┌─────────────┐
  │  Verifier   │──────▶ Check proposer in committee
  │             │──────▶ Verify signature
  │             │──────▶ Validate slot timing
  └─────────────┘
         │
         ▼
  ┌─────────────┐
  │GRANDPA Import│──────▶ Finality processing
  └─────────────┘
         │
         ▼
  ┌─────────────┐
  │   Runtime   │──────▶ Execute block
  │  Execution  │──────▶ Update pallet-consensus state
  └─────────────┘
```

### Block Authoring Flow

```
ASF Worker Loop
       │
       ▼
┌──────────────┐
│ Query Runtime│──────▶ Get PPFA index
│              │──────▶ Get committee
│              │──────▶ Get slot duration
└──────────────┘
       │
       ▼
┌──────────────┐
│Am I Proposer?│
└──────────────┘
       │ Yes
       ▼
┌──────────────┐
│Wait for Slot │
└──────────────┘
       │
       ▼
┌──────────────┐
│ Build Block  │──────▶ Select transactions
│              │──────▶ Create header
│              │──────▶ Set PPFA index
└──────────────┘
       │
       ▼
┌──────────────┐
│ Sign & Seal  │──────▶ Sign with validator key
└──────────────┘
       │
       ▼
┌──────────────┐
│   Propose    │──────▶ Broadcast to network
└──────────────┘
       │
       ▼
┌──────────────┐
│Advance Index │──────▶ Runtime call via extrinsic
└──────────────┘
```

## Key Differences from AURA

| Feature | AURA | ASF |
|---------|------|-----|
| **Proposer Selection** | Round-robin by slot | PPFA committee-based |
| **Slot Duration** | Fixed | Adaptive (6-18s based on health) |
| **Validator Set** | All authorities rotate | Committee of 21 validators |
| **Finality** | GRANDPA | GRANDPA (same) |
| **Epochs** | Fixed epoch duration | 2400 block epochs with rotation |
| **Secondary Blocks** | None | Ant blocks (6-level depth) |
| **Consensus Algorithm** | Simple slot-based | HotStuff 4-phase BFT |

## Implementation Plan

### Phase 1: Runtime API (Week 1, Day 1-2)
- [ ] Create `primitives/consensus-asf` crate
- [ ] Define `AsfApi` trait
- [ ] Add runtime API implementation to PBC runtimes
- [ ] Test API queries

### Phase 2: Inherent Providers (Week 1, Day 3)
- [ ] Create `AsfInherentDataProvider`
- [ ] Implement slot calculation
- [ ] Implement proposer data encoding
- [ ] Test inherent generation

### Phase 3: Block Verifier (Week 1, Day 4-5)
- [ ] Create `AsfVerifier`
- [ ] Implement proposer verification
- [ ] Implement signature verification
- [ ] Implement timing verification
- [ ] Test verification logic

### Phase 4: Import Queue (Week 2, Day 1-2)
- [ ] Create `import_queue()` function
- [ ] Wire up verifier
- [ ] Wire up block import
- [ ] Test block import flow

### Phase 5: Block Authoring Worker (Week 2, Day 3-5)
- [ ] Create `AsfWorker`
- [ ] Implement proposer checking
- [ ] Implement slot timing
- [ ] Implement block building
- [ ] Implement signing
- [ ] Test authoring

### Phase 6: Service Integration (Week 3, Day 1-2)
- [ ] Create `start_asf()` function
- [ ] Integrate with TaskManager
- [ ] Wire up keystore
- [ ] Wire up network
- [ ] Test full service

### Phase 7: Collator Integration (Week 3, Day 3-5)
- [ ] Update btc-pbc-collator service.rs
- [ ] Replace AURA with ASF
- [ ] Test single collator
- [ ] Deploy to all 12 collators
- [ ] Network testing

### Phase 8: Production Hardening (Week 4)
- [ ] Error handling
- [ ] Logging and metrics
- [ ] Recovery mechanisms
- [ ] Performance optimization
- [ ] Documentation

## File Structure

```
09-consensus/
├── primitives/
│   └── consensus-asf/          # sp-consensus-asf
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # AsfApi trait
│           ├── inherents.rs    # Inherent data types
│           └── digests.rs      # Pre-runtime digests
│
└── client/
    └── consensus-asf/          # sc-consensus-asf
        ├── Cargo.toml
        └── src/
            ├── lib.rs          # Public API
            ├── import_queue.rs # Import queue creation
            ├── verifier.rs     # Block verification
            ├── worker.rs       # Block authoring worker
            ├── aux_schema.rs   # Auxiliary storage
            └── inherents.rs    # Inherent data providers
```

## Dependencies

### New Crates to Create
- `sp-consensus-asf` - Runtime API and types
- `sc-consensus-asf` - Service implementation

### Existing Dependencies
- `pallet-consensus` - Runtime state
- `block-production` - Block authoring logic
- `asf-algorithm` - Consensus algorithm
- `validator-management` - Committee management

### Substrate Dependencies
- `sc-consensus` - Base consensus traits
- `sc-client-api` - Client interfaces
- `sp-api` - Runtime API macros
- `sp-runtime` - Runtime types
- `sp-consensus` - Consensus primitives
- `sc-basic-authorship` - Block building

## Testing Strategy

### Unit Tests
- Proposer selection logic
- Slot timing calculations
- Block verification rules
- Signature verification

### Integration Tests
- Runtime API queries
- Block import flow
- Block authoring flow
- Committee rotation

### Network Tests
- Single node block production
- Multi-node consensus
- GRANDPA finality
- Epoch transitions

## Success Criteria

✅ **Runtime Migration Complete:**
- All 12 PBC runtimes use pallet-consensus
- No AURA dependencies in runtimes

✅ **Service Layer Complete:**
- Collators compile without AURA
- Blocks produced with ASF consensus
- PPFA rotation working
- Adaptive slot timing functioning

✅ **Network Functional:**
- Blocks finalized with GRANDPA
- Committee rotation at epochs
- Network health tracking
- Validator rewards distributed

---

**Timeline:** 4 weeks
**Risk Level:** Medium-High (complex Substrate integration)
**Blockers:** None identified
**Dependencies:** Existing ASF algorithm crates (already implemented)

**Next Steps:** Begin Phase 1 - Runtime API implementation
