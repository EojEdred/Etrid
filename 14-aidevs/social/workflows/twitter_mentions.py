#!/usr/bin/env python3
"""
Twitter Mentions Workflow - Auto-respond to community questions

This workflow:
1. Monitors Twitter mentions of @EtridAI_Devs
2. Routes questions to appropriate AI Dev based on topic
3. Generates contextual response using Claude
4. Posts reply with dev signature
5. Logs interaction to GLOBAL_MEMORY.md

Routing Logic:
- "EDSC", "stablecoin", "reserve" → edsc-dev
- "stake", "validator", "consensus" → consensus-dev
- "proposal", "voting", "governance" → governance-dev
- "security", "audit" → audit-dev
- "economic", "tokenomics", "APY" → economics-dev
- "oracle", "data", "metrics" → oracle-dev
- Everything else → gizzi

Usage:
    python twitter_mentions.py --mode stream   # Continuous monitoring
    python twitter_mentions.py --mode poll     # Check once and exit
    python twitter_mentions.py --dry-run       # Test without replying
"""

import asyncio
import os
import sys
import re
from datetime import datetime, timedelta
from pathlib import Path
from typing import Optional, Dict, List

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from connectors.twitter import TwitterConnector
from content.generator import ContentGenerator
from content.moderation import ContentModerator


class TwitterMentionsWorkflow:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

        # Track last processed mention ID to avoid duplicates
        self.last_mention_id = None

        # Question routing keywords
        self.routing_keywords = {
            "edsc-dev": ["edsc", "stablecoin", "reserve", "collateral", "peg"],
            "consensus-dev": ["stake", "validator", "consensus", "block", "finality", "ppfa"],
            "governance-dev": ["proposal", "voting", "governance", "referendum", "treasury"],
            "audit-dev": ["security", "audit", "vulnerability", "safe", "risk"],
            "economics-dev": ["economic", "tokenomics", "apy", "yield", "inflation", "incentive"],
            "oracle-dev": ["oracle", "data", "metrics", "stats", "price feed"],
            "compiler-dev": ["etwasm", "compile", "wasm", "contract", "deploy"],
        }

    async def route_question(self, text: str) -> str:
        """
        Determine which AI Dev should answer based on question content

        Args:
            text: Question text

        Returns:
            Dev ID to route to (e.g., "edsc-dev", "gizzi")
        """
        text_lower = text.lower()

        # Check each dev's keywords
        for dev, keywords in self.routing_keywords.items():
            for keyword in keywords:
                if keyword in text_lower:
                    return dev

        # Default: route to Gizzi for general questions
        return "gizzi"

    async def is_spam(self, text: str, author_id: str) -> bool:
        """
        Detect spam/inappropriate mentions

        Returns True if spam, False if legitimate
        """
        text_lower = text.lower()

        # Spam indicators
        spam_patterns = [
            r"buy.*now",
            r"to.*the.*moon",
            r"100x",
            r"guaranteed.*returns",
            r"investment.*opportunity",
            r"follow.*back",
            r"check.*out.*my.*profile",
        ]

        for pattern in spam_patterns:
            if re.search(pattern, text_lower):
                print(f"🚫 Spam detected: matched pattern '{pattern}'")
                return True

        # Check for blocked terms
        if await self.moderator.contains_blocked_terms(text):
            print(f"🚫 Blocked terms detected in mention")
            return True

        # TODO: Check author reputation (follower count, account age, etc.)
        # For now, accept all non-spam-pattern mentions

        return False

    async def should_respond(self, mention: Dict) -> bool:
        """
        Determine if we should respond to this mention

        Criteria:
        - Not spam
        - Contains a question or statement worth responding to
        - Not a duplicate of recent response
        - Not too long ago (within 24 hours)
        """
        # Check if spam
        if await self.is_spam(mention['text'], mention['author_id']):
            return False

        # Check if too old (only respond to recent mentions)
        created_at = mention['created_at']
        if isinstance(created_at, str):
            # Parse if string
            created_at = datetime.fromisoformat(created_at.replace('Z', '+00:00'))

        age = datetime.utcnow().replace(tzinfo=created_at.tzinfo) - created_at
        if age > timedelta(hours=24):
            print(f"⏰ Mention too old ({age}), skipping")
            return False

        # Check if it's a question or statement (has substance)
        text = mention['text'].replace(f"@{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}", "").strip()

        if len(text) < 10:
            print(f"📏 Mention too short, likely not a real question")
            return False

        # TODO: Check if we've already responded to this user recently
        # (avoid spamming same user with multiple responses)

        return True

    async def generate_response(self, mention: Dict, dev: str) -> str:
        """
        Generate contextual response to mention

        Args:
            mention: Mention object from Twitter
            dev: AI Dev to respond as

        Returns:
            Response text
        """
        question = mention['text'].replace(f"@{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}", "").strip()

        # Build context for Claude
        context = f"""Question from Twitter user:
"{question}"

Generate a helpful response. Guidelines:
- Answer the question directly and accurately
- Be friendly and professional
- Keep under 250 characters (leave room for signature)
- If unsure, say so and point to docs: etrid.network/docs
- Don't make claims you're not certain about
- Use appropriate emoji sparingly (1-2 max)
"""

        response = await self.generator.generate_reply(
            question=question,
            context=None,
            dev=dev
        )

        return response

    async def post_response(self, mention_id: str, response: str, dev: str, username: str) -> Optional[str]:
        """
        Post reply to Twitter

        Args:
            mention_id: Tweet ID being replied to
            response: Response text
            dev: AI Dev name for signature
            username: User to mention

        Returns:
            Tweet ID of reply, or None if failed
        """
        # Add signature
        persona = self.generator.get_persona_info(dev)
        final_response = f"{response}\n\n—{persona['name']}"

        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would reply to @{username}:")
            print(f"   {final_response}\n")
            return "dry_run_id"

        try:
            reply_id = await self.twitter.reply(
                to_tweet_id=mention_id,
                text=final_response,
                username=username
            )

            print(f"✅ Posted reply! Tweet ID: {reply_id}")
            print(f"   URL: https://twitter.com/{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}/status/{reply_id}")

            return reply_id

        except Exception as e:
            print(f"❌ Error posting reply: {e}")
            return None

    async def log_to_memory(self, mention: Dict, dev: str, response: str, reply_id: Optional[str]):
        """Log response to GLOBAL_MEMORY.md"""
        print("📝 Logging to GLOBAL_MEMORY.md...")

        memory_entry = f"""
## [{datetime.utcnow().strftime("%Y-%m-%d %H:%M UTC")}] {dev}
**Event:** Responded to Twitter mention
**Action:** Answered community question
**Status:** COMPLETED
**Priority:** LOW
**Tags:** #twitter #community #qa

**Question:** {mention['text'][:200]}
**Response:** {response[:200]}
**Reply ID:** {reply_id or 'N/A'}
**Author:** {mention['author_id']}

---
"""

        memory_file = Path(__file__).parent.parent.parent / "memory" / "GLOBAL_MEMORY.md"
        with open(memory_file, "a") as f:
            f.write(memory_entry)

        print("✅ Logged to GLOBAL_MEMORY.md")

    async def process_mention(self, mention: Dict):
        """Process a single mention"""
        print(f"\n{'='*60}")
        print(f"Processing mention from @{mention.get('author_id', 'unknown')}")
        print(f"Text: {mention['text'][:100]}...")
        print(f"{'='*60}\n")

        # Step 1: Decide if we should respond
        if not await self.should_respond(mention):
            print("⏭️  Skipping mention (spam or not worth responding)")
            return

        # Step 2: Route to appropriate dev
        dev = await self.route_question(mention['text'])
        print(f"📍 Routing to: {dev}")

        # Step 3: Generate response
        response = await self.generate_response(mention, dev)
        print(f"🤖 Generated response:\n{response}\n")

        # Step 4: Moderate response
        moderation = await self.moderator.moderate(response, strict=False)

        if not moderation['approved']:
            print(f"❌ Response rejected by moderation: {moderation['issues']}")
            return

        if moderation['needs_human_review']:
            print(f"⚠️  Response needs human review: {moderation['warnings']}")
            # TODO: Send to approval queue
            return

        # Step 5: Post reply
        # Extract username from mention (Twitter API should provide this)
        username = mention.get('username', 'user')

        reply_id = await self.post_response(
            mention_id=mention['id'],
            response=response,
            dev=dev,
            username=username
        )

        # Step 6: Log to memory
        if reply_id:
            await self.log_to_memory(mention, dev, response, reply_id)

    async def poll_mentions_once(self):
        """Fetch and process mentions once (polling mode)"""
        print("🔍 Fetching recent mentions...")

        mentions = await self.twitter.get_mentions(since_id=self.last_mention_id)

        if not mentions:
            print("   No new mentions")
            return

        print(f"   Found {len(mentions)} new mention(s)")

        for mention in mentions:
            await self.process_mention(mention)

            # Update last processed ID
            self.last_mention_id = mention['id']

    async def stream_mentions_continuous(self):
        """Monitor mentions continuously (streaming mode)"""
        print("🌊 Starting continuous mention monitoring...")
        print("   Press Ctrl+C to stop\n")

        while True:
            try:
                await self.poll_mentions_once()

                # Wait before next check (every 60 seconds)
                await asyncio.sleep(60)

            except KeyboardInterrupt:
                print("\n\n✋ Stopping mention monitoring...")
                break

            except Exception as e:
                print(f"❌ Error in monitoring loop: {e}")
                await asyncio.sleep(60)  # Wait before retrying

    async def run(self, mode: str = "poll"):
        """
        Execute workflow

        Args:
            mode: "poll" (check once) or "stream" (continuous monitoring)
        """
        print("\n" + "="*60)
        print(f"  Twitter Mentions Workflow - {mode.upper()} mode")
        print("="*60 + "\n")

        try:
            if mode == "stream":
                await self.stream_mentions_continuous()
            else:
                await self.poll_mentions_once()

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
    workflow = TwitterMentionsWorkflow(dry_run=dry_run)
    await workflow.run(mode=mode)


if __name__ == "__main__":
    asyncio.run(main())
