import Foundation
import Combine

// MARK: - Governance Service

public actor GovernanceService {

    private let rpcService: GovernanceRPCService
    private var cachedProposals: [GovernanceProposal] = []
    private var cachedState: ConsensusDayState?
    private var lastFetch: Date?

    private let cacheTimeout: TimeInterval = 60 // 1 minute cache

    public init(rpcService: GovernanceRPCService) {
        self.rpcService = rpcService
    }

    // MARK: - Consensus Day State

    /// Get current Consensus Day state
    public func getConsensusDayState() async throws -> ConsensusDayState {
        if let cached = cachedState,
           let lastFetch = lastFetch,
           Date().timeIntervalSince(lastFetch) < cacheTimeout {
            return cached
        }

        let result = try await rpcService.call(
            method: "governance_getConsensusDayState",
            params: [:]
        )

        let state = try parseConsensusDayState(from: result)
        cachedState = state
        lastFetch = Date()
        return state
    }

    /// Check if Consensus Day is currently active
    public func isConsensusDayActive() async throws -> Bool {
        let state = try await getConsensusDayState()
        return state.phase != .inactive
    }

    /// Get time until next Consensus Day
    public func timeUntilNextConsensusDay() async throws -> TimeInterval {
        let state = try await getConsensusDayState()
        return state.nextConsensusDayDate.timeIntervalSince(Date())
    }

    // MARK: - Proposals

    /// Fetch all active proposals
    public func getActiveProposals() async throws -> [GovernanceProposal] {
        let result = try await rpcService.call(
            method: "governance_getProposals",
            params: ["status": "active"]
        )

        let proposals = try parseProposals(from: result)
        cachedProposals = proposals
        return proposals
    }

    /// Fetch all proposals (including historical)
    public func getAllProposals(limit: Int = 50, offset: Int = 0) async throws -> [GovernanceProposal] {
        let result = try await rpcService.call(
            method: "governance_getProposals",
            params: ["limit": limit, "offset": offset]
        )

        return try parseProposals(from: result)
    }

    /// Get proposal by ID
    public func getProposal(id: UInt64) async throws -> GovernanceProposal {
        let result = try await rpcService.call(
            method: "governance_getProposal",
            params: ["id": id]
        )

        guard let proposalData = result["proposal"] as? [String: Any] else {
            throw GovernanceError.proposalNotFound
        }

        return try parseProposal(from: proposalData)
    }

    /// Submit a new proposal (requires 10,000 ETR bond)
    public func submitProposal(
        title: String,
        description: String,
        category: ProposalCategory,
        budgetAmount: Decimal? = nil,
        budgetCategory: BudgetCategory? = nil,
        fromAddress: String
    ) async throws -> GovernanceProposal {
        // Validate proposal
        guard title.count >= 10 && title.count <= 100 else {
            throw GovernanceError.invalidTitle
        }

        guard description.count >= 50 && description.count <= 5000 else {
            throw GovernanceError.invalidDescription
        }

        // Check Consensus Day phase
        let state = try await getConsensusDayState()
        guard state.phase == .registration else {
            throw GovernanceError.wrongPhase("Proposals can only be submitted during Registration phase")
        }

        var params: [String: Any] = [
            "title": title,
            "description": description,
            "category": category.rawValue,
            "fromAddress": fromAddress
        ]

        if let amount = budgetAmount {
            params["budgetAmount"] = amount.description
        }

        if let budgetCat = budgetCategory {
            params["budgetCategory"] = budgetCat.rawValue
        }

        let result = try await rpcService.call(
            method: "governance_submitProposal",
            params: params
        )

        guard let proposalData = result["proposal"] as? [String: Any] else {
            throw GovernanceError.submissionFailed
        }

        return try parseProposal(from: proposalData)
    }

    // MARK: - Voting

    /// Get user's voting power
    public func getVotingPower(address: String) async throws -> VotingPowerInfo {
        let result = try await rpcService.call(
            method: "governance_getVotingPower",
            params: ["address": address]
        )

        return VotingPowerInfo(
            baseStake: Decimal(string: result["baseStake"] as? String ?? "0") ?? 0,
            durationMultiplier: result["durationMultiplier"] as? Double ?? 1.0,
            historyMultiplier: result["historyMultiplier"] as? Double ?? 1.0,
            roleMultiplier: result["roleMultiplier"] as? Double ?? 1.0,
            effectiveVotingPower: Decimal(string: result["effectiveVotingPower"] as? String ?? "0") ?? 0
        )
    }

    /// Cast a vote on a proposal
    public func castVote(
        proposalId: UInt64,
        ballot: Ballot,
        fromAddress: String
    ) async throws -> VoteRecord {
        // Check Consensus Day phase
        let state = try await getConsensusDayState()
        guard state.phase == .voting else {
            throw GovernanceError.wrongPhase("Votes can only be cast during Voting phase")
        }

        // Check if already voted
        let existingVote = try await getUserVote(proposalId: proposalId, address: fromAddress)
        if existingVote != nil {
            throw GovernanceError.alreadyVoted
        }

        let result = try await rpcService.call(
            method: "governance_castVote",
            params: [
                "proposalId": proposalId,
                "ballot": ballot.rawValue,
                "fromAddress": fromAddress
            ]
        )

        guard let voteData = result["vote"] as? [String: Any] else {
            throw GovernanceError.voteFailed
        }

        return try parseVoteRecord(from: voteData)
    }

    /// Get user's vote on a proposal
    public func getUserVote(proposalId: UInt64, address: String) async throws -> VoteRecord? {
        let result = try await rpcService.call(
            method: "governance_getUserVote",
            params: [
                "proposalId": proposalId,
                "address": address
            ]
        )

        guard let voteData = result["vote"] as? [String: Any] else {
            return nil
        }

        return try parseVoteRecord(from: voteData)
    }

    /// Get all votes for a proposal
    public func getProposalVotes(proposalId: UInt64) async throws -> [VoteRecord] {
        let result = try await rpcService.call(
            method: "governance_getProposalVotes",
            params: ["proposalId": proposalId]
        )

        guard let votesData = result["votes"] as? [[String: Any]] else {
            return []
        }

        return try votesData.map { try parseVoteRecord(from: $0) }
    }

    /// Get all user's votes across proposals
    public func getUserVotes(address: String) async throws -> [VoteRecord] {
        let result = try await rpcService.call(
            method: "governance_getUserVotes",
            params: ["address": address]
        )

        guard let votesData = result["votes"] as? [[String: Any]] else {
            return []
        }

        return try votesData.map { try parseVoteRecord(from: $0) }
    }

    // MARK: - Director Elections

    /// Get all director candidates
    public func getDirectorCandidates() async throws -> [DirectorCandidate] {
        let result = try await rpcService.call(
            method: "governance_getDirectorCandidates",
            params: [:]
        )

        guard let candidatesData = result["candidates"] as? [[String: Any]] else {
            return []
        }

        return try candidatesData.map { try parseDirectorCandidate(from: $0) }
    }

    /// Nominate self as director candidate (requires 128+ ETR stake)
    public func nominateAsDirector(
        name: String,
        manifesto: String,
        fromAddress: String
    ) async throws -> DirectorCandidate {
        guard manifesto.count >= 100 && manifesto.count <= 10000 else {
            throw GovernanceError.invalidManifesto
        }

        let result = try await rpcService.call(
            method: "governance_nominateDirector",
            params: [
                "name": name,
                "manifesto": manifesto,
                "fromAddress": fromAddress
            ]
        )

        guard let candidateData = result["candidate"] as? [String: Any] else {
            throw GovernanceError.nominationFailed
        }

        return try parseDirectorCandidate(from: candidateData)
    }

    /// Vote for a director candidate
    public func voteForDirector(
        candidateAddress: String,
        fromAddress: String
    ) async throws -> VoteRecord {
        let state = try await getConsensusDayState()
        guard state.phase == .voting else {
            throw GovernanceError.wrongPhase("Director votes can only be cast during Voting phase")
        }

        let result = try await rpcService.call(
            method: "governance_voteDirector",
            params: [
                "candidateAddress": candidateAddress,
                "fromAddress": fromAddress
            ]
        )

        guard let voteData = result["vote"] as? [String: Any] else {
            throw GovernanceError.voteFailed
        }

        return try parseVoteRecord(from: voteData)
    }

    /// Get current elected directors
    public func getElectedDirectors() async throws -> [DirectorCandidate] {
        let result = try await rpcService.call(
            method: "governance_getElectedDirectors",
            params: [:]
        )

        guard let directorsData = result["directors"] as? [[String: Any]] else {
            return []
        }

        return try directorsData.map { try parseDirectorCandidate(from: $0) }
    }

    // MARK: - Staking

    /// Get user's staking info
    public func getStakingInfo(address: String) async throws -> StakeInfo? {
        let result = try await rpcService.call(
            method: "staking_getStakeInfo",
            params: ["address": address]
        )

        guard let stakeData = result["stake"] as? [String: Any] else {
            return nil
        }

        return StakeInfo(
            validatorAddress: stakeData["validatorAddress"] as? String ?? "",
            validatorName: stakeData["validatorName"] as? String ?? "Unknown",
            stakedAmount: Decimal(string: stakeData["stakedAmount"] as? String ?? "0") ?? 0,
            rewards: Decimal(string: stakeData["rewards"] as? String ?? "0") ?? 0,
            stakingStartDate: Date(timeIntervalSince1970: stakeData["stakingStartTime"] as? Double ?? 0),
            unbondingEndDate: nil,
            isUnbonding: stakeData["isUnbonding"] as? Bool ?? false,
            votingPower: Decimal(string: stakeData["votingPower"] as? String ?? "0") ?? 0,
            multiplier: stakeData["multiplier"] as? Double ?? 1.0
        )
    }

    /// Stake ETR tokens
    public func stake(
        amount: Decimal,
        validatorAddress: String,
        fromAddress: String
    ) async throws -> String {
        let result = try await rpcService.call(
            method: "staking_stake",
            params: [
                "amount": amount.description,
                "validatorAddress": validatorAddress,
                "fromAddress": fromAddress
            ]
        )

        guard let txHash = result["txHash"] as? String else {
            throw GovernanceError.stakingFailed
        }

        return txHash
    }

    /// Unstake ETR tokens
    public func unstake(
        amount: Decimal,
        fromAddress: String
    ) async throws -> String {
        let result = try await rpcService.call(
            method: "staking_unstake",
            params: [
                "amount": amount.description,
                "fromAddress": fromAddress
            ]
        )

        guard let txHash = result["txHash"] as? String else {
            throw GovernanceError.unstakingFailed
        }

        return txHash
    }

    // MARK: - Rewards

    /// Get pending participation rewards
    public func getPendingRewards(address: String) async throws -> [ParticipationReward] {
        let result = try await rpcService.call(
            method: "governance_getPendingRewards",
            params: ["address": address]
        )

        guard let rewardsData = result["rewards"] as? [[String: Any]] else {
            return []
        }

        return rewardsData.map { data in
            ParticipationReward(
                id: data["id"] as? String ?? UUID().uuidString,
                consensusDayYear: UInt32(data["year"] as? Int ?? 0),
                amount: Decimal(string: data["amount"] as? String ?? "0") ?? 0,
                type: RewardType(rawValue: data["type"] as? String ?? "") ?? .participation,
                claimedAt: nil,
                transactionHash: nil
            )
        }
    }

    /// Claim participation reward
    public func claimReward(
        rewardId: String,
        fromAddress: String
    ) async throws -> String {
        let result = try await rpcService.call(
            method: "governance_claimReward",
            params: [
                "rewardId": rewardId,
                "fromAddress": fromAddress
            ]
        )

        guard let txHash = result["txHash"] as? String else {
            throw GovernanceError.claimFailed
        }

        return txHash
    }

    // MARK: - Registration

    /// Register as Consensus Day participant
    public func registerAsParticipant(fromAddress: String) async throws -> String {
        let state = try await getConsensusDayState()
        guard state.phase == .registration else {
            throw GovernanceError.wrongPhase("Registration only available during Registration phase")
        }

        let result = try await rpcService.call(
            method: "governance_registerParticipant",
            params: ["fromAddress": fromAddress]
        )

        guard let txHash = result["txHash"] as? String else {
            throw GovernanceError.registrationFailed
        }

        return txHash
    }

    /// Check if user is registered for current Consensus Day
    public func isRegistered(address: String) async throws -> Bool {
        let result = try await rpcService.call(
            method: "governance_isRegistered",
            params: ["address": address]
        )

        return result["isRegistered"] as? Bool ?? false
    }

    // MARK: - Dashboard

    /// Get full governance dashboard for user
    public func getDashboard(address: String) async throws -> GovernanceDashboard {
        async let votingPower = getVotingPower(address: address)
        async let proposals = getActiveProposals()
        async let votes = getUserVotes(address: address)
        async let rewards = getPendingRewards(address: address)
        async let state = getConsensusDayState()
        async let directors = getElectedDirectors()
        async let stakeInfo = getStakingInfo(address: address)

        let power = try await votingPower
        let stake = try await stakeInfo

        return GovernanceDashboard(
            userVotingPower: power.effectiveVotingPower,
            userStake: stake?.stakedAmount ?? 0,
            stakeDurationMultiplier: power.durationMultiplier,
            historyMultiplier: power.historyMultiplier,
            activeProposals: try await proposals,
            userVotes: try await votes,
            pendingRewards: try await rewards,
            consensusDayState: try await state,
            directors: try await directors
        )
    }

    // MARK: - Private Parsing Methods

    private func parseConsensusDayState(from data: [String: Any]) throws -> ConsensusDayState {
        guard let phaseStr = data["phase"] as? String,
              let phase = ConsensusDayPhase(rawValue: phaseStr) else {
            throw GovernanceError.parseError
        }

        return ConsensusDayState(
            currentYear: UInt32(data["currentYear"] as? Int ?? 0),
            phase: phase,
            phaseStartTime: Date(timeIntervalSince1970: data["phaseStartTime"] as? Double ?? 0),
            phaseEndTime: Date(timeIntervalSince1970: data["phaseEndTime"] as? Double ?? 0),
            totalProposals: UInt64(data["totalProposals"] as? Int ?? 0),
            activeProposals: UInt32(data["activeProposals"] as? Int ?? 0),
            totalParticipants: UInt32(data["totalParticipants"] as? Int ?? 0),
            totalVotingPower: Decimal(string: data["totalVotingPower"] as? String ?? "0") ?? 0,
            nextConsensusDayDate: Date(timeIntervalSince1970: data["nextConsensusDayDate"] as? Double ?? 0)
        )
    }

    private func parseProposals(from data: [String: Any]) throws -> [GovernanceProposal] {
        guard let proposalsData = data["proposals"] as? [[String: Any]] else {
            return []
        }

        return try proposalsData.map { try parseProposal(from: $0) }
    }

    private func parseProposal(from data: [String: Any]) throws -> GovernanceProposal {
        guard let categoryStr = data["category"] as? String,
              let category = ProposalCategory(rawValue: categoryStr),
              let statusStr = data["status"] as? String,
              let status = ProposalStatus(rawValue: statusStr) else {
            throw GovernanceError.parseError
        }

        var budgetCategory: BudgetCategory?
        if let budgetCatStr = data["budgetCategory"] as? String {
            budgetCategory = BudgetCategory(rawValue: budgetCatStr)
        }

        return GovernanceProposal(
            id: UInt64(data["id"] as? Int ?? 0),
            proposer: data["proposer"] as? String ?? "",
            title: data["title"] as? String ?? "",
            description: data["description"] as? String ?? "",
            category: category,
            status: status,
            votesFor: UInt32(data["votesFor"] as? Int ?? 0),
            votesAgainst: UInt32(data["votesAgainst"] as? Int ?? 0),
            votingPowerFor: Decimal(string: data["votingPowerFor"] as? String ?? "0") ?? 0,
            votingPowerAgainst: Decimal(string: data["votingPowerAgainst"] as? String ?? "0") ?? 0,
            createdAt: Date(timeIntervalSince1970: data["createdAt"] as? Double ?? 0),
            votingEndsAt: data["votingEndsAt"] != nil ? Date(timeIntervalSince1970: data["votingEndsAt"] as? Double ?? 0) : nil,
            executedAt: data["executedAt"] != nil ? Date(timeIntervalSince1970: data["executedAt"] as? Double ?? 0) : nil,
            budgetAmount: data["budgetAmount"] != nil ? Decimal(string: data["budgetAmount"] as? String ?? "0") : nil,
            budgetCategory: budgetCategory
        )
    }

    private func parseVoteRecord(from data: [String: Any]) throws -> VoteRecord {
        guard let ballotStr = data["ballot"] as? String,
              let ballot = Ballot(rawValue: ballotStr) else {
            throw GovernanceError.parseError
        }

        return VoteRecord(
            id: data["id"] as? String ?? UUID().uuidString,
            proposalId: UInt64(data["proposalId"] as? Int ?? 0),
            voter: data["voter"] as? String ?? "",
            ballot: ballot,
            votingPower: Decimal(string: data["votingPower"] as? String ?? "0") ?? 0,
            votedAt: Date(timeIntervalSince1970: data["votedAt"] as? Double ?? 0),
            transactionHash: data["txHash"] as? String
        )
    }

    private func parseDirectorCandidate(from data: [String: Any]) throws -> DirectorCandidate {
        let statusStr = data["status"] as? String ?? "active"
        let status = CandidateStatus(rawValue: statusStr) ?? .active

        return DirectorCandidate(
            id: data["id"] as? String ?? UUID().uuidString,
            address: data["address"] as? String ?? "",
            name: data["name"] as? String ?? "Unknown",
            manifesto: data["manifesto"] as? String ?? "",
            votesReceived: UInt32(data["votesReceived"] as? Int ?? 0),
            votingPowerReceived: Decimal(string: data["votingPowerReceived"] as? String ?? "0") ?? 0,
            registeredAt: Date(timeIntervalSince1970: data["registeredAt"] as? Double ?? 0),
            status: status,
            avatarURL: data["avatarURL"] as? String,
            website: data["website"] as? String,
            socialLinks: data["socialLinks"] as? [String: String] ?? [:]
        )
    }
}

// MARK: - Supporting Types

public struct VotingPowerInfo {
    public let baseStake: Decimal
    public let durationMultiplier: Double
    public let historyMultiplier: Double
    public let roleMultiplier: Double
    public let effectiveVotingPower: Decimal

    public var description: String {
        return """
        Base Stake: \(baseStake) ETR
        Duration Bonus: +\(Int((durationMultiplier - 1) * 100))%
        History Bonus: +\(Int((historyMultiplier - 1) * 100))%
        Role Multiplier: \(roleMultiplier)x
        Effective Voting Power: \(effectiveVotingPower)
        """
    }
}

// MARK: - Governance Errors

public enum GovernanceError: LocalizedError {
    case proposalNotFound
    case invalidTitle
    case invalidDescription
    case invalidManifesto
    case wrongPhase(String)
    case alreadyVoted
    case voteFailed
    case submissionFailed
    case nominationFailed
    case stakingFailed
    case unstakingFailed
    case claimFailed
    case registrationFailed
    case parseError
    case insufficientStake
    case insufficientBond

    public var errorDescription: String? {
        switch self {
        case .proposalNotFound:
            return "Proposal not found"
        case .invalidTitle:
            return "Title must be between 10 and 100 characters"
        case .invalidDescription:
            return "Description must be between 50 and 5000 characters"
        case .invalidManifesto:
            return "Manifesto must be between 100 and 10000 characters"
        case .wrongPhase(let message):
            return message
        case .alreadyVoted:
            return "You have already voted on this proposal"
        case .voteFailed:
            return "Failed to cast vote"
        case .submissionFailed:
            return "Failed to submit proposal"
        case .nominationFailed:
            return "Failed to nominate as director"
        case .stakingFailed:
            return "Failed to stake tokens"
        case .unstakingFailed:
            return "Failed to unstake tokens"
        case .claimFailed:
            return "Failed to claim reward"
        case .registrationFailed:
            return "Failed to register as participant"
        case .parseError:
            return "Failed to parse response"
        case .insufficientStake:
            return "Insufficient stake for this action"
        case .insufficientBond:
            return "Insufficient balance for proposal bond (10,000 ETR required)"
        }
    }
}
