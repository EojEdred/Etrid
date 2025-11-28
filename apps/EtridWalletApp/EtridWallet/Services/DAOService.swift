//
//  DAOService.swift
//  EtridWallet
//
//  Service for DAO operations and governance
//

import Foundation
import Combine

@MainActor
class DAOService: ObservableObject {
    static let shared = DAOService()

    @Published var daos: [DAO] = []
    @Published var proposals: [String: [DAOProposal]] = [:] // daoId -> proposals
    @Published var isLoading = false
    @Published var error: Error?

    private let storageKey = "daos"
    private let proposalsKey = "dao_proposals"

    private init() {
        loadDAOs()
        loadProposals()
    }

    // MARK: - DAO Management

    func createDAO(_ params: CreateDAOParams) async throws -> DAO {
        isLoading = true
        defer { isLoading = false }

        // Validate parameters
        guard !params.name.isEmpty, !params.description.isEmpty else {
            throw WalletError.invalidInput("Name and description are required")
        }

        guard params.quorumPercentage > 0 && params.quorumPercentage <= 100 else {
            throw WalletError.invalidInput("Quorum must be between 0 and 100")
        }

        // Generate treasury address (would deploy actual contract)
        let treasuryAddress = generateTreasuryAddress()

        let dao = DAO(
            id: UUID().uuidString,
            name: params.name,
            description: params.description,
            logoUrl: params.logoUrl,
            governance: params.governance,
            treasuryAddress: treasuryAddress,
            memberCount: 1 + (params.initialMembers?.count ?? 0),
            activeProposalCount: 0,
            treasuryValue: params.initialTreasuryAmount ?? "0",
            createdAt: Date(),
            userRole: .owner
        )

        daos.append(dao)
        saveDAOs()

        // TODO: Deploy DAO smart contract
        // let contractAddress = try await deployDAOContract(dao)

        return dao
    }

    func getDAOs(filter: DAO.MemberRole? = nil) -> [DAO] {
        if let filter = filter {
            return daos.filter { $0.userRole == filter }
        }
        return daos
    }

    func getDAO(id: String) -> DAO? {
        daos.first { $0.id == id }
    }

    func joinDAO(id: String) async throws {
        guard var dao = getDAO(id: id) else {
            throw WalletError.unknown("DAO not found")
        }

        // Check membership requirements
        let canJoin = try await checkMembershipRequirements(dao)
        guard canJoin else {
            throw WalletError.unknown("Does not meet membership requirements")
        }

        // Update DAO
        dao.memberCount += 1
        dao.userRole = .member
        updateDAO(dao)

        // TODO: Register membership on-chain
    }

    func leaveDAO(id: String) async throws {
        guard var dao = getDAO(id: id) else {
            throw WalletError.unknown("DAO not found")
        }

        guard dao.userRole != .owner else {
            throw WalletError.unknown("Owner cannot leave DAO")
        }

        dao.memberCount = max(0, dao.memberCount - 1)
        updateDAO(dao)

        // TODO: Remove membership on-chain
    }

    // MARK: - Proposal Management

    func createProposal(_ params: CreateProposalParams) async throws -> DAOProposal {
        guard let dao = getDAO(id: params.daoId) else {
            throw WalletError.unknown("DAO not found")
        }

        isLoading = true
        defer { isLoading = false }

        let proposal = DAOProposal(
            id: UUID().uuidString,
            daoId: params.daoId,
            title: params.title,
            description: params.description,
            proposer: try await getCurrentAddress(),
            proposerName: "You",
            actions: params.actions,
            createdAt: Date(),
            startTime: Date(),
            endTime: Date().addingTimeInterval(TimeInterval(params.votingPeriodDays * 86400)),
            votesFor: "0",
            votesAgainst: "0",
            votesAbstain: "0",
            status: .active,
            userVote: nil
        )

        var daoProposals = proposals[params.daoId] ?? []
        daoProposals.append(proposal)
        proposals[params.daoId] = daoProposals

        saveProposals()

        // Update DAO
        var updatedDAO = dao
        updatedDAO.activeProposalCount += 1
        updateDAO(updatedDAO)

        // TODO: Submit proposal to smart contract

        return proposal
    }

    func getProposals(daoId: String, filter: DAOProposal.ProposalStatus? = nil) -> [DAOProposal] {
        let daoProposals = proposals[daoId] ?? []
        if let filter = filter {
            return daoProposals.filter { $0.status == filter }
        }
        return daoProposals
    }

    func getProposal(id: String, daoId: String) -> DAOProposal? {
        proposals[daoId]?.first { $0.id == id }
    }

    func vote(proposalId: String, daoId: String, voteType: DAOVoteType, votingPower: String) async throws {
        guard var proposal = getProposal(id: proposalId, daoId: daoId) else {
            throw WalletError.unknown("Proposal not found")
        }

        guard proposal.status == .active else {
            throw WalletError.invalidInput("Proposal is not active")
        }

        guard proposal.userVote == nil else {
            throw WalletError.invalidInput("Already voted on this proposal")
        }

        isLoading = true
        defer { isLoading = false }

        // Update votes
        switch voteType {
        case .for:
            let current = Double(proposal.votesFor) ?? 0
            let power = Double(votingPower) ?? 0
            proposal.votesFor = String(current + power)
        case .against:
            let current = Double(proposal.votesAgainst) ?? 0
            let power = Double(votingPower) ?? 0
            proposal.votesAgainst = String(current + power)
        case .abstain:
            let current = Double(proposal.votesAbstain) ?? 0
            let power = Double(votingPower) ?? 0
            proposal.votesAbstain = String(current + power)
        }

        proposal.userVote = voteType
        updateProposal(proposal, daoId: daoId)

        // TODO: Submit vote to smart contract
    }

    func executeProposal(proposalId: String, daoId: String) async throws {
        guard var proposal = getProposal(id: proposalId, daoId: daoId) else {
            throw WalletError.unknown("Proposal not found")
        }

        guard proposal.status == .succeeded else {
            throw WalletError.invalidInput("Proposal has not succeeded")
        }

        isLoading = true
        defer { isLoading = false }

        proposal.status = .executed
        updateProposal(proposal, daoId: daoId)

        // TODO: Execute proposal actions on-chain
    }

    // MARK: - Treasury

    func getTreasury(daoId: String) async throws -> DAOTreasury {
        // TODO: Fetch treasury data from blockchain
        return DAOTreasury(
            daoId: daoId,
            totalValue: "0",
            assets: [],
            transactions: [],
            analytics: nil
        )
    }

    // MARK: - Members

    func getMembers(daoId: String) async throws -> [DAOMember] {
        // TODO: Fetch members from smart contract
        return getMockMembers()
    }

    func addMember(daoId: String, address: String) async throws {
        guard var dao = getDAO(id: daoId) else {
            throw WalletError.unknown("DAO not found")
        }

        guard dao.userRole == .owner || dao.userRole == .admin else {
            throw WalletError.unknown("Insufficient permissions")
        }

        dao.memberCount += 1
        updateDAO(dao)

        // TODO: Add member on-chain
    }

    func removeMember(daoId: String, address: String) async throws {
        guard var dao = getDAO(id: daoId) else {
            throw WalletError.unknown("DAO not found")
        }

        guard dao.userRole == .owner || dao.userRole == .admin else {
            throw WalletError.unknown("Insufficient permissions")
        }

        dao.memberCount = max(0, dao.memberCount - 1)
        updateDAO(dao)

        // TODO: Remove member on-chain
    }

    // MARK: - Stats

    func getStats(daoId: String) async throws -> DAOStats {
        let daoProposals = proposals[daoId] ?? []
        let total = daoProposals.count
        let passed = daoProposals.filter { $0.status == .succeeded || $0.status == .executed }.count
        let rejected = daoProposals.filter { $0.status == .defeated }.count

        return DAOStats(
            totalProposals: total,
            passedProposals: passed,
            rejectedProposals: rejected,
            averageParticipation: 0,
            treasuryGrowth: 0,
            activeMembers: 0
        )
    }

    // MARK: - Helpers

    private func updateDAO(_ dao: DAO) {
        if let index = daos.firstIndex(where: { $0.id == dao.id }) {
            daos[index] = dao
            saveDAOs()
        }
    }

    private func updateProposal(_ proposal: DAOProposal, daoId: String) {
        if var daoProposals = proposals[daoId],
           let index = daoProposals.firstIndex(where: { $0.id == proposal.id }) {
            daoProposals[index] = proposal
            proposals[daoId] = daoProposals
            saveProposals()
        }
    }

    private func checkMembershipRequirements(_ dao: DAO) async throws -> Bool {
        switch dao.governance.membershipType {
        case .open:
            return true
        case .tokenGated:
            guard let tokenAddress = dao.governance.tokenAddress else { return false }
            // TODO: Check token balance
            return true
        case .nftGated:
            guard let nftAddress = dao.governance.nftAddress else { return false }
            // TODO: Check NFT ownership
            return true
        case .inviteOnly:
            return false
        }
    }

    private func generateTreasuryAddress() -> String {
        "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "").prefix(40)
    }

    private func getCurrentAddress() async throws -> String {
        // TODO: Get from WalletManager
        return "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    }

    private func getMockMembers() -> [DAOMember] {
        [
            DAOMember(
                id: "1",
                userId: "user1",
                address: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
                username: "Alice",
                role: .owner,
                votingPower: 10000,
                joinedAt: Date(),
                proposalsCreated: 5,
                votesCast: 12
            ),
            DAOMember(
                id: "2",
                userId: "user2",
                address: "0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199",
                username: "Bob",
                role: .member,
                votingPower: 5000,
                joinedAt: Date(),
                proposalsCreated: 2,
                votesCast: 8
            )
        ]
    }

    // MARK: - Storage

    private func saveDAOs() {
        if let encoded = try? JSONEncoder().encode(daos) {
            UserDefaults.standard.set(encoded, forKey: storageKey)
        }
    }

    private func loadDAOs() {
        if let data = UserDefaults.standard.data(forKey: storageKey),
           let decoded = try? JSONDecoder().decode([DAO].self, from: data) {
            daos = decoded
        }
    }

    private func saveProposals() {
        if let encoded = try? JSONEncoder().encode(proposals) {
            UserDefaults.standard.set(encoded, forKey: proposalsKey)
        }
    }

    private func loadProposals() {
        if let data = UserDefaults.standard.data(forKey: proposalsKey),
           let decoded = try? JSONDecoder().decode([String: [DAOProposal]].self, from: data) {
            proposals = decoded
        }
    }
}
