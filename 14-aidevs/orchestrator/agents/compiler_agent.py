"""
Compiler AI - Handles compilation, testing, and error debugging
"""

import logging
from .base_agent import BaseAgent
import asyncio

logger = logging.getLogger(__name__)

class CompilerAgent(BaseAgent):
    """AI agent responsible for compilation and testing"""

    async def _start_monitoring(self):
        """Monitor for compilation triggers"""
        logger.info("[Compiler AI] Monitoring for compilation events...")

        # Start background task to watch for git changes
        asyncio.create_task(self._watch_for_changes())

    async def _watch_for_changes(self):
        """Watch for code changes that trigger compilation"""
        while self.started:
            # TODO: Implement git webhook or filesystem watcher
            # For now, this is a placeholder
            await asyncio.sleep(60)  # Check every minute

    async def auto_compile(self):
        """Automatically compile the workspace"""
        logger.info("[Compiler AI] Auto-compiling workspace...")

        result = await self.execute_skill('etrid-compile-build')

        if not result.get('success'):
            # If compilation failed, try to debug errors
            logger.warning("[Compiler AI] Compilation failed, attempting to debug...")
            await self.execute_skill('error-debugging', {
                'errors': result.get('error', '')
            })

        return result

    async def run_tests(self):
        """Run integration tests"""
        logger.info("[Compiler AI] Running tests...")
        return await self.execute_skill('integration-test')
