"""Accounts Wrapper - Account Operations

Basic account operations including transfers, balance queries, and batch operations.
"""

from typing import Dict, Any, List, Optional
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, TransactionError, InvalidAddressError


class AccountsWrapper:
    """
    Wrapper for account operations on FlareChain.

    Provides core account functionality including balance queries, transfers,
    batch operations, and account management.
    """

    # Account constants
    EXISTENTIAL_DEPOSIT = 10**18  # 1 ÉTR minimum balance

    def __init__(self, api: SubstrateInterface):
        """
        Initialize Accounts wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def get_balance(self, address: str) -> int:
        """
        Get account free balance.

        Args:
            address: Account address (SS58 format)

        Returns:
            Free balance in planck (1 ÉTR = 10^18 planck)

        Raises:
            TransactionError: If balance query fails

        Example:
            >>> balance = await wrapper.get_balance(alice_address)
            >>> print(f'Balance: {balance / 10**18} ÉTR')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="System",
                storage_function="Account",
                params=[address]
            )

            if result.value is None:
                return 0

            return int(result.value['data']['free'])

        except Exception as e:
            raise TransactionError(f"Failed to get balance: {str(e)}")

    async def transfer(self, keypair: Keypair, recipient: str, amount: int) -> str:
        """
        Transfer tokens to recipient.

        Args:
            keypair: Sender keypair
            recipient: Recipient address
            amount: Amount to transfer (in planck)

        Returns:
            Transaction hash

        Raises:
            TransactionError: If transfer fails

        Example:
            >>> tx_hash = await wrapper.transfer(
            ...     alice,
            ...     bob_address,
            ...     100 * 10**18  # 100 ÉTR
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Balances",
                call_function="transfer",
                call_params={
                    "dest": recipient,
                    "value": amount,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise TransactionError(f"Transfer failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise TransactionError(f"Failed to transfer: {str(e)}")

    async def transfer_keep_alive(
        self,
        keypair: Keypair,
        recipient: str,
        amount: int,
    ) -> str:
        """
        Transfer with existential deposit protection.

        Args:
            keypair: Sender keypair
            recipient: Recipient address
            amount: Amount to transfer

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.transfer_keep_alive(
            ...     alice,
            ...     bob_address,
            ...     50 * 10**18
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Balances",
                call_function="transferKeepAlive",
                call_params={
                    "dest": recipient,
                    "value": amount,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise TransactionError(f"Transfer failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise TransactionError(f"Failed to transfer keep alive: {str(e)}")

    async def transfer_with_memo(
        self,
        keypair: Keypair,
        recipient: str,
        amount: int,
        memo: str,
    ) -> str:
        """
        Transfer with memo/remark attached.

        Args:
            keypair: Sender keypair
            recipient: Recipient address
            amount: Amount to transfer
            memo: Memo/remark text

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.transfer_with_memo(
            ...     alice,
            ...     bob_address,
            ...     10 * 10**18,
            ...     "Payment for services"
            ... )
        """
        self._ensure_connected()

        try:
            # First transfer
            tx_hash = await self.transfer(keypair, recipient, amount)

            # Then add remark
            remark_call = self.api.compose_call(
                call_module="System",
                call_function="remark",
                call_params={"remark": memo.encode()}
            )

            remark_ext = self.api.create_signed_extrinsic(call=remark_call, keypair=keypair)
            self.api.submit_extrinsic(remark_ext, wait_for_inclusion=True)

            return tx_hash

        except Exception as e:
            raise TransactionError(f"Failed to transfer with memo: {str(e)}")

    async def batch_transfer(
        self,
        keypair: Keypair,
        transfers: List[Dict[str, Any]],
    ) -> str:
        """
        Execute batch transfers (atomic).

        All transfers execute atomically - if one fails, all fail.

        Args:
            keypair: Sender keypair
            transfers: List of transfer dicts with 'recipient' and 'amount' keys

        Returns:
            Transaction hash

        Raises:
            TransactionError: If any transfer fails

        Example:
            >>> transfers = [
            ...     {"recipient": bob_address, "amount": 10 * 10**18},
            ...     {"recipient": charlie_address, "amount": 20 * 10**18},
            ... ]
            >>> tx_hash = await wrapper.batch_transfer(alice, transfers)
        """
        self._ensure_connected()

        try:
            calls = []
            for t in transfers:
                call = self.api.compose_call(
                    call_module="Balances",
                    call_function="transfer",
                    call_params={
                        "dest": t['recipient'],
                        "value": t['amount'],
                    }
                )
                calls.append(call)

            batch_call = self.api.compose_call(
                call_module="Utility",
                call_function="batchAll",
                call_params={"calls": calls}
            )

            extrinsic = self.api.create_signed_extrinsic(call=batch_call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise TransactionError(f"Batch transfer failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise TransactionError(f"Failed batch transfer: {str(e)}")

    async def get_account_info(self, address: str) -> Dict[str, Any]:
        """
        Get detailed account information.

        Args:
            address: Account address

        Returns:
            Dictionary with balance and account details

        Example:
            >>> info = await wrapper.get_account_info(alice_address)
            >>> print(f'Free: {info["free"] / 10**18} ÉTR')
            >>> print(f'Reserved: {info["reserved"] / 10**18} ÉTR')
            >>> print(f'Nonce: {info["nonce"]}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="System",
                storage_function="Account",
                params=[address]
            )

            if result.value is None:
                return {"exists": False}

            data = result.value['data']

            return {
                "exists": True,
                "address": address,
                "free": int(data['free']),
                "reserved": int(data['reserved']),
                "frozen": int(data.get('frozen', 0)),
                "nonce": int(result.value.get('nonce', 0)),
                "providers": int(result.value.get('providers', 0)),
                "consumers": int(result.value.get('consumers', 0)),
                "sufficients": int(result.value.get('sufficients', 0)),
            }

        except Exception as e:
            raise TransactionError(f"Failed to get account info: {str(e)}")

    async def get_total_balance(self, address: str) -> int:
        """
        Get total account balance (free + reserved).

        Args:
            address: Account address

        Returns:
            Total balance in planck

        Example:
            >>> total = await wrapper.get_total_balance(alice_address)
            >>> print(f'Total: {total / 10**18} ÉTR')
        """
        self._ensure_connected()

        try:
            info = await self.get_account_info(address)
            if not info.get('exists', False):
                return 0

            return info['free'] + info['reserved']

        except Exception as e:
            raise TransactionError(f"Failed to get total balance: {str(e)}")

    async def get_existential_deposit(self) -> int:
        """
        Get minimum balance required for account to exist.

        Returns:
            Existential deposit in planck

        Example:
            >>> ed = await wrapper.get_existential_deposit()
            >>> print(f'Existential deposit: {ed / 10**18} ÉTR')
        """
        self._ensure_connected()

        try:
            # Query existential deposit constant
            result = self.api.get_constant(
                module_name="Balances",
                constant_name="ExistentialDeposit"
            )

            return int(result.value) if result else self.EXISTENTIAL_DEPOSIT

        except Exception as e:
            return self.EXISTENTIAL_DEPOSIT

    async def account_exists(self, address: str) -> bool:
        """
        Check if account exists on chain.

        Args:
            address: Account address

        Returns:
            True if account exists (has balance >= ED)

        Example:
            >>> exists = await wrapper.account_exists(alice_address)
            >>> if exists:
            ...     print('Account is active')
        """
        self._ensure_connected()

        try:
            balance = await self.get_balance(address)
            ed = await self.get_existential_deposit()

            return balance >= ed

        except Exception as e:
            return False

    async def get_nonce(self, address: str) -> int:
        """
        Get account nonce (transaction count).

        Args:
            address: Account address

        Returns:
            Account nonce

        Example:
            >>> nonce = await wrapper.get_nonce(alice_address)
            >>> print(f'Nonce: {nonce}')
        """
        self._ensure_connected()

        try:
            info = await self.get_account_info(address)
            return info.get('nonce', 0)

        except Exception as e:
            raise TransactionError(f"Failed to get nonce: {str(e)}")
