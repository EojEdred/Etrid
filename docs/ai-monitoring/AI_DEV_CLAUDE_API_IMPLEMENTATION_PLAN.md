# AI Dev Claude API Implementation Plan
## 12 Autonomous AI Developers Monitoring 21 Validators

**Version:** 2.0
**Date:** 2025-10-31
**Focus:** Claude API architecture for validator monitoring

---

## 🎯 Overview

**Goal:** Deploy 12 autonomous AI dev instances using Claude API to monitor and manage 21 blockchain validators.

**Architecture:** Centralized Python orchestrator running 12 Claude API workers, each monitoring their assigned validators.

---

## 📊 AI Dev to Validator Mapping

| AI Dev ID | Validators | Count | Role |
|-----------|------------|-------|------|
| governance-dev01 | 1 | 1 | Director + Bootnode |
| security-dev01 | 2 | 1 | Director + Bootnode |
| audit-dev01 | 3 | 1 | Director + Bootnode |
| consensus-dev01 | 4-5 | 2 | FlareNodes |
| runtime-dev01 | 6-7 | 2 | FlareNodes |
| compiler-dev01 | 8-9 | 2 | FlareNodes |
| multichain-dev01 | 10-11 | 2 | FlareNodes |
| oracle-dev01 | 12 | 1 | FlareNode |
| edsc-dev01 | 13-14 | 2 | ValidityNodes |
| economics-dev01 | 15-16 | 2 | ValidityNodes |
| ethics-dev01 | 17-18 | 2 | ValidityNodes |
| docs-dev01 | 19-21 | 3 | ValidityNodes |

**Total:** 21 validators monitored by 12 AI devs

---

## 🏗️ Architecture

### Layer 1: Monitoring Data Collection

```
┌─────────────────────────────────────────────────────────────┐
│                   21 Validators (Azure + Oracle)            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │ Val 1    │  │ Val 2    │  │ Val 3    │  ... Val 21     │
│  │ :9615    │  │ :9615    │  │ :9615    │                 │
│  │Prometheus│  │Prometheus│  │Prometheus│                 │
│  └──────────┘  └──────────┘  └──────────┘                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │ RPC 9944 │  │ RPC 9944 │  │ RPC 9944 │  ... RPC        │
│  └──────────┘  └──────────┘  └──────────┘                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │ SSH 22   │  │ SSH 22   │  │ SSH 22   │  ... SSH        │
│  └──────────┘  └──────────┘  └──────────┘                 │
└─────────────────────────────────────────────────────────────┘
                         ↓ ↓ ↓
┌─────────────────────────────────────────────────────────────┐
│              Monitoring Server (Validator 1 or dedicated)   │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ Prometheus (Central)                                   │ │
│  │ - Scrapes all 21 validators every 15s                  │ │
│  │ - URL: http://localhost:9090                           │ │
│  └────────────────────────────────────────────────────────┘ │
│                         ↓                                    │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ ValidatorMonitor Python Class                          │ │
│  │ - Queries Prometheus for metrics                       │ │
│  │ - Executes SSH commands for logs/actions               │ │
│  │ - Calls RPC for blockchain state                       │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Layer 2: AI Dev Workers (Claude API)

```
┌─────────────────────────────────────────────────────────────┐
│         AI Dev Orchestrator (Python)                        │
│  Single process running on monitoring server                │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ Main Loop (every 60 seconds)                           │ │
│  │                                                         │ │
│  │  For each AI dev (12 total):                           │ │
│  │    1. Collect validator status (Prometheus + RPC)      │ │
│  │    2. Call Claude API with status data                 │ │
│  │    3. Parse Claude's response (JSON)                   │ │
│  │    4. Execute actions (restart, alert, etc.)           │ │
│  │    5. Log to GLOBAL_MEMORY.md                          │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐             │
│  │ AI Worker │  │ AI Worker │  │ AI Worker │  ... (12x)   │
│  │governance │  │ security  │  │  audit    │             │
│  └───────────┘  └───────────┘  └───────────┘             │
│       ↓              ↓              ↓                        │
│  ┌────────────────────────────────────────────────────────┐ │
│  │          Claude API (anthropic.Anthropic)              │ │
│  │          Shared API Key - 1 subscription               │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## 💻 Implementation: Core Components

### Component 1: ValidatorMonitor Class

**File:** `/ai-monitoring/validator_monitor.py`

```python
import paramiko
import requests
from typing import Dict, List
import json

class ValidatorMonitor:
    """
    Collects validator status from multiple sources:
    - Prometheus metrics
    - Blockchain RPC
    - SSH logs
    """

    def __init__(self,
                 validator_ips_path: str,
                 ssh_key_path: str,
                 prometheus_url: str):
        """
        Args:
            validator_ips_path: Path to validator-ips.json
            ssh_key_path: Path to SSH private key (~/.ssh/gizzi-validator)
            prometheus_url: Prometheus server URL (http://localhost:9090)
        """
        self.ssh_key_path = ssh_key_path
        self.prometheus_url = prometheus_url

        # Load validator IPs
        with open(validator_ips_path, 'r') as f:
            data = json.load(f)
            self.validators = {v['number']: v for v in data['validators']}

    def get_validators_by_aidevid(self, aidev_id: str) -> List[Dict]:
        """Get all validators assigned to an AI dev"""
        return [v for v in self.validators.values()
                if v['aiDevId'] == aidev_id]

    def check_validator_status(self, validator_num: int) -> Dict:
        """
        Check complete validator status

        Returns:
            {
                "validator_num": 1,
                "name": "Gizzi",
                "ip": "64.181.215.19",
                "metrics": {
                    "block_height": 123456,
                    "peers": 20,
                    "is_syncing": False,
                    "finalized_height": 123450
                },
                "rpc_status": {
                    "online": True,
                    "chain": "flarechain",
                    "best_block": "0x123..."
                },
                "process_status": {
                    "running": True,
                    "uptime_seconds": 86400
                },
                "healthy": True,
                "issues": []
            }
        """
        validator = self.validators[validator_num]

        # 1. Get Prometheus metrics
        metrics = self._query_prometheus_metrics(validator['ip'])

        # 2. Get RPC status
        rpc_status = self._query_rpc(validator['ip'])

        # 3. Get process status via SSH
        process_status = self._check_process_ssh(validator_num)

        # 4. Determine health
        healthy, issues = self._analyze_health(metrics, rpc_status, process_status)

        return {
            "validator_num": validator_num,
            "name": validator['name'],
            "ip": validator['ip'],
            "metrics": metrics,
            "rpc_status": rpc_status,
            "process_status": process_status,
            "healthy": healthy,
            "issues": issues
        }

    def _query_prometheus_metrics(self, ip: str) -> Dict:
        """Query Prometheus for validator metrics"""
        queries = {
            "block_height": f'substrate_block_height{{instance="{ip}:9615"}}',
            "peers": f'substrate_sub_libp2p_peers_count{{instance="{ip}:9615"}}',
            "finalized_height": f'substrate_block_height{{status="finalized",instance="{ip}:9615"}}'
        }

        metrics = {}
        for metric_name, query in queries.items():
            try:
                response = requests.get(
                    f"{self.prometheus_url}/api/v1/query",
                    params={"query": query},
                    timeout=5
                )
                data = response.json()
                if data['data']['result']:
                    metrics[metric_name] = float(data['data']['result'][0]['value'][1])
                else:
                    metrics[metric_name] = None
            except Exception as e:
                metrics[metric_name] = None

        return metrics

    def _query_rpc(self, ip: str) -> Dict:
        """Query validator RPC endpoint"""
        try:
            response = requests.post(
                f"http://{ip}:9944",
                json={
                    "jsonrpc": "2.0",
                    "method": "system_health",
                    "params": [],
                    "id": 1
                },
                timeout=5
            )
            return response.json().get('result', {})
        except:
            return {"online": False}

    def _check_process_ssh(self, validator_num: int) -> Dict:
        """Check validator process via SSH"""
        validator = self.validators[validator_num]

        try:
            ssh = paramiko.SSHClient()
            ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())

            # Determine username from validator config
            username = validator.get('sshUser', 'ubuntu')

            ssh.connect(
                validator['ip'],
                username=username,
                key_filename=self.ssh_key_path,
                timeout=5
            )

            # Check if flarechain-node is running
            stdin, stdout, stderr = ssh.exec_command(
                "systemctl is-active flarechain-validator || pgrep -f flarechain-node"
            )
            output = stdout.read().decode().strip()

            running = output in ['active', 'running'] or output.isdigit()

            ssh.close()

            return {
                "running": running,
                "checked_via": "ssh"
            }
        except Exception as e:
            return {
                "running": None,
                "error": str(e)
            }

    def _analyze_health(self, metrics, rpc, process) -> tuple:
        """
        Analyze validator health from collected data

        Returns:
            (healthy: bool, issues: List[str])
        """
        issues = []

        # Check process running
        if not process.get('running'):
            issues.append("Process not running")

        # Check RPC connectivity
        if not rpc.get('online'):
            issues.append("RPC not responding")

        # Check peer count
        peers = metrics.get('peers')
        if peers is not None and peers < 3:
            issues.append(f"Low peer count: {peers}")

        # Check block height
        block_height = metrics.get('block_height')
        finalized = metrics.get('finalized_height')
        if block_height and finalized:
            if block_height - finalized > 100:
                issues.append(f"Finalization lag: {block_height - finalized} blocks")

        healthy = len(issues) == 0

        return healthy, issues

    def restart_validator(self, validator_num: int, reason: str):
        """Restart validator via SSH"""
        validator = self.validators[validator_num]
        username = validator.get('sshUser', 'ubuntu')

        try:
            ssh = paramiko.SSHClient()
            ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            ssh.connect(
                validator['ip'],
                username=username,
                key_filename=self.ssh_key_path,
                timeout=5
            )

            # Restart via systemd
            stdin, stdout, stderr = ssh.exec_command(
                "sudo systemctl restart flarechain-validator"
            )

            ssh.close()

            print(f"✅ Restarted validator {validator_num}: {reason}")
        except Exception as e:
            print(f"❌ Failed to restart validator {validator_num}: {e}")
```

---

### Component 2: AIDevWorker Class (Claude API)

**File:** `/ai-monitoring/ai_dev_workers.py`

```python
import anthropic
import json
import time
from datetime import datetime
from validator_monitor import ValidatorMonitor

class AIDevWorker:
    """
    Single AI dev instance using Claude API
    Monitors assigned validators and takes autonomous actions
    """

    def __init__(self,
                 aidev_id: str,
                 api_key: str,
                 validator_monitor: ValidatorMonitor,
                 memory_path: str = '/Users/macbook/Desktop/etrid/14-aidevs/memory/GLOBAL_MEMORY.md'):
        """
        Args:
            aidev_id: e.g. "consensus-dev01"
            api_key: Anthropic API key (shared by all 12 workers)
            validator_monitor: ValidatorMonitor instance
            memory_path: Path to GLOBAL_MEMORY.md
        """
        self.aidev_id = aidev_id
        self.client = anthropic.Anthropic(api_key=api_key)
        self.monitor = validator_monitor
        self.memory_path = memory_path

        # Get assigned validators
        self.validators = validator_monitor.get_validators_by_aidevid(aidev_id)

        print(f"[{self.aidev_id}] Initialized with {len(self.validators)} validators")

    def check_validators(self) -> List[Dict]:
        """Check status of all assigned validators"""
        statuses = []
        for validator in self.validators:
            status = self.monitor.check_validator_status(validator['number'])
            statuses.append(status)
        return statuses

    def analyze_with_claude(self, validator_statuses: List[Dict]) -> Dict:
        """
        Use Claude API to analyze validator status and decide actions

        Returns:
            {
                "summary": "All validators healthy",
                "healthy": True,
                "actions_needed": [],
                "alerts": [],
                "reasoning": "..."
            }
        """

        # Build system prompt (AI dev personality and role)
        system_prompt = f"""You are {self.aidev_id}, an autonomous AI developer responsible for monitoring and maintaining blockchain validators for the Ëtrid network.

Your assigned validators: {', '.join([f"validator-{v['number']}" for v in self.validators])}

Your responsibilities:
1. Monitor validator health (block production, peer count, finalization)
2. Detect issues early (offline, low peers, finalization lag)
3. Take action when needed (restart, alert, investigate)
4. Document decisions in GLOBAL_MEMORY

You have access to:
- Prometheus metrics (block height, peers, finalization)
- RPC status (online/offline, chain state)
- Process status (running/stopped)

Decision criteria:
- If validator offline OR peers < 3 OR finalization lag > 100 blocks → restart
- If multiple validators affected → escalate to Gizzi (governance-dev01)
- If issue persists after restart → create detailed investigation log

Respond in JSON format only."""

        # Build user prompt with current validator status
        user_prompt = f"""Current validator status:

```json
{json.dumps(validator_statuses, indent=2)}
```

Analyze this status and provide:
1. Overall health assessment
2. Any actions that should be taken
3. Alerts to raise
4. Brief reasoning

Respond in this exact JSON format:
{{
  "summary": "Brief 1-sentence summary",
  "healthy": true/false,
  "actions_needed": [
    {{"validator": 4, "action": "restart", "reason": "offline"}}
  ],
  "alerts": [
    {{"severity": "warning", "message": "..."}}
  ],
  "reasoning": "Detailed explanation of your assessment"
}}"""

        try:
            # Call Claude API
            message = self.client.messages.create(
                model="claude-sonnet-4-20250514",  # Latest Sonnet
                max_tokens=2048,
                system=system_prompt,
                messages=[
                    {"role": "user", "content": user_prompt}
                ]
            )

            # Parse response
            response_text = message.content[0].text

            # Extract JSON (Claude might wrap it in markdown)
            if "```json" in response_text:
                json_start = response_text.find("```json") + 7
                json_end = response_text.find("```", json_start)
                response_text = response_text[json_start:json_end].strip()

            analysis = json.loads(response_text)

            return analysis

        except Exception as e:
            print(f"[{self.aidev_id}] Error calling Claude API: {e}")
            return {
                "summary": f"Error analyzing status: {e}",
                "healthy": False,
                "actions_needed": [],
                "alerts": [{"severity": "error", "message": str(e)}],
                "reasoning": "Failed to analyze with Claude"
            }

    def execute_actions(self, actions: List[Dict]):
        """Execute actions determined by Claude"""
        for action in actions:
            action_type = action.get('action')
            validator_num = action.get('validator')
            reason = action.get('reason', 'no reason given')

            if action_type == 'restart':
                print(f"[{self.aidev_id}] Restarting validator {validator_num}: {reason}")
                self.monitor.restart_validator(validator_num, reason)

            elif action_type == 'alert':
                print(f"[{self.aidev_id}] ALERT for validator {validator_num}: {reason}")
                # Could integrate with Discord, email, etc.

            elif action_type == 'investigate':
                print(f"[{self.aidev_id}] Investigating validator {validator_num}: {reason}")
                # Could trigger deeper diagnostics

    def log_to_memory(self, analysis: Dict, validator_statuses: List[Dict]):
        """Log monitoring cycle to GLOBAL_MEMORY.md"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

        log_entry = f"""
## [{timestamp}] {self.aidev_id}

**Summary:** {analysis['summary']}

**Validators:** {', '.join([f"#{s['validator_num']}" for s in validator_statuses])}

**Health:** {"✅ Healthy" if analysis['healthy'] else "⚠️ Issues Detected"}

**Status:**
"""
        for status in validator_statuses:
            health_icon = "✅" if status['healthy'] else "❌"
            log_entry += f"- {health_icon} Validator #{status['validator_num']} ({status['name']}): "
            if status['healthy']:
                log_entry += f"{status['metrics'].get('peers', '?')} peers, block {status['metrics'].get('block_height', '?')}\n"
            else:
                log_entry += f"{', '.join(status['issues'])}\n"

        if analysis['actions_needed']:
            log_entry += "\n**Actions Taken:**\n"
            for action in analysis['actions_needed']:
                log_entry += f"- {action['action'].upper()} validator #{action['validator']}: {action['reason']}\n"

        if analysis['alerts']:
            log_entry += "\n**Alerts:**\n"
            for alert in analysis['alerts']:
                log_entry += f"- [{alert['severity'].upper()}] {alert['message']}\n"

        log_entry += f"\n**Reasoning:** {analysis['reasoning']}\n"
        log_entry += "\n---\n"

        # Append to GLOBAL_MEMORY
        try:
            with open(self.memory_path, 'a') as f:
                f.write(log_entry)
        except Exception as e:
            print(f"[{self.aidev_id}] Failed to write to memory: {e}")

    def monitoring_cycle(self):
        """Run one complete monitoring cycle"""
        print(f"\n[{self.aidev_id}] Starting monitoring cycle...")

        # 1. Check validators
        statuses = self.check_validators()

        # 2. Analyze with Claude
        analysis = self.analyze_with_claude(statuses)

        # 3. Execute actions
        if analysis.get('actions_needed'):
            self.execute_actions(analysis['actions_needed'])

        # 4. Log to memory
        self.log_to_memory(analysis, statuses)

        # 5. Print summary
        print(f"[{self.aidev_id}] {analysis['summary']}")

        return analysis
```

---

### Component 3: Orchestrator (Main Loop)

**File:** `/ai-monitoring/orchestrator.py`

```python
import os
import time
from validator_monitor import ValidatorMonitor
from ai_dev_workers import AIDevWorker

class AIDevOrchestrator:
    """
    Orchestrates all 12 AI dev workers
    Runs continuous monitoring cycles
    """

    def __init__(self,
                 api_key: str,
                 validator_ips_path: str,
                 ssh_key_path: str,
                 prometheus_url: str):
        """
        Args:
            api_key: Anthropic API key (shared by all workers)
            validator_ips_path: Path to validator-ips.json
            ssh_key_path: Path to SSH private key
            prometheus_url: Prometheus server URL
        """
        self.api_key = api_key

        # Initialize validator monitor
        self.monitor = ValidatorMonitor(
            validator_ips_path=validator_ips_path,
            ssh_key_path=ssh_key_path,
            prometheus_url=prometheus_url
        )

        # Create all 12 AI dev workers
        self.workers = {
            'governance-dev01': AIDevWorker('governance-dev01', api_key, self.monitor),
            'security-dev01': AIDevWorker('security-dev01', api_key, self.monitor),
            'audit-dev01': AIDevWorker('audit-dev01', api_key, self.monitor),
            'consensus-dev01': AIDevWorker('consensus-dev01', api_key, self.monitor),
            'runtime-dev01': AIDevWorker('runtime-dev01', api_key, self.monitor),
            'compiler-dev01': AIDevWorker('compiler-dev01', api_key, self.monitor),
            'multichain-dev01': AIDevWorker('multichain-dev01', api_key, self.monitor),
            'oracle-dev01': AIDevWorker('oracle-dev01', api_key, self.monitor),
            'edsc-dev01': AIDevWorker('edsc-dev01', api_key, self.monitor),
            'economics-dev01': AIDevWorker('economics-dev01', api_key, self.monitor),
            'ethics-dev01': AIDevWorker('ethics-dev01', api_key, self.monitor),
            'docs-dev01': AIDevWorker('docs-dev01', api_key, self.monitor),
        }

        print(f"✅ Initialized {len(self.workers)} AI dev workers")

    def run_monitoring_cycle(self):
        """Run one monitoring cycle for all AI devs"""
        print("\n" + "="*60)
        print(f"AI DEV ORCHESTRATOR - Monitoring Cycle")
        print("="*60)

        results = {}
        for aidev_id, worker in self.workers.items():
            try:
                results[aidev_id] = worker.monitoring_cycle()
            except Exception as e:
                print(f"[{aidev_id}] ERROR: {e}")
                results[aidev_id] = {"error": str(e)}

        print("\n" + "="*60)
        print("Monitoring cycle complete")
        print("="*60)

        return results

    def run_forever(self, interval_seconds=60):
        """Run continuous monitoring"""
        print(f"\n🚀 Starting AI Dev Monitoring")
        print(f"   Interval: {interval_seconds} seconds")
        print(f"   Workers: {len(self.workers)}")
        print(f"   Total Validators: 21")
        print()

        while True:
            try:
                self.run_monitoring_cycle()
                time.sleep(interval_seconds)
            except KeyboardInterrupt:
                print("\n\n⏹️  Stopping AI dev monitoring...")
                break
            except Exception as e:
                print(f"\n❌ Orchestrator error: {e}")
                time.sleep(interval_seconds)


# Main entry point
if __name__ == '__main__':
    # Configuration
    API_KEY = os.getenv('ANTHROPIC_API_KEY')
    if not API_KEY:
        print("❌ Error: Set ANTHROPIC_API_KEY environment variable")
        exit(1)

    VALIDATOR_IPS_PATH = '/Users/macbook/Desktop/etrid/validator-ips.json'
    SSH_KEY_PATH = '/Users/macbook/.ssh/gizzi-validator'
    PROMETHEUS_URL = 'http://localhost:9090'  # Or monitoring server IP

    # Create orchestrator
    orchestrator = AIDevOrchestrator(
        api_key=API_KEY,
        validator_ips_path=VALIDATOR_IPS_PATH,
        ssh_key_path=SSH_KEY_PATH,
        prometheus_url=PROMETHEUS_URL
    )

    # Run forever (every 60 seconds)
    orchestrator.run_forever(interval_seconds=60)
```

---

## 💰 Cost Analysis

### Claude API Pricing (as of 2025)

**Model:** Claude Sonnet 4 (claude-sonnet-4-20250514)
- Input: $3 per 1M tokens
- Output: $15 per 1M tokens

### Per Monitoring Cycle

**Single AI dev call:**
- Input tokens: ~800 (system prompt + validator status)
- Output tokens: ~200 (JSON response)
- Cost per call: (800 × $3/1M) + (200 × $15/1M) = $0.0054

**Full cycle (12 AI devs):**
- 12 calls × $0.0054 = **$0.0648 per cycle**

### Monthly Cost Estimates

| Interval | Cycles/Day | Cycles/Month | Cost/Month |
|----------|------------|--------------|------------|
| **1 minute** | 1,440 | 43,200 | **$2,799** |
| **5 minutes** | 288 | 8,640 | **$560** |
| **15 minutes** | 96 | 2,880 | **$187** |
| **1 hour** | 24 | 720 | **$47** |

### Recommended Configuration

**Production:** 5-minute intervals
- Cost: **$560/month**
- 288 checks per day per validator
- Responsive to issues
- Cost-effective

**Cost Optimization Strategy:**

```python
def monitoring_cycle_optimized(self):
    """Cost-optimized: Only use Claude if issues detected"""

    # 1. Simple health check (free)
    statuses = self.check_validators()

    # 2. Quick analysis (no API)
    any_issues = any(not s['healthy'] for s in statuses)

    if any_issues:
        # 3. Use Claude only when needed
        analysis = self.analyze_with_claude(statuses)
        self.execute_actions(analysis['actions_needed'])
    else:
        # 4. No Claude call needed
        print(f"[{self.aidev_id}] All validators healthy")
        analysis = {
            "summary": "All validators healthy",
            "healthy": True,
            "actions_needed": [],
            "alerts": []
        }

    self.log_to_memory(analysis, statuses)
    return analysis
```

**With Optimization:**
- Only calls Claude when issues detected
- Assume 10% of cycles have issues
- Cost: **$56/month** (90% savings!)

---

## 🚀 Deployment Steps

### Step 1: Install Dependencies

```bash
# On monitoring server (or your Mac for testing)
pip3 install anthropic paramiko requests
```

### Step 2: Set API Key

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

### Step 3: Create Directory Structure

```bash
mkdir -p /ai-monitoring
cd /ai-monitoring

# Create files
touch validator_monitor.py
touch ai_dev_workers.py
touch orchestrator.py

# Copy validator-ips.json
cp /Users/macbook/Desktop/etrid/validator-ips.json /ai-monitoring/
```

### Step 4: Deploy Code

Copy the three Python files (validator_monitor.py, ai_dev_workers.py, orchestrator.py) to `/ai-monitoring/`.

### Step 5: Test Single AI Dev

```bash
cd /ai-monitoring

# Test consensus-dev only
python3 <<'EOF'
from orchestrator import AIDevOrchestrator
import os

orchestrator = AIDevOrchestrator(
    api_key=os.getenv('ANTHROPIC_API_KEY'),
    validator_ips_path='validator-ips.json',
    ssh_key_path='/Users/macbook/.ssh/gizzi-validator',
    prometheus_url='http://localhost:9090'
)

# Run one cycle for consensus-dev only
worker = orchestrator.workers['consensus-dev01']
worker.monitoring_cycle()
EOF
```

### Step 6: Run All 12 AI Devs

```bash
python3 orchestrator.py
```

**Output:**
```
🚀 Starting AI Dev Monitoring
   Interval: 60 seconds
   Workers: 12
   Total Validators: 21

✅ Initialized 12 AI dev workers
[governance-dev01] Initialized with 1 validators
[security-dev01] Initialized with 1 validators
...

============================================================
AI DEV ORCHESTRATOR - Monitoring Cycle
============================================================

[governance-dev01] Starting monitoring cycle...
[governance-dev01] All validators healthy

[consensus-dev01] Starting monitoring cycle...
[consensus-dev01] Validator 4 offline - restarting
✅ Restarted validator 4: offline

[runtime-dev01] Starting monitoring cycle...
[runtime-dev01] All validators healthy

...

============================================================
Monitoring cycle complete
============================================================
```

### Step 7: Run as Systemd Service (24/7)

```bash
sudo tee /etc/systemd/system/ai-dev-monitoring.service > /dev/null <<'EOF'
[Unit]
Description=AI Dev Blockchain Monitoring
After=network.target prometheus.service

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/ai-monitoring
Environment="ANTHROPIC_API_KEY=your_api_key_here"
ExecStart=/usr/bin/python3 /ai-monitoring/orchestrator.py
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

## 📊 Monitoring the Monitors

### Check GLOBAL_MEMORY.md

```bash
tail -50 /Users/macbook/Desktop/etrid/14-aidevs/memory/GLOBAL_MEMORY.md
```

**Example Output:**
```markdown
## [2025-10-31 14:30:00] consensus-dev01

**Summary:** All validators healthy

**Validators:** #4, #5

**Health:** ✅ Healthy

**Status:**
- ✅ Validator #4 (Consensus Dev Primary): 18 peers, block 123456
- ✅ Validator #5 (Consensus Dev Secondary): 20 peers, block 123456

**Reasoning:** Both validators are producing blocks with healthy peer counts...

---
```

### Prometheus Metrics

Add custom metrics to track AI dev activity:

```python
from prometheus_client import Counter, Gauge, start_http_server

# Metrics
ai_dev_cycles = Counter('ai_dev_monitoring_cycles_total',
                        'Total monitoring cycles', ['aidev_id'])
ai_dev_actions = Counter('ai_dev_actions_total',
                        'Total actions taken', ['aidev_id', 'action'])
ai_dev_claude_calls = Counter('ai_dev_claude_api_calls_total',
                              'Total Claude API calls', ['aidev_id'])

# Start metrics server
start_http_server(8000)  # Metrics at http://localhost:8000/metrics
```

---

## 🎯 Next Steps

### Phase 1: Testing (This Week)
1. ✅ Deploy orchestrator on your Mac
2. ✅ Test with 2 Oracle Cloud validators (1 & 5)
3. ✅ Verify Claude API calls working
4. ✅ Check GLOBAL_MEMORY logging
5. ✅ Test restart action

### Phase 2: Production (After SSH Access Restored)
1. ✅ Deploy to dedicated monitoring server
2. ✅ Configure Prometheus for all 21 validators
3. ✅ Enable all 12 AI dev workers
4. ✅ Set up systemd service (24/7)
5. ✅ Monitor costs and optimize

### Phase 3: Advanced Features (Week 2)
1. ✅ Add Discord/Telegram alerts
2. ✅ Implement cross-dev coordination (Gizzi orchestrator)
3. ✅ Add predictive monitoring (trend analysis)
4. ✅ Create Grafana dashboard for AI dev activity

---

## 💡 Key Design Decisions

### Why Claude API vs Claude Desktop?

| Feature | Claude Desktop | Claude API |
|---------|----------------|------------|
| Cost | $20/month subscription | $56/month optimized |
| Reliability | Requires Mac on 24/7 | Cloud-based, always on |
| Scalability | Limited to Mac | Can run anywhere |
| Automation | Difficult | Native support |
| Production | ❌ Not suitable | ✅ Production-ready |

**Decision:** Use Claude API for production, Desktop for local testing.

### Why Centralized Orchestrator?

**Pros:**
- Single process to manage
- Shared ValidatorMonitor instance (efficient)
- Easy to start/stop/monitor
- Single API key
- Logs in one place

**Cons:**
- Single point of failure (mitigate with systemd restart)

**Alternative (Distributed):** Each AI dev on separate VM
- More complex
- Higher cost (12 VMs)
- More resilient
- Consider for Phase 3

### Why 60-Second Intervals?

**Balance:**
- Fast enough to detect issues quickly
- Slow enough to keep costs reasonable
- Gives validators time to recover
- Avoids alert fatigue

**Can adjust:**
- Critical validators (bootnodes): 30 seconds
- Non-critical: 5 minutes

---

## 📞 Support & Troubleshooting

### Common Issues

**Issue: Claude API rate limits**
```
Solution: Add exponential backoff
Wait 1, 2, 4, 8 seconds between retries
```

**Issue: SSH timeouts**
```
Solution: Increase SSH timeout to 10 seconds
Add retry logic for transient failures
```

**Issue: High costs**
```
Solution: Enable optimized monitoring (only call Claude on issues)
Increase interval to 5 minutes
```

**Issue: Memory growth**
```
Solution: Rotate GLOBAL_MEMORY.md daily
Keep last 7 days only
```

---

## 🎯 Success Criteria

✅ All 21 validators monitored by assigned AI devs
✅ Automatic restarts working (validators come back online)
✅ Claude API calls < $100/month (optimized)
✅ GLOBAL_MEMORY accumulating detailed logs
✅ No validators down > 5 minutes
✅ AI devs detect 100% of issues

---

**Status:** Implementation plan complete and ready to deploy.

**Next Action:** Deploy orchestrator.py and test with existing 2 accessible validators.

**Estimated Setup Time:** 30 minutes to 1 hour.
