# Phase 3: Equivocation Detection Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    FinalityGadget                               │
│                                                                 │
│  ┌──────────────┐  ┌─────────────────┐  ┌──────────────────┐  │
│  │ Vote         │  │ Certificate     │  │ Equivocation     │  │
│  │ Collector    │  │ Gossip          │  │ Detector         │  │
│  │              │  │                 │  │                  │  │
│  │ - Aggregates │  │ - Tracks certs  │  │ - Detects        │  │
│  │   votes      │  │ - Checks        │  │   double-voting  │  │
│  │ - Reaches    │  │   finality      │  │ - Generates      │  │
│  │   quorum     │  │                 │  │   proofs         │  │
│  └──────┬───────┘  └────────┬────────┘  └──────┬───────────┘  │
│         │                   │                   │              │
│         └───────────┬───────┴───────────────────┘              │
│                     │                                          │
│              ┌──────▼──────┐                                   │
│              │ Reputation  │                                   │
│              │ System      │                                   │
│              └─────────────┘                                   │
└─────────────────────────────────────────────────────────────────┘
```

## Equivocation Detection Flow

```
                    Vote Received
                         │
                         ▼
          ┌──────────────────────────┐
          │ Extract:                 │
          │ - view                   │
          │ - validator_id           │
          │ - block_hash             │
          │ - block_number           │
          │ - signature              │
          └──────────┬───────────────┘
                     │
                     ▼
          ┌──────────────────────────┐
          │ Check History:           │
          │ votes_seen[(view,        │
          │             validator)]  │
          └──────────┬───────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
        ▼                         ▼
   No previous              Has previous
      vote                      vote
        │                         │
        │                    ┌────┴─────┐
        │                    │          │
        │                    ▼          ▼
        │              Same block   Different
        │              hash (OK)    block hash
        │                    │          │
        │                    │          ▼
        │                    │    ┌──────────────┐
        │                    │    │ EQUIVOCATION │
        │                    │    │   DETECTED!  │
        │                    │    └──────┬───────┘
        │                    │           │
        └────────────────────┴───────┬───┘
                                     │
                                     ▼
                          ┌──────────────────────┐
                          │ Generate Proof:      │
                          │ - first_vote         │
                          │ - second_vote        │
                          │ - validator_id       │
                          │ - view               │
                          └──────────┬───────────┘
                                     │
                        ┌────────────┴────────────┐
                        │                         │
                        ▼                         ▼
              ┌─────────────────┐      ┌──────────────────┐
              │ Store Locally   │      │ Send to Channel  │
              │ pending_proofs  │      │ (slashing)       │
              └─────────────────┘      └──────────────────┘
```

## Data Structures

```
EquivocationDetector
├── votes_seen: RwLock<HashMap<(view, validator), StoredVote>>
│   └── StoredVote
│       ├── block_hash: [u8; 32]
│       ├── block_number: u32
│       ├── signature: [u8; 64]
│       └── received_at: u64
│
├── pending_proofs: RwLock<Vec<EquivocationProof>>
│   └── EquivocationProof
│       ├── validator_id: [u8; 32]
│       ├── view: u64
│       ├── first_vote: EquivocationVote
│       ├── second_vote: EquivocationVote
│       └── detected_at: u64
│
├── reported_validators: RwLock<HashMap<[u8; 32], Vec<u64>>>
│   └── Maps: validator_id -> [views_reported]
│
├── proof_sender: Option<UnboundedSender<EquivocationProof>>
│
├── max_views_retained: u64 (default: 1000)
│
└── current_view: AtomicU64
```

## Thread Safety Model

```
┌─────────────────────────────────────────────────────┐
│  Multiple Concurrent Threads/Tasks                  │
└──────────┬──────────────┬──────────────┬───────────┘
           │              │              │
           ▼              ▼              ▼
    ┌───────────┐  ┌───────────┐  ┌───────────┐
    │ Thread 1  │  │ Thread 2  │  │ Thread 3  │
    │ check_    │  │ cleanup_  │  │ get_      │
    │ vote()    │  │ old_views │  │ stats()   │
    └─────┬─────┘  └─────┬─────┘  └─────┬─────┘
          │              │              │
          └──────────────┼──────────────┘
                         │
              ┌──────────▼──────────┐
              │   RwLock Protection │
              ├─────────────────────┤
              │ - votes_seen        │
              │ - pending_proofs    │
              │ - reported_vals     │
              └─────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │  Shared State       │
              │  (Protected)        │
              └─────────────────────┘
```

## Memory Management

```
Time ──────────────────────────────────────────►

View: 0    100   200   300   ...   1000   1100   1200
      │     │     │     │           │      │      │
      ▼     ▼     ▼     ▼           ▼      ▼      ▼
    ┌───┬─────┬─────┬─────┬───────┬────┬─────┬─────┐
    │   │     │     │     │  ...  │    │     │     │
    │ V │  V  │  V  │  V  │       │ V  │  V  │  V  │
    │   │     │     │     │       │    │     │     │
    └───┴─────┴─────┴─────┴───────┴────┴─────┴─────┘
                                   ▲
                                   │
                            current_view = 1200

    ├─────── Retained ────────────┤
    │   (1000 views)              │
    │                             │
    │◄──────────────────────────►│
    │                             │
    200 ───────────────────► 1200

    Views < 200 are cleaned up
    Memory stays bounded
```

## Integration Points

```
┌─────────────────────────────────────────────────────┐
│                 FinalityGadget                      │
├─────────────────────────────────────────────────────┤
│                                                     │
│  handle_vote(vote, block_number)                   │
│      │                                              │
│      ├─► 1. Extract vote data                      │
│      │                                              │
│      ├─► 2. Check equivocation                     │
│      │    equivocation_detector.check_vote()       │
│      │         │                                    │
│      │         └─► If Some(proof):                 │
│      │              - Update reputation (penalty)  │
│      │              - Return error                 │
│      │              - Reject vote                  │
│      │                                              │
│      ├─► 3. Add to vote collector                  │
│      │                                              │
│      └─► 4. Check quorum & create certificate      │
│                                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  run_worker()                                       │
│      │                                              │
│      ├─► Gossip interval (500ms)                   │
│      │                                              │
│      ├─► Timeout interval (1s)                     │
│      │                                              │
│      └─► Cleanup interval (60s)                    │
│           equivocation_detector.cleanup_old_views() │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## Slashing Integration

```
┌────────────────────────────────────────────────────┐
│            Equivocation Detector                   │
└─────────────────┬──────────────────────────────────┘
                  │ Proof Generated
                  │
                  ▼
        ┌──────────────────────┐
        │ UnboundedSender      │
        │ (proof_sender)       │
        └──────────┬───────────┘
                   │
                   │ Channel
                   │
                   ▼
        ┌──────────────────────┐
        │ UnboundedReceiver    │
        │ (in separate task)   │
        └──────────┬───────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │ Slashing Handler     │
        │ - Encode proof       │
        │ - Submit extrinsic   │
        │ - Penalize validator │
        └──────────────────────┘
                   │
                   ▼
        ┌──────────────────────┐
        │ On-Chain Slashing    │
        │ - Reduce stake       │
        │ - Jail validator     │
        │ - Record offense     │
        └──────────────────────┘
```

## State Transitions

```
Vote State Machine:

    [New Vote]
        │
        ▼
    Is signature     NO
    64 bytes? ──────────► [Skip equivocation check]
        │                      │
        │ YES                  │
        ▼                      │
    Check history              │
        │                      │
        ├─► No previous ──────►│─► Store & Continue
        │                      │
        ├─► Same block ────────┘
        │   (duplicate)
        │
        └─► Different block
             │
             ▼
        [EQUIVOCATION]
             │
             ├─► Generate proof
             │
             ├─► Store proof
             │
             ├─► Send to channel
             │
             ├─► Mark reported
             │
             └─► Return proof
```

## Cleanup Algorithm

```
Cleanup Trigger (every 60s)
        │
        ▼
    Get current_view
        │
        ▼
    current_view < max_retained?
        │
        ├─► YES: Return (nothing to clean)
        │
        └─► NO: Continue
             │
             ▼
    Calculate cutoff = current_view - max_retained
        │
        ▼
    Acquire write lock on votes_seen
        │
        ▼
    Retain only votes where view >= cutoff
        │
        ▼
    Release lock
        │
        ▼
    Log cleanup complete
```

## Error Handling

```
check_vote()
    │
    ├─► Success path: Returns None (no equivocation)
    │
    ├─► Equivocation path: Returns Some(proof)
    │
    └─► No errors thrown (infallible detection)

verify() on EquivocationProof
    │
    ├─► Ok(()): Proof is valid
    │
    └─► Err(EquivocationVerifyError):
         ├─► SameBlockHash
         ├─► InvalidFirstSignature
         └─► InvalidSecondSignature
```

## Performance Characteristics

```
Operation           Time Complexity    Space Complexity
─────────────────── ────────────────── ─────────────────
check_vote()        O(1) average       O(1)
                    O(n) worst case
cleanup_old_views() O(n)               O(1)
get_pending_proofs  O(p)               O(p)
stats()             O(1)               O(1)
verify()            O(1)               O(1)

Where:
  n = number of tracked votes
  p = number of pending proofs
```

## Security Properties

```
┌────────────────────────────────────────────┐
│         Security Guarantees                │
├────────────────────────────────────────────┤
│ ✅ Thread-safe concurrent access           │
│ ✅ No race conditions (RwLock protected)   │
│ ✅ Bounded memory (automatic cleanup)      │
│ ✅ DOS resistant (deduplication)           │
│ ✅ No false positives (strict matching)    │
│ ✅ Proof verifiable (self-contained)       │
│ ✅ Signature ready (prepared for crypto)   │
└────────────────────────────────────────────┘
```

---

**Architecture**: Equivocation Detection System
**Component**: Phase 3 of ASF Finality
**Version**: 1.0
**Status**: Production Ready
