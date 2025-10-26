"""
Blockchain Client - Connects to Ëtrid FlareChain node
"""

import logging
import asyncio
import websockets
import json
from typing import Dict, Optional, Any, Callable

logger = logging.getLogger(__name__)

class BlockchainClient:
    """Client for Ëtrid blockchain interaction"""

    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.ws = None
        self.connected = False
        self.rpc_endpoint = config['rpc_endpoint']
        self.chain = config.get('chain', 'flare')
        self.network = config.get('network', 'ember-testnet')
        self.request_id = 0

    async def connect(self):
        """Connect to the blockchain WebSocket"""
        try:
            self.ws = await websockets.connect(self.rpc_endpoint)
            self.connected = True
            logger.info(f"Connected to Ëtrid node at {self.rpc_endpoint}")
        except Exception as e:
            logger.error(f"Failed to connect to blockchain: {str(e)}")
            self.connected = False

    async def disconnect(self):
        """Disconnect from blockchain"""
        if self.ws:
            await self.ws.close()
            self.connected = False
            logger.info("Disconnected from blockchain")

    async def _send_request(self, method: str, params: list = None) -> Dict[str, Any]:
        """Send a JSON-RPC request to the blockchain"""
        if not self.connected:
            raise ConnectionError("Not connected to blockchain")

        self.request_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method,
            "params": params or []
        }

        await self.ws.send(json.dumps(request))
        response = await self.ws.recv()
        return json.loads(response)

    async def get_block_number(self) -> Optional[int]:
        """Get current block number"""
        try:
            response = await self._send_request("chain_getHeader")
            if 'result' in response and 'number' in response['result']:
                return int(response['result']['number'], 16)
            return None
        except Exception as e:
            logger.error(f"Error getting block number: {str(e)}")
            return None

    async def get_runtime_version(self) -> Optional[Dict[str, Any]]:
        """Get runtime version"""
        try:
            response = await self._send_request("state_getRuntimeVersion")
            return response.get('result')
        except Exception as e:
            logger.error(f"Error getting runtime version: {str(e)}")
            return None

    async def subscribe_blocks(self, callback: Callable):
        """Subscribe to new blocks"""
        try:
            response = await self._send_request("chain_subscribeNewHeads")
            subscription_id = response.get('result')

            logger.info(f"Subscribed to new blocks: {subscription_id}")

            # Listen for new blocks
            while self.connected:
                message = await self.ws.recv()
                data = json.loads(message)

                if 'params' in data and 'subscription' in data['params']:
                    if data['params']['subscription'] == subscription_id:
                        block_header = data['params']['result']
                        await callback(block_header)

        except Exception as e:
            logger.error(f"Error in block subscription: {str(e)}")

    async def query_storage(self, module: str, storage_item: str, params: list = None) -> Optional[Any]:
        """Query on-chain storage"""
        try:
            # Construct storage key
            storage_key = f"0x{module}{storage_item}"
            if params:
                # Add encoded parameters to storage key
                # This is simplified - production would use proper SCALE encoding
                pass

            response = await self._send_request("state_getStorage", [storage_key])
            return response.get('result')
        except Exception as e:
            logger.error(f"Error querying storage: {str(e)}")
            return None

    async def get_validator_committee(self) -> Optional[list]:
        """Get current validator committee from runtime"""
        try:
            # Query validator committee from pallet-validator-committee
            response = await self._send_request("state_call", [
                "ValidatorCommitteeApi_get_committee",
                "0x"  # No parameters
            ])

            if 'result' in response:
                # Decode the SCALE-encoded response
                # This is simplified - production would use proper SCALE decoding
                return response['result']
            return None
        except Exception as e:
            logger.error(f"Error getting validator committee: {str(e)}")
            return None

    async def health_check(self) -> Dict[str, Any]:
        """Check blockchain connection health"""
        if not self.connected:
            return {"status": "disconnected"}

        try:
            block_number = await self.get_block_number()
            runtime_version = await self.get_runtime_version()

            return {
                "status": "healthy",
                "connected": True,
                "chain": self.chain,
                "network": self.network,
                "block_number": block_number,
                "runtime_version": runtime_version
            }
        except Exception as e:
            return {
                "status": "unhealthy",
                "error": str(e)
            }
