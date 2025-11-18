#!/usr/bin/env python3
"""
Reddit Connector - Monitor Reddit for Etrid mentions and crypto discussions

This connector:
1. Monitors r/cryptocurrency, r/defi, r/polkadot and custom subreddits
2. Searches for Etrid mentions
3. Tracks competitor discussions
4. Fetches trending posts in blockchain communities

Usage:
    from connectors.reddit import RedditConnector

    reddit = RedditConnector()
    mentions = await reddit.get_mentions(limit=10)
    posts = await reddit.get_trending_posts(subreddit="cryptocurrency")
"""

import os
import asyncio
from typing import List, Dict, Optional
from datetime import datetime, timedelta
import praw
from praw.models import Submission, Comment


class RedditConnector:
    """Connector for Reddit API operations"""

    def __init__(self):
        """Initialize Reddit API client"""
        # Initialize PRAW (Python Reddit API Wrapper)
        self.reddit = praw.Reddit(
            client_id=os.getenv("REDDIT_CLIENT_ID"),
            client_secret=os.getenv("REDDIT_CLIENT_SECRET"),
            user_agent=os.getenv("REDDIT_USER_AGENT", "EtridBot/1.0"),
            username=os.getenv("REDDIT_USERNAME"),
            password=os.getenv("REDDIT_PASSWORD"),
        )

        # Subreddits to monitor
        self.monitored_subreddits = [
            "cryptocurrency",
            "defi",
            "polkadot",
            "substrate",
            "CryptoTechnology",
            "CryptoMarkets",
            "ethtrader",
            "ethereum",
        ]

        # Keywords to track
        self.keywords = [
            "etrid",
            "ascending scale of finality",
            "asf consensus",
            "edsc stablecoin",
        ]

    async def search_mentions(self, query: str = "etrid", limit: int = 25, time_filter: str = "week") -> List[Dict]:
        """
        Search Reddit for mentions of Etrid

        Args:
            query: Search query
            limit: Max results
            time_filter: "hour", "day", "week", "month", "year", "all"

        Returns:
            List of post/comment dictionaries
        """
        print(f"🔍 Searching Reddit for '{query}' (limit={limit}, time={time_filter})...")

        mentions = []

        try:
            # Search across all of Reddit
            for submission in self.reddit.subreddit("all").search(query, limit=limit, time_filter=time_filter):
                mentions.append({
                    "type": "post",
                    "id": submission.id,
                    "title": submission.title,
                    "text": submission.selftext,
                    "author": str(submission.author) if submission.author else "[deleted]",
                    "subreddit": str(submission.subreddit),
                    "url": f"https://reddit.com{submission.permalink}",
                    "score": submission.score,
                    "num_comments": submission.num_comments,
                    "created_utc": datetime.fromtimestamp(submission.created_utc),
                    "upvote_ratio": submission.upvote_ratio,
                })

            print(f"   Found {len(mentions)} mention(s)")

        except Exception as e:
            print(f"❌ Error searching Reddit: {e}")

        return mentions

    async def get_trending_posts(self, subreddit: str = "cryptocurrency", limit: int = 10, sort: str = "hot") -> List[Dict]:
        """
        Get trending posts from a subreddit

        Args:
            subreddit: Subreddit name
            limit: Max posts
            sort: "hot", "new", "rising", "top"

        Returns:
            List of post dictionaries
        """
        print(f"📈 Fetching {sort} posts from r/{subreddit}...")

        posts = []

        try:
            subreddit_obj = self.reddit.subreddit(subreddit)

            # Get posts based on sort type
            if sort == "hot":
                submissions = subreddit_obj.hot(limit=limit)
            elif sort == "new":
                submissions = subreddit_obj.new(limit=limit)
            elif sort == "rising":
                submissions = subreddit_obj.rising(limit=limit)
            elif sort == "top":
                submissions = subreddit_obj.top(limit=limit, time_filter="day")
            else:
                submissions = subreddit_obj.hot(limit=limit)

            for submission in submissions:
                posts.append({
                    "id": submission.id,
                    "title": submission.title,
                    "text": submission.selftext[:500],  # First 500 chars
                    "author": str(submission.author) if submission.author else "[deleted]",
                    "subreddit": str(submission.subreddit),
                    "url": f"https://reddit.com{submission.permalink}",
                    "score": submission.score,
                    "num_comments": submission.num_comments,
                    "created_utc": datetime.fromtimestamp(submission.created_utc),
                    "upvote_ratio": submission.upvote_ratio,
                    "flair": submission.link_flair_text,
                })

            print(f"   Found {len(posts)} post(s)")

        except Exception as e:
            print(f"❌ Error fetching posts from r/{subreddit}: {e}")

        return posts

    async def monitor_keywords(self, limit: int = 50) -> List[Dict]:
        """
        Monitor all tracked subreddits for keywords

        Returns:
            List of relevant posts/comments
        """
        print(f"🔍 Monitoring {len(self.monitored_subreddits)} subreddits for keywords...")

        results = []

        for subreddit_name in self.monitored_subreddits:
            try:
                subreddit = self.reddit.subreddit(subreddit_name)

                # Check recent posts
                for submission in subreddit.new(limit=limit):
                    # Check if any keyword appears in title or text
                    text_to_check = f"{submission.title} {submission.selftext}".lower()

                    for keyword in self.keywords:
                        if keyword.lower() in text_to_check:
                            results.append({
                                "type": "post",
                                "keyword": keyword,
                                "subreddit": subreddit_name,
                                "id": submission.id,
                                "title": submission.title,
                                "text": submission.selftext[:300],
                                "author": str(submission.author) if submission.author else "[deleted]",
                                "url": f"https://reddit.com{submission.permalink}",
                                "score": submission.score,
                                "created_utc": datetime.fromtimestamp(submission.created_utc),
                            })
                            break  # Don't duplicate if multiple keywords match

            except Exception as e:
                print(f"❌ Error monitoring r/{subreddit_name}: {e}")

        print(f"   Found {len(results)} relevant post(s)")

        return results

    async def get_post_comments(self, post_id: str, limit: int = 10) -> List[Dict]:
        """
        Get top comments from a post

        Args:
            post_id: Reddit post ID
            limit: Max comments to fetch

        Returns:
            List of comment dictionaries
        """
        print(f"💬 Fetching comments for post {post_id}...")

        comments = []

        try:
            submission = self.reddit.submission(id=post_id)
            submission.comments.replace_more(limit=0)  # Flatten comment tree

            for comment in submission.comments[:limit]:
                if isinstance(comment, Comment):
                    comments.append({
                        "id": comment.id,
                        "text": comment.body[:500],
                        "author": str(comment.author) if comment.author else "[deleted]",
                        "score": comment.score,
                        "created_utc": datetime.fromtimestamp(comment.created_utc),
                        "is_submitter": comment.is_submitter,
                    })

            print(f"   Found {len(comments)} comment(s)")

        except Exception as e:
            print(f"❌ Error fetching comments: {e}")

        return comments

    async def analyze_sentiment(self, posts: List[Dict]) -> Dict:
        """
        Analyze sentiment of posts (basic version)

        Args:
            posts: List of post dictionaries

        Returns:
            Sentiment summary
        """
        if not posts:
            return {"sentiment": "neutral", "positive": 0, "negative": 0, "neutral": 0}

        positive_keywords = ["bullish", "moon", "gem", "excellent", "great", "innovative", "revolutionary"]
        negative_keywords = ["bearish", "scam", "dump", "terrible", "awful", "fail", "dead"]

        positive_count = 0
        negative_count = 0
        neutral_count = 0

        for post in posts:
            text = f"{post.get('title', '')} {post.get('text', '')}".lower()

            has_positive = any(kw in text for kw in positive_keywords)
            has_negative = any(kw in text for kw in negative_keywords)

            if has_positive and not has_negative:
                positive_count += 1
            elif has_negative and not has_positive:
                negative_count += 1
            else:
                neutral_count += 1

        total = len(posts)
        sentiment = "neutral"

        if positive_count > negative_count and positive_count / total > 0.3:
            sentiment = "positive"
        elif negative_count > positive_count and negative_count / total > 0.3:
            sentiment = "negative"

        return {
            "sentiment": sentiment,
            "positive": positive_count,
            "negative": negative_count,
            "neutral": neutral_count,
            "total": total,
            "positive_pct": round(positive_count / total * 100, 1) if total > 0 else 0,
            "negative_pct": round(negative_count / total * 100, 1) if total > 0 else 0,
        }


# Example usage
async def main():
    """Test Reddit connector"""
    reddit = RedditConnector()

    # Search for Etrid mentions
    mentions = await reddit.search_mentions("etrid", limit=5)
    print(f"\n📊 Found {len(mentions)} Etrid mentions")
    for mention in mentions[:3]:
        print(f"   - r/{mention['subreddit']}: {mention['title'][:60]}")

    # Get trending crypto posts
    trending = await reddit.get_trending_posts("cryptocurrency", limit=5)
    print(f"\n🔥 Top 5 posts in r/cryptocurrency:")
    for post in trending[:5]:
        print(f"   - {post['title'][:60]} ({post['score']} upvotes)")

    # Monitor keywords
    keyword_posts = await reddit.monitor_keywords(limit=20)
    print(f"\n🔍 Keyword monitoring found {len(keyword_posts)} relevant posts")

    # Sentiment analysis
    if trending:
        sentiment = await reddit.analyze_sentiment(trending)
        print(f"\n📈 Sentiment: {sentiment['sentiment']} ({sentiment['positive_pct']}% positive)")


if __name__ == "__main__":
    asyncio.run(main())
