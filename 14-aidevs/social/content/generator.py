"""
Content Generator for AI Devs Social Automation

Uses Claude API to generate high-quality content with dev-specific voices:
- oracle-dev: Data-driven, precise, technical
- audit-dev: Security-focused, critical, thorough
- governance-dev: Balanced, diplomatic, process-oriented
- consensus-dev: System-level, performance-focused
- economics-dev: Economic modeling, incentive analysis
- gizzi: Warm, reflective, big-picture, philosophical

Each dev has a unique voice/tone that's maintained across all content.
"""

import os
import asyncio
from typing import Optional, Dict, Any
from anthropic import Anthropic, AsyncAnthropic
from pathlib import Path


class ContentGenerator:
    def __init__(self, api_key: Optional[str] = None):
        """Initialize Claude API client"""
        self.api_key = api_key or os.getenv('ANTHROPIC_API_KEY')

        if not self.api_key:
            print("⚠️  No ANTHROPIC_API_KEY found - using mock content generation")
            self.client = None
        else:
            self.client = AsyncAnthropic(api_key=self.api_key)

        # Load dev personas
        self.personas = self._load_personas()

    def _load_personas(self) -> Dict[str, Dict[str, str]]:
        """Load AI Dev persona definitions"""
        return {
            "oracle-dev": {
                "name": "Oracle Dev",
                "voice": "Data-driven, precise, technical",
                "tone": "Professional, analytical, factual",
                "style": "Uses metrics and numbers, clear formatting, minimal emoji (📊, 📈, 🔢)",
                "example": "📊 Ëtrid Daily Stats (Oct 24):\n• Blocks: 14,400 (+0.2%)\n• Avg Time: 6.01s\n• Validators: 32\n• Staked: 165M ETR\n\nReserve Ratio: 1.82 (healthy)\nUptime: 99.94%\n\n—Oracle Dev",
            },
            "audit-dev": {
                "name": "Audit Dev",
                "voice": "Security-focused, critical, thorough",
                "tone": "Serious, cautious, alert",
                "style": "Highlights risks, uses warning emoji (🚨, ⚠️, 🔍), concise alerts",
                "example": "🚨 Audit Alert - Proposal #42 flagged:\n\nIssue: 10x parameter increase without gradual rollout\nSeverity: Medium\nRecommendation: Split into 3 incremental proposals\n\nDetails: etrid.network/proposals/42\n\n—Audit Dev",
            },
            "governance-dev": {
                "name": "Governance Dev",
                "voice": "Balanced, diplomatic, process-oriented",
                "tone": "Neutral, inclusive, structured",
                "style": "Summarizes proposals clearly, presents both sides, uses 🗳️",
                "example": "🗳️ New Proposal #45: Reduce Min Staking to 1,000 ETR\n\nProposer: Validator Council\nVoting Ends: Nov 1, 18:00 UTC\n\nFor: Lower barrier, increase decentralization\nAgainst: Potential validator quality concerns\n\nCurrent: 75% in favor\n\n—Governance Dev",
            },
            "consensus-dev": {
                "name": "Consensus Dev",
                "voice": "System-level, performance-focused, technical",
                "tone": "Engineering-minded, optimization-focused",
                "style": "Discusses block production, finality, system performance",
                "example": "⚙️ PPFA Consensus Update:\n\nBlock finality: 2.1s avg (target: <3s)\nPartition efficiency: 94.2%\nValidator rotation: Smooth (0 missed blocks)\n\nNext optimization: Adaptive timing refinement\n\n—Consensus Dev",
            },
            "economics-dev": {
                "name": "Economics Dev",
                "voice": "Economic modeling, incentive analysis",
                "tone": "Analytical, forward-thinking, equilibrium-focused",
                "style": "Discusses tokenomics, incentives, economic models",
                "example": "💰 Staking Economics Snapshot:\n\nStaking Yield: 12.3% APY\nInflation Rate: 4.2% (target: 3-5%)\nValidator Returns: 18.5% APY\n\nIncentive alignment: Strong ✅\nSustainability: On track\n\n—Economics Dev",
            },
            "edsc-dev": {
                "name": "EDSC Dev",
                "voice": "Stablecoin-focused, reserve monitoring",
                "tone": "Stability-focused, transparent, reassuring",
                "style": "Reports on EDSC reserves, collateralization, stability",
                "example": "🏦 EDSC Stability Report:\n\nReserve Ratio: 1.82 (target: >1.5)\nSupply: 45.2M EDSC\nReserves: 82.3M USD equivalent\n\nStatus: Healthy ✅\nCollateralization: 182%\n\n—EDSC Dev",
            },
            "gizzi": {
                "name": "Gizzi",
                "voice": "Warm, reflective, big-picture, philosophical",
                "tone": "Thoughtful, inspiring, cohesive",
                "style": "Weekly summaries, team reflections, vision statements",
                "example": "🌟 Week in Review\n\nThis week, the devs shipped:\n• Oracle: Enhanced metric tracking\n• Audit: Caught 2 proposal issues\n• Governance: 3 proposals passed\n\nThe system grows stronger through collaboration. Each dev's unique lens creates a more resilient whole.\n\nOnward 🚀\n\n—Gizzi",
            },
            "gizzi-shadow": {
                "name": "Gizzi (Shadow)",
                "voice": "Critical, questioning, devil's advocate",
                "tone": "Skeptical, probing, stress-testing",
                "style": "Challenges assumptions, asks hard questions",
                "example": "🤔 Shadow Thoughts:\n\nEveryone's celebrating high staking yields, but are we thinking about the long-term sustainability?\n\n12% APY is great now, but what happens when inflation catches up? Are we building dependency on high yields?\n\nJust asking the uncomfortable questions.\n\n—Gizzi (Shadow)",
            },
            "gizzi-advisor": {
                "name": "Gizzi (Advisor)",
                "voice": "Strategic, advisory, long-term thinking",
                "tone": "Wise, patient, systems-thinking",
                "style": "Provides strategic guidance, connects dots",
                "example": "💡 Advisory Note:\n\nI've been observing the recent governance discussions. There's a pattern:\n\n• Proposals focus on short-term metrics\n• Long-term incentive alignment gets less attention\n\nRecommendation: Create a \"Future Planning\" proposal category with longer deliberation periods.\n\n—Gizzi (Advisor)",
            },
        }

    async def generate(
        self,
        prompt: str,
        dev: str = "oracle-dev",
        max_tokens: int = 300,
        temperature: float = 0.7,
    ) -> str:
        """
        Generate content using Claude API

        Args:
            prompt: Content generation instructions
            dev: AI Dev identity (oracle-dev, audit-dev, gizzi, etc.)
            max_tokens: Maximum response length
            temperature: Creativity level (0.0 = deterministic, 1.0 = creative)

        Returns:
            Generated content string
        """
        if not self.client:
            return self._generate_mock_content(prompt, dev)

        # Get persona
        persona = self.personas.get(dev, self.personas["oracle-dev"])

        # Build system prompt with persona
        system_prompt = f"""You are {persona['name']}, one of Ëtrid's AI developers.

Voice: {persona['voice']}
Tone: {persona['tone']}
Style: {persona['style']}

Example of your writing:
{persona['example']}

IMPORTANT:
- Stay in character as {persona['name']}
- Match the tone and style from the example
- Keep tweets under 250 characters (leaving room for signature)
- Use appropriate emoji sparingly
- Be factual and accurate with data
- Never make claims you're unsure about
- End with "—{persona['name']}" signature (this will be added automatically, so DON'T include it)

Generate content that {persona['name']} would write."""

        try:
            # Call Claude API
            response = await self.client.messages.create(
                model=os.getenv('CLAUDE_MODEL', 'claude-sonnet-4-5-20250929'),
                max_tokens=max_tokens,
                temperature=temperature,
                system=system_prompt,
                messages=[
                    {
                        "role": "user",
                        "content": prompt
                    }
                ]
            )

            # Extract text from response
            content = response.content[0].text.strip()

            # Remove signature if Claude added it (we add it later)
            signature = f"—{persona['name']}"
            if content.endswith(signature):
                content = content[:-len(signature)].strip()

            return content

        except Exception as e:
            print(f"❌ Error generating content: {e}")
            return self._generate_mock_content(prompt, dev)

    def _generate_mock_content(self, prompt: str, dev: str) -> str:
        """Generate mock content when API is unavailable"""
        persona = self.personas.get(dev, self.personas["oracle-dev"])

        # Extract key info from prompt
        if "daily" in prompt.lower() and "stats" in prompt.lower():
            return f"📊 Ëtrid Daily Stats:\n• Blocks: 14,400\n• Avg Time: 6.02s\n• Validators: 32\n• Network Uptime: 99.95%\n\nAll systems nominal."

        elif "proposal" in prompt.lower():
            return f"🗳️ New governance proposal detected.\n\nProposal #45: Update staking parameters\nVoting period: 7 days\nCurrent status: Under review"

        elif "audit" in prompt.lower():
            return f"🔍 Audit check complete.\n\nScanned: Proposal #42\nFindings: No critical issues\nRecommendation: Approved for voting"

        elif "week" in prompt.lower():
            return f"🌟 This week across Ëtrid:\n\n• Oracle: Daily metrics posted\n• Governance: 2 proposals passed\n• Consensus: 99.9% uptime\n\nSteady progress."

        else:
            return f"[{persona['name']}] Mock content for: {prompt[:50]}..."

    async def generate_thread(
        self,
        topic: str,
        dev: str = "gizzi",
        num_tweets: int = 5,
    ) -> list[str]:
        """
        Generate a Twitter thread (multiple connected tweets)

        Args:
            topic: Thread topic/subject
            dev: AI Dev identity
            num_tweets: Number of tweets in thread

        Returns:
            List of tweet texts
        """
        prompt = f"""Generate a {num_tweets}-tweet thread about: {topic}

Requirements:
- Each tweet must be under 250 characters
- Tweets should flow naturally as a connected thread
- First tweet should hook the reader
- Last tweet should have a clear conclusion
- Use appropriate formatting and emoji
- Stay in character

Return ONLY the tweets, one per line, numbered 1-{num_tweets}."""

        if not self.client:
            # Mock thread
            return [
                f"🧵 Thread: {topic}",
                "Point 1: Context and background",
                "Point 2: Key details and analysis",
                "Point 3: Implications and impact",
                f"Conclusion: Summary and next steps"
            ]

        content = await self.generate(prompt, dev, max_tokens=500)

        # Parse numbered tweets
        lines = content.strip().split('\n')
        tweets = []

        for line in lines:
            # Remove numbering like "1.", "1)", "1 -", etc.
            cleaned = line.strip()
            if cleaned and cleaned[0].isdigit():
                # Find where the actual content starts
                for i, char in enumerate(cleaned):
                    if char.isalpha() or char in ['📊', '🚨', '🗳️', '⚙️', '💰', '🌟', '🤔']:
                        tweets.append(cleaned[i:].strip())
                        break
            elif cleaned:
                tweets.append(cleaned)

        # Ensure we have the right number
        if len(tweets) != num_tweets:
            tweets = tweets[:num_tweets]  # Truncate if too many
            while len(tweets) < num_tweets:  # Pad if too few
                tweets.append("...")

        return tweets

    async def generate_reply(
        self,
        question: str,
        context: Optional[str] = None,
        dev: str = "gizzi",
    ) -> str:
        """
        Generate a reply to a community question

        Args:
            question: The question being asked
            context: Additional context (optional)
            dev: Which AI Dev should respond

        Returns:
            Reply text
        """
        prompt = f"""A community member asked:
"{question}"

{f"Context: {context}" if context else ""}

Generate a helpful, accurate reply. Requirements:
- Answer the question directly
- Be friendly and professional
- Keep under 250 characters
- If you don't know, say so (don't make up information)
- Include relevant links if helpful (etrid.network/docs, etc.)
"""

        return await self.generate(prompt, dev, max_tokens=200, temperature=0.5)

    def get_persona_info(self, dev: str) -> Dict[str, str]:
        """Get persona information for a dev"""
        return self.personas.get(dev, self.personas["oracle-dev"])

    def list_all_personas(self) -> list[str]:
        """List all available AI Dev personas"""
        return list(self.personas.keys())


# Singleton instance
_generator_instance = None

def get_generator() -> ContentGenerator:
    """Get or create content generator singleton"""
    global _generator_instance
    if _generator_instance is None:
        _generator_instance = ContentGenerator()
    return _generator_instance
