// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "../IEtridGovernance.sol";

/**
 * @title DAO Governance Example
 * @notice Demonstrates using Ëtrid Governance precompile for DAO operations
 * @dev This shows how a DAO on ETH-PBC can integrate with FlareChain governance
 */
contract DAOGovernanceExample {
    IEtridGovernance private constant GOV =
        IEtridGovernance(0x0000000000000000000000000000000000000801);

    mapping(address => bool) public members;
    mapping(uint256 => bool) public daoProposals; // Track which proposals came from this DAO

    event ProposalSubmitted(uint256 indexed proposalId, string title);
    event VoteCast(uint256 indexed proposalId, address indexed voter, bool support);

    modifier onlyMember() {
        require(members[msg.sender], "Not a DAO member");
        _;
    }

    constructor() {
        members[msg.sender] = true; // Creator is first member
    }

    /**
     * @notice Add a new DAO member
     * @param member Address to add as member
     */
    function addMember(address member) external onlyMember {
        members[member] = true;
    }

    /**
     * @notice Submit a proposal to FlareChain governance
     * @param title Short proposal title
     * @param description Detailed proposal description
     * @return proposalId The ID of the created proposal
     */
    function submitProposal(string memory title, string memory description)
        external
        onlyMember
        returns (uint256 proposalId)
    {
        // Submit to FlareChain governance via precompile
        proposalId = GOV.submitProposal(title, description);

        // Track that this proposal came from our DAO
        daoProposals[proposalId] = true;

        emit ProposalSubmitted(proposalId, title);

        return proposalId;
    }

    /**
     * @notice Vote on a proposal
     * @param proposalId ID of proposal to vote on
     * @param support True for YES, false for NO
     */
    function vote(uint256 proposalId, bool support) external onlyMember {
        // Cast vote on FlareChain via precompile
        GOV.voteOnProposal(proposalId, support);

        emit VoteCast(proposalId, msg.sender, support);
    }

    /**
     * @notice Check status of a proposal
     * @param proposalId ID of proposal to check
     * @return status 0=Pending, 1=Active, 2=Passed, 3=Failed
     */
    function getProposalStatus(uint256 proposalId)
        external
        view
        returns (uint8 status)
    {
        status = GOV.getProposalStatus(proposalId);
        return status;
    }

    /**
     * @notice Execute logic based on proposal outcome
     * @param proposalId ID of proposal to check
     * @dev In production, this would trigger actual changes if proposal passed
     */
    function executeIfPassed(uint256 proposalId) external onlyMember {
        uint8 status = GOV.getProposalStatus(proposalId);
        require(status == 2, "Proposal not passed");
        require(daoProposals[proposalId], "Not our proposal");

        // Execute proposal logic here
        // For example:
        // - Update contract parameters
        // - Transfer funds
        // - Enable/disable features
    }
}
