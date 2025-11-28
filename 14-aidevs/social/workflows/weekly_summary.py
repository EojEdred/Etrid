#!/usr/bin/env python3
"""
Weekly Summary Workflow - Post week-in-review from Gizzi

This workflow:
1. Reads GLOBAL_MEMORY.md for the past week's activities
2. Aggregates activities by AI Dev
3. Generates cohesive weekly summary using Claude (Gizzi voice)
4. Posts as Twitter thread (3-5 tweets)
5. Logs to GLOBAL_MEMORY.md

Runs: Every Sunday at 18:00 UTC

Usage:
    python weekly_summary.py --dry-run  # Test without posting
    python weekly_summary.py             # Post to Twitter
"""

import asyncio
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path
from collections import defaultdict
import re

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from connectors.blockchain import BlockchainMonitor
from connectors.twitter import TwitterConnector
from content.generator import ContentGenerator
from content.moderation import ContentModerator


class WeeklySummaryWorkflow:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.blockchain = BlockchainMonitor()
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

    async def parse_global_memory(self, days: int = 7) -> dict:
        """
        Parse GLOBAL_MEMORY.md for activities in the last N days

        Returns:
            {
                "oracle-dev": [list of activities],
                "audit-dev": [list of activities],
                ...
            }
        """
        print(f"📖 Reading GLOBAL_MEMORY.md (last {days} days)...")

        memory_file = Path(__file__).parent.parent.parent / "memory" / "GLOBAL_MEMORY.md"

        if not memory_file.exists():
            print(f"⚠️  GLOBAL_MEMORY.md not found, creating...")
            memory_file.parent.mkdir(parents=True, exist_ok=True)
            memory_file.touch()
            return {}

        # Read file
        with open(memory_file, 'r') as f:
            content = f.read()

        # Parse entries
        cutoff_date = datetime.utcnow() - timedelta(days=days)
        activities_by_dev = defaultdict(list)

        # Split by entries (marked by ## [...] dev_id)
        entry_pattern = r'## \[([\d\-\s:UTC]+)\] ([\w\-]+)'
        entries = re.split(entry_pattern, content)

        # Parse each entry (entries come in groups of 3: date, dev, content)
        for i in range(1, len(entries), 3):
            if i + 2 >= len(entries):
                break

            date_str = entries[i].strip()
            dev_id = entries[i + 1].strip()
            entry_content = entries[i + 2].strip()

            # Parse date
            try:
                # Format: 2025-10-24 14:45 UTC
                entry_date = datetime.strptime(date_str, "%Y-%m-%d %H:%M UTC")

                # Check if within timeframe
                if entry_date >= cutoff_date:
                    # Extract event and action
                    event_match = re.search(r'\*\*Event:\*\* (.+)', entry_content)
                    action_match = re.search(r'\*\*Action:\*\* (.+)', entry_content)

                    event = event_match.group(1) if event_match else "Activity"
                    action = action_match.group(1) if action_match else ""

                    activities_by_dev[dev_id].append({
                        "date": entry_date,
                        "event": event,
                        "action": action,
                        "content": entry_content[:200]  # First 200 chars
                    })

            except ValueError:
                # Skip entries with invalid dates
                continue

        print(f"✅ Parsed {sum(len(acts) for acts in activities_by_dev.values())} activities")
        for dev, acts in activities_by_dev.items():
            print(f"   {dev}: {len(acts)} activities")

        return dict(activities_by_dev)

    async def get_weekly_stats(self) -> dict:
        """Fetch blockchain stats for the week"""
        print("📊 Fetching weekly blockchain stats...")

        current_block = await self.blockchain.get_block_number()
        blocks_per_week = 7 * 24 * 60 * 60 // 6  # ~100,800 blocks

        stats = {
            "blocks_produced": blocks_per_week,
            "avg_block_time": await self.blockchain.get_avg_block_time(
                current_block - blocks_per_week,
                current_block
            ),
            "active_validators": await self.blockchain.get_active_validator_count(),
            "network_uptime": await self.blockchain.get_uptime_percentage(blocks_per_week),
            "reserve_ratio": await self.blockchain.get_reserve_ratio("EDSC"),
        }

        print(f"✅ Stats fetched: {stats}")
        return stats

    async def generate_summary_thread(self, activities: dict, stats: dict) -> list:
        """
        Generate weekly summary thread using Claude (Gizzi voice)

        Returns:
            List of tweet texts (3-5 tweets)
        """
        print("🤖 Generating weekly summary thread with Claude...")

        # Build activity summary for prompt
        activity_summary = []
        for dev, acts in activities.items():
            if acts:
                activity_summary.append(f"{dev}: {len(acts)} activities")
                # Include 1-2 notable events
                for act in acts[:2]:
                    activity_summary.append(f"  - {act['event']}")

        activity_text = "\n".join(activity_summary) if activity_summary else "Limited activity this week"

        prompt = f"""Generate a 4-tweet weekly summary thread for Ëtrid.

This week's AI Dev activities:
{activity_text}

Blockchain stats for the week:
- Blocks: {stats['blocks_produced']:,}
- Avg Block Time: {stats['avg_block_time']:.2f}s
- Validators: {stats['active_validators']}
- Uptime: {stats['network_uptime']:.2f}%
- EDSC Reserve: {stats['reserve_ratio']:.2f}

Requirements:
- Voice: Gizzi (warm, reflective, big-picture)
- Tone: Thoughtful, cohesive, inspiring
- Format: 4 tweets (intro, highlights, stats, conclusion)
- Each tweet under 250 chars
- Start with "🌟 Week in Review" or similar
- End with forward-looking statement

Tweet 1: Opening + theme
Tweet 2: Dev activity highlights
Tweet 3: Network stats
Tweet 4: Reflection + next week
"""

        thread = await self.generator.generate_thread(
            topic=prompt,
            dev="gizzi",
            num_tweets=4
        )

        print(f"✅ Thread generated ({len(thread)} tweets):\n")
        for i, tweet in enumerate(thread, 1):
            print(f"   Tweet {i}: {tweet[:80]}...")

        return thread

    async def post_thread_to_twitter(self, tweets: list) -> list:
        """
        Post thread to Twitter

        Args:
            tweets: List of tweet texts

        Returns:
            List of tweet IDs
        """
        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would post thread:\n")
            for i, tweet in enumerate(tweets, 1):
                print(f"Tweet {i}:")
                print(f"{tweet}")
                print(f"—Gizzi\n")
            return ["dry_run_id"] * len(tweets)

        print("📤 Posting thread to Twitter...")

        # Add signature to each tweet
        tweets_with_sig = [f"{tweet}\n\n—Gizzi" for tweet in tweets]

        try:
            tweet_ids = await self.twitter.post_thread(tweets_with_sig)

            print(f"✅ Thread posted! Tweet IDs: {tweet_ids}")
            print(f"   URL: https://twitter.com/{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}/status/{tweet_ids[0]}")

            # Pin the thread (first tweet)
            await self.twitter.pin_tweet(tweet_ids[0])
            print(f"📌 Pinned thread to profile")

            return tweet_ids

        except Exception as e:
            print(f"❌ Error posting thread: {e}")
            return []

    async def log_to_memory(self, activities: dict, stats: dict, tweet_ids: list):
        """Log weekly summary to GLOBAL_MEMORY.md"""
        print("📝 Logging to GLOBAL_MEMORY.md...")

        memory_entry = f"""
## [{datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")}] gizzi
**Event:** Posted weekly summary to Twitter
**Action:** Generated and posted 4-tweet thread summarizing week's activities
**Status:** COMPLETED
**Priority:** MEDIUM
**Tags:** #weekly-summary #twitter #gizzi

**Activities This Week:**
{chr(10).join([f"- {dev}: {len(acts)} activities" for dev, acts in activities.items()])}

**Network Stats:**
- Blocks: {stats['blocks_produced']:,}
- Uptime: {stats['network_uptime']:.2f}%
- Reserve Ratio: {stats['reserve_ratio']:.2f}

**Thread IDs:** {', '.join(tweet_ids)}

---
"""

        memory_file = Path(__file__).parent.parent.parent / "memory" / "GLOBAL_MEMORY.md"
        with open(memory_file, "a") as f:
            f.write(memory_entry)

        print("✅ Logged to GLOBAL_MEMORY.md")

    async def run(self):
        """Execute complete workflow"""
        print("\n" + "="*60)
        print("  Weekly Summary Workflow - Gizzi")
        print("="*60 + "\n")

        try:
            # Step 1: Parse GLOBAL_MEMORY.md
            activities = await self.parse_global_memory(days=7)

            # Step 2: Fetch weekly blockchain stats
            stats = await self.get_weekly_stats()

            # Step 3: Generate summary thread
            thread = await self.generate_summary_thread(activities, stats)

            # Step 4: Moderate each tweet
            all_approved = True
            for i, tweet in enumerate(thread, 1):
                moderation = await self.moderator.moderate(tweet, strict=False)

                if not moderation['approved']:
                    print(f"❌ Tweet {i} rejected by moderation: {moderation['issues']}")
                    all_approved = False
                    break

            if not all_approved:
                print("\n❌ Workflow failed: Content moderation")
                return False

            # Step 5: Post thread
            tweet_ids = await self.post_thread_to_twitter(thread)

            if not tweet_ids:
                print("\n❌ Workflow failed: Failed to post thread")
                return False

            # Step 6: Log to memory
            await self.log_to_memory(activities, stats, tweet_ids)

            print("\n" + "="*60)
            print("  ✅ Weekly Summary Workflow Complete!")
            print("="*60 + "\n")

            return True

        except Exception as e:
            print(f"\n❌ Workflow error: {e}")
            import traceback
            traceback.print_exc()
            return False


async def main():
    # Check for dry-run flag
    dry_run = "--dry-run" in sys.argv

    # Run workflow
    workflow = WeeklySummaryWorkflow(dry_run=dry_run)
    success = await workflow.run()

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    asyncio.run(main())
