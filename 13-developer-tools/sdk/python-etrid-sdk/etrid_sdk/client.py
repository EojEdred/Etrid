"""
Ëtrid Client - Main API Connection
"""

from typing import Optional
from substrateinterface import SubstrateInterface, Keypair


class EtridClient:
    """
    Main client for connecting to Ëtrid blockchain.
    
    Example:
        >>> client = EtridClient("wss://rpc.etrid.io")
        >>> print(client.api.chain)
        'Etrid'
    """
    
    def __init__(
        self,
        url: str = "ws://127.0.0.1:9944",
        type_registry_preset: str = "substrate-node-template",
        auto_discover: bool = True,
    ):
        """
        Initialize Ëtrid client.
        
        Args:
            url: WebSocket URL of Ëtrid node
            type_registry_preset: Type registry preset to use
            auto_discover: Whether to auto-discover custom types
        """
        self.url = url
        self.api = SubstrateInterface(
            url=url,
            type_registry_preset=type_registry_preset,
            auto_discover=auto_discover,
        )
        
    def is_connected(self) -> bool:
        """Check if client is connected to node."""
        try:
            return self.api.websocket.connected
        except AttributeError:
            return False
            
    def get_chain(self) -> str:
        """Get the chain name."""
        return self.api.chain
        
    def get_block_number(self) -> int:
        """Get current block number."""
        return self.api.get_block_number(None)
        
    def get_block_hash(self, block_id: Optional[int] = None) -> str:
        """Get block hash for given block number."""
        return self.api.get_block_hash(block_id)
        
    def create_keypair(
        self,
        mnemonic: Optional[str] = None,
        ss58_format: int = 42,
    ) -> Keypair:
        """
        Create or restore a keypair.
        
        Args:
            mnemonic: Optional mnemonic phrase to restore keypair
            ss58_format: SS58 address format (42 for Substrate)
            
        Returns:
            Keypair instance
        """
        if mnemonic:
            return Keypair.create_from_mnemonic(mnemonic, ss58_format=ss58_format)
        return Keypair.create_from_mnemonic(Keypair.generate_mnemonic(), ss58_format=ss58_format)
        
    def close(self):
        """Close the connection to the node."""
        if self.api:
            self.api.close()
