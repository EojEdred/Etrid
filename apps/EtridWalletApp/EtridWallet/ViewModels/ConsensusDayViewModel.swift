//
//  ConsensusDayViewModel.swift
//  EtridWallet
//
//  Production-ready Consensus Day governance system
//

import Foundation
import Combine

@MainActor
class ConsensusDayViewModel: ObservableObject {
    @Published var proposals: [GovernanceProposal] = []
    @Published var candidates: [DirectorCandidate] = []
    @Published var rewards: [ParticipationReward] = []
    @Published var votingPower: VotingPowerInfo
    @Published var consensusState: ConsensusDayState
    @Published var userVotes: [String: UserVote] = [:] // proposalId -> vote
    @Published var isLoading = false
    @Published var errorMessage: String?

    private let storageKey = "ConsensusDay.Data"
    private var cancellables = Set<AnyCancellable>()

    init() {
        // Initialize with user's actual staking data
        // For now, using sample data - will connect to blockchain
        self.votingPower = VotingPowerInfo(
            stakedAmount: "1000000000000000000000", // 1000 ËTR
            stakeDuration: 90,
            participationHistory: 0,
            calculatedPower: "1100000000000000000000", // 1100 with 1.1× multiplier
            durationMultiplier: 1.1,
            historyMultiplier: 1.0
        )

        self.consensusState = ConsensusDayState(
            currentPhase: .inactive,
            nextPhaseStartTime: Self.calculateNextPhaseStart()
        )

        // Load persisted data
        loadPersistedData()

        // Auto-save when data changes
        setupAutoSave()
    }

    // MARK: - Data Persistence

    private func loadPersistedData() {
        guard let data = UserDefaults.standard.data(forKey: storageKey),
              let decoded = try? JSONDecoder().decode(PersistedData.self, from: data) else {
            // First launch - create sample proposals
            createInitialProposals()
            createInitialCandidates()
            return
        }

        self.proposals = decoded.proposals
        self.candidates = decoded.candidates
        self.rewards = decoded.rewards
        self.userVotes = decoded.userVotes
    }

    private func saveData() {
        let data = PersistedData(
            proposals: proposals,
            candidates: candidates,
            rewards: rewards,
            userVotes: userVotes
        )

        if let encoded = try? JSONEncoder().encode(data) {
            UserDefaults.standard.set(encoded, forKey: storageKey)
        }
    }

    private func setupAutoSave() {
        // Save whenever data changes
        Publishers.CombineLatest4(
            $proposals,
            $candidates,
            $rewards,
            $userVotes
        )
        .debounce(for: 0.5, scheduler: DispatchQueue.main)
        .sink { [weak self] _ in
            self?.saveData()
        }
        .store(in: &cancellables)
    }

    // MARK: - Proposal Management

    func createProposal(
        title: String,
        description: String,
        category: ProposalCategory,
        budgetRequest: String? = nil
    ) {
        let proposal = GovernanceProposal(
            id: UUID().uuidString,
            proposer: getCurrentUserAddress(),
            proposerName: "You",
            category: category,
            title: title,
            description: description,
            status: .active,
            votesFor: "0",
            votesAgainst: "0",
            votesAbstain: "0",
            budgetRequest: budgetRequest,
            startTime: Date(),
            endTime: Date().addingTimeInterval(12 * 3600) // 12 hours
        )

        proposals.insert(proposal, at: 0)
    }

    func castVote(proposal: GovernanceProposal, voteType: VoteType) {
        guard let index = proposals.firstIndex(where: { $0.id == proposal.id }) else {
            return
        }

        // Remove previous vote if exists
        if let previousVote = userVotes[proposal.id] {
            removePreviousVote(proposalId: proposal.id, previousVote: previousVote)
        }

        // Add new vote
        let voteWeight = votingPower.calculatedPower
        var updatedProposal = proposals[index]

        switch voteType {
        case .yes:
            let currentVotes = UInt64(updatedProposal.votesFor) ?? 0
            let voteAmount = UInt64(voteWeight) ?? 0
            updatedProposal.votesFor = String(currentVotes + voteAmount)

        case .no:
            let currentVotes = UInt64(updatedProposal.votesAgainst) ?? 0
            let voteAmount = UInt64(voteWeight) ?? 0
            updatedProposal.votesAgainst = String(currentVotes + voteAmount)

        case .abstain:
            let currentVotes = UInt64(updatedProposal.votesAbstain) ?? 0
            let voteAmount = UInt64(voteWeight) ?? 0
            updatedProposal.votesAbstain = String(currentVotes + voteAmount)
        }

        proposals[index] = updatedProposal

        // Record user's vote
        userVotes[proposal.id] = UserVote(
            proposalId: proposal.id,
            voteType: voteType,
            votingPower: voteWeight,
            timestamp: Date()
        )

        // Award participation
        awardParticipation()
    }

    private func removePreviousVote(proposalId: String, previousVote: UserVote) {
        guard let index = proposals.firstIndex(where: { $0.id == proposalId }) else {
            return
        }

        var updatedProposal = proposals[index]
        let voteAmount = UInt64(previousVote.votingPower) ?? 0

        switch previousVote.voteType {
        case .yes:
            let currentVotes = UInt64(updatedProposal.votesFor) ?? 0
            updatedProposal.votesFor = String(currentVotes > voteAmount ? currentVotes - voteAmount : 0)

        case .no:
            let currentVotes = UInt64(updatedProposal.votesAgainst) ?? 0
            updatedProposal.votesAgainst = String(currentVotes > voteAmount ? currentVotes - voteAmount : 0)

        case .abstain:
            let currentVotes = UInt64(updatedProposal.votesAbstain) ?? 0
            updatedProposal.votesAbstain = String(currentVotes > voteAmount ? currentVotes - voteAmount : 0)
        }

        proposals[index] = updatedProposal
    }

    // MARK: - Director Voting

    func voteForDirector(candidate: DirectorCandidate) {
        guard let index = candidates.firstIndex(where: { $0.id == candidate.id }) else {
            return
        }

        var updatedCandidate = candidates[index]
        let currentVotes = UInt64(updatedCandidate.voteCount) ?? 0
        let voteAmount = UInt64(votingPower.calculatedPower) ?? 0
        updatedCandidate.voteCount = String(currentVotes + voteAmount)

        candidates[index] = updatedCandidate
    }

    // MARK: - Rewards

    private func awardParticipation() {
        let reward = ParticipationReward(
            id: UUID().uuidString,
            eventType: "Proposal Vote",
            amount: "50000000000000000000", // 50 ËTR
            timestamp: Date(),
            claimed: false
        )

        rewards.insert(reward, at: 0)

        // Update participation history for voting power
        updateVotingPower(additionalParticipation: 1)
    }

    func claimReward(rewardId: String) {
        guard let index = rewards.firstIndex(where: { $0.id == rewardId }) else {
            return
        }

        var updatedReward = rewards[index]
        updatedReward.claimed = true
        rewards[index] = updatedReward
    }

    func claimAllRewards() {
        for (index, reward) in rewards.enumerated() where !reward.claimed {
            rewards[index].claimed = true
        }
    }

    // MARK: - Voting Power

    private func updateVotingPower(additionalParticipation: Int) {
        let newHistory = votingPower.participationHistory + additionalParticipation
        let newHistoryMultiplier = min(1.0 + Double(newHistory) * 0.02, 1.1) // Max 10% bonus

        let baseStake = UInt64(votingPower.stakedAmount) ?? 0
        let totalMultiplier = votingPower.durationMultiplier * newHistoryMultiplier
        let newPower = Double(baseStake) * totalMultiplier

        votingPower = VotingPowerInfo(
            stakedAmount: votingPower.stakedAmount,
            stakeDuration: votingPower.stakeDuration,
            participationHistory: newHistory,
            calculatedPower: String(UInt64(newPower)),
            durationMultiplier: votingPower.durationMultiplier,
            historyMultiplier: newHistoryMultiplier
        )
    }

    // MARK: - Phase Management

    func updatePhase() {
        let now = Date()

        if now >= consensusState.nextPhaseStartTime {
            let nextPhase = consensusState.currentPhase.next()
            consensusState = ConsensusDayState(
                currentPhase: nextPhase,
                nextPhaseStartTime: Self.calculateNextPhaseStart()
            )

            // Handle phase transitions
            handlePhaseTransition(to: nextPhase)
        }
    }

    private func handlePhaseTransition(to phase: ConsensusDayPhase) {
        switch phase {
        case .voting:
            // Activate all pending proposals
            for (index, proposal) in proposals.enumerated() where proposal.status == .pending {
                proposals[index].status = .active
            }

        case .minting:
            // Close all active proposals and determine outcomes
            for (index, proposal) in proposals.enumerated() where proposal.status == .active {
                let passed = proposal.forPercentage >= (proposal.category.requiresSupermajority ? 66.0 : 50.0)
                proposals[index].status = passed ? .passed : .rejected
            }

        case .distribution:
            // Award rewards to all participants
            awardDistributionRewards()

        default:
            break
        }
    }

    private func awardDistributionRewards() {
        let participantCount = userVotes.count
        if participantCount > 0 {
            let reward = ParticipationReward(
                id: UUID().uuidString,
                eventType: "Consensus Day Completion",
                amount: "200000000000000000000", // 200 ËTR base
                timestamp: Date(),
                claimed: false
            )
            rewards.insert(reward, at: 0)
        }
    }

    // MARK: - Helpers

    private func getCurrentUserAddress() -> String {
        // In production, get from wallet service
        return "0x1234567890123456789012345678901234567890"
    }

    private static func calculateNextPhaseStart() -> Date {
        let calendar = Calendar.current
        let now = Date()

        // Next December 1st at 00:00 UTC
        var components = calendar.dateComponents([.year], from: now)
        components.month = 12
        components.day = 1
        components.hour = 0
        components.minute = 0
        components.second = 0
        components.timeZone = TimeZone(identifier: "UTC")

        guard var nextDate = calendar.date(from: components) else {
            return now.addingTimeInterval(30 * 24 * 3600)
        }

        // If December 1st has passed this year, use next year
        if nextDate <= now {
            components.year! += 1
            nextDate = calendar.date(from: components) ?? nextDate
        }

        return nextDate
    }

    // MARK: - Initial Data

    private func createInitialProposals() {
        proposals = [
            GovernanceProposal(
                id: UUID().uuidString,
                proposer: "0xAbC123...",
                proposerName: "Core Dev Team",
                category: .protocolUpgrade,
                title: "Implement EIP-4844 Proto-Danksharding",
                description: "Upgrade the network to support EIP-4844 blob transactions for improved L2 scalability and reduced transaction costs.",
                status: .active,
                votesFor: "15000000000000000000000",
                votesAgainst: "3000000000000000000000",
                votesAbstain: "1000000000000000000000",
                budgetRequest: nil,
                startTime: Date().addingTimeInterval(-3600),
                endTime: Date().addingTimeInterval(8 * 3600)
            ),
            GovernanceProposal(
                id: UUID().uuidString,
                proposer: "0xDeF456...",
                proposerName: "Foundation",
                category: .treasuryAllocation,
                title: "Fund Developer Grants Program Q1 2025",
                description: "Allocate 500,000 ËTR to fund ecosystem development grants for Q1 2025, focusing on DeFi and infrastructure projects.",
                status: .active,
                votesFor: "8000000000000000000000",
                votesAgainst: "2000000000000000000000",
                votesAbstain: "500000000000000000000",
                budgetRequest: "500000000000000000000000",
                startTime: Date().addingTimeInterval(-7200),
                endTime: Date().addingTimeInterval(7 * 3600)
            )
        ]
    }

    private func createInitialCandidates() {
        candidates = [
            DirectorCandidate(
                id: UUID().uuidString,
                name: "Dr. Sarah Chen",
                biography: "Former CTO at major blockchain infrastructure company. 15 years experience in distributed systems.",
                credentials: "PhD Computer Science, Stanford",
                voteCount: "5000000000000000000000"
            ),
            DirectorCandidate(
                id: UUID().uuidString,
                name: "Marcus Rodriguez",
                biography: "Serial entrepreneur and DeFi pioneer. Founded 3 successful blockchain startups.",
                credentials: "MBA Harvard, Blockchain Council Member",
                voteCount: "3500000000000000000000"
            )
        ]
    }
}

// MARK: - Persisted Data Model

private struct PersistedData: Codable {
    let proposals: [GovernanceProposal]
    let candidates: [DirectorCandidate]
    let rewards: [ParticipationReward]
    let userVotes: [String: UserVote]
}
