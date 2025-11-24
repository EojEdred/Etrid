//
//  DAO.swift
//  EtridWallet
//
//  Models for DAO management and governance
//

import Foundation

// MARK: - DAO

struct DAO: Identifiable, Codable, Hashable {
    let id: String
    let name: String
    let description: String
    let logoUrl: String?
    let governance: DAOGovernance
    let treasuryAddress: String
    var memberCount: Int
    var activeProposalCount: Int
    var treasuryValue: String
    let createdAt: Date
    var userRole: MemberRole

    enum MemberRole: String, Codable {
        case owner = "Owner"
        case admin = "Admin"
        case member = "Member"
        case voter = "Voter"
    }
}

// MARK: - DAO Governance

struct DAOGovernance: Codable, Hashable {
    let votingPeriod: Int // in seconds
    let quorumPercentage: Double
    let proposalThreshold: String // minimum tokens to create proposal
    let membershipType: MembershipType
    let tokenAddress: String?
    let nftAddress: String?

    enum MembershipType: String, Codable {
        case open = "Open"
        case tokenGated = "Token Gated"
        case nftGated = "NFT Gated"
        case inviteOnly = "Invite Only"
    }

    var votingPeriodDays: Int {
        votingPeriod / 86400
    }
}

// MARK: - DAO Proposal

struct DAOProposal: Identifiable, Codable, Hashable {
    let id: String
    let daoId: String
    let title: String
    let description: String
    let proposer: String
    let proposerName: String?
    let actions: [ProposalAction]
    let createdAt: Date
    let startTime: Date
    let endTime: Date
    var votesFor: String
    var votesAgainst: String
    var votesAbstain: String
    var status: ProposalStatus
    var userVote: DAOVoteType?

    enum ProposalStatus: String, Codable {
        case pending = "Pending"
        case active = "Active"
        case succeeded = "Succeeded"
        case defeated = "Defeated"
        case queued = "Queued"
        case executed = "Executed"
        case cancelled = "Cancelled"
        case expired = "Expired"
    }

    var totalVotes: Double {
        let forVotes = Double(votesFor) ?? 0
        let againstVotes = Double(votesAgainst) ?? 0
        let abstainVotes = Double(votesAbstain) ?? 0
        return forVotes + againstVotes + abstainVotes
    }

    var forPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return (Double(votesFor) ?? 0) / totalVotes * 100
    }

    var againstPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return (Double(votesAgainst) ?? 0) / totalVotes * 100
    }

    var abstainPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        return (Double(votesAbstain) ?? 0) / totalVotes * 100
    }

    var timeRemaining: String {
        let now = Date()
        if now < startTime {
            return "Starts in \(timeUntil(startTime))"
        } else if now > endTime {
            return "Ended"
        } else {
            return timeUntil(endTime)
        }
    }

    private func timeUntil(_ date: Date) -> String {
        let interval = date.timeIntervalSince(Date())
        let days = Int(interval / 86400)
        let hours = Int((interval.truncatingRemainder(dividingBy: 86400)) / 3600)

        if days > 0 {
            return "\(days)d \(hours)h"
        } else if hours > 0 {
            return "\(hours)h"
        } else {
            let minutes = Int((interval.truncatingRemainder(dividingBy: 3600)) / 60)
            return "\(minutes)m"
        }
    }
}

// MARK: - Proposal Action

struct ProposalAction: Codable, Hashable {
    let target: String // contract address
    let value: String // ETH value
    let signature: String // function signature
    let calldata: String // encoded parameters
    let description: String

    var actionType: ActionType {
        if value != "0" && !value.isEmpty {
            return .transfer
        } else if !calldata.isEmpty {
            return .contractCall
        } else {
            return .other
        }
    }

    enum ActionType {
        case transfer
        case contractCall
        case other
    }
}

// MARK: - Vote

struct DAOVote: Identifiable, Codable {
    let id: String
    let proposalId: String
    let voter: String
    let voterName: String?
    let voteType: DAOVoteType
    let votingPower: String
    let reason: String?
    let timestamp: Date
}

enum DAOVoteType: String, Codable, CaseIterable {
    case `for` = "For"
    case against = "Against"
    case abstain = "Abstain"

    var color: String {
        switch self {
        case .for: return "green"
        case .against: return "red"
        case .abstain: return "gray"
        }
    }
}

// MARK: - DAO Member

struct DAOMember: Identifiable, Codable {
    let id: String
    let userId: String
    let address: String
    let username: String?
    let role: DAO.MemberRole
    let votingPower: Double
    let joinedAt: Date
    var proposalsCreated: Int
    var votesCast: Int

    var votingPowerFormatted: String {
        if votingPower >= 1_000_000 {
            return String(format: "%.1fM", votingPower / 1_000_000)
        } else if votingPower >= 1_000 {
            return String(format: "%.1fK", votingPower / 1_000)
        } else {
            return String(format: "%.0f", votingPower)
        }
    }
}

// MARK: - DAO Treasury

struct DAOTreasury: Codable {
    let daoId: String
    var totalValue: String
    var assets: [TreasuryAsset]
    var transactions: [TreasuryTransaction]
    var analytics: TreasuryAnalytics?
}

struct TreasuryAsset: Identifiable, Codable {
    let id: String
    let symbol: String
    let name: String
    let balance: String
    let value: String // in USD
    let iconUrl: String?
    let contractAddress: String?
}

struct TreasuryTransaction: Identifiable, Codable {
    let id: String
    let type: TransactionType
    let asset: String
    let amount: String
    let from: String
    let to: String
    let timestamp: Date
    let txHash: String

    enum TransactionType: String, Codable {
        case deposit = "Deposit"
        case withdrawal = "Withdrawal"
        case swap = "Swap"
        case stake = "Stake"
        case unstake = "Unstake"
    }
}

struct TreasuryAnalytics: Codable {
    let totalValueUSD: Double
    let monthlyChange: Double
    let assetAllocation: [AssetAllocation]
    let transactionVolume: [VolumeData]

    struct AssetAllocation: Codable {
        let asset: String
        let percentage: Double
        let value: Double
    }

    struct VolumeData: Codable {
        let date: Date
        let volume: Double
    }
}

// MARK: - DAO Stats

struct DAOStats: Codable {
    let totalProposals: Int
    let passedProposals: Int
    let rejectedProposals: Int
    let averageParticipation: Double
    let treasuryGrowth: Double
    let activeMembers: Int

    var passRate: Double {
        guard totalProposals > 0 else { return 0 }
        return Double(passedProposals) / Double(totalProposals) * 100
    }
}

// MARK: - Create DAO Parameters

struct CreateDAOParams {
    let name: String
    let description: String
    let logoUrl: String?
    let votingPeriod: Int
    let quorumPercentage: Double
    let proposalThreshold: String
    let membershipType: DAOGovernance.MembershipType
    let tokenAddress: String?
    let nftAddress: String?
    let initialMembers: [String]?
    let initialTreasuryAmount: String?

    var governance: DAOGovernance {
        DAOGovernance(
            votingPeriod: votingPeriod,
            quorumPercentage: quorumPercentage,
            proposalThreshold: proposalThreshold,
            membershipType: membershipType,
            tokenAddress: tokenAddress,
            nftAddress: nftAddress
        )
    }
}

// MARK: - Create Proposal Parameters

struct CreateProposalParams {
    let daoId: String
    let title: String
    let description: String
    let actions: [ProposalAction]
    let votingPeriodDays: Int
}
