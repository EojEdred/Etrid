// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title GovernanceDAO
 * @notice Example DAO using Ëtrid Governance Precompile (0x801)
 * @dev Uses XCM to submit proposals and vote on Primearc Core Chain governance
 */

// Interface for Ëtrid Governance Precompile
interface IEtridGovernance {
    function submitProposal(string memory title, string memory description) external returns (uint256 proposalId);
    function voteOnProposal(uint256 proposalId, bool support) external;
    function getProposalStatus(uint256 proposalId) external view returns (uint8 status);
}

contract GovernanceDAO {
    // Governance precompile address
    IEtridGovernance private constant GOVERNANCE = IEtridGovernance(0x0000000000000000000000000000000000000801);

    // Proposal status enum (matches Primearc Core Chain)
    enum ProposalStatus {
        Pending,    // 0
        Active,     // 1
        Passed,     // 2
        Failed      // 3
    }

    struct Proposal {
        string title;
        string description;
        address proposer;
        uint256 primearcProposalId;
        uint256 timestamp;
    }

    mapping(uint256 => Proposal) public proposals;
    mapping(address => mapping(uint256 => bool)) public hasVoted;
    uint256 public proposalCount;

    event ProposalSubmitted(
        uint256 indexed localProposalId,
        uint256 indexed primearcProposalId,
        address indexed proposer,
        string title
    );

    event VoteCast(
        uint256 indexed localProposalId,
        uint256 indexed primearcProposalId,
        address indexed voter,
        bool support
    );

    event ProposalStatusChecked(
        uint256 indexed localProposalId,
        uint256 indexed primearcProposalId,
        ProposalStatus status
    );

    /**
     * @notice Submit a new governance proposal
     * @param title Proposal title
     * @param description Proposal description
     * @return localProposalId Local proposal ID
     * @return primearcProposalId Primearc Core Chain proposal ID
     */
    function submitProposal(
        string memory title,
        string memory description
    ) external returns (uint256 localProposalId, uint256 primearcProposalId) {
        // Submit proposal to Primearc Core Chain via XCM
        primearcProposalId = GOVERNANCE.submitProposal(title, description);

        // Store locally
        localProposalId = proposalCount++;
        proposals[localProposalId] = Proposal({
            title: title,
            description: description,
            proposer: msg.sender,
            primearcProposalId: primearcProposalId,
            timestamp: block.timestamp
        });

        emit ProposalSubmitted(localProposalId, primearcProposalId, msg.sender, title);

        return (localProposalId, primearcProposalId);
    }

    /**
     * @notice Vote on a proposal
     * @param localProposalId Local proposal ID
     * @param support True to vote yes, false to vote no
     */
    function vote(uint256 localProposalId, bool support) external {
        require(localProposalId < proposalCount, "Invalid proposal ID");
        require(!hasVoted[msg.sender][localProposalId], "Already voted");

        Proposal storage proposal = proposals[localProposalId];

        // Submit vote to Primearc Core Chain via XCM
        GOVERNANCE.voteOnProposal(proposal.primearcProposalId, support);

        // Mark as voted
        hasVoted[msg.sender][localProposalId] = true;

        emit VoteCast(localProposalId, proposal.primearcProposalId, msg.sender, support);
    }

    /**
     * @notice Get proposal status from Primearc Core Chain
     * @param localProposalId Local proposal ID
     * @return status Proposal status (0=Pending, 1=Active, 2=Passed, 3=Failed)
     */
    function getProposalStatus(uint256 localProposalId) external view returns (ProposalStatus status) {
        require(localProposalId < proposalCount, "Invalid proposal ID");

        Proposal storage proposal = proposals[localProposalId];
        uint8 statusCode = GOVERNANCE.getProposalStatus(proposal.primearcProposalId);

        return ProposalStatus(statusCode);
    }

    /**
     * @notice Check and emit proposal status
     * @param localProposalId Local proposal ID
     */
    function checkProposalStatus(uint256 localProposalId) external {
        require(localProposalId < proposalCount, "Invalid proposal ID");

        Proposal storage proposal = proposals[localProposalId];
        uint8 statusCode = GOVERNANCE.getProposalStatus(proposal.primearcProposalId);
        ProposalStatus status = ProposalStatus(statusCode);

        emit ProposalStatusChecked(localProposalId, proposal.primearcProposalId, status);
    }

    /**
     * @notice Get proposal details
     * @param localProposalId Local proposal ID
     * @return Proposal details
     */
    function getProposal(uint256 localProposalId) external view returns (
        string memory title,
        string memory description,
        address proposer,
        uint256 primearcProposalId,
        uint256 timestamp
    ) {
        require(localProposalId < proposalCount, "Invalid proposal ID");

        Proposal storage proposal = proposals[localProposalId];
        return (
            proposal.title,
            proposal.description,
            proposal.proposer,
            proposal.primearcProposalId,
            proposal.timestamp
        );
    }

    /**
     * @notice Check if an address has voted on a proposal
     * @param voter Voter address
     * @param localProposalId Local proposal ID
     * @return voted True if already voted
     */
    function hasUserVoted(address voter, uint256 localProposalId) external view returns (bool voted) {
        return hasVoted[voter][localProposalId];
    }
}

/**
 * @title MultiSigGovernance
 * @notice Multi-signature governance using Ëtrid Governance Precompile
 * @dev Requires multiple approvals before submitting proposal to Primearc Core Chain
 */
contract MultiSigGovernance {
    IEtridGovernance private constant GOVERNANCE = IEtridGovernance(0x0000000000000000000000000000000000000801);

    address[] public signers;
    uint256 public requiredSignatures;

    struct PendingProposal {
        string title;
        string description;
        address[] approvers;
        bool submitted;
        uint256 primearcProposalId;
    }

    mapping(uint256 => PendingProposal) public pendingProposals;
    uint256 public pendingProposalCount;

    event ProposalCreated(uint256 indexed proposalId, address indexed creator, string title);
    event ProposalApproved(uint256 indexed proposalId, address indexed approver);
    event ProposalSubmitted(uint256 indexed proposalId, uint256 indexed primearcProposalId);

    constructor(address[] memory _signers, uint256 _requiredSignatures) {
        require(_signers.length >= _requiredSignatures, "Invalid signer count");
        require(_requiredSignatures > 0, "Require at least 1 signature");

        signers = _signers;
        requiredSignatures = _requiredSignatures;
    }

    /**
     * @notice Create a pending proposal
     * @param title Proposal title
     * @param description Proposal description
     * @return proposalId Pending proposal ID
     */
    function createProposal(
        string memory title,
        string memory description
    ) external returns (uint256 proposalId) {
        require(isSigner(msg.sender), "Not a signer");

        proposalId = pendingProposalCount++;
        address[] memory approvers = new address[](1);
        approvers[0] = msg.sender;

        pendingProposals[proposalId] = PendingProposal({
            title: title,
            description: description,
            approvers: approvers,
            submitted: false,
            primearcProposalId: 0
        });

        emit ProposalCreated(proposalId, msg.sender, title);

        // Auto-submit if only 1 signature required
        if (requiredSignatures == 1) {
            _submitProposal(proposalId);
        }

        return proposalId;
    }

    /**
     * @notice Approve a pending proposal
     * @param proposalId Pending proposal ID
     */
    function approveProposal(uint256 proposalId) external {
        require(isSigner(msg.sender), "Not a signer");
        require(proposalId < pendingProposalCount, "Invalid proposal ID");

        PendingProposal storage proposal = pendingProposals[proposalId];
        require(!proposal.submitted, "Already submitted");
        require(!hasApproved(proposalId, msg.sender), "Already approved");

        proposal.approvers.push(msg.sender);

        emit ProposalApproved(proposalId, msg.sender);

        // Submit if threshold reached
        if (proposal.approvers.length >= requiredSignatures) {
            _submitProposal(proposalId);
        }
    }

    /**
     * @notice Submit approved proposal to Primearc Core Chain
     * @param proposalId Pending proposal ID
     */
    function _submitProposal(uint256 proposalId) internal {
        PendingProposal storage proposal = pendingProposals[proposalId];

        uint256 primearcProposalId = GOVERNANCE.submitProposal(
            proposal.title,
            proposal.description
        );

        proposal.submitted = true;
        proposal.primearcProposalId = primearcProposalId;

        emit ProposalSubmitted(proposalId, primearcProposalId);
    }

    /**
     * @notice Check if address is a signer
     * @param addr Address to check
     * @return isSigner_ True if address is a signer
     */
    function isSigner(address addr) public view returns (bool isSigner_) {
        for (uint256 i = 0; i < signers.length; i++) {
            if (signers[i] == addr) {
                return true;
            }
        }
        return false;
    }

    /**
     * @notice Check if address has approved a proposal
     * @param proposalId Proposal ID
     * @param addr Address to check
     * @return approved True if address has approved
     */
    function hasApproved(uint256 proposalId, address addr) public view returns (bool approved) {
        PendingProposal storage proposal = pendingProposals[proposalId];
        for (uint256 i = 0; i < proposal.approvers.length; i++) {
            if (proposal.approvers[i] == addr) {
                return true;
            }
        }
        return false;
    }
}
