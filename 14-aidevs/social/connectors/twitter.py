"""
Twitter API Connector for AI Devs

Handles all Twitter interactions:
- Posting tweets
- Posting threads
- Replying to mentions
- Monitoring mentions
- Retrieving user data

Uses Twitter API v2 via tweepy library
"""

import os
import tweepy
from typing import Optional, List
from datetime import datetime

class TwitterConnector:
    def __init__(self):
        """Initialize Twitter API client"""
        self.client = tweepy.Client(
            consumer_key=os.getenv('TWITTER_API_KEY'),
            consumer_secret=os.getenv('TWITTER_API_SECRET'),
            access_token=os.getenv('TWITTER_ACCESS_TOKEN'),
            access_token_secret=os.getenv('TWITTER_ACCESS_SECRET')
        )

        # Rate limiting
        self.posts_today = 0
        self.max_posts_per_day = 1500

    async def post(self, text: str, reply_to: Optional[str] = None) -> str:
        """
        Post a tweet

        Args:
            text: Tweet content
            reply_to: Tweet ID to reply to (optional)

        Returns:
            Tweet ID of posted tweet
        """
        # Check rate limits
        if self.posts_today >= self.max_posts_per_day:
            raise Exception(f"Daily post limit reached ({self.max_posts_per_day})")

        # Validate length
        if len(text) > 280:
            raise Exception(f"Tweet too long: {len(text)} characters")

        # Post tweet
        response = self.client.create_tweet(
            text=text,
            in_reply_to_tweet_id=reply_to
        )

        self.posts_today += 1

        return response.data['id']

    async def post_thread(self, tweets: List[str]) -> List[str]:
        """
        Post a thread of tweets

        Args:
            tweets: List of tweet texts

        Returns:
            List of tweet IDs
        """
        tweet_ids = []
        previous_id = None

        for i, tweet_text in enumerate(tweets):
            # Add thread numbering (1/5, 2/5, etc.)
            numbered_text = f"{tweet_text}\n\n{i+1}/{len(tweets)}"

            # Post tweet (reply to previous if not first)
            tweet_id = await self.post(numbered_text, reply_to=previous_id)
            tweet_ids.append(tweet_id)
            previous_id = tweet_id

        return tweet_ids

    async def reply(self, to_tweet_id: str, text: str, username: str) -> str:
        """
        Reply to a tweet (mention user in reply)

        Args:
            to_tweet_id: Tweet ID to reply to
            text: Reply content
            username: Username to mention

        Returns:
            Tweet ID of reply
        """
        # Add mention if not already in text
        if f"@{username}" not in text:
            reply_text = f"@{username} {text}"
        else:
            reply_text = text

        return await self.post(reply_text, reply_to=to_tweet_id)

    async def get_mentions(self, since_id: Optional[str] = None) -> List[dict]:
        """
        Get recent mentions of @EtridAI_Devs

        Args:
            since_id: Only return tweets after this ID

        Returns:
            List of mention objects
        """
        # Get authenticated user ID
        me = self.client.get_me()
        user_id = me.data.id

        # Fetch mentions
        mentions = self.client.get_users_mentions(
            user_id,
            since_id=since_id,
            expansions=['author_id'],
            tweet_fields=['created_at', 'public_metrics'],
            max_results=100
        )

        if not mentions.data:
            return []

        # Format results
        results = []
        for mention in mentions.data:
            results.append({
                'id': mention.id,
                'text': mention.text,
                'created_at': mention.created_at,
                'author_id': mention.author_id,
                'metrics': mention.public_metrics
            })

        return results

    async def pin_tweet(self, tweet_id: str):
        """Pin a tweet to profile"""
        me = self.client.get_me()
        self.client.pin_tweet(tweet_id)

    async def unpin_tweet(self):
        """Unpin currently pinned tweet"""
        me = self.client.get_me()
        self.client.unpin_tweet()

    async def delete_tweet(self, tweet_id: str):
        """Delete a tweet"""
        self.client.delete_tweet(tweet_id)

    async def like_tweet(self, tweet_id: str):
        """Like a tweet"""
        self.client.like(tweet_id)

    async def retweet(self, tweet_id: str):
        """Retweet a tweet"""
        self.client.retweet(tweet_id)

    async def get_tweet(self, tweet_id: str) -> dict:
        """Get tweet details"""
        tweet = self.client.get_tweet(
            tweet_id,
            tweet_fields=['created_at', 'public_metrics', 'author_id']
        )

        return {
            'id': tweet.data.id,
            'text': tweet.data.text,
            'created_at': tweet.data.created_at,
            'author_id': tweet.data.author_id,
            'metrics': tweet.data.public_metrics
        }

    async def get_user(self, username: str) -> dict:
        """Get user details"""
        user = self.client.get_user(
            username=username,
            user_fields=['created_at', 'public_metrics', 'verified']
        )

        return {
            'id': user.data.id,
            'username': user.data.username,
            'name': user.data.name,
            'created_at': user.data.created_at,
            'verified': user.data.verified,
            'followers': user.data.public_metrics['followers_count']
        }
