"""Bridge Wrapper - Cross-Chain Transfers

Enables cross-chain asset transfers across 13 supported blockchains.
"""

from typing import Dict, Any, List, Optional
from enum import Enum
from substrateinterface import SubstrateInterface, Keypair
from ..errors import (
    NotConnectedError,
    BridgeError,
    UnsupportedChainError,
    AmountBelowMinimumError,
)


class SupportedChain(Enum):
    """Supported blockchain networks."""
    BTC = "BTC"          # Bitcoin
    ETH = "ETH"          # Ethereum
    SOL = "SOL"          # Solana
    XRP = "XRP"          # Ripple
    BNB = "BNB"          # Binance Smart Chain
    TRX = "TRX"          # Tron
    ADA = "ADA"          # Cardano
    MATIC = "MATIC"      # Polygon
    DOGE = "DOGE"        # Dogecoin
    LTC = "LTC"          # Litecoin
    XLM = "XLM"          # Stellar
    LINK = "LINK"        # Chainlink
    USDT = "USDT"        # Tether (multi-chain)


class TransferStatus(Enum):
    """Bridge transfer status."""
    PENDING = "Pending"
    IN_PROGRESS = "InProgress"
    COMPLETED = "Completed"
    FAILED = "Failed"
    REFUNDED = "Refunded"


class BridgeWrapper:
    """
    Wrapper for Bridge pallet - 13 chain cross-chain transfers.

    Enables seamless asset transfers across 13 major blockchain networks
    with automated fee calculation and transfer status tracking.
    """

    # Minimum transfer amounts per chain (in planck equivalent)
    MIN_TRANSFER_AMOUNTS = {
        SupportedChain.BTC.value: 10000,        # 0.0001 BTC
        SupportedChain.ETH.value: 1000000,      # 0.001 ETH
        SupportedChain.SOL.value: 10000000,     # 0.01 SOL
        SupportedChain.XRP.value: 1000000,      # 1 XRP
        SupportedChain.BNB.value: 1000000,      # 0.001 BNB
        SupportedChain.TRX.value: 1000000,      # 1 TRX
        SupportedChain.ADA.value: 1000000,      # 1 ADA
        SupportedChain.MATIC.value: 1000000,    # 1 MATIC
        SupportedChain.DOGE.value: 1000000,     # 1 DOGE
        SupportedChain.LTC.value: 10000,        # 0.0001 LTC
        SupportedChain.XLM.value: 1000000,      # 1 XLM
        SupportedChain.LINK.value: 100000,      # 0.1 LINK
        SupportedChain.USDT.value: 1000000,     # 1 USDT
    }

    def __init__(self, api: SubstrateInterface):
        """
        Initialize Bridge wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    def _validate_chain(self, chain: str):
        """Validate chain is supported."""
        if chain not in [c.value for c in SupportedChain]:
            raise UnsupportedChainError(f"Chain not supported: {chain}")

    async def bridge(
        self,
        keypair: Keypair,
        from_chain: str,
        to_chain: str,
        amount: int,
        recipient: str,
    ) -> Dict[str, Any]:
        """
        Initiate cross-chain bridge transfer.

        Args:
            keypair: Sender keypair
            from_chain: Source chain (use SupportedChain enum values)
            to_chain: Destination chain
            amount: Amount to bridge (in planck)
            recipient: Recipient address on destination chain

        Returns:
            Dictionary with transfer_id, tx_hash, fee, and estimated_time

        Raises:
            UnsupportedChainError: If chain not supported
            AmountBelowMinimumError: If amount below minimum
            BridgeError: If bridge fails

        Example:
            >>> result = await wrapper.bridge(
            ...     alice,
            ...     SupportedChain.ETH.value,
            ...     SupportedChain.BTC.value,
            ...     1000000000000000000,  # 1 ETH
            ...     "bc1q..."
            ... )
            >>> print('Transfer ID:', result['transfer_id'])
            >>> print('Estimated time:', result['estimated_time'], 'minutes')
        """
        self._ensure_connected()

        try:
            # Validate chains
            self._validate_chain(from_chain)
            self._validate_chain(to_chain)

            # Check minimum amount
            min_amount = self.MIN_TRANSFER_AMOUNTS.get(from_chain, 0)
            if amount < min_amount:
                raise AmountBelowMinimumError(
                    f"Amount {amount} below minimum {min_amount} for {from_chain}"
                )

            call = self.api.compose_call(
                call_module="Bridge",
                call_function="initiateBridge",
                call_params={
                    "from_chain": from_chain,
                    "to_chain": to_chain,
                    "amount": amount,
                    "recipient": recipient,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise BridgeError(f"Bridge failed: {receipt.error_message}")

            # Extract BridgeInitiated event
            for event in receipt.triggered_events:
                if event.event_module.name == "Bridge" and event.event.name == "BridgeInitiated":
                    transfer_id = event.params[0]['value']
                    fee = int(event.params[3]['value'])

                    return {
                        "transfer_id": transfer_id,
                        "tx_hash": receipt.extrinsic_hash,
                        "from_chain": from_chain,
                        "to_chain": to_chain,
                        "amount": amount,
                        "recipient": recipient,
                        "fee": fee,
                        "estimated_time": await self.estimate_bridge_time(from_chain, to_chain),
                    }

            raise BridgeError("Bridge initiated but event not found")

        except (UnsupportedChainError, AmountBelowMinimumError):
            raise
        except Exception as e:
            raise BridgeError(f"Failed to initiate bridge: {str(e)}")

    async def get_transfer_status(self, transfer_id: str) -> Dict[str, Any]:
        """
        Get bridge transfer status.

        Args:
            transfer_id: Transfer identifier

        Returns:
            Dictionary with transfer details and status

        Example:
            >>> status = await wrapper.get_transfer_status(transfer_id)
            >>> print('Status:', status['status'])
            >>> print('Confirmations:', status['confirmations'])
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Bridge",
                storage_function="Transfers",
                params=[transfer_id]
            )

            if result.value is None:
                raise BridgeError(f"Transfer not found: {transfer_id}")

            return {
                "transfer_id": transfer_id,
                "from_chain": result.value.get('from_chain', ''),
                "to_chain": result.value.get('to_chain', ''),
                "amount": int(result.value.get('amount', 0)),
                "recipient": result.value.get('recipient', ''),
                "sender": result.value.get('sender', ''),
                "status": result.value.get('status', TransferStatus.PENDING.value),
                "confirmations": int(result.value.get('confirmations', 0)),
                "required_confirmations": int(result.value.get('required_confirmations', 0)),
                "fee": int(result.value.get('fee', 0)),
                "initiated_at": int(result.value.get('initiated_at', 0)),
                "completed_at": int(result.value.get('completed_at', 0)),
            }

        except Exception as e:
            raise BridgeError(f"Failed to get transfer status: {str(e)}")

    async def get_supported_chains(self) -> List[str]:
        """
        Get list of supported chains.

        Returns:
            List of supported chain names

        Example:
            >>> chains = await wrapper.get_supported_chains()
            >>> print('Supported chains:', chains)
        """
        return [chain.value for chain in SupportedChain]

    async def get_bridge_fee(self, from_chain: str, to_chain: str, amount: int) -> int:
        """
        Get bridge fee for transfer.

        Args:
            from_chain: Source chain
            to_chain: Destination chain
            amount: Transfer amount

        Returns:
            Fee in planck

        Raises:
            UnsupportedChainError: If chain not supported

        Example:
            >>> fee = await wrapper.get_bridge_fee(
            ...     SupportedChain.ETH.value,
            ...     SupportedChain.BTC.value,
            ...     1000000000000000000
            ... )
            >>> print('Bridge fee:', fee)
        """
        self._ensure_connected()

        try:
            self._validate_chain(from_chain)
            self._validate_chain(to_chain)

            result = self.api.query(
                module="Bridge",
                storage_function="calculateFee",
                params=[from_chain, to_chain, amount]
            )

            return int(result.value) if result and result.value else 0

        except UnsupportedChainError:
            raise
        except Exception as e:
            raise BridgeError(f"Failed to get bridge fee: {str(e)}")

    async def estimate_bridge_time(self, from_chain: str, to_chain: str) -> int:
        """
        Estimate bridge transfer time.

        Args:
            from_chain: Source chain
            to_chain: Destination chain

        Returns:
            Estimated time in minutes

        Raises:
            UnsupportedChainError: If chain not supported

        Example:
            >>> time = await wrapper.estimate_bridge_time(
            ...     SupportedChain.ETH.value,
            ...     SupportedChain.BTC.value
            ... )
            >>> print(f'Estimated time: {time} minutes')
        """
        self._ensure_connected()

        try:
            self._validate_chain(from_chain)
            self._validate_chain(to_chain)

            # Estimated confirmation times (in minutes)
            confirmation_times = {
                SupportedChain.BTC.value: 60,
                SupportedChain.ETH.value: 15,
                SupportedChain.SOL.value: 1,
                SupportedChain.XRP.value: 4,
                SupportedChain.BNB.value: 3,
                SupportedChain.TRX.value: 3,
                SupportedChain.ADA.value: 20,
                SupportedChain.MATIC.value: 2,
                SupportedChain.DOGE.value: 60,
                SupportedChain.LTC.value: 30,
                SupportedChain.XLM.value: 5,
                SupportedChain.LINK.value: 15,
                SupportedChain.USDT.value: 15,
            }

            source_time = confirmation_times.get(from_chain, 15)
            dest_time = confirmation_times.get(to_chain, 15)

            # Total time = source confirmations + processing + dest confirmations
            return source_time + 5 + dest_time

        except UnsupportedChainError:
            raise
        except Exception as e:
            raise BridgeError(f"Failed to estimate bridge time: {str(e)}")

    async def get_bridge_history(
        self,
        address: str,
        limit: int = 10,
    ) -> List[Dict[str, Any]]:
        """
        Get bridge transfer history for address.

        Args:
            address: Account address
            limit: Maximum number of transfers to return

        Returns:
            List of transfer records

        Example:
            >>> history = await wrapper.get_bridge_history(alice_address, limit=5)
            >>> for transfer in history:
            ...     print(f"{transfer['from_chain']} -> {transfer['to_chain']}: {transfer['status']}")
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Bridge",
                storage_function="TransferHistory",
                params=[address]
            )

            if result.value is None:
                return []

            transfers = result.value if isinstance(result.value, list) else []
            return transfers[:limit]

        except Exception as e:
            raise BridgeError(f"Failed to get bridge history: {str(e)}")
