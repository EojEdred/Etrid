# Next Session: AI Devs DID Integration & Digital Footprint

**Session Goal:** Transform AI Devs from local skill packs into on-chain identities with digital presence

**Status:** Ready to Begin
**Estimated Time:** 2-3 hours
**Priority:** HIGH (Critical path for AI governance visibility)

---

## 📋 Session Todo List

### Part 1: Enhance AI Devs Robustness

#### 1.1 Shared Knowledge Layer
- [ ] Create `/ai-devs/skills/CLAUDE_SKILLS/` directory
- [ ] Extract skills from Claude packet into JSON skill cards
- [ ] Create skill card schema:
  ```json
  {
    "skill": "Risk Analysis",
    "description": "Evaluates proposal risk using Monte Carlo simulation",
    "available_to": ["governance-dev", "audit-dev"],
    "parameters": {},
    "returns": {},
    "claude_prompt": "..."
  }
  ```
- [ ] Add skill cards for:
  - [ ] Risk Analysis (governance, audit)
  - [ ] Code Review (compiler, audit)
  - [ ] Security Audit (security, audit)
  - [ ] Economic Modeling (economics, oracle)
  - [ ] Consensus Analysis (consensus, runtime)
  - [ ] Legal Compliance Check (governance, audit, ethics)
  - [ ] Performance Benchmarking (compiler, runtime)
  - [ ] Proposal Generation (governance)

#### 1.2 MCP Orchestration Hooks
- [ ] Create `mcp_config.yaml` template for AI Devs
- [ ] Add to each AI Dev pack:
  ```yaml
  connectors:
    - claude_instance: "claude://skills"
    - gizzi_core: "gizzi://orchestration"
    - vector_db: "qdrant://etrid-memory"

  skills_path: "../skills/CLAUDE_SKILLS/"
  memory_path: "./MEMORY.md"
  global_memory_path: "../GLOBAL_MEMORY.md"
  ```
- [ ] Update each of 12 AI Dev packs with mcp_config.yaml
- [ ] Test MCP routing to Claude skills

#### 1.3 Cross-Peer Memory
- [ ] Create `/ai-devs/GLOBAL_MEMORY.md` (shared across all devs)
- [ ] Add memory protocol:
  ```markdown
  # Global AI Dev Memory

  ## [2025-10-24 18:30 UTC] oracle-dev
  **Event:** Anomalous reserve ratio detected (EDSC/BTC ratio off by 2.3%)
  **Action:** Flagged for audit-dev review
  **Status:** PENDING

  ## [2025-10-24 18:35 UTC] audit-dev
  **Event:** Reviewed oracle-dev flag #1234
  **Finding:** Within acceptable variance, no action needed
  **Status:** RESOLVED
  ```
- [ ] Update each Dev's MEMORY.md to reference GLOBAL_MEMORY.md
- [ ] Add "memory sync" task to orchestrator (every 5 minutes)

#### 1.4 Audit Trail Layer
- [ ] Create `/ai-devs/logs/` directory structure:
  ```
  /logs/
    /consensus-dev/
    /governance-dev/
    /audit-dev/
    ...
  ```
- [ ] Add logging middleware to orchestrator
- [ ] Log format:
  ```json
  {
    "timestamp": "2025-10-24T18:30:00Z",
    "dev_id": "consensus-dev",
    "action": "skill_execution",
    "skill": "validator-rotation",
    "result": "success",
    "metadata": {}
  }
  ```
- [ ] Set up log streaming to Git repo (automated commits)
- [ ] Optional: Stream logs to Notion via API

---

### Part 2: AI DID (Decentralized Identifiers)

#### 2.1 Create DID Documents
- [ ] Create `/ai-devs/dids/` directory
- [ ] Generate DID document for each of 12 AI Devs:

  **List of DIDs to create:**
  1. `did:etrid:consensus-dev01` (Consensus Dev)
  2. `did:etrid:compiler-dev01` (Compiler Dev)
  3. `did:etrid:governance-dev01` (Governance Dev)
  4. `did:etrid:audit-dev01` (Audit Dev)
  5. `did:etrid:oracle-dev01` (Oracle Dev)
  6. `did:etrid:runtime-dev01` (Runtime Dev)
  7. `did:etrid:economics-dev01` (Economics Dev)
  8. `did:etrid:edsc-dev01` (EDSC Stablecoin Dev)
  9. `did:etrid:security-dev01` (Security Dev)
  10. `did:etrid:multichain-dev01` (Multichain Integration Dev)
  11. `did:etrid:ethics-dev01` (Ethics & Legal Dev)
  12. `did:etrid:docs-dev01` (Documentation Dev)

- [ ] Generate DID documents for Gizzi personas:
  - `did:etrid:gizzi` (Main Gizzi - Lead AI Dev)
  - `did:etrid:gizzi-claude` (GizziClaude - Deep Reasoning)
  - `did:etrid:gizzi-claudecode` (GizziClaudeCode - Rust/Substrate Builder)

#### 2.2 DID Document Structure
- [ ] Create DID template with:
  - Unique ID (did:etrid:*)
  - Controller (did:etrid:gizzi for all AI Devs)
  - Public key (Ed25519)
  - Service endpoints (MCP, Docker, P2P)
  - Metadata (Twitter, GitHub, memory/skill refs)

#### 2.3 Generate Keypairs
- [ ] Generate Ed25519 keypairs for each AI Dev
- [ ] Store private keys securely (encrypted, offline backup)
- [ ] Store public keys in DID documents
- [ ] Create key rotation procedure

---

### Part 3: Digital Footprint (Twitter/Social)

#### 3.1 Twitter Strategy Decision
- [ ] **Option A (Recommended for MVP):** One consolidated account
  - Handle: `@EtridAI_Devs`
  - Bio: "12 autonomous AI Devs building the Ëtrid blockchain + governance stack. Powered by E³20. Overseen by Gizzi (Lead AI Dev)."
  - Each Dev posts in their voice (threads)

- [ ] **Option B (Future):** Individual accounts per dev
  - Reserve handles: @EtridConsensus, @EtridGovernance, etc.
  - Gizzi main account: @EtridGizzi
  - All accounts follow each other

**Decision:** Start with Option A for easier ops

#### 3.2 Create Twitter Presence
- [ ] Reserve Twitter handle: `@EtridAI_Devs`
- [ ] Write bio (link to GitHub, DID registry)
- [ ] Create profile image (Ëtrid logo + "AI Devs")
- [ ] Create banner image (12 AI Devs visualization)
- [ ] Link to:
  - GitHub: github.com/etrid/ai-devs (TBD - public repo or wiki)
  - DID Registry: etrid.network/dids (TBD - web interface)
  - Block Explorer: explorer.etrid.network (TBD)

#### 3.3 Write Dev Bios (aligned with SKILL.md)
- [ ] consensus-dev: "I maintain PPFA consensus + validator rotation. Adaptive slot timing is my specialty. #ËtridConsensus"
- [ ] compiler-dev: "Rust + Substrate compiler. I build what the chain runs. Benchmarking fanatic. #ËtridCompiler"
- [ ] governance-dev: "Proposal generator + bylaw enforcer. Democracy without drama. #ËtridGovernance"
- [ ] audit-dev: "Cross-checking proposals, code, and economics. Trust but verify. #ËtridAudit"
- [ ] oracle-dev: "Price feeds + reserve tracking. On-chain data oracle. #ËtridOracle"
- [ ] runtime-dev: "WebAssembly runtime + ETWasm VM. Execution layer specialist. #ËtridRuntime"
- [ ] economics-dev: "Reserve ratios, staking yields, inflation curves. Economics in code. #ËtridEconomics"
- [ ] edsc-dev: "Stablecoin bridge operator. EDSC/BTC/ETH reserves managed. #ËtridEDSC"
- [ ] security-dev: "Threat detection + security hardening. Defense in depth. #ËtridSecurity"
- [ ] multichain-dev: "Bridge protocols for Bitcoin, Ethereum, Flare. Multichain ops. #ËtridMultichain"
- [ ] ethics-dev: "Legal compliance + ethical AI oversight. Rules matter. #ËtridEthics"
- [ ] docs-dev: "Documentation generator. If it's not documented, it doesn't exist. #ËtridDocs"

#### 3.4 Example Tweet Content
- [ ] Create 5 sample tweets for launch:
  1. Introduction thread (Gizzi introduces the team)
  2. consensus-dev update (PPFA rotation test results)
  3. audit-dev finding (flagged proposal inconsistency)
  4. oracle-dev data (reserve ratio update)
  5. Gizzi reflection ("The Devs are alive and talking to each other")

---

### Part 4: OpenDID Integration (On-Chain)

#### 4.1 Review OpenDID Pallet
- [ ] Read `/pallets/pallet-did-registry/src/lib.rs`
- [ ] Understand DID registration extrinsics
- [ ] Check if DID resolution is implemented
- [ ] Verify DID document storage schema

#### 4.2 Extend OpenDID for AI Devs (if needed)
- [ ] Add AI Dev metadata fields to DID documents:
  - `dev_type` (enum: Consensus, Compiler, Governance, etc.)
  - `skill_ref` (IPFS hash or GitHub URL)
  - `memory_ref` (IPFS hash or GitHub URL)
  - `service_endpoint` (MCP URL, Docker endpoint, P2P)
  - `twitter_handle` (optional)
  - `github_repo` (optional)

#### 4.3 Register DIDs On-Chain
- [ ] Write script to batch-register all 15 DIDs (12 devs + 3 Gizzi personas)
- [ ] Use `pallet-did-registry::register_did()` extrinsic
- [ ] Fund accounts with ETR for transaction fees
- [ ] Submit transactions and verify on-chain storage

#### 4.4 DID Resolution Interface
- [ ] Create DID resolver API endpoint
- [ ] Input: `did:etrid:consensus-dev01`
- [ ] Output: Full DID document (JSON)
- [ ] Add to block explorer (DID lookup tab)

---

### Part 5: Repository Structure Updates

#### 5.1 Create New Directories
```
/ai-devs/
  /dids/                          # DID documents (JSON)
    consensus-dev01.json
    compiler-dev01.json
    ...
    gizzi.json
    gizzi-claude.json
    gizzi-claudecode.json

  /skills/
    /CLAUDE_SKILLS/               # Shared skill cards
      risk-analysis.json
      code-review.json
      security-audit.json
      ...

  /logs/                          # Audit trail logs
    /consensus-dev/
    /compiler-dev/
    ...

  /memory/
    GLOBAL_MEMORY.md              # Shared memory across all devs

  /packs/                         # Existing AI Dev zips
    01_consensus-dev.zip
    02_compiler-dev.zip
    ...

  DID_REGISTRY.md                 # Human-readable DID directory
  DIGITAL_FOOTPRINT.md            # Twitter/social presence guide
```

#### 5.2 Update Each AI Dev Pack
- [ ] Add `mcp_config.yaml` to each zip
- [ ] Add reference to GLOBAL_MEMORY.md
- [ ] Add reference to CLAUDE_SKILLS/
- [ ] Add DID reference in README.md
- [ ] Update SKILL.md with DID info

#### 5.3 Create Registry Documents
- [ ] `DID_REGISTRY.md` - List all DIDs with descriptions
- [ ] `DIGITAL_FOOTPRINT.md` - Twitter strategy, bios, example content
- [ ] `MCP_INTEGRATION.md` - How to orchestrate AI Devs with MCP
- [ ] `MEMORY_PROTOCOL.md` - How cross-peer memory works

---

### Part 6: Web Interface (Optional but Recommended)

#### 6.1 DID Registry Web Page
- [ ] Create page at `etrid.network/dids`
- [ ] List all 15 AI Dev DIDs
- [ ] Click to view full DID document
- [ ] Link to GitHub (skills/memory)
- [ ] Link to Twitter
- [ ] Show service status (online/offline)

#### 6.2 AI Devs Dashboard
- [ ] Create page at `etrid.network/ai-devs`
- [ ] Show all 12 AI Devs with avatars
- [ ] Display current status (via health API)
- [ ] Show recent memory updates
- [ ] Show recent logs
- [ ] Link to individual Dev pages

#### 6.3 Deploy to Vercel
- [ ] Add to existing `etrid-crypto-website` Next.js app
- [ ] Create `/app/ai-devs/` route
- [ ] Create `/app/dids/` route
- [ ] Deploy to production

---

### Part 7: Testing & Validation

#### 7.1 MCP Orchestration Test
- [ ] Test skill execution via MCP (consensus-dev calls risk-analysis skill)
- [ ] Verify CLAUDE_SKILLS/ routing works
- [ ] Verify GLOBAL_MEMORY.md sync works
- [ ] Test cross-peer communication (oracle-dev writes, audit-dev reads)

#### 7.2 DID Resolution Test
- [ ] Query DID via API: `GET /did/etrid:consensus-dev01`
- [ ] Verify DID document returned correctly
- [ ] Test on-chain DID lookup via block explorer

#### 7.3 Digital Footprint Test
- [ ] Post first tweet from @EtridAI_Devs
- [ ] Verify DID link resolves from Twitter bio
- [ ] Test tweet → memory log flow (tweet about consensus update, log in MEMORY.md)

---

## 📊 Success Criteria

- [ ] All 15 DIDs created and stored in `/dids/`
- [ ] All 15 DIDs registered on-chain via OpenDID pallet
- [ ] DID resolver API working (resolve any DID to full document)
- [ ] CLAUDE_SKILLS/ directory created with 8+ skill cards
- [ ] GLOBAL_MEMORY.md created and syncing across devs
- [ ] MCP orchestration working (1 test skill execution)
- [ ] Twitter account created and first 5 tweets posted
- [ ] DID Registry web page deployed and accessible
- [ ] All 12 AI Dev packs updated with mcp_config.yaml
- [ ] Audit trail logging working (logs written to /logs/)

---

## 🚀 Deliverables

### Documentation
1. `DID_REGISTRY.md` - Complete list of all AI Dev DIDs
2. `DIGITAL_FOOTPRINT.md` - Twitter strategy and content guide
3. `MCP_INTEGRATION.md` - How to orchestrate AI Devs
4. `MEMORY_PROTOCOL.md` - Cross-peer memory protocol

### Code/Data
1. `/dids/` folder with 15 DID documents (JSON)
2. `/skills/CLAUDE_SKILLS/` with 8+ skill cards
3. `/memory/GLOBAL_MEMORY.md` (initialized)
4. `/logs/` directory structure
5. Updated AI Dev packs with mcp_config.yaml

### On-Chain
1. 15 DIDs registered in OpenDID pallet
2. DID resolver extrinsic callable
3. DIDs queryable via block explorer

### Web/Social
1. Twitter account @EtridAI_Devs live with 5 tweets
2. DID Registry page at etrid.network/dids
3. AI Devs dashboard at etrid.network/ai-devs

---

## ⏰ Estimated Timeline

**Session 1 (2 hours):**
- Create DID documents for all 15 identities
- Generate keypairs
- Create CLAUDE_SKILLS/ directory with skill cards
- Create GLOBAL_MEMORY.md
- Create Twitter account and write bios

**Session 2 (2 hours):**
- Extend OpenDID pallet for AI Dev metadata
- Register all 15 DIDs on-chain
- Create DID resolver API
- Update all AI Dev packs with mcp_config.yaml

**Session 3 (2 hours):**
- Create DID Registry web page
- Deploy to Vercel
- Post first 5 tweets
- Test MCP orchestration with skill execution
- Write documentation (4 markdown files)

**Total Time:** 6 hours across 3 sessions

---

## 💡 Next Steps After This Session

1. **Activate AI Devs in Production**
   - Deploy orchestrator with full MCP integration
   - Connect to live Ëtrid blockchain node
   - Begin autonomous skill execution

2. **Expand Digital Presence**
   - Post daily dev logs to Twitter
   - Create GitHub organization for AI Devs
   - Set up Notion workspace for AI Dev wiki

3. **Governance Integration**
   - AI Devs participate in on-chain governance
   - Audit-dev reviews all proposals automatically
   - Governance-dev generates proposals for community vote

4. **Community Engagement**
   - Invite developers to interact with AI Devs via API
   - Create Discord bot tied to AI Devs (ask questions, get dev responses)
   - Host AMA with Gizzi and the AI Dev team

---

**Status:** Ready to begin in next session
**Priority:** HIGH - Critical for AI governance visibility and E³20 identity layer

---

*"From zips to DIDs — the AI Devs are about to get their digital souls."* 🚀
