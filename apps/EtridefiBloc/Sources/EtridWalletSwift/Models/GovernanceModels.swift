import Foundation

// MARK: - Governance Proposal Models

public struct GovernanceProposal: Identifiable, Codable {
    public let id: UInt64
    public let proposer: String
    public let title: String
    public let description: String
    public let category: ProposalCategory
    public let status: ProposalStatus
    public let votesFor: UInt32
    public let votesAgainst: UInt32
    public let votingPowerFor: Decimal
    public let votingPowerAgainst: Decimal
    public let createdAt: Date
    public let votingEndsAt: Date?
    public let executedAt: Date?
    public let budgetAmount: Decimal?
    public let budgetCategory: BudgetCategory?

    public init(
        id: UInt64,
        proposer: String,
        title: String,
        description: String,
        category: ProposalCategory,
        status: ProposalStatus,
        votesFor: UInt32 = 0,
        votesAgainst: UInt32 = 0,
        votingPowerFor: Decimal = 0,
        votingPowerAgainst: Decimal = 0,
        createdAt: Date = Date(),
        votingEndsAt: Date? = nil,
        executedAt: Date? = nil,
        budgetAmount: Decimal? = nil,
        budgetCategory: BudgetCategory? = nil
    ) {
        self.id = id
        self.proposer = proposer
        self.title = title
        self.description = description
        self.category = category
        self.status = status
        self.votesFor = votesFor
        self.votesAgainst = votesAgainst
        self.votingPowerFor = votingPowerFor
        self.votingPowerAgainst = votingPowerAgainst
        self.createdAt = createdAt
        self.votingEndsAt = votingEndsAt
        self.executedAt = executedAt
        self.budgetAmount = budgetAmount
        self.budgetCategory = budgetCategory
    }

    public var totalVotes: UInt32 {
        votesFor + votesAgainst
    }

    public var approvalPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return Double(votesFor) / Double(totalVotes) * 100
    }

    public var totalVotingPower: Decimal {
        votingPowerFor + votingPowerAgainst
    }

    public var votingPowerApprovalPercentage: Double {
        guard totalVotingPower > 0 else { return 0 }
        return ((votingPowerFor / totalVotingPower) as NSDecimalNumber).doubleValue * 100
    }

    public var isVotingActive: Bool {
        guard let endsAt = votingEndsAt else { return status == .active }
        return status == .active && Date() < endsAt
    }

    public var timeRemaining: String? {
        guard let endsAt = votingEndsAt, status == .active else { return nil }
        let remaining = endsAt.timeIntervalSince(Date())
        if remaining <= 0 { return "Voting ended" }

        let hours = Int(remaining) / 3600
        let minutes = (Int(remaining) % 3600) / 60

        if hours > 24 {
            let days = hours / 24
            return "\(days)d \(hours % 24)h remaining"
        } else if hours > 0 {
            return "\(hours)h \(minutes)m remaining"
        } else {
            return "\(minutes)m remaining"
        }
    }
}

public enum ProposalCategory: String, Codable, CaseIterable {
    case inflationRate = "Inflation Rate"
    case parameterChange = "Parameter Change"
    case budgetAllocation = "Budget Allocation"
    case protocolUpgrade = "Protocol Upgrade"
    case directorElection = "Director Election"
    case emergencyAction = "Emergency Action"

    public var icon: String {
        switch self {
        case .inflationRate: return "percent"
        case .parameterChange: return "slider.horizontal.3"
        case .budgetAllocation: return "dollarsign.circle"
        case .protocolUpgrade: return "arrow.up.circle"
        case .directorElection: return "person.3.fill"
        case .emergencyAction: return "exclamationmark.triangle.fill"
        }
    }

    public var requiredApproval: Double {
        switch self {
        case .protocolUpgrade, .emergencyAction:
            return 66.0 // Supermajority
        default:
            return 50.0 // Simple majority
        }
    }
}

public enum ProposalStatus: String, Codable {
    case pending = "Pending"
    case active = "Active"
    case approved = "Approved"
    case rejected = "Rejected"
    case executed = "Executed"
    case cancelled = "Cancelled"

    public var color: String {
        switch self {
        case .pending: return "gray"
        case .active: return "blue"
        case .approved: return "green"
        case .rejected: return "red"
        case .executed: return "purple"
        case .cancelled: return "orange"
        }
    }
}

public enum BudgetCategory: String, Codable, CaseIterable {
    case infrastructure = "Infrastructure"
    case marketing = "Marketing"
    case security = "Security"
    case communityGrants = "Community Grants"
    case operations = "Operations"
    case research = "Research"
    case legal = "Legal"
    case emergencyReserves = "Emergency Reserves"

    public var icon: String {
        switch self {
        case .infrastructure: return "server.rack"
        case .marketing: return "megaphone.fill"
        case .security: return "shield.fill"
        case .communityGrants: return "gift.fill"
        case .operations: return "gearshape.fill"
        case .research: return "flask.fill"
        case .legal: return "scale.3d"
        case .emergencyReserves: return "lifepreserver.fill"
        }
    }
}

// MARK: - Voting Models

public struct VoteRecord: Identifiable, Codable {
    public let id: String
    public let proposalId: UInt64
    public let voter: String
    public let ballot: Ballot
    public let votingPower: Decimal
    public let votedAt: Date
    public let transactionHash: String?

    public init(
        id: String = UUID().uuidString,
        proposalId: UInt64,
        voter: String,
        ballot: Ballot,
        votingPower: Decimal,
        votedAt: Date = Date(),
        transactionHash: String? = nil
    ) {
        self.id = id
        self.proposalId = proposalId
        self.voter = voter
        self.ballot = ballot
        self.votingPower = votingPower
        self.votedAt = votedAt
        self.transactionHash = transactionHash
    }
}

public enum Ballot: String, Codable, CaseIterable {
    case yes = "Yes"
    case no = "No"
    case abstain = "Abstain"

    public var icon: String {
        switch self {
        case .yes: return "hand.thumbsup.fill"
        case .no: return "hand.thumbsdown.fill"
        case .abstain: return "minus.circle.fill"
        }
    }

    public var color: String {
        switch self {
        case .yes: return "green"
        case .no: return "red"
        case .abstain: return "gray"
        }
    }
}

// MARK: - Director Election Models

public struct DirectorCandidate: Identifiable, Codable {
    public let id: String
    public let address: String
    public let name: String
    public let manifesto: String
    public var votesReceived: UInt32
    public var votingPowerReceived: Decimal
    public let registeredAt: Date
    public var status: CandidateStatus
    public let avatarURL: String?
    public let website: String?
    public let socialLinks: [String: String]

    public init(
        id: String = UUID().uuidString,
        address: String,
        name: String,
        manifesto: String,
        votesReceived: UInt32 = 0,
        votingPowerReceived: Decimal = 0,
        registeredAt: Date = Date(),
        status: CandidateStatus = .active,
        avatarURL: String? = nil,
        website: String? = nil,
        socialLinks: [String: String] = [:]
    ) {
        self.id = id
        self.address = address
        self.name = name
        self.manifesto = manifesto
        self.votesReceived = votesReceived
        self.votingPowerReceived = votingPowerReceived
        self.registeredAt = registeredAt
        self.status = status
        self.avatarURL = avatarURL
        self.website = website
        self.socialLinks = socialLinks
    }
}

public enum CandidateStatus: String, Codable {
    case active = "Active"
    case elected = "Elected"
    case withdrawn = "Withdrawn"
    case disqualified = "Disqualified"
}

// MARK: - Consensus Day Models

public struct ConsensusDayState: Codable {
    public let currentYear: UInt32
    public let phase: ConsensusDayPhase
    public let phaseStartTime: Date
    public let phaseEndTime: Date
    public let totalProposals: UInt64
    public let activeProposals: UInt32
    public let totalParticipants: UInt32
    public let totalVotingPower: Decimal
    public let nextConsensusDayDate: Date

    public init(
        currentYear: UInt32,
        phase: ConsensusDayPhase,
        phaseStartTime: Date,
        phaseEndTime: Date,
        totalProposals: UInt64,
        activeProposals: UInt32,
        totalParticipants: UInt32,
        totalVotingPower: Decimal,
        nextConsensusDayDate: Date
    ) {
        self.currentYear = currentYear
        self.phase = phase
        self.phaseStartTime = phaseStartTime
        self.phaseEndTime = phaseEndTime
        self.totalProposals = totalProposals
        self.activeProposals = activeProposals
        self.totalParticipants = totalParticipants
        self.totalVotingPower = totalVotingPower
        self.nextConsensusDayDate = nextConsensusDayDate
    }

    public var timeRemainingInPhase: TimeInterval {
        return phaseEndTime.timeIntervalSince(Date())
    }

    public var phaseProgress: Double {
        let totalDuration = phaseEndTime.timeIntervalSince(phaseStartTime)
        let elapsed = Date().timeIntervalSince(phaseStartTime)
        guard totalDuration > 0 else { return 0 }
        return min(1.0, max(0, elapsed / totalDuration))
    }
}

public enum ConsensusDayPhase: String, Codable, CaseIterable {
    case inactive = "Inactive"
    case registration = "Registration"
    case voting = "Voting"
    case execution = "Execution"
    case minting = "Minting"
    case distribution = "Distribution"

    /// Determines the current Consensus Day phase based on calendar date
    /// - Jan 1 - Nov 30: Registration phase (proposals can be submitted)
    /// - Dec 1: Voting phase (Consensus Day - voting on proposals)
    /// - Dec 2-30: Execution phase (approved proposals being implemented)
    /// - Dec 31: Minting/Distribution phase (rewards and completion)
    public static func currentPhase(for date: Date = Date()) -> ConsensusDayPhase {
        let calendar = Calendar.current
        let month = calendar.component(.month, from: date)
        let day = calendar.component(.day, from: date)

        switch month {
        case 12: // December
            switch day {
            case 1:
                return .voting
            case 2...30:
                return .execution
            case 31:
                return .distribution
            default:
                return .inactive
            }
        default: // January through November
            return .registration
        }
    }

    /// Get the start date of the current phase in the given year
    public static func phaseStartDate(for phase: ConsensusDayPhase, year: Int) -> Date? {
        var components = DateComponents()
        components.year = year

        switch phase {
        case .registration:
            components.month = 1
            components.day = 1
        case .voting:
            components.month = 12
            components.day = 1
        case .execution:
            components.month = 12
            components.day = 2
        case .distribution, .minting:
            components.month = 12
            components.day = 31
        case .inactive:
            return nil
        }

        return Calendar.current.date(from: components)
    }

    /// Get the end date of the current phase in the given year
    public static func phaseEndDate(for phase: ConsensusDayPhase, year: Int) -> Date? {
        var components = DateComponents()
        components.year = year

        switch phase {
        case .registration:
            components.month = 11
            components.day = 30
            components.hour = 23
            components.minute = 59
            components.second = 59
        case .voting:
            components.month = 12
            components.day = 1
            components.hour = 23
            components.minute = 59
            components.second = 59
        case .execution:
            components.month = 12
            components.day = 30
            components.hour = 23
            components.minute = 59
            components.second = 59
        case .distribution, .minting:
            components.month = 12
            components.day = 31
            components.hour = 23
            components.minute = 59
            components.second = 59
        case .inactive:
            return nil
        }

        return Calendar.current.date(from: components)
    }

    /// Get the next Consensus Day (Dec 1) from the current date
    public static func nextConsensusDay(from date: Date = Date()) -> Date {
        let calendar = Calendar.current
        let year = calendar.component(.year, from: date)
        let month = calendar.component(.month, from: date)
        let day = calendar.component(.day, from: date)

        var components = DateComponents()
        components.month = 12
        components.day = 1

        // If we're past Dec 1, next Consensus Day is next year
        if month == 12 && day >= 1 {
            components.year = year + 1
        } else {
            components.year = year
        }

        return calendar.date(from: components) ?? date
    }

    /// Days remaining until Consensus Day
    public static func daysUntilConsensusDay(from date: Date = Date()) -> Int {
        let nextCD = nextConsensusDay(from: date)
        let calendar = Calendar.current
        let components = calendar.dateComponents([.day], from: date, to: nextCD)
        return components.day ?? 0
    }

    public var duration: TimeInterval {
        switch self {
        case .inactive: return 0
        case .registration: return 334 * 24 * 3600 // Jan 1 - Nov 30 (~11 months)
        case .voting: return 24 * 3600 // Dec 1 (24 hours)
        case .execution: return 29 * 24 * 3600 // Dec 2-30 (29 days)
        case .minting: return 12 * 3600 // Dec 31 first half
        case .distribution: return 12 * 3600 // Dec 31 second half
        }
    }

    public var description: String {
        switch self {
        case .inactive:
            return "Consensus Day cycle is inactive"
        case .registration:
            return "Submit proposals and register to participate (Jan 1 - Nov 30)"
        case .voting:
            return "Cast your votes on active proposals (Dec 1 - Consensus Day)"
        case .execution:
            return "Approved proposals are being implemented (Dec 2-30)"
        case .minting:
            return "Approved budgets are being minted (Dec 31)"
        case .distribution:
            return "Rewards are being distributed to participants (Dec 31)"
        }
    }

    public var icon: String {
        switch self {
        case .inactive: return "moon.zzz.fill"
        case .registration: return "person.badge.plus.fill"
        case .voting: return "checkmark.seal.fill"
        case .execution: return "hammer.fill"
        case .minting: return "bitcoinsign.circle.fill"
        case .distribution: return "gift.fill"
        }
    }

    /// Whether proposals can be submitted during this phase
    public var allowsProposalSubmission: Bool {
        return self == .registration
    }

    /// Whether voting is active during this phase
    public var allowsVoting: Bool {
        return self == .voting
    }
}

// MARK: - Participation & Rewards

public struct ParticipationReward: Identifiable, Codable {
    public let id: String
    public let consensusDayYear: UInt32
    public let amount: Decimal
    public let type: RewardType
    public let claimedAt: Date?
    public let transactionHash: String?

    public init(
        id: String = UUID().uuidString,
        consensusDayYear: UInt32,
        amount: Decimal,
        type: RewardType,
        claimedAt: Date? = nil,
        transactionHash: String? = nil
    ) {
        self.id = id
        self.consensusDayYear = consensusDayYear
        self.amount = amount
        self.type = type
        self.claimedAt = claimedAt
        self.transactionHash = transactionHash
    }

    public var isClaimed: Bool {
        claimedAt != nil
    }
}

public enum RewardType: String, Codable {
    case participation = "Participation"
    case validatorBonus = "Validator Bonus"
    case directorStipend = "Director Stipend"
    case proposerReward = "Proposer Reward"

    public var icon: String {
        switch self {
        case .participation: return "person.fill"
        case .validatorBonus: return "checkmark.shield.fill"
        case .directorStipend: return "star.fill"
        case .proposerReward: return "doc.badge.plus"
        }
    }
}

// MARK: - Governance Dashboard

public struct GovernanceDashboard {
    public let userVotingPower: Decimal
    public let userStake: Decimal
    public let stakeDurationMultiplier: Double
    public let historyMultiplier: Double
    public let activeProposals: [GovernanceProposal]
    public let userVotes: [VoteRecord]
    public let pendingRewards: [ParticipationReward]
    public let consensusDayState: ConsensusDayState?
    public let directors: [DirectorCandidate]

    public init(
        userVotingPower: Decimal,
        userStake: Decimal,
        stakeDurationMultiplier: Double = 1.0,
        historyMultiplier: Double = 1.0,
        activeProposals: [GovernanceProposal] = [],
        userVotes: [VoteRecord] = [],
        pendingRewards: [ParticipationReward] = [],
        consensusDayState: ConsensusDayState? = nil,
        directors: [DirectorCandidate] = []
    ) {
        self.userVotingPower = userVotingPower
        self.userStake = userStake
        self.stakeDurationMultiplier = stakeDurationMultiplier
        self.historyMultiplier = historyMultiplier
        self.activeProposals = activeProposals
        self.userVotes = userVotes
        self.pendingRewards = pendingRewards
        self.consensusDayState = consensusDayState
        self.directors = directors
    }

    public var effectiveVotingPower: Decimal {
        return userStake * Decimal(stakeDurationMultiplier) * Decimal(historyMultiplier)
    }

    public var totalUnclaimedRewards: Decimal {
        pendingRewards.filter { !$0.isClaimed }.reduce(0) { $0 + $1.amount }
    }

    public var proposalsUserVotedOn: Set<UInt64> {
        Set(userVotes.map { $0.proposalId })
    }
}

// MARK: - Submission Bond

public struct ProposalBond {
    public static let requiredAmount: Decimal = 10_000 // 10,000 ETR
    public static let refundCondition = "Refunded if proposal is approved"
    public static let slashCondition = "Slashed if proposal is spam/malicious"
}
