# AI Devs On-Chain Integration Plan
## Fully Integrated AI Agents for FlareChain Validators

**Created:** November 4, 2025
**Goal:** Make AI Devs a standard part of every FlareChain validator node stack
**Timeline:** 4-6 weeks for MVP, 3 months for full deployment

---

## 🎯 Vision

**Every FlareChain validator runs their own AI Devs instance:**
- Autonomous monitoring of their node
- Auto-compile FlareChain updates
- Generate governance proposals
- Security auditing
- Economic analysis
- Distributed AI network across all 21 validators

---

## 📊 Current State vs Target State

### Current State (Nov 4, 2025)
```
AI Devs:
├─ Status: Scaffolding complete
├─ Agents: 6 agents defined
├─ Skills: 27/29 loaded (but not implemented)
├─ Integration: None - standalone system
├─ Deployment: Manual Docker Compose
└─ On-chain: No connection

FlareChain:
├─ Validators: 16/21 active
├─ Consensus: Working (ASF + GRANDPA)
├─ Block Height: ~7,000+
└─ AI Integration: None
```

### Target State (3 months)
```
AI Devs:
├─ Status: Fully operational
├─ Agents: 6 agents implemented & tested
├─ Skills: 29/29 fully functional
├─ Integration: Deep blockchain integration
├─ Deployment: One-command validator add-on
└─ On-chain: DID-based agent registration

FlareChain:
├─ Validators: 21/21 active (each with AI Devs)
├─ AI Pallet: On-chain agent registry
├─ AI Network: 21 distributed AI agents
└─ Governance: AI-assisted proposals
```

---

## 🏗️ Architecture: On-Chain AI Integration

### Layer 1: Blockchain Integration

**New FlareChain Pallet: `pallet-ai-agents`**

```rust
// Substrate pallet for AI agent registration and coordination
pub struct AiAgent {
    did: Vec<u8>,           // Decentralized ID (did:etrid:compiler-01)
    agent_type: AgentType,  // Compiler, Governance, Security, etc.
    owner: AccountId,       // Validator who runs this agent
    endpoint: Vec<u8>,      // HTTP endpoint for agent
    stake: Balance,         // Optional stake requirement
    reputation: u32,        // Performance score
    status: AgentStatus,    // Active, Inactive, Slashed
}

pub enum AgentType {
    Compiler,
    Governance,
    Runtime,
    Economics,
    Security,
    Oracle,
}

pub enum AgentStatus {
    Active,
    Inactive,
    Slashed(SlashingReason),
}

// Storage
pub type AgentRegistry = StorageMap<DID, AiAgent>;
pub type ValidatorAgents = StorageMap<AccountId, Vec<DID>>;
pub type AgentActions = StorageMap<DID, Vec<AgentAction>>;
```

**Key Functions:**
```rust
// Extrinsics (callable by validators)
register_agent(did: Vec<u8>, agent_type: AgentType, endpoint: Vec<u8>)
deregister_agent(did: Vec<u8>)
update_agent_status(did: Vec<u8>, status: AgentStatus)
report_agent_action(did: Vec<u8>, action: AgentAction)

// Agent actions logged on-chain
pub enum AgentAction {
    CompileCompleted { commit_hash: Vec<u8>, success: bool },
    ProposalGenerated { proposal_id: u32 },
    SecurityAuditCompleted { findings: Vec<u8> },
    NodeHealthCheck { status: HealthStatus },
}
```

**Benefits:**
- ✅ Validators prove they're running AI Devs
- ✅ On-chain reputation system for AI quality
- ✅ Slashing for misbehaving AI agents
- ✅ Coordination between validators' AI agents

---

### Layer 2: Validator Node Stack Integration

**Standard Validator Deployment:**
```
┌─────────────────────────────────────────────────┐
│  Validator VM (e.g., Gizzi)                     │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────────────────────────────────┐  │
│  │  FlareChain Node (flarechain-validator)  │  │
│  │  - Block production (AURA)                │  │
│  │  - Finality voting (GRANDPA + ASF)        │  │
│  │  - RPC: ws://localhost:9944               │  │
│  └──────────────────────────────────────────┘  │
│           ↕ WebSocket RPC                       │
│  ┌──────────────────────────────────────────┐  │
│  │  AI Devs Orchestrator                     │  │
│  │  - HTTP API: localhost:4000               │  │
│  │  - 6 AI agents running                    │  │
│  │  - Connected to blockchain                │  │
│  │  - DID: did:etrid:validator:ACCOUNT_ID   │  │
│  └──────────────────────────────────────────┘  │
│           ↕                                      │
│  ┌──────────────────────────────────────────┐  │
│  │  VectorDB (Agent Memory)                  │  │
│  │  - Persistent skill execution history     │  │
│  └──────────────────────────────────────────┘  │
│                                                  │
│  ┌──────────────────────────────────────────┐  │
│  │  LLM Backend (Choose one)                 │  │
│  │  - Ollama (local, free)                   │  │
│  │  - Claude API (cloud, paid)               │  │
│  │  - GPT-4 API (cloud, paid)                │  │
│  └──────────────────────────────────────────┘  │
│                                                  │
└─────────────────────────────────────────────────┘
```

**Services:**
- `flarechain-validator.service` (already running)
- `ai-devs.service` (new)
- `vectordb.service` (new)
- `ollama.service` (optional, if using local LLM)

---

### Layer 3: AI Network Coordination

**Distributed AI Agent Network:**
```
Validator 1 (Gizzi)          Validator 2 (Azure-VM1)      Validator 3 (AuditDev)
├─ Compiler AI-01            ├─ Compiler AI-02            ├─ Compiler AI-03
├─ Governance AI-01          ├─ Governance AI-02          ├─ Governance AI-03
├─ Security AI-01            ├─ Security AI-02            ├─ Security AI-03
...                          ...                          ...
        ↓                            ↓                            ↓
    ────────────────────────────────────────────────────────────────
                         FlareChain Blockchain
                      (pallet-ai-agents registry)
    ────────────────────────────────────────────────────────────────
                                  ↓
                    Consensus on AI actions:
                    - Governance proposals (need 11/21 AI agreement)
                    - Security findings (need 2/3 AI agreement)
                    - Compile results (majority wins)
```

**Coordination Mechanisms:**
1. **AI Consensus**: Multiple AIs vote on actions before submitting to chain
2. **Load Balancing**: Compile jobs distributed across validators
3. **Redundancy**: If one AI fails, others take over
4. **Quality Control**: AIs with low reputation get less weight

---

## 🔧 Implementation Roadmap

### Phase 1: Complete AI Skills Implementation (2 weeks)

**Finish scaffolded skills:**

#### 1.1 Compiler AI Skills
```python
# skills/compiler-dev/etrid-compile-build.py

import subprocess
import json

async def execute(context, parameters):
    """
    Compiles FlareChain from source
    Returns: build status, errors, warnings
    """
    # 1. Check for code changes
    repo_path = "/opt/etrid"
    result = subprocess.run(
        ["git", "pull"],
        cwd=repo_path,
        capture_output=True
    )

    # 2. Run cargo build
    build_result = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=repo_path,
        capture_output=True,
        timeout=600  # 10 min timeout
    )

    # 3. Parse errors with LLM if build fails
    if build_result.returncode != 0:
        errors = build_result.stderr.decode()

        # Ask Claude to analyze
        analysis = await context.llm.ask(
            f"Rust build failed with:\n{errors}\n\nWhat's the issue?"
        )

        return {
            "success": False,
            "errors": errors,
            "analysis": analysis,
            "action": "notify_governance"
        }

    # 4. Run tests
    test_result = subprocess.run(
        ["cargo", "test", "--release"],
        cwd=repo_path,
        capture_output=True
    )

    return {
        "success": True,
        "binary_path": f"{repo_path}/target/release/flarechain-node",
        "tests_passed": test_result.returncode == 0
    }
```

#### 1.2 Governance AI Skills
```python
# skills/governance-dev/proposal-generator.py

async def execute(context, parameters):
    """
    Generates governance proposal using Claude
    """
    proposal_type = parameters.get("proposal_type", "treasury")

    # 1. Query blockchain for current state
    chain_state = await context.blockchain.query_state([
        "treasury_balance",
        "active_validators",
        "current_epoch"
    ])

    # 2. Ask Claude to draft proposal
    prompt = f"""
    You are a FlareChain governance AI.

    Current state:
    - Treasury: {chain_state['treasury_balance']} ETR
    - Validators: {chain_state['active_validators']}/21 active
    - Epoch: {chain_state['current_epoch']}

    Generate a {proposal_type} proposal that would benefit the network.
    Include: title, description, requested amount (if treasury), rationale.
    """

    proposal = await context.llm.ask(prompt, model="claude-sonnet-4")

    # 3. Store in VectorDB for voting simulation
    await context.memory.store("governance_proposals", {
        "proposal": proposal,
        "timestamp": time.time(),
        "agent_did": context.agent.did
    })

    # 4. Simulate vote outcome
    vote_simulation = await simulate_vote(proposal, context)

    return {
        "proposal": proposal,
        "estimated_pass_probability": vote_simulation.pass_probability,
        "action": "submit_to_governance_forum" if vote_simulation.pass_probability > 0.6 else "revise"
    }
```

#### 1.3 Security AI Skills
```python
# skills/security-dev/security-hardening.py

async def execute(context, parameters):
    """
    Runs security audit on runtime
    """
    # 1. Get latest runtime code
    runtime_path = "/opt/etrid/runtime/flare-chain"

    # 2. Run static analysis
    audit_results = []

    # Cargo audit for known vulnerabilities
    audit = subprocess.run(
        ["cargo", "audit", "--json"],
        cwd=runtime_path,
        capture_output=True
    )

    vulnerabilities = json.loads(audit.stdout)

    # 3. Ask Claude to analyze code patterns
    dangerous_patterns = [
        "unsafe",
        ".unwrap()",
        "panic!",
        "overflow"
    ]

    for pattern in dangerous_patterns:
        grep = subprocess.run(
            ["grep", "-rn", pattern, "src/"],
            cwd=runtime_path,
            capture_output=True
        )

        if grep.stdout:
            occurrences = grep.stdout.decode()

            # Claude analyzes if pattern is dangerous
            analysis = await context.llm.ask(
                f"Is this Rust code pattern dangerous?\n{occurrences}"
            )

            audit_results.append({
                "pattern": pattern,
                "occurrences": occurrences,
                "severity": analysis.get("severity"),
                "recommendation": analysis.get("fix")
            })

    # 4. Report on-chain if critical findings
    critical_findings = [r for r in audit_results if r['severity'] == 'CRITICAL']

    if critical_findings:
        await context.blockchain.report_agent_action(
            context.agent.did,
            "SecurityAuditCompleted",
            {"critical_count": len(critical_findings)}
        )

    return {
        "vulnerabilities": len(vulnerabilities),
        "dangerous_patterns": len(audit_results),
        "critical_findings": critical_findings,
        "report": audit_results
    }
```

#### 1.4 Oracle AI Skills
```python
# skills/oracle-dev/price-feed.py

async def execute(context, parameters):
    """
    Fetches price data and submits to on-chain oracle
    """
    import aiohttp

    # 1. Fetch from multiple sources
    sources = [
        "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd",
        "https://api.binance.com/api/v3/ticker/price?symbol=ETHUSDT",
        "https://api.kraken.com/0/public/Ticker?pair=ETHUSD"
    ]

    prices = []
    async with aiohttp.ClientSession() as session:
        for source in sources:
            try:
                async with session.get(source) as resp:
                    data = await resp.json()
                    price = parse_price(data, source)
                    prices.append(price)
            except:
                pass

    # 2. Calculate median (more robust than average)
    median_price = sorted(prices)[len(prices)//2]

    # 3. Submit to on-chain oracle pallet
    await context.blockchain.submit_extrinsic(
        "oracle",
        "update_price",
        {
            "asset": "ETH/USD",
            "price": median_price,
            "timestamp": int(time.time())
        }
    )

    return {
        "asset": "ETH/USD",
        "price": median_price,
        "sources": len(prices),
        "submitted": True
    }
```

**Complete all 29 skills similarly**

---

### Phase 2: On-Chain Pallet Development (1.5 weeks)

**Add `pallet-ai-agents` to FlareChain runtime:**

```bash
cd ~/Desktop/etrid/05-multichain/flare-chain

# Create new pallet
cargo generate --git https://github.com/paritytech/substrate-node-template \
    --name pallet-ai-agents pallets/ai-agents

# Implement storage and extrinsics
```

**File: `pallets/ai-agents/src/lib.rs`**

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    // AI Agent structure
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct AiAgent<AccountId> {
        pub did: BoundedVec<u8, ConstU32<128>>,
        pub agent_type: AgentType,
        pub owner: AccountId,
        pub endpoint: BoundedVec<u8, ConstU32<256>>,
        pub reputation: u32,
        pub status: AgentStatus,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum AgentType {
        Compiler,
        Governance,
        Runtime,
        Economics,
        Security,
        Oracle,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum AgentStatus {
        Active,
        Inactive,
        Slashed,
    }

    // Storage
    #[pallet::storage]
    pub type AgentRegistry<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, ConstU32<128>>, // DID
        AiAgent<T::AccountId>,
    >;

    #[pallet::storage]
    pub type ValidatorAgents<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId, // Validator account
        BoundedVec<BoundedVec<u8, ConstU32<128>>, ConstU32<10>>, // List of DIDs
        ValueQuery,
    >;

    // Events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AgentRegistered { did: Vec<u8>, owner: T::AccountId, agent_type: AgentType },
        AgentDeregistered { did: Vec<u8> },
        AgentActionReported { did: Vec<u8>, action: Vec<u8> },
        AgentSlashed { did: Vec<u8>, reason: Vec<u8> },
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        AgentAlreadyExists,
        AgentNotFound,
        NotAgentOwner,
        InvalidEndpoint,
        TooManyAgents,
    }

    // Extrinsics
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new AI agent
        #[pallet::weight(10_000)]
        pub fn register_agent(
            origin: OriginFor<T>,
            did: Vec<u8>,
            agent_type: AgentType,
            endpoint: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let did_bounded: BoundedVec<u8, ConstU32<128>> = did.clone().try_into()
                .map_err(|_| Error::<T>::InvalidEndpoint)?;

            // Check not already registered
            ensure!(
                !AgentRegistry::<T>::contains_key(&did_bounded),
                Error::<T>::AgentAlreadyExists
            );

            let agent = AiAgent {
                did: did_bounded.clone(),
                agent_type: agent_type.clone(),
                owner: who.clone(),
                endpoint: endpoint.try_into().map_err(|_| Error::<T>::InvalidEndpoint)?,
                reputation: 100, // Starting reputation
                status: AgentStatus::Active,
            };

            // Store agent
            AgentRegistry::<T>::insert(&did_bounded, agent);

            // Add to validator's agent list
            ValidatorAgents::<T>::try_mutate(&who, |agents| {
                agents.try_push(did_bounded.clone())
                    .map_err(|_| Error::<T>::TooManyAgents)
            })?;

            Self::deposit_event(Event::AgentRegistered {
                did,
                owner: who,
                agent_type
            });

            Ok(())
        }

        /// Deregister an AI agent
        #[pallet::weight(10_000)]
        pub fn deregister_agent(
            origin: OriginFor<T>,
            did: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let did_bounded: BoundedVec<u8, ConstU32<128>> = did.clone().try_into()
                .map_err(|_| Error::<T>::InvalidEndpoint)?;

            let agent = AgentRegistry::<T>::get(&did_bounded)
                .ok_or(Error::<T>::AgentNotFound)?;

            // Verify ownership
            ensure!(agent.owner == who, Error::<T>::NotAgentOwner);

            // Remove from storage
            AgentRegistry::<T>::remove(&did_bounded);

            ValidatorAgents::<T>::mutate(&who, |agents| {
                agents.retain(|d| d != &did_bounded);
            });

            Self::deposit_event(Event::AgentDeregistered { did });

            Ok(())
        }

        /// Report an agent action
        #[pallet::weight(10_000)]
        pub fn report_agent_action(
            origin: OriginFor<T>,
            did: Vec<u8>,
            action: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let did_bounded: BoundedVec<u8, ConstU32<128>> = did.clone().try_into()
                .map_err(|_| Error::<T>::InvalidEndpoint)?;

            let agent = AgentRegistry::<T>::get(&did_bounded)
                .ok_or(Error::<T>::AgentNotFound)?;

            ensure!(agent.owner == who, Error::<T>::NotAgentOwner);

            // Log action (in production, might store in separate storage)
            Self::deposit_event(Event::AgentActionReported { did, action });

            Ok(())
        }
    }
}
```

**Add to runtime:**
```rust
// runtime/src/lib.rs

// Add pallet
impl pallet_ai_agents::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

construct_runtime!(
    pub struct Runtime {
        // ... existing pallets
        AiAgents: pallet_ai_agents,
    }
);
```

**Build and upgrade runtime:**
```bash
cargo build --release
# Create runtime upgrade proposal
```

---

### Phase 3: Validator Deployment Package (1 week)

**Create one-command installer for validators:**

**File: `install-ai-devs.sh`**

```bash
#!/bin/bash
# AI Devs Installer for FlareChain Validators
# Usage: curl -sSL https://raw.githubusercontent.com/etrid/etrid/main/install-ai-devs.sh | bash

set -e

echo "🤖 Installing AI Devs for FlareChain Validator..."

# 1. Detect validator setup
VALIDATOR_HOME=$(eval echo ~$(whoami))
ETRID_PATH="$VALIDATOR_HOME/etrid"

if [ ! -d "$ETRID_PATH" ]; then
    echo "❌ Etrid not found. Cloning..."
    git clone https://github.com/etrid/etrid.git "$ETRID_PATH"
fi

cd "$ETRID_PATH/14-aidevs"

# 2. Check prerequisites
echo "📋 Checking prerequisites..."

# Docker
if ! command -v docker &> /dev/null; then
    echo "Installing Docker..."
    curl -fsSL https://get.docker.com | sh
    sudo usermod -aG docker $(whoami)
fi

# Docker Compose
if ! command -v docker-compose &> /dev/null; then
    echo "Installing Docker Compose..."
    sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" \
        -o /usr/local/bin/docker-compose
    sudo chmod +x /usr/local/bin/docker-compose
fi

# 3. Configure LLM backend
echo "🧠 Choose AI backend:"
echo "1) Ollama (free, runs locally)"
echo "2) Claude API (paid, better quality)"
echo "3) GPT-4 API (paid, OpenAI)"
read -p "Select [1-3]: " backend_choice

case $backend_choice in
    1)
        echo "Installing Ollama..."
        curl -fsSL https://ollama.com/install.sh | sh
        ollama pull llama3
        echo "LLM_BACKEND=ollama" > .env
        echo "LLM_MODEL=llama3" >> .env
        ;;
    2)
        read -p "Enter Anthropic API key: " api_key
        echo "LLM_BACKEND=claude" > .env
        echo "LLM_MODEL=claude-sonnet-4" >> .env
        echo "ANTHROPIC_API_KEY=$api_key" >> .env
        ;;
    3)
        read -p "Enter OpenAI API key: " api_key
        echo "LLM_BACKEND=openai" > .env
        echo "LLM_MODEL=gpt-4" >> .env
        echo "OPENAI_API_KEY=$api_key" >> .env
        ;;
esac

# 4. Configure blockchain connection
VALIDATOR_ACCOUNT=$(grep -oP 'VALIDATOR_ACCOUNT=\K.*' ~/.bashrc || echo "")

if [ -z "$VALIDATOR_ACCOUNT" ]; then
    read -p "Enter your validator AccountId (SS58 address): " VALIDATOR_ACCOUNT
fi

echo "BLOCKCHAIN_ENABLED=true" >> .env
echo "BLOCKCHAIN_ENDPOINT=ws://localhost:9944" >> .env
echo "VALIDATOR_ACCOUNT=$VALIDATOR_ACCOUNT" >> .env

# 5. Generate DID for agents
AGENT_DID="did:etrid:validator:$(echo -n $VALIDATOR_ACCOUNT | md5sum | cut -d' ' -f1)"
echo "AGENT_DID_PREFIX=$AGENT_DID" >> .env

# 6. Start services
echo "🚀 Starting AI Devs..."
docker-compose up -d

# Wait for startup
sleep 10

# 7. Verify
echo "✅ Checking health..."
curl -s http://localhost:4000/health | jq . || echo "⚠️ AI Devs not responding yet (give it a minute)"

# 8. Register agents on-chain
echo "📝 Registering AI agents on FlareChain..."

# Create registration script
cat > register-agents.sh << 'REGISTER_EOF'
#!/bin/bash
# Registers AI agents with pallet-ai-agents

AGENTS=(
    "compiler:Compiler:http://localhost:4000/agents/compiler"
    "governance:Governance:http://localhost:4000/agents/governance"
    "runtime:Runtime:http://localhost:4000/agents/runtime"
    "economics:Economics:http://localhost:4000/agents/economics"
    "security:Security:http://localhost:4000/agents/security"
    "oracle:Oracle:http://localhost:4000/agents/oracle"
)

for agent_info in "${AGENTS[@]}"; do
    IFS=':' read -r name type endpoint <<< "$agent_info"

    echo "Registering $name..."

    # Call register_agent extrinsic
    curl -X POST http://localhost:9944 \
        -H "Content-Type: application/json" \
        -d "{
            \"jsonrpc\": \"2.0\",
            \"method\": \"author_submitExtrinsic\",
            \"params\": [{
                \"call\": \"aiAgents.registerAgent\",
                \"args\": [
                    \"$AGENT_DID-$name\",
                    \"$type\",
                    \"$endpoint\"
                ]
            }],
            \"id\": 1
        }"
done
REGISTER_EOF

chmod +x register-agents.sh

echo "
╔═══════════════════════════════════════════════════════════╗
║              AI Devs Installation Complete!               ║
╠═══════════════════════════════════════════════════════════╣
║                                                           ║
║  Services Running:                                        ║
║   • AI Devs Orchestrator: http://localhost:4000          ║
║   • VectorDB: http://localhost:6333                      ║
║   • Grafana: http://localhost:3000                       ║
║                                                           ║
║  Next Steps:                                              ║
║   1. Check status: curl localhost:4000/health            ║
║   2. View agents: curl localhost:4000/agents             ║
║   3. Register on-chain: ./register-agents.sh             ║
║                                                           ║
║  Logs: docker-compose logs -f ai-devs                    ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
"
```

**Make it downloadable:**
```bash
# Host on GitHub
cp install-ai-devs.sh ~/Desktop/etrid/scripts/
cd ~/Desktop/etrid
git add scripts/install-ai-devs.sh
git commit -m "Add AI Devs one-click installer"
git push
```

---

### Phase 4: Testing & Deployment (1.5 weeks)

**Test on all 3 VMs:**

```bash
# Test on Gizzi
ssh ubuntu@64.181.215.19
curl -sSL https://raw.githubusercontent.com/etrid/etrid/main/scripts/install-ai-devs.sh | bash

# Verify
curl localhost:4000/health

# Test on AuditDev
ssh audit-dev01@129.80.122.34
curl -sSL https://raw.githubusercontent.com/etrid/etrid/main/scripts/install-ai-devs.sh | bash

# Test on Azure VM1
ssh azureuser@20.69.26.209
curl -sSL https://raw.githubusercontent.com/etrid/etrid/main/scripts/install-ai-devs.sh | bash
```

**Automated Testing:**
```bash
# Create test suite
cd ~/Desktop/etrid/14-aidevs/tests

# Test script
./test-ai-devs.sh --validator=gizzi
./test-ai-devs.sh --validator=auditdev
./test-ai-devs.sh --validator=azure-vm1
```

---

## 📦 Deployment Timeline

### Week 1-2: Skill Implementation
- [ ] Day 1-3: Implement Compiler AI skills
- [ ] Day 4-6: Implement Governance AI skills
- [ ] Day 7-9: Implement Security/Oracle AI skills
- [ ] Day 10-14: Test all skills locally

### Week 3: On-Chain Integration
- [ ] Day 15-17: Build pallet-ai-agents
- [ ] Day 18-19: Runtime integration & testing
- [ ] Day 20-21: Deploy runtime upgrade to testnet

### Week 4: Validator Packaging
- [ ] Day 22-24: Create install script
- [ ] Day 25-26: Test on 3 validators
- [ ] Day 27-28: Documentation & refinement

### Week 5-6: Network Rollout
- [ ] Day 29-31: Deploy to 5 validators (Directors)
- [ ] Day 32-35: Deploy to remaining 16 validators
- [ ] Day 36-42: Monitor, fix issues, optimize

---

## 💰 Cost Analysis

### Per Validator Node

**One-Time Setup:**
- Development: $0 (you're building it)
- Installation: $0 (automated script)

**Monthly Operating Costs:**

**Option 1: Ollama (Free)**
- LLM: $0/month (runs locally)
- RAM: +2GB required
- CPU: +10% usage
- **Total: $0/month**

**Option 2: Claude API**
- API calls: ~100k tokens/day
- Cost: ~$15-30/month
- **Total: $15-30/month**

**Option 3: GPT-4**
- API calls: ~100k tokens/day
- Cost: ~$20-40/month
- **Total: $20-40/month**

**Recommendation for Validators:**
Start with **Ollama (free)** for basic automation, upgrade to Claude/GPT-4 if you want higher quality AI reasoning.

---

## 🎯 Success Metrics

### MVP Success (Week 4)
- [ ] 3 validators running AI Devs successfully
- [ ] All 29 skills functional
- [ ] Agents registered on-chain
- [ ] 0 critical bugs
- [ ] < 5 min installation time

### Network Success (Week 6)
- [ ] 21/21 validators running AI Devs
- [ ] > 95% uptime across all agents
- [ ] 10+ on-chain agent actions per day
- [ ] 1+ AI-generated governance proposal tested
- [ ] Positive validator feedback

### Long-Term Success (3 months)
- [ ] AI network generates 50% of governance proposals
- [ ] Auto-compilation working on 100% of commits
- [ ] Security audits running daily
- [ ] 0 missed oracle updates
- [ ] Validator satisfaction > 8/10

---

## 🚀 Quick Start (For You, Today)

**Want to get started right now?**

```bash
cd ~/Desktop/etrid/14-aidevs

# 1. Install Ollama (free LLM)
/bin/bash -c "$(curl -fsSL https://ollama.com/install.sh)"
ollama pull llama3

# 2. Configure
cp .env.example .env
echo "LLM_BACKEND=ollama" >> .env
echo "LLM_MODEL=llama3" >> .env
echo "BLOCKCHAIN_ENABLED=true" >> .env
echo "BLOCKCHAIN_ENDPOINT=ws://localhost:9944" >> .env

# 3. Start (assumes Docker is running)
docker-compose up -d

# 4. Verify
curl localhost:4000/health | jq
curl localhost:4000/agents | jq

# 5. Test a skill
curl -X POST localhost:4000/execute \
  -H "Content-Type: application/json" \
  -d '{"agent_name":"compiler","skill_name":"etrid-compile-build","parameters":{}}'
```

---

## 📚 Documentation to Create

1. **VALIDATOR_AI_DEVS_GUIDE.md** - How to install & maintain
2. **AI_SKILLS_REFERENCE.md** - What each skill does
3. **TROUBLESHOOTING.md** - Common issues & fixes
4. **PALLET_AI_AGENTS_SPEC.md** - On-chain pallet documentation
5. **AI_NETWORK_ARCHITECTURE.md** - How distributed AI works

---

**This plan transforms AI Devs from a prototype to production infrastructure integrated into every FlareChain validator! 🤖🚀**

Let me know which phase you want to start with!
