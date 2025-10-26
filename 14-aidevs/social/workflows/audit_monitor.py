#!/usr/bin/env python3
"""
Audit Monitor Workflow - Security and audit alerts

This workflow:
1. Monitors blockchain for security-relevant events:
   - Proposal flags (suspicious parameters)
   - Validator slashing
   - Large treasury spends
   - Reserve ratio warnings
   - Bridge anomalies
2. Analyzes events for severity
3. Generates alert tweets using Claude (Audit Dev voice)
4. Posts alerts (with human approval for critical issues)
5. Logs to GLOBAL_MEMORY.md

Severity Levels:
- Low: Informational, no immediate action needed
- Medium: Worth flagging, monitor closely
- High: Requires attention, potential risk
- Critical: Immediate human review required

Usage:
    python audit_monitor.py --mode stream   # Continuous monitoring
    python audit_monitor.py --mode poll     # Check once
    python audit_monitor.py --dry-run       # Test without posting
"""

import asyncio
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional, Dict, List

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from connectors.blockchain import BlockchainMonitor
from connectors.twitter import TwitterConnector
from content.generator import ContentGenerator
from content.moderation import ContentModerator


class AuditMonitorWorkflow:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.blockchain = BlockchainMonitor()
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

        # Track processed events
        self.processed_events = set()

        # Severity thresholds
        self.severity_config = {
            "reserve_ratio_critical": 1.0,   # < 1.0 = critical
            "reserve_ratio_warning": 1.3,    # < 1.3 = warning
            "large_treasury_spend": 100_000_000_000_000_000_000_000,  # > 100k ETR
            "validator_slash_threshold": 1_000_000_000_000_000_000,    # > 1 ETR slashed
        }

    async def scan_for_audit_events(self, since_block: Optional[int] = None) -> List[Dict]:
        """
        Scan blockchain for audit-worthy events

        Returns:
            List of event objects with severity ratings
        """
        print("🔍 Scanning blockchain for audit events...")

        events = await self.blockchain.get_audit_events(since_block=since_block)

        # Filter out processed events
        new_events = [
            e for e in events
            if self._event_id(e) not in self.processed_events
        ]

        if new_events:
            print(f"✅ Found {len(new_events)} new audit event(s)")
            for event in new_events:
                print(f"   - {event['event_type']} (severity: {event.get('severity', 'unknown')})")
        else:
            print("   No new audit events")

        return new_events

    def _event_id(self, event: Dict) -> str:
        """Generate unique ID for event"""
        return f"{event['event_type']}_{event.get('block_number', 0)}_{event.get('proposal_id', 0)}"

    async def check_reserve_ratio(self) -> Optional[Dict]:
        """
        Check EDSC reserve ratio and return alert if needed

        Returns:
            Alert event dict or None
        """
        print("💰 Checking EDSC reserve ratio...")

        reserve_ratio = await self.blockchain.get_reserve_ratio("EDSC")

        if reserve_ratio < self.severity_config['reserve_ratio_critical']:
            return {
                "event_type": "reserve_ratio_critical",
                "severity": "critical",
                "reserve_ratio": reserve_ratio,
                "threshold": self.severity_config['reserve_ratio_critical'],
                "detected_at": datetime.utcnow(),
            }
        elif reserve_ratio < self.severity_config['reserve_ratio_warning']:
            return {
                "event_type": "reserve_ratio_warning",
                "severity": "high",
                "reserve_ratio": reserve_ratio,
                "threshold": self.severity_config['reserve_ratio_warning'],
                "detected_at": datetime.utcnow(),
            }

        print(f"   Reserve ratio OK: {reserve_ratio:.2f}")
        return None

    async def analyze_proposal(self, proposal_id: int) -> Optional[Dict]:
        """
        Analyze governance proposal for security issues

        Returns:
            Alert event dict or None
        """
        print(f"🔍 Analyzing proposal #{proposal_id}...")

        # TODO: Implement proposal analysis logic
        # - Check for suspicious parameter changes (>10x increase/decrease)
        # - Check for unusual treasury spends
        # - Check for privileged access changes
        # - Use Claude to analyze proposal text for red flags

        # For now, return None (no issues)
        return None

    async def generate_alert_tweet(self, event: Dict) -> str:
        """
        Generate alert tweet using Claude (Audit Dev voice)

        Args:
            event: Event object with severity, type, details

        Returns:
            Tweet text
        """
        print(f"🤖 Generating alert for {event['event_type']}...")

        # Build event description
        if event['event_type'] == 'reserve_ratio_critical':
            details = f"EDSC reserve ratio dropped to {event['reserve_ratio']:.2f} (below {event['threshold']:.2f})"
            severity_emoji = "🚨"
        elif event['event_type'] == 'reserve_ratio_warning':
            details = f"EDSC reserve ratio at {event['reserve_ratio']:.2f} (approaching threshold {event['threshold']:.2f})"
            severity_emoji = "⚠️"
        elif event['event_type'] == 'validator_slashed':
            details = f"Validator {event.get('validator', 'unknown')[:8]}... slashed {event.get('amount', 0)} for misconduct"
            severity_emoji = "🚨"
        elif event['event_type'] == 'proposal_flagged':
            details = f"Proposal #{event.get('proposal_id', '?')} flagged: {event.get('reason', 'Unknown reason')}"
            severity_emoji = "⚠️"
        elif event['event_type'] == 'large_treasury_spend':
            amount_eth = event.get('amount', 0) / 1_000_000_000_000_000_000  # Convert to ETR
            details = f"Large treasury spend detected: {amount_eth:,.0f} ETR"
            severity_emoji = "⚠️"
        else:
            details = str(event)
            severity_emoji = "🔍"

        prompt = f"""Generate an audit alert tweet for Ëtrid.

Event Type: {event['event_type']}
Severity: {event.get('severity', 'unknown').upper()}
Details: {details}

Requirements:
- Voice: Audit Dev (security-focused, critical, thorough)
- Tone: Serious, alert, factual
- Format: Clear alert with {severity_emoji} emoji
- Keep under 250 chars
- Include recommended action if applicable
- Don't cause panic, but be direct about risk
- Include link if relevant: etrid.network/audits

Example structure:
{severity_emoji} Audit Alert

{details}

Severity: {event.get('severity', 'unknown').upper()}
{"Recommendation: [action]" if event.get('severity') in ['high', 'critical'] else ""}
"""

        tweet = await self.generator.generate(
            prompt=prompt,
            dev="audit-dev",
            max_tokens=200
        )

        print(f"✅ Alert tweet generated")
        return tweet

    async def requires_human_approval(self, event: Dict) -> bool:
        """
        Check if event requires human approval before posting

        Critical and high-severity events need human review
        """
        severity = event.get('severity', 'low')
        return severity in ['critical', 'high']

    async def post_alert(self, tweet: str, event: Dict) -> Optional[str]:
        """
        Post alert to Twitter

        Args:
            tweet: Tweet text
            event: Event object

        Returns:
            Tweet ID or None
        """
        # Check if human approval required
        if await self.requires_human_approval(event):
            print(f"⚠️  Event requires human approval - skipping auto-post")
            print(f"   Severity: {event.get('severity')}")
            print(f"   Would post: {tweet}")

            # TODO: Send to approval queue (Slack notification, etc.)
            return None

        # Add signature
        final_tweet = tweet + "\n\n—Audit Dev"

        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would post:\n{final_tweet}\n")
            return "dry_run_id"

        try:
            tweet_id = await self.twitter.post(final_tweet)

            print(f"✅ Posted alert! Tweet ID: {tweet_id}")
            print(f"   URL: https://twitter.com/{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}/status/{tweet_id}")

            return tweet_id

        except Exception as e:
            print(f"❌ Error posting alert: {e}")
            return None

    async def log_to_memory(self, event: Dict, tweet_id: Optional[str]):
        """Log audit event to GLOBAL_MEMORY.md"""
        print("📝 Logging to GLOBAL_MEMORY.md...")

        memory_entry = f"""
## [{datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")}] audit-dev
**Event:** Security/audit alert detected
**Action:** {"Posted alert to Twitter" if tweet_id else "Flagged for human review"}
**Status:** {"COMPLETED" if tweet_id else "PENDING_REVIEW"}
**Priority:** {event.get('severity', 'unknown').upper()}
**Tags:** #audit #security #alert

**Event Type:** {event['event_type']}
**Severity:** {event.get('severity', 'unknown')}
**Details:** {str(event)[:200]}
**Tweet ID:** {tweet_id or 'PENDING_APPROVAL'}

---
"""

        memory_file = Path(__file__).parent.parent.parent / "memory" / "GLOBAL_MEMORY.md"
        with open(memory_file, "a") as f:
            f.write(memory_entry)

        print("✅ Logged to GLOBAL_MEMORY.md")

    async def process_event(self, event: Dict):
        """Process a single audit event"""
        print(f"\n{'='*60}")
        print(f"Processing Audit Event: {event['event_type']}")
        print(f"Severity: {event.get('severity', 'unknown').upper()}")
        print(f"{'='*60}\n")

        # Generate alert tweet
        tweet = await self.generate_alert_tweet(event)
        print(f"📝 Generated alert:\n{tweet}\n")

        # Moderate content
        moderation = await self.moderator.moderate(tweet, strict=True)

        if not moderation['approved']:
            print(f"❌ Alert rejected by moderation: {moderation['issues']}")
            return

        # Post alert (or flag for human review if critical)
        tweet_id = await self.post_alert(tweet, event)

        # Log to memory
        await self.log_to_memory(event, tweet_id)

        # Mark as processed
        self.processed_events.add(self._event_id(event))

    async def poll_once(self):
        """Poll for audit events once"""
        print("🔍 Polling for audit events...")

        # Check multiple sources
        events = []

        # 1. Blockchain events (slashing, large spends, etc.)
        blockchain_events = await self.scan_for_audit_events()
        events.extend(blockchain_events)

        # 2. Reserve ratio check
        reserve_event = await self.check_reserve_ratio()
        if reserve_event:
            events.append(reserve_event)

        # 3. TODO: Other checks (bridge anomalies, etc.)

        # Process each event
        for event in events:
            await self.process_event(event)

    async def stream_continuous(self):
        """Monitor for audit events continuously"""
        print("🌊 Starting continuous audit monitoring...")
        print("   Press Ctrl+C to stop\n")

        while True:
            try:
                await self.poll_once()

                # Wait before next check (every 2 minutes for security monitoring)
                print("⏰ Waiting 2 minutes before next check...\n")
                await asyncio.sleep(120)

            except KeyboardInterrupt:
                print("\n\n✋ Stopping audit monitoring...")
                break

            except Exception as e:
                print(f"❌ Error in monitoring loop: {e}")
                await asyncio.sleep(60)

    async def run(self, mode: str = "poll"):
        """
        Execute workflow

        Args:
            mode: "poll" (check once) or "stream" (continuous)
        """
        print("\n" + "="*60)
        print(f"  Audit Monitor Workflow - {mode.upper()} mode")
        print("="*60 + "\n")

        try:
            if mode == "stream":
                await self.stream_continuous()
            else:
                await self.poll_once()

            print("\n" + "="*60)
            print("  ✅ Workflow Complete!")
            print("="*60 + "\n")

        except Exception as e:
            print(f"\n❌ Workflow error: {e}")
            import traceback
            traceback.print_exc()


async def main():
    # Parse arguments
    dry_run = "--dry-run" in sys.argv
    mode = "stream" if "--mode stream" in " ".join(sys.argv) or "--stream" in sys.argv else "poll"

    # Run workflow
    workflow = AuditMonitorWorkflow(dry_run=dry_run)
    await workflow.run(mode=mode)


if __name__ == "__main__":
    asyncio.run(main())
