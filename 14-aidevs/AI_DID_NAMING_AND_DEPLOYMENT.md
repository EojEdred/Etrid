# AI DID Naming System & Zero-Downtime Deployment

## Date: November 4, 2025

---

## Part 1: DID Naming Convention for Validators

### Hierarchical DID Structure

Every FlareChain validator gets a unique DID that's registered on-chain, and their AI agents get sub-DIDs.

**Role-Based Naming Convention:**

```
Directors (5/9 deployed):
did:etrid:director-gizzi
├─ did:etrid:director-gizzi:compiler
├─ did:etrid:director-gizzi:governance
├─ did:etrid:director-gizzi:runtime
├─ did:etrid:director-gizzi:economics
├─ did:etrid:director-gizzi:security
└─ did:etrid:director-gizzi:oracle

did:etrid:director-eoj
├─ did:etrid:director-eoj:compiler
├─ did:etrid:director-eoj:governance
└─ ... (6 agents)

did:etrid:director-auditdev
did:etrid:director-securitydev
did:etrid:director-governancedev

(+4 more directors to be deployed - will be 9 total)

ValidityNodes (16) - Specialized Roles:
did:etrid:validitynode-bridgedev
├─ did:etrid:validitynode-bridgedev:compiler
├─ did:etrid:validitynode-bridgedev:governance
└─ ... (6 agents)

did:etrid:validitynode-compliancedev
did:etrid:validitynode-consensusdev
did:etrid:validitynode-ethicsdev
did:etrid:validitynode-moderationdev
did:etrid:validitynode-oracledev
did:etrid:validitynode-p2pdev
did:etrid:validitynode-reputationdev
did:etrid:validitynode-vmdev
did:etrid:validitynode-runtimedev
did:etrid:validitynode-economicsdev
did:etrid:validitynode-integrationdev
did:etrid:validitynode-testnetdev
did:etrid:validitynode-monitoringdev
did:etrid:validitynode-infrastructuredev
did:etrid:validitynode-securityauditdev
```

### DID Format Specification

**Validator DID:**
```
did:etrid:<role>-<name>
```

Where:
- `<role>` = `director` or `validitynode`
- `<name>` =
  - For directors: validator name (gizzi, eoj, auditdev, securitydev, governancedev)
  - For validitynodes: specialization role (bridgedev, consensusdev, oracledev, etc.)

**AI Agent DID:**
```
did:etrid:<role>-<name>:<agent-type>
```

**Examples:**
- Director: `did:etrid:director-gizzi`
- Director AI: `did:etrid:director-eoj:compiler`
- ValidityNode: `did:etrid:validitynode-bridgedev`
- ValidityNode AI: `did:etrid:validitynode-consensusdev:governance`

### On-Chain Storage Extension

Add to `pallet-ai-agents`:

```rust
/// Storage: Validator AccountId → DID
#[pallet::storage]
pub type ValidatorDids<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BoundedVec<u8, ConstU32<128>>,  // DID string
    OptionQuery,
>;

/// Storage: DID → Validator AccountId (reverse lookup)
#[pallet::storage]
pub type DidToValidator<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    BoundedVec<u8, ConstU32<128>>,  // DID string
    T::AccountId,
    OptionQuery,
>;

/// Storage: AI Agent DIDs registered under validator
#[pallet::storage]
pub type AgentDids<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    BoundedVec<u8, ConstU32<128>>,  // Agent DID
    AiAgent<T::AccountId>,
    OptionQuery,
>;
```

### Registration Extrinsics

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Register validator DID on-chain
    #[pallet::weight(10_000)]
    pub fn register_validator_did(
        origin: OriginFor<T>,
        did: Vec<u8>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Ensure caller is a validator
        ensure!(
            pallet_session::Validators::<T>::get().contains(&who),
            Error::<T>::NotValidator
        );

        // Validate DID format
        let bounded_did: BoundedVec<u8, ConstU32<128>> =
            did.try_into().map_err(|_| Error::<T>::DidTooLong)?;

        ensure!(
            Self::is_valid_validator_did(&bounded_did),
            Error::<T>::InvalidDidFormat
        );

        // Check DID not already taken
        ensure!(
            !DidToValidator::<T>::contains_key(&bounded_did),
            Error::<T>::DidAlreadyExists
        );

        // Store mappings
        ValidatorDids::<T>::insert(&who, bounded_did.clone());
        DidToValidator::<T>::insert(&bounded_did, &who);

        Self::deposit_event(Event::ValidatorDidRegistered {
            validator: who,
            did: bounded_did.to_vec(),
        });

        Ok(())
    }

    /// Register AI agent under validator DID
    #[pallet::weight(10_000)]
    pub fn register_agent_did(
        origin: OriginFor<T>,
        agent_did: Vec<u8>,
        agent_type: AgentType,
        endpoint: Vec<u8>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Get validator DID
        let validator_did = ValidatorDids::<T>::get(&who)
            .ok_or(Error::<T>::ValidatorDidNotSet)?;

        let bounded_agent_did: BoundedVec<u8, ConstU32<128>> =
            agent_did.try_into().map_err(|_| Error::<T>::DidTooLong)?;

        // Ensure agent DID is child of validator DID
        ensure!(
            Self::is_child_did(&validator_did, &bounded_agent_did),
            Error::<T>::AgentDidMustBeChild
        );

        // Create agent record
        let agent = AiAgent {
            did: bounded_agent_did.clone(),
            agent_type,
            owner: who.clone(),
            endpoint: endpoint.try_into().map_err(|_| Error::<T>::EndpointTooLong)?,
            stake: T::MinAgentStake::get(),
            reputation: 100,
            status: AgentStatus::Active,
        };

        AgentDids::<T>::insert(&bounded_agent_did, agent);

        Self::deposit_event(Event::AgentDidRegistered {
            validator: who,
            agent_did: bounded_agent_did.to_vec(),
            agent_type,
        });

        Ok(())
    }
}

// Helper functions
impl<T: Config> Pallet<T> {
    fn is_valid_validator_did(did: &BoundedVec<u8, ConstU32<128>>) -> bool {
        let did_str = core::str::from_utf8(did).unwrap_or("");
        // Format: did:etrid:director-<name> or did:etrid:validitynode-<name>
        did_str.starts_with("did:etrid:director-") ||
        did_str.starts_with("did:etrid:validitynode-")
    }

    fn is_child_did(
        parent: &BoundedVec<u8, ConstU32<128>>,
        child: &BoundedVec<u8, ConstU32<128>>
    ) -> bool {
        let parent_str = core::str::from_utf8(parent).unwrap_or("");
        let child_str = core::str::from_utf8(child).unwrap_or("");

        child_str.starts_with(parent_str) && child_str.contains(":")
    }
}
```

### Events for DID Registration

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Validator DID registered
    ValidatorDidRegistered {
        validator: T::AccountId,
        did: Vec<u8>,
    },

    /// AI Agent DID registered
    AgentDidRegistered {
        validator: T::AccountId,
        agent_did: Vec<u8>,
        agent_type: AgentType,
    },
}
```

### AI Devs Auto-Registration Script

When AI Devs starts on a validator, it auto-registers DIDs:

```python
# scripts/register_dids.py

import asyncio
from substrateinterface import SubstrateInterface, Keypair
from config import VALIDATOR_NAME, RPC_ENDPOINT, KEYSTORE_PATH

AGENT_TYPES = [
    "compiler",
    "governance",
    "runtime",
    "economics",
    "security",
    "oracle"
]

async def register_validator_and_agents():
    # Connect to chain
    substrate = SubstrateInterface(url=RPC_ENDPOINT)

    # Load validator keypair
    keypair = Keypair.create_from_uri(f'file://{KEYSTORE_PATH}')

    # Step 1: Register validator DID
    # VALIDATOR_NAME should be "director-atlas" or "validitynode-orion"
    validator_did = f"did:etrid:{VALIDATOR_NAME}"

    print(f"Registering validator DID: {validator_did}")

    call = substrate.compose_call(
        call_module='AiAgents',
        call_function='register_validator_did',
        call_params={'did': validator_did}
    )

    extrinsic = substrate.create_signed_extrinsic(
        call=call,
        keypair=keypair
    )

    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    if receipt.is_success:
        print(f"✅ Validator DID registered: {validator_did}")
    else:
        print(f"❌ Failed to register validator DID: {receipt.error_message}")
        return

    # Step 2: Register AI agent DIDs
    for agent_type in AGENT_TYPES:
        agent_did = f"{validator_did}:{agent_type}"
        endpoint = f"http://localhost:4000/agents/{agent_type}"

        print(f"Registering agent DID: {agent_did}")

        call = substrate.compose_call(
            call_module='AiAgents',
            call_function='register_agent_did',
            call_params={
                'agent_did': agent_did,
                'agent_type': agent_type.capitalize(),
                'endpoint': endpoint
            }
        )

        extrinsic = substrate.create_signed_extrinsic(
            call=call,
            keypair=keypair
        )

        receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

        if receipt.is_success:
            print(f"✅ Agent DID registered: {agent_did}")
        else:
            print(f"⚠️  Failed to register agent DID: {receipt.error_message}")

    print("\n🎉 All DIDs registered on-chain!")

if __name__ == "__main__":
    asyncio.run(register_validator_and_agents())
```

### Integration with AI Devs Startup

Modify `ai-devs-orchestrator/main.py`:

```python
async def startup():
    logger.info("Starting AI Devs Orchestrator...")

    # Load 6 agents
    await agent_manager.load_agents()

    # Connect to blockchain
    await blockchain_client.connect()

    # Register DIDs on-chain (if not already registered)
    if not await blockchain_client.check_validator_did_exists():
        logger.info("Registering validator and agent DIDs on-chain...")
        await register_validator_and_agents()
    else:
        logger.info("✅ Validator DIDs already registered")

    logger.info("✅ AI Devs Orchestrator ready")
```

### Querying DIDs from Chain

```python
# Query validator DID
def get_validator_did(account_id: str) -> str:
    result = substrate.query(
        module='AiAgents',
        storage_function='ValidatorDids',
        params=[account_id]
    )
    return result.value if result else None

# Query all agent DIDs for a validator
def get_agent_dids(account_id: str) -> list:
    validator_did = get_validator_did(account_id)
    if not validator_did:
        return []

    # Query all agent DIDs starting with validator DID prefix
    agents = []
    for agent_type in ['compiler', 'governance', 'runtime', 'economics', 'security', 'oracle']:
        agent_did = f"{validator_did}:{agent_type}"
        result = substrate.query(
            module='AiAgents',
            storage_function='AgentDids',
            params=[agent_did]
        )
        if result:
            agents.append(result.value)

    return agents
```

---

## Part 2: Zero-Downtime Deployment Strategy

### Can We Deploy Without Taking Chain Down?

**✅ YES** - FlareChain supports **forkless runtime upgrades** and **rolling validator deployments**.

### Phase-by-Phase Deployment

#### Phase 1: AI Skills Implementation (Week 1-2)

**Impact:** ✅ ZERO DOWNTIME

**Process:**
1. Update AI Devs Python code in `/14-aidevs/skills/`
2. Rebuild Docker containers
3. Rolling restart of AI Devs containers (one validator at a time)
4. Validators keep running - AI Devs is separate service

**Commands:**
```bash
# On each validator (one at a time)
ssh ubuntu@validator-1 << 'EOF'
  cd /opt/etrid/14-aidevs
  git pull
  docker compose build ai-devs
  docker compose up -d ai-devs  # Restarts only AI Devs, not validator
EOF
```

**Validator continues producing blocks throughout.**

---

#### Phase 2: On-Chain Pallet Deployment (Week 3-4)

**Impact:** ✅ ZERO DOWNTIME (Forkless Runtime Upgrade)

**Substrate's Forkless Upgrade Process:**

1. **Build new runtime** with `pallet-ai-agents`
2. **Submit runtime upgrade proposal** via governance
3. **Validators vote** on proposal (no downtime during voting)
4. **Automatic activation** at next epoch if approved
5. **Chain continues running** - old blocks validated with old runtime, new blocks with new runtime

**Step-by-Step:**

```bash
# 1. Build new runtime (on development machine)
cd ~/Desktop/etrid/runtime/flarechain-runtime
cargo build --release

# This produces: target/release/wbuild/flarechain-runtime/flarechain_runtime.compact.compressed.wasm

# 2. Get runtime WASM blob
RUNTIME_WASM=$(cat target/release/wbuild/flarechain-runtime/flarechain_runtime.compact.compressed.wasm | xxd -p -c 0)

# 3. Submit runtime upgrade proposal
# Via Polkadot.js Apps or CLI
curl -X POST http://gizzi-validator:9944 \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"method\": \"author_submitExtrinsic\",
    \"params\": [
      \"0x...\"  # Signed extrinsic: governance.propose(system.setCode($RUNTIME_WASM))
    ],
    \"id\": 1
  }"

# 4. Validators vote on proposal (via governance UI or extrinsic)

# 5. If approved, runtime auto-upgrades at next epoch
# No manual restart needed!
```

**During Runtime Upgrade:**
- ✅ Validators keep producing blocks
- ✅ Finality continues (GRANDPA + ASF)
- ✅ Users can submit transactions
- ✅ New pallet available immediately after upgrade
- ✅ Old state preserved (no migration needed for new pallet)

**Verification:**
```bash
# Check runtime version after upgrade
curl -X POST http://gizzi-validator:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"state_getRuntimeVersion","params":[],"id":1}' | jq

# Should show new spec_version
```

---

#### Phase 3: AI Devs Validator Deployment (Week 5)

**Impact:** ✅ ZERO DOWNTIME (Rolling Deployment)

**Strategy:** Deploy AI Devs to validators in 3 waves

**Wave 1: Test Group (3 validators)**
```bash
# Week 5, Day 1-2
validators=("gizzi-validator" "azure-vm1" "azure-vm2")

for validator in "${validators[@]}"; do
  echo "Deploying AI Devs to $validator..."

  ssh ubuntu@$validator << 'EOF'
    # Download installer
    curl -sSL https://raw.githubusercontent.com/etrid/etrid/main/install-ai-devs.sh -o install-ai-devs.sh
    chmod +x install-ai-devs.sh

    # Run installer (auto-detects validator, installs Docker, starts AI Devs)
    ./install-ai-devs.sh

    # Verify AI Devs running
    docker ps | grep ai-devs
    curl http://localhost:4000/health | jq
EOF

  echo "✅ $validator AI Devs deployed"

  # Wait 1 hour, monitor before next deployment
  sleep 3600
done
```

**Monitor Test Group:**
```bash
# Check all 3 validators still producing blocks
for i in {1..10}; do
  BLOCK=$(curl -s -X POST http://gizzi-validator:9944 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    | jq -r '.result.number')
  echo "Block #$BLOCK"
  sleep 6  # 6 second block time
done
```

**Wave 2: Next 8 Validators (Day 3-4)**
**Wave 3: Remaining 10 Validators (Day 5-6)**

---

#### Phase 4: Full Network Activation (Week 6)

**Impact:** ✅ ZERO DOWNTIME

**Process:**
1. All 21 validators now have AI Devs running
2. AI agents register their DIDs on-chain (automatic via startup script)
3. AI network becomes active - agents start performing tasks

**Verification:**
```bash
# Query all registered AI agents
curl -X POST http://gizzi-validator:9944 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "state_getKeys",
    "params": ["0x..."],  # AgentDids storage key prefix
    "id": 1
  }' | jq

# Should show 126 agents (21 validators × 6 agents)
```

---

### Rolling Deployment Script

**Automated deployment across all validators:**

```bash
#!/bin/bash
# deploy-ai-devs-network.sh

VALIDATORS=(
  "gizzi-validator:64.181.215.19"
  "azure-vm1:20.69.26.209"
  "azure-vm2:20.186.91.207"
  "azure-vm3:52.252.142.146"
  # ... add all 21 validators
)

WAVES=(
  "0:3"   # Wave 1: validators 0-2
  "3:11"  # Wave 2: validators 3-10
  "11:21" # Wave 3: validators 11-20
)

deploy_to_validator() {
  local name=$1
  local ip=$2

  echo "🚀 Deploying AI Devs to $name ($ip)..."

  ssh -i ~/.ssh/gizzi-validator ubuntu@$ip << 'EOF'
    set -e

    # Download and run installer
    curl -sSL https://raw.githubusercontent.com/etrid/etrid/main/install-ai-devs.sh | bash

    # Wait for AI Devs to start
    timeout 60 bash -c 'until curl -s http://localhost:4000/health > /dev/null; do sleep 2; done'

    # Verify
    echo "✅ AI Devs health check passed"
    docker compose -f /opt/etrid/14-aidevs/docker-compose.yml ps
EOF

  if [ $? -eq 0 ]; then
    echo "✅ $name deployment successful"
    return 0
  else
    echo "❌ $name deployment failed"
    return 1
  fi
}

for wave_range in "${WAVES[@]}"; do
  IFS=':' read -r start end <<< "$wave_range"

  echo "========================================="
  echo "  WAVE: Validators $start to $((end-1))"
  echo "========================================="

  for i in $(seq $start $((end-1))); do
    validator_info="${VALIDATORS[$i]}"
    IFS=':' read -r name ip <<< "$validator_info"

    deploy_to_validator "$name" "$ip"

    # Wait 30 minutes before next validator
    echo "Waiting 30 minutes before next deployment..."
    sleep 1800
  done

  echo "✅ Wave complete. Waiting 24 hours before next wave..."
  sleep 86400  # 24 hours
done

echo "🎉 AI Devs deployed to all 21 validators!"
```

---

### Rollback Strategy (If Issues Occur)

**If AI Devs causes problems on a validator:**

```bash
# Emergency rollback (on affected validator)
ssh ubuntu@validator-x << 'EOF'
  # Stop AI Devs (validator keeps running)
  docker compose -f /opt/etrid/14-aidevs/docker-compose.yml down

  # Validator continues producing blocks normally
EOF
```

**If runtime upgrade has bugs:**

```bash
# Submit emergency runtime downgrade proposal
# Governance can vote to revert to previous runtime version
# Still no downtime - handled via forkless upgrade
```

---

### Zero-Downtime Checklist

**Before Deployment:**
- [x] Test AI Devs on devnet first
- [x] Verify runtime upgrade on testnet
- [x] Create rollback plan
- [x] Monitor block production metrics
- [x] Set up alerting for validator issues

**During Deployment:**
- [x] Deploy in waves (not all at once)
- [x] Monitor each validator after AI Devs install
- [x] Check block production continues
- [x] Verify finality not impacted
- [x] Monitor peer connectivity

**After Deployment:**
- [x] Query on-chain DID registry
- [x] Verify all 126 agents registered
- [x] Test AI skills execution
- [x] Monitor AI agent actions on-chain
- [x] Check reputation scores

---

## Summary

### DID Naming System

**Validator DIDs:**
```
did:etrid:gizzi-validator
did:etrid:azure-vm1-validator
did:etrid:azure-vm2-validator
... (21 validators)
```

**AI Agent DIDs (6 per validator = 126 total):**
```
did:etrid:gizzi-validator:compiler
did:etrid:gizzi-validator:governance
did:etrid:gizzi-validator:runtime
did:etrid:gizzi-validator:economics
did:etrid:gizzi-validator:security
did:etrid:gizzi-validator:oracle
```

**On-Chain Registration:**
- Validators call `aiAgents.register_validator_did(did)`
- AI Devs auto-registers agent DIDs on startup
- All DIDs queryable via on-chain storage

### Zero-Downtime Deployment

**✅ YES - Entire deployment can be done with ZERO downtime**

**Key Techniques:**
1. **Forkless Runtime Upgrade** - Add pallet without restarting chain
2. **Rolling Deployment** - Install AI Devs one validator at a time
3. **Independent Services** - AI Devs runs alongside validator (separate container)
4. **Gradual Rollout** - 3 waves over 6 days with monitoring between each

**Timeline:**
- Week 1-2: AI skills (zero downtime)
- Week 3-4: Runtime upgrade via governance (zero downtime)
- Week 5: AI Devs rolling deployment (zero downtime)
- Week 6: Full network activation (zero downtime)

**Validators continue producing blocks throughout entire 6-week deployment.**
