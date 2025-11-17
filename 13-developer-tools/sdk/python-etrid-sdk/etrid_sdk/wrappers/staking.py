"""Staking Wrapper - Validator Staking

Nominate validators and earn staking rewards on FlareChain.
"""

from typing import Dict, Any, List, Optional
from enum import Enum
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, StakingError


class RewardDestination(Enum):
    """Reward payout destination options."""
    STAKED = "Staked"         # Auto-compound rewards
    STASH = "Stash"           # Send to stash account
    CONTROLLER = "Controller"  # Send to controller account
    ACCOUNT = "Account"        # Send to specific account


class StakingWrapper:
    """
    Wrapper for Staking pallet - FlareChain validator nomination.

    Enables staking operations including bonding, unbonding, nominating validators,
    and earning staking rewards with configurable payout destinations.
    """

    # Staking constants
    MIN_BOND_AMOUNT = 100 * 10**18  # Minimum 100 ÉTR
    UNBONDING_PERIOD_ERAS = 28      # 28 eras (~7 days)
    MAX_NOMINATIONS = 16            # Maximum validator nominations

    def __init__(self, api: SubstrateInterface):
        """
        Initialize Staking wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def bond(
        self,
        keypair: Keypair,
        controller: str,
        amount: int,
        payee: str = RewardDestination.STAKED.value,
    ) -> str:
        """
        Bond tokens for staking.

        Args:
            keypair: Stash account keypair
            controller: Controller account address
            amount: Amount to bond (in planck)
            payee: Reward destination (use RewardDestination enum)

        Returns:
            Transaction hash

        Raises:
            StakingError: If bonding fails

        Example:
            >>> tx_hash = await wrapper.bond(
            ...     alice,
            ...     controller_address,
            ...     1000 * 10**18,
            ...     RewardDestination.STAKED.value
            ... )
        """
        self._ensure_connected()

        try:
            if amount < self.MIN_BOND_AMOUNT:
                raise StakingError(f"Amount below minimum: {self.MIN_BOND_AMOUNT}")

            call = self.api.compose_call(
                call_module="Staking",
                call_function="bond",
                call_params={
                    "controller": controller,
                    "value": amount,
                    "payee": payee,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Bond failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to bond: {str(e)}")

    async def bond_extra(self, keypair: Keypair, amount: int) -> str:
        """
        Bond additional tokens to existing stake.

        Args:
            keypair: Controller keypair
            amount: Additional amount to bond

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.bond_extra(controller, 500 * 10**18)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Staking",
                call_function="bondExtra",
                call_params={"max_additional": amount}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Bond extra failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to bond extra: {str(e)}")

    async def unbond(self, keypair: Keypair, amount: int) -> str:
        """
        Unbond staked tokens (starts unbonding period).

        Args:
            keypair: Controller keypair
            amount: Amount to unbond (in planck)

        Returns:
            Transaction hash

        Raises:
            StakingError: If unbonding fails

        Example:
            >>> tx_hash = await wrapper.unbond(controller, 200 * 10**18)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Staking",
                call_function="unbond",
                call_params={"value": amount}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Unbond failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to unbond: {str(e)}")

    async def withdraw_unbonded(self, keypair: Keypair, num_slashing_spans: int = 0) -> str:
        """
        Withdraw unbonded tokens after unbonding period.

        Args:
            keypair: Controller keypair
            num_slashing_spans: Number of slashing spans to remove

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.withdraw_unbonded(controller)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Staking",
                call_function="withdrawUnbonded",
                call_params={"num_slashing_spans": num_slashing_spans}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Withdraw failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to withdraw unbonded: {str(e)}")

    async def nominate(self, keypair: Keypair, targets: List[str]) -> str:
        """
        Nominate validator targets.

        Args:
            keypair: Controller keypair
            targets: List of validator addresses (max 16)

        Returns:
            Transaction hash

        Raises:
            StakingError: If nomination fails

        Example:
            >>> validators = ["5GrwvaEF...", "5FHneW46..."]
            >>> tx_hash = await wrapper.nominate(controller, validators)
        """
        self._ensure_connected()

        try:
            if len(targets) > self.MAX_NOMINATIONS:
                raise StakingError(f"Too many nominations. Max: {self.MAX_NOMINATIONS}")

            call = self.api.compose_call(
                call_module="Staking",
                call_function="nominate",
                call_params={"targets": targets}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Nominate failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to nominate: {str(e)}")

    async def chill(self, keypair: Keypair) -> str:
        """
        Stop nominating (chill account).

        Args:
            keypair: Controller keypair

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.chill(controller)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Staking",
                call_function="chill"
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Chill failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to chill: {str(e)}")

    async def get_validator_status(self, address: str) -> Dict[str, Any]:
        """
        Get validator status and preferences.

        Args:
            address: Validator address

        Returns:
            Validator status dictionary

        Example:
            >>> status = await wrapper.get_validator_status(validator_address)
            >>> print(f'Commission: {status["commission"]}%')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Staking",
                storage_function="Validators",
                params=[address]
            )

            if result.value is None:
                return {"is_validator": False}

            return {
                "is_validator": True,
                "commission": int(result.value.get('commission', 0)),
                "blocked": bool(result.value.get('blocked', False)),
            }

        except Exception as e:
            raise StakingError(f"Failed to get validator status: {str(e)}")

    async def get_staking_info(self, address: str) -> Optional[Dict[str, Any]]:
        """
        Get staking information for account.

        Args:
            address: Stash address

        Returns:
            Staking info dictionary or None if not staking

        Example:
            >>> info = await wrapper.get_staking_info(stash_address)
            >>> if info:
            ...     print(f'Total bonded: {info["total"] / 10**18} ÉTR')
            ...     print(f'Active: {info["active"] / 10**18} ÉTR')
        """
        self._ensure_connected()

        try:
            bonded = self.api.query(
                module="Staking",
                storage_function="Bonded",
                params=[address]
            )

            if bonded.value is None:
                return None

            ledger = self.api.query(
                module="Staking",
                storage_function="Ledger",
                params=[bonded.value]
            )

            if ledger.value is None:
                return None

            return {
                "stash": address,
                "controller": bonded.value,
                "total": int(ledger.value.get('total', 0)),
                "active": int(ledger.value.get('active', 0)),
                "unlocking": ledger.value.get('unlocking', []),
                "claimed_rewards": ledger.value.get('claimed_rewards', []),
            }

        except Exception as e:
            raise StakingError(f"Failed to get staking info: {str(e)}")

    async def get_nominators(self, address: str) -> List[str]:
        """
        Get nominations for account.

        Args:
            address: Nominator address

        Returns:
            List of nominated validator addresses

        Example:
            >>> nominations = await wrapper.get_nominators(nominator_address)
            >>> print(f'Nominating {len(nominations)} validators')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Staking",
                storage_function="Nominators",
                params=[address]
            )

            if result.value is None:
                return []

            return list(result.value.get('targets', []))

        except Exception as e:
            raise StakingError(f"Failed to get nominations: {str(e)}")

    async def set_commission(self, keypair: Keypair, commission: int) -> str:
        """
        Set validator commission rate (validators only).

        Args:
            keypair: Validator controller keypair
            commission: Commission percentage (0-100)

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.set_commission(validator, 5)  # 5%
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Staking",
                call_function="validate",
                call_params={"prefs": {"commission": commission}}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise StakingError(f"Set commission failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise StakingError(f"Failed to set commission: {str(e)}")

    async def estimate_rewards(self, address: str, amount: int) -> Dict[str, int]:
        """
        Estimate staking rewards for amount.

        Args:
            address: Account address
            amount: Staked amount (in planck)

        Returns:
            Dictionary with estimated rewards (daily, monthly, yearly)

        Example:
            >>> rewards = await wrapper.estimate_rewards(
            ...     stash_address,
            ...     1000 * 10**18
            ... )
            >>> print(f'Yearly: {rewards["yearly"] / 10**18} ÉTR')
        """
        self._ensure_connected()

        try:
            # Base APY of ~10% for FlareChain
            base_apy = 0.10

            yearly = int(amount * base_apy)
            monthly = yearly // 12
            daily = yearly // 365

            return {
                "daily": daily,
                "monthly": monthly,
                "yearly": yearly,
                "apy": base_apy * 100,  # As percentage
            }

        except Exception as e:
            raise StakingError(f"Failed to estimate rewards: {str(e)}")

    async def get_network_stats(self) -> Dict[str, Any]:
        """
        Get network staking statistics.

        Returns:
            Network-wide staking stats

        Example:
            >>> stats = await wrapper.get_network_stats()
            >>> print(f'Total staked: {stats["total_staked"] / 10**18} ÉTR')
            >>> print(f'Active era: {stats["active_era"]}')
        """
        self._ensure_connected()

        try:
            total_issuance = self.api.query(
                module="Balances",
                storage_function="TotalIssuance"
            )

            active_era = self.api.query(
                module="Staking",
                storage_function="ActiveEra"
            )

            era_reward = self.api.query(
                module="Staking",
                storage_function="ErasValidatorReward",
                params=[active_era.value.get('index', 0)] if active_era and active_era.value else [0]
            )

            return {
                "total_issuance": int(total_issuance.value) if total_issuance else 0,
                "active_era": int(active_era.value.get('index', 0)) if active_era and active_era.value else 0,
                "era_reward": int(era_reward.value) if era_reward and era_reward.value else 0,
            }

        except Exception as e:
            raise StakingError(f"Failed to get network stats: {str(e)}")
