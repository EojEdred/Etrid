#!/usr/bin/env python3
"""
Multi-AI Router for Ëtrid Validator Monitoring
Intelligently routes queries to Ollama, GPT-4, or Claude based on task type
"""

import os
import time
import json
import requests
from enum import Enum
from typing import Dict, Tuple, Optional

# Optional imports (install as needed)
try:
    import openai
    OPENAI_AVAILABLE = True
except ImportError:
    OPENAI_AVAILABLE = False
    print("Warning: openai not installed. GPT models unavailable. Install: pip install openai")

try:
    import anthropic
    ANTHROPIC_AVAILABLE = True
except ImportError:
    ANTHROPIC_AVAILABLE = False
    print("Warning: anthropic not installed. Claude unavailable. Install: pip install anthropic")


class AIModel(Enum):
    """Available AI models"""
    OLLAMA = "ollama"
    GPT4_TURBO = "gpt4-turbo"
    GPT4 = "gpt4"
    CLAUDE = "claude"


class AIRouter:
    """
    Intelligent router that selects the best AI model for each task

    Usage:
        router = AIRouter()
        result = router.query("Is validator 6 healthy?")
        print(f"Response: {result['response']}")
        print(f"Cost: ${result['cost']:.4f}")
    """

    def __init__(self,
                 openai_api_key: Optional[str] = None,
                 anthropic_api_key: Optional[str] = None,
                 ollama_url: str = "http://localhost:11434",
                 default_model: AIModel = AIModel.OLLAMA):
        """
        Initialize AI Router

        Args:
            openai_api_key: OpenAI API key (or set OPENAI_API_KEY env var)
            anthropic_api_key: Anthropic API key (or set ANTHROPIC_API_KEY env var)
            ollama_url: Ollama server URL
            default_model: Default model if routing fails
        """
        self.ollama_url = ollama_url
        self.default_model = default_model

        # Initialize OpenAI
        if OPENAI_AVAILABLE:
            api_key = openai_api_key or os.getenv('OPENAI_API_KEY')
            if api_key:
                self.openai_client = openai.OpenAI(api_key=api_key)
            else:
                self.openai_client = None
                print("Warning: No OpenAI API key found. GPT models unavailable.")
        else:
            self.openai_client = None

        # Initialize Anthropic
        if ANTHROPIC_AVAILABLE:
            api_key = anthropic_api_key or os.getenv('ANTHROPIC_API_KEY')
            if api_key:
                self.anthropic_client = anthropic.Anthropic(api_key=api_key)
            else:
                self.anthropic_client = None
                print("Warning: No Anthropic API key found. Claude unavailable.")
        else:
            self.anthropic_client = None

    def route_query(self, query: str, context: Optional[Dict] = None) -> Tuple[AIModel, str]:
        """
        Intelligently route query to best AI model

        Args:
            query: The query text
            context: Optional context dict with keys:
                - is_critical: bool
                - query_type: str
                - force_model: AIModel

        Returns:
            (selected_model, reasoning)
        """
        context = context or {}

        # Allow forcing specific model
        if 'force_model' in context:
            return context['force_model'], "User forced model selection"

        query_lower = query.lower()

        # Quick health checks → Ollama (free, fast)
        if any(word in query_lower for word in ['health', 'status', 'online', 'peers', 'quick']):
            return AIModel.OLLAMA, "Simple health check, use free Ollama"

        # Code analysis → GPT-4 Turbo (best at code, cost-effective)
        if any(word in query_lower for word in ['code', 'function', 'bug', 'debug', 'rust', 'compile', 'syntax']):
            if self.openai_client:
                return AIModel.GPT4_TURBO, "Code analysis, GPT-4 Turbo excels here"
            else:
                return AIModel.CLAUDE, "Code analysis, Claude as GPT unavailable"

        # Math/calculations → GPT-4 (better at math)
        if any(word in query_lower for word in ['calculate', 'sum', 'average', 'statistics', 'count', 'percentage']):
            if self.openai_client:
                return AIModel.GPT4_TURBO, "Mathematical reasoning, GPT-4 Turbo efficient"
            else:
                return AIModel.CLAUDE, "Math, Claude as GPT unavailable"

        # Governance/ethics/complex decisions → Claude (superior reasoning)
        if any(word in query_lower for word in ['governance', 'decision', 'should', 'ethics', 'recommend', 'strategy']):
            if self.anthropic_client:
                return AIModel.CLAUDE, "Requires careful reasoning, use Claude"
            else:
                return AIModel.GPT4_TURBO, "Complex reasoning, GPT as Claude unavailable"

        # Critical decisions → Claude (most careful)
        if context.get('is_critical', False):
            if self.anthropic_client:
                return AIModel.CLAUDE, "Critical decision requires Claude's careful analysis"
            else:
                return AIModel.GPT4_TURBO, "Critical decision, GPT as Claude unavailable"

        # Log analysis → Ollama or GPT-4 Turbo depending on complexity
        if 'log' in query_lower or 'error' in query_lower:
            if len(query) > 1000:
                # Long logs, use GPT-4 Turbo for better context
                return AIModel.GPT4_TURBO, "Long log analysis, GPT-4 Turbo handles context well"
            else:
                return AIModel.OLLAMA, "Short log analysis, Ollama sufficient"

        # Default: Ollama (free, fast enough for most queries)
        return AIModel.OLLAMA, "General query, use free Ollama"

    def query(self, prompt: str, context: Optional[Dict] = None, system_prompt: Optional[str] = None) -> Dict:
        """
        Query the appropriate AI model

        Args:
            prompt: User prompt
            context: Optional routing context
            system_prompt: Optional system instructions

        Returns:
            {
                "model": "gpt4-turbo",
                "response": "...",
                "cost": 0.005,
                "time": 0.8,
                "tokens": {"input": 100, "output": 50}
            }
        """
        context = context or {}
        selected_model, reasoning = self.route_query(prompt, context)

        print(f"[AI Router] Selected {selected_model.value}: {reasoning}")

        # Route to appropriate model
        if selected_model == AIModel.OLLAMA:
            return self._query_ollama(prompt, system_prompt)
        elif selected_model in [AIModel.GPT4, AIModel.GPT4_TURBO]:
            if self.openai_client:
                return self._query_gpt(prompt, selected_model, system_prompt)
            else:
                print("[AI Router] GPT unavailable, falling back to Ollama")
                return self._query_ollama(prompt, system_prompt)
        elif selected_model == AIModel.CLAUDE:
            if self.anthropic_client:
                return self._query_claude(prompt, system_prompt)
            else:
                print("[AI Router] Claude unavailable, falling back to Ollama")
                return self._query_ollama(prompt, system_prompt)
        else:
            raise ValueError(f"Unsupported model: {selected_model}")

    def _query_ollama(self, prompt: str, system_prompt: Optional[str] = None) -> Dict:
        """Query Ollama (free, local)"""
        start = time.time()

        payload = {
            "model": "llama3.1:8b",
            "prompt": prompt,
            "stream": False
        }

        if system_prompt:
            payload["system"] = system_prompt

        try:
            response = requests.post(
                f"{self.ollama_url}/api/generate",
                json=payload,
                timeout=60
            )
            response.raise_for_status()
            result = response.json()

            elapsed = time.time() - start

            return {
                "model": "ollama-llama3.1:8b",
                "response": result.get("response", ""),
                "cost": 0.0,
                "time": elapsed,
                "tokens": {
                    "input": result.get("prompt_eval_count", 0),
                    "output": result.get("eval_count", 0)
                }
            }
        except Exception as e:
            return {
                "model": "ollama-llama3.1:8b",
                "response": f"Error querying Ollama: {str(e)}",
                "cost": 0.0,
                "time": time.time() - start,
                "tokens": {"input": 0, "output": 0},
                "error": str(e)
            }

    def _query_gpt(self, prompt: str, model: AIModel, system_prompt: Optional[str] = None) -> Dict:
        """Query GPT-4 or GPT-4 Turbo"""
        start = time.time()

        model_name = "gpt-4-turbo-preview" if model == AIModel.GPT4_TURBO else "gpt-4"

        messages = []
        if system_prompt:
            messages.append({"role": "system", "content": system_prompt})
        messages.append({"role": "user", "content": prompt})

        try:
            response = self.openai_client.chat.completions.create(
                model=model_name,
                messages=messages,
                temperature=0.7,
                max_tokens=2000
            )

            elapsed = time.time() - start

            # Calculate cost
            # GPT-4 Turbo: $10/1M input, $30/1M output
            # GPT-4: $30/1M input, $60/1M output
            if model == AIModel.GPT4_TURBO:
                input_cost = response.usage.prompt_tokens * 10 / 1_000_000
                output_cost = response.usage.completion_tokens * 30 / 1_000_000
            else:
                input_cost = response.usage.prompt_tokens * 30 / 1_000_000
                output_cost = response.usage.completion_tokens * 60 / 1_000_000

            total_cost = input_cost + output_cost

            return {
                "model": model_name,
                "response": response.choices[0].message.content,
                "cost": total_cost,
                "time": elapsed,
                "tokens": {
                    "input": response.usage.prompt_tokens,
                    "output": response.usage.completion_tokens
                }
            }
        except Exception as e:
            return {
                "model": model_name,
                "response": f"Error querying GPT: {str(e)}",
                "cost": 0.0,
                "time": time.time() - start,
                "tokens": {"input": 0, "output": 0},
                "error": str(e)
            }

    def _query_claude(self, prompt: str, system_prompt: Optional[str] = None) -> Dict:
        """Query Claude API"""
        start = time.time()

        try:
            message = self.anthropic_client.messages.create(
                model="claude-sonnet-4-20250514",
                max_tokens=2048,
                system=system_prompt if system_prompt else "You are a helpful AI assistant.",
                messages=[{"role": "user", "content": prompt}]
            )

            elapsed = time.time() - start

            # Calculate cost
            # Claude Sonnet 4: $3/1M input, $15/1M output
            input_tokens = message.usage.input_tokens
            output_tokens = message.usage.output_tokens

            input_cost = input_tokens * 3 / 1_000_000
            output_cost = output_tokens * 15 / 1_000_000
            total_cost = input_cost + output_cost

            return {
                "model": "claude-sonnet-4",
                "response": message.content[0].text,
                "cost": total_cost,
                "time": elapsed,
                "tokens": {
                    "input": input_tokens,
                    "output": output_tokens
                }
            }
        except Exception as e:
            return {
                "model": "claude-sonnet-4",
                "response": f"Error querying Claude: {str(e)}",
                "cost": 0.0,
                "time": time.time() - start,
                "tokens": {"input": 0, "output": 0},
                "error": str(e)
            }

    def compare_models(self, prompt: str) -> Dict:
        """
        Query all available models and compare results

        Returns dict with results from each model
        """
        results = {}

        # Always try Ollama
        results['ollama'] = self._query_ollama(prompt)

        # Try GPT if available
        if self.openai_client:
            results['gpt4_turbo'] = self._query_gpt(prompt, AIModel.GPT4_TURBO)

        # Try Claude if available
        if self.anthropic_client:
            results['claude'] = self._query_claude(prompt)

        # Calculate totals
        total_cost = sum(r['cost'] for r in results.values())
        avg_time = sum(r['time'] for r in results.values()) / len(results)

        return {
            "results": results,
            "total_cost": total_cost,
            "avg_time": avg_time
        }


# Example usage and testing
if __name__ == "__main__":
    import sys

    print("=" * 60)
    print("AI Router - Multi-Model Query System")
    print("=" * 60)

    router = AIRouter()

    # Test queries
    test_queries = [
        ("Is validator 6 healthy?", None),
        ("Analyze this Rust code: fn add(a: i32, b: i32) -> i32 { a + b }", None),
        ("Should we restart validator 6 with 2 peers and 150 block lag?", {"is_critical": True}),
        ("Calculate the average block height across 21 validators.", None),
    ]

    total_cost = 0.0

    for query, context in test_queries:
        print(f"\n{'='*60}")
        print(f"Query: {query[:80]}...")
        print(f"{'='*60}")

        result = router.query(query, context=context)

        print(f"Model: {result['model']}")
        print(f"Time: {result['time']:.2f}s")
        print(f"Cost: ${result['cost']:.5f}")
        print(f"Tokens: {result['tokens']['input']} in, {result['tokens']['output']} out")
        print(f"\nResponse:\n{result['response'][:200]}...")

        total_cost += result['cost']

    print(f"\n{'='*60}")
    print(f"Total Cost: ${total_cost:.5f}")
    print(f"{'='*60}")

    # Interactive mode
    if len(sys.argv) > 1 and sys.argv[1] == "interactive":
        print("\nInteractive Mode - Type 'exit' to quit")
        while True:
            query = input("\nQuery: ")
            if query.lower() in ['exit', 'quit']:
                break

            result = router.query(query)
            print(f"\n[{result['model']}] ${result['cost']:.5f}, {result['time']:.2f}s")
            print(f"{result['response']}")
