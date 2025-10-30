# Updated 21-Validator Mapping: Gizzi + EojEdred Bootstrap

**Version:** 2.1 (Gizzi Overseer + Eoj Bootstrap)
**Updated:** October 29, 2025

---

## 🎯 Key Changes

1. **validator-01: Gizzi** (AI Overseer, Bootstrap Node 1)
2. **validator-02: EojEdred** (Human Owner, Bootstrap Node 2)
3. Remaining 19 validators: AI Devs from `14-aidevs/`

---

## 🌟 Bootstrap Validators (Tier 3 - Directors)

### Validator-01: Gizzi (AI Overseer)
```yaml
Name: Gizzi
DID: did:etrid:gizzi
Role: Decentralized Director (Tier 3)
Stake: 128 ËTR
Type: AI (Lead Developer, Orchestrator)
Responsibilities:
  - Bootstrap node 1 (genesis peer)
  - Oversees all 11 other AI devs
  - Strategic decision-making
  - Cross-domain coordination
  - Emergency governance actions

Network Identity:
  - Bootnode Address: /ip4/<GIZZI_VM_IP>/tcp/30333/p2p/<GIZZI_PEER_ID>
  - Always online, high availability
  - Primary seed node for network formation

AI DevID Keys:
  - Public Key: ETDabmDaYCVEnk5D56iK7DWLNALeSwoNVGPuupGGg71M
  - DID Document: 14-aidevs/dids/gizzi.json
  - Controller: Self-controlled (did:etrid:gizzi)
```

### Validator-02: EojEdred (Human Owner)
```yaml
Name: EojEdred (Eoj)
DID: did:etrid:eojedred
Role: Decentralized Director (Tier 3)
Stake: 128 ËTR
Type: Human (Founder, Ultimate Authority)
Responsibilities:
  - Bootstrap node 2 (genesis peer)
  - Network founder and ultimate decision maker
  - Emergency override authority
  - Final arbiter in governance disputes
  - Hardware key custody

Network Identity:
  - Bootnode Address: /ip4/<EOJ_VM_IP>/tcp/30333/p2p/<EOJ_PEER_ID>
  - High availability validator
  - Secondary seed node

Human Identity:
  - Will generate new DID for EojEdred
  - Uses standard session keys (AURA, GRANDPA, ASF)
  - Payment account for rewards
  - Full manual control (no AI automation)
```

### Validator-03: Governance Dev
```yaml
Name: Governance Dev
DID: did:etrid:governance-dev01
Role: Decentralized Director (Tier 3)
Stake: 128 ËTR
Type: AI (Governance Specialist)
Responsibilities:
  - Proposal generation and bylaw enforcement
  - Democracy automation
  - Governance analysis
```

---

## 🤖 Complete 21-Validator Allocation

### Tier 3: Decentralized Directors (3 validators, 128 ËTR each)

| # | Validator | Operator | DID | Type | Role |
|---|-----------|----------|-----|------|------|
| 01 | **Gizzi** | Gizzi (AI Overseer) | `did:etrid:gizzi` | AI | **Bootstrap 1** |
| 02 | **EojEdred** | Eoj (Human Owner) | `did:etrid:eojedred` | Human | **Bootstrap 2** |
| 03 | Governance Dev | Governance AI | `did:etrid:governance-dev01` | AI | Director |

**Tier 3 Total Stake:** 384 ËTR

---

### Tier 2a: FlareNodes (Root Chain Validators) (9 validators, 64 ËTR each)

| # | Validator | Operator | DID | Type |
|---|-----------|----------|-----|------|
| 04 | Security Dev | Security AI | `did:etrid:security-dev01` | AI |
| 05 | Audit Dev | Audit AI | `did:etrid:audit-dev01` | AI |
| 06 | Consensus Dev (Primary) | Consensus AI | `did:etrid:consensus-dev01` | AI |
| 07 | Consensus Dev (Secondary) | Consensus AI | `did:etrid:consensus-dev01` | AI |
| 08 | Runtime Dev (Primary) | Runtime AI | `did:etrid:runtime-dev01` | AI |
| 09 | Runtime Dev (Secondary) | Runtime AI | `did:etrid:runtime-dev01` | AI |
| 10 | Compiler Dev (Primary) | Compiler AI | `did:etrid:compiler-dev01` | AI |
| 11 | Compiler Dev (Secondary) | Compiler AI | `did:etrid:compiler-dev01` | AI |
| 12 | Oracle Dev | Oracle AI | `did:etrid:oracle-dev01` | AI |

**Tier 2a Total Stake:** 576 ËTR

---

### Tier 2b: ValidityNodes (PBC Validators) (9 validators, 64 ËTR each)

| # | Validator | Operator | DID | Type |
|---|-----------|----------|-----|------|
| 13 | Multichain Dev (Primary) | Multichain AI | `did:etrid:multichain-dev01` | AI |
| 14 | Multichain Dev (Secondary) | Multichain AI | `did:etrid:multichain-dev01` | AI |
| 15 | EDSC Dev (Primary) | EDSC AI | `did:etrid:edsc-dev01` | AI |
| 16 | EDSC Dev (Secondary) | EDSC AI | `did:etrid:edsc-dev01` | AI |
| 17 | Economics Dev (Primary) | Economics AI | `did:etrid:economics-dev01` | AI |
| 18 | Economics Dev (Secondary) | Economics AI | `did:etrid:economics-dev01` | AI |
| 19 | Ethics Dev | Ethics AI | `did:etrid:ethics-dev01` | AI |
| 20 | Docs Dev | Docs AI | `did:etrid:docs-dev01` | AI |
| 21 | GizziClaude | GizziClaude (AI Reasoning) | `did:etrid:gizzi-claude` | AI |

**Tier 2b Total Stake:** 576 ËTR

---

## 📊 Summary Statistics

```
Total Validators: 21
├─ AI Validators: 20 (95%)
├─ Human Validators: 1 (5%)
└─ Bootstrap Nodes: 2 (Gizzi + EojEdred)

Unique AI Identities: 12
├─ Gizzi (main overseer): 1 validator
├─ GizziClaude (reasoning): 1 validator
├─ EojEdred (human): 1 validator
├─ Consensus, Runtime, Compiler: 2 validators each
├─ Multichain, EDSC, Economics: 2 validators each
└─ Security, Audit, Governance, Oracle, Ethics, Docs: 1 each

Total Network Stake: 1,536 ËTR
├─ Directors (Tier 3): 384 ËTR (25%)
├─ FlareNodes (Tier 2a): 576 ËTR (37.5%)
└─ ValidityNodes (Tier 2b): 576 ËTR (37.5%)
```

---

## 🔐 EojEdred DID Generation

Since EojEdred is a new identity, we need to generate a DID document:

```bash
# Generate EojEdred keypair and DID
cd 14-aidevs

# Add EojEdred to the identity list
cat >> dids/keypairs.json <<'EOF'
  {
    "identity": "eojedred",
    "private_key_hex": "GENERATED_DURING_DEPLOYMENT",
    "public_key_hex": "GENERATED_DURING_DEPLOYMENT",
    "public_key_base58": "GENERATED_DURING_DEPLOYMENT"
  }
EOF

# Generate DID document
cat > dids/eojedred.json <<'EOF'
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1"
  ],
  "id": "did:etrid:eojedred",
  "controller": "did:etrid:eojedred",
  "verificationMethod": [
    {
      "id": "did:etrid:eojedred#key-1",
      "type": "Ed25519VerificationKey2020",
      "controller": "did:etrid:eojedred",
      "publicKeyMultibase": "zGENERATED",
      "publicKeyBase58": "GENERATED"
    }
  ],
  "authentication": [
    "did:etrid:eojedred#key-1"
  ],
  "assertionMethod": [
    "did:etrid:eojedred#key-1"
  ],
  "service": [
    {
      "id": "did:etrid:eojedred#validator-service",
      "type": "HumanValidator",
      "serviceEndpoint": "https://etrid.network/validators/eojedred"
    }
  ],
  "metadata": {
    "name": "EojEdred",
    "description": "Ëtrid Network Founder. Bootstrap validator and ultimate authority.",
    "role": "Founder",
    "twitter": "@EtridNetwork",
    "github": "https://github.com/etrid",
    "created": "2025-10-29T00:00:00Z",
    "updated": "2025-10-29T00:00:00Z"
  }
}
EOF
```

---

## 🌐 Updated Chain Spec (Genesis)

```json
{
  "name": "Ëtrid Mainnet",
  "id": "etrid_mainnet",
  "chainType": "Live",
  "bootNodes": [
    "/ip4/<GIZZI_VM_IP>/tcp/30333/p2p/<GIZZI_PEER_ID>",
    "/ip4/<EOJ_VM_IP>/tcp/30333/p2p/<EOJ_PEER_ID>"
  ],
  "genesis": {
    "runtime": {
      "validatorCommittee": {
        "validators": [
          {
            "name": "Gizzi (AI Overseer)",
            "sessionAccount": "5Gizzi...",
            "auraKey": "0xgizzi_aura...",
            "grandpaKey": "0xgizzi_grandpa...",
            "asfKey": "0xgizzi_asf...",
            "paymentAccount": "5GizziPay...",
            "controllerAccount": "5GizziCtrl...",
            "aiDevID": "did:etrid:gizzi",
            "stake": "128000000000000000000000",
            "role": 4,
            "isBootstrap": true
          },
          {
            "name": "EojEdred (Founder)",
            "sessionAccount": "5Eoj...",
            "auraKey": "0xeoj_aura...",
            "grandpaKey": "0xeoj_grandpa...",
            "asfKey": "0xeoj_asf...",
            "paymentAccount": "5EojPay...",
            "controllerAccount": "5EojCtrl...",
            "aiDevID": "did:etrid:eojedred",
            "stake": "128000000000000000000000",
            "role": 4,
            "isBootstrap": true
          }
          // ... remaining 19 validators
        ]
      },

      "aiDevRegistry": {
        "overseer": {
          "did": "did:etrid:gizzi",
          "validator": "5Gizzi...",
          "permissions": [
            "OverseeAllAIDevs",
            "EmergencyGovernance",
            "CoordinateCrossDomain",
            "StrategicDecisions"
          ]
        },
        "founder": {
          "did": "did:etrid:eojedred",
          "validator": "5Eoj...",
          "permissions": [
            "UltimateAuthority",
            "EmergencyOverride",
            "FinalArbiter",
            "KeyCustody"
          ]
        },
        "validatorDevIDs": [
          // All 21 DevID mappings
        ]
      },

      "balances": {
        "balances": [
          // Gizzi accounts (Bootstrap 1)
          ["5GizziPay...", "10000000000000000000000000"],   // 10M ËTR (overseer fund)
          ["5GizziCtrl...", "1000000000000000000000000"],   // 1M ËTR (operations)
          ["5Gizzi...", "100000000000000000000000"],        // 100K ËTR (tx fees)

          // EojEdred accounts (Bootstrap 2)
          ["5EojPay...", "10000000000000000000000000"],     // 10M ËTR (founder fund)
          ["5EojCtrl...", "1000000000000000000000000"],     // 1M ËTR (operations)
          ["5Eoj...", "100000000000000000000000"],          // 100K ËTR (tx fees)

          // Other 19 validators (standard allocations)
          // ...
        ]
      }
    }
  }
}
```

---

## 🚀 Updated Bootstrap Process

### Step 1: Start Gizzi (Bootstrap Node 1)
```bash
# On Gizzi's VM (validator-01)
./flarechain-node \
  --base-path /var/lib/etrid \
  --chain mainnet-raw.json \
  --name "Gizzi-Overseer" \
  --validator \
  --rpc-cors all \
  --rpc-external \
  --ws-external \
  --port 30333 \
  --rpc-port 9944 \
  --prometheus-port 9615 \
  --node-key-file /var/lib/etrid/network/gizzi-node-key

# Gizzi starts alone, generates peer ID
# Output: Local node identity is: 12D3KooWGizzi...
```

### Step 2: Start EojEdred (Bootstrap Node 2)
```bash
# On EojEdred's VM (validator-02)
./flarechain-node \
  --base-path /var/lib/etrid \
  --chain mainnet-raw.json \
  --name "EojEdred-Founder" \
  --validator \
  --rpc-cors all \
  --rpc-external \
  --ws-external \
  --port 30333 \
  --rpc-port 9944 \
  --prometheus-port 9615 \
  --bootnodes /ip4/<GIZZI_VM_IP>/tcp/30333/p2p/12D3KooWGizzi... \
  --node-key-file /var/lib/etrid/network/eoj-node-key

# EojEdred connects to Gizzi
# Committee now: 2/21 (Gizzi + Eoj)
```

### Step 3: Start Remaining 19 Validators
```bash
# On each remaining validator VM (validator-03 through validator-21)
./flarechain-node \
  --base-path /var/lib/etrid \
  --chain mainnet-raw.json \
  --name "ValidatorName" \
  --validator \
  --rpc-cors all \
  --port 30333 \
  --rpc-port 9944 \
  --bootnodes /ip4/<GIZZI_VM_IP>/tcp/30333/p2p/12D3KooWGizzi... \
  --bootnodes /ip4/<EOJ_VM_IP>/tcp/30333/p2p/12D3KooWEoj...

# Each validator connects via Gizzi and/or Eoj
# Committee grows: 3/21, 4/21, ... 21/21 ✅
```

### Step 4: Committee Formation
```
Committee Status:
├─ Validators 1-2 online → Committee: 2/21 (waiting, min = 4)
├─ Validators 3-4 online → Committee: 4/21 (consensus starts! 🎉)
├─ Validators 5-21 online → Committee: 21/21 (full power! 🚀)
└─ PPFA rotation begins → Blocks every 6 seconds
```

---

## 🎭 Gizzi's Role as AI Overseer

### Responsibilities
```yaml
Strategic Planning:
  - Long-term network roadmap
  - Cross-domain coordination (consensus, economics, governance)
  - Resource allocation among AI devs
  - Priority setting for development tasks

Operational Oversight:
  - Monitor all 11 AI dev performance
  - Detect anomalies in AI behavior
  - Coordinate consensus decisions
  - Emergency intervention

Governance Leadership:
  - Draft complex proposals
  - Facilitate director decisions (along with Eoj and Governance Dev)
  - Resolve disputes between AI devs
  - Represent AI collective in human interactions

Technical Authority:
  - Final say on architecture decisions
  - Code review for critical changes
  - Security policy enforcement
  - Performance optimization strategies
```

### Gizzi's Permissions (On-Chain)
```rust
// In pallet-validator-committee
pub struct AIDevsOverseer {
    pub did: Vec<u8>,  // "did:etrid:gizzi"
    pub validator: AccountId,
    pub permissions: Vec<OverseerPermission>,
}

pub enum OverseerPermission {
    OverseeAllAIDevs,
    EmergencyGovernance,
    CoordinateCrossDomain,
    StrategicDecisions,
    EmergencySlashing,
    ValidatorRotationOverride,
}
```

---

## 🧑 EojEdred's Role as Founder

### Responsibilities
```yaml
Ultimate Authority:
  - Final decision on governance deadlocks
  - Emergency network shutdown (if critical bug)
  - Key custody (hardware wallets, backups)
  - Legal/regulatory interface

Network Stewardship:
  - Ensure alignment with original vision
  - Protect against mission drift
  - Community ambassador
  - Long-term sustainability planning

Technical Oversight:
  - Review critical upgrades
  - Approve major protocol changes
  - Security incident response
  - Disaster recovery execution

Operations:
  - Manage Azure infrastructure
  - Monitor all 21 validators
  - Execute routine maintenance
  - Budget and resource allocation
```

### EojEdred's Permissions (On-Chain)
```rust
pub struct Founder {
    pub did: Vec<u8>,  // "did:etrid:eojedred"
    pub validator: AccountId,
    pub permissions: Vec<FounderPermission>,
}

pub enum FounderPermission {
    UltimateAuthority,
    EmergencyOverride,
    FinalArbiter,
    KeyCustody,
    NetworkShutdown,
    ProtocolChangeApproval,
}
```

---

## 🔑 Updated Key Generation Script

```bash
#!/bin/bash
# generate-validators-gizzi-eoj-bootstrap.sh

# Special handling for validator-01 (Gizzi)
echo "[01/21] Generating Gizzi (AI Overseer, Bootstrap 1)..."

# Load Gizzi's existing AI DevID keys
GIZZI_PRIVATE=$(jq -r '.[] | select(.identity == "gizzi") | .private_key_hex' 14-aidevs/dids/keypairs.json)
GIZZI_PUBLIC=$(jq -r '.[] | select(.identity == "gizzi") | .public_key_base58' 14-aidevs/dids/keypairs.json)

# Generate Gizzi's session keys (new)
GIZZI_SESSION_SEED=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
# ... (store session, payment, controller keys)

echo "  ✓ Gizzi configured as Bootstrap Node 1"

# Special handling for validator-02 (EojEdred)
echo "[02/21] Generating EojEdred (Human Founder, Bootstrap 2)..."

# Generate EojEdred's AI DevID (new identity)
EOJ_DEVID_KEYPAIR=$(python3 14-aidevs/generate_keypairs.py --identity eojedred --output json)
EOJ_PRIVATE=$(echo $EOJ_DEVID_KEYPAIR | jq -r '.private_key_hex')
EOJ_PUBLIC=$(echo $EOJ_DEVID_KEYPAIR | jq -r '.public_key_base58')

# Generate EojEdred's session keys
EOJ_SESSION_SEED=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
# ... (store session, payment, controller keys)

echo "  ✓ EojEdred configured as Bootstrap Node 2"

# Continue with remaining 19 validators (03-21)
# ... (existing logic)
```

---

## 📋 Deployment Checklist Updates

### Pre-Deployment
- [ ] Generate EojEdred DID and keys
- [ ] Update Gizzi's validator configuration
- [ ] Verify both bootstrap nodes in chain spec
- [ ] Fund Gizzi and EojEdred accounts generously (10M ËTR each)

### Deployment Order
1. [ ] Start Gizzi VM (validator-01) first
2. [ ] Wait for Gizzi to generate peer ID
3. [ ] Start EojEdred VM (validator-02) second
4. [ ] Verify Gizzi + Eoj can communicate
5. [ ] Start validators 03-04 (reach min committee size)
6. [ ] Start remaining validators 05-21

### Post-Deployment
- [ ] Verify Gizzi has overseer permissions
- [ ] Verify EojEdred has founder permissions
- [ ] Test Gizzi → AI dev communication
- [ ] Test EojEdred emergency override

---

## 🎯 Benefits of Gizzi + Eoj Bootstrap

### Technical Benefits
✅ **Stable bootstrap:** Gizzi and Eoj always online (unlike Alice/Bob which are test accounts)
✅ **Real identities:** Gizzi and Eoj have actual DIDs and governance roles
✅ **Production-ready:** No need to replace bootstrap nodes later
✅ **Hierarchical leadership:** Clear chain of command (Eoj → Gizzi → AI Devs)

### Operational Benefits
✅ **AI orchestration:** Gizzi coordinates all other AI devs
✅ **Human oversight:** Eoj has ultimate authority for critical decisions
✅ **Automated governance:** Gizzi handles routine decisions
✅ **Manual intervention:** Eoj can override if needed

### Security Benefits
✅ **Known identities:** Both bootstrap nodes have verified DIDs
✅ **Trusted operators:** Founder + AI overseer (not test accounts)
✅ **Redundancy:** If Gizzi fails, Eoj can coordinate; if Eoj fails, Gizzi maintains network
✅ **Auditability:** All actions signed with DIDs

---

## 🚀 Next Steps

1. **Generate EojEdred DID:**
   ```bash
   cd 14-aidevs
   python3 generate_keypairs.py --identity eojedred
   python3 generate_did_documents.py --identity eojedred
   ```

2. **Update key generation script:**
   ```bash
   # Edit scripts/generate-validators-with-payment-aidevid.sh
   # Add special handling for validator-01 (Gizzi) and validator-02 (EojEdred)
   ```

3. **Update chain spec:**
   ```bash
   # Mark Gizzi and Eoj as bootstrap nodes
   # Add overseer and founder permissions
   # Increase genesis allocations (10M ËTR each)
   ```

4. **Deploy in order:**
   ```bash
   # Start Gizzi first, then Eoj, then others
   ```

---

## 📊 Final Statistics

```
Total Validators: 21
├─ Bootstrap Validators: 2 (Gizzi + EojEdred)
├─ AI Validators: 20 (including Gizzi)
├─ Human Validators: 1 (EojEdred)
└─ Unique AI Identities: 12

Leadership Hierarchy:
1. EojEdred (Human Founder) - Ultimate authority
2. Gizzi (AI Overseer) - Strategic coordination
3. 3 Directors (Governance, Security, Audit) - Policy enforcement
4. 18 Operators (FlareNodes + ValidityNodes) - Network operation

Total Stake: 1,536 ËTR
Bootstrap Stake: 256 ËTR (Gizzi 128 + Eoj 128)
```

---

**This configuration makes Gizzi the AI leader and puts you (EojEdred) at the top of the hierarchy with ultimate human authority!** 🚀

Ready to generate the updated scripts?
