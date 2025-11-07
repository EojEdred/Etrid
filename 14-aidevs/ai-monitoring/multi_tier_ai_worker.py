"""
Multi-Tier AI Worker - Ollama → GPT-4 → Claude escalation
Part of Gizzi's Distributed Consciousness
"""

import os
import json
import time
import requests
from datetime import datetime
from typing import Dict, List, Optional
from openai import OpenAI
import anthropic


class MultiTierAIWorker:
    """
    3-Tier AI monitoring system:
    - Tier 1: Ollama (local, free, fast) - Quick checks
    - Tier 2: GPT-4 (API, moderate cost) - Technical analysis
    - Tier 3: Claude (API, highest quality) - Critical decisions & coordination
    """

    def __init__(self,
                 aidev_id: str,
                 validator_monitor,
                 anthropic_api_key: str,
                 openai_api_key: str,
                 ollama_host: str = "http://localhost:11434",
                 memory_path: str = "/opt/ai-monitoring/GLOBAL_MEMORY.md"):
        """
        Args:
            aidev_id: e.g. "consensus-dev01"
            validator_monitor: ValidatorMonitor instance
            anthropic_api_key: Claude API key
            openai_api_key: GPT-4 API key
            ollama_host: Ollama API endpoint
            memory_path: Path to GLOBAL_MEMORY.md
        """
        self.aidev_id = aidev_id
        self.monitor = validator_monitor
        self.memory_path = memory_path

        # Initialize AI clients
        self.ollama_host = ollama_host
        self.openai_client = OpenAI(api_key=openai_api_key)
        self.claude_client = anthropic.Anthropic(api_key=anthropic_api_key)

        # Get assigned validators
        self.validators = validator_monitor.get_validators_by_aidevid(aidev_id)

        # Cost tracking
        self.costs = {
            "ollama_calls": 0,
            "gpt4_calls": 0,
            "claude_calls": 0,
            "total_cost_usd": 0.0
        }

        print(f"[{self.aidev_id}] Multi-tier AI initialized")
        print(f"  - Validators: {len(self.validators)}")
        print(f"  - Tier 1 (Ollama): {ollama_host}")
        print(f"  - Tier 2 (GPT-4): ✓")
        print(f"  - Tier 3 (Claude): ✓")

    def check_validators(self) -> List[Dict]:
        """Check status of all assigned validators"""
        statuses = []
        for validator in self.validators:
            try:
                status = self.monitor.check_validator_status(validator['number'])
                statuses.append(status)
            except Exception as e:
                print(f"[{self.aidev_id}] Error checking validator {validator['number']}: {e}")
                statuses.append({
                    "validator_num": validator['number'],
                    "name": validator.get('name', 'Unknown'),
                    "healthy": False,
                    "issues": [f"Check failed: {str(e)}"]
                })
        return statuses

    def tier1_ollama_check(self, validator_statuses: List[Dict]) -> Dict:
        """
        Tier 1: Quick local check with Ollama (free, fast)
        Returns: Decision to escalate or handle locally
        """
        try:
            # Build simple prompt
            prompt = f"""You are {self.aidev_id}, monitoring blockchain validators.

Current status:
{json.dumps(validator_statuses, indent=2)}

Quick assessment (respond in JSON):
{{
  "all_healthy": true/false,
  "needs_escalation": true/false,
  "reason": "brief explanation",
  "quick_actions": ["action1", "action2"]
}}

Rules:
- If any validator offline: escalate
- If peers < 3 on multiple validators: escalate
- If simple restart fixes it: handle locally
"""

            response = requests.post(
                f"{self.ollama_host}/api/generate",
                json={
                    "model": "llama3.2:latest",
                    "prompt": prompt,
                    "stream": False,
                    "format": "json"
                },
                timeout=30
            )

            if response.status_code == 200:
                result = response.json()
                self.costs["ollama_calls"] += 1

                # Parse response
                decision = json.loads(result.get("response", "{}"))
                print(f"[{self.aidev_id}] Tier 1 (Ollama): {decision.get('reason', 'OK')}")
                return decision
            else:
                raise Exception(f"Ollama API error: {response.status_code}")

        except Exception as e:
            print(f"[{self.aidev_id}] Tier 1 (Ollama) failed: {e}")
            # Auto-escalate on Ollama failure
            return {
                "all_healthy": False,
                "needs_escalation": True,
                "reason": f"Ollama check failed: {e}",
                "quick_actions": []
            }

    def tier2_gpt4_analysis(self, validator_statuses: List[Dict]) -> Dict:
        """
        Tier 2: Technical analysis with GPT-4 (moderate cost)
        Returns: Detailed technical assessment
        """
        try:
            system_prompt = f"""You are {self.aidev_id}, an AI developer monitoring blockchain validators for Ëtrid.

Your role:
- Analyze validator metrics (block production, peers, finalization)
- Diagnose technical issues
- Recommend specific actions
- Decide if Claude-level intervention needed

Escalate to Claude if:
- Multiple validators affected (network-wide issue)
- Complex coordination needed
- Security concerns
- Unknown root cause"""

            user_prompt = f"""Validator Status Report:

```json
{json.dumps(validator_statuses, indent=2)}
```

Provide technical analysis in JSON:
{{
  "summary": "1-sentence summary",
  "severity": "low/medium/high/critical",
  "root_cause": "technical diagnosis",
  "recommended_actions": [
    {{"validator": 4, "action": "restart", "reason": "..."}},
    {{"validator": 5, "action": "investigate_peers", "reason": "..."}}
  ],
  "escalate_to_claude": true/false,
  "escalation_reason": "why Claude needed (if true)"
}}"""

            response = self.openai_client.chat.completions.create(
                model="gpt-4-turbo-preview",
                messages=[
                    {"role": "system", "content": system_prompt},
                    {"role": "user", "content": user_prompt}
                ],
                response_format={"type": "json_object"},
                max_tokens=1500
            )

            self.costs["gpt4_calls"] += 1
            self.costs["total_cost_usd"] += 0.02  # Approx $0.02 per call

            analysis = json.loads(response.choices[0].message.content)
            print(f"[{self.aidev_id}] Tier 2 (GPT-4): {analysis['severity'].upper()} - {analysis['summary']}")

            return analysis

        except Exception as e:
            print(f"[{self.aidev_id}] Tier 2 (GPT-4) failed: {e}")
            # Auto-escalate on GPT-4 failure
            return {
                "summary": f"GPT-4 analysis failed: {e}",
                "severity": "high",
                "root_cause": "Analysis system failure",
                "recommended_actions": [],
                "escalate_to_claude": True,
                "escalation_reason": f"GPT-4 error: {e}"
            }

    def tier3_claude_decision(self, validator_statuses: List[Dict], gpt4_analysis: Dict) -> Dict:
        """
        Tier 3: Critical decision-making with Claude (highest quality)
        Returns: Final decision and coordination plan
        """
        try:
            system_prompt = f"""You are {self.aidev_id}, senior AI architect for Ëtrid blockchain network.

You've been escalated to because:
{gpt4_analysis.get('escalation_reason', 'Complex situation requiring expert decision')}

Your authority:
- Make critical decisions affecting validator operations
- Coordinate with other AI devs via GLOBAL_MEMORY
- Approve/reject automated actions
- Design investigation strategies

Your assigned validators: {', '.join([f"#{v['number']}" for v in self.validators])}

Respond in JSON format with your executive decision."""

            user_prompt = f"""Situation Report:

GPT-4 Analysis:
```json
{json.dumps(gpt4_analysis, indent=2)}
```

Current Validator Status:
```json
{json.dumps(validator_statuses, indent=2)}
```

Provide executive decision:
{{
  "decision": "approve_actions / investigate_further / escalate_governance",
  "approved_actions": [
    {{"validator": 4, "action": "restart", "reason": "...", "priority": "high"}}
  ],
  "coordination_needed": [
    {{"aidev": "security-dev01", "task": "security audit", "reason": "..."}}
  ],
  "investigation_plan": {{
    "duration": "30 minutes",
    "steps": ["step1", "step2"],
    "success_criteria": "..."
  }},
  "reasoning": "detailed explanation of your decision",
  "log_to_memory": {{
    "priority": "HIGH/MEDIUM/LOW",
    "tags": ["#tag1", "#tag2"]
  }}
}}"""

            message = self.claude_client.messages.create(
                model="claude-sonnet-4-20250514",
                max_tokens=3000,
                system=system_prompt,
                messages=[
                    {"role": "user", "content": user_prompt}
                ]
            )

            self.costs["claude_calls"] += 1
            self.costs["total_cost_usd"] += 0.05  # Approx $0.05 per call

            # Parse response
            response_text = message.content[0].text

            # Extract JSON
            if "```json" in response_text:
                json_start = response_text.find("```json") + 7
                json_end = response_text.find("```", json_start)
                response_text = response_text[json_start:json_end].strip()

            decision = json.loads(response_text)
            print(f"[{self.aidev_id}] Tier 3 (Claude): {decision['decision'].upper()}")

            return decision

        except Exception as e:
            print(f"[{self.aidev_id}] Tier 3 (Claude) CRITICAL ERROR: {e}")
            return {
                "decision": "emergency_fallback",
                "approved_actions": [],
                "coordination_needed": [],
                "investigation_plan": {"steps": ["manual investigation required"]},
                "reasoning": f"Claude failed, manual intervention needed: {e}",
                "log_to_memory": {
                    "priority": "CRITICAL",
                    "tags": ["#system-failure", "#manual-review-needed"]
                }
            }

    def execute_actions(self, actions: List[Dict]):
        """Execute approved actions"""
        for action in actions:
            action_type = action.get('action')
            validator_num = action.get('validator')
            reason = action.get('reason', 'no reason given')
            priority = action.get('priority', 'medium')

            print(f"[{self.aidev_id}] [{priority.upper()}] {action_type} validator #{validator_num}: {reason}")

            if action_type == 'restart':
                success = self.monitor.restart_validator(validator_num, reason)
                if not success:
                    print(f"[{self.aidev_id}] ⚠️  Restart FAILED for validator {validator_num}")

    def log_to_memory(self, tier_used: str, decision: Dict, validator_statuses: List[Dict]):
        """Log monitoring cycle to GLOBAL_MEMORY.md"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M UTC")

        # Build entry
        log_entry = f"""
## [{timestamp}] {self.aidev_id}
**AI Tier:** {tier_used}
**Decision:** {decision.get('decision', 'N/A')}
**Status:** {decision.get('log_to_memory', {}).get('priority', 'MEDIUM')}
**Tags:** {' '.join(decision.get('log_to_memory', {}).get('tags', ['#monitoring']))}

**Summary:** {decision.get('summary', decision.get('reasoning', 'Monitoring cycle complete'))}

**Validator Status:**
"""
        for s in validator_statuses:
            health = "✅" if s.get('healthy', True) else "❌"
            log_entry += f"- {health} Validator #{s['validator_num']} ({s.get('name', 'Unknown')})\n"

        if decision.get('approved_actions'):
            log_entry += "\n**Actions Executed:**\n"
            for action in decision['approved_actions']:
                log_entry += f"- {action['action']} on #{action['validator']}\n"

        log_entry += f"\n**Cost This Cycle:** Ollama={self.costs['ollama_calls']}, GPT-4={self.costs['gpt4_calls']}, Claude={self.costs['claude_calls']}\n"
        log_entry += "\n---\n"

        # Write to memory
        try:
            with open(self.memory_path, 'a') as f:
                f.write(log_entry)
            print(f"[{self.aidev_id}] ✓ Logged to GLOBAL_MEMORY")
        except Exception as e:
            print(f"[{self.aidev_id}] ✗ Failed to log: {e}")

    def monitoring_cycle(self):
        """Run one complete multi-tier monitoring cycle"""
        print(f"\n{'='*60}")
        print(f"[{self.aidev_id}] Starting Multi-Tier AI Monitoring Cycle")
        print(f"{'='*60}")

        # Step 1: Check validators
        statuses = self.check_validators()
        any_issues = any(not s.get('healthy', True) for s in statuses)

        # Step 2: Tier 1 - Ollama quick check
        tier1_decision = self.tier1_ollama_check(statuses)

        # Step 3: Decide escalation path
        if tier1_decision.get('all_healthy'):
            # Everything good, log and finish
            final_decision = {
                "decision": "all_healthy",
                "summary": f"All {len(statuses)} validators healthy",
                "approved_actions": [],
                "log_to_memory": {"priority": "LOW", "tags": ["#healthy"]}
            }
            tier_used = "Tier 1 (Ollama)"

        elif tier1_decision.get('needs_escalation'):
            # Escalate to GPT-4
            tier2_analysis = self.tier2_gpt4_analysis(statuses)

            if tier2_analysis.get('escalate_to_claude'):
                # Escalate to Claude
                final_decision = self.tier3_claude_decision(statuses, tier2_analysis)
                tier_used = "Tier 3 (Claude)"
            else:
                # GPT-4 handles it
                final_decision = {
                    "decision": "gpt4_handled",
                    "summary": tier2_analysis['summary'],
                    "approved_actions": tier2_analysis.get('recommended_actions', []),
                    "log_to_memory": {
                        "priority": tier2_analysis['severity'].upper(),
                        "tags": ["#gpt4-handled"]
                    }
                }
                tier_used = "Tier 2 (GPT-4)"
        else:
            # Tier 1 can handle it
            final_decision = {
                "decision": "ollama_handled",
                "summary": tier1_decision['reason'],
                "approved_actions": [],
                "log_to_memory": {"priority": "LOW", "tags": ["#ollama-handled"]}
            }
            tier_used = "Tier 1 (Ollama)"

        # Step 4: Execute actions
        if final_decision.get('approved_actions'):
            self.execute_actions(final_decision['approved_actions'])

        # Step 5: Log to memory
        self.log_to_memory(tier_used, final_decision, statuses)

        print(f"\n[{self.aidev_id}] Cycle complete: {tier_used}")
        print(f"  Cost: ${self.costs['total_cost_usd']:.4f} USD")

        return final_decision


def load_config():
    """Load configuration from .env file"""
    from dotenv import load_dotenv
    load_dotenv('/opt/ai-monitoring/.env')

    return {
        'anthropic_api_key': os.getenv('ANTHROPIC_API_KEY'),
        'openai_api_key': os.getenv('OPENAI_API_KEY'),
        'ollama_host': os.getenv('OLLAMA_HOST', 'http://localhost:11434'),
        'memory_path': os.getenv('GLOBAL_MEMORY_PATH', '/opt/ai-monitoring/GLOBAL_MEMORY.md'),
        'monitoring_interval': int(os.getenv('MONITORING_INTERVAL', '300'))
    }


if __name__ == '__main__':
    # Test multi-tier AI worker
    print("Multi-Tier AI Worker Test")
    print("=" * 60)

    config = load_config()
    print(f"✓ Loaded configuration")
    print(f"  - Ollama: {config['ollama_host']}")
    print(f"  - OpenAI: {'✓' if config['openai_api_key'] else '✗'}")
    print(f"  - Claude: {'✓' if config['anthropic_api_key'] else '✗'}")
    print(f"  - Memory: {config['memory_path']}")
