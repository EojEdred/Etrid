#!/usr/bin/env python3
"""
Ollama Client for Ëtrid Validators
Provides easy interface for validators to query Ollama and Gizzi APIs
"""

import requests
import json
from typing import Dict, Optional, List


class OllamaClient:
    """Client for querying Ollama on Gizzi VM or locally"""

    def __init__(self, host: str = "http://64.181.215.19:11434", model: str = "llama3.1:8b"):
        """
        Initialize Ollama client

        Args:
            host: Ollama API endpoint (default: Gizzi VM centralized)
                  For local: http://localhost:11434
            model: Model to use (default: llama3.1:8b)
        """
        self.host = host
        self.model = model
        self.gizzi_api_base = "http://64.181.215.19:8080"

    def query(self, prompt: str, system_prompt: Optional[str] = None, temperature: float = 0.7) -> str:
        """
        Send a query to Ollama and get response

        Args:
            prompt: User prompt
            system_prompt: Optional system instructions
            temperature: Creativity (0.0 = deterministic, 1.0 = creative)

        Returns:
            Model response text
        """
        url = f"{self.host}/api/generate"

        payload = {
            "model": self.model,
            "prompt": prompt,
            "stream": False,
            "options": {
                "temperature": temperature
            }
        }

        if system_prompt:
            payload["system"] = system_prompt

        try:
            response = requests.post(url, json=payload, timeout=60)
            response.raise_for_status()
            result = response.json()
            return result.get("response", "")
        except requests.exceptions.Timeout:
            return "Error: Ollama query timed out (>60s)"
        except requests.exceptions.ConnectionError:
            return f"Error: Cannot connect to Ollama at {self.host}"
        except Exception as e:
            return f"Error querying Ollama: {str(e)}"

    def query_gizzi_api(self, endpoint: str, analysis_prompt: str) -> str:
        """
        Query a Gizzi API and ask Ollama to analyze the result

        Args:
            endpoint: Gizzi API endpoint (e.g., "/api/network/status")
            analysis_prompt: What to ask Ollama about the data

        Returns:
            Ollama's analysis of the API data
        """
        # Get data from Gizzi API
        try:
            api_url = f"{self.gizzi_api_base}{endpoint}"
            api_response = requests.get(api_url, timeout=10)
            api_response.raise_for_status()
            api_data = api_response.json()
        except Exception as e:
            return f"Error querying Gizzi API: {str(e)}"

        # Ask Ollama to analyze it
        full_prompt = f"""Here is data from the Gizzi network API (endpoint: {endpoint}):

{json.dumps(api_data, indent=2)}

{analysis_prompt}

Provide a clear, concise analysis."""

        return self.query(full_prompt)

    def analyze_validator_health(self, validator_metrics: Dict) -> Dict:
        """
        Analyze validator health metrics using Ollama

        Args:
            validator_metrics: Dict with keys like:
                - block_height
                - peers
                - finalized_height
                - is_syncing
                - etc.

        Returns:
            Dict with analysis:
                - health: "healthy" | "warning" | "critical"
                - issues: List of detected issues
                - recommendations: List of recommended actions
        """
        prompt = f"""You are analyzing blockchain validator health metrics.

Metrics:
{json.dumps(validator_metrics, indent=2)}

Analyze and respond ONLY with valid JSON in this exact format:
{{
  "health": "healthy|warning|critical",
  "issues": ["list", "of", "issues"],
  "recommendations": ["list", "of", "actions"]
}}

Health criteria:
- healthy: peers >= 5, blocks producing, finalization lag < 50
- warning: peers 2-4, OR finalization lag 50-100
- critical: peers < 2, OR offline, OR finalization lag > 100"""

        response = self.query(prompt, temperature=0.3)

        # Parse JSON response
        try:
            # Extract JSON from response (might have extra text)
            start = response.find("{")
            end = response.rfind("}") + 1
            if start >= 0 and end > start:
                json_str = response[start:end]
                return json.loads(json_str)
            else:
                raise ValueError("No JSON found in response")
        except Exception as e:
            # Fallback if JSON parsing fails
            return {
                "health": "unknown",
                "issues": [f"Failed to parse Ollama response: {str(e)}"],
                "recommendations": ["Contact Gizzi for manual analysis"]
            }

    def analyze_logs(self, log_lines: List[str], question: str = "Are there any errors or issues?") -> str:
        """
        Analyze validator logs using Ollama

        Args:
            log_lines: List of log lines (last 50-100 recommended)
            question: What to ask about the logs

        Returns:
            Ollama's analysis
        """
        logs_text = "\n".join(log_lines)

        prompt = f"""Analyze these blockchain validator logs:

{logs_text}

Question: {question}

Provide a concise analysis focusing on:
1. Any errors or warnings
2. Performance issues
3. Connectivity problems
4. Recommended actions"""

        return self.query(prompt, temperature=0.5)

    def ask_network_question(self, question: str) -> str:
        """
        Ask a general question about the network state

        Args:
            question: Natural language question

        Returns:
            Ollama's answer based on current network data
        """
        return self.query_gizzi_api(
            "/api/network/status",
            question
        )

    def compare_with_network(self, my_metrics: Dict) -> str:
        """
        Compare this validator's metrics with the network

        Args:
            my_metrics: This validator's current metrics

        Returns:
            Analysis of how this validator compares to others
        """
        return self.query_gizzi_api(
            "/api/network/status",
            f"""My validator metrics:
{json.dumps(my_metrics, indent=2)}

Compare my metrics to the network. Am I:
- Falling behind in block height?
- Low on peers compared to others?
- Having finalization issues?

Be specific about differences."""
        )

    def get_quick_health_summary(self, validator_id: int) -> str:
        """
        Get a quick health summary for a validator

        Args:
            validator_id: Validator number (1-21)

        Returns:
            One-sentence health summary
        """
        return self.query_gizzi_api(
            f"/api/validator/{validator_id}",
            "Summarize health in ONE sentence. Just say if it's healthy or what the problem is."
        )


# Helper functions for validators

def check_my_health(validator_id: int) -> str:
    """Quick health check for this validator"""
    client = OllamaClient()
    return client.get_quick_health_summary(validator_id)


def diagnose_issue(log_lines: List[str]) -> str:
    """Diagnose issue from logs"""
    client = OllamaClient()
    return client.analyze_logs(log_lines, "What's wrong and how do I fix it?")


def compare_to_network(my_block_height: int, my_peers: int, my_finalized: int) -> str:
    """Compare my metrics to the network"""
    client = OllamaClient()
    my_metrics = {
        "block_height": my_block_height,
        "peers": my_peers,
        "finalized_height": my_finalized
    }
    return client.compare_with_network(my_metrics)


def ask_gizzi(question: str) -> str:
    """Ask a question about the network"""
    client = OllamaClient()
    return client.ask_network_question(question)


# CLI interface for validators
if __name__ == "__main__":
    import sys
    import subprocess

    if len(sys.argv) < 2:
        print("""
Ollama Client for Ëtrid Validators

Usage:
    # Quick health check
    python3 ollama_client.py health <validator_id>

    # Analyze recent logs
    python3 ollama_client.py logs

    # Compare to network
    python3 ollama_client.py compare <block_height> <peers> <finalized_height>

    # Ask a question
    python3 ollama_client.py ask "your question here"

Examples:
    python3 ollama_client.py health 6
    python3 ollama_client.py logs
    python3 ollama_client.py compare 12345 8 12340
    python3 ollama_client.py ask "How many validators are online?"
""")
        sys.exit(1)

    command = sys.argv[1]

    if command == "health":
        if len(sys.argv) < 3:
            print("Usage: ollama_client.py health <validator_id>")
            sys.exit(1)
        validator_id = int(sys.argv[2])
        print(check_my_health(validator_id))

    elif command == "logs":
        # Get recent logs
        try:
            logs = subprocess.check_output(
                ['journalctl', '-u', 'flare-node', '-n', '100', '--no-pager'],
                stderr=subprocess.STDOUT
            ).decode()
            log_lines = logs.split('\n')
            print(diagnose_issue(log_lines))
        except Exception as e:
            print(f"Error reading logs: {e}")

    elif command == "compare":
        if len(sys.argv) < 5:
            print("Usage: ollama_client.py compare <block_height> <peers> <finalized_height>")
            sys.exit(1)
        block_height = int(sys.argv[2])
        peers = int(sys.argv[3])
        finalized = int(sys.argv[4])
        print(compare_to_network(block_height, peers, finalized))

    elif command == "ask":
        if len(sys.argv) < 3:
            print("Usage: ollama_client.py ask 'your question'")
            sys.exit(1)
        question = " ".join(sys.argv[2:])
        print(ask_gizzi(question))

    else:
        print(f"Unknown command: {command}")
        sys.exit(1)
