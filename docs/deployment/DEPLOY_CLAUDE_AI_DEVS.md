# Deploy Claude AI Devs - Multiple Instances
## Run 12 AI Devs as Separate Claude API Instances

**Purpose:** Deploy autonomous AI dev monitoring using Claude API (headless)

---

## Architecture

```
┌────────────────────────────────────────────────────┐
│        Monitoring Server (or dedicated VM)         │
│                                                     │
│  ┌──────────────────────────────────────────────┐ │
│  │  Orchestrator (Python)                       │ │
│  │  - Schedules monitoring tasks                │ │
│  │  - Routes to AI devs                         │ │
│  └──────────────────────────────────────────────┘ │
│              ↓                                     │
│  ┌──────────────────────────────────────────────┐ │
│  │  12 Claude API Workers (Python threads)     │ │
│  │                                               │ │
│  │  [consensus-dev] → Monitors val 4-5          │ │
│  │  [runtime-dev]   → Monitors val 6-7          │ │
│  │  [compiler-dev]  → Monitors val 8-9          │ │
│  │  [multichain-dev] → Monitors val 10-11       │ │
│  │  ... (12 total AI devs)                      │ │
│  └──────────────────────────────────────────────┘ │
│              ↓                                     │
│  Uses validator_monitor.py to check validators    │
└────────────────────────────────────────────────────┘
              ↓
    21 Validators (SSH + RPC)
```

---

## Implementation

### 1. Install Claude API SDK

```bash
# On monitoring server
pip3 install anthropic
```

---

### 2. Create AI Dev Workers

```python
# File: /ai-monitoring/ai_dev_workers.py

import anthropic
import json
import time
from validator_monitor import ValidatorMonitor

class AIDevWorker:
    """
    Single AI dev instance - monitors assigned validators
    """

    def __init__(self, aidev_id: str, api_key: str, validator_monitor: ValidatorMonitor):
        self.aidev_id = aidev_id
        self.client = anthropic.Anthropic(api_key=api_key)
        self.monitor = validator_monitor
        self.validators = validator_monitor.get_validators_by_aidevid(aidev_id)

    def check_validators(self) -> dict:
        """Check all validators assigned to this AI dev"""
        results = []
        for v in self.validators:
            status = self.monitor.check_validator_status(v['number'])
            results.append(status)
        return results

    def analyze_and_act(self, validator_statuses: list) -> dict:
        """
        Use Claude to analyze validator status and decide actions
        """
        # Build prompt
        prompt = f"""You are {self.aidev_id}, monitoring these validators:

{json.dumps(validator_statuses, indent=2)}

Analyze the status and determine:
1. Are all validators healthy?
2. Do any need to be restarted?
3. What actions should be taken?

Respond in JSON format:
{{
  "summary": "brief status summary",
  "healthy": true/false,
  "actions_needed": [
    {{"validator": 4, "action": "restart", "reason": "offline"}}
  ],
  "alerts": []
}}
"""

        # Call Claude API
        message = self.client.messages.create(
            model="claude-sonnet-4",
            max_tokens=1024,
            messages=[
                {"role": "user", "content": prompt}
            ]
        )

        # Parse response
        response_text = message.content[0].text
        analysis = json.loads(response_text)

        return analysis

    def execute_actions(self, actions: list):
        """Execute actions determined by Claude"""
        for action in actions:
            if action['action'] == 'restart':
                validator_num = action['validator']
                reason = action['reason']
                print(f"[{self.aidev_id}] Restarting validator {validator_num}: {reason}")
                self.monitor.restart_validator(validator_num, reason)

    def monitoring_cycle(self):
        """Run one monitoring cycle"""
        print(f"\n[{self.aidev_id}] Starting monitoring cycle...")

        # 1. Check validator status
        statuses = self.check_validators()

        # 2. Use Claude to analyze
        analysis = self.analyze_and_act(statuses)

        # 3. Execute actions if needed
        if analysis.get('actions_needed'):
            self.execute_actions(analysis['actions_needed'])

        # 4. Log to GLOBAL_MEMORY
        log_entry = {
            "timestamp": time.time(),
            "aidev": self.aidev_id,
            "summary": analysis['summary'],
            "validators": statuses
        }

        with open('/Users/macbook/Desktop/etrid/14-aidevs/memory/GLOBAL_MEMORY.md', 'a') as f:
            f.write(f"\n## [{self.aidev_id}] {analysis['summary']}\n")

        print(f"[{self.aidev_id}] {analysis['summary']}")

        return analysis


# Main orchestrator - runs all 12 AI devs
class AIDevOrchestrator:
    """
    Coordinates all 12 AI dev workers
    """

    def __init__(self, api_key: str, validator_monitor: ValidatorMonitor):
        self.api_key = api_key
        self.monitor = validator_monitor

        # Create 12 AI dev workers
        self.workers = {
            'governance-dev01': AIDevWorker('governance-dev01', api_key, validator_monitor),
            'security-dev01': AIDevWorker('security-dev01', api_key, validator_monitor),
            'audit-dev01': AIDevWorker('audit-dev01', api_key, validator_monitor),
            'consensus-dev01': AIDevWorker('consensus-dev01', api_key, validator_monitor),
            'runtime-dev01': AIDevWorker('runtime-dev01', api_key, validator_monitor),
            'compiler-dev01': AIDevWorker('compiler-dev01', api_key, validator_monitor),
            'multichain-dev01': AIDevWorker('multichain-dev01', api_key, validator_monitor),
            'oracle-dev01': AIDevWorker('oracle-dev01', api_key, validator_monitor),
            'edsc-dev01': AIDevWorker('edsc-dev01', api_key, validator_monitor),
            'economics-dev01': AIDevWorker('economics-dev01', api_key, validator_monitor),
            'ethics-dev01': AIDevWorker('ethics-dev01', api_key, validator_monitor),
            'docs-dev01': AIDevWorker('docs-dev01', api_key, validator_monitor),
        }

    def run_monitoring_cycle(self):
        """Run monitoring for all 12 AI devs"""
        print("\n" + "="*60)
        print("AI DEV ORCHESTRATOR - Monitoring Cycle")
        print("="*60)

        results = {}
        for aidev_id, worker in self.workers.items():
            results[aidev_id] = worker.monitoring_cycle()

        print("\n" + "="*60)
        print("Monitoring cycle complete")
        print("="*60)

        return results

    def run_forever(self, interval_seconds=60):
        """Run continuous monitoring"""
        print(f"Starting continuous monitoring (every {interval_seconds}s)...")

        while True:
            try:
                self.run_monitoring_cycle()
                time.sleep(interval_seconds)
            except KeyboardInterrupt:
                print("\nStopping monitoring...")
                break
            except Exception as e:
                print(f"Error: {e}")
                time.sleep(interval_seconds)


# Usage
if __name__ == '__main__':
    import os

    # Get API key from environment
    API_KEY = os.getenv('ANTHROPIC_API_KEY')
    if not API_KEY:
        print("Error: Set ANTHROPIC_API_KEY environment variable")
        exit(1)

    # Initialize validator monitor
    monitor = ValidatorMonitor(
        validator_ips_path='/Users/macbook/Desktop/etrid/validator-ips.json',
        ssh_key_path='/Users/macbook/.ssh/gizzi-validator',
        prometheus_url='http://64.181.215.19:9090'
    )

    # Create orchestrator
    orchestrator = AIDevOrchestrator(API_KEY, monitor)

    # Run forever (every 60 seconds)
    orchestrator.run_forever(interval_seconds=60)
```

---

### 3. Deploy to Server

```bash
# Copy files to monitoring server
scp validator-ips.json ubuntu@MONITORING_SERVER:/opt/ai-monitoring/
scp validator_monitor.py ubuntu@MONITORING_SERVER:/opt/ai-monitoring/
scp ai_dev_workers.py ubuntu@MONITORING_SERVER:/opt/ai-monitoring/
scp ~/.ssh/gizzi-validator ubuntu@MONITORING_SERVER:~/.ssh/

# SSH to monitoring server
ssh ubuntu@MONITORING_SERVER

# Install dependencies
pip3 install anthropic requests

# Set API key
export ANTHROPIC_API_KEY="your_anthropic_api_key_here"

# Run AI dev orchestrator
cd /opt/ai-monitoring
python3 ai_dev_workers.py

# Output:
# ============================================================
# AI DEV ORCHESTRATOR - Monitoring Cycle
# ============================================================
#
# [consensus-dev01] Starting monitoring cycle...
# [consensus-dev01] All validators healthy (4, 5)
#
# [runtime-dev01] Starting monitoring cycle...
# [runtime-dev01] All validators healthy (6, 7)
#
# ... (all 12 AI devs)
```

---

### 4. Run as Systemd Service (24/7)

```bash
# Create systemd service
sudo tee /etc/systemd/system/ai-dev-monitoring.service > /dev/null << 'EOF'
[Unit]
Description=AI Dev Blockchain Monitoring
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/ai-monitoring
Environment="ANTHROPIC_API_KEY=your_api_key_here"
ExecStart=/usr/bin/python3 /opt/ai-monitoring/ai_dev_workers.py
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Start service
sudo systemctl daemon-reload
sudo systemctl start ai-dev-monitoring
sudo systemctl enable ai-dev-monitoring

# Check logs
sudo journalctl -u ai-dev-monitoring -f
```

---

## Cost Estimate

### Claude API Usage

**Per monitoring cycle:**
- 12 AI devs × 1 API call = 12 calls
- ~500 tokens per call
- Total: ~6,000 tokens per cycle

**Running every minute:**
- 60 cycles/hour × 6,000 tokens = 360,000 tokens/hour
- 24 hours × 360,000 = 8.64M tokens/day

**Cost (Claude Sonnet 4):**
- Input: $3 per 1M tokens
- Output: $15 per 1M tokens
- Average: ~$50-100/day ($1,500-3,000/month)

### Optimizations to Reduce Cost:

1. **Run less frequently:** Every 5 minutes instead of 1 minute
   - Cost: $300-600/month

2. **Use Claude Haiku for simple checks:**
   - $0.25 per 1M tokens
   - Cost: $10-20/month

3. **Only call Claude when anomaly detected:**
   - Simple Python checks first
   - Claude only for complex analysis
   - Cost: $50-100/month

---

## Recommended Setup (Cost-Effective)

```python
# Modified monitoring cycle
def monitoring_cycle_optimized(self):
    """Cost-optimized monitoring"""

    # 1. Simple check (no Claude API)
    statuses = self.check_validators()

    # 2. Only use Claude if anomaly detected
    if any(not s['healthy'] for s in statuses):
        # Use Claude to analyze and decide action
        analysis = self.analyze_and_act(statuses)
        self.execute_actions(analysis['actions_needed'])
    else:
        # No Claude needed, just log
        print(f"[{self.aidev_id}] All validators healthy")

    return statuses
```

**Cost with this approach:** $20-50/month

---

## Alternative: Use Claude Desktop (No API Cost)

If you want to avoid API costs entirely, run on your Mac:

```bash
# On your Mac
cd /Users/macbook/Desktop/etrid/ai-monitoring

# Run locally (uses Claude Desktop, no API cost)
python3 ai_dev_workers.py
```

**Pros:** No API cost
**Cons:** Your Mac must be on 24/7

---

## Which Option Should You Use?

| Option | Cost | Reliability | Complexity |
|--------|------|-------------|------------|
| **Claude Desktop (your Mac)** | $0 | Requires Mac on | Easy |
| **Claude API (Haiku, 5min)** | ~$20/month | 24/7 | Medium |
| **Claude API (Sonnet, 1min)** | ~$300/month | 24/7, best analysis | Medium |

**Recommendation for testing:** Start with Claude Desktop on your Mac
**Recommendation for production:** Claude API (Haiku, 5 minute interval)

---

## Quick Start

**Option A: Use Your Mac (No API Cost)**

```bash
cd /Users/macbook/Desktop/etrid/ai-monitoring

# Install dependencies
pip3 install anthropic requests

# Set API key (for testing)
export ANTHROPIC_API_KEY="your_key"

# Run monitoring
python3 ai_dev_workers.py
```

**Option B: Deploy to Server (Production)**

1. Follow "Deploy to Server" section above
2. Use systemd service for 24/7 operation
3. Monitor with `journalctl -u ai-dev-monitoring -f`

---

**Ready to implement?** Let me know which option you prefer and we'll set it up!
