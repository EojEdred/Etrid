// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IEtridGovernance
 * @notice Interface for interacting with FlareChain governance from ETH-PBC
 * @dev Precompile address: 0x0000000000000000000000000000000000000801
 *
 * This precompile allows Solidity contracts on ETH-PBC to participate in
 * FlareChain governance by submitting proposals and voting via XCM messaging.
 *
 * Proposal Status Values:
 * - 0: Pending (proposal submitted, not yet active)
 * - 1: Active (voting in progress)
 * - 2: Passed (proposal approved and executed)
 * - 3: Failed (proposal rejected or expired)
 *
 * Example Usage:
 * ```solidity
 * IEtridGovernance gov = IEtridGovernance(0x0000000000000000000000000000000000000801);
 * uint256 proposalId = gov.submitProposal("Upgrade Protocol", "Detailed proposal...");
 * gov.voteOnProposal(proposalId, true); // Vote YES
 * ```
 */
interface IEtridGovernance {
    /**
     * @notice Submit a governance proposal to FlareChain
     * @param title Short title for the proposal (max 256 characters)
     * @param description Detailed description (max 10,000 characters)
     * @return proposalId Unique identifier for the created proposal
     *
     * Requirements:
     * - Caller must have sufficient stake on FlareChain (checked via XCM)
     * - Title must be non-empty and <= 256 characters
     * - Description must be non-empty and <= 10,000 characters
     *
     * Example:
     * ```solidity
     * uint256 id = gov.submitProposal(
     *     "Enable ETH-PBC Feature X",
     *     "This proposal enables feature X which will..."
     * );
     * ```
     */
    function submitProposal(string memory title, string memory description)
        external
        returns (uint256 proposalId);

    /**
     * @notice Vote on an active governance proposal
     * @param proposalId The ID of the proposal to vote on
     * @param support True for YES, false for NO
     *
     * Requirements:
     * - Proposal must be in Active status
     * - Caller must have voting rights on FlareChain
     * - Caller cannot vote twice on same proposal
     *
     * Example:
     * ```solidity
     * gov.voteOnProposal(42, true);  // Vote YES on proposal 42
     * gov.voteOnProposal(43, false); // Vote NO on proposal 43
     * ```
     */
    function voteOnProposal(uint256 proposalId, bool support) external;

    /**
     * @notice Get the current status of a proposal
     * @param proposalId The ID of the proposal to query
     * @return status The current status (0=Pending, 1=Active, 2=Passed, 3=Failed)
     *
     * Example:
     * ```solidity
     * uint8 status = gov.getProposalStatus(42);
     * if (status == 1) {
     *     // Proposal is active, can vote
     * }
     * ```
     */
    function getProposalStatus(uint256 proposalId) external view returns (uint8 status);
}
