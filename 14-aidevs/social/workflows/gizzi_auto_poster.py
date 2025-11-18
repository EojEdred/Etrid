#!/usr/bin/env python3
"""
Gizzi Auto Poster - Automatically generate and post content based on aggregated data

This workflow:
1. Loads aggregated content from content aggregator
2. Selects content opportunities based on priority
3. Generates tweets/threads using Claude
4. Posts to X (Twitter) at optimal times
5. Tracks posting history to avoid repetition

Usage:
    python gizzi_auto_poster.py                    # Post based on latest aggregated data
    python gizzi_auto_poster.py --dry-run          # Test without posting
    python gizzi_auto_poster.py --force-topic "defi" # Force specific topic
    python gizzi_auto_poster.py --thread           # Generate thread instead of single tweet
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


class GizziAutoPoster:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

        # Storage paths
        self.data_dir = Path(__file__).parent.parent / "data" / "aggregated_content"
        self.history_dir = Path(__file__).parent.parent / "data" / "posting_history"
        self.history_dir.mkdir(parents=True, exist_ok=True)

        self.dev = "gizzi"  # Gizzi persona

    async def load_aggregated_data(self) -> Optional[Dict]:
        """Load latest aggregated content"""
        latest_file = self.data_dir / "latest.json"

        if not latest_file.exists():
            print("❌ No aggregated data found. Run gizzi_content_aggregator.py first!")
            return None

        with open(latest_file, "r") as f:
            data = json.load(f)

        print(f"✅ Loaded aggregated data from {data['date']}")
        return data

    async def select_opportunity(self, opportunities: List[Dict], force_topic: Optional[str] = None) -> Optional[Dict]:
        """
        Select best content opportunity to post about

        Args:
            opportunities: List of content opportunities
            force_topic: Force specific topic if provided

        Returns:
            Selected opportunity or None
        """
        if not opportunities:
            print("⚠️  No content opportunities available")
            return None

        # If topic forced, find matching opportunity
        if force_topic:
            for opp in opportunities:
                if force_topic.lower() in str(opp.get("topic", "")).lower() or \
                   force_topic.lower() in str(opp.get("idea", "")).lower():
                    print(f"🎯 Forced topic '{force_topic}' - selected: {opp['type']}")
                    return opp
            print(f"⚠️  Forced topic '{force_topic}' not found in opportunities")

        # Filter out recently posted topics
        recent_posts = await self.get_recent_posts(hours=48)
        recent_topics = [p.get("topic", "").lower() for p in recent_posts]

        # Prioritize high priority opportunities
        high_priority = [o for o in opportunities if o.get("priority") == "high"]
        medium_priority = [o for o in opportunities if o.get("priority") == "medium"]

        # Try high priority first
        for opp in high_priority:
            topic = str(opp.get("topic", opp.get("type", ""))).lower()
            if topic not in recent_topics:
                print(f"✅ Selected high-priority opportunity: {opp['type']}")
                return opp

        # Then medium priority
        for opp in medium_priority:
            topic = str(opp.get("topic", opp.get("type", ""))).lower()
            if topic not in recent_topics:
                print(f"✅ Selected medium-priority opportunity: {opp['type']}")
                return opp

        # Fallback: return first opportunity
        if opportunities:
            print(f"⚠️  All topics recently posted, using oldest opportunity")
            return opportunities[0]

        return None

    async def generate_tweet(self, opportunity: Dict, aggregated_data: Dict) -> str:
        """
        Generate tweet content based on opportunity

        Args:
            opportunity: Selected content opportunity
            aggregated_data: Full aggregated data for context

        Returns:
            Generated tweet text
        """
        print(f"\n🤖 Generating tweet for opportunity: {opportunity['type']}")

        # Build context for Claude
        opp_type = opportunity["type"]

        if opp_type == "trending_topic":
            context = f"""
You are Gizzi, the visionary AI developer behind Etrid blockchain. You're thoughtful, strategic, and always connecting current trends to Etrid's innovations.

A topic is trending in crypto: "{opportunity['topic']}"
It's mentioned in {opportunity['mentions']} recent articles.

Generate a tweet that:
1. Comments on this trending topic
2. Naturally relates it to Etrid's features (ASF consensus, EDSC stablecoin, multichain architecture)
3. Provides value/insight, not just promotion
4. Is 280 characters or less
5. Uses 1-2 relevant hashtags
6. Has an engaging, conversational tone

DO NOT be overly promotional. Focus on thought leadership.
"""

        elif opp_type == "competitor_analysis":
            competitors = aggregated_data["news_digest"].get("competitor_news", [])[:2]
            comp_list = [c.get("title", "competitor news") for c in competitors]

            context = f"""
You are Gizzi, the strategic AI developer for Etrid blockchain.

Recent competitor activity:
{chr(10).join(f"- {c}" for c in comp_list[:3])}

Generate a tweet that:
1. Acknowledges innovation in the blockchain space
2. Subtly highlights Etrid's unique advantages (ASF, multichain, EDSC)
3. Positions Etrid as next-generation solution
4. Is respectful to competitors (no trash-talking)
5. 280 characters or less
6. Uses hashtags: #Blockchain #Web3

Focus on differentiation, not competition.
"""

        elif opp_type == "reddit_engagement":
            mentions = opportunity.get("posts", [])[:2]
            mention_summaries = [f"- {m.get('title', 'Discussion about Etrid')}" for m in mentions]

            context = f"""
You are Gizzi, the community-focused AI developer for Etrid.

Etrid is being discussed on Reddit:
{chr(10).join(mention_summaries)}

Generate a tweet that:
1. Acknowledges the Reddit community discussing Etrid
2. Invites more questions/discussions
3. Provides a link to etrid.network or documentation
4. Is friendly and welcoming
5. 280 characters or less

Be authentic and community-focused.
"""

        elif opp_type == "positive_sentiment":
            sentiment_score = opportunity.get("sentiment_score", 70)

            context = f"""
You are Gizzi, the optimistic AI developer for Etrid blockchain.

Crypto market sentiment is positive today ({sentiment_score}% positive discussions).

Generate a tweet that:
1. Acknowledges positive market sentiment
2. Connects it to Etrid's mission/progress
3. Shares excitement about blockchain's future
4. Teases upcoming Etrid developments (without specifics)
5. 280 characters or less
6. Uses 1-2 positive emojis

Be genuine and inspiring, not hype-driven.
"""

        elif opp_type == "educational":
            topics = opportunity.get("topics", ["blockchain technology"])[:2]

            context = f"""
You are Gizzi, the educator AI developer for Etrid blockchain.

Topics trending that people need clarity on: {', '.join(topics)}

Generate a tweet announcing an educational thread about {topics[0]} and how Etrid implements it.

The tweet should:
1. Tease an upcoming thread (post this tweet, we'll generate thread separately)
2. Make the topic accessible and interesting
3. Promise clear explanations
4. 280 characters or less
5. Use 🧵 emoji to indicate thread coming

Be clear and engaging.
"""

        else:
            # Generic content opportunity
            context = f"""
You are Gizzi, thoughtful AI developer for Etrid blockchain.

Content idea: {opportunity.get('idea', 'Share Etrid update')}

Generate a tweet that:
1. Executes this content idea
2. Provides value to followers
3. Relates to Etrid naturally
4. Is 280 characters or less
5. Engaging and authentic tone

Be creative and valuable.
"""

        # Generate tweet using Claude
        tweet = await self.generator.generate_content(
            prompt=context,
            dev=self.dev,
            max_length=280
        )

        print(f"✅ Generated tweet ({len(tweet)} chars):\n{tweet}\n")

        return tweet

    async def generate_thread(self, opportunity: Dict, aggregated_data: Dict) -> List[str]:
        """
        Generate tweet thread (multiple tweets)

        Args:
            opportunity: Selected content opportunity
            aggregated_data: Full aggregated data for context

        Returns:
            List of tweet texts (thread)
        """
        print(f"\n🧵 Generating thread for opportunity: {opportunity['type']}")

        # Build context for Claude to generate 3-5 tweet thread
        context = f"""
You are Gizzi, AI developer for Etrid blockchain. You're creating an educational thread.

Topic: {opportunity.get('topic', opportunity.get('idea', 'Etrid technology'))}

Generate a 4-tweet thread that:
1. Tweet 1: Hook - interesting question or statement about the topic
2. Tweet 2: Explain the problem/challenge in the space
3. Tweet 3: How Etrid solves it (ASF consensus, EDSC, multichain, etc.)
4. Tweet 4: Call-to-action (learn more at etrid.network, ask questions, etc.)

Requirements:
- Each tweet MUST be under 280 characters
- Use 🧵 emoji in first tweet
- Number tweets (1/4, 2/4, etc.)
- Educational and valuable, not just promotional
- Clear, accessible language
- Use 1-2 hashtags in last tweet

Format your response as:
TWEET1: [tweet 1 text]
TWEET2: [tweet 2 text]
TWEET3: [tweet 3 text]
TWEET4: [tweet 4 text]
"""

        # Generate thread
        thread_text = await self.generator.generate_content(
            prompt=context,
            dev=self.dev,
            max_length=1200  # ~4 tweets worth
        )

        # Parse thread into individual tweets
        tweets = []
        for line in thread_text.split("\n"):
            if line.strip().startswith("TWEET"):
                tweet = line.split(":", 1)[1].strip() if ":" in line else line
                if tweet and len(tweet) <= 280:
                    tweets.append(tweet)

        # Validate thread
        if len(tweets) < 3:
            print("⚠️  Thread generation failed, falling back to single tweet")
            single_tweet = await self.generate_tweet(opportunity, aggregated_data)
            return [single_tweet]

        print(f"✅ Generated {len(tweets)}-tweet thread:")
        for i, tweet in enumerate(tweets, 1):
            print(f"   {i}/{len(tweets)}: {tweet[:60]}... ({len(tweet)} chars)")

        return tweets

    async def post_tweet(self, tweet: str) -> Optional[str]:
        """Post single tweet"""
        # Moderate content
        moderation = await self.moderator.moderate(tweet, strict=False)

        if not moderation["approved"]:
            print(f"❌ Tweet rejected by moderation: {moderation['issues']}")
            return None

        if moderation["needs_human_review"]:
            print(f"⚠️  Tweet needs human review: {moderation['warnings']}")
            # For now, skip posting
            return None

        # Post tweet
        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would post:")
            print(f"   {tweet}\n")
            return "dry_run_tweet_id"

        try:
            tweet_id = await self.twitter.post_tweet(tweet)
            print(f"✅ Posted tweet! ID: {tweet_id}")
            print(f"   URL: https://twitter.com/{os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')}/status/{tweet_id}")
            return tweet_id

        except Exception as e:
            print(f"❌ Error posting tweet: {e}")
            return None

    async def post_thread(self, tweets: List[str]) -> Optional[List[str]]:
        """Post tweet thread"""
        tweet_ids = []
        reply_to_id = None

        for i, tweet in enumerate(tweets, 1):
            print(f"\n📤 Posting tweet {i}/{len(tweets)}...")

            # Moderate
            moderation = await self.moderator.moderate(tweet, strict=False)

            if not moderation["approved"]:
                print(f"❌ Tweet {i} rejected: {moderation['issues']}")
                break

            # Post
            if self.dry_run:
                print(f"🧪 DRY RUN - Would post tweet {i}/{len(tweets)}:")
                print(f"   {tweet}")
                tweet_id = f"dry_run_tweet_{i}"
            else:
                try:
                    if reply_to_id:
                        tweet_id = await self.twitter.reply(
                            to_tweet_id=reply_to_id,
                            text=tweet,
                            username=os.getenv('TWITTER_USERNAME', 'EtridAI_Devs')
                        )
                    else:
                        tweet_id = await self.twitter.post_tweet(tweet)

                    print(f"✅ Posted tweet {i}/{len(tweets)}: {tweet_id}")

                except Exception as e:
                    print(f"❌ Error posting tweet {i}: {e}")
                    break

            tweet_ids.append(tweet_id)
            reply_to_id = tweet_id

            # Wait 2 seconds between tweets
            if i < len(tweets):
                await asyncio.sleep(2)

        if len(tweet_ids) == len(tweets):
            print(f"\n✅ Successfully posted complete thread ({len(tweet_ids)} tweets)")
            return tweet_ids
        else:
            print(f"\n⚠️  Partial thread posted ({len(tweet_ids)}/{len(tweets)} tweets)")
            return tweet_ids if tweet_ids else None

    async def save_posting_history(self, opportunity: Dict, tweet_ids: List[str]):
        """Save posting history"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        filename = f"post_{timestamp}.json"
        filepath = self.history_dir / filename

        history_entry = {
            "timestamp": datetime.now().isoformat(),
            "date": datetime.now().strftime("%Y-%m-%d"),
            "dev": self.dev,
            "opportunity_type": opportunity["type"],
            "topic": opportunity.get("topic", ""),
            "tweet_ids": tweet_ids,
            "thread": len(tweet_ids) > 1,
        }

        if not self.dry_run:
            with open(filepath, "w") as f:
                json.dump(history_entry, f, indent=2)
            print(f"✅ Saved posting history: {filepath}")

    async def get_recent_posts(self, hours: int = 48) -> List[Dict]:
        """Get recent posting history"""
        cutoff = datetime.now() - timedelta(hours=hours)
        recent_posts = []

        for filepath in self.history_dir.glob("post_*.json"):
            with open(filepath, "r") as f:
                post = json.load(f)
                post_time = datetime.fromisoformat(post["timestamp"])

                if post_time > cutoff:
                    recent_posts.append(post)

        return recent_posts

    async def run(self, force_topic: Optional[str] = None, generate_thread: bool = False):
        """Execute auto-posting workflow"""
        print("\n" + "="*60)
        print("  GIZZI AUTO POSTER")
        print("="*60 + "\n")

        try:
            # Load aggregated data
            aggregated_data = await self.load_aggregated_data()
            if not aggregated_data:
                return

            # Select opportunity
            opportunities = aggregated_data.get("opportunities", [])
            opportunity = await self.select_opportunity(opportunities, force_topic)

            if not opportunity:
                print("⚠️  No suitable content opportunity found")
                return

            # Generate content
            if generate_thread:
                tweets = await self.generate_thread(opportunity, aggregated_data)
                tweet_ids = await self.post_thread(tweets)
            else:
                tweet = await self.generate_tweet(opportunity, aggregated_data)
                tweet_id = await self.post_tweet(tweet)
                tweet_ids = [tweet_id] if tweet_id else []

            # Save history
            if tweet_ids:
                await self.save_posting_history(opportunity, tweet_ids)

            print("\n" + "="*60)
            print("  ✅ AUTO POSTING COMPLETE!")
            print("="*60 + "\n")

        except Exception as e:
            print(f"\n❌ Auto-posting error: {e}")
            import traceback
            traceback.print_exc()


async def main():
    # Parse arguments
    dry_run = "--dry-run" in sys.argv
    generate_thread = "--thread" in sys.argv

    force_topic = None
    for i, arg in enumerate(sys.argv):
        if arg == "--force-topic" and i + 1 < len(sys.argv):
            force_topic = sys.argv[i + 1]

    # Run workflow
    poster = GizziAutoPoster(dry_run=dry_run)
    await poster.run(force_topic=force_topic, generate_thread=generate_thread)


if __name__ == "__main__":
    asyncio.run(main())
