# Ëtrid Validator Deployment: AI DevID + Payment Integration

## Overview

This document extends the 21-validator deployment to include:
1. **Payment/Stash accounts** for receiving validator rewards
2. **AI DevID integration** for validator identity and verification
3. **Complete key hierarchy** management

---

## 🔑 Complete Key Architecture

Each validator has **4 key types**:

```
Validator-01 (Consensus Dev)
├─ 1. SESSION KEYS (Consensus)
│  ├─ AURA Key (sr25519) - Block production
│  ├─ GRANDPA Key (ed25519) - Finality voting
│  └─ ASF Key (sr25519) - Committee authorization
│
├─ 2. PAYMENT/STASH ACCOUNT (Rewards)
│  ├─ Stash Account (sr25519) - Receives rewards
│  └─ Controller Account (sr25519) - Manages validator
│
├─ 3. AI DEVID KEYS (Identity)
│  └─ Ed25519 DID Key - AI identity verification
│
└─ 4. NETWORK KEY (P2P)
   └─ Network Secret (ed25519) - Libp2p identity
```

---

## 🤖 AI DevID to Validator Mapping

Since you have 12 AI Devs and need 21 validators, here's the allocation:

### Tier 3: Decentralized Directors (3 validators)

| Validator | AI DevID | DID | Role | Stake |
|-----------|----------|-----|------|-------|
| validator-01 | Governance Dev | `did:etrid:governance-dev01` | Director | 128 ËTR |
| validator-02 | Security Dev | `did:etrid:security-dev01` | Director | 128 ËTR |
| validator-03 | Audit Dev | `did:etrid:audit-dev01` | Director | 128 ËTR |

### Tier 2a: FlareNodes (9 validators)

| Validator | AI DevID | DID | Role | Stake |
|-----------|----------|-----|------|-------|
| validator-04 | Consensus Dev (Primary) | `did:etrid:consensus-dev01` | FlareNode | 64 ËTR |
| validator-05 | Consensus Dev (Secondary) | `did:etrid:consensus-dev01` | FlareNode | 64 ËTR |
| validator-06 | Runtime Dev (Primary) | `did:etrid:runtime-dev01` | FlareNode | 64 ËTR |
| validator-07 | Runtime Dev (Secondary) | `did:etrid:runtime-dev01` | FlareNode | 64 ËTR |
| validator-08 | Compiler Dev (Primary) | `did:etrid:compiler-dev01` | FlareNode | 64 ËTR |
| validator-09 | Compiler Dev (Secondary) | `did:etrid:compiler-dev01` | FlareNode | 64 ËTR |
| validator-10 | Multichain Dev (Primary) | `did:etrid:multichain-dev01` | FlareNode | 64 ËTR |
| validator-11 | Multichain Dev (Secondary) | `did:etrid:multichain-dev01` | FlareNode | 64 ËTR |
| validator-12 | Oracle Dev | `did:etrid:oracle-dev01` | FlareNode | 64 ËTR |

### Tier 2b: ValidityNodes (9 validators)

| Validator | AI DevID | DID | Role | Stake |
|-----------|----------|-----|------|-------|
| validator-13 | EDSC Dev (Primary) | `did:etrid:edsc-dev01` | ValidityNode | 64 ËTR |
| validator-14 | EDSC Dev (Secondary) | `did:etrid:edsc-dev01` | ValidityNode | 64 ËTR |
| validator-15 | Economics Dev (Primary) | `did:etrid:economics-dev01` | ValidityNode | 64 ËTR |
| validator-16 | Economics Dev (Secondary) | `did:etrid:economics-dev01` | ValidityNode | 64 ËTR |
| validator-17 | Ethics Dev (Primary) | `did:etrid:ethics-dev01` | ValidityNode | 64 ËTR |
| validator-18 | Ethics Dev (Secondary) | `did:etrid:ethics-dev01` | ValidityNode | 64 ËTR |
| validator-19 | Docs Dev (Primary) | `did:etrid:docs-dev01` | ValidityNode | 64 ËTR |
| validator-20 | Docs Dev (Secondary) | `did:etrid:docs-dev01` | ValidityNode | 64 ËTR |
| validator-21 | Docs Dev (Tertiary) | `did:etrid:docs-dev01` | ValidityNode | 64 ËTR |

**Rationale:**
- Critical AI devs (Consensus, Runtime, Compiler) run 2 validators each for redundancy
- Less critical AI devs run 1-3 validators to reach 21 total
- Each validator has unique session keys but shares AI DevID

---

## 🔐 Updated Key Generation Script

```bash
#!/bin/bash
# scripts/generate-21-validators-with-payment-aidevid.sh

KEYVAULT_NAME="etrid-val-keys"
BINARY_PATH="/opt/etrid/flarechain-node"
AIDEVID_DIR="14-aidevs/dids"

# Load AI DevID keypairs
AIDEVID_KEYPAIRS=$(cat $AIDEVID_DIR/keypairs.json)

# AI DevID mapping (validator index → AI DevID)
declare -A VALIDATOR_TO_AIDEVID=(
  [1]="governance-dev01"
  [2]="security-dev01"
  [3]="audit-dev01"
  [4]="consensus-dev01"
  [5]="consensus-dev01"
  [6]="runtime-dev01"
  [7]="runtime-dev01"
  [8]="compiler-dev01"
  [9]="compiler-dev01"
  [10]="multichain-dev01"
  [11]="multichain-dev01"
  [12]="oracle-dev01"
  [13]="edsc-dev01"
  [14]="edsc-dev01"
  [15]="economics-dev01"
  [16]="economics-dev01"
  [17]="ethics-dev01"
  [18]="ethics-dev01"
  [19]="docs-dev01"
  [20]="docs-dev01"
  [21]="docs-dev01"
)

echo "Generating keys for 21 validators with payment accounts and AI DevIDs..."
echo ""

for i in {01..21}; do
  VALIDATOR_NAME="validator-$i"
  AIDEVID="${VALIDATOR_TO_AIDEVID[$((10#$i))]}"

  echo "[$i/21] Generating $VALIDATOR_NAME (AI: $AIDEVID)..."

  # ═══════════════════════════════════════════════════════════
  # 1. SESSION KEYS (Consensus)
  # ═══════════════════════════════════════════════════════════

  # Generate sr25519 seed (master key for session keys)
  SESSION_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json)
  SESSION_SEED=$(echo $SESSION_SEED_JSON | jq -r '.secretSeed')
  SESSION_PHRASE=$(echo $SESSION_SEED_JSON | jq -r '.secretPhrase')
  SESSION_ACCOUNT_ID=$(echo $SESSION_SEED_JSON | jq -r '.ss58Address')

  # Derive AURA key
  AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SESSION_SEED" --output-type json)
  AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

  # Derive GRANDPA key
  GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SESSION_SEED" --output-type json)
  GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

  # ASF key (same as AURA for now)
  ASF_PUBKEY=$AURA_PUBKEY

  # ═══════════════════════════════════════════════════════════
  # 2. PAYMENT/STASH ACCOUNT (Rewards)
  # ═══════════════════════════════════════════════════════════

  # Generate separate payment account
  PAYMENT_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json)
  PAYMENT_SEED=$(echo $PAYMENT_SEED_JSON | jq -r '.secretSeed')
  PAYMENT_PHRASE=$(echo $PAYMENT_SEED_JSON | jq -r '.secretPhrase')
  PAYMENT_ACCOUNT_ID=$(echo $PAYMENT_SEED_JSON | jq -r '.ss58Address')
  PAYMENT_PUBKEY=$(echo $PAYMENT_SEED_JSON | jq -r '.publicKey')

  # Generate controller account (manages validator, lower security)
  CONTROLLER_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json)
  CONTROLLER_SEED=$(echo $CONTROLLER_SEED_JSON | jq -r '.secretSeed')
  CONTROLLER_ACCOUNT_ID=$(echo $CONTROLLER_SEED_JSON | jq -r '.ss58Address')
  CONTROLLER_PUBKEY=$(echo $CONTROLLER_SEED_JSON | jq -r '.publicKey')

  # ═══════════════════════════════════════════════════════════
  # 3. AI DEVID KEYS (Identity)
  # ═══════════════════════════════════════════════════════════

  # Extract AI DevID keys from keypairs.json
  AIDEVID_PRIVATE=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .private_key_hex")
  AIDEVID_PUBLIC=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .public_key_base58")
  AIDEVID_DID="did:etrid:${AIDEVID}"

  # ═══════════════════════════════════════════════════════════
  # 4. STORE IN AZURE KEY VAULT
  # ═══════════════════════════════════════════════════════════

  # Session keys
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-session-seed" \
    --value "$SESSION_SEED" \
    --tags "validator=$VALIDATOR_NAME" "type=session_seed" "aidevid=$AIDEVID"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-session-phrase" \
    --value "$SESSION_PHRASE"

  # Payment/Stash account
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-payment-seed" \
    --value "$PAYMENT_SEED" \
    --tags "validator=$VALIDATOR_NAME" "type=payment_seed"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-payment-phrase" \
    --value "$PAYMENT_PHRASE"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-payment-account" \
    --value "$PAYMENT_ACCOUNT_ID"

  # Controller account
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-controller-seed" \
    --value "$CONTROLLER_SEED"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-controller-account" \
    --value "$CONTROLLER_ACCOUNT_ID"

  # AI DevID reference
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-aidevid" \
    --value "$AIDEVID_DID"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-aidevid-pubkey" \
    --value "$AIDEVID_PUBLIC"

  # Public keys (for chain spec)
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-aura-pubkey" \
    --value "$AURA_PUBKEY"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-grandpa-pubkey" \
    --value "$GRANDPA_PUBKEY"

  # ═══════════════════════════════════════════════════════════
  # 5. WRITE TO JSON (for chain spec)
  # ═══════════════════════════════════════════════════════════

  # Determine stake and role
  if [ $((10#$i)) -le 3 ]; then
    STAKE="128000000000000000000000"
    ROLE=4  # DecentralizedDirector
    ROLE_NAME="Director"
  elif [ $((10#$i)) -le 12 ]; then
    STAKE="64000000000000000000000"
    ROLE=3  # FlareNode
    ROLE_NAME="FlareNode"
  else
    STAKE="64000000000000000000000"
    ROLE=2  # ValidityNode
    ROLE_NAME="ValidityNode"
  fi

  cat >> generated-keys/validator-keys-complete.json <<EOF
  {
    "validatorIndex": $((10#$i)),
    "name": "$VALIDATOR_NAME",

    "sessionKeys": {
      "accountId": "$SESSION_ACCOUNT_ID",
      "auraKey": "$AURA_PUBKEY",
      "grandpaKey": "$GRANDPA_PUBKEY",
      "asfKey": "$ASF_PUBKEY"
    },

    "paymentAccount": {
      "accountId": "$PAYMENT_ACCOUNT_ID",
      "publicKey": "$PAYMENT_PUBKEY"
    },

    "controllerAccount": {
      "accountId": "$CONTROLLER_ACCOUNT_ID",
      "publicKey": "$CONTROLLER_PUBKEY"
    },

    "aiDevID": {
      "did": "$AIDEVID_DID",
      "identity": "$AIDEVID",
      "publicKey": "$AIDEVID_PUBLIC"
    },

    "role": {
      "type": $ROLE,
      "name": "$ROLE_NAME",
      "stake": "$STAKE"
    }
  }$([ $((10#$i)) -eq 21 ] || echo ",")
EOF

  echo "  ✓ Generated: Session + Payment + DevID ($AIDEVID_DID)"
done

echo ""
echo "✅ All 21 validators generated with complete key hierarchy"
echo "📄 Keys saved to: generated-keys/validator-keys-complete.json"
echo "💾 Keys backed up to Azure Key Vault: $KEYVAULT_NAME"
```

---

## 📋 Updated Chain Spec Structure

```json
{
  "name": "Ëtrid Mainnet",
  "id": "etrid_mainnet",
  "chainType": "Live",
  "genesis": {
    "runtime": {
      "validatorCommittee": {
        "validators": [
          {
            "sessionAccountId": "5GrwvaEF...",  // Session keys account
            "auraKey": "0x1234...",
            "grandpaKey": "0x5678...",
            "asfKey": "0x9abc..."
          }
          // ... 21 validators
        ]
      },

      "staking": {
        "validatorStaking": [
          {
            "stashAccount": "5DfhG...",         // Payment account (receives rewards)
            "controllerAccount": "5CkRh...",    // Controller account
            "sessionAccount": "5GrwvaEF...",    // Links to session keys
            "stake": "128000000000000000000000",
            "role": 4,  // DecentralizedDirector
            "active": true
          }
          // ... 21 staking records
        ]
      },

      "aiDevRegistry": {
        "validatorDevIDs": [
          {
            "validatorIndex": 1,
            "sessionAccount": "5GrwvaEF...",
            "paymentAccount": "5DfhG...",
            "aiDevID": "did:etrid:governance-dev01",
            "aiDevPubkey": "3jMSk31C8sYWS6pKiJ93yQU4M72teijsVA4Q8YrZCsw4",
            "registeredAt": 0
          }
          // ... 21 DevID mappings
        ]
      },

      "balances": {
        "balances": [
          // Fund payment accounts (for receiving rewards)
          ["5DfhG...", "1000000000000000000000000"],  // validator-01 payment account (1M ËTR seed)
          ["5CkRh...", "100000000000000000000000"],   // validator-01 controller account (100K ËTR for tx fees)

          // Fund session accounts (for tx fees during consensus)
          ["5GrwvaEF...", "10000000000000000000000"],  // validator-01 session account (10K ËTR for tx fees)

          // Repeat for all 21 validators...
        ]
      }
    }
  }
}
```

---

## 💰 How Validator Payments Work

### Payment Flow

```
1. Validator produces block (uses SESSION KEYS)
   ↓
2. Runtime calculates block reward (10 ËTR)
   ↓
3. Reward sent to PAYMENT ACCOUNT (stash)
   ↓
4. Payment account accumulates rewards
   ↓
5. Controller account can:
   - Withdraw rewards to cold storage
   - Re-stake for higher stake
   - Transfer to treasury
```

### Key Separation (Security)

```
┌──────────────────────────────────────────────────────────┐
│                  KEY SECURITY LEVELS                     │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  HOT KEYS (on validator VM):                            │
│  ├─ Session Keys (AURA, GRANDPA, ASF)                   │
│  │  └─ Used every 6 seconds for consensus               │
│  └─ Network Key                                          │
│     └─ Used continuously for P2P                         │
│                                                          │
│  WARM KEYS (Azure Key Vault):                           │
│  └─ Controller Account                                   │
│     └─ Used occasionally for validator management        │
│                                                          │
│  COLD KEYS (Offline/Hardware Wallet):                   │
│  └─ Payment/Stash Account                                │
│     └─ Receives rewards, rarely signs transactions       │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

**Best Practice:**
- **Session keys**: Hot (on validator, auto-used)
- **Controller keys**: Warm (Key Vault, manual use)
- **Payment keys**: Cold (offline, withdraw rewards monthly)

---

## 🔧 VM Setup with Payment Keys

```bash
#!/bin/bash
# On each validator VM during setup

VALIDATOR_NAME=$(hostname)
KEYVAULT_NAME="etrid-val-keys"

# Authenticate with Managed Identity
az login --identity

# ═══════════════════════════════════════════════════════════
# 1. RETRIEVE SESSION KEYS (insert into node keystore)
# ═══════════════════════════════════════════════════════════

SESSION_SEED=$(az keyvault secret show \
  --vault-name $KEYVAULT_NAME \
  --name "${VALIDATOR_NAME}-session-seed" \
  --query value -o tsv)

# Insert session keys
flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain mainnet \
  --key-type aura \
  --scheme sr25519 \
  --suri "$SESSION_SEED"

flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain mainnet \
  --key-type gran \
  --scheme ed25519 \
  --suri "$SESSION_SEED"

flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain mainnet \
  --key-type asfk \
  --scheme sr25519 \
  --suri "$SESSION_SEED"

unset SESSION_SEED

# ═══════════════════════════════════════════════════════════
# 2. RETRIEVE PAYMENT ACCOUNT (store locally for reference)
# ═══════════════════════════════════════════════════════════

PAYMENT_ACCOUNT=$(az keyvault secret show \
  --vault-name $KEYVAULT_NAME \
  --name "${VALIDATOR_NAME}-payment-account" \
  --query value -o tsv)

# Save payment account for monitoring
echo "$PAYMENT_ACCOUNT" > /var/lib/etrid/payment-account.txt
chown etrid:etrid /var/lib/etrid/payment-account.txt
chmod 400 /var/lib/etrid/payment-account.txt

# ═══════════════════════════════════════════════════════════
# 3. RETRIEVE AI DEVID (for verification)
# ═══════════════════════════════════════════════════════════

AIDEVID=$(az keyvault secret show \
  --vault-name $KEYVAULT_NAME \
  --name "${VALIDATOR_NAME}-aidevid" \
  --query value -o tsv)

# Save AI DevID for logging/monitoring
echo "$AIDEVID" > /var/lib/etrid/aidevid.txt
chown etrid:etrid /var/lib/etrid/aidevid.txt

echo "✓ Session keys inserted"
echo "✓ Payment account: $PAYMENT_ACCOUNT"
echo "✓ AI DevID: $AIDEVID"
```

---

## 📊 Monitoring Validator Rewards

```bash
#!/bin/bash
# scripts/monitor-validator-rewards.sh

KEYVAULT_NAME="etrid-val-keys"
RPC_ENDPOINT="http://localhost:9944"

echo "Validator Reward Balances:"
echo "─────────────────────────────────────────────────────────"

for i in {01..21}; do
  VALIDATOR_NAME="validator-$i"

  # Get payment account from Key Vault
  PAYMENT_ACCOUNT=$(az keyvault secret show \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-payment-account" \
    --query value -o tsv)

  # Query balance on-chain
  BALANCE=$(curl -s -H "Content-Type: application/json" \
    -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_accountBalance\", \"params\": [\"$PAYMENT_ACCOUNT\"]}" \
    $RPC_ENDPOINT | jq -r '.result.free')

  # Convert from wei to ËTR (18 decimals)
  BALANCE_ETR=$(echo "scale=2; $BALANCE / 1000000000000000000" | bc)

  echo "$VALIDATOR_NAME: $BALANCE_ETR ËTR ($PAYMENT_ACCOUNT)"
done

echo "─────────────────────────────────────────────────────────"
```

---

## 🆔 AI DevID On-Chain Registration

```rust
// Example extrinsic to register AI DevID
// In: src/pallets/pallet-validator-committee/src/lib.rs

#[pallet::call_index(10)]
#[pallet::weight(10_000)]
pub fn register_validator_devid(
    origin: OriginFor<T>,
    session_account: T::AccountId,
    payment_account: T::AccountId,
    devid: Vec<u8>,  // "did:etrid:consensus-dev01"
    devid_pubkey: Vec<u8>,
    signature: Vec<u8>,  // Signature proving ownership
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Verify signature
    ensure!(
        Self::verify_devid_signature(&devid, &devid_pubkey, &signature),
        Error::<T>::InvalidSignature
    );

    // Store mapping
    ValidatorDevIDs::<T>::insert(&session_account, ValidatorDevID {
        session_account: session_account.clone(),
        payment_account: payment_account.clone(),
        devid: devid.clone(),
        devid_pubkey,
        registered_at: <frame_system::Pallet<T>>::block_number(),
    });

    Self::deposit_event(Event::ValidatorDevIDRegistered(session_account, devid));

    Ok(())
}
```

---

## 🔐 Security Best Practices

### Key Storage Hierarchy

| Key Type | Storage Location | Access Frequency | Security Level |
|----------|-----------------|------------------|----------------|
| Session Keys | VM keystore | Continuous (6s) | HOT |
| Controller Keys | Azure Key Vault | Monthly | WARM |
| Payment Keys | Hardware Wallet | Quarterly | COLD |
| AI DevID Keys | Encrypted local | On registration | WARM |

### Backup Strategy (3-2-1 Rule + AI DevID)

```
3 Copies:
├─ 1. Azure Key Vault (primary, HSM-backed)
├─ 2. Encrypted USB drive (offline, fireproof safe)
└─ 3. Paper backup (bank vault) + AI DevID export

2 Media:
├─ Digital (Azure + USB)
└─ Physical (paper + AI DevID documents)

1 Off-Site:
└─ Bank safety deposit box + GitHub backup (14-aidevs/dids/)
```

### AI DevID Key Rotation

```bash
# If AI DevID key compromised, rotate and re-register
# 1. Generate new AI DevID keypair
python 14-aidevs/generate_keypairs.py --identity consensus-dev01-v2

# 2. Update DID document
python 14-aidevs/generate_did_documents.py --identity consensus-dev01-v2

# 3. Register new DevID on-chain via governance
# 4. Update all validators using old DevID
```

---

## 📈 Economics: Validator Rewards

### Reward Calculation

```rust
// Example: validator-01 (Director, 128 ËTR stake)

Base reward per epoch (1 week):
- Director: 20 ËTR base

Performance multiplier:
- Uptime: 100% → 1.0
- Blocks produced: 100% → 1.0
- Finality votes: 100% → 1.0
- Average: 1.0

Epoch reward = 20 ËTR × 1.0 = 20 ËTR

Annual reward = 20 ËTR × 52 weeks = 1,040 ËTR
APY = 1,040 / 128 = 8.125%
```

### Payment Flow

```
Week 1: Validator produces 1,008 blocks (1 per 60 slots, 7 days)
├─ Block rewards: 10 ËTR × 1,008 = 10,080 ËTR
├─ Finality rewards: 0.1 ËTR × 1,008 = 100.8 ËTR
└─ Total earned: 10,180.8 ËTR

Payment:
├─ Sent to: Payment account (5DfhG...)
├─ Available for: Withdrawal, re-staking, governance
└─ Taxed: 10% to treasury (1,018.08 ËTR)

Net payment: 9,162.72 ËTR per week
```

---

## 🚀 Complete Deployment Checklist

### Phase 1: Key Generation (Day 1)

- [ ] Run `generate-21-validators-with-payment-aidevid.sh`
- [ ] Verify all keys stored in Azure Key Vault
- [ ] Export payment account addresses
- [ ] Verify AI DevID mappings
- [ ] Backup Key Vault to offline storage

### Phase 2: Chain Spec (Day 2)

- [ ] Update chain spec with session keys
- [ ] Add staking records (session → payment → controller)
- [ ] Add AI DevID registry mappings
- [ ] Fund payment accounts in genesis balances
- [ ] Fund controller accounts for tx fees
- [ ] Convert to raw format

### Phase 3: VM Deployment (Day 3-5)

- [ ] Deploy 21 VMs on Azure
- [ ] Configure Managed Identity access to Key Vault
- [ ] Insert session keys into each validator
- [ ] Save payment account addresses locally
- [ ] Save AI DevID references
- [ ] Start validators

### Phase 4: Registration (Day 6)

- [ ] Register all 21 AI DevIDs on-chain
- [ ] Verify payment account mappings
- [ ] Test reward distribution
- [ ] Verify committee formation

### Phase 5: Monitoring (Day 7)

- [ ] Set up reward monitoring dashboard
- [ ] Configure alerts for missed payments
- [ ] Test reward withdrawal to cold storage
- [ ] Verify AI DevID signatures

---

## 📞 Troubleshooting

### Issue: Payment account not receiving rewards

**Diagnosis:**
```bash
# Check if staking record exists
curl -s http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params":["pallet_staking::ValidatorStaking"]}' \
  | jq
```

**Solution:**
- Verify chain spec includes staking record
- Ensure session account → payment account mapping
- Check if validator is in active set

### Issue: AI DevID signature verification failed

**Diagnosis:**
```bash
# Check DevID registration
curl -s http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "state_getStorage", "params":["pallet_validator_committee::ValidatorDevIDs"]}' \
  | jq
```

**Solution:**
- Verify AI DevID pubkey matches keypairs.json
- Re-sign registration with correct private key
- Check DID document is up-to-date

---

## 💡 Advanced: AI DevID-Signed Actions

```rust
// Example: Validator signs governance vote with AI DevID

use ed25519_dalek::{Keypair, Signature, Signer};

// Load AI DevID private key
let devid_secret = hex::decode(AIDEVID_PRIVATE_HEX)?;
let devid_keypair = Keypair::from_bytes(&devid_secret)?;

// Sign vote
let vote_msg = format!("vote:proposal:123:aye");
let signature: Signature = devid_keypair.sign(vote_msg.as_bytes());

// Submit to chain
submit_vote(
    validator_account,
    proposal_id: 123,
    vote: Vote::Aye,
    devid_signature: signature.to_bytes(),
);
```

**On-chain verification:**
```rust
// In pallet-governance
fn verify_devid_vote(
    validator: &AccountId,
    vote_data: &[u8],
    signature: &[u8],
) -> bool {
    // Get validator's AI DevID pubkey
    let devid = ValidatorDevIDs::<T>::get(validator)?;

    // Verify signature
    let pubkey = ed25519_dalek::PublicKey::from_bytes(&devid.devid_pubkey)?;
    pubkey.verify(vote_data, &Signature::from_bytes(signature)?).is_ok()
}
```

---

## 📄 Summary

This integration provides:

✅ **Payment accounts** for each validator to receive rewards
✅ **AI DevID integration** for verifiable validator identity
✅ **Complete key hierarchy** (4 key types per validator)
✅ **Secure key storage** (Hot → Warm → Cold)
✅ **On-chain registration** of AI Dev identities
✅ **Reward monitoring** and withdrawal automation
✅ **Signature verification** for AI-signed actions

**Next Steps:**
1. Run updated key generation script
2. Update chain spec with payment + DevID mappings
3. Deploy validators with complete key setup
4. Register AI DevIDs on-chain
5. Monitor reward distribution

**Questions?** Review the validator key hierarchy diagram or check Azure Key Vault logs.
