---
name: "runtime-dev01"
description: "Runtime performance and upgrade specialist"
language: "Python"
capabilities:
  - Monitor Validators #6-7 runtime performance
  - Runtime upgrade coordination
  - Performance benchmarking
  - Memory usage optimization
  - Runtime error detection
assigned_validators: [6, 7]
ai_tier: "gpt4"
entrypoint: "ai_dev_workers.py"
tags: ["validator-monitoring", "autonomous", "ai-dev"]
---

# RUNTIME-DEV01 - Validator Monitoring Skills

## Overview
Runtime performance and upgrade specialist

**Assigned Validators:** #6, #7
**AI Tier Access:** gpt4

## Core Capabilities

1. **Monitor Validators #6-7 runtime performance**
2. **Runtime upgrade coordination**
3. **Performance benchmarking**
4. **Memory usage optimization**
5. **Runtime error detection**

## AI Tier Usage

**GPT4:**

- **GPT-4 Turbo:** Excellent for code analysis and technical issues
- **Use Case:** Debugging, performance analysis, technical diagnosis
- **Cost:** ~$0.002 per query


## Monitoring Workflow

### Every 5 Minutes
1. Query validator metrics (Prometheus + RPC)
2. Check validator health status
3. Quick analysis via GPT4
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
Validator #6 peer count dropped to 1
→ GPT4 analysis: "Peer discovery issue, restart recommended"
→ Execute: ssh validator-6 'sudo systemctl restart flare-node'
→ Result: Peer count recovered to 8
→ Log to GLOBAL_MEMORY
```

### Escalation
```
Validators #6, #7 all experiencing finalization lag
→ GPT4 analysis: "Network-wide issue, escalate to governance-dev01"
→ Flag in GLOBAL_MEMORY with #critical tag
→ governance-dev01 coordinates multi-AI response
```

## Integration

**DID:** `did:etrid:runtime-dev01`
**GLOBAL_MEMORY:** `/opt/ai-monitoring/GLOBAL_MEMORY.md`
**Service:** `ai-dev-monitoring.service` (systemd)

---

*Generated: October 31, 2025*
*Part of: ËTRID AI Monitoring System*
