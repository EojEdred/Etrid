"""Governance Wrapper - On-Chain Governance

Participate in on-chain governance through proposals and voting on FlareChain.
"""

from typing import Dict, Any, List, Optional
from enum import Enum
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, GovernanceError, ProposalNotFoundError


class ProposalStatus(Enum):
    """Proposal status states."""
    PENDING = "Pending"
    ACTIVE = "Active"
    APPROVED = "Approved"
    REJECTED = "Rejected"
    EXECUTED = "Executed"
    CANCELLED = "Cancelled"


class VoteType(Enum):
    """Vote conviction levels."""
    NONE = 0       # No conviction, 0.1x voting weight
    LOCKED_1X = 1  # 1x voting weight, locked for 1 period
    LOCKED_2X = 2  # 2x voting weight, locked for 2 periods
    LOCKED_3X = 3  # 3x voting weight, locked for 3 periods
    LOCKED_4X = 4  # 4x voting weight, locked for 4 periods
    LOCKED_5X = 5  # 5x voting weight, locked for 5 periods
    LOCKED_6X = 6  # 6x voting weight, locked for 6 periods


class GovernanceWrapper:
    """
    Wrapper for Governance pallet - On-chain proposals and voting.

    Enables community participation in network governance through democratic
    proposal creation, voting, delegation, and execution.
    """

    # Governance constants
    MIN_PROPOSAL_DEPOSIT = 1000 * 10**18  # Minimum 1000 ÉTR deposit
    VOTING_PERIOD = 100800                 # ~7 days (in blocks)
    ENACTMENT_PERIOD = 100800              # ~7 days delay before execution

    def __init__(self, api: SubstrateInterface):
        """
        Initialize Governance wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def create_proposal(
        self,
        keypair: Keypair,
        title: str,
        description: str,
        call_hash: str,
        deposit: int,
    ) -> Dict[str, Any]:
        """
        Create a governance proposal.

        Args:
            keypair: Proposer keypair
            title: Proposal title
            description: Detailed description
            call_hash: Hash of the proposed call
            deposit: Proposal deposit (in planck)

        Returns:
            Dictionary with proposal_id, tx_hash, title, and deposit

        Raises:
            GovernanceError: If proposal creation fails

        Example:
            >>> proposal = await wrapper.create_proposal(
            ...     alice,
            ...     "Increase Block Gas Limit",
            ...     "Proposal to increase block gas limit to 15M VMw",
            ...     "0x1234...",
            ...     1000 * 10**18
            ... )
            >>> print('Proposal ID:', proposal['proposal_id'])
        """
        self._ensure_connected()

        try:
            if deposit < self.MIN_PROPOSAL_DEPOSIT:
                raise GovernanceError(f"Deposit below minimum: {self.MIN_PROPOSAL_DEPOSIT}")

            call = self.api.compose_call(
                call_module="Governance",
                call_function="propose",
                call_params={
                    "title": title,
                    "description": description,
                    "proposal_hash": call_hash,
                    "deposit": deposit,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Proposal creation failed: {receipt.error_message}")

            for event in receipt.triggered_events:
                if event.event_module.name == "Governance" and event.event.name == "ProposalCreated":
                    return {
                        "proposal_id": event.params[0]['value'],
                        "tx_hash": receipt.extrinsic_hash,
                        "title": title,
                        "deposit": deposit,
                    }

            raise GovernanceError("Proposal created but event not found")

        except Exception as e:
            raise GovernanceError(f"Failed to create proposal: {str(e)}")

    async def vote(
        self,
        keypair: Keypair,
        proposal_id: int,
        approve: bool,
        stake: int,
        conviction: int = VoteType.NONE.value,
    ) -> str:
        """
        Vote on a governance proposal.

        Args:
            keypair: Voter keypair
            proposal_id: Proposal ID
            approve: True to vote yes, False to vote no
            stake: Voting stake (in planck)
            conviction: Vote conviction level (use VoteType enum)

        Returns:
            Transaction hash

        Raises:
            GovernanceError: If vote fails

        Example:
            >>> tx_hash = await wrapper.vote(
            ...     alice,
            ...     1,
            ...     True,
            ...     100 * 10**18,
            ...     VoteType.LOCKED_3X.value
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Governance",
                call_function="vote",
                call_params={
                    "proposal_id": proposal_id,
                    "approve": approve,
                    "stake": stake,
                    "conviction": conviction,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Vote failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise GovernanceError(f"Failed to vote: {str(e)}")

    async def remove_vote(self, keypair: Keypair, proposal_id: int) -> str:
        """
        Remove vote from proposal.

        Args:
            keypair: Voter keypair
            proposal_id: Proposal ID

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.remove_vote(alice, 1)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Governance",
                call_function="removeVote",
                call_params={"proposal_id": proposal_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Remove vote failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise GovernanceError(f"Failed to remove vote: {str(e)}")

    async def execute_proposal(self, keypair: Keypair, proposal_id: int) -> str:
        """
        Execute an approved proposal.

        Args:
            keypair: Executor keypair
            proposal_id: Proposal ID to execute

        Returns:
            Transaction hash

        Raises:
            GovernanceError: If execution fails

        Example:
            >>> tx_hash = await wrapper.execute_proposal(alice, 1)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Governance",
                call_function="execute",
                call_params={"proposal_id": proposal_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Execution failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise GovernanceError(f"Failed to execute proposal: {str(e)}")

    async def cancel_proposal(self, keypair: Keypair, proposal_id: int) -> str:
        """
        Cancel a proposal (proposer only).

        Args:
            keypair: Proposer keypair
            proposal_id: Proposal ID to cancel

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.cancel_proposal(alice, 1)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Governance",
                call_function="cancel",
                call_params={"proposal_id": proposal_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Cancel failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise GovernanceError(f"Failed to cancel proposal: {str(e)}")

    async def get_active_proposals(self) -> List[Dict[str, Any]]:
        """
        Get all active proposals.

        Returns:
            List of active proposal dictionaries

        Example:
            >>> proposals = await wrapper.get_active_proposals()
            >>> for p in proposals:
            ...     print(f"{p['title']}: {p['status']}")
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Governance",
                storage_function="ActiveProposals"
            )

            if result.value is None:
                return []

            return list(result.value) if isinstance(result.value, list) else []

        except Exception as e:
            raise GovernanceError(f"Failed to get active proposals: {str(e)}")

    async def get_proposal(self, proposal_id: int) -> Dict[str, Any]:
        """
        Get proposal details.

        Args:
            proposal_id: Proposal ID

        Returns:
            Proposal details dictionary

        Raises:
            ProposalNotFoundError: If proposal not found

        Example:
            >>> proposal = await wrapper.get_proposal(1)
            >>> print(f'{proposal["title"]}: {proposal["status"]}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Governance",
                storage_function="Proposals",
                params=[proposal_id]
            )

            if result.value is None:
                raise ProposalNotFoundError(f"Proposal {proposal_id} not found")

            return {
                "proposal_id": proposal_id,
                "title": result.value.get('title', ''),
                "description": result.value.get('description', ''),
                "proposer": result.value.get('proposer', ''),
                "deposit": int(result.value.get('deposit', 0)),
                "status": result.value.get('status', ProposalStatus.PENDING.value),
                "call_hash": result.value.get('call_hash', ''),
                "created_at": int(result.value.get('created_at', 0)),
                "voting_ends_at": int(result.value.get('voting_ends_at', 0)),
            }

        except ProposalNotFoundError:
            raise
        except Exception as e:
            raise GovernanceError(f"Failed to get proposal: {str(e)}")

    async def get_proposal_results(self, proposal_id: int) -> Dict[str, Any]:
        """
        Get voting results for proposal.

        Args:
            proposal_id: Proposal ID

        Returns:
            Vote tally dictionary

        Raises:
            ProposalNotFoundError: If proposal not found

        Example:
            >>> results = await wrapper.get_proposal_results(1)
            >>> print(f'Ayes: {results["ayes"]}')
            >>> print(f'Nays: {results["nays"]}')
            >>> print(f'Turnout: {results["turnout"]}%')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Governance",
                storage_function="ProposalVotes",
                params=[proposal_id]
            )

            if result.value is None:
                raise ProposalNotFoundError(f"Proposal {proposal_id} not found")

            ayes = int(result.value.get('ayes', 0))
            nays = int(result.value.get('nays', 0))
            total_stake = int(result.value.get('total_stake', 0))

            # Calculate turnout percentage
            total_issuance = self.api.query(
                module="Balances",
                storage_function="TotalIssuance"
            )
            issuance = int(total_issuance.value) if total_issuance else 1
            turnout = (total_stake / issuance) * 100 if issuance > 0 else 0

            return {
                "proposal_id": proposal_id,
                "ayes": ayes,
                "nays": nays,
                "total_stake": total_stake,
                "turnout": turnout,
                "approval_rate": (ayes / (ayes + nays) * 100) if (ayes + nays) > 0 else 0,
            }

        except ProposalNotFoundError:
            raise
        except Exception as e:
            raise GovernanceError(f"Failed to get proposal results: {str(e)}")

    async def get_proposal_history(self, limit: int = 10) -> List[Dict[str, Any]]:
        """
        Get proposal history.

        Args:
            limit: Maximum number of proposals to return

        Returns:
            List of historical proposals

        Example:
            >>> history = await wrapper.get_proposal_history(limit=5)
            >>> for p in history:
            ...     print(f'{p["title"]}: {p["status"]}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Governance",
                storage_function="ProposalHistory"
            )

            if result.value is None:
                return []

            history = result.value if isinstance(result.value, list) else []
            return history[:limit]

        except Exception as e:
            raise GovernanceError(f"Failed to get proposal history: {str(e)}")

    async def get_delegations(self, address: str) -> List[Dict[str, Any]]:
        """
        Get vote delegations for address.

        Args:
            address: Account address

        Returns:
            List of delegation records

        Example:
            >>> delegations = await wrapper.get_delegations(alice_address)
            >>> print(f'{len(delegations)} delegations')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Governance",
                storage_function="Delegations",
                params=[address]
            )

            if result.value is None:
                return []

            return list(result.value) if isinstance(result.value, list) else []

        except Exception as e:
            raise GovernanceError(f"Failed to get delegations: {str(e)}")

    async def delegate_votes(
        self,
        keypair: Keypair,
        delegate_to: str,
        conviction: int = VoteType.NONE.value,
    ) -> str:
        """
        Delegate voting power to another account.

        Args:
            keypair: Delegator keypair
            delegate_to: Delegate address
            conviction: Vote conviction level

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.delegate_votes(
            ...     alice,
            ...     bob_address,
            ...     VoteType.LOCKED_2X.value
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Governance",
                call_function="delegate",
                call_params={
                    "to": delegate_to,
                    "conviction": conviction,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Delegation failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise GovernanceError(f"Failed to delegate votes: {str(e)}")

    async def undelegate_votes(self, keypair: Keypair) -> str:
        """
        Remove vote delegation.

        Args:
            keypair: Delegator keypair

        Returns:
            Transaction hash

        Example:
            >>> tx_hash = await wrapper.undelegate_votes(alice)
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Governance",
                call_function="undelegate"
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise GovernanceError(f"Undelegation failed: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise GovernanceError(f"Failed to undelegate votes: {str(e)}")

    async def get_governance_stats(self) -> Dict[str, Any]:
        """
        Get governance statistics.

        Returns:
            Network-wide governance stats

        Example:
            >>> stats = await wrapper.get_governance_stats()
            >>> print(f'Active: {stats["active_proposals"]}')
            >>> print(f'Participation: {stats["participation_rate"]}%')
        """
        self._ensure_connected()

        try:
            active = await self.get_active_proposals()

            # Get total proposals count
            history = await self.get_proposal_history(limit=1000)

            return {
                "active_proposals": len(active),
                "total_proposals": len(history),
                "participation_rate": 0.0,  # Would calculate from voting data
                "average_turnout": 0.0,
            }

        except Exception as e:
            raise GovernanceError(f"Failed to get governance stats: {str(e)}")

    async def estimate_proposal_outcome(self, proposal_id: int) -> Dict[str, Any]:
        """
        Estimate proposal outcome based on current votes.

        Args:
            proposal_id: Proposal ID

        Returns:
            Outcome prediction dictionary

        Example:
            >>> estimate = await wrapper.estimate_proposal_outcome(1)
            >>> print(f'Likely outcome: {estimate["likely_outcome"]}')
            >>> print(f'Confidence: {estimate["confidence"] * 100}%')
        """
        self._ensure_connected()

        try:
            results = await self.get_proposal_results(proposal_id)
            total = results['ayes'] + results['nays']

            if total == 0:
                return {
                    "likely_outcome": "unknown",
                    "confidence": 0.0,
                    "votes_needed": 0,
                }

            approval_rate = results['ayes'] / total

            return {
                "likely_outcome": "pass" if approval_rate > 0.5 else "fail",
                "confidence": approval_rate,
                "votes_needed": max(0, int((total / 2) - results['ayes'] + 1)),
                "approval_rate": approval_rate * 100,
            }

        except Exception as e:
            raise GovernanceError(f"Failed to estimate outcome: {str(e)}")
