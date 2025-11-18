#!/usr/bin/env python3
"""
Gizzi Content Aggregator - Scrape and aggregate content for posting

This workflow:
1. Scrapes crypto news, tech publications, Reddit
2. Filters for relevant blockchain/DeFi content
3. Identifies trending topics and competitor activity
4. Stores aggregated content for Gizzi to use in posts
5. Generates content ideas based on scraped data

Usage:
    python gizzi_content_aggregator.py              # Run full aggregation
    python gizzi_content_aggregator.py --dry-run    # Test without saving
    python gizzi_content_aggregator.py --reddit-only # Only scrape Reddit
"""

import asyncio
import os
import sys
import json
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List

# Add parent directory to path
sys.path.append(str(Path(__file__).parent.parent))

from connectors.web_scraper import WebScraper
from connectors.reddit import RedditConnector
from content.generator import ContentGenerator


class GizziContentAggregator:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.web_scraper = WebScraper()
        self.reddit = RedditConnector()
        self.generator = ContentGenerator()

        # Storage path for aggregated content
        self.storage_dir = Path(__file__).parent.parent / "data" / "aggregated_content"
        self.storage_dir.mkdir(parents=True, exist_ok=True)

    async def aggregate_news(self) -> Dict:
        """Aggregate crypto and tech news"""
        print("\n" + "="*60)
        print("📰 AGGREGATING NEWS")
        print("="*60 + "\n")

        # Scrape crypto news (last 24h)
        crypto_news = await self.web_scraper.scrape_crypto_news(limit=20, hours=24)

        # Scrape tech news
        tech_news = await self.web_scraper.scrape_tech_radar(limit=15)

        # Filter tech news for relevance
        relevant_tech = await self.web_scraper.filter_relevant_articles(tech_news)

        # Track competitor mentions
        competitor_news = await self.web_scraper.track_competitors(hours=24)

        # Get trending topics
        all_articles = crypto_news + relevant_tech
        trending_topics = await self.web_scraper.get_trending_topics(all_articles)

        news_digest = {
            "crypto_news": crypto_news[:15],  # Top 15
            "tech_news": relevant_tech[:10],   # Top 10 relevant
            "competitor_news": competitor_news[:8],
            "trending_topics": dict(list(trending_topics.items())[:10]),
            "total_articles": len(all_articles),
        }

        print(f"✅ Aggregated {len(crypto_news)} crypto articles")
        print(f"✅ Aggregated {len(relevant_tech)} relevant tech articles")
        print(f"✅ Found {len(competitor_news)} competitor mentions")
        print(f"✅ Identified {len(trending_topics)} trending topics")

        return news_digest

    async def aggregate_reddit(self) -> Dict:
        """Aggregate Reddit discussions"""
        print("\n" + "="*60)
        print("🔍 AGGREGATING REDDIT")
        print("="*60 + "\n")

        # Search for Etrid mentions
        etrid_mentions = await self.reddit.search_mentions("etrid", limit=10, time_filter="week")

        # Get trending posts from key subreddits
        crypto_trending = await self.reddit.get_trending_posts("cryptocurrency", limit=10, sort="hot")
        defi_trending = await self.reddit.get_trending_posts("defi", limit=10, sort="hot")
        polkadot_trending = await self.reddit.get_trending_posts("polkadot", limit=10, sort="hot")

        # Monitor keywords
        keyword_posts = await self.reddit.monitor_keywords(limit=30)

        # Analyze sentiment
        all_posts = crypto_trending + defi_trending + polkadot_trending
        sentiment = await self.reddit.analyze_sentiment(all_posts)

        reddit_digest = {
            "etrid_mentions": etrid_mentions,
            "crypto_trending": crypto_trending[:5],  # Top 5
            "defi_trending": defi_trending[:5],
            "polkadot_trending": polkadot_trending[:5],
            "keyword_posts": keyword_posts[:10],
            "sentiment": sentiment,
            "total_posts_analyzed": len(all_posts),
        }

        print(f"✅ Found {len(etrid_mentions)} Etrid mentions")
        print(f"✅ Analyzed {len(all_posts)} trending posts")
        print(f"✅ Found {len(keyword_posts)} keyword matches")
        print(f"✅ Overall sentiment: {sentiment['sentiment']} ({sentiment['positive_pct']}% positive)")

        return reddit_digest

    async def identify_content_opportunities(self, news_digest: Dict, reddit_digest: Dict) -> List[Dict]:
        """
        Analyze aggregated data to identify posting opportunities

        Returns:
            List of content ideas for Gizzi
        """
        print("\n" + "="*60)
        print("💡 IDENTIFYING CONTENT OPPORTUNITIES")
        print("="*60 + "\n")

        opportunities = []

        # 1. Trending topic opportunities
        for topic, count in list(news_digest["trending_topics"].items())[:5]:
            if count >= 3:  # Topic appears in 3+ articles
                opportunities.append({
                    "type": "trending_topic",
                    "topic": topic,
                    "mentions": count,
                    "priority": "high" if count >= 5 else "medium",
                    "idea": f"Comment on trending '{topic}' topic in context of Etrid",
                    "source": "news_aggregation",
                })

        # 2. Competitor activity opportunities
        if len(news_digest["competitor_news"]) >= 2:
            opportunities.append({
                "type": "competitor_analysis",
                "count": len(news_digest["competitor_news"]),
                "priority": "medium",
                "idea": "Compare Etrid's features vs recent competitor announcements",
                "source": "competitor_tracking",
            })

        # 3. Reddit engagement opportunities
        if len(reddit_digest["etrid_mentions"]) > 0:
            opportunities.append({
                "type": "reddit_engagement",
                "mentions": len(reddit_digest["etrid_mentions"]),
                "priority": "high",
                "idea": "Respond to Etrid mentions on Reddit",
                "source": "reddit_monitoring",
                "posts": reddit_digest["etrid_mentions"][:3],
            })

        # 4. Sentiment-based opportunities
        if reddit_digest["sentiment"]["sentiment"] == "positive":
            opportunities.append({
                "type": "positive_sentiment",
                "priority": "medium",
                "idea": "Capitalize on positive crypto sentiment with Etrid benefits thread",
                "source": "sentiment_analysis",
                "sentiment_score": reddit_digest["sentiment"]["positive_pct"],
            })
        elif reddit_digest["sentiment"]["sentiment"] == "negative":
            opportunities.append({
                "type": "negative_sentiment",
                "priority": "low",
                "idea": "Address crypto market concerns, highlight Etrid's stability features (EDSC, ASF)",
                "source": "sentiment_analysis",
                "sentiment_score": reddit_digest["sentiment"]["negative_pct"],
            })

        # 5. Keyword match opportunities
        if len(reddit_digest["keyword_posts"]) >= 3:
            opportunities.append({
                "type": "keyword_relevance",
                "count": len(reddit_digest["keyword_posts"]),
                "priority": "medium",
                "idea": "Engage in discussions about blockchain tech where Etrid is relevant",
                "source": "keyword_monitoring",
            })

        # 6. Educational content opportunities
        # If many articles about complex topics, create explainer content
        complex_topics = ["consensus", "finality", "stablecoin", "multichain"]
        trending_complex = [t for t in news_digest["trending_topics"].keys() if any(ct in t for ct in complex_topics)]

        if len(trending_complex) >= 2:
            opportunities.append({
                "type": "educational",
                "topics": trending_complex[:3],
                "priority": "medium",
                "idea": f"Create explainer thread on {trending_complex[0]} and how Etrid implements it",
                "source": "topic_analysis",
            })

        print(f"✅ Identified {len(opportunities)} content opportunities:")
        for opp in opportunities:
            print(f"   [{opp['priority'].upper()}] {opp['type']}: {opp['idea'][:80]}")

        return opportunities

    async def save_aggregated_data(self, news_digest: Dict, reddit_digest: Dict, opportunities: List[Dict]):
        """Save aggregated data to file for later use"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        filename = f"aggregated_{timestamp}.json"
        filepath = self.storage_dir / filename

        data = {
            "timestamp": datetime.now().isoformat(),
            "date": datetime.now().strftime("%Y-%m-%d"),
            "news_digest": news_digest,
            "reddit_digest": reddit_digest,
            "opportunities": opportunities,
        }

        if self.dry_run:
            print(f"\n🧪 DRY RUN - Would save to: {filepath}")
            print(f"   Data size: {len(json.dumps(data))} bytes")
        else:
            with open(filepath, "w") as f:
                json.dump(data, f, indent=2, default=str)
            print(f"\n✅ Saved aggregated data to: {filepath}")

        # Also save as "latest.json" for easy access
        latest_filepath = self.storage_dir / "latest.json"
        if not self.dry_run:
            with open(latest_filepath, "w") as f:
                json.dump(data, f, indent=2, default=str)
            print(f"✅ Updated latest.json")

    async def load_latest_aggregated_data(self) -> Dict:
        """Load the most recent aggregated data"""
        latest_filepath = self.storage_dir / "latest.json"

        if not latest_filepath.exists():
            print("⚠️  No aggregated data found")
            return {}

        with open(latest_filepath, "r") as f:
            data = json.load(f)

        print(f"✅ Loaded aggregated data from {data['date']}")
        return data

    async def run(self, reddit_only: bool = False):
        """Execute full aggregation workflow"""
        print("\n" + "="*60)
        print("  GIZZI CONTENT AGGREGATOR")
        print("="*60 + "\n")

        start_time = datetime.now()

        try:
            # Aggregate news (unless reddit-only mode)
            if reddit_only:
                news_digest = {"crypto_news": [], "tech_news": [], "competitor_news": [], "trending_topics": {}, "total_articles": 0}
            else:
                news_digest = await self.aggregate_news()

            # Aggregate Reddit
            reddit_digest = await self.aggregate_reddit()

            # Identify content opportunities
            opportunities = await self.identify_content_opportunities(news_digest, reddit_digest)

            # Save aggregated data
            await self.save_aggregated_data(news_digest, reddit_digest, opportunities)

            # Summary
            duration = (datetime.now() - start_time).total_seconds()
            print("\n" + "="*60)
            print("  ✅ AGGREGATION COMPLETE!")
            print("="*60)
            print(f"  Duration: {duration:.1f}s")
            print(f"  News articles: {news_digest['total_articles']}")
            print(f"  Reddit posts: {reddit_digest['total_posts_analyzed']}")
            print(f"  Opportunities: {len(opportunities)}")
            print("="*60 + "\n")

        except Exception as e:
            print(f"\n❌ Aggregation error: {e}")
            import traceback
            traceback.print_exc()


async def main():
    # Parse arguments
    dry_run = "--dry-run" in sys.argv
    reddit_only = "--reddit-only" in sys.argv

    # Run workflow
    aggregator = GizziContentAggregator(dry_run=dry_run)
    await aggregator.run(reddit_only=reddit_only)


if __name__ == "__main__":
    asyncio.run(main())
