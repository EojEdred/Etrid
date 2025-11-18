#!/usr/bin/env python3
"""
Web Scraper Connector - Scrape crypto news, tech blogs, and competitor info

This connector:
1. Scrapes crypto news sites (CoinDesk, CoinTelegraph, The Block, etc.)
2. Monitors tech publications (TechRadar, TechCrunch, VentureBeat)
3. Tracks competitor announcements
4. Extracts relevant blockchain/DeFi content
5. Aggregates RSS feeds

Usage:
    from connectors.web_scraper import WebScraper

    scraper = WebScraper()
    articles = await scraper.scrape_crypto_news(limit=10)
    tech_news = await scraper.scrape_tech_radar()
"""

import os
import asyncio
from typing import List, Dict, Optional
from datetime import datetime, timedelta
import feedparser
import aiohttp
from bs4 import BeautifulSoup
from newspaper import Article
from fake_useragent import UserAgent


class WebScraper:
    """Connector for web scraping operations"""

    def __init__(self):
        """Initialize web scraper"""
        self.user_agent = UserAgent()

        # Crypto news RSS feeds
        self.crypto_feeds = {
            "CoinDesk": "https://www.coindesk.com/arc/outboundfeeds/rss/",
            "CoinTelegraph": "https://cointelegraph.com/rss",
            "The Block": "https://www.theblock.co/rss.xml",
            "Decrypt": "https://decrypt.co/feed",
            "Bitcoin Magazine": "https://bitcoinmagazine.com/.rss/full/",
            "CryptoSlate": "https://cryptoslate.com/feed/",
            "BeInCrypto": "https://beincrypto.com/feed/",
        }

        # Tech news RSS feeds
        self.tech_feeds = {
            "TechRadar": "https://www.techradar.com/rss",
            "TechCrunch": "https://techcrunch.com/feed/",
            "VentureBeat": "https://venturebeat.com/feed/",
            "The Verge": "https://www.theverge.com/rss/index.xml",
            "Ars Technica": "https://feeds.arstechnica.com/arstechnica/index",
        }

        # Keywords to filter for relevance
        self.keywords = [
            "blockchain",
            "cryptocurrency",
            "defi",
            "stablecoin",
            "consensus",
            "proof of stake",
            "layer 1",
            "polkadot",
            "substrate",
            "multichain",
            "cross-chain",
            "finality",
            "validator",
            "staking",
        ]

        # Competitor projects to track
        self.competitors = [
            "avalanche",
            "cosmos",
            "polkadot",
            "cardano",
            "algorand",
            "near protocol",
            "aptos",
            "sui",
        ]

    async def fetch_url(self, url: str, timeout: int = 10) -> Optional[str]:
        """
        Fetch URL content with error handling

        Args:
            url: URL to fetch
            timeout: Request timeout in seconds

        Returns:
            HTML content or None if failed
        """
        headers = {"User-Agent": self.user_agent.random}

        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(url, headers=headers, timeout=timeout) as response:
                    if response.status == 200:
                        return await response.text()
                    else:
                        print(f"⚠️  HTTP {response.status} for {url}")
                        return None

        except asyncio.TimeoutError:
            print(f"⏰ Timeout fetching {url}")
            return None
        except Exception as e:
            print(f"❌ Error fetching {url}: {e}")
            return None

    async def parse_rss_feed(self, feed_url: str, limit: int = 10) -> List[Dict]:
        """
        Parse RSS feed and extract articles

        Args:
            feed_url: RSS feed URL
            limit: Max articles to return

        Returns:
            List of article dictionaries
        """
        articles = []

        try:
            # Parse RSS feed (feedparser is sync, but fast)
            feed = feedparser.parse(feed_url)

            for entry in feed.entries[:limit]:
                # Extract article data
                article = {
                    "title": entry.get("title", ""),
                    "url": entry.get("link", ""),
                    "published": entry.get("published", ""),
                    "summary": entry.get("summary", "")[:300],  # First 300 chars
                    "source": feed.feed.get("title", "Unknown"),
                }

                # Parse published date
                if "published_parsed" in entry:
                    article["published_date"] = datetime(*entry.published_parsed[:6])
                else:
                    article["published_date"] = None

                articles.append(article)

        except Exception as e:
            print(f"❌ Error parsing RSS feed {feed_url}: {e}")

        return articles

    async def scrape_crypto_news(self, limit: int = 10, hours: int = 24) -> List[Dict]:
        """
        Scrape crypto news from multiple sources

        Args:
            limit: Max articles per source
            hours: Only include articles from last N hours

        Returns:
            List of article dictionaries
        """
        print(f"📰 Scraping crypto news from {len(self.crypto_feeds)} sources...")

        all_articles = []
        cutoff_time = datetime.now() - timedelta(hours=hours)

        # Fetch from all crypto feeds
        tasks = []
        for source_name, feed_url in self.crypto_feeds.items():
            tasks.append(self.parse_rss_feed(feed_url, limit))

        results = await asyncio.gather(*tasks, return_exceptions=True)

        for articles in results:
            if isinstance(articles, list):
                all_articles.extend(articles)

        # Filter by time
        recent_articles = [
            a for a in all_articles
            if a.get("published_date") and a["published_date"] > cutoff_time
        ]

        # Sort by date (newest first)
        recent_articles.sort(key=lambda x: x.get("published_date", datetime.min), reverse=True)

        print(f"   Found {len(recent_articles)} articles from last {hours}h")

        return recent_articles

    async def scrape_tech_radar(self, limit: int = 10) -> List[Dict]:
        """
        Scrape TechRadar and other tech publications

        Args:
            limit: Max articles per source

        Returns:
            List of article dictionaries
        """
        print(f"🔧 Scraping tech news from {len(self.tech_feeds)} sources...")

        all_articles = []

        # Fetch from all tech feeds
        tasks = []
        for source_name, feed_url in self.tech_feeds.items():
            tasks.append(self.parse_rss_feed(feed_url, limit))

        results = await asyncio.gather(*tasks, return_exceptions=True)

        for articles in results:
            if isinstance(articles, list):
                all_articles.extend(articles)

        print(f"   Found {len(all_articles)} tech articles")

        return all_articles

    async def filter_relevant_articles(self, articles: List[Dict]) -> List[Dict]:
        """
        Filter articles for blockchain/crypto relevance

        Args:
            articles: List of articles

        Returns:
            Filtered list of relevant articles
        """
        relevant = []

        for article in articles:
            text_to_check = f"{article.get('title', '')} {article.get('summary', '')}".lower()

            # Check if any keyword appears
            if any(keyword in text_to_check for keyword in self.keywords):
                article["matched_keywords"] = [
                    kw for kw in self.keywords if kw in text_to_check
                ]
                relevant.append(article)

        print(f"🔍 Filtered to {len(relevant)} relevant articles (from {len(articles)} total)")

        return relevant

    async def extract_full_article(self, url: str) -> Optional[Dict]:
        """
        Extract full article content using newspaper3k

        Args:
            url: Article URL

        Returns:
            Article dictionary with full content
        """
        try:
            article = Article(url)
            article.download()
            article.parse()

            return {
                "url": url,
                "title": article.title,
                "text": article.text,
                "authors": article.authors,
                "publish_date": article.publish_date,
                "top_image": article.top_image,
                "keywords": article.keywords if hasattr(article, 'keywords') else [],
            }

        except Exception as e:
            print(f"❌ Error extracting article {url}: {e}")
            return None

    async def track_competitors(self, hours: int = 24) -> List[Dict]:
        """
        Track competitor mentions in crypto news

        Args:
            hours: Look back window

        Returns:
            List of articles mentioning competitors
        """
        print(f"🔍 Tracking competitor mentions from last {hours}h...")

        # Scrape crypto news
        articles = await self.scrape_crypto_news(limit=20, hours=hours)

        competitor_mentions = []

        for article in articles:
            text_to_check = f"{article.get('title', '')} {article.get('summary', '')}".lower()

            # Check for competitor mentions
            mentioned_competitors = [
                comp for comp in self.competitors if comp in text_to_check
            ]

            if mentioned_competitors:
                article["competitors_mentioned"] = mentioned_competitors
                competitor_mentions.append(article)

        print(f"   Found {len(competitor_mentions)} articles mentioning competitors")

        return competitor_mentions

    async def get_trending_topics(self, articles: List[Dict]) -> Dict[str, int]:
        """
        Analyze trending topics from articles

        Args:
            articles: List of articles

        Returns:
            Dictionary of topic -> count
        """
        topic_counts = {}

        for article in articles:
            text = f"{article.get('title', '')} {article.get('summary', '')}".lower()

            # Count keyword occurrences
            for keyword in self.keywords:
                if keyword in text:
                    topic_counts[keyword] = topic_counts.get(keyword, 0) + 1

        # Sort by count
        sorted_topics = dict(sorted(topic_counts.items(), key=lambda x: x[1], reverse=True))

        return sorted_topics

    async def scrape_twitter_trending(self) -> List[str]:
        """
        Get trending topics on crypto Twitter (X)

        Note: This is a placeholder. Actual implementation would use Twitter API
        or scrape trending hashtags. For now, returns common crypto hashtags.

        Returns:
            List of trending hashtags/topics
        """
        # Placeholder - would need Twitter API access or web scraping
        # Common crypto trending topics
        return [
            "#Bitcoin",
            "#Ethereum",
            "#DeFi",
            "#Crypto",
            "#Blockchain",
            "#Web3",
            "#NFT",
            "#Altcoins",
        ]

    async def aggregate_daily_digest(self) -> Dict:
        """
        Create comprehensive daily digest of crypto/tech news

        Returns:
            Dictionary with categorized content
        """
        print("📊 Aggregating daily digest...")

        # Fetch all content types
        crypto_news = await self.scrape_crypto_news(limit=15, hours=24)
        tech_news = await self.scrape_tech_radar(limit=10)
        relevant_tech = await self.filter_relevant_articles(tech_news)
        competitor_news = await self.track_competitors(hours=24)
        trending_topics = await self.get_trending_topics(crypto_news + relevant_tech)

        digest = {
            "date": datetime.now().strftime("%Y-%m-%d"),
            "crypto_news": crypto_news[:10],  # Top 10
            "tech_news": relevant_tech[:5],   # Top 5 relevant
            "competitor_mentions": competitor_news[:5],
            "trending_topics": dict(list(trending_topics.items())[:10]),  # Top 10 topics
            "total_articles_analyzed": len(crypto_news) + len(tech_news),
        }

        print(f"✅ Daily digest complete: {digest['total_articles_analyzed']} articles analyzed")

        return digest


# Example usage
async def main():
    """Test web scraper"""
    scraper = WebScraper()

    # Scrape crypto news
    print("\n" + "="*60)
    crypto_articles = await scraper.scrape_crypto_news(limit=5, hours=24)
    print(f"\n📰 Top 5 crypto news articles:")
    for article in crypto_articles[:5]:
        print(f"   - [{article['source']}] {article['title'][:60]}")

    # Scrape tech news
    print("\n" + "="*60)
    tech_articles = await scraper.scrape_tech_radar(limit=5)
    relevant_tech = await scraper.filter_relevant_articles(tech_articles)
    print(f"\n🔧 Relevant tech articles:")
    for article in relevant_tech[:3]:
        print(f"   - [{article['source']}] {article['title'][:60]}")

    # Track competitors
    print("\n" + "="*60)
    competitor_articles = await scraper.track_competitors(hours=24)
    print(f"\n🔍 Competitor mentions:")
    for article in competitor_articles[:3]:
        print(f"   - {article['title'][:60]}")
        print(f"     Mentioned: {', '.join(article['competitors_mentioned'])}")

    # Get trending topics
    print("\n" + "="*60)
    all_articles = crypto_articles + relevant_tech
    trending = await scraper.get_trending_topics(all_articles)
    print(f"\n📈 Trending topics:")
    for topic, count in list(trending.items())[:5]:
        print(f"   - {topic}: {count} mentions")

    # Daily digest
    print("\n" + "="*60)
    digest = await scraper.aggregate_daily_digest()
    print(f"\n📊 Daily Digest Summary:")
    print(f"   - Crypto news: {len(digest['crypto_news'])} articles")
    print(f"   - Tech news: {len(digest['tech_news'])} articles")
    print(f"   - Competitor mentions: {len(digest['competitor_mentions'])} articles")
    print(f"   - Top trending topic: {list(digest['trending_topics'].keys())[0] if digest['trending_topics'] else 'N/A'}")


if __name__ == "__main__":
    asyncio.run(main())
