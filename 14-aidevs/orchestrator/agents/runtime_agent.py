"""
Runtime AI - Handles runtime upgrades and node management
"""

import logging
from .base_agent import BaseAgent
import asyncio

logger = logging.getLogger(__name__)

class RuntimeAgent(BaseAgent):
    """AI agent responsible for runtime operations"""

    async def _start_monitoring(self):
        """Monitor runtime and node health"""
        logger.info("[Runtime AI] Monitoring runtime health...")

        # Start background task to monitor node
        asyncio.create_task(self._monitor_node_health())

    async def _monitor_node_health(self):
        """Monitor blockchain node health"""
        while self.started:
            try:
                health = await self.blockchain.health_check()

                if health.get('status') != 'healthy':
                    logger.warning(f"[Runtime AI] Node unhealthy: {health}")

                await asyncio.sleep(60)  # Check every minute
            except Exception as e:
                logger.error(f"Error monitoring node health: {str(e)}")
                await asyncio.sleep(60)

    async def upgrade_runtime(self, wasm_path: str):
        """Perform runtime upgrade"""
        logger.info(f"[Runtime AI] Upgrading runtime with {wasm_path}...")

        return await self.execute_skill('runtime-upgrade', {
            'wasm_path': wasm_path
        })

    async def launch_node(self, chain: str):
        """Launch a blockchain node"""
        logger.info(f"[Runtime AI] Launching {chain} node...")

        return await self.execute_skill('node-launcher', {
            'chain': chain
        })
