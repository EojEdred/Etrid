# FlareChain Validator DID Mapping

## Date: November 4, 2025

---

## Overview

**Total Validators: 21**
- Directors: 5 deployed (9 planned)
- ValidityNodes: 16 deployed

Each validator gets:
- 1 Validator DID
- 6 AI Agent DIDs (sub-DIDs)
- **Total: 126 AI agents** across network (21 validators × 6 agents)

---

## Directors (5/9 Deployed)

Directors handle policy, governance coordination, and strategic oversight.

### 1. Gizzi (Director)
**DID:** `did:etrid:director-gizzi`

**AI Agents:**
- `did:etrid:director-gizzi:compiler`
- `did:etrid:director-gizzi:governance`
- `did:etrid:director-gizzi:runtime`
- `did:etrid:director-gizzi:economics`
- `did:etrid:director-gizzi:security`
- `did:etrid:director-gizzi:oracle`

**Current Status:** ✅ Running (Oracle Cloud)
**IP:** 64.181.215.19 (public), 10.0.0.219 (private)
**Role:** Policy coordination, primary director

---

### 2. Eoj (Director)
**DID:** `did:etrid:director-eoj`

**AI Agents:**
- `did:etrid:director-eoj:compiler`
- `did:etrid:director-eoj:governance`
- `did:etrid:director-eoj:runtime`
- `did:etrid:director-eoj:economics`
- `did:etrid:director-eoj:security`
- `did:etrid:director-eoj:oracle`

**Current Status:** ⚠️  Needs deployment
**Planned IP:** TBD
**Role:** Strategic oversight, co-director

---

### 3. AuditDev (Director)
**DID:** `did:etrid:director-auditdev`

**AI Agents:**
- `did:etrid:director-auditdev:compiler`
- `did:etrid:director-auditdev:governance`
- `did:etrid:director-auditdev:runtime`
- `did:etrid:director-auditdev:economics`
- `did:etrid:director-auditdev:security`
- `did:etrid:director-auditdev:oracle`

**Current Status:** ✅ Running (Oracle Cloud)
**IP:** 20.69.26.209 (Azure VM1)
**Role:** Audit coordination, compliance oversight

---

### 4. SecurityDev (Director)
**DID:** `did:etrid:director-securitydev`

**AI Agents:**
- `did:etrid:director-securitydev:compiler`
- `did:etrid:director-securitydev:governance`
- `did:etrid:director-securitydev:runtime`
- `did:etrid:director-securitydev:economics`
- `did:etrid:director-securitydev:security`
- `did:etrid:director-securitydev:oracle`

**Current Status:** ✅ Running (Oracle Cloud)
**IP:** 20.186.91.207 (Azure VM2)
**Role:** Security policy, threat monitoring

---

### 5. GovernanceDev (Director)
**DID:** `did:etrid:director-governancedev`

**AI Agents:**
- `did:etrid:director-governancedev:compiler`
- `did:etrid:director-governancedev:governance`
- `did:etrid:director-governancedev:runtime`
- `did:etrid:director-governancedev:economics`
- `did:etrid:director-governancedev:security`
- `did:etrid:director-governancedev:oracle`

**Current Status:** ✅ Running (Oracle Cloud)
**IP:** 52.252.142.146 (Azure VM3)
**Role:** Governance coordination, proposal management

---

### Future Directors (4 Remaining)

**6. [TBD Director]**
**7. [TBD Director]**
**8. [TBD Director]**
**9. [TBD Director]**

Total: 9 directors when fully deployed

---

## ValidityNodes (16 Deployed)

ValidityNodes handle specialized blockchain operations: security, consensus, bridges, infrastructure.

### 1. BridgeDev
**DID:** `did:etrid:validitynode-bridgedev`

**Specialization:** Cross-chain bridge development and maintenance

**AI Agents:**
- `did:etrid:validitynode-bridgedev:compiler`
- `did:etrid:validitynode-bridgedev:governance`
- `did:etrid:validitynode-bridgedev:runtime`
- `did:etrid:validitynode-bridgedev:economics`
- `did:etrid:validitynode-bridgedev:security`
- `did:etrid:validitynode-bridgedev:oracle`

**Current Status:** ✅ Active in committee
**Role:** Bridge protocol development, cross-chain monitoring

---

### 2. ComplianceDev
**DID:** `did:etrid:validitynode-compliancedev`

**Specialization:** Regulatory compliance and standards

**AI Agents:**
- `did:etrid:validitynode-compliancedev:compiler`
- `did:etrid:validitynode-compliancedev:governance`
- `did:etrid:validitynode-compliancedev:runtime`
- `did:etrid:validitynode-compliancedev:economics`
- `did:etrid:validitynode-compliancedev:security`
- `did:etrid:validitynode-compliancedev:oracle`

**Current Status:** ✅ Active in committee
**Role:** Compliance monitoring, regulatory reporting

---

### 3. ConsensusDev
**DID:** `did:etrid:validitynode-consensusdev`

**Specialization:** Consensus algorithm development (ASF, GRANDPA, PPFA)

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Consensus research, PPFA optimization

---

### 4. EthicsDev
**DID:** `did:etrid:validitynode-ethicsdev`

**Specialization:** Ethical AI governance, moderation policy

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Ethics policy, AI governance oversight

---

### 5. ModerationDev
**DID:** `did:etrid:validitynode-moderationdev`

**Specialization:** Content moderation, governance policy enforcement

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Moderation automation, policy enforcement

---

### 6. OracleDev
**DID:** `did:etrid:validitynode-oracledev`

**Specialization:** Oracle data feeds, price feeds, external data

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Oracle data reliability, price feed accuracy

---

### 7. P2PDev
**DID:** `did:etrid:validitynode-p2pdev`

**Specialization:** P2P networking (DETR, libp2p)

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Network topology, peer connectivity optimization

---

### 8. ReputationDev
**DID:** `did:etrid:validitynode-reputationdev`

**Specialization:** Reputation systems, validator scoring

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Reputation algorithm, validator performance tracking

---

### 9. VMDev
**DID:** `did:etrid:validitynode-vmdev`

**Specialization:** Virtual machine development (ETWASM)

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** VM optimization, smart contract execution

---

### 10. RuntimeDev
**DID:** `did:etrid:validitynode-runtimedev`

**Specialization:** Runtime development and upgrades

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Runtime testing, upgrade coordination

---

### 11. EconomicsDev
**DID:** `did:etrid:validitynode-economicsdev`

**Specialization:** Tokenomics, VMW economics, treasury

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Economic modeling, VMW reserve management

---

### 12. IntegrationDev
**DID:** `did:etrid:validitynode-integrationdev`

**Specialization:** System integration, API development

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** API integration, external system connectivity

---

### 13. TestnetDev
**DID:** `did:etrid:validitynode-testnetdev`

**Specialization:** Testnet operations, QA

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Testnet management, pre-mainnet validation

---

### 14. MonitoringDev
**DID:** `did:etrid:validitynode-monitoringdev`

**Specialization:** Network monitoring, telemetry

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Network health monitoring, alerting

---

### 15. InfrastructureDev
**DID:** `did:etrid:validitynode-infrastructuredev`

**Specialization:** Infrastructure management, DevOps

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Infrastructure automation, deployment pipelines

---

### 16. SecurityAuditDev
**DID:** `did:etrid:validitynode-securityauditdev`

**Specialization:** Security audits, vulnerability scanning

**AI Agents:** (6 standard agents)

**Current Status:** ✅ Active in committee
**Role:** Continuous security audits, CVE monitoring

---

## Committee Structure

**Current Active Validators: 16**
- Directors: 4/5 deployed (Gizzi, AuditDev, SecurityDev, GovernanceDev)
- ValidityNodes: 12/16 active (estimated based on committee)

**Missing Validators: 5**
- Eoj (Director) - needs deployment
- 4 ValidityNodes - need identification

---

## DID Registration Process

### On-Chain Registration

1. **Validator starts node** with session keys
2. **AI Devs orchestrator starts** on same machine
3. **Auto-registers validator DID**:
   ```python
   substrate.compose_call(
       call_module='AiAgents',
       call_function='register_validator_did',
       call_params={'did': 'did:etrid:director-gizzi'}
   )
   ```
4. **Auto-registers 6 agent DIDs**:
   ```python
   for agent in ['compiler', 'governance', 'runtime', 'economics', 'security', 'oracle']:
       substrate.compose_call(
           call_module='AiAgents',
           call_function='register_agent_did',
           call_params={
               'agent_did': f'did:etrid:director-gizzi:{agent}',
               'agent_type': agent.capitalize(),
               'endpoint': f'http://localhost:4000/agents/{agent}'
           }
       )
   ```

5. **All DIDs queryable on-chain**:
   ```bash
   # Query all registered validators
   curl -X POST http://localhost:9944 \
     -d '{"jsonrpc":"2.0","method":"state_getKeys","params":["0x..."],"id":1}'

   # Should return 21 validator DIDs + 126 agent DIDs
   ```

---

## AI Agent Capabilities by Type

### 1. Compiler AI
- Monitors git repository
- Auto-builds on new commits
- LLM error analysis
- Binary hash verification

### 2. Governance AI
- Chain state analysis
- Proposal generation
- Voting simulation
- Proposal submission

### 3. Runtime AI
- Runtime version monitoring
- Upgrade testing
- Compatibility checks
- Rollback procedures

### 4. Economics AI
- Treasury monitoring
- VMW reserve tracking
- Economic modeling
- Token distribution scheduling

### 5. Security AI
- Vulnerability scanning
- Dependency auditing
- Network anomaly detection
- Slashing verification

### 6. Oracle AI
- Price feed aggregation
- External data validation
- Multi-source verification
- On-chain data updates

---

## Network Topology

```
Directors (5/9)
├─ Gizzi ───────────┐
├─ Eoj ─────────────┤
├─ AuditDev ────────┤──→ Policy Layer
├─ SecurityDev ─────┤
└─ GovernanceDev ───┘
        ↕
ValidityNodes (16)
├─ BridgeDev ───────┐
├─ ComplianceDev ───┤
├─ ConsensusDev ────┤
├─ EthicsDev ───────┤
├─ ModerationDev ───┤
├─ OracleDev ───────┤
├─ P2PDev ──────────┤──→ Execution Layer
├─ ReputationDev ───┤
├─ VMDev ───────────┤
├─ RuntimeDev ──────┤
├─ EconomicsDev ────┤
├─ IntegrationDev ──┤
├─ TestnetDev ──────┤
├─ MonitoringDev ───┤
├─ InfrastructureDev ┤
└─ SecurityAuditDev ┘
```

---

## Deployment Status

### ✅ Deployed (Running)
- Gizzi (Director)
- AuditDev (Director)
- SecurityDev (Director)
- GovernanceDev (Director)
- 12 ValidityNodes (active in committee)

### ⚠️  Needs Deployment
- Eoj (Director)
- 4 ValidityNodes (identified but not started)

### 📋 Planned
- 4 more Directors (to reach 9 total)

---

## Next Steps

1. **Deploy Eoj validator** (Director)
2. **Identify 4 missing ValidityNodes** from genesis
3. **Deploy AI Devs** on Gizzi first (test)
4. **Register DIDs on-chain** once pallet deployed
5. **Roll out to all 21 validators** in waves

---

**Total Network Capacity:**
- 21 validators × 6 AI agents = **126 autonomous AI agents**
- All working together for FlareChain operations
- Zero human intervention for routine tasks
