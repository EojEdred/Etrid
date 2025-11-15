"""
Ëtrid AI Compute Network - Python SDK

Install:
    pip install etrid-ai

Usage:
    from etrid_ai import AICompute

    client = AICompute(api_key="your_telegram_user_id")

    # Submit AI inference job
    result = client.run(
        model="gpt-4",
        prompt="Write a haiku about blockchain",
        max_tokens=50
    )
    print(result.output)
"""

import asyncio
import json
from typing import Optional, Dict, Any
from dataclasses import dataclass
import websockets
from substrateinterface import SubstrateInterface, Keypair

@dataclass
class JobResult:
    """AI job result"""
    job_id: int
    output: str
    cost: float  # in ËDSC
    compute_time: float  # in seconds
    gpu_model: str
    status: str

class AICompute:
    """Ëtrid AI Compute Network client"""

    def __init__(
        self,
        api_key: Optional[str] = None,
        ws_url: str = "wss://ai-compute-pbc.etrid.network",
        auto_payment: bool = True,
    ):
        """
        Initialize AI Compute client

        Args:
            api_key: Your Telegram user ID or Ëtrid account
            ws_url: WebSocket URL for AI-Compute-PBC
            auto_payment: Auto-pay from balance (default: True)
        """
        self.api_key = api_key
        self.ws_url = ws_url
        self.auto_payment = auto_payment
        self.substrate = None
        self.account = None

    async def connect(self):
        """Connect to blockchain"""
        self.substrate = SubstrateInterface(url=self.ws_url)

        # Create account from API key (or load from keystore)
        if self.api_key:
            # For demo: derive keypair from API key
            # In production: use proper key management
            self.account = Keypair.create_from_mnemonic(self.api_key)

    async def run(
        self,
        model: str,
        prompt: str,
        max_tokens: int = 100,
        priority: str = "standard",  # economy|standard|premium
        payment: Optional[float] = None,
    ) -> JobResult:
        """
        Run AI inference job

        Args:
            model: Model name (e.g., "gpt-4", "stable-diffusion")
            prompt: Input prompt
            max_tokens: Maximum output tokens
            priority: Job priority tier
            payment: Manual payment amount (or auto-calculated)

        Returns:
            JobResult with output and metadata
        """
        if not self.substrate:
            await self.connect()

        # 1. Lookup model ID from model-registry pallet
        model_id = await self._lookup_model(model)

        # 2. Calculate payment (if not provided)
        if payment is None:
            payment = self._estimate_cost(model, max_tokens, priority)

        # 3. Submit job to job-marketplace pallet
        job_id = await self._submit_job(
            model_id=model_id,
            input_data=prompt,
            max_tokens=max_tokens,
            payment=payment,
            priority=priority,
        )

        # 4. Wait for job completion
        result = await self._wait_for_result(job_id)

        return result

    async def _lookup_model(self, model_name: str) -> int:
        """Lookup model ID from registry"""
        # Query ModelByName storage map
        result = await self.substrate.query(
            module="ModelRegistry",
            storage_function="ModelByName",
            params=[model_name.encode()],
        )

        if result and len(result) > 0:
            return result[0]  # Return first version
        else:
            raise ValueError(f"Model '{model_name}' not found in registry")

    def _estimate_cost(self, model: str, max_tokens: int, priority: str) -> float:
        """Estimate job cost in ËDSC"""
        # Base prices (ËDSC per 1K tokens)
        base_prices = {
            "gpt-4": 0.03,
            "gpt-3.5": 0.001,
            "claude": 0.025,
            "stable-diffusion": 0.01,
            "whisper": 0.006,
        }

        # Priority multipliers
        priority_multipliers = {
            "economy": 1.0,
            "standard": 1.5,
            "premium": 3.0,
        }

        base_price = base_prices.get(model, 0.01)
        multiplier = priority_multipliers.get(priority, 1.5)

        cost = (base_price * max_tokens / 1000) * multiplier
        return round(cost, 6)

    async def _submit_job(
        self,
        model_id: int,
        input_data: str,
        max_tokens: int,
        payment: float,
        priority: str,
    ) -> int:
        """Submit job to blockchain"""
        # Prepare extrinsic call
        call = self.substrate.compose_call(
            call_module="JobMarketplace",
            call_function="submit_job",
            call_params={
                "model_id": model_id,
                "model_type": "LLM",  # Auto-detect from model registry
                "input_hash": self._hash_input(input_data),
                "output_format": b"json",
                "max_compute_time": max_tokens * 2,  # Rough estimate
                "payment": int(payment * 10**18),  # Convert to smallest unit
                "priority": priority.capitalize(),
            },
        )

        # Sign and submit
        extrinsic = self.substrate.create_signed_extrinsic(
            call=call,
            keypair=self.account,
        )

        receipt = await self.substrate.submit_extrinsic(
            extrinsic,
            wait_for_inclusion=True,
        )

        # Extract job ID from events
        job_id = self._extract_job_id(receipt)
        return job_id

    async def _wait_for_result(self, job_id: int, timeout: int = 300) -> JobResult:
        """Wait for job completion and fetch result"""
        start_time = asyncio.get_event_loop().time()

        while True:
            # Query job status
            job = await self.substrate.query(
                module="JobMarketplace",
                storage_function="Jobs",
                params=[job_id],
            )

            if job:
                status = str(job['status'])

                if status == "Completed":
                    # Fetch result from off-chain storage (IPFS/Arweave)
                    result_hash = job['result_hash']
                    output = await self._fetch_result_data(result_hash)

                    return JobResult(
                        job_id=job_id,
                        output=output,
                        cost=float(job['payment']) / 10**18,
                        compute_time=float(job['completed_at'] - job['submitted_at']),
                        gpu_model="RTX 4090",  # TODO: Fetch from GPU registry
                        status="Completed",
                    )

                elif status == "Failed":
                    raise RuntimeError(f"Job {job_id} failed")

            # Check timeout
            if asyncio.get_event_loop().time() - start_time > timeout:
                raise TimeoutError(f"Job {job_id} timed out after {timeout}s")

            # Wait 2 seconds before next poll
            await asyncio.sleep(2)

    def _hash_input(self, data: str) -> bytes:
        """Hash input data (BLAKE2-256)"""
        import hashlib
        return hashlib.blake2b(data.encode(), digest_size=32).digest()

    def _extract_job_id(self, receipt) -> int:
        """Extract job ID from extrinsic receipt"""
        for event in receipt.triggered_events:
            if event.value['module_id'] == 'JobMarketplace' and \
               event.value['event_id'] == 'JobSubmitted':
                return event.value['attributes']['job_id']
        raise RuntimeError("Job ID not found in receipt")

    async def _fetch_result_data(self, result_hash: bytes) -> str:
        """Fetch result data from IPFS/Arweave"""
        # TODO: Implement IPFS fetch
        # For demo, return placeholder
        return "This is the AI model output (fetched from IPFS)"

    def list_models(self) -> list:
        """List available AI models"""
        # Query total models from ModelRegistry
        total = self.substrate.query(
            module="ModelRegistry",
            storage_function="TotalModels",
        )

        models = []
        for i in range(total):
            model = self.substrate.query(
                module="ModelRegistry",
                storage_function="Models",
                params=[i],
            )
            if model:
                models.append({
                    "id": i,
                    "name": model['name'].decode(),
                    "version": model['version'].decode(),
                    "aidid": model['aidid'].decode(),
                    "royalty": model['royalty_bps'] / 100,  # Convert to %
                })

        return models

    async def get_gpu_stats(self) -> Dict[str, Any]:
        """Get network statistics"""
        active_gpus = await self.substrate.query(
            module="GpuRegistry",
            storage_function="ActiveGpuCount",
        )

        total_jobs = await self.substrate.query(
            module="JobMarketplace",
            storage_function="TotalJobs",
        )

        return {
            "active_gpus": active_gpus,
            "total_jobs": total_jobs,
            "network": "AI-Compute-PBC",
            "status": "online",
        }


# Async wrapper for easier usage
class AIComputeSync(AICompute):
    """Synchronous wrapper for AICompute"""

    def run(self, *args, **kwargs) -> JobResult:
        """Synchronous run method"""
        return asyncio.run(super().run(*args, **kwargs))

    def get_gpu_stats(self) -> Dict[str, Any]:
        """Synchronous stats method"""
        return asyncio.run(super().get_gpu_stats())


# Example usage
if __name__ == "__main__":
    # Initialize client
    client = AIComputeSync(api_key="your_telegram_user_id")

    # Run GPT-4 inference
    result = client.run(
        model="gpt-4",
        prompt="Write a haiku about decentralized AI",
        max_tokens=50,
        priority="standard",
    )

    print(f"Output: {result.output}")
    print(f"Cost: ${result.cost} ËDSC")
    print(f"Compute Time: {result.compute_time}s")
    print(f"GPU: {result.gpu_model}")

    # List available models
    models = client.list_models()
    print(f"\nAvailable models: {len(models)}")
    for model in models[:5]:
        print(f"  - {model['name']} ({model['version']}) - {model['aidid']}")

    # Get network stats
    stats = client.get_gpu_stats()
    print(f"\nNetwork Stats:")
    print(f"  Active GPUs: {stats['active_gpus']}")
    print(f"  Total Jobs: {stats['total_jobs']}")
