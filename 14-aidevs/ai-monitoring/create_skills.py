#!/usr/bin/env python3
"""
Quickly create SKILL.md files for all 12 AI devs
"""

import os

SKILLS = {
    "governance-dev01": {
        "validators": [1],
        "description": "Gizzi - Director with distributed consciousness. Oversees all validators and coordinates network-wide responses.",
        "capabilities": [
            "Monitor Validator #1 (Gizzi VM) 24/7",
            "Oversee all 11 other AI dev workers",
            "Coordinate network-wide incident response",
            "Access distributed consciousness (Ollama + GPT-4 + Claude)",
            "Make governance decisions via Claude",
            "Emergency escalation to DD board"
        ],
        "ai_tier": "all"
    },
    "security-dev01": {
        "validators": [2, 3],
        "description": "Security monitoring and threat response specialist",
        "capabilities": [
            "Monitor Validators #2-3 for security issues",
            "Detect anomalous behavior patterns",
            "SSH access log analysis",
            "Firewall rule verification",
            "Escalate security incidents"
        ],
        "ai_tier": "claude"
    },
    "audit-dev01": {
        "validators": [2, 3],
        "description": "Transaction validation and audit trail specialist",
        "capabilities": [
            "Monitor Validators #2-3 transaction validity",
            "Verify block production integrity",
            "Audit trail logging",
            "Compliance monitoring",
            "Report generation"
        ],
        "ai_tier": "gpt4"
    },
    "consensus-dev01": {
        "validators": [4, 5],
        "description": "PPFA consensus and validator rotation specialist",
        "capabilities": [
            "Monitor Validators #4-5 PPFA sealing",
            "Validator rotation scheduling",
            "Adaptive slot timing optimization",
            "Finalization monitoring",
            "Consensus health analysis"
        ],
        "ai_tier": "gpt4"
    },
    "runtime-dev01": {
        "validators": [6, 7],
        "description": "Runtime performance and upgrade specialist",
        "capabilities": [
            "Monitor Validators #6-7 runtime performance",
            "Runtime upgrade coordination",
            "Performance benchmarking",
            "Memory usage optimization",
            "Runtime error detection"
        ],
        "ai_tier": "gpt4"
    },
    "compiler-dev01": {
        "validators": [8, 9],
        "description": "Rust/Substrate compiler and WASM specialist",
        "capabilities": [
            "Monitor Validators #8-9 compilation health",
            "WASM execution monitoring",
            "Build error diagnosis",
            "Compiler optimization suggestions",
            "Benchmarking coordination"
        ],
        "ai_tier": "gpt4"
    },
    "multichain-dev01": {
        "validators": [10, 11],
        "description": "Cross-chain state and bridge monitoring specialist",
        "capabilities": [
            "Monitor Validators #10-11 bridge health",
            "Cross-chain state synchronization",
            "Bridge transaction verification",
            "PBC coordination",
            "Multi-chain finalization tracking"
        ],
        "ai_tier": "gpt4"
    },
    "oracle-dev01": {
        "validators": [12],
        "description": "Price feed oracle and EDSC reserve monitoring",
        "capabilities": [
            "Monitor Validator #12 oracle feeds",
            "Price feed accuracy verification",
            "EDSC reserve ratio monitoring",
            "Oracle data source validation",
            "Daily statistics reporting"
        ],
        "ai_tier": "ollama"
    },
    "edsc-dev01": {
        "validators": [13, 14],
        "description": "Economic validity and EDSC operations specialist",
        "capabilities": [
            "Monitor Validators #13-14 EDSC operations",
            "Economic validity verification",
            "Reserve ratio compliance",
            "Minting/burning event monitoring",
            "Economic anomaly detection"
        ],
        "ai_tier": "claude"
    },
    "economics-dev01": {
        "validators": [15, 16],
        "description": "Token economics and Distribution Pay specialist",
        "capabilities": [
            "Monitor Validators #15-16 Distribution Pay",
            "Token distribution verification",
            "Economic model analysis",
            "Inflation/deflation tracking",
            "Distribution Pay schedule execution"
        ],
        "ai_tier": "gpt4"
    },
    "ethics-dev01": {
        "validators": [17, 18],
        "description": "Transaction fairness and governance ethics specialist",
        "capabilities": [
            "Monitor Validators #17-18 transaction fairness",
            "Governance proposal ethics review",
            "Community standards enforcement",
            "Dispute resolution",
            "Ethical AI operation oversight"
        ],
        "ai_tier": "claude"
    },
    "docs-dev01": {
        "validators": [19, 20, 21],
        "description": "Network documentation and API update specialist",
        "capabilities": [
            "Monitor Validators #19-21 API endpoints",
            "Documentation accuracy verification",
            "API version tracking",
            "Community communication",
            "Knowledge base updates"
        ],
        "ai_tier": "ollama"
    }
}

# Create skills directory for each dev
for aidev_id, config in SKILLS.items():
    skill_dir = f"/Users/macbook/Desktop/etrid/ai-monitoring/skills/{aidev_id}"
    os.makedirs(skill_dir, exist_ok=True)

    # Create SKILL.md
    skill_content = f"""---
name: "{aidev_id}"
description: "{config['description']}"
language: "Python"
capabilities:
{chr(10).join('  - ' + cap for cap in config['capabilities'])}
assigned_validators: {config['validators']}
ai_tier: "{config['ai_tier']}"
entrypoint: "ai_dev_workers.py"
tags: ["validator-monitoring", "autonomous", "ai-dev"]
---

# {aidev_id.upper()} - Validator Monitoring Skills

## Overview
{config['description']}

**Assigned Validators:** {', '.join(f'#{v}' for v in config['validators'])}
**AI Tier Access:** {config['ai_tier']}

## Core Capabilities

{chr(10).join(f'{i+1}. **{cap}**' for i, cap in enumerate(config['capabilities']))}

## AI Tier Usage

**{config['ai_tier'].upper()}:**
"""

    if config['ai_tier'] == 'all':
        skill_content += """
- **Ollama Layer:** Instant reflex responses for all validators
- **GPT-4 Layer:** Technical analysis and root cause diagnosis
- **Claude Layer:** Strategic decisions and governance
- **Multi-Model Consensus:** For critical network-wide decisions
"""
    elif config['ai_tier'] == 'claude':
        skill_content += """
- **Claude Sonnet 4:** Superior reasoning for critical decisions
- **Use Case:** Governance, ethics, risk assessment
- **Cost:** ~$0.001 per decision
"""
    elif config['ai_tier'] == 'gpt4':
        skill_content += """
- **GPT-4 Turbo:** Excellent for code analysis and technical issues
- **Use Case:** Debugging, performance analysis, technical diagnosis
- **Cost:** ~$0.002 per query
"""
    else:  # ollama
        skill_content += """
- **Ollama (llama3.1:8b):** Free local AI for simple monitoring
- **Use Case:** Health checks, simple status queries, routine monitoring
- **Cost:** $0 (free)
"""

    skill_content += f"""

## Monitoring Workflow

### Every 5 Minutes
1. Query validator metrics (Prometheus + RPC)
2. Check validator health status
3. Quick analysis via {config['ai_tier'].upper()}
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
Validator #{config['validators'][0]} peer count dropped to 1
→ {config['ai_tier'].upper()} analysis: "Peer discovery issue, restart recommended"
→ Execute: ssh validator-{config['validators'][0]} 'sudo systemctl restart flare-node'
→ Result: Peer count recovered to 8
→ Log to GLOBAL_MEMORY
```

### Escalation
```
Validators {', '.join(f'#{v}' for v in config['validators'])} all experiencing finalization lag
→ {config['ai_tier'].upper()} analysis: "Network-wide issue, escalate to governance-dev01"
→ Flag in GLOBAL_MEMORY with #critical tag
→ governance-dev01 coordinates multi-AI response
```

## Integration

**DID:** `did:etrid:{aidev_id}`
**GLOBAL_MEMORY:** `/opt/ai-monitoring/GLOBAL_MEMORY.md`
**Service:** `ai-dev-monitoring.service` (systemd)

---

*Generated: October 31, 2025*
*Part of: ËTRID AI Monitoring System*
"""

    with open(f"{skill_dir}/SKILL.md", 'w') as f:
        f.write(skill_content)

    print(f"✅ Created {aidev_id}/SKILL.md")

print("\n✅ All SKILL.md files created!")
print(f"Location: /Users/macbook/Desktop/etrid/ai-monitoring/skills/")
