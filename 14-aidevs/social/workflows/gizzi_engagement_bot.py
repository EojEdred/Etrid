#!/usr/bin/env python3
"""
Gizzi Engagement Bot - Actively engage with retweets, mentions, and replies

This workflow:
1. Monitors Twitter for mentions, retweets, replies to Etrid content
2. Intelligently responds to questions and comments
3. Thanks supporters and engagers
4. Identifies influencers and high-value interactions
5. Maintains conversation threads
6. Tracks engagement metrics

Usage:
    python gizzi_engagement_bot.py --mode stream    # Continuous monitoring
    python gizzi_engagement_bot.py --mode poll      # Check once
    python gizzi_engagement_bot.py --dry-run        # Test without posting
    python gizzi_engagement_bot.py --aggressive     # More active engagement
"""

import asyncio
import os
import sys
import json
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from connectors.twitter import TwitterConnector
from content.generator import ContentGenerator
from content.moderation import ContentModerator


class GizziEngagementBot:
    def __init__(self, dry_run=False, aggressive=False):
        self.dry_run = dry_run
        self.aggressive = aggressive  # More active engagement mode
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

        self.dev = "gizzi"

        # Engagement tracking
        self.data_dir = Path(__file__).parent.parent / "data" / "engagement"
        self.data_dir.mkdir(parents=True, exist_ok=True)

        # Track processed interactions to avoid duplicates
        self.processed_ids = self.load_processed_ids()

        # Engagement limits (to avoid spam)
        self.hourly_limit = 20 if not aggressive else 40
        self.hourly_count = 0
        self.last_reset = datetime.now()

    def load_processed_ids(self) -> set:
        """Load IDs of already-processed tweets"""
        processed_file = self.data_dir / "processed_ids.json"

        if processed_file.exists():
            with open(processed_file, "r") as f:
                data = json.load(f)
                return set(data.get("ids", []))

        return set()

    def save_processed_id(self, tweet_id: str):
        """Save processed tweet ID"""
        self.processed_ids.add(tweet_id)

        processed_file = self.data_dir / "processed_ids.json"

        # Keep only last 1000 IDs to prevent file from growing too large
        recent_ids = list(self.processed_ids)[-1000:]

        with open(processed_file, "w") as f:
            json.dump({"ids": recent_ids, "last_updated": datetime.now().isoformat()}, f)

    async def check_rate_limit(self) -> bool:
        """Check if we're within rate limits"""
        # Reset hourly counter
        if (datetime.now() - self.last_reset).total_seconds() > 3600:
            self.hourly_count = 0
            self.last_reset = datetime.now()

        if self.hourly_count >= self.hourly_limit:
            print(f"⚠️  Hourly limit reached ({self.hourly_limit}), skipping engagement")
            return False

        return True

    async def get_mentions(self) -> List[Dict]:
        """Get recent mentions"""
        print("🔍 Checking mentions...")

        mentions = await self.twitter.get_mentions(since_id=None)

        # Filter out already processed
        new_mentions = [m for m in mentions if m["id"] not in self.processed_ids]

        print(f"   Found {len(new_mentions)} new mention(s)")

        return new_mentions

    async def get_retweets_of_our_tweets(self) -> List[Dict]:
        """Get recent retweets of our content"""
        print("🔁 Checking retweets...")

        # This would require getting our recent tweets and checking their retweets
        # Simplified version: get our timeline and check engagement

        retweets = []

        # Note: This is a placeholder - actual implementation would use Twitter API
        # to fetch retweets of specific tweet IDs

        print(f"   Found {len(retweets)} new retweet(s)")

        return retweets

    async def get_replies_to_our_tweets(self) -> List[Dict]:
        """Get replies to our tweets"""
        print("💬 Checking replies...")

        # Placeholder - would fetch replies to recent tweets
        replies = []

        print(f"   Found {len(replies)} new repl(y/ies)")

        return replies

    async def classify_interaction(self, tweet: Dict) -> str:
        """
        Classify type of interaction

        Returns:
            "question", "praise", "criticism", "discussion", "spam"
        """
        text = tweet["text"].lower()

        # Check for questions
        question_indicators = ["?", "how", "what", "when", "where", "why", "who", "can you", "could you"]
        if any(ind in text for ind in question_indicators):
            return "question"

        # Check for praise
        praise_indicators = ["great", "amazing", "awesome", "excellent", "love", "thank", "brilliant", "genius"]
        if any(ind in text for ind in praise_indicators):
            return "praise"

        # Check for criticism
        criticism_indicators = ["bad", "terrible", "awful", "scam", "fake", "disappointed", "worse"]
        if any(ind in text for ind in criticism_indicators):
            return "criticism"

        # Check for spam
        spam_indicators = ["buy now", "investment opportunity", "guarantee", "follow back"]
        if any(ind in text for ind in spam_indicators):
            return "spam"

        # Default: discussion
        return "discussion"

    async def should_engage(self, tweet: Dict, interaction_type: str) -> bool:
        """
        Decide if we should engage with this tweet

        Args:
            tweet: Tweet data
            interaction_type: Type of interaction

        Returns:
            True if should engage
        """
        # Never engage with spam
        if interaction_type == "spam":
            return False

        # Always engage with questions (high value)
        if interaction_type == "question":
            return True

        # Always thank praise (builds community)
        if interaction_type == "praise":
            return True

        # Handle criticism thoughtfully
        if interaction_type == "criticism":
            # Only engage if constructive (contains specific feedback)
            if len(tweet["text"]) > 50:  # Substantial criticism
                return True
            return False

        # For discussions, engage based on mode
        if interaction_type == "discussion":
            if self.aggressive:
                return True
            else:
                # Only engage with users who have some followers (avoid bots)
                # Placeholder: would check user follower count
                return True

        return False

    async def generate_response(self, tweet: Dict, interaction_type: str) -> str:
        """
        Generate contextual response based on interaction type

        Args:
            tweet: Tweet data
            interaction_type: Type of interaction

        Returns:
            Response text
        """
        print(f"🤖 Generating response for {interaction_type}...")

        # Build context for Claude based on interaction type
        if interaction_type == "question":
            context = f"""
You are Gizzi, helpful AI developer for Etrid blockchain. Someone asked a question on Twitter.

Question: "{tweet['text']}"

Generate a helpful, friendly response that:
1. Answers their question accurately (or admits if you don't know)
2. Provides value and encourages learning more
3. Points to resources if applicable (etrid.network/docs)
4. Is concise (under 280 characters including signature)
5. Warm and welcoming tone

If you don't know the answer, say so and offer to find out.
"""

        elif interaction_type == "praise":
            context = f"""
You are Gizzi, grateful AI developer for Etrid blockchain. Someone praised Etrid on Twitter.

Their message: "{tweet['text']}"

Generate a thankful response that:
1. Genuinely thanks them for support
2. Keeps it brief and authentic (not over-the-top)
3. Optionally invites them to join community/discord
4. Under 280 characters
5. Warm and humble tone

Be genuine, not corporate.
"""

        elif interaction_type == "criticism":
            context = f"""
You are Gizzi, thoughtful AI developer for Etrid blockchain. Someone has criticism on Twitter.

Their criticism: "{tweet['text']}"

Generate a constructive response that:
1. Acknowledges their concern respectfully
2. Provides clarification if there's a misunderstanding
3. Thanks them for feedback if it's constructive
4. Doesn't get defensive or argumentative
5. Under 280 characters
6. Professional and respectful tone

Handle criticism maturely and productively.
"""

        else:  # discussion
            context = f"""
You are Gizzi, engaging AI developer for Etrid blockchain. Someone mentioned Etrid in a discussion.

Their tweet: "{tweet['text']}"

Generate an engaging response that:
1. Adds value to the conversation
2. Relates to Etrid naturally (don't force it)
3. Encourages further discussion
4. Under 280 characters
5. Friendly and conversational tone

Be a good conversation partner, not a marketer.
"""

        # Generate response
        response = await self.generator.generate_reply(
            question=tweet["text"],
            context=context,
            dev=self.dev
        )

        print(f"✅ Generated response ({len(response)} chars): {response[:60]}...")

        return response

    async def post_response(self, tweet: Dict, response: str) -> Optional[str]:
        """Post response to tweet"""
        # Add Gizzi signature
        persona = self.generator.get_persona_info(self.dev)
        final_response = f"{response}\n\n—{persona['name']}"

        # Moderate
        moderation = await self.moderator.moderate(final_response, strict=False)

        if not moderation["approved"]:
            print(f"❌ Response rejected: {moderation['issues']}")
            return None

        # Post
        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would reply to @{tweet.get('username', 'user')}:")
            print(f"   {final_response}\n")
            return "dry_run_reply_id"

        try:
            reply_id = await self.twitter.reply(
                to_tweet_id=tweet["id"],
                text=final_response,
                username=tweet.get("username", "user")
            )

            print(f"✅ Posted reply: {reply_id}")

            # Update counters
            self.hourly_count += 1

            return reply_id

        except Exception as e:
            print(f"❌ Error posting reply: {e}")
            return None

    async def process_interaction(self, tweet: Dict):
        """Process a single interaction"""
        print(f"\n{'='*60}")
        print(f"Processing interaction from @{tweet.get('username', 'user')}")
        print(f"Text: {tweet['text'][:80]}...")
        print(f"{'='*60}\n")

        # Check rate limits
        if not await self.check_rate_limit():
            return

        # Classify interaction
        interaction_type = await self.classify_interaction(tweet)
        print(f"📊 Classified as: {interaction_type}")

        # Decide if should engage
        should_engage = await self.should_engage(tweet, interaction_type)

        if not should_engage:
            print(f"⏭️  Skipping engagement")
            self.save_processed_id(tweet["id"])
            return

        # Generate response
        response = await self.generate_response(tweet, interaction_type)

        # Post response
        reply_id = await self.post_response(tweet, response)

        # Mark as processed
        self.save_processed_id(tweet["id"])

        # Log engagement
        await self.log_engagement(tweet, interaction_type, reply_id)

    async def log_engagement(self, tweet: Dict, interaction_type: str, reply_id: Optional[str]):
        """Log engagement for analytics"""
        log_file = self.data_dir / f"engagement_{datetime.now().strftime('%Y%m')}.jsonl"

        log_entry = {
            "timestamp": datetime.now().isoformat(),
            "tweet_id": tweet["id"],
            "username": tweet.get("username", "unknown"),
            "interaction_type": interaction_type,
            "replied": reply_id is not None,
            "reply_id": reply_id,
        }

        with open(log_file, "a") as f:
            f.write(json.dumps(log_entry) + "\n")

    async def poll_once(self):
        """Check for interactions once"""
        print("\n" + "="*60)
        print("  POLLING FOR INTERACTIONS")
        print("="*60 + "\n")

        # Get all interaction types
        mentions = await self.get_mentions()
        retweets = await self.get_retweets_of_our_tweets()
        replies = await self.get_replies_to_our_tweets()

        all_interactions = mentions + retweets + replies

        print(f"\n📊 Total interactions: {len(all_interactions)}")

        # Process each
        for interaction in all_interactions:
            await self.process_interaction(interaction)

            # Small delay between processing
            await asyncio.sleep(2)

    async def stream_continuous(self):
        """Monitor continuously"""
        print("\n" + "="*60)
        print("  CONTINUOUS ENGAGEMENT MONITORING")
        print("="*60)
        print(f"  Mode: {'AGGRESSIVE' if self.aggressive else 'NORMAL'}")
        print(f"  Hourly limit: {self.hourly_limit}")
        print("="*60 + "\n")

        print("Press Ctrl+C to stop\n")

        while True:
            try:
                await self.poll_once()

                # Wait before next check (60 seconds)
                print(f"\n⏳ Waiting 60s before next check... (Engaged: {self.hourly_count}/{self.hourly_limit} this hour)")
                await asyncio.sleep(60)

            except KeyboardInterrupt:
                print("\n\n✋ Stopping engagement monitoring...")
                break

            except Exception as e:
                print(f"❌ Error in monitoring loop: {e}")
                await asyncio.sleep(60)

    async def run(self, mode: str = "poll"):
        """Execute engagement bot"""
        print("\n" + "="*60)
        print(f"  GIZZI ENGAGEMENT BOT - {mode.upper()} MODE")
        print("="*60 + "\n")

        try:
            if mode == "stream":
                await self.stream_continuous()
            else:
                await self.poll_once()

            print("\n" + "="*60)
            print("  ✅ ENGAGEMENT CHECK COMPLETE!")
            print("="*60 + "\n")

        except Exception as e:
            print(f"\n❌ Engagement bot error: {e}")
            import traceback
            traceback.print_exc()


async def main():
    # Parse arguments
    dry_run = "--dry-run" in sys.argv
    aggressive = "--aggressive" in sys.argv
    mode = "stream" if "--mode stream" in " ".join(sys.argv) or "--stream" in sys.argv else "poll"

    # Run bot
    bot = GizziEngagementBot(dry_run=dry_run, aggressive=aggressive)
    await bot.run(mode=mode)


if __name__ == "__main__":
    asyncio.run(main())
