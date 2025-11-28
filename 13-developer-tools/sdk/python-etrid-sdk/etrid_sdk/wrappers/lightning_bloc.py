"""
Lightning-Bloc Wrapper - Layer 3 Payment Channels
"""

from typing import Dict, List, Optional, Any
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, ChannelError, RouteNotFoundError


class LightningBlocWrapper:
    """
    Wrapper for Lightning-Bloc pallet operations.
    
    Lightning-Bloc provides Layer 3 payment channels with 500K+ TPS.
    """
    
    def __init__(self, api: SubstrateInterface):
        """
        Initialize Lightning-Bloc wrapper.
        
        Args:
            api: Connected Substrate API instance
        """
        self.api = api
        
    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()
            
    async def open_channel(
        self,
        keypair: Keypair,
        recipient: str,
        amount: int,
    ) -> Dict[str, Any]:
        """
        Open a new payment channel.
        
        Args:
            keypair: Sender keypair
            recipient: Recipient address
            amount: Channel capacity in Planck (1 ÉTR = 10^18 Planck)
            
        Returns:
            Dictionary with channel_id and tx_hash
            
        Example:
            >>> channel = await wrapper.open_channel(
            ...     alice,
            ...     "5GrwvaEF...",
            ...     1000 * 10**18
            ... )
            >>> print(channel['channel_id'])
        """
        self._ensure_connected()
        
        call = self.api.compose_call(
            call_module="LightningBloc",
            call_function="openChannel",
            call_params={
                "recipient": recipient,
                "amount": amount,
            }
        )
        
        extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
        receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        
        if not receipt.is_success:
            raise ChannelError(f"Failed to open channel: {receipt.error_message}")
            
        # Extract channel_id from events
        for event in receipt.triggered_events:
            if event.event_module.name == "LightningBloc" and event.event.name == "ChannelOpened":
                channel_id = event.params[0]['value']
                return {
                    "channel_id": channel_id,
                    "tx_hash": receipt.extrinsic_hash,
                    "recipient": recipient,
                    "amount": amount,
                }
                
        raise ChannelError("Channel opened but no event found")
        
    async def get_channel(self, channel_id: str) -> Optional[Dict[str, Any]]:
        """
        Get channel information.
        
        Args:
            channel_id: Channel identifier
            
        Returns:
            Channel information or None if not found
        """
        self._ensure_connected()
        
        result = self.api.query(
            module="LightningBloc",
            storage_function="Channels",
            params=[channel_id]
        )
        
        if result.value is None:
            return None
            
        return {
            "channel_id": channel_id,
            "from": result.value['from'],
            "to": result.value['to'],
            "balance": int(result.value['balance']),
            "nonce": int(result.value['nonce']),
            "status": result.value['status'],
        }
        
    async def send_payment(
        self,
        keypair: Keypair,
        channel_id: str,
        amount: int,
    ) -> Dict[str, Any]:
        """
        Send payment through channel.
        
        Args:
            keypair: Sender keypair
            channel_id: Channel to use
            amount: Payment amount in Planck
            
        Returns:
            Transaction result
        """
        self._ensure_connected()
        
        call = self.api.compose_call(
            call_module="LightningBloc",
            call_function="sendPayment",
            call_params={
                "channel_id": channel_id,
                "amount": amount,
            }
        )
        
        extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
        receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        
        if not receipt.is_success:
            raise ChannelError(f"Payment failed: {receipt.error_message}")
            
        return {
            "tx_hash": receipt.extrinsic_hash,
            "channel_id": channel_id,
            "amount": amount,
        }
        
    async def close_channel(
        self,
        keypair: Keypair,
        channel_id: str,
    ) -> str:
        """
        Close a payment channel.
        
        Args:
            keypair: Channel owner keypair
            channel_id: Channel to close
            
        Returns:
            Transaction hash
        """
        self._ensure_connected()
        
        call = self.api.compose_call(
            call_module="LightningBloc",
            call_function="closeChannel",
            call_params={"channel_id": channel_id}
        )
        
        extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
        receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        
        if not receipt.is_success:
            raise ChannelError(f"Failed to close channel: {receipt.error_message}")
            
        return receipt.extrinsic_hash
        
    async def get_route(
        self,
        from_address: str,
        to_address: str,
        amount: int,
    ) -> List[str]:
        """
        Find payment route from source to destination.
        
        Args:
            from_address: Source address
            to_address: Destination address
            amount: Payment amount
            
        Returns:
            List of channel IDs forming the route
            
        Raises:
            RouteNotFoundError: If no route exists
        """
        self._ensure_connected()
        
        result = self.api.query(
            module="LightningBloc",
            storage_function="findRoute",
            params=[from_address, to_address, amount]
        )
        
        if result.value is None or len(result.value) == 0:
            raise RouteNotFoundError(
                f"No route found from {from_address} to {to_address}"
            )
            
        return [channel_id for channel_id in result.value]
