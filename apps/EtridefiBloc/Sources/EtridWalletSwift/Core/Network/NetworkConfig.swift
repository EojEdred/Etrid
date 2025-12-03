import Foundation

// MARK: - ËTRID Network Configuration

/// Network endpoints for Primearc Core Chain and PBC chains
public struct NetworkConfig {

    // MARK: - Instance Properties (for backward compatibility)

    public let rpcURL: String
    public let wsURL: String?
    public let explorerURL: String
    public let chainId: Int?
    public let isTestnet: Bool

    public init(
        rpcURL: String,
        wsURL: String? = nil,
        explorerURL: String = "https://explorer.etrid.network",
        chainId: Int? = nil,
        isTestnet: Bool = false
    ) {
        self.rpcURL = rpcURL
        self.wsURL = wsURL
        self.explorerURL = explorerURL
        self.chainId = chainId
        self.isTestnet = isTestnet
    }

    /// ËTRID Mainnet configuration (Primearc Core Chain)
    public static let etridMainnet = NetworkConfig(
        rpcURL: primearcCoreHTTP,
        wsURL: primearcCoreWS,
        explorerURL: "https://explorer.etrid.org",
        chainId: 8888,
        isTestnet: false
    )

    /// ËTRID Testnet configuration
    public static let etridTestnet = NetworkConfig(
        rpcURL: "https://testnet-rpc.etrid.network",
        wsURL: "wss://testnet-ws.etrid.network",
        explorerURL: "https://testnet-explorer.etrid.network",
        chainId: 8889,
        isTestnet: true
    )

    // MARK: - Primearc Core Chain (Relay Chain)

    /// Primearc Core Chain WebSocket endpoint (Public SSL - after DNS propagation)
    public static let primearcCoreWS = "wss://rpc.etrid.org"

    /// Primearc Core Chain HTTP endpoint (Public SSL)
    public static let primearcCoreHTTP = "https://rpc.etrid.org"

    /// Fallback endpoints (Contabo proxy - stable, won't migrate)
    public static let primearcCoreFallbackWS = "ws://157.173.200.80:9944"
    public static let primearcCoreFallbackHTTP = "http://157.173.200.80:9944"

    /// Tailscale endpoints (internal network - gizzi-io-validator)
    public static let primearcCoreTailscaleWS = "ws://100.96.84.69:9944"
    public static let primearcCoreTailscaleHTTP = "http://100.96.84.69:9944"

    /// ts-val-06 Tailscale (proxy node)
    public static let proxyTailscaleWS = "ws://100.89.102.75:9944"

    // MARK: - PBC Chain RPC Endpoints

    /// Partition Burst Chain (PBC) endpoints
    /// Note: These ports are configured but some may need rebuilt binaries for RPC binding
    public enum PBCChain: String, CaseIterable {
        case xrp = "XRP-PBC"
        case btc = "BTC-PBC"
        case ada = "ADA-PBC"
        case doge = "DOGE-PBC"
        case trx = "TRX-PBC"
        case matic = "MATIC-PBC"
        case bnb = "BNB-PBC"
        case link = "LINK-PBC"
        case scUsdt = "SC-USDT-PBC"
        case edsc = "EDSC-PBC"
        case sol = "SOL-PBC"
        case xlm = "XLM-PBC"

        /// RPC port for this PBC chain
        public var rpcPort: Int {
            switch self {
            case .xrp: return 9945
            case .btc: return 9946
            case .ada: return 9947
            case .doge: return 9948
            case .trx: return 9949
            case .matic: return 9950
            case .bnb: return 9951
            case .link: return 9952
            case .scUsdt: return 9953
            case .edsc: return 9954
            case .sol: return 9955
            case .xlm: return 9956
            }
        }

        /// P2P port for this PBC chain (collators are running on these)
        public var p2pPort: Int {
            switch self {
            case .xrp: return 30341
            case .btc: return 30342
            case .ada: return 30343
            case .doge: return 30344
            case .trx: return 30345
            case .matic: return 30346
            case .bnb: return 30347
            case .link: return 30348
            case .scUsdt: return 30349
            case .edsc: return 30350
            case .sol: return 30351
            case .xlm: return 30352
            }
        }

        /// WebSocket URL for this PBC chain
        public func wsURL(vmIP: String = "100.96.84.69") -> String {
            return "ws://\(vmIP):\(rpcPort)"
        }

        /// HTTP URL for this PBC chain
        public func httpURL(vmIP: String = "100.96.84.69") -> String {
            return "http://\(vmIP):\(rpcPort)"
        }

        /// Display name
        public var displayName: String {
            rawValue
        }

        /// Native token symbol
        public var nativeToken: String {
            switch self {
            case .xrp: return "bXRP"
            case .btc: return "bBTC"
            case .ada: return "bADA"
            case .doge: return "bDOGE"
            case .trx: return "bTRX"
            case .matic: return "bMATIC"
            case .bnb: return "bBNB"
            case .link: return "bLINK"
            case .scUsdt: return "scUSDT"
            case .edsc: return "EDSC"
            case .sol: return "bSOL"
            case .xlm: return "bXLM"
            }
        }

        /// Decimals for the bridged token
        public var decimals: Int {
            switch self {
            case .btc: return 8
            case .xrp, .trx, .ada: return 6
            case .xlm: return 7
            case .doge: return 8
            case .sol: return 9
            case .matic, .bnb, .link, .edsc: return 18
            case .scUsdt: return 6
            }
        }
    }

    // MARK: - Active Network Configuration

    /// Current network environment
    public enum Environment {
        case mainnet
        case testnet
        case local

        public var primearcWSEndpoint: String {
            switch self {
            case .mainnet:
                return "wss://rpc.etrid.io"  // Future mainnet endpoint
            case .testnet:
                return NetworkConfig.primearcCoreWS  // Current Tailscale endpoint
            case .local:
                return "ws://127.0.0.1:9944"
            }
        }
    }

    /// Current active environment
    public static var currentEnvironment: Environment = .testnet
}

// MARK: - Substrate JSON-RPC Service

/// Service for making Substrate JSON-RPC calls to ËTRID network
public actor SubstrateRPCService {

    private let endpoint: URL
    private let session: URLSession

    public init(endpoint: String) throws {
        guard let url = URL(string: endpoint) else {
            throw RPCError.invalidEndpoint
        }
        self.endpoint = url
        self.session = URLSession(configuration: .default)
    }

    /// Make a JSON-RPC call
    public func call<T: Decodable>(
        method: String,
        params: [Any] = []
    ) async throws -> T {
        var request = URLRequest(url: endpoint)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let body: [String: Any] = [
            "jsonrpc": "2.0",
            "id": Int.random(in: 1...999999),
            "method": method,
            "params": params
        ]

        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw RPCError.httpError
        }

        let rpcResponse = try JSONDecoder().decode(RPCResponse<T>.self, from: data)

        if let error = rpcResponse.error {
            throw RPCError.rpcError(code: error.code, message: error.message)
        }

        guard let result = rpcResponse.result else {
            throw RPCError.noResult
        }

        return result
    }

    /// Get chain info (genesis hash, runtime version, etc.)
    public func getChainInfo() async throws -> ChainInfo {
        async let genesisHash: String = call(method: "chain_getBlockHash", params: [0])
        async let runtimeVersion: RuntimeVersion = call(method: "state_getRuntimeVersion")

        return try await ChainInfo(
            genesisHash: genesisHash,
            runtimeVersion: runtimeVersion
        )
    }

    /// Get account balance
    public func getBalance(address: String) async throws -> AccountBalance {
        let storageKey = try encodeStorageKey(module: "System", item: "Account", key: address)
        let result: String? = try await call(method: "state_getStorage", params: [storageKey])

        // Decode SCALE-encoded AccountInfo
        if let data = result {
            return try decodeAccountBalance(from: data)
        }

        return AccountBalance(free: 0, reserved: 0, frozen: 0)
    }

    /// Submit signed extrinsic
    public func submitExtrinsic(_ signedExtrinsic: String) async throws -> String {
        return try await call(method: "author_submitExtrinsic", params: [signedExtrinsic])
    }

    /// Subscribe to new blocks (requires WebSocket)
    public func subscribeNewHeads() async throws -> String {
        return try await call(method: "chain_subscribeNewHeads")
    }

    // MARK: - Encoding Helpers

    private func encodeStorageKey(module: String, item: String, key: String) throws -> String {
        // XXHash128 of module + item + key
        // Simplified - in production use proper Substrate storage key encoding
        let moduleHash = xxhash128(module)
        let itemHash = xxhash128(item)
        let keyHash = blake2b128Concat(key)

        return "0x" + moduleHash + itemHash + keyHash
    }

    private func xxhash128(_ input: String) -> String {
        // Placeholder - implement XXHash128
        return String(repeating: "0", count: 32)
    }

    private func blake2b128Concat(_ input: String) -> String {
        // Placeholder - implement Blake2b128Concat
        return String(repeating: "0", count: 64)
    }

    private func decodeAccountBalance(from hexData: String) throws -> AccountBalance {
        // Placeholder - implement SCALE decoding
        return AccountBalance(free: 0, reserved: 0, frozen: 0)
    }
}

// MARK: - RPC Types

public struct RPCResponse<T: Decodable>: Decodable {
    public let jsonrpc: String
    public let id: Int
    public let result: T?
    public let error: RPCErrorInfo?
}

public struct RPCErrorInfo: Decodable {
    public let code: Int
    public let message: String
}

public struct ChainInfo {
    public let genesisHash: String
    public let runtimeVersion: RuntimeVersion
}

public struct RuntimeVersion: Decodable {
    public let specName: String
    public let implName: String
    public let specVersion: Int
    public let implVersion: Int
    public let transactionVersion: Int
}

public struct AccountBalance {
    public let free: UInt128
    public let reserved: UInt128
    public let frozen: UInt128

    public var transferable: UInt128 {
        free > frozen ? free - frozen : 0
    }
}

public typealias UInt128 = UInt64  // Simplified - use proper UInt128 in production

public enum RPCError: LocalizedError {
    case invalidEndpoint
    case httpError
    case rpcError(code: Int, message: String)
    case noResult
    case encodingError
    case decodingError

    public var errorDescription: String? {
        switch self {
        case .invalidEndpoint:
            return "Invalid RPC endpoint URL"
        case .httpError:
            return "HTTP request failed"
        case .rpcError(let code, let message):
            return "RPC Error \(code): \(message)"
        case .noResult:
            return "No result in RPC response"
        case .encodingError:
            return "Failed to encode request"
        case .decodingError:
            return "Failed to decode response"
        }
    }
}

// MARK: - Governance Configuration

/// Governance-related RPC methods for ËTRID Consensus Day
public extension SubstrateRPCService {

    /// Get current consensus day phase
    func getConsensusPhase() async throws -> ConsensusPhase {
        return try await call(method: "governance_currentPhase")
    }

    /// Get active proposals
    func getActiveProposals() async throws -> [RPCProposal] {
        return try await call(method: "governance_activeProposals")
    }

    /// Get user's voting power
    func getVotingPower(address: String) async throws -> VotingPower {
        return try await call(method: "governance_votingPower", params: [address])
    }

    /// Submit a vote
    func submitVote(
        proposalId: String,
        vote: VoteType,
        signedExtrinsic: String
    ) async throws -> String {
        return try await call(method: "governance_vote", params: [
            proposalId,
            vote.rawValue,
            signedExtrinsic
        ])
    }

    /// Get distribution schedule
    func getDistributionSchedule() async throws -> DistributionSchedule {
        return try await call(method: "governance_distributionSchedule")
    }
}

// MARK: - Governance Types

public enum ConsensusPhase: String, Decodable {
    case proposalSubmission = "ProposalSubmission"
    case voting = "Voting"
    case execution = "Execution"
    case distribution = "Distribution"
}

// Note: ProposalStatus is defined in GovernanceModels.swift

public struct RPCProposal: Decodable, Identifiable {
    public let id: String
    public let title: String
    public let description: String
    public let proposer: String
    public let status: String  // Use String for RPC response, convert to ProposalStatus in service layer
    public let votesFor: UInt128
    public let votesAgainst: UInt128
    public let votesAbstain: UInt128
    public let createdAt: Date
    public let expiresAt: Date
}

public struct VotingPower: Decodable {
    public let totalPower: UInt128
    public let delegatedPower: UInt128
    public let ownPower: UInt128
    public let lockedUntil: Date?
}

public enum VoteType: String {
    case aye = "Aye"
    case nay = "Nay"
    case abstain = "Abstain"
}

public struct DistributionSchedule: Decodable {
    public let nextDistributionBlock: UInt64
    public let totalRewards: UInt128
    public let validatorShare: Double
    public let collatorShare: Double
    public let stakeholderShare: Double
}
