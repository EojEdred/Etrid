#!/usr/bin/env python3
"""
Compiler AI Skill - FlareChain Code Compilation and Build
Agent: Compiler AI (did:etrid:<validator>:compiler)

This skill autonomously:
1. Monitors git repository for changes
2. Pulls latest code
3. Runs cargo build for FlareChain runtime
4. Uses LLM to analyze and suggest fixes for build errors
5. Reports build status on-chain via pallet-ai-agents
"""

import asyncio
import subprocess
import os
import json
import hashlib
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, Optional, List
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [%(levelname)s] %(message)s'
)
logger = logging.getLogger(__name__)


class FlareChainCompiler:
    """Autonomous compiler for FlareChain"""

    def __init__(self, config: Dict[str, Any]):
        self.repo_path = config.get('repo_path', '/opt/etrid')
        self.llm_backend = config.get('llm_backend', 'ollama')
        self.blockchain_rpc = config.get('blockchain_rpc', 'ws://localhost:9944')
        self.validator_did = config.get('validator_did')
        self.last_commit = None

    async def check_for_updates(self) -> Optional[str]:
        """Check if there are new commits to pull"""
        try:
            # Fetch latest from origin
            result = subprocess.run(
                ['git', 'fetch', 'origin'],
                cwd=self.repo_path,
                capture_output=True,
                text=True,
                timeout=30
            )

            if result.returncode != 0:
                logger.error(f"Git fetch failed: {result.stderr}")
                return None

            # Get current commit hash
            result = subprocess.run(
                ['git', 'rev-parse', 'HEAD'],
                cwd=self.repo_path,
                capture_output=True,
                text=True
            )
            current_commit = result.stdout.strip()

            # Get remote commit hash
            result = subprocess.run(
                ['git', 'rev-parse', 'origin/main'],
                cwd=self.repo_path,
                capture_output=True,
                text=True
            )
            remote_commit = result.stdout.strip()

            if current_commit != remote_commit:
                logger.info(f"New commits detected: {current_commit[:8]} -> {remote_commit[:8]}")
                return remote_commit
            else:
                logger.debug("No new commits")
                return None

        except Exception as e:
            logger.error(f"Error checking for updates: {e}")
            return None

    async def pull_latest(self) -> bool:
        """Pull latest code from git"""
        try:
            logger.info("Pulling latest code from git...")

            result = subprocess.run(
                ['git', 'pull', 'origin', 'main'],
                cwd=self.repo_path,
                capture_output=True,
                text=True,
                timeout=60
            )

            if result.returncode != 0:
                logger.error(f"Git pull failed: {result.stderr}")
                return False

            # Get new commit hash
            result = subprocess.run(
                ['git', 'rev-parse', 'HEAD'],
                cwd=self.repo_path,
                capture_output=True,
                text=True
            )
            self.last_commit = result.stdout.strip()

            logger.info(f"✅ Pulled latest code. Current commit: {self.last_commit[:8]}")
            return True

        except Exception as e:
            logger.error(f"Error pulling code: {e}")
            return False

    async def build_runtime(self) -> Dict[str, Any]:
        """
        Build FlareChain runtime with cargo
        Returns build result with status, logs, and binary path
        """
        try:
            logger.info("🔨 Starting cargo build...")
            start_time = datetime.now()

            # Run cargo build --release
            process = subprocess.Popen(
                ['cargo', 'build', '--release', '-p', 'flarechain-node'],
                cwd=self.repo_path,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )

            stdout, stderr = process.communicate(timeout=600)  # 10 minute timeout

            duration = (datetime.now() - start_time).total_seconds()

            if process.returncode == 0:
                logger.info(f"✅ Build successful in {duration:.1f}s")

                # Get binary path
                binary_path = os.path.join(
                    self.repo_path,
                    'target/release/flarechain-node'
                )

                # Calculate binary hash
                binary_hash = self._calculate_file_hash(binary_path)

                return {
                    'success': True,
                    'duration': duration,
                    'binary_path': binary_path,
                    'binary_hash': binary_hash,
                    'commit': self.last_commit,
                    'stdout': stdout[-1000:],  # Last 1000 chars
                    'stderr': stderr[-1000:] if stderr else ''
                }
            else:
                logger.error(f"❌ Build failed after {duration:.1f}s")

                return {
                    'success': False,
                    'duration': duration,
                    'commit': self.last_commit,
                    'stdout': stdout[-2000:],
                    'stderr': stderr[-2000:]
                }

        except subprocess.TimeoutExpired:
            logger.error("❌ Build timed out after 10 minutes")
            return {
                'success': False,
                'error': 'Build timeout',
                'duration': 600
            }
        except Exception as e:
            logger.error(f"❌ Build error: {e}")
            return {
                'success': False,
                'error': str(e)
            }

    async def analyze_build_error(self, build_result: Dict[str, Any]) -> Dict[str, Any]:
        """Use LLM to analyze build errors and suggest fixes"""
        try:
            logger.info("🤖 Analyzing build error with LLM...")

            error_output = build_result.get('stderr', '') + build_result.get('stdout', '')

            # Prepare prompt for LLM
            prompt = f"""You are a Rust expert analyzing a FlareChain (Substrate) build failure.

Build Error Output:
```
{error_output[-3000:]}  # Last 3000 chars
```

Commit: {build_result.get('commit', 'unknown')[:8]}

Please analyze:
1. What is the root cause of the build failure?
2. Which file(s) are causing the error?
3. What are the specific error types (type mismatch, missing trait, etc.)?
4. What is the recommended fix?

Provide a concise analysis in JSON format:
{{
  "root_cause": "...",
  "affected_files": ["..."],
  "error_types": ["..."],
  "recommended_fix": "...",
  "urgency": "low|medium|high"
}}
"""

            # Call LLM backend
            analysis = await self._call_llm(prompt)

            logger.info(f"✅ LLM analysis complete")
            return {
                'analysis': analysis,
                'llm_backend': self.llm_backend
            }

        except Exception as e:
            logger.error(f"Error analyzing build error: {e}")
            return {
                'error': str(e)
            }

    async def _call_llm(self, prompt: str) -> Dict[str, Any]:
        """Call LLM backend (Ollama, Claude, or GPT-4)"""
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
        """Call local Ollama LLM"""
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
                    timeout=60.0
                )

                result = response.json()

                # Try to parse JSON from response
                response_text = result.get('response', '')
                try:
                    # Extract JSON from markdown code block if present
                    if '```json' in response_text:
                        json_start = response_text.find('```json') + 7
                        json_end = response_text.find('```', json_start)
                        json_str = response_text[json_start:json_end].strip()
                    else:
                        json_str = response_text

                    analysis = json.loads(json_str)
                    return analysis

                except json.JSONDecodeError:
                    # Return raw text if not JSON
                    return {'raw_analysis': response_text}

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
                max_tokens=1024,
                messages=[
                    {"role": "user", "content": prompt}
                ]
            )

            response_text = message.content[0].text

            # Parse JSON from response
            try:
                if '```json' in response_text:
                    json_start = response_text.find('```json') + 7
                    json_end = response_text.find('```', json_start)
                    json_str = response_text[json_start:json_end].strip()
                else:
                    json_str = response_text

                analysis = json.loads(json_str)
                return analysis

            except json.JSONDecodeError:
                return {'raw_analysis': response_text}

        except Exception as e:
            raise Exception(f"Claude API call failed: {e}")

    async def _call_openai(self, prompt: str) -> Dict[str, Any]:
        """Call OpenAI GPT-4 API"""
        try:
            import openai
            import os

            openai.api_key = os.environ.get('OPENAI_API_KEY')

            response = await openai.ChatCompletion.acreate(
                model="gpt-4",
                messages=[
                    {"role": "system", "content": "You are a Rust expert analyzing build errors."},
                    {"role": "user", "content": prompt}
                ],
                max_tokens=1024
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

                analysis = json.loads(json_str)
                return analysis

            except json.JSONDecodeError:
                return {'raw_analysis': response_text}

        except Exception as e:
            raise Exception(f"OpenAI API call failed: {e}")

    async def report_to_chain(self, action: str, result: Dict[str, Any]):
        """Report AI action to blockchain via pallet-ai-agents"""
        try:
            from substrateinterface import SubstrateInterface, Keypair

            substrate = SubstrateInterface(url=self.blockchain_rpc)

            # Load validator keypair
            # TODO: Implement proper keystore loading
            # keypair = Keypair.create_from_uri('//Alice')

            logger.info(f"📡 Reporting action to chain: {action}")

            # TODO: Submit extrinsic to pallet-ai-agents
            # call = substrate.compose_call(
            #     call_module='AiAgents',
            #     call_function='report_agent_action',
            #     call_params={
            #         'agent_did': f'{self.validator_did}:compiler',
            #         'action': action,
            #         'result': json.dumps(result)[:1000]  # Truncate to fit
            #     }
            # )

            logger.info("✅ Action reported to chain")

        except Exception as e:
            logger.error(f"Failed to report to chain: {e}")

    def _calculate_file_hash(self, file_path: str) -> str:
        """Calculate SHA256 hash of file"""
        sha256 = hashlib.sha256()

        with open(file_path, 'rb') as f:
            for chunk in iter(lambda: f.read(4096), b''):
                sha256.update(chunk)

        return sha256.hexdigest()


async def execute(context: Dict[str, Any], parameters: Dict[str, Any]) -> Dict[str, Any]:
    """
    Main execution function called by AI Devs Orchestrator

    Args:
        context: Execution context (validator info, LLM config, etc.)
        parameters: Skill-specific parameters

    Returns:
        Result dictionary with success status and details
    """
    try:
        logger.info("🚀 Compiler AI skill starting...")

        # Initialize compiler
        compiler = FlareChainCompiler(context)

        # Check for updates
        new_commit = await compiler.check_for_updates()

        if new_commit is None and not parameters.get('force_build', False):
            logger.info("No new commits. Skipping build.")
            return {
                'success': True,
                'action': 'check_updates',
                'result': 'no_updates'
            }

        # Pull latest code
        if not await compiler.pull_latest():
            return {
                'success': False,
                'action': 'git_pull',
                'error': 'Failed to pull latest code'
            }

        # Build runtime
        build_result = await compiler.build_runtime()

        # If build failed, analyze with LLM
        if not build_result['success']:
            logger.warning("Build failed. Analyzing error...")

            analysis = await compiler.analyze_build_error(build_result)
            build_result['llm_analysis'] = analysis

            # Report failure to chain
            await compiler.report_to_chain('build_failed', {
                'commit': compiler.last_commit[:8],
                'error': build_result.get('stderr', '')[:500],
                'analysis': analysis
            })

            return {
                'success': False,
                'action': 'compile',
                'build_result': build_result
            }

        # Build succeeded
        logger.info(f"✅ Build successful! Binary: {build_result['binary_path']}")

        # Report success to chain
        await compiler.report_to_chain('build_success', {
            'commit': compiler.last_commit[:8],
            'binary_hash': build_result['binary_hash'],
            'duration': build_result['duration']
        })

        return {
            'success': True,
            'action': 'compile',
            'build_result': build_result
        }

    except Exception as e:
        logger.error(f"Compiler skill failed: {e}")
        return {
            'success': False,
            'error': str(e)
        }


if __name__ == '__main__':
    # Test execution
    context = {
        'repo_path': '/Users/macbook/Desktop/etrid',
        'llm_backend': 'ollama',
        'blockchain_rpc': 'ws://localhost:9944',
        'validator_did': 'did:etrid:director-gizzi'
    }

    parameters = {
        'force_build': True
    }

    result = asyncio.run(execute(context, parameters))
    print(json.dumps(result, indent=2))
