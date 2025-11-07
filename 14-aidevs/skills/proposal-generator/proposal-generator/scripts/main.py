#!/usr/bin/env python3
"""
Governance AI Skill - Proposal Generation and Analysis
Agent: Governance AI (did:etrid:<validator>:governance)

This skill autonomously:
1. Monitors chain state for governance needs (treasury balance, runtime version, etc.)
2. Generates well-formatted governance proposals using LLM
3. Simulates voting outcomes before submission
4. Tracks proposal lifecycle and voting results
5. Reports governance activity on-chain
"""

import asyncio
import json
from datetime import datetime, timedelta
from typing import Dict, Any, Optional, List
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s'
)
logger = logging.getLogger(__name__)


class GovernanceAI:
    """Autonomous governance proposal generator"""

    def __init__(self, config: Dict[str, Any]):
        self.blockchain_rpc = config.get('blockchain_rpc', 'ws://localhost:9944')
        self.llm_backend = config.get('llm_backend', 'ollama')
        self.validator_did = config.get('validator_did')
        self.substrate = None

    async def connect_to_chain(self):
        """Connect to blockchain RPC"""
        try:
            from substrateinterface import SubstrateInterface

            self.substrate = SubstrateInterface(url=self.blockchain_rpc)
            logger.info(f"✅ Connected to FlareChain: {self.blockchain_rpc}")

            # Get chain info
            chain = self.substrate.chain
            version = self.substrate.runtime_version
            logger.info(f"Chain: {chain}, Runtime: {version}")

        except Exception as e:
            logger.error(f"Failed to connect to chain: {e}")
            raise

    async def analyze_governance_needs(self) -> Dict[str, Any]:
        """
        Analyze chain state to identify governance needs
        Returns list of potential proposals to create
        """
        try:
            logger.info("🔍 Analyzing governance needs...")

            needs = {
                'treasury_spending': await self._check_treasury_status(),
                'runtime_upgrade': await self._check_runtime_upgrade_needed(),
                'validator_changes': await self._check_validator_set(),
                'parameter_changes': await self._check_parameter_tuning()
            }

            proposals_needed = []

            # Check if treasury needs spending
            if needs['treasury_spending']['needs_action']:
                proposals_needed.append({
                    'type': 'treasury',
                    'priority': 'high',
                    'reason': needs['treasury_spending']['reason']
                })

            # Check if runtime upgrade needed
            if needs['runtime_upgrade']['needs_action']:
                proposals_needed.append({
                    'type': 'runtime_upgrade',
                    'priority': 'critical',
                    'reason': needs['runtime_upgrade']['reason']
                })

            # Check validator changes
            if needs['validator_changes']['needs_action']:
                proposals_needed.append({
                    'type': 'validator_set',
                    'priority': 'medium',
                    'reason': needs['validator_changes']['reason']
                })

            logger.info(f"Found {len(proposals_needed)} governance needs")

            return {
                'proposals_needed': proposals_needed,
                'chain_state': needs
            }

        except Exception as e:
            logger.error(f"Error analyzing governance needs: {e}")
            return {'proposals_needed': [], 'error': str(e)}

    async def _check_treasury_status(self) -> Dict[str, Any]:
        """Check if treasury has excess funds that should be spent"""
        try:
            # Query treasury balance
            treasury_account = self.substrate.query(
                module='Treasury',
                storage_function='TreasuryAccount'
            )

            # TODO: Get actual treasury balance
            # For now, mock data
            treasury_balance = 1_000_000_000_000  # 1M ETR

            threshold = 500_000_000_000  # 500K ETR

            if treasury_balance > threshold:
                return {
                    'needs_action': True,
                    'reason': f'Treasury has {treasury_balance / 1e12:.2f}M ETR, above threshold of {threshold / 1e12:.2f}M',
                    'balance': treasury_balance
                }
            else:
                return {
                    'needs_action': False,
                    'balance': treasury_balance
                }

        except Exception as e:
            logger.warning(f"Could not check treasury: {e}")
            return {'needs_action': False, 'error': str(e)}

    async def _check_runtime_upgrade_needed(self) -> Dict[str, Any]:
        """Check if runtime upgrade is available"""
        try:
            current_version = self.substrate.runtime_version

            # TODO: Check GitHub for new runtime releases
            # For now, mock check
            return {'needs_action': False}

        except Exception as e:
            return {'needs_action': False, 'error': str(e)}

    async def _check_validator_set(self) -> Dict[str, Any]:
        """Check if validator set needs changes"""
        try:
            validators = self.substrate.query(
                module='Session',
                storage_function='Validators'
            )

            # Check if we have expected number (21)
            validator_count = len(validators.value) if validators else 0

            if validator_count < 21:
                return {
                    'needs_action': True,
                    'reason': f'Only {validator_count}/21 validators active',
                    'current_count': validator_count
                }
            else:
                return {
                    'needs_action': False,
                    'current_count': validator_count
                }

        except Exception as e:
            return {'needs_action': False, 'error': str(e)}

    async def _check_parameter_tuning(self) -> Dict[str, Any]:
        """Check if chain parameters need tuning"""
        try:
            # TODO: Check various chain parameters
            # - Block time
            # - Epoch duration
            # - Slashing parameters
            # - Fee parameters

            return {'needs_action': False}

        except Exception as e:
            return {'needs_action': False, 'error': str(e)}

    async def generate_proposal(self, proposal_type: str, context: Dict[str, Any]) -> Dict[str, Any]:
        """
        Use LLM to generate a well-formatted governance proposal
        """
        try:
            logger.info(f"📝 Generating {proposal_type} proposal...")

            # Create prompt for LLM
            prompt = self._create_proposal_prompt(proposal_type, context)

            # Call LLM
            proposal_text = await self._call_llm(prompt)

            # Parse and structure proposal
            proposal = {
                'type': proposal_type,
                'title': proposal_text.get('title'),
                'description': proposal_text.get('description'),
                'rationale': proposal_text.get('rationale'),
                'implementation': proposal_text.get('implementation'),
                'timeline': proposal_text.get('timeline'),
                'generated_at': datetime.now().isoformat(),
                'generated_by': f'{self.validator_did}:governance'
            }

            logger.info(f"✅ Generated proposal: {proposal['title']}")

            return proposal

        except Exception as e:
            logger.error(f"Error generating proposal: {e}")
            return {'error': str(e)}

    def _create_proposal_prompt(self, proposal_type: str, context: Dict[str, Any]) -> str:
        """Create LLM prompt for proposal generation"""

        if proposal_type == 'treasury':
            return f"""You are a governance expert for FlareChain blockchain. Generate a treasury spending proposal.

Context:
- Treasury Balance: {context.get('balance', 0) / 1e12:.2f}M ETR
- Threshold: 500K ETR
- Reason: {context.get('reason')}

Create a detailed treasury proposal that includes:
1. Title (concise, < 100 chars)
2. Description (2-3 paragraphs explaining the proposal)
3. Rationale (why this spending is needed)
4. Implementation (how funds will be distributed)
5. Timeline (expected duration)

Suggested spending categories:
- Development grants
- Marketing and community growth
- Infrastructure improvements
- Security audits
- Ecosystem partnerships

Format as JSON:
{{
  "title": "...",
  "description": "...",
  "rationale": "...",
  "implementation": "...",
  "timeline": "..."
}}
"""

        elif proposal_type == 'validator_set':
            return f"""You are a governance expert for FlareChain. Generate a proposal to address validator set issues.

Context:
- Current validators: {context.get('current_count', 0)}
- Expected validators: 21
- Reason: {context.get('reason')}

Create a proposal to recruit or activate missing validators.

Format as JSON with title, description, rationale, implementation, timeline.
"""

        elif proposal_type == 'runtime_upgrade':
            return f"""Generate a runtime upgrade proposal for FlareChain.

Context: {json.dumps(context, indent=2)}

Include upgrade details, testing plan, and rollback strategy.

Format as JSON with title, description, rationale, implementation, timeline.
"""

        else:
            return f"""Generate a governance proposal for FlareChain.

Type: {proposal_type}
Context: {json.dumps(context, indent=2)}

Format as JSON with title, description, rationale, implementation, timeline.
"""

    async def simulate_voting(self, proposal: Dict[str, Any]) -> Dict[str, Any]:
        """
        Simulate likely voting outcome based on validator preferences and chain state
        """
        try:
            logger.info("🗳️  Simulating voting outcome...")

            # Get current validators
            validators = self.substrate.query(
                module='Session',
                storage_function='Validators'
            )

            validator_count = len(validators.value) if validators else 21

            # Use LLM to predict voting outcome
            prompt = f"""You are analyzing a FlareChain governance proposal voting simulation.

Proposal:
Title: {proposal['title']}
Type: {proposal['type']}
Description: {proposal['description']}

Validator Info:
- Total validators: {validator_count}
- Voting threshold: 50% + 1

Analyze this proposal and predict:
1. How many validators would likely vote YES
2. How many would likely vote NO
3. How many might abstain
4. Overall likelihood of passing (0-100%)
5. Key concerns that might cause NO votes

Respond in JSON:
{{
  "predicted_yes": <number>,
  "predicted_no": <number>,
  "predicted_abstain": <number>,
  "pass_likelihood": <percentage>,
  "concerns": ["...", "..."],
  "recommendation": "submit|revise|reject"
}}
"""

            simulation_result = await self._call_llm(prompt)

            logger.info(f"✅ Voting simulation: {simulation_result.get('pass_likelihood', 0)}% likely to pass")

            return simulation_result

        except Exception as e:
            logger.error(f"Error simulating voting: {e}")
            return {'error': str(e)}

    async def submit_proposal(self, proposal: Dict[str, Any]) -> Dict[str, Any]:
        """Submit proposal to blockchain"""
        try:
            logger.info(f"📤 Submitting proposal: {proposal['title']}")

            # TODO: Implement actual on-chain proposal submission
            # This would call governance pallet extrinsic

            # For now, log the proposal
            logger.info(f"""
Proposal Ready for Submission:
==============================
Title: {proposal['title']}
Type: {proposal['type']}
Description: {proposal['description'][:200]}...
Generated by: {proposal['generated_by']}
""")

            return {
                'success': True,
                'proposal_id': 'mock_id_123',  # Would be real proposal ID from chain
                'submitted_at': datetime.now().isoformat()
            }

        except Exception as e:
            logger.error(f"Error submitting proposal: {e}")
            return {'success': False, 'error': str(e)}

    async def _call_llm(self, prompt: str) -> Dict[str, Any]:
        """Call LLM backend"""
        try:
            if self.llm_backend == 'ollama':
                return await self._call_ollama(prompt)
            elif self.llm_backend == 'claude':
                return await self._call_claude(prompt)
            elif self.llm_backend == 'openai':
                return await self._call_openai(prompt)
            else:
                raise ValueError(f"Unknown LLM backend: {self.llm_backend}")

        except Exception as e:
            logger.error(f"LLM call failed: {e}")
            return {'error': str(e)}

    async def _call_ollama(self, prompt: str) -> Dict[str, Any]:
        """Call local Ollama"""
        try:
            import httpx

            async with httpx.AsyncClient() as client:
                response = await client.post(
                    'http://localhost:11434/api/generate',
                    json={
                        'model': 'llama3',
                        'prompt': prompt,
                        'stream': False
                    },
                    timeout=120.0
                )

                result = response.json()
                response_text = result.get('response', '')

                # Try to parse JSON
                try:
                    if '```json' in response_text:
                        json_start = response_text.find('```json') + 7
                        json_end = response_text.find('```', json_start)
                        json_str = response_text[json_start:json_end].strip()
                    else:
                        json_str = response_text

                    return json.loads(json_str)

                except json.JSONDecodeError:
                    return {'raw_response': response_text}

        except Exception as e:
            raise Exception(f"Ollama call failed: {e}")

    async def _call_claude(self, prompt: str) -> Dict[str, Any]:
        """Call Claude API"""
        try:
            import anthropic
            import os

            client = anthropic.Anthropic(
                api_key=os.environ.get('ANTHROPIC_API_KEY')
            )

            message = client.messages.create(
                model="claude-sonnet-4-5-20250929",
                max_tokens=2048,
                messages=[{"role": "user", "content": prompt}]
            )

            response_text = message.content[0].text

            # Parse JSON
            try:
                if '```json' in response_text:
                    json_start = response_text.find('```json') + 7
                    json_end = response_text.find('```', json_start)
                    json_str = response_text[json_start:json_end].strip()
                else:
                    json_str = response_text

                return json.loads(json_str)

            except json.JSONDecodeError:
                return {'raw_response': response_text}

        except Exception as e:
            raise Exception(f"Claude API call failed: {e}")

    async def _call_openai(self, prompt: str) -> Dict[str, Any]:
        """Call OpenAI GPT-4"""
        try:
            import openai
            import os

            openai.api_key = os.environ.get('OPENAI_API_KEY')

            response = await openai.ChatCompletion.acreate(
                model="gpt-4",
                messages=[
                    {"role": "system", "content": "You are a blockchain governance expert."},
                    {"role": "user", "content": prompt}
                ],
                max_tokens=2048
            )

            response_text = response.choices[0].message.content

            # Parse JSON
            try:
                if '```json' in response_text:
                    json_start = response_text.find('```json') + 7
                    json_end = response_text.find('```', json_start)
                    json_str = response_text[json_start:json_end].strip()
                else:
                    json_str = response_text

                return json.loads(json_str)

            except json.JSONDecodeError:
                return {'raw_response': response_text}

        except Exception as e:
            raise Exception(f"OpenAI API call failed: {e}")


async def execute(context: Dict[str, Any], parameters: Dict[str, Any]) -> Dict[str, Any]:
    """
    Main execution function called by AI Devs Orchestrator

    Args:
        context: Execution context (validator info, LLM config, etc.)
        parameters: Skill-specific parameters
          - mode: 'analyze' | 'generate' | 'simulate' | 'submit'
          - proposal_type: 'treasury' | 'runtime_upgrade' | 'validator_set'

    Returns:
        Result dictionary with success status and details
    """
    try:
        logger.info("🚀 Governance AI skill starting...")

        # Initialize governance AI
        gov_ai = GovernanceAI(context)

        # Connect to chain
        await gov_ai.connect_to_chain()

        mode = parameters.get('mode', 'analyze')

        if mode == 'analyze':
            # Analyze what proposals are needed
            result = await gov_ai.analyze_governance_needs()

            return {
                'success': True,
                'action': 'analyze',
                'result': result
            }

        elif mode == 'generate':
            # Generate a specific proposal
            proposal_type = parameters.get('proposal_type', 'treasury')
            proposal_context = parameters.get('context', {})

            proposal = await gov_ai.generate_proposal(proposal_type, proposal_context)

            return {
                'success': True,
                'action': 'generate',
                'proposal': proposal
            }

        elif mode == 'simulate':
            # Simulate voting for a proposal
            proposal = parameters.get('proposal')

            if not proposal:
                return {'success': False, 'error': 'No proposal provided for simulation'}

            simulation = await gov_ai.simulate_voting(proposal)

            return {
                'success': True,
                'action': 'simulate',
                'simulation': simulation
            }

        elif mode == 'submit':
            # Submit proposal to chain
            proposal = parameters.get('proposal')

            if not proposal:
                return {'success': False, 'error': 'No proposal provided for submission'}

            submission = await gov_ai.submit_proposal(proposal)

            return {
                'success': submission['success'],
                'action': 'submit',
                'result': submission
            }

        else:
            return {'success': False, 'error': f'Unknown mode: {mode}'}

    except Exception as e:
        logger.error(f"Governance AI skill failed: {e}")
        return {
            'success': False,
            'error': str(e)
        }


if __name__ == '__main__':
    # Test execution
    context = {
        'blockchain_rpc': 'ws://localhost:9944',
        'llm_backend': 'ollama',
        'validator_did': 'did:etrid:director-gizzi'
    }

    # Test 1: Analyze governance needs
    print("=== Test 1: Analyze Governance Needs ===")
    parameters = {'mode': 'analyze'}
    result = asyncio.run(execute(context, parameters))
    print(json.dumps(result, indent=2))

    # Test 2: Generate treasury proposal
    print("\n=== Test 2: Generate Treasury Proposal ===")
    parameters = {
        'mode': 'generate',
        'proposal_type': 'treasury',
        'context': {
            'balance': 1_000_000_000_000,
            'reason': 'Treasury above 500K ETR threshold'
        }
    }
    result = asyncio.run(execute(context, parameters))
    print(json.dumps(result, indent=2))
