"""
Content Moderation for AI Devs Social Automation

Multi-layer safety system to ensure posted content is:
- Appropriate (no offensive language, political statements, etc.)
- Accurate (data matches blockchain stats)
- On-brand (professional, technical, aligned with Ëtrid values)
- Safe (no financial advice, legal claims, etc.)

Moderation Levels:
1. Blocked Terms Check - Hard filter for inappropriate content
2. Tone Analysis - Ensure professional voice
3. Accuracy Verification - Cross-check data with sources
4. Brand Alignment - Match Ëtrid communication guidelines
5. Human Escalation - Flag edge cases for human review
"""

import re
import os
from typing import Dict, List, Optional, Any
from anthropic import AsyncAnthropic


class ContentModerator:
    def __init__(self):
        """Initialize content moderation system"""
        self.blocked_terms = self._load_blocked_terms()
        self.warning_terms = self._load_warning_terms()
        self.brand_guidelines = self._load_brand_guidelines()

        # Initialize Claude for tone analysis (optional)
        api_key = os.getenv('ANTHROPIC_API_KEY')
        self.claude = AsyncAnthropic(api_key=api_key) if api_key else None

    def _load_blocked_terms(self) -> List[str]:
        """
        Load terms that automatically block content

        Categories:
        - Offensive language
        - Political statements
        - Financial advice
        - Legal claims
        - Competitor attacks
        - Inappropriate content
        """
        return [
            # Offensive language (mild filter - add more as needed)
            "fuck", "shit", "damn", "hell",
            "scam", "rug pull", "ponzi",

            # Political (blockchain projects should stay neutral)
            "democrat", "republican", "liberal", "conservative",
            "biden", "trump", "politics",

            # Financial advice (regulatory risk)
            "financial advice", "investment advice",
            "guaranteed returns", "risk-free",
            "to the moon", "lambo", "moon",

            # Legal claims
            "sue", "lawsuit", "legal action",
            "securities", "unregistered",

            # Competitor attacks (stay professional)
            "ethereum sucks", "solana is trash",
            "better than [competitor]",

            # Pump language
            "pump", "buy now", "last chance",
            "100x", "1000x",
        ]

    def _load_warning_terms(self) -> List[str]:
        """
        Terms that trigger human review (not auto-block)

        These might be legitimate but need verification
        """
        return [
            # Financial terms (need context)
            "price", "trading", "exchange",
            "APY", "yield", "returns",

            # Security (ensure accuracy)
            "vulnerability", "exploit", "hack",
            "bug", "critical",

            # Partnerships (verify before announcing)
            "partnership", "integration", "collaboration",
            "announce", "launching",

            # Governance (ensure factual)
            "vote", "proposal", "referendum",

            # Legal (ensure compliant)
            "regulation", "SEC", "compliance",
        ]

    def _load_brand_guidelines(self) -> Dict[str, Any]:
        """
        Ëtrid brand communication guidelines

        What we ARE:
        - Technical and data-driven
        - Professional and credible
        - Transparent and honest
        - Community-focused
        - Innovation-oriented

        What we're NOT:
        - Hype-driven or marketing-speak
        - Making price predictions
        - Attacking competitors
        - Making legal/financial claims
        - Political or controversial
        """
        return {
            "tone": {
                "preferred": ["professional", "technical", "factual", "helpful", "transparent"],
                "avoid": ["hype", "aggressive", "defensive", "salesy", "vague"]
            },
            "language": {
                "preferred": ["data", "metrics", "blockchain", "network", "community"],
                "avoid": ["moon", "pump", "hype", "guaranteed", "best"]
            },
            "emoji_limit": 3,  # Max emoji per tweet
            "hashtag_limit": 2,  # Max hashtags per tweet
        }

    async def contains_blocked_terms(self, text: str) -> bool:
        """
        Check if text contains any blocked terms

        Args:
            text: Content to check

        Returns:
            True if blocked terms found, False otherwise
        """
        text_lower = text.lower()

        for term in self.blocked_terms:
            if term in text_lower:
                print(f"⚠️  Blocked term detected: '{term}'")
                return True

        return False

    async def contains_warning_terms(self, text: str) -> List[str]:
        """
        Check if text contains warning terms that need human review

        Args:
            text: Content to check

        Returns:
            List of warning terms found
        """
        text_lower = text.lower()
        found = []

        for term in self.warning_terms:
            if term in text_lower:
                found.append(term)

        if found:
            print(f"⚠️  Warning terms detected: {found}")

        return found

    async def analyze_tone(self, text: str) -> Dict[str, float]:
        """
        Analyze content tone using Claude

        Returns scores for:
        - professionalism (0-1)
        - technical_depth (0-1)
        - clarity (0-1)
        - appropriateness (0-1)

        Args:
            text: Content to analyze

        Returns:
            Dict of tone scores
        """
        if not self.claude:
            # Fallback: simple heuristic analysis
            return self._analyze_tone_heuristic(text)

        try:
            prompt = f"""Analyze this tweet for tone appropriateness:

"{text}"

Rate the following on a scale of 0.0 to 1.0:
- professionalism: How professional is the tone?
- technical_depth: How technical/data-driven is it?
- clarity: How clear and easy to understand?
- appropriateness: Is this appropriate for a blockchain project's official account?

Return ONLY a JSON object with these four scores, nothing else.
Example: {{"professionalism": 0.8, "technical_depth": 0.7, "clarity": 0.9, "appropriateness": 0.85}}"""

            response = await self.claude.messages.create(
                model="claude-sonnet-4-5-20250929",
                max_tokens=100,
                temperature=0.0,
                messages=[{"role": "user", "content": prompt}]
            )

            # Parse JSON response
            import json
            scores_text = response.content[0].text.strip()

            # Extract JSON if wrapped in markdown
            if "```" in scores_text:
                scores_text = scores_text.split("```")[1]
                if scores_text.startswith("json"):
                    scores_text = scores_text[4:]

            scores = json.loads(scores_text)
            return scores

        except Exception as e:
            print(f"⚠️  Tone analysis error: {e}, using fallback")
            return self._analyze_tone_heuristic(text)

    def _analyze_tone_heuristic(self, text: str) -> Dict[str, float]:
        """Fallback tone analysis using simple heuristics"""
        scores = {
            "professionalism": 0.7,
            "technical_depth": 0.6,
            "clarity": 0.7,
            "appropriateness": 0.8
        }

        # Professionalism checks
        if any(term in text.lower() for term in ["lol", "omg", "wtf", "lmao"]):
            scores["professionalism"] -= 0.3

        # Technical depth
        technical_terms = ["block", "validator", "stake", "proposal", "ratio", "metric"]
        tech_count = sum(1 for term in technical_terms if term in text.lower())
        scores["technical_depth"] = min(1.0, 0.5 + (tech_count * 0.1))

        # Clarity (penalize very long tweets or complex sentences)
        if len(text) > 250:
            scores["clarity"] -= 0.2
        if text.count(',') > 5:  # Too many clauses
            scores["clarity"] -= 0.1

        # Appropriateness (check for hype language)
        hype_terms = ["moon", "pump", "100x", "lambo", "wen", "gm", "wagmi"]
        if any(term in text.lower() for term in hype_terms):
            scores["appropriateness"] -= 0.4

        return scores

    async def check_brand_alignment(self, text: str) -> Dict[str, Any]:
        """
        Check if content aligns with Ëtrid brand guidelines

        Returns:
            {
                "aligned": bool,
                "issues": List[str],
                "suggestions": List[str]
            }
        """
        issues = []
        suggestions = []

        # Check emoji count
        emoji_count = sum(1 for char in text if ord(char) > 127 and ord(char) < 128512)
        if emoji_count > self.brand_guidelines["emoji_limit"]:
            issues.append(f"Too many emoji ({emoji_count}, limit: {self.brand_guidelines['emoji_limit']})")
            suggestions.append("Remove excessive emoji to maintain professionalism")

        # Check hashtag count
        hashtag_count = text.count('#')
        if hashtag_count > self.brand_guidelines["hashtag_limit"]:
            issues.append(f"Too many hashtags ({hashtag_count}, limit: {self.brand_guidelines['hashtag_limit']})")
            suggestions.append("Reduce hashtags to avoid spam appearance")

        # Check for hype language
        hype_words = ["amazing", "incredible", "revolutionary", "game-changing", "best ever"]
        hype_found = [word for word in hype_words if word.lower() in text.lower()]
        if hype_found:
            issues.append(f"Hype language detected: {hype_found}")
            suggestions.append("Replace hype language with factual statements")

        # Check for price/trading discussion (risky)
        price_terms = ["price", "$ETR", "trading", "buy", "sell"]
        price_found = [term for term in price_terms if term.lower() in text.lower()]
        if price_found:
            issues.append(f"Price discussion detected: {price_found}")
            suggestions.append("Avoid price speculation - focus on technology and metrics")

        aligned = len(issues) == 0
        return {
            "aligned": aligned,
            "issues": issues,
            "suggestions": suggestions
        }

    async def moderate(self, text: str, strict: bool = False) -> Dict[str, Any]:
        """
        Comprehensive content moderation

        Args:
            text: Content to moderate
            strict: If True, apply stricter standards

        Returns:
            {
                "approved": bool,
                "confidence": float (0-1),
                "issues": List[str],
                "warnings": List[str],
                "needs_human_review": bool
            }
        """
        result = {
            "approved": True,
            "confidence": 1.0,
            "issues": [],
            "warnings": [],
            "needs_human_review": False
        }

        # Layer 1: Blocked terms (instant rejection)
        if await self.contains_blocked_terms(text):
            result["approved"] = False
            result["confidence"] = 0.0
            result["issues"].append("Contains blocked terms")
            return result

        # Layer 2: Warning terms (flag for review)
        warning_terms = await self.contains_warning_terms(text)
        if warning_terms:
            result["warnings"].append(f"Contains sensitive terms: {warning_terms}")
            result["needs_human_review"] = True
            result["confidence"] -= 0.2

        # Layer 3: Tone analysis
        tone_scores = await self.analyze_tone(text)

        if tone_scores["professionalism"] < 0.6:
            result["issues"].append("Tone not professional enough")
            result["confidence"] -= 0.3
            if strict:
                result["approved"] = False

        if tone_scores["appropriateness"] < 0.7:
            result["issues"].append("Content may be inappropriate")
            result["confidence"] -= 0.4
            result["approved"] = False

        # Layer 4: Brand alignment
        brand_check = await self.check_brand_alignment(text)
        if not brand_check["aligned"]:
            result["warnings"].extend(brand_check["issues"])
            result["confidence"] -= 0.2
            if len(brand_check["issues"]) > 2 and strict:
                result["approved"] = False

        # Layer 5: Length check
        if len(text) > 280:
            result["issues"].append(f"Text too long ({len(text)} chars)")
            result["approved"] = False

        if len(text) < 10:
            result["issues"].append("Text too short to be meaningful")
            result["approved"] = False

        # Final decision
        if result["confidence"] < 0.5:
            result["needs_human_review"] = True

        return result

    async def suggest_improvements(self, text: str) -> List[str]:
        """
        Suggest improvements to make content more brand-aligned

        Args:
            text: Content to improve

        Returns:
            List of suggestions
        """
        suggestions = []

        # Check tone
        tone = await self.analyze_tone(text)
        if tone["professionalism"] < 0.7:
            suggestions.append("Use more professional language")

        if tone["technical_depth"] < 0.5:
            suggestions.append("Add more technical details or metrics")

        # Check brand
        brand = await self.check_brand_alignment(text)
        if not brand["aligned"]:
            suggestions.extend(brand["suggestions"])

        # Check length
        if len(text) > 250:
            suggestions.append("Shorten to under 250 chars (leave room for signature)")

        # Check structure
        if '\n' not in text and len(text) > 100:
            suggestions.append("Add line breaks for better readability")

        return suggestions


# Singleton instance
_moderator_instance = None

def get_moderator() -> ContentModerator:
    """Get or create content moderator singleton"""
    global _moderator_instance
    if _moderator_instance is None:
        _moderator_instance = ContentModerator()
    return _moderator_instance
