#!/usr/bin/env python3
"""
Governance Monitor Workflow - Track and announce governance proposals

This workflow:
1. Monitors on-chain governance events (new proposals, voting results)
2. Generates proposal summaries using Claude (Governance Dev voice)
3. Posts announcements to Twitter
4. Logs to GLOBAL_MEMORY.md

Event Types:
- Proposed: New proposal created
- Passed: Proposal approved by vote
- NotPassed: Proposal rejected
- Executed: Proposal executed on-chain

Usage:
    python governance_monitor.py --mode stream   # Continuous monitoring
    python governance_monitor.py --mode poll     # Check once
    python governance_monitor.py --dry-run       # Test without posting
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


class GovernanceMonitorWorkflow:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.blockchain = BlockchainMonitor()
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

        # Track processed proposals to avoid duplicates
        self.processed_proposals = set()

    async def fetch_new_proposals(self) -> List[Dict]:
        """Fetch new governance proposals from blockchain"""
        print("🔍 Fetching new governance proposals...")

        proposals = await self.blockchain.get_governance_proposals(status="active")

        # Filter out already processed
        new_proposals = [
            p for p in proposals
            if p['id'] not in self.processed_proposals
        ]

        if new_proposals:
            print(f"✅ Found {len(new_proposals)} new proposal(s)")
        else:
            print("   No new proposals")

        return new_proposals

    async def generate_proposal_announcement(self, proposal: Dict) -> str:
        """
        Generate proposal announcement tweet

        Args:
            proposal: Proposal object from blockchain

        Returns:
            Tweet text
        """
        print(f"🤖 Generating announcement for proposal #{proposal['id']}...")

        prompt = f"""Generate a tweet announcing a new Ëtrid governance proposal.

Proposal Details:
- ID: #{proposal['id']}
- Title: {proposal.get('title', 'Governance Proposal')}
- Proposer: {proposal.get('proposer', 'Unknown')[:8]}...
- Description: {proposal.get('description', 'No description')}
- Voting Ends: {proposal.get('voting_ends', 'TBD')}
- Current Votes: For: {proposal.get('votes_for', 0):,} / Against: {proposal.get('votes_against', 0):,}

Requirements:
- Voice: Governance Dev (balanced, diplomatic, process-oriented)
- Tone: Neutral, informative
- Format: Clear proposal summary with voting info
- Include 🗳️ emoji
- Keep under 250 chars
- Include link: etrid.network/proposals/{proposal['id']}
- End with call to action: "Review and vote"

Example structure:
🗳️ New Proposal #{proposal['id']}: [Title]

[1-2 sentence summary]

Voting: [Date] UTC
Current: [For] / [Against]

Review: etrid.network/proposals/{proposal['id']}
"""

        tweet = await self.generator.generate(
            prompt=prompt,
            dev="governance-dev",
            max_tokens=200
        )

        print(f"✅ Announcement generated")
        return tweet

    async def generate_result_announcement(self, proposal: Dict, passed: bool) -> str:
        """
        Generate proposal result announcement

        Args:
            proposal: Proposal object
            passed: True if passed, False if rejected

        Returns:
            Tweet text
        """
        print(f"🤖 Generating result announcement for proposal #{proposal['id']}...")

        status = "PASSED ✅" if passed else "REJECTED ❌"

        prompt = f"""Generate a tweet announcing the result of a governance proposal vote.

Proposal: #{proposal['id']} - {proposal.get('title', 'Governance Proposal')}
Result: {status}
Final Votes: For: {proposal.get('votes_for', 0):,} / Against: {proposal.get('votes_against', 0):,}

Requirements:
- Voice: Governance Dev
- Tone: {"Positive but professional" if passed else "Neutral and factual"}
- Format: Clear result statement
- Include 🗳️ emoji
- Keep under 250 chars
- Mention what happens next (execution date if passed, or archive if rejected)
"""

        tweet = await self.generator.generate(
            prompt=prompt,
            dev="governance-dev",
            max_tokens=200
        )

        return tweet

    async def post_announcement(self, tweet: str) -> Optional[str]:
        """
        Post announcement to Twitter

        Args:
            tweet: Tweet text

        Returns:
            Tweet ID or None
        """
        # Add signature
        final_tweet = tweet + "\n\n—Governance Dev"

        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would post:\n{final_tweet}\n")
            return "dry_run_id"

        try:
            tweet_id = await self.twitter.post(final_tweet)

            print(f"✅ Posted announcement! Tweet ID: {tweet_id}")
            print(f"   URL: https://twitter.com/{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}/status/{tweet_id}")

            return tweet_id

        except Exception as e:
            print(f"❌ Error posting announcement: {e}")
            return None

    async def log_to_memory(self, proposal: Dict, tweet_id: Optional[str], event_type: str):
        """Log governance activity to GLOBAL_MEMORY.md"""
        print("📝 Logging to GLOBAL_MEMORY.md...")

        memory_entry = f"""
## [{datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")}] governance-dev
**Event:** Governance proposal {event_type}
**Action:** Posted announcement for proposal #{proposal['id']}
**Status:** COMPLETED
**Priority:** {"HIGH" if event_type in ["proposed", "passed"] else "MEDIUM"}
**Tags:** #governance #proposal #twitter

**Proposal:** #{proposal['id']} - {proposal.get('title', 'N/A')}
**Tweet ID:** {tweet_id or 'N/A'}
**Votes:** For: {proposal.get('votes_for', 0):,} / Against: {proposal.get('votes_against', 0):,}

---
"""

        memory_file = Path(__file__).parent.parent.parent / "memory" / "GLOBAL_MEMORY.md"
        with open(memory_file, "a") as f:
            f.write(memory_entry)

        print("✅ Logged to GLOBAL_MEMORY.md")

    async def process_proposal(self, proposal: Dict, event_type: str = "proposed"):
        """
        Process a single governance proposal

        Args:
            proposal: Proposal object
            event_type: "proposed", "passed", "rejected"
        """
        print(f"\n{'='*60}")
        print(f"Processing Proposal #{proposal['id']}")
        print(f"Event: {event_type.upper()}")
        print(f"{'='*60}\n")

        # Generate appropriate announcement
        if event_type == "proposed":
            tweet = await self.generate_proposal_announcement(proposal)
        elif event_type in ["passed", "rejected"]:
            tweet = await self.generate_result_announcement(
                proposal,
                passed=(event_type == "passed")
            )
        else:
            print(f"⚠️  Unknown event type: {event_type}")
            return

        print(f"📝 Generated tweet:\n{tweet}\n")

        # Moderate content
        moderation = await self.moderator.moderate(tweet, strict=False)

        if not moderation['approved']:
            print(f"❌ Tweet rejected by moderation: {moderation['issues']}")
            return

        # Post announcement
        tweet_id = await self.post_announcement(tweet)

        # Log to memory
        if tweet_id:
            await self.log_to_memory(proposal, tweet_id, event_type)

        # Mark as processed
        self.processed_proposals.add(proposal['id'])

    async def poll_once(self):
        """Poll for new proposals once"""
        print("🔍 Polling for governance events...")

        # Fetch new proposals
        proposals = await self.fetch_new_proposals()

        for proposal in proposals:
            await self.process_proposal(proposal, event_type="proposed")

        # TODO: Also check for completed proposals (passed/rejected)
        # This would require tracking proposals we've announced and checking their status

    async def stream_continuous(self):
        """Monitor governance events continuously"""
        print("🌊 Starting continuous governance monitoring...")
        print("   Press Ctrl+C to stop\n")

        while True:
            try:
                await self.poll_once()

                # Wait before next check (every 5 minutes)
                print("⏰ Waiting 5 minutes before next check...\n")
                await asyncio.sleep(300)

            except KeyboardInterrupt:
                print("\n\n✋ Stopping governance monitoring...")
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
        print(f"  Governance Monitor Workflow - {mode.upper()} mode")
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
    workflow = GovernanceMonitorWorkflow(dry_run=dry_run)
    await workflow.run(mode=mode)


if __name__ == "__main__":
    asyncio.run(main())
