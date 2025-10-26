"""RPC client for connecting to Ëtrid blockchain nodes"""

from typing import Optional
from substrateinterface import SubstrateInterface
from .types import Balance

class EtridClient:
    """Ëtrid blockchain client"""

    def __init__(self, endpoint: str):
        """
        Create a new client instance

        Args:
            endpoint: WebSocket endpoint (e.g., 'ws://localhost:9944')
        """
        self.endpoint = endpoint
        self.substrate: Optional[SubstrateInterface] = None

    async def __aenter__(self):
        """Async context manager entry"""
        await self.connect()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit"""
        await self.disconnect()

    async def connect(self):
        """Connect to the blockchain"""
        self.substrate = SubstrateInterface(url=self.endpoint)

    async def disconnect(self):
        """Disconnect from the blockchain"""
        if self.substrate:
            self.substrate.close()

    async def get_block_number(self) -> int:
        """Get the current block number"""
        if not self.substrate:
            raise RuntimeError("Not connected")
        result = self.substrate.get_block_number(None)
        return result

    async def get_balance(self, address: str) -> Balance:
        """
        Get account balance

        Args:
            address: Account address (SS58 format)

        Returns:
            Balance information
        """
        if not self.substrate:
            raise RuntimeError("Not connected")

        result = self.substrate.query("System", "Account", [address])
        data = result.value['data']

        return Balance(
            free=data['free'],
            reserved=data['reserved'],
            frozen=data.get('frozen', 0)
        )

    async def get_chain_name(self) -> str:
        """Get the chain name"""
        if not self.substrate:
            raise RuntimeError("Not connected")
        return self.substrate.chain

    @property
    def query(self):
        """Query interface"""
        return self
