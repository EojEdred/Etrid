"""AI DID Wrapper - AI Identity Standard

World's first AI identity standard on blockchain.
Enables AI agents to register, verify, and manage their digital identities.
"""

from typing import Dict, Any, List, Optional
from enum import Enum
from substrateinterface import SubstrateInterface, Keypair
from ..errors import (
    NotConnectedError,
    AIDidError,
    AIAlreadyRegisteredError,
    InvalidProfileError,
)


class AIType(Enum):
    """AI types supported by the DID system."""
    LLM = "LLM"                    # Large Language Model
    CV = "CV"                      # Computer Vision
    GENERATIVE = "Generative"      # Generative AI
    RL = "RL"                      # Reinforcement Learning
    NLP = "NLP"                    # Natural Language Processing


class ReputationTier(Enum):
    """Reputation tiers for AI agents."""
    UNVERIFIED = "Unverified"
    BRONZE = "Bronze"
    SILVER = "Silver"
    GOLD = "Gold"
    PLATINUM = "Platinum"


class AIDidWrapper:
    """
    Wrapper for AI DID pallet - World's first AI identity standard.

    Enables AI agents to register, verify identities, manage metadata,
    track reputation, and control permissions.
    """

    def __init__(self, api: SubstrateInterface):
        """
        Initialize AI DID wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def register_ai(
        self,
        keypair: Keypair,
        name: str,
        ai_type: str,
        api_endpoint: str,
        metadata: Dict[str, Any],
    ) -> Dict[str, Any]:
        """
        Register a new AI identity.

        Args:
            keypair: Account keypair for the AI
            name: AI name
            ai_type: AI type (use AIType enum values)
            api_endpoint: API endpoint for AI interaction
            metadata: Additional metadata (model, version, capabilities, etc.)

        Returns:
            Dictionary with ai_id, tx_hash, and registration details

        Raises:
            AIAlreadyRegisteredError: If AI already registered
            InvalidProfileError: If profile data is invalid

        Example:
            >>> result = await wrapper.register_ai(
            ...     ai_keypair,
            ...     "GPT-Assistant",
            ...     AIType.LLM.value,
            ...     "https://api.example.com",
            ...     {"model": "gpt-4", "version": "1.0"}
            ... )
            >>> print('AI ID:', result['ai_id'])
        """
        self._ensure_connected()

        try:
            # Validate AI type
            if ai_type not in [t.value for t in AIType]:
                raise InvalidProfileError(f"Invalid AI type: {ai_type}")

            call = self.api.compose_call(
                call_module="AIDid",
                call_function="registerAI",
                call_params={
                    "name": name,
                    "ai_type": ai_type,
                    "api_endpoint": api_endpoint,
                    "metadata": metadata,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                if "AlreadyRegistered" in str(receipt.error_message):
                    raise AIAlreadyRegisteredError(f"AI already registered: {keypair.ss58_address}")
                raise AIDidError(f"Failed to register AI: {receipt.error_message}")

            # Extract AIRegistered event
            for event in receipt.triggered_events:
                if event.event_module.name == "AIDid" and event.event.name == "AIRegistered":
                    ai_id = event.params[0]['value']
                    return {
                        "ai_id": ai_id,
                        "tx_hash": receipt.extrinsic_hash,
                        "name": name,
                        "ai_type": ai_type,
                        "api_endpoint": api_endpoint,
                        "owner": keypair.ss58_address,
                    }

            raise AIDidError("AI registered but event not found")

        except (AIAlreadyRegisteredError, InvalidProfileError):
            raise
        except Exception as e:
            raise AIDidError(f"Failed to register AI: {str(e)}")

    async def get_ai_profile(self, ai_id: str) -> Optional[Dict[str, Any]]:
        """
        Get AI profile information.

        Args:
            ai_id: AI identifier

        Returns:
            AI profile dictionary or None if not found

        Example:
            >>> profile = await wrapper.get_ai_profile(ai_id)
            >>> if profile:
            ...     print('Name:', profile['name'])
            ...     print('Type:', profile['ai_type'])
            ...     print('Reputation:', profile['reputation_score'])
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="AIDid",
                storage_function="AIProfiles",
                params=[ai_id]
            )

            if result.value is None:
                return None

            return {
                "ai_id": ai_id,
                "name": result.value.get('name', ''),
                "ai_type": result.value.get('ai_type', ''),
                "api_endpoint": result.value.get('api_endpoint', ''),
                "owner": result.value.get('owner', ''),
                "metadata": result.value.get('metadata', {}),
                "reputation_score": int(result.value.get('reputation_score', 0)),
                "reputation_tier": result.value.get('reputation_tier', ReputationTier.UNVERIFIED.value),
                "registered_at": int(result.value.get('registered_at', 0)),
                "verified": bool(result.value.get('verified', False)),
            }

        except Exception as e:
            raise AIDidError(f"Failed to get AI profile: {str(e)}")

    async def update_ai_metadata(
        self,
        keypair: Keypair,
        ai_id: str,
        metadata: Dict[str, Any],
    ) -> str:
        """
        Update AI metadata.

        Args:
            keypair: AI owner keypair
            ai_id: AI identifier
            metadata: Updated metadata

        Returns:
            Transaction hash

        Raises:
            AIDidError: If update fails

        Example:
            >>> tx_hash = await wrapper.update_ai_metadata(
            ...     ai_keypair,
            ...     ai_id,
            ...     {"version": "2.0", "capabilities": ["chat", "code"]}
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="AIDid",
                call_function="updateMetadata",
                call_params={
                    "ai_id": ai_id,
                    "metadata": metadata,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise AIDidError(f"Failed to update metadata: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise AIDidError(f"Failed to update metadata: {str(e)}")

    async def update_reputation(
        self,
        keypair: Keypair,
        ai_id: str,
        score_delta: int,
        reason: str,
    ) -> str:
        """
        Update AI reputation score.

        Args:
            keypair: Authorized account keypair
            ai_id: AI identifier
            score_delta: Score change (can be negative)
            reason: Reason for reputation change

        Returns:
            Transaction hash

        Raises:
            AIDidError: If update fails

        Example:
            >>> tx_hash = await wrapper.update_reputation(
            ...     admin_keypair,
            ...     ai_id,
            ...     100,
            ...     "Successfully completed 10 tasks"
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="AIDid",
                call_function="updateReputation",
                call_params={
                    "ai_id": ai_id,
                    "score_delta": score_delta,
                    "reason": reason,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise AIDidError(f"Failed to update reputation: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise AIDidError(f"Failed to update reputation: {str(e)}")

    async def get_reputation_tier(self, ai_id: str) -> str:
        """
        Get AI reputation tier.

        Args:
            ai_id: AI identifier

        Returns:
            Reputation tier (Unverified, Bronze, Silver, Gold, Platinum)

        Example:
            >>> tier = await wrapper.get_reputation_tier(ai_id)
            >>> print('Tier:', tier)
        """
        self._ensure_connected()

        try:
            profile = await self.get_ai_profile(ai_id)
            if not profile:
                return ReputationTier.UNVERIFIED.value

            return profile.get('reputation_tier', ReputationTier.UNVERIFIED.value)

        except Exception as e:
            raise AIDidError(f"Failed to get reputation tier: {str(e)}")

    async def grant_permission(
        self,
        keypair: Keypair,
        ai_id: str,
        grantee: str,
        permission: str,
    ) -> str:
        """
        Grant permission to an account.

        Args:
            keypair: AI owner keypair
            ai_id: AI identifier
            grantee: Account to grant permission to
            permission: Permission name (e.g., "read", "execute", "manage")

        Returns:
            Transaction hash

        Raises:
            AIDidError: If grant fails

        Example:
            >>> tx_hash = await wrapper.grant_permission(
            ...     ai_keypair,
            ...     ai_id,
            ...     user_address,
            ...     "execute"
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="AIDid",
                call_function="grantPermission",
                call_params={
                    "ai_id": ai_id,
                    "grantee": grantee,
                    "permission": permission,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise AIDidError(f"Failed to grant permission: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise AIDidError(f"Failed to grant permission: {str(e)}")

    async def revoke_permission(
        self,
        keypair: Keypair,
        ai_id: str,
        grantee: str,
        permission: str,
    ) -> str:
        """
        Revoke permission from an account.

        Args:
            keypair: AI owner keypair
            ai_id: AI identifier
            grantee: Account to revoke permission from
            permission: Permission name

        Returns:
            Transaction hash

        Raises:
            AIDidError: If revoke fails

        Example:
            >>> tx_hash = await wrapper.revoke_permission(
            ...     ai_keypair,
            ...     ai_id,
            ...     user_address,
            ...     "execute"
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="AIDid",
                call_function="revokePermission",
                call_params={
                    "ai_id": ai_id,
                    "grantee": grantee,
                    "permission": permission,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise AIDidError(f"Failed to revoke permission: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise AIDidError(f"Failed to revoke permission: {str(e)}")

    async def get_permissions(self, ai_id: str, account: str) -> List[str]:
        """
        Get permissions for an account.

        Args:
            ai_id: AI identifier
            account: Account address

        Returns:
            List of permission names

        Example:
            >>> permissions = await wrapper.get_permissions(ai_id, user_address)
            >>> print('Permissions:', permissions)
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="AIDid",
                storage_function="Permissions",
                params=[ai_id, account]
            )

            if result.value is None:
                return []

            return list(result.value) if isinstance(result.value, list) else []

        except Exception as e:
            raise AIDidError(f"Failed to get permissions: {str(e)}")

    async def verify_ai(
        self,
        keypair: Keypair,
        ai_id: str,
        verified: bool = True,
    ) -> str:
        """
        Verify or unverify an AI (admin only).

        Args:
            keypair: Admin keypair
            ai_id: AI identifier
            verified: Verification status

        Returns:
            Transaction hash

        Raises:
            AIDidError: If verification fails

        Example:
            >>> tx_hash = await wrapper.verify_ai(
            ...     admin_keypair,
            ...     ai_id,
            ...     True
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="AIDid",
                call_function="verifyAI",
                call_params={
                    "ai_id": ai_id,
                    "verified": verified,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise AIDidError(f"Failed to verify AI: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise AIDidError(f"Failed to verify AI: {str(e)}")
