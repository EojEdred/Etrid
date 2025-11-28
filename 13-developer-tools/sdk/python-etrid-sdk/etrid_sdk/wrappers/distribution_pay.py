"""Distribution Pay Wrapper - Daily Rewards Distribution

Daily distribution of 27,397 ÉTR across 5 categories:
- Validators: Block production rewards
- Nominators: Validator support rewards
- Developers: Smart contract deployment rewards
- Community: Governance participation rewards
- Liquidity Providers: DEX liquidity rewards
"""

from typing import Dict, Any, List
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, DistributionError, NotEligibleError


class DistributionCategory:
    """Distribution reward categories."""
    VALIDATOR = "Validator"
    NOMINATOR = "Nominator"
    DEVELOPER = "Developer"
    COMMUNITY = "Community"
    LIQUIDITY_PROVIDER = "LiquidityProvider"


class DistributionPayWrapper:
    """Wrapper for Distribution Pay pallet - 27,397 ÉTR daily rewards."""

    def __init__(self, api: SubstrateInterface):
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def claim_reward(self, keypair: Keypair, category: str) -> str:
        """
        Claim reward for given category.

        Args:
            keypair: Account keypair to claim with
            category: Reward category (use DistributionCategory constants)

        Returns:
            Transaction hash

        Raises:
            NotEligibleError: If not eligible for category
            DistributionError: If claim fails
        """
        self._ensure_connected()

        try:
            # Check eligibility first
            if not await self.is_eligible(keypair.ss58_address, category):
                raise NotEligibleError(f"Not eligible for {category} rewards")

            # Create claim extrinsic
            call = self.api.compose_call(
                call_module="DistributionPay",
                call_function="claimReward",
                call_params={"category": category}
            )

            # Sign and submit
            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise DistributionError(f"Claim failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise DistributionError(f"Failed to claim reward: {str(e)}")

    async def get_pending_rewards(self, address: str) -> Dict[str, int]:
        """
        Get pending rewards by category.

        Args:
            address: Account address (SS58 format)

        Returns:
            Dictionary mapping category name to pending amount (in planck)
        """
        self._ensure_connected()

        try:
            rewards = {}

            for category in [
                DistributionCategory.VALIDATOR,
                DistributionCategory.NOMINATOR,
                DistributionCategory.DEVELOPER,
                DistributionCategory.COMMUNITY,
                DistributionCategory.LIQUIDITY_PROVIDER
            ]:
                result = self.api.query(
                    module="DistributionPay",
                    storage_function="PendingRewards",
                    params=[address, category]
                )

                rewards[category] = int(result.value) if result else 0

            return rewards

        except Exception as e:
            raise DistributionError(f"Failed to get pending rewards: {str(e)}")

    async def is_eligible(self, address: str, category: str) -> bool:
        """
        Check if address is eligible for category rewards.

        Args:
            address: Account address
            category: Reward category

        Returns:
            True if eligible, False otherwise
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="DistributionPay",
                storage_function="Eligibility",
                params=[address, category]
            )

            return bool(result.value) if result else False

        except Exception as e:
            raise DistributionError(f"Failed to check eligibility: {str(e)}")

    async def get_eligible_categories(self, address: str) -> List[str]:
        """
        Get list of categories address is eligible for.

        Args:
            address: Account address

        Returns:
            List of eligible category names
        """
        self._ensure_connected()

        eligible = []

        for category in [
            DistributionCategory.VALIDATOR,
            DistributionCategory.NOMINATOR,
            DistributionCategory.DEVELOPER,
            DistributionCategory.COMMUNITY,
            DistributionCategory.LIQUIDITY_PROVIDER
        ]:
            if await self.is_eligible(address, category):
                eligible.append(category)

        return eligible

    async def get_distribution_schedule(self) -> Dict[str, Any]:
        """
        Get current distribution schedule.

        Returns:
            Dictionary with daily amounts per category
        """
        self._ensure_connected()

        try:
            schedule = self.api.query(
                module="DistributionPay",
                storage_function="DistributionSchedule"
            )

            return {
                "total_daily": 27397 * 10**18,  # 27,397 ÉTR
                "validator": int(schedule.value.get("validator", 0)) if schedule else 0,
                "nominator": int(schedule.value.get("nominator", 0)) if schedule else 0,
                "developer": int(schedule.value.get("developer", 0)) if schedule else 0,
                "community": int(schedule.value.get("community", 0)) if schedule else 0,
                "liquidity_provider": int(schedule.value.get("liquidity_provider", 0)) if schedule else 0,
            }

        except Exception as e:
            raise DistributionError(f"Failed to get distribution schedule: {str(e)}")

    async def get_claim_history(self, address: str, limit: int = 10) -> List[Dict[str, Any]]:
        """
        Get recent claim history for address.

        Args:
            address: Account address
            limit: Maximum number of claims to return

        Returns:
            List of claim records
        """
        self._ensure_connected()

        try:
            # Query claim events from recent blocks
            claims = []

            current_block = self.api.get_block_number(None)

            # Search last 1000 blocks
            for block_num in range(current_block, max(0, current_block - 1000), -1):
                block_hash = self.api.get_block_hash(block_num)
                events = self.api.get_events(block_hash)

                for event in events:
                    if (event.value['module_id'] == 'DistributionPay' and
                        event.value['event_id'] == 'RewardClaimed'):

                        event_data = event.value['attributes']
                        if event_data.get('who') == address:
                            claims.append({
                                "block": block_num,
                                "category": event_data.get('category'),
                                "amount": int(event_data.get('amount', 0)),
                                "timestamp": event_data.get('timestamp')
                            })

                            if len(claims) >= limit:
                                return claims

            return claims

        except Exception as e:
            raise DistributionError(f"Failed to get claim history: {str(e)}")
