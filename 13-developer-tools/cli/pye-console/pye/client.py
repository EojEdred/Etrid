"""
ËTRID RPC Client

Provides WebSocket and HTTP connectivity to ËTRID nodes.
"""

import json
import time
from typing import Any, Dict, Optional, Union
from websocket import create_connection, WebSocketException
import requests
from rich.console import Console

console = Console()


class EtridRPCError(Exception):
    """Custom exception for RPC errors"""
    pass


class EtridClient:
    """
    Client for connecting to ËTRID nodes via JSON-RPC over WebSocket.

    Args:
        ws_url: WebSocket endpoint URL (default: ws://localhost:9944)
        http_url: HTTP endpoint URL (optional, for fallback)
        timeout: Request timeout in seconds
    """

    def __init__(
        self,
        ws_url: str = "ws://localhost:9944",
        http_url: Optional[str] = None,
        timeout: int = 30,
    ):
        self.ws_url = ws_url
        self.http_url = http_url or ws_url.replace("ws://", "http://").replace("wss://", "https://")
        self.timeout = timeout
        self.ws = None
        self._request_id = 0

    def _get_next_id(self) -> int:
        """Generate next request ID"""
        self._request_id += 1
        return self._request_id

    def connect(self) -> None:
        """Establish WebSocket connection"""
        try:
            self.ws = create_connection(self.ws_url, timeout=self.timeout)
            console.print(f"[green]✓[/green] Connected to {self.ws_url}")
        except WebSocketException as e:
            raise EtridRPCError(f"Failed to connect to {self.ws_url}: {e}")

    def disconnect(self) -> None:
        """Close WebSocket connection"""
        if self.ws:
            self.ws.close()
            self.ws = None
            console.print("[yellow]Disconnected from ËTRID node[/yellow]")

    def _send_request(self, method: str, params: Optional[list] = None) -> Dict[str, Any]:
        """
        Send JSON-RPC request and return response

        Args:
            method: RPC method name
            params: Method parameters (optional)

        Returns:
            Response data

        Raises:
            EtridRPCError: If request fails
        """
        request = {
            "jsonrpc": "2.0",
            "id": self._get_next_id(),
            "method": method,
            "params": params or [],
        }

        try:
            # Try WebSocket first
            if not self.ws:
                self.connect()

            self.ws.send(json.dumps(request))
            response_raw = self.ws.recv()
            response = json.loads(response_raw)

            if "error" in response:
                error = response["error"]
                raise EtridRPCError(f"RPC Error: {error.get('message', 'Unknown error')}")

            return response.get("result", {})

        except (WebSocketException, ConnectionError) as e:
            # Fallback to HTTP if WebSocket fails
            console.print(f"[yellow]WebSocket failed, trying HTTP...[/yellow]")
            try:
                http_response = requests.post(
                    self.http_url,
                    json=request,
                    timeout=self.timeout,
                    headers={"Content-Type": "application/json"},
                )
                http_response.raise_for_status()
                response = http_response.json()

                if "error" in response:
                    error = response["error"]
                    raise EtridRPCError(f"RPC Error: {error.get('message', 'Unknown error')}")

                return response.get("result", {})

            except requests.RequestException as http_e:
                raise EtridRPCError(f"Both WebSocket and HTTP failed: {e}, {http_e}")

    # Core RPC methods

    def get_block(self, block_hash: Optional[str] = None) -> Dict[str, Any]:
        """
        Get block by hash or latest block

        Args:
            block_hash: Block hash (optional, defaults to latest)

        Returns:
            Block data
        """
        params = [block_hash] if block_hash else []
        return self._send_request("chain_getBlock", params)

    def get_block_hash(self, block_number: Optional[int] = None) -> str:
        """
        Get block hash by number

        Args:
            block_number: Block number (optional, defaults to latest)

        Returns:
            Block hash
        """
        params = [block_number] if block_number is not None else []
        return self._send_request("chain_getBlockHash", params)

    def get_balance(self, address: str) -> Dict[str, Any]:
        """
        Get account balance

        Args:
            address: Account address

        Returns:
            Balance information
        """
        return self._send_request("account_getBalance", [address])

    def get_account_info(self, address: str) -> Dict[str, Any]:
        """
        Get account information

        Args:
            address: Account address

        Returns:
            Account data
        """
        return self._send_request("account_getInfo", [address])

    def send_transaction(
        self,
        from_addr: str,
        to_addr: str,
        amount: Union[int, str],
        signature: Optional[str] = None,
    ) -> str:
        """
        Send transaction

        Args:
            from_addr: Sender address
            to_addr: Recipient address
            amount: Amount to send
            signature: Transaction signature (optional)

        Returns:
            Transaction hash
        """
        tx_data = {
            "from": from_addr,
            "to": to_addr,
            "value": str(amount),
        }
        if signature:
            tx_data["signature"] = signature

        return self._send_request("author_submitTransaction", [tx_data])

    def get_transaction(self, tx_hash: str) -> Dict[str, Any]:
        """
        Get transaction by hash

        Args:
            tx_hash: Transaction hash

        Returns:
            Transaction data
        """
        return self._send_request("chain_getTransaction", [tx_hash])

    def query_state(self, storage_key: str, block_hash: Optional[str] = None) -> Any:
        """
        Query chain state

        Args:
            storage_key: Storage key to query
            block_hash: Block hash (optional)

        Returns:
            State value
        """
        params = [storage_key]
        if block_hash:
            params.append(block_hash)
        return self._send_request("state_getStorage", params)

    # Staking methods

    def stake(self, address: str, amount: Union[int, str]) -> str:
        """
        Stake tokens

        Args:
            address: Staker address
            amount: Amount to stake

        Returns:
            Transaction hash
        """
        return self._send_request("staking_stake", [address, str(amount)])

    def unstake(self, address: str, amount: Union[int, str]) -> str:
        """
        Unstake tokens

        Args:
            address: Staker address
            amount: Amount to unstake

        Returns:
            Transaction hash
        """
        return self._send_request("staking_unstake", [address, str(amount)])

    def get_stake_info(self, address: str) -> Dict[str, Any]:
        """
        Get staking information

        Args:
            address: Staker address

        Returns:
            Stake data
        """
        return self._send_request("staking_getInfo", [address])

    # Consensus methods

    def get_consensus_status(self) -> Dict[str, Any]:
        """
        Get current consensus status

        Returns:
            Consensus information
        """
        return self._send_request("consensus_getStatus", [])

    def register_validity_node(self, address: str, stake_amount: Union[int, str]) -> str:
        """
        Register as validity node

        Args:
            address: Node address
            stake_amount: Required stake amount

        Returns:
            Registration transaction hash
        """
        return self._send_request("consensus_registerValidityNode", [address, str(stake_amount)])

    def submit_vote(self, proposal_id: str, vote: bool) -> str:
        """
        Submit consensus vote

        Args:
            proposal_id: Proposal ID
            vote: Vote (True for yes, False for no)

        Returns:
            Vote transaction hash
        """
        return self._send_request("consensus_submitVote", [proposal_id, vote])

    # Utility methods

    def get_chain_info(self) -> Dict[str, Any]:
        """
        Get chain information

        Returns:
            Chain metadata
        """
        return self._send_request("system_chain", [])

    def get_node_health(self) -> Dict[str, Any]:
        """
        Get node health status

        Returns:
            Health information
        """
        return self._send_request("system_health", [])

    def get_sync_state(self) -> Dict[str, Any]:
        """
        Get node sync state

        Returns:
            Sync status
        """
        return self._send_request("system_syncState", [])

    def __enter__(self):
        """Context manager entry"""
        self.connect()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        self.disconnect()
