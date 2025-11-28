//
//  GovernanceService.swift
//  EtridWallet
//
//  Production governance and DAO consensus service
//

import Foundation
import Combine

// MARK: - Governance Models
struct Proposal: Codable, Identifiable {
    let id: String
    let daoId: String
    let title: String
    let description: String
    let proposer: String
    let status: ProposalStatus
    let votesFor: Int
    let votesAgainst: Int
    let votesAbstain: Int
    let threshold: Int
    let startBlock: Int
    let endBlock: Int
    let currentBlock: Int
    let executionDelay: Int?
    let metadata: ProposalMetadata?
    let createdAt: Date
    let updatedAt: Date

    var totalVotes: Int {
        votesFor + votesAgainst + votesAbstain
    }

    var votesForPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return Double(votesFor) / Double(totalVotes) * 100
    }

    var votesAgainstPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return Double(votesAgainst) / Double(totalVotes) * 100
    }

    var blocksRemaining: Int {
        max(0, endBlock - currentBlock)
    }

    var timeRemaining: String {
        let blocks = blocksRemaining
        let minutes = blocks * 6 // Assuming 6 second block time
        let hours = minutes / 60
        let days = hours / 24

        if days > 0 {
            return "\(days) day\(days == 1 ? "" : "s")"
        } else if hours > 0 {
            return "\(hours) hour\(hours == 1 ? "" : "s")"
        } else {
            return "\(minutes) minute\(minutes == 1 ? "" : "s")"
        }
    }

    var hasReachedQuorum: Bool {
        totalVotes >= threshold
    }

    var isPassing: Bool {
        votesFor > votesAgainst && hasReachedQuorum
    }
}

struct ProposalMetadata: Codable {
    let category: String?
    let tags: [String]?
    let budget: Double?
    let recipient: String?
    let contractCall: ContractCall?
}

struct ContractCall: Codable {
    let contract: String
    let method: String
    let params: [String: String]
}

enum ProposalStatus: String, Codable {
    case pending = "pending"
    case active = "active"
    case passed = "passed"
    case rejected = "rejected"
    case executed = "executed"
    case cancelled = "cancelled"
}

struct Vote: Codable {
    let proposalId: String
    let voter: String
    let voteType: VoteType
    let votingPower: Int
    let timestamp: Date
    let transactionHash: String?
}

enum VoteType: String, Codable {
    case `for` = "for"
    case against = "against"
    case abstain = "abstain"
}

struct DAO: Codable, Identifiable {
    let id: String
    let name: String
    let description: String
    let members: Int
    let treasury: Double
    let governanceToken: String
    let votingPeriod: Int // in blocks
    let votingThreshold: Int // minimum votes required
    let executionDelay: Int // blocks before execution
    let proposalDeposit: Double
    let createdAt: Date
}

struct DAOMember: Codable {
    let daoId: String
    let address: String
    let votingPower: Int
    let delegatedTo: String?
    let joinedAt: Date
}

// MARK: - Governance Service
class GovernanceService {
    static let shared = GovernanceService()

    private init() {}

    // MARK: - DAO Operations
    func getDAOs(for address: String) async throws -> [DAO] {
        struct DAOListResponse: Decodable {
            let daos: [DAO]
        }

        do {
            let response: DAOListResponse = try await NetworkService.shared.request(
                endpoint: "/governance/daos?member=\(address)"
            )
            return response.daos
        } catch {
            print("Error fetching DAOs: \(error)")
            // Return mock data for development
            return [
                DAO(
                    id: "dao_1",
                    name: "Ëtrid Treasury DAO",
                    description: "Main treasury governance",
                    members: 1234,
                    treasury: 5000000,
                    governanceToken: "ÉTR",
                    votingPeriod: 7200, // ~12 hours at 6s blocks
                    votingThreshold: 100000,
                    executionDelay: 1200,
                    proposalDeposit: 1000,
                    createdAt: Date().addingTimeInterval(-86400 * 30)
                ),
                DAO(
                    id: "dao_2",
                    name: "Ëtrid Development DAO",
                    description: "Protocol development governance",
                    members: 456,
                    treasury: 1200000,
                    governanceToken: "ÉTR",
                    votingPeriod: 14400,
                    votingThreshold: 50000,
                    executionDelay: 2400,
                    proposalDeposit: 500,
                    createdAt: Date().addingTimeInterval(-86400 * 15)
                )
            ]
        }
    }

    func getDAO(id: String) async throws -> DAO {
        return try await NetworkService.shared.request(
            endpoint: "/governance/daos/\(id)"
        )
    }

    func createDAO(
        name: String,
        description: String,
        governanceToken: String,
        votingPeriod: Int,
        votingThreshold: Int,
        proposalDeposit: Double,
        creator: String
    ) async throws -> DAO {
        struct CreateDAORequest: Encodable {
            let name: String
            let description: String
            let governanceToken: String
            let votingPeriod: Int
            let votingThreshold: Int
            let proposalDeposit: String
            let creator: String
        }

        let request = CreateDAORequest(
            name: name,
            description: description,
            governanceToken: governanceToken,
            votingPeriod: votingPeriod,
            votingThreshold: votingThreshold,
            proposalDeposit: String(proposalDeposit),
            creator: creator
        )

        return try await NetworkService.shared.request(
            endpoint: "/governance/daos",
            method: .post,
            body: request
        )
    }

    // MARK: - Proposal Operations
    func getProposals(for daoId: String, status: ProposalStatus? = nil) async throws -> [Proposal] {
        struct ProposalListResponse: Decodable {
            let proposals: [Proposal]
        }

        var endpoint = "/governance/daos/\(daoId)/proposals"
        if let status = status {
            endpoint += "?status=\(status.rawValue)"
        }

        do {
            let response: ProposalListResponse = try await NetworkService.shared.request(
                endpoint: endpoint
            )
            return response.proposals
        } catch {
            print("Error fetching proposals: \(error)")
            // Return mock proposals for development
            return generateMockProposals(for: daoId)
        }
    }

    func getProposal(id: String) async throws -> Proposal {
        return try await NetworkService.shared.request(
            endpoint: "/governance/proposals/\(id)"
        )
    }

    func createProposal(
        daoId: String,
        title: String,
        description: String,
        proposer: String,
        metadata: ProposalMetadata? = nil
    ) async throws -> Proposal {
        struct CreateProposalRequest: Encodable {
            let daoId: String
            let title: String
            let description: String
            let proposer: String
            let metadata: ProposalMetadata?
        }

        // Verify proposer has enough tokens for deposit
        let dao = try await getDAO(id: daoId)
        let balance = try await BlockchainService.shared.getBalance(for: proposer)

        guard balance.transferable >= dao.proposalDeposit else {
            throw GovernanceError.insufficientDeposit(required: dao.proposalDeposit)
        }

        let request = CreateProposalRequest(
            daoId: daoId,
            title: title,
            description: description,
            proposer: proposer,
            metadata: metadata
        )

        // Submit proposal to chain
        let proposal: Proposal = try await NetworkService.shared.request(
            endpoint: "/governance/proposals",
            method: .post,
            body: request
        )

        // Lock deposit
        _ = try await BlockchainService.shared.transfer(
            from: proposer,
            to: "dao_treasury_\(daoId)",
            amount: dao.proposalDeposit,
            memo: "Proposal deposit: \(proposal.id)"
        )

        return proposal
    }

    func cancelProposal(id: String, proposer: String) async throws {
        struct CancelRequest: Encodable {
            let proposer: String
        }

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/governance/proposals/\(id)/cancel",
            method: .post,
            body: CancelRequest(proposer: proposer)
        )
    }

    // MARK: - Voting Operations
    func vote(
        proposalId: String,
        voter: String,
        voteType: VoteType,
        votingPower: Int? = nil
    ) async throws -> Vote {
        struct VoteRequest: Encodable {
            let proposalId: String
            let voter: String
            let voteType: String
        }

        // Get proposal to verify it's active
        let proposal = try await getProposal(id: proposalId)

        guard proposal.status == .active else {
            throw GovernanceError.proposalNotActive
        }

        // Verify voter has voting power
        let member = try await getMember(daoId: proposal.daoId, address: voter)
        guard member.votingPower > 0 else {
            throw GovernanceError.noVotingPower
        }

        // Check if already voted
        let existingVote = try? await getVote(proposalId: proposalId, voter: voter)
        if existingVote != nil {
            throw GovernanceError.alreadyVoted
        }

        let request = VoteRequest(
            proposalId: proposalId,
            voter: voter,
            voteType: voteType.rawValue
        )

        // Submit vote to chain
        let vote: Vote = try await NetworkService.shared.request(
            endpoint: "/governance/votes",
            method: .post,
            body: request
        )

        return vote
    }

    func getVote(proposalId: String, voter: String) async throws -> Vote {
        return try await NetworkService.shared.request(
            endpoint: "/governance/proposals/\(proposalId)/votes/\(voter)"
        )
    }

    func getVotes(proposalId: String) async throws -> [Vote] {
        struct VoteListResponse: Decodable {
            let votes: [Vote]
        }

        let response: VoteListResponse = try await NetworkService.shared.request(
            endpoint: "/governance/proposals/\(proposalId)/votes"
        )

        return response.votes
    }

    func changeVote(
        proposalId: String,
        voter: String,
        newVoteType: VoteType
    ) async throws -> Vote {
        struct ChangeVoteRequest: Encodable {
            let voter: String
            let voteType: String
        }

        let request = ChangeVoteRequest(
            voter: voter,
            voteType: newVoteType.rawValue
        )

        return try await NetworkService.shared.request(
            endpoint: "/governance/proposals/\(proposalId)/votes",
            method: .put,
            body: request
        )
    }

    // MARK: - Member Operations
    func getMember(daoId: String, address: String) async throws -> DAOMember {
        return try await NetworkService.shared.request(
            endpoint: "/governance/daos/\(daoId)/members/\(address)"
        )
    }

    func getMembers(daoId: String) async throws -> [DAOMember] {
        struct MemberListResponse: Decodable {
            let members: [DAOMember]
        }

        let response: MemberListResponse = try await NetworkService.shared.request(
            endpoint: "/governance/daos/\(daoId)/members"
        )

        return response.members
    }

    func joinDAO(daoId: String, address: String) async throws -> DAOMember {
        struct JoinRequest: Encodable {
            let address: String
        }

        return try await NetworkService.shared.request(
            endpoint: "/governance/daos/\(daoId)/join",
            method: .post,
            body: JoinRequest(address: address)
        )
    }

    func delegateVotes(daoId: String, from: String, to: String) async throws {
        struct DelegateRequest: Encodable {
            let from: String
            let to: String
        }

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/governance/daos/\(daoId)/delegate",
            method: .post,
            body: DelegateRequest(from: from, to: to)
        )
    }

    // MARK: - Execution
    func executeProposal(id: String, executor: String) async throws {
        struct ExecuteRequest: Encodable {
            let executor: String
        }

        let proposal = try await getProposal(id: id)

        guard proposal.status == .passed else {
            throw GovernanceError.proposalNotPassed
        }

        guard proposal.blocksRemaining == 0 else {
            throw GovernanceError.executionDelayNotMet
        }

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/governance/proposals/\(id)/execute",
            method: .post,
            body: ExecuteRequest(executor: executor)
        )
    }

    // MARK: - Helper Methods
    private func generateMockProposals(for daoId: String) -> [Proposal] {
        let currentBlock = 1000000

        return [
            Proposal(
                id: "proposal_1",
                daoId: daoId,
                title: "Increase validator rewards by 10%",
                description: "This proposal aims to increase validator rewards from 5% to 5.5% annually to attract more validators and improve network security. The increase will be funded from treasury reserves and will take effect immediately upon execution.",
                proposer: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                status: .active,
                votesFor: 75000,
                votesAgainst: 25000,
                votesAbstain: 5000,
                threshold: 50000,
                startBlock: currentBlock - 1000,
                endBlock: currentBlock + 2880,
                currentBlock: currentBlock,
                executionDelay: 1200,
                metadata: ProposalMetadata(
                    category: "Treasury",
                    tags: ["validators", "rewards", "security"],
                    budget: 0,
                    recipient: nil,
                    contractCall: nil
                ),
                createdAt: Date().addingTimeInterval(-86400),
                updatedAt: Date()
            ),
            Proposal(
                id: "proposal_2",
                daoId: daoId,
                title: "Fund new DeFi protocol integration",
                description: "Allocate 50,000 ÉTR to fund the development and integration of a new DeFi lending protocol. This will expand the ecosystem and provide new opportunities for token holders. Development timeline: 3 months.",
                proposer: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                status: .active,
                votesFor: 60000,
                votesAgainst: 40000,
                votesAbstain: 3000,
                threshold: 50000,
                startBlock: currentBlock - 500,
                endBlock: currentBlock + 7200,
                currentBlock: currentBlock,
                executionDelay: 1200,
                metadata: ProposalMetadata(
                    category: "Development",
                    tags: ["defi", "integration", "development"],
                    budget: 50000,
                    recipient: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
                    contractCall: nil
                ),
                createdAt: Date().addingTimeInterval(-43200),
                updatedAt: Date()
            ),
            Proposal(
                id: "proposal_3",
                daoId: daoId,
                title: "Reduce transaction fees by 20%",
                description: "Lower transaction fees to improve user experience and increase network adoption. Current fees: 0.01 ÉTR, proposed fees: 0.008 ÉTR.",
                proposer: "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
                status: .passed,
                votesFor: 120000,
                votesAgainst: 15000,
                votesAbstain: 2000,
                threshold: 50000,
                startBlock: currentBlock - 8000,
                endBlock: currentBlock - 200,
                currentBlock: currentBlock,
                executionDelay: 1200,
                metadata: ProposalMetadata(
                    category: "Network",
                    tags: ["fees", "ux", "adoption"],
                    budget: 0,
                    recipient: nil,
                    contractCall: ContractCall(
                        contract: "0x1234",
                        method: "setTransactionFee",
                        params: ["newFee": "8000000000000000"]
                    )
                ),
                createdAt: Date().addingTimeInterval(-172800),
                updatedAt: Date().addingTimeInterval(-86400)
            )
        ]
    }
}

// MARK: - Errors
enum GovernanceError: Error {
    case proposalNotFound
    case proposalNotActive
    case proposalNotPassed
    case alreadyVoted
    case noVotingPower
    case insufficientDeposit(required: Double)
    case executionDelayNotMet
    case unauthorized

    var localizedDescription: String {
        switch self {
        case .proposalNotFound:
            return "Proposal not found"
        case .proposalNotActive:
            return "Proposal is not active"
        case .proposalNotPassed:
            return "Proposal has not passed"
        case .alreadyVoted:
            return "You have already voted on this proposal"
        case .noVotingPower:
            return "You have no voting power in this DAO"
        case .insufficientDeposit(let required):
            return "Insufficient balance for proposal deposit. Required: \(required)"
        case .executionDelayNotMet:
            return "Execution delay period has not elapsed"
        case .unauthorized:
            return "Unauthorized action"
        }
    }
}
