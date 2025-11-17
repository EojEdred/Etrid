"""Reserve Vault Wrapper - Collateralized Lending

DeFi lending protocol with over-collateralization and liquidation protection.
"""

from typing import Dict, Any, Optional
from substrateinterface import SubstrateInterface, Keypair
from ..errors import (
    NotConnectedError,
    VaultError,
    InsufficientCollateralError,
    UndercollateralizedVaultError,
)


class ReserveVaultWrapper:
    """
    Wrapper for Reserve Vault pallet - DeFi lending.

    Provides collateralized lending with automated liquidation protection
    and health factor monitoring.
    """

    # Collateralization ratios
    MIN_COLLATERAL_RATIO = 150  # 150% minimum
    LIQUIDATION_THRESHOLD = 120  # 120% liquidation threshold
    LIQUIDATION_PENALTY = 10    # 10% liquidation penalty

    def __init__(self, api: SubstrateInterface):
        """
        Initialize Reserve Vault wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def create_vault(
        self,
        keypair: Keypair,
        collateral: int,
    ) -> Dict[str, Any]:
        """
        Create a collateral vault.

        Args:
            keypair: Account keypair
            collateral: Initial collateral amount (in planck)

        Returns:
            Dictionary with vault_id, tx_hash, and vault details

        Raises:
            InsufficientCollateralError: If collateral too low
            VaultError: If creation fails

        Example:
            >>> vault = await wrapper.create_vault(
            ...     alice,
            ...     1000 * 10**18  # 1000 ÉTR
            ... )
            >>> print('Vault ID:', vault['vault_id'])
        """
        self._ensure_connected()

        try:
            if collateral < 100 * 10**18:  # Minimum 100 ÉTR
                raise InsufficientCollateralError("Minimum collateral is 100 ÉTR")

            call = self.api.compose_call(
                call_module="ReserveVault",
                call_function="createVault",
                call_params={"collateral": collateral}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise VaultError(f"Failed to create vault: {receipt.error_message}")

            # Extract VaultCreated event
            for event in receipt.triggered_events:
                if event.event_module.name == "ReserveVault" and event.event.name == "VaultCreated":
                    vault_id = event.params[0]['value']
                    return {
                        "vault_id": vault_id,
                        "tx_hash": receipt.extrinsic_hash,
                        "owner": keypair.ss58_address,
                        "collateral": collateral,
                        "borrowed": 0,
                    }

            raise VaultError("Vault created but event not found")

        except (InsufficientCollateralError, VaultError):
            raise
        except Exception as e:
            raise VaultError(f"Failed to create vault: {str(e)}")

    async def deposit_collateral(
        self,
        keypair: Keypair,
        vault_id: str,
        amount: int,
    ) -> str:
        """
        Deposit additional collateral to vault.

        Args:
            keypair: Vault owner keypair
            vault_id: Vault identifier
            amount: Collateral amount to deposit (in planck)

        Returns:
            Transaction hash

        Raises:
            VaultError: If deposit fails

        Example:
            >>> tx_hash = await wrapper.deposit_collateral(
            ...     alice,
            ...     vault_id,
            ...     500 * 10**18
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="ReserveVault",
                call_function="depositCollateral",
                call_params={
                    "vault_id": vault_id,
                    "amount": amount,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise VaultError(f"Failed to deposit collateral: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise VaultError(f"Failed to deposit collateral: {str(e)}")

    async def borrow(
        self,
        keypair: Keypair,
        vault_id: str,
        amount: int,
    ) -> str:
        """
        Borrow against vault collateral.

        Args:
            keypair: Vault owner keypair
            vault_id: Vault identifier
            amount: Amount to borrow (in planck)

        Returns:
            Transaction hash

        Raises:
            UndercollateralizedVaultError: If borrow would undercollateralize vault
            VaultError: If borrow fails

        Example:
            >>> tx_hash = await wrapper.borrow(
            ...     alice,
            ...     vault_id,
            ...     300 * 10**18
            ... )
        """
        self._ensure_connected()

        try:
            # Check if borrow would undercollateralize vault
            vault = await self.get_vault(vault_id)
            if not vault:
                raise VaultError(f"Vault not found: {vault_id}")

            borrow_limit = await self.calculate_borrow_limit(vault_id)
            if vault['borrowed'] + amount > borrow_limit:
                raise UndercollateralizedVaultError(
                    f"Borrow would exceed limit. Max: {borrow_limit}, Requested: {amount}"
                )

            call = self.api.compose_call(
                call_module="ReserveVault",
                call_function="borrow",
                call_params={
                    "vault_id": vault_id,
                    "amount": amount,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise VaultError(f"Failed to borrow: {receipt.error_message}")

            return receipt.extrinsic_hash

        except (UndercollateralizedVaultError, VaultError):
            raise
        except Exception as e:
            raise VaultError(f"Failed to borrow: {str(e)}")

    async def repay(
        self,
        keypair: Keypair,
        vault_id: str,
        amount: int,
    ) -> str:
        """
        Repay borrowed amount.

        Args:
            keypair: Vault owner keypair
            vault_id: Vault identifier
            amount: Amount to repay (in planck)

        Returns:
            Transaction hash

        Raises:
            VaultError: If repayment fails

        Example:
            >>> tx_hash = await wrapper.repay(
            ...     alice,
            ...     vault_id,
            ...     100 * 10**18
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="ReserveVault",
                call_function="repay",
                call_params={
                    "vault_id": vault_id,
                    "amount": amount,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise VaultError(f"Failed to repay: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise VaultError(f"Failed to repay: {str(e)}")

    async def get_vault(self, vault_id: str) -> Optional[Dict[str, Any]]:
        """
        Get vault information.

        Args:
            vault_id: Vault identifier

        Returns:
            Vault information or None if not found

        Example:
            >>> vault = await wrapper.get_vault(vault_id)
            >>> if vault:
            ...     print('Collateral:', vault['collateral'])
            ...     print('Borrowed:', vault['borrowed'])
            ...     print('Health factor:', vault['health_factor'])
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="ReserveVault",
                storage_function="Vaults",
                params=[vault_id]
            )

            if result.value is None:
                return None

            collateral = int(result.value.get('collateral', 0))
            borrowed = int(result.value.get('borrowed', 0))

            return {
                "vault_id": vault_id,
                "owner": result.value.get('owner', ''),
                "collateral": collateral,
                "borrowed": borrowed,
                "health_factor": await self.get_health_factor(vault_id),
                "created_at": int(result.value.get('created_at', 0)),
                "interest_rate": float(result.value.get('interest_rate', 0.0)),
            }

        except Exception as e:
            raise VaultError(f"Failed to get vault: {str(e)}")

    async def get_health_factor(self, vault_id: str) -> float:
        """
        Get vault health factor.

        Health factor = (collateral / borrowed) * 100
        < 120% = liquidatable
        < 150% = risky
        >= 150% = healthy

        Args:
            vault_id: Vault identifier

        Returns:
            Health factor as percentage

        Example:
            >>> health = await wrapper.get_health_factor(vault_id)
            >>> if health < 150:
            ...     print('Warning: Low health factor!')
        """
        self._ensure_connected()

        try:
            vault = await self.get_vault(vault_id)
            if not vault:
                return 0.0

            if vault['borrowed'] == 0:
                return float('inf')

            health_factor = (vault['collateral'] / vault['borrowed']) * 100
            return health_factor

        except Exception as e:
            raise VaultError(f"Failed to get health factor: {str(e)}")

    async def is_liquidatable(self, vault_id: str) -> bool:
        """
        Check if vault is liquidatable.

        Args:
            vault_id: Vault identifier

        Returns:
            True if vault can be liquidated

        Example:
            >>> if await wrapper.is_liquidatable(vault_id):
            ...     print('Vault is at risk of liquidation!')
        """
        self._ensure_connected()

        try:
            health_factor = await self.get_health_factor(vault_id)
            return health_factor < self.LIQUIDATION_THRESHOLD

        except Exception as e:
            raise VaultError(f"Failed to check liquidation status: {str(e)}")

    async def liquidate(
        self,
        keypair: Keypair,
        vault_id: str,
    ) -> str:
        """
        Liquidate an undercollateralized vault.

        Args:
            keypair: Liquidator keypair
            vault_id: Vault identifier

        Returns:
            Transaction hash

        Raises:
            VaultError: If liquidation fails

        Example:
            >>> if await wrapper.is_liquidatable(vault_id):
            ...     tx_hash = await wrapper.liquidate(liquidator, vault_id)
        """
        self._ensure_connected()

        try:
            if not await self.is_liquidatable(vault_id):
                raise VaultError("Vault is not liquidatable")

            call = self.api.compose_call(
                call_module="ReserveVault",
                call_function="liquidate",
                call_params={"vault_id": vault_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise VaultError(f"Failed to liquidate: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise VaultError(f"Failed to liquidate vault: {str(e)}")

    async def calculate_borrow_limit(self, vault_id: str) -> int:
        """
        Calculate maximum borrowable amount.

        Args:
            vault_id: Vault identifier

        Returns:
            Maximum borrow amount (in planck)

        Example:
            >>> limit = await wrapper.calculate_borrow_limit(vault_id)
            >>> print(f'Max borrow: {limit / 10**18} ÉTR')
        """
        self._ensure_connected()

        try:
            vault = await self.get_vault(vault_id)
            if not vault:
                return 0

            # Maximum borrow = collateral / (MIN_COLLATERAL_RATIO / 100)
            max_borrow = int(vault['collateral'] * 100 / self.MIN_COLLATERAL_RATIO)
            return max_borrow

        except Exception as e:
            raise VaultError(f"Failed to calculate borrow limit: {str(e)}")
