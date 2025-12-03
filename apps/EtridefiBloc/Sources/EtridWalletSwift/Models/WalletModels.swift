import Foundation

// MARK: - Wallet Account Model

public struct Wallet: Identifiable, Codable, Equatable {
    public let id: UUID
    public let name: String
    public let createdAt: Date
    public var accounts: [WalletAccountInfo]
    public var isBackedUp: Bool

    public init(
        id: UUID = UUID(),
        name: String,
        createdAt: Date = Date(),
        accounts: [WalletAccountInfo] = [],
        isBackedUp: Bool = false
    ) {
        self.id = id
        self.name = name
        self.createdAt = createdAt
        self.accounts = accounts
        self.isBackedUp = isBackedUp
    }
}

public struct WalletAccountInfo: Identifiable, Codable, Equatable {
    public let id: UUID
    public let address: String
    public let chain: SupportedChain
    public let derivationPath: String
    public var balances: [TokenBalance]
    public var isActive: Bool

    public init(
        id: UUID = UUID(),
        address: String,
        chain: SupportedChain,
        derivationPath: String,
        balances: [TokenBalance] = [],
        isActive: Bool = true
    ) {
        self.id = id
        self.address = address
        self.chain = chain
        self.derivationPath = derivationPath
        self.balances = balances
        self.isActive = isActive
    }

    public var totalUSDValue: Double {
        balances.reduce(0) { $0 + $1.usdValue }
    }
}

// MARK: - Token Models

public struct TokenBalance: Identifiable, Codable, Equatable {
    public let id: UUID
    public let token: Token
    public var amount: Decimal
    public var usdPrice: Double
    public var lastUpdated: Date

    public init(
        id: UUID = UUID(),
        token: Token,
        amount: Decimal,
        usdPrice: Double = 0,
        lastUpdated: Date = Date()
    ) {
        self.id = id
        self.token = token
        self.amount = amount
        self.usdPrice = usdPrice
        self.lastUpdated = lastUpdated
    }

    public var usdValue: Double {
        return (amount as NSDecimalNumber).doubleValue * usdPrice
    }

    public var formattedAmount: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = token.decimals > 6 ? 6 : token.decimals
        formatter.minimumFractionDigits = 2
        return formatter.string(from: amount as NSDecimalNumber) ?? "0.00"
    }

    public var formattedUSDValue: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        return formatter.string(from: NSNumber(value: usdValue)) ?? "$0.00"
    }
}

public struct Token: Identifiable, Codable, Equatable, Hashable {
    public let id: String
    public let symbol: String
    public let name: String
    public let decimals: Int
    public let chain: SupportedChain
    public let contractAddress: String?
    public let logoURL: String?
    public let isBridged: Bool
    public let originalChain: SupportedChain?

    public init(
        id: String,
        symbol: String,
        name: String,
        decimals: Int,
        chain: SupportedChain,
        contractAddress: String? = nil,
        logoURL: String? = nil,
        isBridged: Bool = false,
        originalChain: SupportedChain? = nil
    ) {
        self.id = id
        self.symbol = symbol
        self.name = name
        self.decimals = decimals
        self.chain = chain
        self.contractAddress = contractAddress
        self.logoURL = logoURL
        self.isBridged = isBridged
        self.originalChain = originalChain
    }

    // Native ËTRID token
    public static let etrid = Token(
        id: "etrid",
        symbol: "ETR",
        name: "ËTRID",
        decimals: 18,
        chain: .etrid
    )

    // Bridged tokens
    public static let bridgedSOL = Token(
        id: "bridged-sol",
        symbol: "bSOL",
        name: "Bridged Solana",
        decimals: 9,
        chain: .etrid,
        isBridged: true,
        originalChain: .solana
    )

    public static let bridgedBNB = Token(
        id: "bridged-bnb",
        symbol: "bBNB",
        name: "Bridged BNB",
        decimals: 18,
        chain: .etrid,
        isBridged: true,
        originalChain: .bnbChain
    )

    public static let bridgedXRP = Token(
        id: "bridged-xrp",
        symbol: "bXRP",
        name: "Bridged XRP",
        decimals: 6,
        chain: .etrid,
        isBridged: true,
        originalChain: .xrp
    )

    public static let bridgedTRX = Token(
        id: "bridged-trx",
        symbol: "bTRX",
        name: "Bridged TRON",
        decimals: 6,
        chain: .etrid,
        isBridged: true,
        originalChain: .tron
    )

    public static let bridgedADA = Token(
        id: "bridged-ada",
        symbol: "bADA",
        name: "Bridged Cardano",
        decimals: 6,
        chain: .etrid,
        isBridged: true,
        originalChain: .cardano
    )

    public static let bridgedXLM = Token(
        id: "bridged-xlm",
        symbol: "bXLM",
        name: "Bridged Stellar",
        decimals: 7,
        chain: .etrid,
        isBridged: true,
        originalChain: .stellar
    )

    public static let bridgedDOGE = Token(
        id: "bridged-doge",
        symbol: "bDOGE",
        name: "Bridged Dogecoin",
        decimals: 8,
        chain: .etrid,
        isBridged: true,
        originalChain: .dogecoin
    )

    // Stablecoins
    public static let usdt = Token(
        id: "usdt",
        symbol: "USDT",
        name: "Tether USD",
        decimals: 6,
        chain: .etrid
    )

    public static let usdc = Token(
        id: "usdc",
        symbol: "USDC",
        name: "USD Coin",
        decimals: 6,
        chain: .etrid
    )

    // ËTRID Stablecoin
    public static let edsc = Token(
        id: "edsc",
        symbol: "EDSC",
        name: "ËTRID Dollar Stablecoin",
        decimals: 18,
        chain: .etrid
    )

    // ===========================================
    // Wrapped ETR (wETR) on External Chains
    // These are your DEPLOYED tokens for trading
    // ===========================================

    public static let wETR_Solana = Token(
        id: "wetr-solana",
        symbol: "wETR",
        name: "Wrapped ËTRID (Solana)",
        decimals: 9,
        chain: .solana,
        contractAddress: "CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4",
        isBridged: false  // This is wETR, not a bridged external token
    )

    public static let wETR_BSC = Token(
        id: "wetr-bsc",
        symbol: "wETR",
        name: "Wrapped ËTRID (BSC)",
        decimals: 18,
        chain: .bnbChain,
        contractAddress: "0xcc9b37fed77a01329502f8844620577742eb0dc6",
        isBridged: false
    )

    public static let wETR_Polygon = Token(
        id: "wetr-polygon",
        symbol: "wETR",
        name: "Wrapped ËTRID (Polygon)",
        decimals: 18,
        chain: .ethereum,  // Note: Using ethereum as chain type for EVM compatibility
        contractAddress: "0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb",
        isBridged: false
    )

    public static let wETR_Ethereum = Token(
        id: "wetr-eth",
        symbol: "wETR",
        name: "Wrapped ËTRID (Ethereum)",
        decimals: 18,
        chain: .ethereum,
        contractAddress: "0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb",
        isBridged: false
    )

    public static let wETR_Arbitrum = Token(
        id: "wetr-arb",
        symbol: "wETR",
        name: "Wrapped ËTRID (Arbitrum)",
        decimals: 18,
        chain: .ethereum,  // Arbitrum is EVM compatible
        contractAddress: "0x1A065196152C2A70e54AC06D3a3433e3D8606eF3",
        isBridged: false
    )

    // All supported tokens
    public static var allTokens: [Token] {
        [
            // Native ËTRID
            .etrid, .edsc,
            // Wrapped ETR on external chains (DEPLOYED - LIVE)
            .wETR_Solana, .wETR_BSC, .wETR_Polygon, .wETR_Ethereum, .wETR_Arbitrum,
            // Bridged external tokens on ËTRID
            .bridgedSOL, .bridgedBNB, .bridgedXRP, .bridgedTRX,
            .bridgedADA, .bridgedXLM, .bridgedDOGE,
            // Stablecoins
            .usdt, .usdc
        ]
    }

    /// Wrapped ETR tokens available on external chains
    public static var wrappedETRTokens: [Token] {
        [.wETR_Solana, .wETR_BSC, .wETR_Polygon, .wETR_Ethereum, .wETR_Arbitrum]
    }

    public static var bridgedTokens: [Token] {
        allTokens.filter { $0.isBridged }
    }
}

// MARK: - Transaction Models

public struct Transaction: Identifiable, Codable {
    public let id: String
    public let hash: String
    public let type: TransactionType
    public let status: TransactionStatus
    public let from: String
    public let to: String
    public let amount: Decimal
    public let token: Token
    public let fee: Decimal
    public let feeToken: Token
    public let timestamp: Date
    public let blockNumber: UInt64?
    public let confirmations: Int
    public let memo: String?

    public init(
        id: String = UUID().uuidString,
        hash: String,
        type: TransactionType,
        status: TransactionStatus,
        from: String,
        to: String,
        amount: Decimal,
        token: Token,
        fee: Decimal,
        feeToken: Token,
        timestamp: Date,
        blockNumber: UInt64? = nil,
        confirmations: Int = 0,
        memo: String? = nil
    ) {
        self.id = id
        self.hash = hash
        self.type = type
        self.status = status
        self.from = from
        self.to = to
        self.amount = amount
        self.token = token
        self.fee = fee
        self.feeToken = feeToken
        self.timestamp = timestamp
        self.blockNumber = blockNumber
        self.confirmations = confirmations
        self.memo = memo
    }
}

public enum TransactionType: String, Codable, CaseIterable {
    case send = "Send"
    case receive = "Receive"
    case swap = "Swap"
    case bridgeDeposit = "Bridge Deposit"
    case bridgeWithdraw = "Bridge Withdraw"
    case stake = "Stake"
    case unstake = "Unstake"
    case governanceVote = "Governance Vote"
    case governanceProposal = "Governance Proposal"
    case claimReward = "Claim Reward"

    public var icon: String {
        switch self {
        case .send: return "arrow.up.circle.fill"
        case .receive: return "arrow.down.circle.fill"
        case .swap: return "arrow.triangle.2.circlepath.circle.fill"
        case .bridgeDeposit: return "arrow.right.circle.fill"
        case .bridgeWithdraw: return "arrow.left.circle.fill"
        case .stake: return "lock.circle.fill"
        case .unstake: return "lock.open.fill"
        case .governanceVote: return "checkmark.circle.fill"
        case .governanceProposal: return "doc.circle.fill"
        case .claimReward: return "gift.circle.fill"
        }
    }
}

public enum TransactionStatus: String, Codable {
    case pending = "Pending"
    case confirmed = "Confirmed"
    case failed = "Failed"
    case cancelled = "Cancelled"

    public var color: String {
        switch self {
        case .pending: return "orange"
        case .confirmed: return "green"
        case .failed: return "red"
        case .cancelled: return "gray"
        }
    }
}

// MARK: - Bridge Models

public struct BridgeDeposit: Identifiable, Codable {
    public let id: String
    public let sourceChain: SupportedChain
    public let sourceAddress: String
    public let sourceTxHash: String
    public let targetAddress: String
    public let amount: Decimal
    public let token: Token
    public let status: BridgeStatus
    public let confirmations: Int
    public let requiredConfirmations: Int
    public let createdAt: Date
    public let completedAt: Date?
    public let etridTxHash: String?

    public init(
        id: String = UUID().uuidString,
        sourceChain: SupportedChain,
        sourceAddress: String,
        sourceTxHash: String,
        targetAddress: String,
        amount: Decimal,
        token: Token,
        status: BridgeStatus = .pending,
        confirmations: Int = 0,
        requiredConfirmations: Int,
        createdAt: Date = Date(),
        completedAt: Date? = nil,
        etridTxHash: String? = nil
    ) {
        self.id = id
        self.sourceChain = sourceChain
        self.sourceAddress = sourceAddress
        self.sourceTxHash = sourceTxHash
        self.targetAddress = targetAddress
        self.amount = amount
        self.token = token
        self.status = status
        self.confirmations = confirmations
        self.requiredConfirmations = requiredConfirmations
        self.createdAt = createdAt
        self.completedAt = completedAt
        self.etridTxHash = etridTxHash
    }

    public var progress: Double {
        guard requiredConfirmations > 0 else { return 0 }
        return min(1.0, Double(confirmations) / Double(requiredConfirmations))
    }
}

public struct BridgeWithdrawal: Identifiable, Codable {
    public let id: String
    public let targetChain: SupportedChain
    public let targetAddress: String
    public let amount: Decimal
    public let token: Token
    public let status: BridgeStatus
    public let etridTxHash: String
    public let targetTxHash: String?
    public let createdAt: Date
    public let completedAt: Date?

    public init(
        id: String = UUID().uuidString,
        targetChain: SupportedChain,
        targetAddress: String,
        amount: Decimal,
        token: Token,
        status: BridgeStatus = .pending,
        etridTxHash: String,
        targetTxHash: String? = nil,
        createdAt: Date = Date(),
        completedAt: Date? = nil
    ) {
        self.id = id
        self.targetChain = targetChain
        self.targetAddress = targetAddress
        self.amount = amount
        self.token = token
        self.status = status
        self.etridTxHash = etridTxHash
        self.targetTxHash = targetTxHash
        self.createdAt = createdAt
        self.completedAt = completedAt
    }
}

public enum BridgeStatus: String, Codable {
    case pending = "Pending"
    case confirming = "Confirming"
    case confirmed = "Confirmed"
    case minting = "Minting"
    case completed = "Completed"
    case failed = "Failed"

    public var displayText: String {
        rawValue
    }
}

// MARK: - Staking Models

public struct StakeInfo: Codable {
    public let validatorAddress: String
    public let validatorName: String
    public let stakedAmount: Decimal
    public let rewards: Decimal
    public let stakingStartDate: Date
    public let unbondingEndDate: Date?
    public let isUnbonding: Bool
    public let votingPower: Decimal
    public let multiplier: Double

    public init(
        validatorAddress: String,
        validatorName: String,
        stakedAmount: Decimal,
        rewards: Decimal,
        stakingStartDate: Date,
        unbondingEndDate: Date? = nil,
        isUnbonding: Bool = false,
        votingPower: Decimal = 0,
        multiplier: Double = 1.0
    ) {
        self.validatorAddress = validatorAddress
        self.validatorName = validatorName
        self.stakedAmount = stakedAmount
        self.rewards = rewards
        self.stakingStartDate = stakingStartDate
        self.unbondingEndDate = unbondingEndDate
        self.isUnbonding = isUnbonding
        self.votingPower = votingPower
        self.multiplier = multiplier
    }

    public var effectiveVotingPower: Decimal {
        return votingPower * Decimal(multiplier)
    }
}

// MARK: - App State

public enum AppState {
    case onboarding
    case locked
    case unlocked
    case loading
}

public enum OnboardingStep: Int, CaseIterable {
    case welcome = 0
    case createOrImport = 1
    case createMnemonic = 2
    case verifyMnemonic = 3
    case setPasscode = 4
    case enableBiometrics = 5
    case complete = 6
}
