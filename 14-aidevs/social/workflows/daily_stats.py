#!/usr/bin/env python3
"""
Daily Stats Workflow - Auto-post daily blockchain metrics to Twitter

This workflow:
1. Fetches 24h blockchain stats from FlareChain
2. Routes to Oracle Dev for data interpretation
3. Generates tweet using Claude (Oracle Dev voice)
4. Posts to Twitter with auto-signature
5. Logs activity to GLOBAL_MEMORY.md

Usage:
    python daily_stats.py --dry-run  # Test without posting
    python daily_stats.py             # Post to Twitter

Schedule:
    Runs daily at 12:00 UTC via cron
"""

import asyncio
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from connectors.blockchain import BlockchainMonitor
from connectors.twitter import TwitterConnector
from content.generator import ContentGenerator
from content.moderation import ContentModerator

class DailyStatsWorkflow:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.blockchain = BlockchainMonitor()
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

    async def fetch_24h_stats(self):
        """Fetch blockchain stats for last 24 hours"""
        print("📊 Fetching 24h blockchain stats...")

        # Get current block number
        current_block = await self.blockchain.get_block_number()

        # Calculate blocks in last 24h (assuming 6s block time)
        blocks_per_day = 24 * 60 * 60 // 6  # 14,400 blocks
        start_block = current_block - blocks_per_day

        # Fetch stats
        stats = {
            "date": datetime.utcnow().strftime("%b %d"),
            "blocks_produced": blocks_per_day,
            "avg_block_time": await self.blockchain.get_avg_block_time(start_block, current_block),
            "active_validators": await self.blockchain.get_active_validator_count(),
            "total_staked": await self.blockchain.get_total_staked(),
            "reserve_ratio": await self.blockchain.get_reserve_ratio("EDSC"),
            "network_uptime": await self.blockchain.get_uptime_percentage(blocks_per_day),
        }

        # Calculate change from previous day
        stats["blocks_change"] = await self.calculate_change("blocks", blocks_per_day)

        print(f"✅ Stats fetched: {stats}")
        return stats

    async def calculate_change(self, metric, current_value):
        """Calculate percentage change from previous day"""
        # In production, fetch previous day's value from database/cache
        # For now, return mock data
        previous_value = current_value * 0.998  # Mock: slight increase
        change = ((current_value - previous_value) / previous_value) * 100
        return round(change, 2)

    async def generate_tweet(self, stats):
        """Generate tweet content using Claude (Oracle Dev voice)"""
        print("🤖 Generating tweet with Claude...")

        prompt = f"""Generate a daily blockchain stats tweet for Ëtrid.

Stats:
- Date: {stats['date']}
- Blocks: {stats['blocks_produced']:,} ({stats['blocks_change']:+.1f}%)
- Avg Block Time: {stats['avg_block_time']:.2f}s
- Active Validators: {stats['active_validators']}
- Total Staked: {stats['total_staked'] / 1_000_000:.1f}M ETR
- Reserve Ratio: {stats['reserve_ratio']:.2f} ({'healthy' if stats['reserve_ratio'] > 1.5 else 'warning'})
- Network Uptime: {stats['network_uptime']:.2f}%

Voice: Oracle Dev (data-driven, precise, technical)
Tone: Informative, professional
Format: Clear metrics with emojis
Max Length: 250 characters (leave room for signature)
Include: #ËtridDevLog hashtag
"""

        tweet = await self.generator.generate(
            prompt=prompt,
            dev="oracle-dev",
            max_tokens=150
        )

        print(f"✅ Tweet generated:\n{tweet}\n")
        return tweet

    async def verify_accuracy(self, tweet, stats):
        """Verify tweet accuracy (Audit Dev check)"""
        print("🔍 Verifying accuracy...")

        # Check that all numbers in tweet match stats
        checks = [
            str(stats['blocks_produced']) in tweet or str(stats['blocks_produced'] // 1000) in tweet,
            f"{stats['avg_block_time']:.2f}" in tweet or f"{stats['avg_block_time']:.1f}" in tweet,
            str(stats['active_validators']) in tweet,
            str(round(stats['total_staked'] / 1_000_000, 1)) in tweet,
        ]

        if all(checks):
            print("✅ Accuracy verified")
            return True
        else:
            print("❌ Accuracy check failed - numbers don't match")
            return False

    async def moderate_content(self, tweet):
        """Check content safety and appropriateness"""
        print("🛡️ Moderating content...")

        # Check length
        if len(tweet) > 250:
            print(f"⚠️  Tweet too long: {len(tweet)} chars")
            return False

        # Check for blocked terms
        if await self.moderator.contains_blocked_terms(tweet):
            print("⚠️  Tweet contains blocked terms")
            return False

        # Check tone (should be professional)
        tone_score = await self.moderator.analyze_tone(tweet)
        if tone_score['professionalism'] < 0.7:
            print(f"⚠️  Tweet tone not professional enough: {tone_score}")
            return False

        print("✅ Content moderation passed")
        return True

    async def post_to_twitter(self, tweet):
        """Post tweet with Oracle Dev signature"""
        # Add signature
        final_tweet = tweet + "\n\n—Oracle Dev"

        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would post:\n{final_tweet}\n")
            return "dry_run_id"
        else:
            print("📤 Posting to Twitter...")
            tweet_id = await self.twitter.post(final_tweet)
            print(f"✅ Posted! Tweet ID: {tweet_id}")
            print(f"   URL: https://twitter.com/EtridAI_Devs/status/{tweet_id}")
            return tweet_id

    async def log_to_memory(self, stats, tweet, tweet_id):
        """Log activity to GLOBAL_MEMORY.md"""
        print("📝 Logging to GLOBAL_MEMORY.md...")

        memory_entry = f"""
## [{datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")}] oracle-dev
**Event:** Posted daily blockchain stats to Twitter
**Action:** Generated and posted 24h metrics summary
**Status:** COMPLETED
**Priority:** LOW
**Tags:** #stats #twitter #automation

**Details:**
- Blocks: {stats['blocks_produced']:,} ({stats['blocks_change']:+.1f}%)
- Avg Block Time: {stats['avg_block_time']:.2f}s
- Reserve Ratio: {stats['reserve_ratio']:.2f}
- Network Uptime: {stats['network_uptime']:.2f}%
- Tweet ID: {tweet_id}

**Tweet Text:**
{tweet}

---
"""

        # Append to GLOBAL_MEMORY.md
        memory_file = Path(__file__).parent.parent.parent / "memory" / "GLOBAL_MEMORY.md"
        with open(memory_file, "a") as f:
            f.write(memory_entry)

        print("✅ Logged to GLOBAL_MEMORY.md")

    async def run(self):
        """Execute complete workflow"""
        print("\n" + "="*60)
        print("  Daily Stats Workflow - Oracle Dev")
        print("="*60 + "\n")

        try:
            # Step 1: Fetch stats
            stats = await self.fetch_24h_stats()

            # Step 2: Generate tweet
            tweet = await self.generate_tweet(stats)

            # Step 3: Verify accuracy
            if not await self.verify_accuracy(tweet, stats):
                print("\n❌ Workflow failed: Accuracy check")
                return False

            # Step 4: Moderate content
            if not await self.moderate_content(tweet):
                print("\n❌ Workflow failed: Content moderation")
                return False

            # Step 5: Post to Twitter
            tweet_id = await self.post_to_twitter(tweet)

            # Step 6: Log to memory
            await self.log_to_memory(stats, tweet, tweet_id)

            print("\n" + "="*60)
            print("  ✅ Daily Stats Workflow Complete!")
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
    workflow = DailyStatsWorkflow(dry_run=dry_run)
    success = await workflow.run()

    sys.exit(0 if success else 1)

if __name__ == "__main__":
    asyncio.run(main())
