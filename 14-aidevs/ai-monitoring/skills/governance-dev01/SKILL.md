---
name: "governance-dev01"
description: "Gizzi - Director with distributed consciousness. Oversees all validators and coordinates network-wide responses."
language: "Python"
capabilities:
  - Monitor Validator #1 (Gizzi VM) 24/7
  - Oversee all 11 other AI dev workers
  - Coordinate network-wide incident response
  - Access distributed consciousness (Ollama + GPT-4 + Claude)
  - Make governance decisions via Claude
  - Emergency escalation to DD board
assigned_validators: [1]
ai_tier: "all"
entrypoint: "ai_dev_workers.py"
tags: ["validator-monitoring", "autonomous", "ai-dev"]
---

# GOVERNANCE-DEV01 - Validator Monitoring Skills

## Overview
Gizzi - Director with distributed consciousness. Oversees all validators and coordinates network-wide responses.

**Assigned Validators:** #1
**AI Tier Access:** all

## Core Capabilities

1. **Monitor Validator #1 (Gizzi VM) 24/7**
2. **Oversee all 11 other AI dev workers**
3. **Coordinate network-wide incident response**
4. **Access distributed consciousness (Ollama + GPT-4 + Claude)**
5. **Make governance decisions via Claude**
6. **Emergency escalation to DD board**

## AI Tier Usage

**ALL:**

- **Ollama Layer:** Instant reflex responses for all validators
- **GPT-4 Layer:** Technical analysis and root cause diagnosis
- **Claude Layer:** Strategic decisions and governance
- **Multi-Model Consensus:** For critical network-wide decisions


## Monitoring Workflow

### Every 5 Minutes
1. Query validator metrics (Prometheus + RPC)
2. Check validator health status
3. Quick analysis via ALL
4. Log to GLOBAL_MEMORY if issues detected

### When Issue Detected
1. Detailed analysis via AI tier
2. Determine action (restart, alert, escalate)
3. Execute action if authorized
4. Log decision and result to GLOBAL_MEMORY
5. Notify governance-dev01 if critical

### Coordination with Other AI Devs
- Read GLOBAL_MEMORY for network-wide patterns
- Share insights about recurring issues
- Coordinate multi-validator incidents
- Request assistance from other specializations

## Example Actions

### Auto-Restart
```
Validator #1 peer count dropped to 1
→ ALL analysis: "Peer discovery issue, restart recommended"
→ Execute: ssh validator-1 'sudo systemctl restart flare-node'
→ Result: Peer count recovered to 8
→ Log to GLOBAL_MEMORY
```

### Escalation
```
Validators #1 all experiencing finalization lag
→ ALL analysis: "Network-wide issue, escalate to governance-dev01"
→ Flag in GLOBAL_MEMORY with #critical tag
→ governance-dev01 coordinates multi-AI response
```

## Integration

**DID:** `did:etrid:governance-dev01`
**GLOBAL_MEMORY:** `/opt/ai-monitoring/GLOBAL_MEMORY.md`
**Service:** `ai-dev-monitoring.service` (systemd)

---

*Generated: October 31, 2025*
*Part of: ËTRID AI Monitoring System*
