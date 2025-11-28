"""
Base Agent - Abstract base class for all AI Dev agents
"""

import logging
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional
from datetime import datetime
import asyncio

logger = logging.getLogger(__name__)

class BaseAgent(ABC):
    """Base class for all AI Dev agents"""

    def __init__(self, config: Dict[str, Any], skills_loader, vectordb, blockchain):
        self.config = config
        self.skills_loader = skills_loader
        self.vectordb = vectordb
        self.blockchain = blockchain

        self.did = config['did']
        self.description = config['description']
        self.skills = config['skills']
        self.priority = config['priority']

        self.started = False
        self.metrics = {
            "executions_total": 0,
            "executions_success": 0,
            "executions_failed": 0,
            "last_execution": None
        }

    async def start(self):
        """Start the agent"""
        self.started = True
        logger.info(f"Agent {self.__class__.__name__} started (DID: {self.did})")

        # Start agent-specific monitoring tasks
        await self._start_monitoring()

    async def stop(self):
        """Stop the agent"""
        self.started = False
        logger.info(f"Agent {self.__class__.__name__} stopped")

    @abstractmethod
    async def _start_monitoring(self):
        """Start agent-specific monitoring (implemented by subclasses)"""
        pass

    async def execute_skill(self, skill_name: str, parameters: Dict[str, Any] = None) -> Dict[str, Any]:
        """Execute a skill"""
        if skill_name not in self.skills:
            raise ValueError(f"Skill '{skill_name}' not available to this agent")

        logger.info(f"[{self.__class__.__name__}] Executing skill: {skill_name}")

        # Update metrics
        self.metrics["executions_total"] += 1
        self.metrics["last_execution"] = datetime.utcnow().isoformat()

        try:
            # Execute the skill
            result = await self.skills_loader.execute_skill(
                name=skill_name,
                parameters=parameters
            )

            # Update metrics
            if result.get('success'):
                self.metrics["executions_success"] += 1
            else:
                self.metrics["executions_failed"] += 1

            return result

        except Exception as e:
            logger.error(f"Skill execution error: {str(e)}")
            self.metrics["executions_failed"] += 1
            return {
                "success": False,
                "error": str(e)
            }

    async def get_status(self) -> Dict[str, Any]:
        """Get agent status"""
        return {
            "did": self.did,
            "started": self.started,
            "skills_count": len(self.skills),
            "priority": self.priority
        }

    async def get_metrics(self) -> Dict[str, Any]:
        """Get agent metrics"""
        return self.metrics
