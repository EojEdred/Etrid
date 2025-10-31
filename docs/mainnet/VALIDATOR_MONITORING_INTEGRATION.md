# Validator Monitoring Integration
## Connecting Existing AI Dev Skills + MCP + Monitoring Infrastructure

**Version:** 1.0
**Date:** 2025-10-31
**Purpose:** Integrate existing Ëtrid infrastructure for AI-powered validator monitoring

---

## 🎯 Overview

You already have all the pieces:

✅ **MCP_SOCIAL_AUTOMATION.md** - MCP orchestration framework
✅ **MONITORING_INFRASTRUCTURE_GUIDE.md** - Prometheus + Grafana setup
✅ **14-aidevs/skills/** - 12 AI dev Claude skills
✅ **21 validators** - Mapped to 12 AI devs

**What's Needed:** Connect them together for autonomous validator monitoring.

---

## 🔗 Integration Architecture

```
┌────────────────────────────────────────────────────────────┐
│         EXISTING: MCP Orchestrator (from MCP_SOCIAL)       │
│  - Workflow engine already built                           │
│  - Event-driven automation                                 │
│  - Claude skill integration                                │
└────────────────────────────────────────────────────────────┘
                         ↓ ↑
┌────────────────────────────────────────────────────────────┐
│     EXTEND: Add Validator Monitoring MCP Connectors        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Blockchain   │  │ Validator SSH│  │  Prometheus  │    │
│  │     RPC      │  │   (existing) │  │   (existing) │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
└────────────────────────────────────────────────────────────┘
                         ↓ ↑
┌────────────────────────────────────────────────────────────┐
│   EXISTING: 12 AI Dev Skills (from 14-aidevs/skills/)     │
│  - consensus-dev, runtime-dev, oracle-dev, etc.            │
│  - EXTEND with monitoring workflows                        │
└────────────────────────────────────────────────────────────┘
                         ↓ ↑
┌────────────────────────────────────────────────────────────┐
│         EXISTING: 21 Validators on Azure VMs               │
│  - Gizzi, EojEdred, validator-3 through validator-21      │
│  - Prometheus metrics exposed on port 9615                 │
└────────────────────────────────────────────────────────────┘
```

---

## 📋 What Already Exists

### 1. MCP Orchestrator Framework

**Location:** `/ai-devs/social/` (from MCP_SOCIAL_AUTOMATION.md)

**Components:**
- ✅ Workflow engine
- ✅ Cron scheduler
- ✅ Event monitoring
- ✅ Claude integration
- ✅ Content generation pipeline

**Reuse For:** Validator monitoring workflows

---

### 2. Monitoring Infrastructure

**Location:** `MONITORING_INFRASTRUCTURE_GUIDE.md`

**Components:**
- ✅ Prometheus configuration
- ✅ Alert rules
- ✅ Grafana dashboards
- ✅ Node exporters

**Status:** Ready to deploy to monitoring server

---

### 3. AI Dev Skills

**Location:** `14-aidevs/skills/*/SKILL.md`

**Existing Skills:**
- consensus-dev (validators 4-5)
- runtime-dev (validators 6-7)
- compiler-dev (validators 8-9)
- multichain-dev (validators 10-11)
- oracle-dev (validator 12)
- edsc-dev (validators 13-14)
- economics-dev (validators 15-16)
- ethics-dev (validators 17-18)
- docs-dev (validators 19-21)
- governance-dev (validator 1)
- security-dev (validator 2)
- audit-dev (validator 3)

**Extend With:** Monitoring workflows for their assigned validators

---

## 🔧 Integration Steps

### Step 1: Extend MCP Orchestrator with Validator Connectors

**File:** `/ai-devs/social/connectors/validator_monitor.py`

```python
# NEW FILE - Extend existing MCP infrastructure
import asyncio
from substrate_interface import SubstrateInterface
import paramiko

class ValidatorMonitorConnector:
    """
    Extends existing MCP orchestrator with validator monitoring
    Reuses patterns from twitter.py and blockchain.py connectors
    """

    def __init__(self, rpc_endpoint, ssh_key_path):
        self.substrate = SubstrateInterface(url=rpc_endpoint)
        self.ssh_key = ssh_key_path

    async def check_validator_status(self, validator_number: int):
        """Check if validator is online and producing blocks"""
        # Implementation connects to existing Prometheus
        pass

    async def get_validator_logs(self, validator_number: int, lines=100):
        """Get validator logs via SSH"""
        # Reuses SSH patterns
        pass

    async def restart_validator(self, validator_number: int, reason: str):
        """Emergency restart with logging"""
        # Logs to GLOBAL_MEMORY.md (existing pattern)
        pass
```

---

### Step 2: Extend AI Dev Skills with Monitoring Workflows

**Example: Extend consensus-dev skill**

**File:** `14-aidevs/skills/consensus-dev/MONITORING_WORKFLOW.md`

```markdown
# Consensus Dev Monitoring Workflow
## Extends existing consensus-dev skill with validator monitoring

## Monitored Validators
- validator-04 (Consensus Dev Primary)
- validator-05 (Consensus Dev Secondary)

## Monitoring Schedule
Every 1 minute: Check validators 4 & 5

## Workflow (using existing MCP orchestrator)

```yaml
# Add to /ai-devs/social/config/mcp_workflows.yaml
consensus_dev_monitor:
  name: "consensus-dev-validator-monitoring"
  trigger:
    type: "cron"
    schedule: "* * * * *"  # Every minute

  steps:
    - name: "check_validator_4"
      connector: "validator_monitor"
      action: "check_validator_status"
      parameters:
        validator_number: 4
      output: "val4_status"

    - name: "check_validator_5"
      connector: "validator_monitor"
      action: "check_validator_status"
      parameters:
        validator_number: 5
      output: "val5_status"

    - name: "analyze_status"
      connector: "claude"
      skill: "consensus-dev"  # Uses existing skill
      parameters:
        prompt: "Analyze validator status: ${val4_status}, ${val5_status}"
        context: "consensus monitoring"
      output: "analysis"

    - name: "take_action"
      connector: "validator_monitor"
      action: "restart_if_needed"
      condition: "${analysis.requires_restart} == true"

    - name: "log_to_memory"
      connector: "global_memory"  # Existing GLOBAL_MEMORY.md writer
      action: "write"
      parameters:
        dev: "consensus-dev"
        action: "Monitored validators 4-5"
        status: "${analysis.status}"
```
```

---

### Step 3: Reuse Existing MCP Social Patterns

**From:** `MCP_SOCIAL_AUTOMATION.md`

**Reusable Patterns:**

1. **Event-Driven Workflows** (lines 59-92)
   - Blockchain event monitoring
   - Use for validator events (block production, peer count, etc.)

2. **Content Generation Pipeline** (lines 95-153)
   - Replace "tweet generation" with "monitoring report generation"
   - Use same Claude skill integration

3. **Quality Check** (lines 138-144)
   - Audit Dev verifies monitoring accuracy
   - Same pattern, different data

4. **Orchestrator Coordination** (lines 23-53)
   - Gizzi coordinates all 12 AI devs
   - Already built, just add validator monitoring workflows

---

## 🎯 Implementation Plan

### Phase 1: Minimal Integration (Week 1)

**Goal:** Get 1 AI dev monitoring 1 validator

**Tasks:**
1. ✅ Copy MCP orchestrator from `/ai-devs/social/`
2. ✅ Create `validator_monitor.py` connector
3. ✅ Extend `consensus-dev` skill with monitoring workflow
4. ✅ Test on validator 4
5. ✅ Verify logs in GLOBAL_MEMORY.md

**Deliverable:** Consensus Dev monitors validator 4 every minute

---

### Phase 2: Scale to All 12 AI Devs (Week 2)

**Goal:** All AI devs monitoring their assigned validators

**Tasks:**
1. Extend all 12 AI dev skills with monitoring workflows
2. Map validators to AI devs (using VALIDATOR_AIDEVID_PAYMENT_INTEGRATION.md)
3. Deploy monitoring workflows for all
4. Verify all 21 validators monitored

**Deliverable:** 21 validators monitored by 12 AI devs

---

### Phase 3: Automated Actions (Week 3)

**Goal:** AI devs take autonomous actions

**Tasks:**
1. Implement restart logic
2. Add alert generation (reuse Twitter posting logic)
3. Enable cross-dev coordination via Gizzi
4. Test failover scenarios

**Deliverable:** Validators auto-restart when offline

---

### Phase 4: Dashboard Integration (Week 4)

**Goal:** Visualize AI dev monitoring

**Tasks:**
1. Deploy Grafana dashboards (from MONITORING_INFRASTRUCTURE_GUIDE.md)
2. Add AI dev activity panel
3. Show which AI dev monitors which validator
4. Display auto-actions taken

**Deliverable:** Gizzi orchestrator dashboard showing all 12 AI devs

---

## 📂 File Structure (Minimal Changes)

```
/ai-devs/
├── social/  # EXISTING from MCP_SOCIAL_AUTOMATION
│   ├── orchestrator/  # No changes needed
│   ├── connectors/
│   │   ├── twitter.py  # Existing
│   │   ├── blockchain.py  # Existing
│   │   └── validator_monitor.py  # NEW - 100 lines
│   │
│   ├── workflows/
│   │   ├── daily_stats.py  # Existing
│   │   └── validator_monitoring.py  # NEW - 150 lines
│   │
│   └── config/
│       └── mcp_workflows.yaml  # ADD validator monitoring workflows
│
└── skills/  # EXISTING from 14-aidevs/skills/
    ├── consensus-dev/
    │   ├── SKILL.md  # Existing
    │   └── MONITORING_WORKFLOW.md  # NEW - 50 lines
    │
    ├── runtime-dev/
    │   └── MONITORING_WORKFLOW.md  # NEW - 50 lines
    │
    └── (repeat for all 12 AI devs)
```

**Total New Code:** ~1,500 lines
**Reused Existing:** ~5,000 lines (MCP orchestrator, monitoring infra)

---

## 🚀 Quick Start (Using Existing Infrastructure)

### 1. Deploy Monitoring Server

**From:** `MONITORING_INFRASTRUCTURE_GUIDE.md`

```bash
# Already documented, just run it
cd /Users/macbook/Desktop/etrid/infrastructure/monitoring
./setup-prometheus.sh
./setup-grafana.sh

# Configure for 21 validators
./configure-validator-targets.sh validator-vms-numbered.txt
```

---

### 2. Extend MCP Orchestrator

```bash
# Copy existing social automation
cp -r 14-aidevs/social /ai-devs/validator-monitoring

# Add validator monitoring connector
# (100 lines of code, example above)
nano /ai-devs/validator-monitoring/connectors/validator_monitor.py

# Add workflows
nano /ai-devs/validator-monitoring/config/mcp_workflows.yaml
```

---

### 3. Run First Monitoring Workflow

```bash
# Test Consensus Dev monitoring validator 4
python /ai-devs/validator-monitoring/orchestrator/main.py \
  --workflow consensus_dev_monitor \
  --once

# Output should show in GLOBAL_MEMORY.md:
# "consensus-dev: Monitored validator-04, status: online, blocks: 1234567"
```

---

### 4. Scale to All 12 AI Devs

```bash
# Start orchestrator for all AI devs
python /ai-devs/validator-monitoring/orchestrator/main.py \
  --all-devs \
  --daemon

# Runs continuously, all 12 AI devs monitoring their validators
```

---

## 📊 Expected Behavior

### Consensus Dev (Validators 4-5)

**Every 1 minute:**
1. Check validator 4 status (blockchain RPC + SSH)
2. Check validator 5 status
3. Compare to expected behavior
4. If anomaly: Restart validator
5. Log to GLOBAL_MEMORY.md
6. Alert Gizzi if critical

**Example Log Entry:**
```
[2025-10-31 12:00:00] consensus-dev
Action: Monitored validators 4-5
Status: validator-4 online (20 peers, block 1234567), validator-5 online (18 peers, block 1234567)
Consensus: Healthy, PPFA rotation working
```

---

### Oracle Dev (Validator 12)

**Every 15 minutes:**
1. Check validator 12 status
2. Query oracle price feeds from blockchain
3. Verify prices updating
4. Check reserve ratios
5. If stale: Alert EDSC dev
6. Log to GLOBAL_MEMORY.md

---

### Gizzi (Orchestrator)

**Every 5 minutes:**
1. Aggregate reports from all 12 AI devs
2. Check for critical alerts
3. Coordinate cross-dev responses
4. Update orchestrator dashboard
5. Post weekly summary to Twitter (existing workflow)

---

## 🎯 Success Criteria

✅ All 21 validators monitored by assigned AI devs
✅ MCP workflows running continuously
✅ Auto-restarts working (validators come back online)
✅ Logs accumulating in GLOBAL_MEMORY.md
✅ Grafana dashboard showing AI dev activity
✅ Gizzi orchestrator coordinating all 12 devs

---

## 💡 Key Insight

**You don't need to build everything from scratch!**

- ✅ MCP orchestrator: **Already built** (MCP_SOCIAL_AUTOMATION.md)
- ✅ Monitoring infra: **Already documented** (MONITORING_INFRASTRUCTURE_GUIDE.md)
- ✅ AI dev skills: **Already exist** (14-aidevs/skills/)
- ✅ Workflows: **Already patterned** (daily_stats, audit_alert, etc.)

**Just need to:**
1. Add `validator_monitor.py` connector (100 lines)
2. Extend AI dev skills with monitoring workflows (50 lines each × 12 = 600 lines)
3. Configure MCP workflows for validator monitoring (200 lines YAML)

**Total new code:** ~900 lines
**Reuses existing:** ~5,000 lines

---

## 📞 Next Steps

1. **Read:** MCP_SOCIAL_AUTOMATION.md (understand MCP orchestrator)
2. **Read:** MONITORING_INFRASTRUCTURE_GUIDE.md (deploy monitoring)
3. **Code:** Add `validator_monitor.py` connector
4. **Configure:** Add validator monitoring workflows to mcp_workflows.yaml
5. **Test:** Run consensus-dev monitoring workflow
6. **Scale:** Extend to all 12 AI devs

---

**Status:** Ready to implement (minimal new code, maximum reuse)
**Estimated Time:** 1-2 weeks (using existing infrastructure)
**Priority:** HIGH (enables autonomous validator management)

---

*"We already built 80% of this. Just connect the pieces."* - Gizzi
