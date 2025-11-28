//
//  ETHPBC.swift
//  EtridWallet
//
//  Ethereum PBC (Precompiled Blockchain) Models
//

import Foundation

// MARK: - Precompile Address
enum PrecompileAddress: String, Codable, CaseIterable {
    case oracle = "0x0000000000000000000000000000000000000800"
    case governance = "0x0000000000000000000000000000000000000801"
    case staking = "0x0000000000000000000000000000000000000802"
    case ethWrap = "0x0000000000000000000000000000000000000803"
    case bridge = "0x0000000000000000000000000000000000000804"
    case tokenRegistry = "0x0000000000000000000000000000000000000805"
    case stateProof = "0x0000000000000000000000000000000000000806"

    var name: String {
        switch self {
        case .oracle: return "Oracle"
        case .governance: return "Governance"
        case .staking: return "Staking"
        case .ethWrap: return "ETH Wrap/Unwrap"
        case .bridge: return "Bridge"
        case .tokenRegistry: return "Token Registry"
        case .stateProof: return "State Proof"
        }
    }

    var description: String {
        switch self {
        case .oracle:
            return "Access real-time price feeds and oracle data"
        case .governance:
            return "Participate in on-chain governance and voting"
        case .staking:
            return "Stake and unstake tokens for network security"
        case .ethWrap:
            return "Wrap ETH to wETH and unwrap wETH to ETH"
        case .bridge:
            return "Bridge assets across different chains"
        case .tokenRegistry:
            return "Query token information and metadata"
        case .stateProof:
            return "Verify state proofs and cross-chain data"
        }
    }

    var icon: String {
        switch self {
        case .oracle: return "chart.line.uptrend.xyaxis"
        case .governance: return "person.3.fill"
        case .staking: return "lock.fill"
        case .ethWrap: return "arrow.triangle.2.circlepath"
        case .bridge: return "arrow.left.arrow.right"
        case .tokenRegistry: return "list.bullet.rectangle"
        case .stateProof: return "checkmark.shield.fill"
        }
    }
}

// MARK: - Precompile Call
struct PrecompileCall: Codable, Identifiable {
    let id: String
    let precompile: PrecompileAddress
    let method: String
    let params: [String: Any]
    let gasLimit: String?
    let value: String?
    let timestamp: Date

    enum CodingKeys: String, CodingKey {
        case id, precompile, method, gasLimit, value, timestamp
    }

    init(id: String = UUID().uuidString, precompile: PrecompileAddress, method: String, params: [String: Any], gasLimit: String? = nil, value: String? = nil) {
        self.id = id
        self.precompile = precompile
        self.method = method
        self.params = params
        self.gasLimit = gasLimit
        self.value = value
        self.timestamp = Date()
    }

    // Custom encoding/decoding for Any type
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decode(String.self, forKey: .id)
        precompile = try container.decode(PrecompileAddress.self, forKey: .precompile)
        method = try container.decode(String.self, forKey: .method)
        gasLimit = try container.decodeIfPresent(String.self, forKey: .gasLimit)
        value = try container.decodeIfPresent(String.self, forKey: .value)
        timestamp = try container.decode(Date.self, forKey: .timestamp)
        params = [:] // Simplified for now
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(id, forKey: .id)
        try container.encode(precompile, forKey: .precompile)
        try container.encode(method, forKey: .method)
        try container.encodeIfPresent(gasLimit, forKey: .gasLimit)
        try container.encodeIfPresent(value, forKey: .value)
        try container.encode(timestamp, forKey: .timestamp)
    }
}

// MARK: - Precompile Response
struct PrecompileResponse: Codable {
    let success: Bool
    let result: String?
    let error: String?
    let gasUsed: String?
    let transactionHash: String?

    var txHashDisplay: String? {
        guard let hash = transactionHash else { return nil }
        let prefix = hash.prefix(8)
        let suffix = hash.suffix(6)
        return "\(prefix)...\(suffix)"
    }
}

// MARK: - Oracle Price
struct OraclePrice: Codable, Identifiable {
    let id: String
    let token: String
    let price: String
    let decimals: Int
    let timestamp: Date
    let source: String
    let confidence: Double // 0-100

    var priceDisplay: String {
        if let value = Double(price) {
            let divisor = pow(10.0, Double(decimals))
            let actualPrice = value / divisor
            return String(format: "$%.2f", actualPrice)
        }
        return price
    }

    var confidenceDisplay: String {
        String(format: "%.1f%%", confidence)
    }

    var confidenceColor: String {
        if confidence >= 95 {
            return "#00FF00"
        } else if confidence >= 80 {
            return "#FFA500"
        } else {
            return "#FF0000"
        }
    }
}

// MARK: - Governance Proposal
struct ETHPBCGovernanceProposal: Codable, Identifiable {
    let id: String
    let proposalId: String
    let title: String
    let description: String
    let proposer: String
    let startBlock: Int
    let endBlock: Int
    let currentBlock: Int
    let votesFor: String
    let votesAgainst: String
    let votesAbstain: String
    let status: ProposalStatus
    let quorum: String
    let createdAt: Date

    enum ProposalStatus: String, Codable {
        case pending = "pending"
        case active = "active"
        case succeeded = "succeeded"
        case defeated = "defeated"
        case executed = "executed"
        case cancelled = "cancelled"

        var color: String {
            switch self {
            case .pending: return "#808080"
            case .active: return "#FFA500"
            case .succeeded: return "#00FF00"
            case .defeated: return "#FF0000"
            case .executed: return "#0000FF"
            case .cancelled: return "#8B0000"
            }
        }
    }

    var proposerDisplay: String {
        let prefix = proposer.prefix(8)
        let suffix = proposer.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var totalVotes: Double {
        let forVotes = Double(votesFor) ?? 0
        let againstVotes = Double(votesAgainst) ?? 0
        let abstainVotes = Double(votesAbstain) ?? 0
        return forVotes + againstVotes + abstainVotes
    }

    var forPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        let forVotes = Double(votesFor) ?? 0
        return (forVotes / totalVotes) * 100
    }

    var againstPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        let againstVotes = Double(votesAgainst) ?? 0
        return (againstVotes / totalVotes) * 100
    }

    var abstainPercentage: Double {
        guard totalVotes > 0 else { return 0 }
        let abstainVotes = Double(votesAbstain) ?? 0
        return (abstainVotes / totalVotes) * 100
    }

    var blocksRemaining: Int {
        max(endBlock - currentBlock, 0)
    }

    var isActive: Bool {
        status == .active && currentBlock >= startBlock && currentBlock <= endBlock
    }
}

// MARK: - Vote Choice
enum VoteChoice: String, Codable {
    case forVote = "for"
    case against = "against"
    case abstain = "abstain"

    var displayName: String {
        switch self {
        case .forVote: return "For"
        case .against: return "Against"
        case .abstain: return "Abstain"
        }
    }

    var color: String {
        switch self {
        case .forVote: return "#00FF00"
        case .against: return "#FF0000"
        case .abstain: return "#808080"
        }
    }
}

// MARK: - Staking Info
struct StakingInfo: Codable {
    let stakedAmount: String
    let rewards: String
    let apr: Double
    let lockPeriod: Int // blocks
    let unlockBlock: Int?
    let canUnstake: Bool

    var stakedDisplay: String {
        if let value = Double(stakedAmount) {
            return String(format: "%.4f ETR", value)
        }
        return stakedAmount
    }

    var rewardsDisplay: String {
        if let value = Double(rewards) {
            return String(format: "%.4f ETR", value)
        }
        return rewards
    }

    var aprDisplay: String {
        String(format: "%.2f%%", apr)
    }

    var blocksUntilUnlock: Int? {
        guard let unlock = unlockBlock else { return nil }
        return max(unlock - lockPeriod, 0)
    }
}

// MARK: - Wrapped ETH
struct WrappedETH: Codable {
    let wethBalance: String
    let ethBalance: String
    let contractAddress: String

    var wethDisplay: String {
        if let value = Double(wethBalance) {
            return String(format: "%.6f wETH", value)
        }
        return wethBalance
    }

    var ethDisplay: String {
        if let value = Double(ethBalance) {
            return String(format: "%.6f ETH", value)
        }
        return ethBalance
    }
}

// MARK: - Token Info
struct TokenInfo: Codable, Identifiable {
    let id: String
    let address: String
    let name: String
    let symbol: String
    let decimals: Int
    let totalSupply: String
    let verified: Bool

    var addressDisplay: String {
        let prefix = address.prefix(8)
        let suffix = address.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var supplyDisplay: String {
        if let value = Double(totalSupply) {
            let divisor = pow(10.0, Double(decimals))
            let actualSupply = value / divisor
            return String(format: "%.2f %@", actualSupply, symbol)
        }
        return "\(totalSupply) \(symbol)"
    }
}

// MARK: - State Proof
struct StateProof: Codable {
    let blockHash: String
    let stateRoot: String
    let proof: [String]
    let key: String
    let value: String
    let verified: Bool

    var blockHashDisplay: String {
        let prefix = blockHash.prefix(8)
        let suffix = blockHash.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var proofSize: Int {
        proof.count
    }
}

// MARK: - Oracle Request
struct GetOraclePriceRequest: Codable {
    let token: String
}

// MARK: - Vote Request
struct VoteOnProposalRequest: Codable {
    let proposalId: String
    let choice: VoteChoice
    let votingPower: String?
}

// MARK: - Stake Request
struct StakeTokensRequest: Codable {
    let amount: String
    let lockPeriod: Int? // blocks
}

// MARK: - Unstake Request
struct UnstakeTokensRequest: Codable {
    let amount: String
}

// MARK: - Wrap Request
struct WrapETHRequest: Codable {
    let amount: String
}

// MARK: - Unwrap Request
struct UnwrapETHRequest: Codable {
    let amount: String
}

// MARK: - Token Query Request
struct QueryTokenRequest: Codable {
    let address: String
}

// MARK: - Verify Proof Request
struct VerifyProofRequest: Codable {
    let blockHash: String
    let stateRoot: String
    let proof: [String]
    let key: String
    let value: String
}

// MARK: - Precompile Transaction
struct PrecompileTransaction: Codable, Identifiable {
    let id: String
    let txHash: String
    let precompile: PrecompileAddress
    let method: String
    let status: TransactionStatus
    let gasUsed: String?
    let timestamp: Date
    let result: String?

    enum TransactionStatus: String, Codable {
        case pending = "pending"
        case confirmed = "confirmed"
        case failed = "failed"

        var color: String {
            switch self {
            case .pending: return "#FFA500"
            case .confirmed: return "#00FF00"
            case .failed: return "#FF0000"
            }
        }
    }

    var txHashDisplay: String {
        let prefix = txHash.prefix(8)
        let suffix = txHash.suffix(6)
        return "\(prefix)...\(suffix)"
    }
}
