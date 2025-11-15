# Dual-Role Validator Architecture for ËTRID

**Date:** November 12, 2025
**Architecture:** 21 Validators with Dual Roles

---

## Architecture Overview

### The Design Goal

**ASF Committee (Block Production):** All 21 validators participate
**GRANDPA Consensus (Finality):** Only 9 DecentralizedDirector validators participate

This creates a **two-tier consensus model**:
- Tier 1: All 21 validators propose and vote on blocks (ASF/PPFA)
- Tier 2: 9 DecentralizedDirector validators finalize blocks (GRANDPA)

---

## Validator Roles

### Role Distribution

**9 DecentralizedDirector Validators** (Dual Role):
- **ASF Role:** Participate in committee as FlareNode (peerType=1)
- **GRANDPA Role:** Participate in finality as DecentralizedDirector (own identity)
- **Keys:** Both AURA + GRANDPA session keys
- **Stake:** 128 ETR minimum

**12 Standard Validators** (Single Role):
- **ASF Role:** Participate in committee as ValidityNode/FlareNode (peerType=0 or 1)
- **GRANDPA Role:** None (not in GRANDPA authority set)
- **Keys:** Only AURA session keys (no GRANDPA needed)
- **Stake:** 64 ETR minimum

---

## How It Works

### ASF Committee (21 Members)

**Block Production via PPFA (Probabilistic Pre-finality Algorithm):**

All 21 validators participate in ASF committee:
- 9 DecentralizedDirector validators (registered as FlareNode for ASF purposes)
- 12 ValidityNode/FlareNode validators

**Committee Selection Logic:**
```rust
// File: 09-consensus/validator-management/src/lib.rs

pub fn can_be_in_committee(&self) -> bool {
    matches!(
        self,
        PeerType::ValidityNode | PeerType::FlareNode
        // Note: DecentralizedDirector validators register as FlareNode for ASF
    )
}
```

**How DDs Join ASF:**
DecentralizedDirector validators present themselves as **FlareNode** for ASF committee purposes.

This means:
- In ValidatorCommittee pallet registration: `peerType: 1` (FlareNode)
- In GRANDPA authority set: Use their actual DecentralizedDirector identity

---

### GRANDPA Consensus (9 Authorities)

**Finality via GRANDPA:**

Only 9 DecentralizedDirector validators participate in GRANDPA finality:

**Runtime Configuration:**
```rust
// File: 05-multichain/flare-chain/runtime/src/lib.rs (lines 68-72)

impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: Grandpa,  // Only GRANDPA, no AURA
    }
}
```

**GRANDPA Authority Set:**
Only includes the 9 DecentralizedDirector validators who have GRANDPA keys.

---

## Implementation Strategy

### Option A: Dual Registration (Recommended)

Each DecentralizedDirector validator registers **twice**:

1. **For ASF Committee** - As FlareNode (peerType=1)
2. **For GRANDPA** - As DecentralizedDirector (in session keys)

**Genesis Configuration:**
```json
{
  "validatorCommittee": {
    "validators": [
      // 9 DecentralizedDirector validators (registered as FlareNode for ASF)
      {
        "accountId": "0xDD1_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode (for ASF committee eligibility)
        "auraKey": "0xDD1_AURA",
        "grandpaKey": "0xDD1_GRANDPA"
      },
      {
        "accountId": "0xDD2_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode
        "auraKey": "0xDD2_AURA",
        "grandpaKey": "0xDD2_GRANDPA"
      },
      // ... 7 more DDs (total 9)

      // 12 Standard validators (ValidityNode or FlareNode)
      {
        "accountId": "0xVN1_ACCOUNT",
        "stake": 64000000000000000000000,
        "peerType": 0,  // ValidityNode or 1 for FlareNode
        "auraKey": "0xVN1_AURA",
        "grandpaKey": null  // No GRANDPA key needed
      },
      // ... 11 more standard validators
    ]
  },
  "session": {
    "keys": [
      // Only 9 DecentralizedDirector validators in session
      ["0xDD1_ACCOUNT", "0xDD1_ACCOUNT", {"grandpa": "0xDD1_GRANDPA"}],
      ["0xDD2_ACCOUNT", "0xDD2_ACCOUNT", {"grandpa": "0xDD2_GRANDPA"}],
      // ... 7 more DDs (total 9)
      // Note: 12 standard validators NOT in session.keys
    ]
  }
}
```

**Key Points:**
- ✅ All 21 validators in `validatorCommittee.validators` (ASF committee)
- ✅ 9 DDs have `peerType: 1` (FlareNode) to pass ASF committee filter
- ✅ Only 9 DDs in `session.keys` (GRANDPA authority set)
- ✅ 12 standard validators have no GRANDPA keys

---

### Option B: Separate PeerType Field for ASF vs GRANDPA

Add a **separate field** to distinguish ASF role from GRANDPA role.

**New Genesis Structure:**
```json
{
  "validatorCommittee": {
    "validators": [
      {
        "accountId": "0xDD1_ACCOUNT",
        "stake": 128000000000000000000000,
        "asfPeerType": 1,     // FlareNode for ASF committee
        "grandpaPeerType": 2, // DecentralizedDirector for GRANDPA
        "auraKey": "0xDD1_AURA",
        "grandpaKey": "0xDD1_GRANDPA"
      },
      {
        "accountId": "0xVN1_ACCOUNT",
        "stake": 64000000000000000000000,
        "asfPeerType": 0,     // ValidityNode for ASF
        "grandpaPeerType": null,  // Not in GRANDPA
        "auraKey": "0xVN1_AURA",
        "grandpaKey": null
      }
    ]
  }
}
```

**Code Changes:**
```rust
// File: 09-consensus/validator-management/src/lib.rs

pub struct ValidatorInfo {
    pub id: ValidatorId,
    pub stake: Balance,
    pub asf_peer_type: PeerType,     // For ASF committee eligibility
    pub grandpa_peer_type: Option<PeerType>,  // For GRANDPA participation
    // ...
}

pub fn can_be_in_asf_committee(&self) -> bool {
    matches!(
        self.asf_peer_type,
        PeerType::ValidityNode | PeerType::FlareNode
    )
}

pub fn can_be_in_grandpa(&self) -> bool {
    self.grandpa_peer_type == Some(PeerType::DecentralizedDirector)
}
```

**Pros:**
- ✅ Explicit separation of roles
- ✅ Clear which validators participate in what
- ✅ Flexible for future changes

**Cons:**
- ⚠️  More complex genesis config
- ⚠️  Requires pallet changes
- ⚠️  Migration needed from existing schema

---

## Recommended Approach: Option A (Dual Registration)

Use **Option A** because:
1. Minimal code changes (works with existing ValidatorCommittee pallet)
2. Simple genesis configuration
3. Clear separation: ASF uses `peerType`, GRANDPA uses `session.keys`
4. No pallet migrations needed

---

## Session Keys Configuration

### DecentralizedDirector Validators (9)

**Generate both AURA + GRANDPA keys:**
```bash
# On each of the 9 DD validators
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
    http://localhost:9944

# Returns combined session key (AURA + GRANDPA)
# Example: 0xAURA_32BYTES_GRANDPA_32BYTES
```

**Keystore contents:**
```
~/.local/share/flarechain-node/chains/flare_mainnet/keystore/
├── 6772616e64706131...  (GRANDPA key - "gran")
└── 61757261...          (AURA key - "aura")
```

### Standard Validators (12)

**Option 1: Generate AURA only (no GRANDPA needed)**
```bash
# Insert AURA key manually
curl -H "Content-Type: application/json" \
    -d '{
        "id":1,
        "jsonrpc":"2.0",
        "method": "author_insertKey",
        "params": [
            "aura",
            "secret phrase here",
            "0xAURA_PUBLIC_KEY"
        ]
    }' \
    http://localhost:9944
```

**Option 2: Generate both but only use AURA**
```bash
# Generate both (GRANDPA key will be ignored)
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
    http://localhost:9944

# Extract only AURA key for genesis
```

**Keystore contents:**
```
~/.local/share/flarechain-node/chains/flare_mainnet/keystore/
└── 61757261...  (AURA key only - no GRANDPA)
```

---

## GRANDPA Authority Set

### How GRANDPA Determines Authorities

GRANDPA reads authorities from **session keys** in genesis:

**File:** `05-multichain/flare-chain/runtime/src/lib.rs`

```rust
impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
    fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
        Grandpa::grandpa_authorities()  // Reads from pallet_grandpa
    }
}
```

**Pallet Grandpa** gets initial authorities from genesis:
```json
{
  "session": {
    "keys": [
      ["0xDD1", "0xDD1", {"grandpa": "0xDD1_GRANDPA"}],
      ["0xDD2", "0xDD2", {"grandpa": "0xDD2_GRANDPA"}],
      // ... only 9 DDs, not 21 validators
    ]
  }
}
```

**Result:**
- GRANDPA authority set = 9 DecentralizedDirector validators
- ASF committee = 21 validators (9 DDs + 12 standard)

---

## Consensus Flow

### Block Production (ASF/PPFA)

```
1. ASF Committee selects PPFA proposer from 21 validators
2. Proposer creates block
3. Committee votes on block (21 validators vote)
4. Block accepted if 2/3+ of 21 vote yes (14+ votes needed)
5. Block added to chain (not yet finalized)
```

### Block Finalization (GRANDPA)

```
1. GRANDPA authority set (9 DDs) receives new block
2. Each DD signs finality vote
3. GRANDPA collects votes from 9 DDs
4. Block finalized if 2/3+ of 9 vote (6+ votes needed)
5. Block becomes irreversible
```

### Combined Flow

```
Block #N
├─ ASF: Proposed by 1 of 21 validators
├─ ASF: Voted by 14+ of 21 validators → Block accepted
└─ GRANDPA: Finalized by 6+ of 9 DD validators → Block finalized

Block #N+1
└─ (same process)
```

---

## Quorum Requirements

### ASF Committee (21 validators)
- **Quorum:** 2/3 + 1 = 15 validators
- **Byzantine Tolerance:** Can tolerate up to 6 malicious/offline validators
- **Liveness:** Requires 15+ validators online

### GRANDPA Consensus (9 DDs)
- **Quorum:** 2/3 = 6 validators (GRANDPA uses 2/3, not 2/3+1)
- **Byzantine Tolerance:** Can tolerate up to 2 malicious/offline DDs
- **Liveness:** Requires 6+ DDs online

### Overall System Requirements

For the system to function:
- ✅ At least 15 of 21 validators online (ASF committee)
- ✅ At least 6 of 9 DDs online (GRANDPA finality)

**Failure scenarios:**
- If < 15 validators online: Block production stops
- If < 6 DDs online: Blocks produce but don't finalize (chain stalls)

---

## Genesis Configuration Template

```json
{
  "validatorCommittee": {
    "validators": [
      // === 9 DecentralizedDirector Validators ===
      // d1 (Oracle)
      {
        "accountId": "0xD1_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode (for ASF eligibility)
        "auraKey": "0xD1_AURA",
        "grandpaKey": "0xD1_GRANDPA"
      },
      // d5 (Oracle)
      {
        "accountId": "0xD5_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode
        "auraKey": "0xD5_AURA",
        "grandpaKey": "0xD5_GRANDPA"
      },
      // V0B-EojEdred (Azure)
      {
        "accountId": "0xV0B_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode
        "auraKey": "0xV0B_AURA",
        "grandpaKey": "0xV0B_GRANDPA"
      },
      // V1-Governance (Azure)
      {
        "accountId": "0xV1_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode
        "auraKey": "0xV1_AURA",
        "grandpaKey": "0xV1_GRANDPA"
      },
      // V2-Security (Azure)
      {
        "accountId": "0xV2_ACCOUNT",
        "stake": 128000000000000000000000,
        "peerType": 1,  // FlareNode
        "auraKey": "0xV2_AURA",
        "grandpaKey": "0xV2_GRANDPA"
      },
      // ... 4 more DDs (total 9)

      // === 12 Standard Validators ===
      {
        "accountId": "0xVN1_ACCOUNT",
        "stake": 64000000000000000000000,
        "peerType": 0,  // ValidityNode
        "auraKey": "0xVN1_AURA",
        "grandpaKey": null  // No GRANDPA participation
      },
      {
        "accountId": "0xVN2_ACCOUNT",
        "stake": 64000000000000000000000,
        "peerType": 1,  // FlareNode
        "auraKey": "0xVN2_AURA",
        "grandpaKey": null
      },
      // ... 10 more standard validators (total 12)
    ]
  },
  "session": {
    "keys": [
      // Only 9 DecentralizedDirector validators
      ["0xD1_ACCOUNT", "0xD1_ACCOUNT", {"grandpa": "0xD1_GRANDPA"}],
      ["0xD5_ACCOUNT", "0xD5_ACCOUNT", {"grandpa": "0xD5_GRANDPA"}],
      ["0xV0B_ACCOUNT", "0xV0B_ACCOUNT", {"grandpa": "0xV0B_GRANDPA"}],
      ["0xV1_ACCOUNT", "0xV1_ACCOUNT", {"grandpa": "0xV1_GRANDPA"}],
      ["0xV2_ACCOUNT", "0xV2_ACCOUNT", {"grandpa": "0xV2_GRANDPA"}],
      // ... 4 more DDs (total 9)
      // Note: 12 standard validators NOT included here
    ]
  },
  "grandpa": {
    "authorities": [
      ["0xD1_GRANDPA", 1],
      ["0xD5_GRANDPA", 1],
      ["0xV0B_GRANDPA", 1],
      ["0xV1_GRANDPA", 1],
      ["0xV2_GRANDPA", 1],
      // ... 4 more (total 9)
    ]
  }
}
```

---

## Validator Assignment Strategy

### Which VMs Should Be DecentralizedDirectors?

**High Priority VMs (Primary DDs):**
1. **d1** (Oracle) - Critical infrastructure, different architecture
2. **d5** (Oracle) - Critical infrastructure, different architecture
3. **V0B-EojEdred** (Azure Sub 2) - Primary dev, governance operations
4. **V1-Governance** (Azure Sub 2) - Governance role
5. **V2-Security** (Azure Sub 2) - Security monitoring

**Additional DDs (Choose 4 from Azure Sub 1):**
6. **azure-we-1** (West Europe) - Geographic diversity
7. **azure-ne-1** (North Europe) - Geographic diversity
8. **azure-uk-1** (UK South) - Geographic diversity
9. **azure-fr-1** (France Central) - Geographic diversity

**Rationale:**
- High priority VMs have specific roles (dev, governance, security)
- Geographic distribution across 4 Azure regions for resilience
- Mix of Oracle (2) and Azure (7) for cloud diversity

**Standard Validators (12 remaining):**
- All other Azure VMs in West Europe, North Europe, UK South, France Central

---

## Deployment Checklist

### For DecentralizedDirector Validators (9)

- [ ] Generate AURA + GRANDPA session keys
- [ ] Verify both keys in keystore
- [ ] Set `peerType: 1` (FlareNode) in ValidatorCommittee genesis
- [ ] Include in `session.keys` with GRANDPA public key
- [ ] Stake: 128 ETR minimum
- [ ] Backup keystore (critical - these are GRANDPA authorities)

### For Standard Validators (12)

- [ ] Generate AURA session keys (GRANDPA optional)
- [ ] Set `peerType: 0 or 1` (ValidityNode or FlareNode) in genesis
- [ ] Do NOT include in `session.keys`
- [ ] Stake: 64 ETR minimum
- [ ] Backup keystore

### For All Validators (21)

- [ ] Include in `validatorCommittee.validators` array
- [ ] Configure full peer mesh (20 peers each)
- [ ] Deploy correct architecture binary (ARM vs x86_64)
- [ ] Verify firewall allows port 30333

---

## Verification Procedures

### Verify ASF Committee Size

```bash
journalctl -u flarechain-validator | grep -i "committee.*21\|loaded.*21"
```

**Expected:**
```
✅ Loaded 21 committee members from runtime
ASF committee initialized with 21 validators
```

### Verify GRANDPA Authority Count

```bash
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "grandpa_roundState"}' | \
    jq '.result.setId, .result.authorities | length'
```

**Expected:**
```
0  (set ID)
9  (authority count)
```

### Verify Dual Role for DDs

Check that DecentralizedDirector validators:
1. Appear in ASF committee logs (participating in block production)
2. Appear in GRANDPA logs (participating in finality)

```bash
# DD validator log check
journalctl -u flarechain-validator | grep -i "grandpa.*vote\|asf.*propose"
```

**Expected for DD validators:**
```
ASF: Proposing block #42 as PPFA proposer
GRANDPA: Voting on finalization of #41
```

**Expected for standard validators:**
```
ASF: Proposing block #43 as PPFA proposer
(No GRANDPA messages)
```

---

## Advantages of This Architecture

### Security Benefits

1. **Separation of Concerns:**
   - Block production (ASF) decentralized across 21 validators
   - Finality (GRANDPA) controlled by trusted 9 DecentralizedDirectors

2. **Defense in Depth:**
   - Even if some standard validators are compromised, DDs control finality
   - DDs can reject malicious blocks produced by ASF committee

3. **Upgrade Control:**
   - DDs can coordinate runtime upgrades via GRANDPA
   - Standard validators can't force unwanted changes

### Performance Benefits

1. **Faster Finality:**
   - Only 9 DDs need to vote (vs 21)
   - Smaller consensus group = faster finality

2. **Lower Latency:**
   - GRANDPA messages only between 9 DDs
   - Less network overhead

3. **Scalability:**
   - Can add more standard validators without slowing GRANDPA
   - ASF committee can grow, GRANDPA stays lean

---

## Summary

### Architecture

**21 Validators = 9 DDs + 12 Standard**

**ASF Committee:** 21 validators
- 9 DecentralizedDirector validators (as FlareNode, peerType=1)
- 12 Standard validators (ValidityNode or FlareNode, peerType=0 or 1)

**GRANDPA Consensus:** 9 validators
- Only DecentralizedDirector validators
- Control finality via session keys

### Implementation

**Genesis Config:**
- `validatorCommittee.validators`: All 21 validators
- 9 DDs with `peerType: 1` to join ASF committee
- `session.keys`: Only 9 DDs for GRANDPA

**No Code Changes Needed:**
- Existing `can_be_in_committee()` works (allows FlareNode)
- GRANDPA reads from `session.keys` (only 9 DDs)

### Result

✅ All 21 validators participate in ASF block production
✅ Only 9 DD validators participate in GRANDPA finality
✅ Two-tier consensus for security and performance

---

*Document created: November 12, 2025*
*Architecture: Dual-role validators for ASF + GRANDPA*
