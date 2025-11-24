//
//  ConsensusDay.swift
//  EtridWallet
//
//  Consensus Day models for voting, staking, and governance
//

import Foundation

// MARK: - Consensus Day Phase
enum ConsensusDayPhase: String, Codable, CaseIterable {
    case inactive = "Inactive"
    case registration = "Registration"
    case voting = "Voting"
    case minting = "Minting"
    case distribution = "Distribution"

    var duration: TimeInterval {
        switch self {
        case .inactive: return 0
        case .registration: return 6 * 3600  // 6 hours
        case .voting: return 12 * 3600       // 12 hours
        case .minting: return 3 * 3600       // 3 hours
        case .distribution: return 1 * 3600  // 1 hour
        }
    }

    var icon: String {
        switch self {
        case .inactive: return "moon.zzz"
        case .registration: return "doc.text.badge.plus"
        case .voting: return "checkmark.ballot"
        case .minting: return "dollarsign.circle"
        case .distribution: return "arrow.branch"
        }
    }

    var color: String {
        switch self {
        case .inactive: return "gray"
        case .registration: return "blue"
        case .voting: return "purple"
        case .minting: return "orange"
        case .distribution: return "green"
        }
    }
}

// MARK: - Proposal Category
enum ProposalCategory: String, Codable, CaseIterable, Identifiable {
    case inflationRate = "Inflation Rate"
    case parameterChange = "Parameter Change"
    case budgetAllocation = "Budget Allocation"
    case protocolUpgrade = "Protocol Upgrade"
    case directorElection = "Director Election"
    case emergencyAction = "Emergency Action"

    var id: String { rawValue }

    var icon: String {
        switch self {
        case .inflationRate: return "chart.line.uptrend.xyaxis"
        case .parameterChange: return "slider.horizontal.3"
        case .budgetAllocation: return "dollarsign.square"
        case .protocolUpgrade: return "arrow.up.square"
        case .directorElection: return "person.3"
        case .emergencyAction: return "exclamationmark.triangle"
        }
    }

    var color: String {
        switch self {
        case .inflationRate: return "orange"
        case .parameterChange: return "blue"
        case .budgetAllocation: return "green"
        case .protocolUpgrade: return "purple"
        case .directorElection: return "indigo"
        case .emergencyAction: return "red"
        }
    }

    var requiresSupermajority: Bool {
        self == .protocolUpgrade || self == .emergencyAction
    }
}

// MARK: - Proposal Status
enum ProposalStatus: String, Codable {
    case pending = "Pending"
    case active = "Active"
    case passing = "Passing"
    case failing = "Failing"
    case approved = "Approved"
    case rejected = "Rejected"
    case expired = "Expired"

    var color: String {
        switch self {
        case .pending: return "gray"
        case .active: return "blue"
        case .passing: return "green"
        case .failing: return "orange"
        case .approved: return "green"
        case .rejected: return "red"
        case .expired: return "gray"
        }
    }
}

// MARK: - Vote Type
enum VoteType: String, Codable {
    case yes = "Yes"
    case no = "No"
    case abstain = "Abstain"

    var color: String {
        switch self {
        case .yes: return "green"
        case .no: return "red"
        case .abstain: return "gray"
        }
    }

    var icon: String {
        switch self {
        case .yes: return "hand.thumbsup.fill"
        case .no: return "hand.thumbsdown.fill"
        case .abstain: return "minus.circle.fill"
        }
    }
}

// MARK: - Governance Proposal
struct GovernanceProposal: Identifiable, Codable {
    let id: String
    let proposer: String
    let proposerName: String?
    let category: ProposalCategory
    let title: String
    let description: String
    var status: ProposalStatus
    var votesFor: String  // Wei amount
    var votesAgainst: String  // Wei amount
    var votesAbstain: String  // Wei amount
    let budgetRequest: String?  // Wei amount
    let createdAt: Date
    let votingEndsAt: Date?

    var totalVotes: Double {
        let forVotes = Double(votesFor) ?? 0
        let againstVotes = Double(votesAgainst) ?? 0
        let abstainVotes = Double(votesAbstain) ?? 0
        return forVotes + againstVotes + abstainVotes
    }

    var forPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return ((Double(votesFor) ?? 0) / totalVotes) * 100
    }

    var againstPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return ((Double(votesAgainst) ?? 0) / totalVotes) * 100
    }

    var abstainPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return ((Double(votesAbstain) ?? 0) / totalVotes) * 100
    }

    var formattedBudget: String {
        guard let budget = budgetRequest else { return "N/A" }
        let value = WeiConverter.weiToEther(budget)
        return String(format: "%.2f ËTR", value)
    }

    var isPassing: Bool {
        let threshold = category.requiresSupermajority ? 66.0 : 50.0
        return forPercentage > threshold
    }

    var timeRemaining: String {
        guard let endsAt = votingEndsAt else { return "No deadline" }
        let interval = endsAt.timeIntervalSinceNow

        if interval < 0 {
            return "Ended"
        }

        let hours = Int(interval / 3600)
        let minutes = Int((interval.truncatingRemainder(dividingBy: 3600)) / 60)

        if hours > 24 {
            let days = hours / 24
            return "\(days)d remaining"
        } else if hours > 0 {
            return "\(hours)h \(minutes)m remaining"
        } else {
            return "\(minutes)m remaining"
        }
    }
}

// MARK: - Director Candidate
struct DirectorCandidate: Identifiable, Codable {
    let id: String
    let address: String
    let name: String?
    let bio: String?
    let isIncumbent: Bool
    var voteCount: String  // Wei amount
    let nominatedAt: Date

    var displayName: String {
        name ?? address.formatAddress()
    }

    var formattedVotes: String {
        let value = WeiConverter.weiToEther(voteCount)
        return String(format: "%.2f ËTR", value)
    }
}

// MARK: - User Vote Record
struct UserVote: Identifiable, Codable {
    let id: String
    let proposalId: String
    let proposalTitle: String
    let voteType: VoteType
    let votingPower: String  // Wei amount
    let timestamp: Date

    var formattedPower: String {
        let value = WeiConverter.weiToEther(votingPower)
        return String(format: "%.2f votes", value)
    }
}

// MARK: - Consensus Day State
struct ConsensusDayState: Codable {
    let currentPhase: ConsensusDayPhase
    let phaseStartedAt: Date
    let nextPhase: ConsensusDayPhase?
    let nextConsensusDayDate: Date  // Next December 1st

    var phaseEndsAt: Date {
        phaseStartedAt.addingTimeInterval(currentPhase.duration)
    }

    var timeUntilNextPhase: TimeInterval {
        phaseEndsAt.timeIntervalSinceNow
    }

    var timeUntilConsensusDay: TimeInterval {
        nextConsensusDayDate.timeIntervalSinceNow
    }

    var daysUntilConsensusDay: Int {
        Int(timeUntilConsensusDay / 86400)
    }

    var isActive: Bool {
        currentPhase != .inactive
    }
}

// MARK: - Voting Power Info
struct VotingPowerInfo: Codable {
    let stakedAmount: String  // Wei amount
    let stakeDuration: Int  // Days
    let participationHistory: Int  // Number of past consensus days participated
    let calculatedPower: String  // Wei amount
    let durationMultiplier: Double  // 1.0 - 1.2
    let historyMultiplier: Double  // 1.0 - 1.1

    var formattedStake: String {
        let value = WeiConverter.weiToEther(stakedAmount)
        return String(format: "%.2f ËTR", value)
    }

    var formattedPower: String {
        let value = WeiConverter.weiToEther(calculatedPower)
        return String(format: "%.2f votes", value)
    }

    var powerMultiplier: Double {
        durationMultiplier * historyMultiplier
    }
}

// MARK: - Participation Reward
struct ParticipationReward: Identifiable, Codable {
    let id: String
    let consensusDayDate: Date
    let votedProposals: Int
    let totalProposals: Int
    let baseReward: String  // Wei amount
    let completenessBonus: String  // Wei amount (20% if voted on all)
    let totalReward: String  // Wei amount
    let claimed: Bool

    var completionPercentage: Double {
        guard totalProposals > 0 else { return 0 }
        return (Double(votedProposals) / Double(totalProposals)) * 100
    }

    var hasCompletenessBonus: Bool {
        votedProposals == totalProposals
    }

    var formattedReward: String {
        let value = WeiConverter.weiToEther(totalReward)
        return String(format: "%.4f ËTR", value)
    }
}

// MARK: - Mock Data
extension GovernanceProposal {
    static let mockProposals: [GovernanceProposal] = [
        GovernanceProposal(
            id: "1",
            proposer: "0x1234567890123456789012345678901234567890",
            proposerName: "Alice Foundation",
            category: .inflationRate,
            title: "Set Annual Inflation to 2.5%",
            description: "Proposal to set the annual inflation rate to 2.5% to support network growth and validator rewards.",
            status: .active,
            votesFor: String(UInt64(5 * 1e18)),
            votesAgainst: String(UInt64(2 * 1e18)),
            votesAbstain: String(UInt64(1 * 1e18)),
            budgetRequest: nil,
            createdAt: Date().addingTimeInterval(-7200),
            votingEndsAt: Date().addingTimeInterval(36000)
        ),
        GovernanceProposal(
            id: "2",
            proposer: "0x2345678901234567890123456789012345678901",
            proposerName: "Dev Team",
            category: .budgetAllocation,
            title: "Fund Community Development Program",
            description: "Allocate 1M ËTR to support community-driven development initiatives and hackathons.",
            status: .active,
            votesFor: String(UInt64(8 * 1e18)),
            votesAgainst: String(UInt64(3 * 1e18)),
            votesAbstain: String(UInt64(1 * 1e18)),
            budgetRequest: String(UInt64(1 * 1e18)),
            createdAt: Date().addingTimeInterval(-3600),
            votingEndsAt: Date().addingTimeInterval(40000)
        ),
        GovernanceProposal(
            id: "3",
            proposer: "0x3456789012345678901234567890123456789012",
            proposerName: nil,
            category: .protocolUpgrade,
            title: "Upgrade Runtime to v2.0",
            description: "Major runtime upgrade including performance improvements and new features.",
            status: .active,
            votesFor: String(UInt64(7 * 1e18)),
            votesAgainst: String(UInt64(2 * 1e18)),
            votesAbstain: String(UInt64(1 * 1e18)),
            budgetRequest: nil,
            createdAt: Date().addingTimeInterval(-1800),
            votingEndsAt: Date().addingTimeInterval(42000)
        )
    ]
}

extension DirectorCandidate {
    static let mockCandidates: [DirectorCandidate] = [
        DirectorCandidate(
            id: "1",
            address: "0x1234567890123456789012345678901234567890",
            name: "Alice Chen",
            bio: "Blockchain architect with 10 years experience. Focused on scalability and security.",
            isIncumbent: true,
            voteCount: String(UInt64(10 * 1e18)),
            nominatedAt: Date().addingTimeInterval(-7200)
        ),
        DirectorCandidate(
            id: "2",
            address: "0x2345678901234567890123456789012345678901",
            name: "Bob Martinez",
            bio: "DeFi protocol developer and community advocate.",
            isIncumbent: false,
            voteCount: String(UInt64(8 * 1e18)),
            nominatedAt: Date().addingTimeInterval(-5400)
        ),
        DirectorCandidate(
            id: "3",
            address: "0x3456789012345678901234567890123456789012",
            name: "Carol Kim",
            bio: "Economics researcher specializing in tokenomics and governance.",
            isIncumbent: true,
            voteCount: String(UInt64(9 * 1e18)),
            nominatedAt: Date().addingTimeInterval(-3600)
        )
    ]
}

extension ConsensusDayState {
    static var mock: ConsensusDayState {
        // Next December 1st
        let calendar = Calendar.current
        let now = Date()
        let currentYear = calendar.component(.year, from: now)
        let currentMonth = calendar.component(.month, from: now)

        var components = DateComponents()
        components.year = currentMonth >= 12 ? currentYear + 1 : currentYear
        components.month = 12
        components.day = 1
        components.hour = 0
        components.minute = 0

        let nextConsensusDay = calendar.date(from: components) ?? now

        return ConsensusDayState(
            currentPhase: .inactive,
            phaseStartedAt: Date(),
            nextPhase: .registration,
            nextConsensusDayDate: nextConsensusDay
        )
    }
}
