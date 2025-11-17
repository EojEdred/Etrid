// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridGovernance
 * @dev Interface for Etrid Governance Precompile (0x801)
 *
 * Enables ETH PBC contracts to participate in FlareChain governance.
 * Proposals and votes are executed via XCM messages to FlareChain.
 *
 * Address: 0x0000000000000000000000000000000000000801
 *
 * @notice Unified governance across all 14 Etrid chains
 */
interface IEtridGovernance {
    /**
     * @notice Proposal status enum
     * @dev 0 = Pending, 1 = Active, 2 = Passed, 3 = Failed
     */
    enum ProposalStatus {
        Pending,
        Active,
        Passed,
        Failed
    }

    /**
     * @notice Submit a new governance proposal to FlareChain
     * @param title Proposal title (max 256 characters)
     * @param description Detailed proposal description (max 10,000 characters)
     * @return proposalId Unique proposal identifier
     *
     * @dev Example:
     *   uint256 proposalId = governance.submitProposal(
     *       "Enable New Feature",
     *       "This proposal enables feature X which provides benefit Y..."
     *   );
     *
     * @dev Emits ProposalSubmitted event on FlareChain
     */
    function submitProposal(string memory title, string memory description)
        external
        returns (uint256 proposalId);

    /**
     * @notice Vote on an active governance proposal
     * @param proposalId The proposal ID to vote on
     * @param support true = YES vote, false = NO vote
     *
     * @dev Example:
     *   governance.voteOnProposal(42, true); // Vote YES on proposal 42
     *
     * @dev Reverts if:
     *   - Proposal doesn't exist
     *   - Proposal is not in Active status
     *   - Caller has already voted
     *   - Caller has insufficient voting power
     */
    function voteOnProposal(uint256 proposalId, bool support) external;

    /**
     * @notice Get the current status of a proposal
     * @param proposalId The proposal ID to query
     * @return status Current proposal status (0-3)
     *
     * @dev Example:
     *   uint8 status = governance.getProposalStatus(42);
     *   require(status == uint8(ProposalStatus.Passed), "Proposal not passed");
     */
    function getProposalStatus(uint256 proposalId)
        external
        view
        returns (uint8 status);
}
