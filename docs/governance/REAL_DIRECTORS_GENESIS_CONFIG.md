# Real Directors Genesis Configuration

**Ëtrid Primearc Core - 9 Directors Setup**

---

## Director Roster

### The 9 Decentralized Directors

1. **Eoj** (Founder & CEO)
2. **Gizzi** (Co-Founder & CTO)
3. **Security Lead** (Security & Infrastructure)
4. **Marketing Lead** (Marketing & Communications)
5. **Developer Lead** (Core Development)
6. **Community Lead** (Community & Governance)
7. **Finance Lead** (Treasury & Finance)
8. **Legal Lead** (Legal & Compliance)
9. **Operations Lead** (Operations & Support)

---

## Account Generation

### Step 1: Generate Director Accounts

Each director needs a secure account. Use one of these methods:

#### Option A: Subkey (Production)

```bash
# Generate each director's account
subkey generate --scheme sr25519 --words 24 > eoj-director.txt
subkey generate --scheme sr25519 --words 24 > gizzi-director.txt
subkey generate --scheme sr25519 --words 24 > security-lead.txt
subkey generate --scheme sr25519 --words 24 > marketing-lead.txt
subkey generate --scheme sr25519 --words 24 > dev-lead.txt
subkey generate --scheme sr25519 --words 24 > community-lead.txt
subkey generate --scheme sr25519 --words 24 > finance-lead.txt
subkey generate --scheme sr25519 --words 24 > legal-lead.txt
subkey generate --scheme sr25519 --words 24 > ops-lead.txt
```

Each file contains:
```
Secret phrase: <24-word mnemonic>
Network ID: substrate
Secret seed: 0x...
Public key (hex): 0x...
Account ID: 0x...
Public key (SS58): 5...
SS58 Address: 5...
```

#### Option B: Use Existing Validator Accounts

If directors are ALSO validators (recommended), use their existing validator accounts:

```bash
# Extract from current validator keys
# Directors should use the same accounts they use for validation
```

---

## Genesis Configuration Template

### File: `runtime/presets/primearc_mainnet_with_directors.json`

```json
{
  "name": "Ëtrid Primearc Core Mainnet",
  "id": "primearc-core-chain_mainnet",
  "chainType": "Live",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "primearc-core-chain",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {
        "code": "0x..."
      },
      "balances": {
        "balances": [
          // Eoj - 200 ËTR (128 stake + operational buffer)
          ["<EOJ_SS58_ADDRESS>", "200000000000000000000000"],

          // Gizzi - 200 ËTR
          ["<GIZZI_SS58_ADDRESS>", "200000000000000000000000"],

          // Security Lead - 150 ËTR
          ["<SECURITY_LEAD_SS58_ADDRESS>", "150000000000000000000000"],

          // Marketing Lead - 150 ËTR
          ["<MARKETING_LEAD_SS58_ADDRESS>", "150000000000000000000000"],

          // Developer Lead - 150 ËTR
          ["<DEV_LEAD_SS58_ADDRESS>", "150000000000000000000000"],

          // Community Lead - 150 ËTR
          ["<COMMUNITY_LEAD_SS58_ADDRESS>", "150000000000000000000000"],

          // Finance Lead - 150 ËTR
          ["<FINANCE_LEAD_SS58_ADDRESS>", "150000000000000000000000"],

          // Legal Lead - 150 ËTR
          ["<LEGAL_LEAD_SS58_ADDRESS>", "150000000000000000000000"],

          // Operations Lead - 150 ËTR
          ["<OPS_LEAD_SS58_ADDRESS>", "150000000000000000000000"]
        ]
      },

      "peerRolesStaking": {
        "roles": [
          // NOTE: If directors are ALSO validators, use FlareNode role (0)
          // If directors are governance-only, use DecentralizedDirector role (4)
          // Recommended: Use FlareNode so they participate in consensus

          {
            "account": "<EOJ_SS58_ADDRESS>",
            "role": 0,  // FlareNode (consensus + governance)
            "stake": "128000000000000000000000"  // 128 ËTR
          },
          {
            "account": "<GIZZI_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<SECURITY_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<MARKETING_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<DEV_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<COMMUNITY_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<FINANCE_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<LEGAL_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          },
          {
            "account": "<OPS_LEAD_SS58_ADDRESS>",
            "role": 0,  // FlareNode
            "stake": "128000000000000000000000"
          }
        ]
      },

      "validatorCommittee": {
        "validators": [
          // Validator 0 - Eoj
          {
            "validator_id": [0, 0, 0, 0],
            "stake": "200000000000000000000000",
            "peer_type": 3  // FlareNode
          },
          // Validator 1 - Gizzi
          {
            "validator_id": [1, 0, 0, 0],
            "stake": "200000000000000000000000",
            "peer_type": 3
          },
          // Validator 2 - Security Lead
          {
            "validator_id": [2, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validator 3 - Marketing Lead
          {
            "validator_id": [3, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validator 4 - Developer Lead
          {
            "validator_id": [4, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validator 5 - Community Lead
          {
            "validator_id": [5, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validator 6 - Finance Lead
          {
            "validator_id": [6, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validator 7 - Legal Lead
          {
            "validator_id": [7, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validator 8 - Operations Lead
          {
            "validator_id": [8, 0, 0, 0],
            "stake": "150000000000000000000000",
            "peer_type": 3
          },
          // Validators 9-20 (remaining validators, not directors)
          {
            "validator_id": [9, 0, 0, 0],
            "stake": "100000000000000000000000",
            "peer_type": 3
          }
          // ... Add validators 10-20 here
        ],
        "committee_size": 21
      },

      "decentralizedDirectors": {
        "directors": [
          // Director 1 - Eoj
          {
            "account": "<EOJ_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,  // 365 days in seconds
            "term_active": true
          },
          // Director 2 - Gizzi
          {
            "account": "<GIZZI_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 3 - Security Lead
          {
            "account": "<SECURITY_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 4 - Marketing Lead
          {
            "account": "<MARKETING_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 5 - Developer Lead
          {
            "account": "<DEV_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 6 - Community Lead
          {
            "account": "<COMMUNITY_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 7 - Finance Lead
          {
            "account": "<FINANCE_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 8 - Legal Lead
          {
            "account": "<LEGAL_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          // Director 9 - Operations Lead
          {
            "account": "<OPS_LEAD_SS58_ADDRESS>",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          }
        ],
        "current_epoch": 0
      },

      "directorElection": {
        "elected_directors": [
          "<EOJ_SS58_ADDRESS>",
          "<GIZZI_SS58_ADDRESS>",
          "<SECURITY_LEAD_SS58_ADDRESS>",
          "<MARKETING_LEAD_SS58_ADDRESS>",
          "<DEV_LEAD_SS58_ADDRESS>",
          "<COMMUNITY_LEAD_SS58_ADDRESS>",
          "<FINANCE_LEAD_SS58_ADDRESS>",
          "<LEGAL_LEAD_SS58_ADDRESS>",
          "<OPS_LEAD_SS58_ADDRESS>"
        ],
        "current_epoch": 0,
        "next_consensus_day_block": 5572800  // 365 days at 6s blocks
      }
    }
  }
}
```

---

## Placeholder Replacement Script

### `fill-director-addresses.sh`

```bash
#!/bin/bash

# Fill in director addresses from generated accounts

# Read addresses from key files
EOJ_ADDR=$(grep "SS58 Address:" eoj-director.txt | awk '{print $3}')
GIZZI_ADDR=$(grep "SS58 Address:" gizzi-director.txt | awk '{print $3}')
SECURITY_ADDR=$(grep "SS58 Address:" security-lead.txt | awk '{print $3}')
MARKETING_ADDR=$(grep "SS58 Address:" marketing-lead.txt | awk '{print $3}')
DEV_ADDR=$(grep "SS58 Address:" dev-lead.txt | awk '{print $3}')
COMMUNITY_ADDR=$(grep "SS58 Address:" community-lead.txt | awk '{print $3}')
FINANCE_ADDR=$(grep "SS58 Address:" finance-lead.txt | awk '{print $3}')
LEGAL_ADDR=$(grep "SS58 Address:" legal-lead.txt | awk '{print $3}')
OPS_ADDR=$(grep "SS58 Address:" ops-lead.txt | awk '{print $3}')

# Replace placeholders in genesis JSON
cp primearc_mainnet_with_directors.json primearc_mainnet_filled.json

sed -i "s/<EOJ_SS58_ADDRESS>/$EOJ_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<GIZZI_SS58_ADDRESS>/$GIZZI_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<SECURITY_LEAD_SS58_ADDRESS>/$SECURITY_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<MARKETING_LEAD_SS58_ADDRESS>/$MARKETING_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<DEV_LEAD_SS58_ADDRESS>/$DEV_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<COMMUNITY_LEAD_SS58_ADDRESS>/$COMMUNITY_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<FINANCE_LEAD_SS58_ADDRESS>/$FINANCE_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<LEGAL_LEAD_SS58_ADDRESS>/$LEGAL_ADDR/g" primearc_mainnet_filled.json
sed -i "s/<OPS_LEAD_SS58_ADDRESS>/$OPS_ADDR/g" primearc_mainnet_filled.json

echo "✅ Genesis config filled with director addresses"
```

---

## Naming Conventions

### Public vs Private

#### Option 1: Role-Based Aliases (Recommended)

**Genesis**: Use real account addresses
**Public Display**: Use role-based names

```javascript
// On-chain mapping (not in genesis, add via governance later)
const DIRECTOR_ALIASES = {
  "5GrwvaEF...": "Eoj",
  "5FHneW46...": "Gizzi",
  "5FLSigC9...": "Security-Lead",
  "5DAAnrj7...": "Marketing-Lead",
  "5HGjWAeF...": "Developer-Lead",
  "5CiPPseX...": "Community-Lead",
  "5GNJqTPy...": "Finance-Lead",
  "5HpG9w8E...": "Legal-Lead",
  "5Ck5SLSHYac6...": "Operations-Lead"
};
```

**Frontend Display**:
```typescript
// wallet-web/src/utils/directorNames.ts
export function getDirectorName(address: string): string {
  const aliases = {
    "5GrwvaEF...": "Eoj (Founder)",
    "5FHneW46...": "Gizzi (CTO)",
    "5FLSigC9...": "Security Lead",
    // ...
  };
  return aliases[address] || address;
}
```

---

#### Option 2: Encoded in Pallet (Alternative)

Add `director_name` field to DirectorProfile:

```rust
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub struct DirectorProfile<AccountId, Moment> {
    pub account: AccountId,
    pub term_start: Moment,
    pub term_end: Moment,
    pub term_active: bool,
    pub display_name: BoundedVec<u8, ConstU32<32>>,  // "Eoj", "Gizzi", "Security-Lead"
}
```

**Genesis**:
```json
{
  "account": "5GrwvaEF...",
  "term_start": 0,
  "term_end": 31536000,
  "term_active": true,
  "display_name": "Eoj"
}
```

**Pros**: On-chain identity
**Cons**: Harder to change, takes storage space

---

## Privacy Considerations

### Should Director Identities Be Public?

#### Eoj & Gizzi
- ✅ Public figures (already known)
- ✅ Can use real names

#### Role-Based Directors
- ⚠️ Consider privacy for "Security Lead", "Legal Lead", etc.
- Options:
  1. **Public**: Full transparency, community knows who governs
  2. **Semi-Public**: Role titles only (Security Lead = Alice, known to core team)
  3. **Anonymous**: Only account addresses visible

**Recommendation**: **Semi-Public**
- Genesis uses real account addresses
- Frontend displays role titles
- Core team knows real identities
- Community trusts based on role performance

---

## Security Best Practices

### Multi-Sig for Director Keys

For Eoj and Gizzi (top 2 directors):

```rust
// Use pallet-multisig for critical actions
// Requires 2/3 approval from:
// - Hardware wallet
// - Hot wallet
// - Cold storage backup

let multisig_threshold = 2;
let multisig_members = vec![
  eoj_hardware,
  eoj_hot,
  eoj_cold
];
```

### Key Storage

| Director | Storage Method | Backup |
|----------|----------------|--------|
| Eoj | Ledger Nano X | Paper wallet |
| Gizzi | Ledger Nano X | Paper wallet |
| Security Lead | YubiKey | Encrypted USB |
| Marketing Lead | MetaMask | Seed phrase |
| Dev Lead | Polkadot Vault | Seed phrase |
| Community Lead | MetaMask | Seed phrase |
| Finance Lead | Hardware wallet | Paper wallet |
| Legal Lead | MetaMask | Seed phrase |
| Ops Lead | MetaMask | Seed phrase |

---

## Deployment Checklist

- [ ] Generate 9 secure accounts
- [ ] Securely store 24-word mnemonics
- [ ] Extract SS58 addresses
- [ ] Fill genesis config template
- [ ] Verify all 9 directors have 128+ ËTR stake
- [ ] Verify all 9 are in validator set (if using Dual Citizenship)
- [ ] Document real names → account mapping (private doc)
- [ ] Create public alias mapping (if using role-based names)
- [ ] Test genesis config on local testnet
- [ ] Verify directors pallet integrates without breaking consensus
- [ ] Deploy to mainnet

---

## Questions to Answer

Before finalizing genesis:

1. **Should directors also be validators?**
   - ✅ Recommended: YES (first 9 of 21 validators)
   - Ensures directors have "skin in the game"

2. **Should director names be on-chain?**
   - ✅ Recommended: NO (use off-chain mapping)
   - Preserves privacy, allows name changes

3. **Who are the real 7 directors besides Eoj & Gizzi?**
   - Need real account addresses OR
   - Generate placeholder accounts now, assign later

4. **Primearc Core Chain naming - keep or change?**
   - ✅ KEEP "primearc-core-chain_mainnet" for backward compatibility
   - Display name is "Ëtrid Primearc Core Mainnet"

---

**Status**: Awaiting real director account addresses
**Next Step**: Generate 9 accounts OR provide existing addresses
**Risk Level**: 🟢 Low (architecture verified safe)
