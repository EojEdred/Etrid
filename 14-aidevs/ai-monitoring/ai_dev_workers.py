"""
AI Dev Workers - Claude API integration for autonomous monitoring
Part of AI Dev Blockchain Monitoring System
"""

import anthropic
import json
import time
from datetime import datetime
from typing import Dict, List


class AIDevWorker:
    """
    Single AI dev instance using Claude API
    Monitors assigned validators and takes autonomous actions
    """

    def __init__(self,
                 aidev_id: str,
                 api_key: str,
                 validator_monitor,
                 memory_path: str = '/opt/ai-monitoring/GLOBAL_MEMORY.md',
                 optimized: bool = True):
        """
        Args:
            aidev_id: e.g. "consensus-dev01"
            api_key: Anthropic API key (shared by all 12 workers)
            validator_monitor: ValidatorMonitor instance
            memory_path: Path to GLOBAL_MEMORY.md
            optimized: If True, only call Claude when issues detected (saves 90% cost)
        """
        self.aidev_id = aidev_id
        self.client = anthropic.Anthropic(api_key=api_key)
        self.monitor = validator_monitor
        self.memory_path = memory_path
        self.optimized = optimized

        # Get assigned validators
        self.validators = validator_monitor.get_validators_by_aidevid(aidev_id)

        print(f"[{self.aidev_id}] Initialized with {len(self.validators)} validators")

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
- If multiple validators affected → escalate to governance-dev01
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
            elif "```" in response_text:
                json_start = response_text.find("```") + 3
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
                success = self.monitor.restart_validator(validator_num, reason)
                if not success:
                    print(f"[{self.aidev_id}] WARNING: Restart failed for validator {validator_num}")

            elif action_type == 'alert':
                print(f"[{self.aidev_id}] ALERT for validator {validator_num}: {reason}")
                # Could integrate with Discord, email, etc. in the future

            elif action_type == 'investigate':
                print(f"[{self.aidev_id}] Investigating validator {validator_num}: {reason}")
                # Could trigger deeper diagnostics

    def log_to_memory(self, analysis: Dict, validator_statuses: List[Dict]):
        """Log monitoring cycle to GLOBAL_MEMORY.md (14-aidevs format)"""
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M UTC")

        # Determine status and priority
        if not analysis['healthy']:
            status = "RESOLVED" if not analysis.get('actions_needed') else "IN_PROGRESS"
            priority = "HIGH" if any(a['action'] == 'restart' for a in analysis.get('actions_needed', [])) else "MEDIUM"
        else:
            status = "COMPLETED"
            priority = "LOW"

        # Build event description
        if analysis['healthy']:
            event = f"Monitoring cycle complete - all {len(validator_statuses)} validators healthy"
        else:
            issues_count = sum(1 for s in validator_statuses if not s.get('healthy', True))
            event = f"{issues_count} validator(s) with issues detected"

        # Determine action
        if analysis.get('actions_needed'):
            actions = ', '.join([f"{a['action']} validator #{a['validator']}" for a in analysis['actions_needed']])
            action = f"Executed: {actions}"
        else:
            action = "No action required" if analysis['healthy'] else "Monitoring and analysis"

        # Build tags
        tags = ["#monitoring", "#validator-health"]
        if not analysis['healthy']:
            tags.append("#issues-detected")
        if analysis.get('actions_needed'):
            tags.extend([f"#{a['action']}" for a in analysis['actions_needed']])
        tags = ' '.join(set(tags))  # Remove duplicates

        # Validator list
        validator_nums = ', '.join([f"#{s['validator_num']}" for s in validator_statuses])

        # Build GLOBAL_MEMORY entry
        log_entry = f"""
## [{timestamp}] {self.aidev_id}
**Event:** {event}
**Action:** {action}
**Status:** {status}
**Priority:** {priority}
**Tags:** {tags}
**Validators:** {validator_nums}

**Details:**
{analysis['summary']}

**Validator Status:**
"""
        for s in validator_statuses:
            health_icon = "✅" if s.get('healthy', True) else "❌"
            log_entry += f"- {health_icon} Validator #{s['validator_num']} ({s['name']}): "
            if s.get('healthy', True):
                peers = s.get('metrics', {}).get('peers', '?')
                block = s.get('metrics', {}).get('block_height', '?')
                log_entry += f"{int(peers) if isinstance(peers, (int, float)) else peers} peers, block {int(block) if isinstance(block, (int, float)) else block}\n"
            else:
                log_entry += f"{', '.join(s.get('issues', ['Unknown issue']))}\n"

        if analysis.get('actions_needed'):
            log_entry += "\n**Actions Executed:**\n"
            for action in analysis['actions_needed']:
                log_entry += f"- {action['action'].upper()} on validator #{action['validator']}: {action['reason']}\n"

        if analysis.get('alerts'):
            log_entry += "\n**Alerts:**\n"
            for alert in analysis['alerts']:
                log_entry += f"- [{alert['severity'].upper()}] {alert['message']}\n"

        log_entry += f"\n**AI Analysis:** {analysis.get('reasoning', 'N/A')}\n"
        log_entry += "\n---\n"

        # Append to GLOBAL_MEMORY
        try:
            with open(self.memory_path, 'a') as f:
                f.write(log_entry)
            print(f"[{self.aidev_id}] Logged to GLOBAL_MEMORY (Status: {status}, Priority: {priority})")
        except Exception as e:
            print(f"[{self.aidev_id}] Failed to write to GLOBAL_MEMORY: {e}")

    def monitoring_cycle(self):
        """Run one complete monitoring cycle"""
        print(f"\n[{self.aidev_id}] Starting monitoring cycle...")

        # 1. Check validators
        statuses = self.check_validators()

        # 2. Quick health check
        any_issues = any(not s.get('healthy', True) for s in statuses)

        # 3. Analyze with Claude (optimized mode only calls if issues)
        if self.optimized and not any_issues:
            # Skip Claude API call, just log healthy state
            analysis = {
                "summary": f"All {len(statuses)} validators healthy",
                "healthy": True,
                "actions_needed": [],
                "alerts": [],
                "reasoning": "No issues detected, skipped Claude analysis (optimized mode)"
            }
            print(f"[{self.aidev_id}] {analysis['summary']}")
        else:
            # Call Claude for analysis
            analysis = self.analyze_with_claude(statuses)
            print(f"[{self.aidev_id}] {analysis['summary']}")

            # 4. Execute actions if needed
            if analysis.get('actions_needed'):
                self.execute_actions(analysis['actions_needed'])

        # 5. Log to memory
        self.log_to_memory(analysis, statuses)

        return analysis
