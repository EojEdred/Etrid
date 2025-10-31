---
name: "oracle-dev01"
description: "Price feed oracle and EDSC reserve monitoring"
language: "Python"
capabilities:
  - Monitor Validator #12 oracle feeds
  - Price feed accuracy verification
  - EDSC reserve ratio monitoring
  - Oracle data source validation
  - Daily statistics reporting
assigned_validators: [12]
ai_tier: "ollama"
entrypoint: "ai_dev_workers.py"
tags: ["validator-monitoring", "autonomous", "ai-dev"]
---

# ORACLE-DEV01 - Validator Monitoring Skills

## Overview
Price feed oracle and EDSC reserve monitoring

**Assigned Validators:** #12
**AI Tier Access:** ollama

## Core Capabilities

1. **Monitor Validator #12 oracle feeds**
2. **Price feed accuracy verification**
3. **EDSC reserve ratio monitoring**
4. **Oracle data source validation**
5. **Daily statistics reporting**

## AI Tier Usage

**OLLAMA:**

- **Ollama (llama3.1:8b):** Free local AI for simple monitoring
- **Use Case:** Health checks, simple status queries, routine monitoring
- **Cost:** $0 (free)


## Monitoring Workflow

### Every 5 Minutes
1. Query validator metrics (Prometheus + RPC)
2. Check validator health status
3. Quick analysis via OLLAMA
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
Validator #12 peer count dropped to 1
→ OLLAMA analysis: "Peer discovery issue, restart recommended"
→ Execute: ssh validator-12 'sudo systemctl restart flare-node'
→ Result: Peer count recovered to 8
→ Log to GLOBAL_MEMORY
```

### Escalation
```
Validators #12 all experiencing finalization lag
→ OLLAMA analysis: "Network-wide issue, escalate to governance-dev01"
→ Flag in GLOBAL_MEMORY with #critical tag
→ governance-dev01 coordinates multi-AI response
```

## Integration

**DID:** `did:etrid:oracle-dev01`
**GLOBAL_MEMORY:** `/opt/ai-monitoring/GLOBAL_MEMORY.md`
**Service:** `ai-dev-monitoring.service` (systemd)

---

*Generated: October 31, 2025*
*Part of: ËTRID AI Monitoring System*
