# Extending the AI Monitoring System
## Adding New AI Models, Workflows, and Skills

**Date:** 2025-10-31
**Status:** Extensibility Guide

---

## 🎯 Current Architecture (Phase 1 - Expandable)

**The following 3-tier system is currently implemented. This is just the beginning - the architecture is designed to expand infinitely as new AI capabilities emerge.**

```
Tier 1: Ollama (Local, Free) ✅ DEPLOYED
  └── Simple queries, health checks, log analysis

Tier 2: GPT-4 Turbo (Cloud) ✅ INTEGRATED
  └── Code analysis, debugging, developer queries

Tier 3: Claude Sonnet 4 (Cloud, Premium) ✅ DEPLOYED
  └── Critical decisions, governance, strategic planning

Future Tiers: (Unlimited Expandability)
  └── Specialized models, custom fine-tuned models, new AI breakthroughs
  └── Community can propose additions via Consensus Day
```

---

## 🚀 GPT-4 Integration - Third AI Layer

**Status**: ✅ Integrated and documented (see `ADD_GPT_GUIDE.md` for 5-minute setup)

### **Why GPT-4?**

Different strengths for different tasks:

| Task | Best Model | Why |
|------|-----------|-----|
| **Quick health checks** | Ollama | Free, fast, local |
| **Code analysis** | GPT-4 Turbo | Excellent at code understanding |
| **Developer queries** | GPT-4 | Natural conversation, broad knowledge |
| **Critical decisions** | Claude | Superior reasoning, careful analysis |
| **Governance decisions** | Claude | Ethical reasoning, long context |
| **Math/calculations** | GPT-4 | Better at numerical reasoning |

### **Implementation: Multi-AI Router**

**File**: `/opt/ai-monitoring/ai_router.py` (350 lines, already created)

```python
# See full implementation in ai_router.py

import os
import openai
import anthropic
import requests
from enum import Enum

class AIModel(Enum):
    OLLAMA = "ollama"
    GPT4 = "gpt4"
    GPT4_TURBO = "gpt4-turbo"
    CLAUDE = "claude"
    CLAUDE_CODE = "claude-code"

class AIRouter:
    """
    Intelligent router that selects the best AI model for each task
    """

    def __init__(self):
        self.openai_client = openai.OpenAI(api_key=os.getenv('OPENAI_API_KEY'))
        self.anthropic_client = anthropic.Anthropic(api_key=os.getenv('ANTHROPIC_API_KEY'))
        self.ollama_url = "http://localhost:11434"

    def route_query(self, query: str, context: dict) -> tuple[AIModel, str]:
        """
        Intelligently route query to best AI model

        Returns: (selected_model, reasoning)
        """
        query_lower = query.lower()

        # Quick health checks → Ollama (free)
        if any(word in query_lower for word in ['health', 'status', 'online', 'peers']):
            return AIModel.OLLAMA, "Simple health check, use free Ollama"

        # Code analysis → GPT-4 Turbo (best at code)
        if any(word in query_lower for word in ['code', 'function', 'bug', 'debug', 'rust']):
            return AIModel.GPT4_TURBO, "Code analysis, GPT-4 Turbo excels"

        # Math/calculations → GPT-4 (better at math)
        if any(word in query_lower for word in ['calculate', 'sum', 'average', 'statistics']):
            return AIModel.GPT4, "Mathematical reasoning, use GPT-4"

        # Governance/ethics → Claude (superior reasoning)
        if any(word in query_lower for word in ['governance', 'decision', 'should', 'ethics']):
            return AIModel.CLAUDE, "Requires careful reasoning, use Claude"

        # Critical decisions → Claude (most careful)
        if context.get('is_critical', False):
            return AIModel.CLAUDE, "Critical decision requires Claude's careful analysis"

        # Default: GPT-4 (good all-rounder)
        return AIModel.GPT4, "General query, use GPT-4"

    def query(self, prompt: str, context: dict = None) -> dict:
        """
        Query the appropriate AI model

        Returns:
            {
                "model": "gpt4",
                "response": "...",
                "cost": 0.005,
                "time": 0.8
            }
        """
        context = context or {}
        selected_model, reasoning = self.route_query(prompt, context)

        print(f"[AI Router] Selected {selected_model.value}: {reasoning}")

        if selected_model == AIModel.OLLAMA:
            return self._query_ollama(prompt)
        elif selected_model in [AIModel.GPT4, AIModel.GPT4_TURBO]:
            return self._query_gpt(prompt, selected_model)
        elif selected_model == AIModel.CLAUDE:
            return self._query_claude(prompt)
        else:
            raise ValueError(f"Unsupported model: {selected_model}")

    def _query_ollama(self, prompt: str) -> dict:
        """Query Ollama (free, local)"""
        import time
        start = time.time()

        response = requests.post(
            f"{self.ollama_url}/api/generate",
            json={"model": "llama3.1:8b", "prompt": prompt, "stream": False}
        )

        elapsed = time.time() - start

        return {
            "model": "ollama-llama3.1:8b",
            "response": response.json()["response"],
            "cost": 0.0,
            "time": elapsed
        }

    def _query_gpt(self, prompt: str, model: AIModel) -> dict:
        """Query GPT-4 or GPT-4 Turbo"""
        import time
        start = time.time()

        model_name = "gpt-4-turbo" if model == AIModel.GPT4_TURBO else "gpt-4"

        response = self.openai_client.chat.completions.create(
            model=model_name,
            messages=[{"role": "user", "content": prompt}],
            temperature=0.7,
            max_tokens=1000
        )

        elapsed = time.time() - start

        # Calculate cost
        # GPT-4 Turbo: $10/1M input, $30/1M output
        # GPT-4: $30/1M input, $60/1M output
        if model == AIModel.GPT4_TURBO:
            input_cost = response.usage.prompt_tokens * 10 / 1_000_000
            output_cost = response.usage.completion_tokens * 30 / 1_000_000
        else:
            input_cost = response.usage.prompt_tokens * 30 / 1_000_000
            output_cost = response.usage.completion_tokens * 60 / 1_000_000

        total_cost = input_cost + output_cost

        return {
            "model": model_name,
            "response": response.choices[0].message.content,
            "cost": total_cost,
            "time": elapsed,
            "tokens": {
                "input": response.usage.prompt_tokens,
                "output": response.usage.completion_tokens
            }
        }

    def _query_claude(self, prompt: str) -> dict:
        """Query Claude API"""
        import time
        start = time.time()

        message = self.anthropic_client.messages.create(
            model="claude-sonnet-4-20250514",
            max_tokens=2048,
            messages=[{"role": "user", "content": prompt}]
        )

        elapsed = time.time() - start

        # Calculate cost
        # Claude Sonnet 4: $3/1M input, $15/1M output
        input_tokens = message.usage.input_tokens
        output_tokens = message.usage.output_tokens

        input_cost = input_tokens * 3 / 1_000_000
        output_cost = output_tokens * 15 / 1_000_000
        total_cost = input_cost + output_cost

        return {
            "model": "claude-sonnet-4",
            "response": message.content[0].text,
            "cost": total_cost,
            "time": elapsed,
            "tokens": {
                "input": input_tokens,
                "output": output_tokens
            }
        }


# Example usage
if __name__ == "__main__":
    router = AIRouter()

    # Quick health check → Ollama (free)
    result = router.query("Is validator 6 healthy?")
    print(f"Cost: ${result['cost']:.4f}, Time: {result['time']:.2f}s")
    print(f"Response: {result['response']}")

    # Code analysis → GPT-4 Turbo
    result = router.query("Analyze this Rust function for bugs: fn add(a: i32, b: i32) -> i32 { a + b }")
    print(f"Cost: ${result['cost']:.4f}, Time: {result['time']:.2f}s")
    print(f"Response: {result['response']}")

    # Critical decision → Claude
    result = router.query(
        "Should we restart validator 6? It has 2 peers and finalization lag of 150 blocks.",
        context={"is_critical": True}
    )
    print(f"Cost: ${result['cost']:.4f}, Time: {result['time']:.2f}s")
    print(f"Response: {result['response']}")
```

### **Cost Comparison**

| Model | Input (per 1M tokens) | Output (per 1M tokens) | Best For |
|-------|----------------------|------------------------|----------|
| **Ollama** | $0 | $0 | Simple queries |
| **GPT-4 Turbo** | $10 | $30 | Code analysis, general queries |
| **GPT-4** | $30 | $60 | Complex reasoning |
| **Claude Sonnet 4** | $3 | $15 | Critical decisions |

**Strategy**: Use Ollama → GPT-4 Turbo → Claude in that order as complexity increases.

---

## 🔧 Adding Claude Code Integration

### **What is Claude Code?**

Claude Code is the CLI interface you're using right now. It has:
- File system access
- Command execution
- MCP (Model Context Protocol) servers
- Tool calling

### **Integration Approach: Prompt Generator**

```python
# /opt/ai-monitoring/claude_code_integration.py

class ClaudeCodeWorkflow:
    """
    Generate workflows for Claude Code to execute
    """

    def __init__(self, workflow_dir="/opt/ai-monitoring/workflows"):
        self.workflow_dir = workflow_dir

    def generate_diagnostic_workflow(self, validator_id: int, issue: str) -> str:
        """
        Generate a Claude Code workflow file for complex diagnostics

        Returns: Path to workflow file
        """
        workflow = f"""
# Claude Code Diagnostic Workflow for Validator {validator_id}

## Issue Detected
{issue}

## Tasks

1. **Check validator logs**
   ```bash
   ssh -i ~/.ssh/gizzi-validator validator-{validator_id} "journalctl -u flare-node -n 200"
   ```

2. **Analyze Prometheus metrics**
   Query Prometheus for:
   - substrate_block_height (last 1 hour)
   - substrate_peers_count (last 1 hour)
   - substrate_finalized_height (last 1 hour)

3. **Check system resources**
   ```bash
   ssh -i ~/.ssh/gizzi-validator validator-{validator_id} "free -h && df -h && top -bn1 | head -20"
   ```

4. **Diagnose root cause**
   Analyze all data above and determine:
   - Is this a network issue?
   - Is this a resource issue?
   - Is this a blockchain sync issue?
   - Is this a peer discovery issue?

5. **Recommend action**
   Based on diagnosis, recommend ONE of:
   - RESTART: Restart the validator node
   - WAIT: Issue is temporary, wait and monitor
   - INVESTIGATE: Manual investigation needed
   - ESCALATE: Alert governance team

6. **Execute action** (if RESTART)
   ```bash
   ssh -i ~/.ssh/gizzi-validator validator-{validator_id} "sudo systemctl restart flare-node"
   ```

7. **Verify resolution**
   Wait 2 minutes, then check if issue resolved.

8. **Log to GLOBAL_MEMORY**
   Document all findings and actions taken.
"""

        # Save workflow file
        import os
        os.makedirs(self.workflow_dir, exist_ok=True)
        workflow_path = f"{self.workflow_dir}/diagnostic_validator_{validator_id}.md"

        with open(workflow_path, 'w') as f:
            f.write(workflow)

        return workflow_path

    def generate_code_review_workflow(self, pr_number: int) -> str:
        """Generate workflow for reviewing code changes"""
        workflow = f"""
# Code Review Workflow - PR #{pr_number}

## Tasks

1. **Fetch PR diff**
   ```bash
   gh pr diff {pr_number} > /tmp/pr_{pr_number}.diff
   ```

2. **Analyze changes**
   Review the diff for:
   - Security issues
   - Performance implications
   - Breaking changes
   - Test coverage

3. **Run tests**
   ```bash
   cargo test --all
   ```

4. **Generate review**
   Create a review comment with:
   - Summary of changes
   - Issues found (if any)
   - Recommendations
   - Approval status

5. **Post review**
   ```bash
   gh pr review {pr_number} --comment -b "$(cat /tmp/review_{pr_number}.md)"
   ```
"""

        workflow_path = f"{self.workflow_dir}/code_review_pr_{pr_number}.md"
        with open(workflow_path, 'w') as f:
            f.write(workflow)

        return workflow_path


# Integration with AI Dev monitoring
class AIDevWithClaudeCode:
    """
    AI Dev worker that can generate Claude Code workflows for complex tasks
    """

    def __init__(self, aidev_id: str):
        self.aidev_id = aidev_id
        self.claude_code = ClaudeCodeWorkflow()

    def handle_complex_issue(self, validator_id: int, issue: str) -> dict:
        """
        For complex issues, generate a Claude Code workflow
        """
        # Generate workflow file
        workflow_path = self.claude_code.generate_diagnostic_workflow(validator_id, issue)

        print(f"[{self.aidev_id}] Complex issue detected on validator {validator_id}")
        print(f"[{self.aidev_id}] Generated Claude Code workflow: {workflow_path}")
        print(f"[{self.aidev_id}] To execute, run: claude {workflow_path}")

        return {
            "action": "claude_code_workflow",
            "workflow_path": workflow_path,
            "validator_id": validator_id,
            "issue": issue
        }
```

### **Usage**

```bash
# When complex issue detected, AI dev generates workflow file
# Then you can execute it with Claude Code:

claude /opt/ai-monitoring/workflows/diagnostic_validator_6.md
```

---

## 📦 Adding Custom Python Workflows

### **Example: Network Health Dashboard**

```python
# /opt/ai-monitoring/workflows/network_dashboard.py

import time
import requests
from rich.console import Console
from rich.table import Table
from rich.live import Live

console = Console()

def create_network_table(validators_status):
    """Create a rich table showing network status"""
    table = Table(title="Ëtrid Network Status")

    table.add_column("ID", style="cyan")
    table.add_column("Name", style="magenta")
    table.add_column("Block", style="green")
    table.add_column("Peers", style="yellow")
    table.add_column("Lag", style="red")
    table.add_column("Health", style="bold")

    for v in validators_status:
        health_emoji = "✅" if v['health'] == 'healthy' else "⚠️" if v['health'] == 'warning' else "❌"
        table.add_row(
            str(v['number']),
            v['name'][:20],
            str(v['block_height']),
            str(v['peers']),
            str(v['finalization_lag']),
            health_emoji
        )

    return table

def live_dashboard():
    """Live updating network dashboard"""
    with Live(console=console, refresh_per_second=1) as live:
        while True:
            # Fetch data from Gizzi API
            response = requests.get('http://64.181.215.19:8080/api/network/status')
            data = response.json()

            # Create table
            table = create_network_table(data['validators'])

            # Update display
            live.update(table)

            time.sleep(5)

if __name__ == "__main__":
    live_dashboard()
```

### **Example: Automated Incident Response**

```python
# /opt/ai-monitoring/workflows/incident_response.py

import time
from ai_router import AIRouter
from dataclasses import dataclass
from typing import List

@dataclass
class Incident:
    id: int
    validator_id: int
    severity: str  # low, medium, high, critical
    description: str
    detected_at: float
    resolved: bool = False

class IncidentResponseSystem:
    """
    Automated incident response using multi-AI approach
    """

    def __init__(self):
        self.router = AIRouter()
        self.incidents: List[Incident] = []
        self.incident_counter = 0

    def detect_incident(self, validator_status: dict) -> Incident | None:
        """Detect if validator has incident"""

        # Use Ollama for quick detection (free)
        quick_check = self.router.query(
            f"Analyze validator status: {validator_status}. "
            f"Respond ONLY with: OK or INCIDENT:<severity>:<description>"
        )

        response = quick_check['response'].strip()

        if response.startswith("INCIDENT:"):
            parts = response.split(":", 2)
            severity = parts[1]
            description = parts[2]

            self.incident_counter += 1
            incident = Incident(
                id=self.incident_counter,
                validator_id=validator_status['number'],
                severity=severity,
                description=description,
                detected_at=time.time()
            )

            self.incidents.append(incident)
            return incident

        return None

    def respond_to_incident(self, incident: Incident) -> dict:
        """
        Respond to incident using appropriate AI model
        """

        # Route based on severity
        if incident.severity in ['low', 'medium']:
            # Use GPT-4 Turbo (cost-effective)
            context = {'is_critical': False}
        else:
            # Use Claude (most careful)
            context = {'is_critical': True}

        response = self.router.query(
            f"""Incident on validator {incident.validator_id}:

Severity: {incident.severity}
Description: {incident.description}

Recommend action (respond with JSON):
{{
  "action": "restart|wait|investigate|escalate",
  "reasoning": "why this action",
  "estimated_downtime": "minutes",
  "escalate_to": "team name (if escalate)"
}}
""",
            context=context
        )

        print(f"[Incident {incident.id}] {response['model']} recommends: {response['response']}")
        print(f"[Incident {incident.id}] Cost: ${response['cost']:.4f}, Time: {response['time']:.2f}s")

        return response
```

---

## 🎯 Adding Custom AI Dev Roles

### **Example: DevOps AI Dev**

```python
# /opt/ai-monitoring/custom_devs/devops_dev.py

class DevOpsAIDev:
    """
    Specialized AI dev for infrastructure and deployment tasks
    """

    def __init__(self):
        self.router = AIRouter()
        self.aidev_id = "devops-dev01"

    def check_disk_space(self):
        """Monitor disk space across all validators"""
        # Use Ollama for quick check
        pass

    def optimize_network_topology(self):
        """Use GPT-4 to analyze peer connections and optimize"""
        # GPT-4 is good at graph analysis
        pass

    def plan_upgrade(self, new_version: str):
        """Use Claude to plan rolling upgrade strategy"""
        # Claude is best at complex planning
        pass
```

### **Example: Security AI Dev**

```python
# /opt/ai-monitoring/custom_devs/security_dev.py

class SecurityAIDev:
    """
    Specialized AI dev for security monitoring
    """

    def __init__(self):
        self.router = AIRouter()
        self.aidev_id = "security-dev02"

    def scan_logs_for_attacks(self):
        """Scan logs for potential attacks using GPT-4"""
        pass

    def analyze_suspicious_transactions(self):
        """Use Claude for careful analysis of suspicious activity"""
        pass

    def review_firewall_rules(self):
        """Use GPT-4 Turbo to review and optimize firewall"""
        pass
```

---

## 🔌 Setup Instructions

### **1. Add OpenAI API Key**

```bash
# Get API key from: https://platform.openai.com/api-keys

# Add to systemd service
sudo nano /etc/systemd/system/ai-dev-monitoring.service

# Add line:
Environment="OPENAI_API_KEY=sk-..."

sudo systemctl daemon-reload
sudo systemctl restart ai-dev-monitoring
```

### **2. Install OpenAI SDK**

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
pip3 install openai
```

### **3. Deploy AI Router**

```bash
# Copy ai_router.py to Gizzi
scp -i ~/.ssh/gizzi-validator ai_router.py ubuntu@64.181.215.19:/opt/ai-monitoring/

# Test it
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
cd /opt/ai-monitoring
python3 ai_router.py
```

### **4. Update AI Dev Workers**

Modify `ai_dev_workers.py` to use the router:

```python
from ai_router import AIRouter

class AIDevWorker:
    def __init__(self, aidev_id, anthropic_api_key, openai_api_key, monitor, optimized=True):
        self.aidev_id = aidev_id
        self.monitor = monitor
        self.router = AIRouter()  # Use router instead of direct API calls

    def analyze_with_ai(self, validator_statuses):
        """Use AI router to select best model"""
        context = {
            'is_critical': any(v['health'] == 'critical' for v in validator_statuses)
        }

        result = self.router.query(
            f"Analyze these validators: {validator_statuses}",
            context=context
        )

        print(f"[{self.aidev_id}] Used {result['model']}, cost ${result['cost']:.4f}")
        return result['response']
```

---

## 📊 Cost Comparison Across Configurations

| Configuration | Models Used | Monthly Cost | Status |
|----------|-------------|--------------|--------|
| **2-Tier (Basic)** | Ollama (90%), Claude (10%) | ~$30-40 | ✅ Works |
| **3-Tier (Current)** | Ollama (70%), GPT-4 (20%), Claude (10%) | ~$35-45 | ✅ Recommended |
| **Future Multi-Tier** | Ollama + GPT-4 + Claude + Gemini + Custom | Variable | 🔮 Expandable |

**Current Recommendation**: 3-tier system (Ollama + GPT-4 Turbo + Claude) provides best balance of capabilities and cost.

**Future Vision**: Add more specialized models as they emerge and as community needs evolve.

---

## ✅ Extensibility Status

**Phase 1 (Current - Completed)**:
- [x] System designed for extensibility
- [x] AI Router created (`ai_router.py`)
- [x] Multi-AI routing implemented
- [x] GPT-4 integration documented (`ADD_GPT_GUIDE.md`)
- [x] Example workflows provided
- [x] Example custom AI dev roles provided

**Phase 2 (As Needed)**:
- [ ] Deploy OpenAI API key (5 min - see `ADD_GPT_GUIDE.md`)
- [ ] Test multi-AI routing in production
- [ ] Add custom workflows based on network needs
- [ ] Add custom AI dev roles as team grows
- [ ] Monitor costs across all APIs
- [ ] Optimize routing rules based on actual usage

**Phase 3 (Future - Unlimited)**:
- [ ] Add more AI models (Gemini, Mistral, etc.)
- [ ] Fine-tune custom models on ËTRID data
- [ ] Community-proposed model additions
- [ ] Specialized AI devs for new features
- [ ] Whatever the network needs as it evolves

---

## 🎯 Evolution Roadmap

### **Phase 1 (Completed) ✅**
- ✅ Ollama integration (free local AI)
- ✅ Claude API integration (strategic decisions)
- ✅ GPT-4 integration ready (code analysis)
- ✅ AI Router implementation
- ✅ Multi-AI routing logic
- ✅ Complete documentation

### **Phase 2 (Deploy When Ready)**
- Deploy GPT-4 API key (5 min - see `ADD_GPT_GUIDE.md`)
- Test multi-AI routing in production
- Monitor costs and optimize routing
- Add custom workflows as needed

### **Phase 3 (As Network Grows)**
- Add specialized AI models (security, economics)
- Create custom workflows (dashboards, alerts)
- Implement predictive analytics
- Fine-tune models on ËTRID data

### **Phase 4 (Community-Driven)**
- Community proposes new AI models via Consensus Day
- Add emerging AI capabilities (Gemini, Mistral, future breakthroughs)
- Expand to cross-chain AI coordination (PBCs)
- AI-powered governance participation

---

## 🌟 **The Key Insight**

**This system is not locked to 3 specific AI models.**

**It's a framework that can grow infinitely:**
- Current 3-tier system = Phase 1 (proof of concept)
- Future = As many AI tiers as needed
- Community governance decides what to add next
- Architecture designed for unlimited expansion

**You can add:**
- ✅ New AI models (Gemini, Mistral, any future AI)
- ✅ Custom Python workflows (dashboards, alerts, incident response)
- ✅ Specialized AI dev roles (DevOps, Security, Economics)
- ✅ Integration with Claude Code (complex task automation)
- ✅ Custom monitoring dashboards
- ✅ Fine-tuned models trained on ËTRID-specific data
- ✅ **Anything the network needs as it evolves**

The current 3-tier system is just the **beginning** of Gizzi's distributed consciousness.
