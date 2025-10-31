---
name: "security-dev01"
description: "Security monitoring and threat response specialist"
language: "Python"
capabilities:
  - Monitor Validators #2-3 for security issues
  - Detect anomalous behavior patterns
  - SSH access log analysis
  - Firewall rule verification
  - Escalate security incidents
assigned_validators: [2, 3]
ai_tier: "claude"
entrypoint: "ai_dev_workers.py"
tags: ["validator-monitoring", "autonomous", "ai-dev"]
---

# SECURITY-DEV01 - Validator Monitoring Skills

## Overview
Security monitoring and threat response specialist

**Assigned Validators:** #2, #3
**AI Tier Access:** claude

## Core Capabilities

1. **Monitor Validators #2-3 for security issues**
2. **Detect anomalous behavior patterns**
3. **SSH access log analysis**
4. **Firewall rule verification**
5. **Escalate security incidents**

## AI Tier Usage

**CLAUDE:**

- **Claude Sonnet 4:** Superior reasoning for critical decisions
- **Use Case:** Governance, ethics, risk assessment
- **Cost:** ~$0.001 per decision


## Monitoring Workflow

### Every 5 Minutes
1. Query validator metrics (Prometheus + RPC)
2. Check validator health status
3. Quick analysis via CLAUDE
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
Validator #2 peer count dropped to 1
→ CLAUDE analysis: "Peer discovery issue, restart recommended"
→ Execute: ssh validator-2 'sudo systemctl restart flare-node'
→ Result: Peer count recovered to 8
→ Log to GLOBAL_MEMORY
```

### Escalation
```
Validators #2, #3 all experiencing finalization lag
→ CLAUDE analysis: "Network-wide issue, escalate to governance-dev01"
→ Flag in GLOBAL_MEMORY with #critical tag
→ governance-dev01 coordinates multi-AI response
```

## Integration

**DID:** `did:etrid:security-dev01`
**GLOBAL_MEMORY:** `/opt/ai-monitoring/GLOBAL_MEMORY.md`
**Service:** `ai-dev-monitoring.service` (systemd)

---

*Generated: October 31, 2025*
*Part of: ËTRID AI Monitoring System*
