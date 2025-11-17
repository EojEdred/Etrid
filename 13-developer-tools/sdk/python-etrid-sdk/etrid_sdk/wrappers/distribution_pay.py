"""Distribution Pay Wrapper - Daily Rewards Distribution"""

from typing import Dict, Any
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, DistributionError


class DistributionPayWrapper:
    """Wrapper for Distribution Pay pallet - 27,397 ÉTR daily rewards."""
    
    def __init__(self, api: SubstrateInterface):
        self.api = api
        
    def _ensure_connected(self):
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()
            
    async def claim_reward(self, keypair: Keypair, category: str) -> str:
        """Claim reward for given category."""
        self._ensure_connected()
        # Implementation here
        pass
        
    async def get_pending_rewards(self, address: str) -> Dict[str, Any]:
        """Get pending rewards by category."""
        self._ensure_connected()
        # Implementation here
        pass
