"""Oracle Wrapper - Price Feeds"""

from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, OracleError


class OracleWrapper:
    """Wrapper for Oracle pallet - Decentralized price feeds."""
    
    def __init__(self, api: SubstrateInterface):
        self.api = api
        
    def _ensure_connected(self):
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()
            
    async def get_price(self, pair: str) -> int:
        """Get current price for trading pair."""
        self._ensure_connected()
        # Implementation here
        pass
