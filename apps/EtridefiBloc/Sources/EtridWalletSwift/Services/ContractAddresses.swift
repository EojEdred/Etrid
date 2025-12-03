import Foundation

// MARK: - Contract Addresses Configuration

/// Deployed contract addresses for ËTRID multichain ecosystem
public struct ContractAddresses {

    // MARK: - Network Identifiers

    public enum Network: String, CaseIterable, Codable, Sendable {
        case primearc = "primearc"         // Primearc Core Chain (native ETR)
        case ethereum = "ethereum"          // Ethereum Mainnet
        case bnbChain = "bsc"              // BNB Smart Chain
        case polygon = "polygon"            // Polygon PoS
        case arbitrum = "arbitrum"          // Arbitrum One
        case optimism = "optimism"          // Optimism
        case base = "base"                  // Base
        case solana = "solana"              // Solana

        public var chainId: Int {
            switch self {
            case .primearc: return 5765  // ËTRID chain ID
            case .ethereum: return 1
            case .bnbChain: return 56
            case .polygon: return 137
            case .arbitrum: return 42161
            case .optimism: return 10
            case .base: return 8453
            case .solana: return 0  // Solana uses different addressing
            }
        }

        public var rpcURL: String {
            switch self {
            case .primearc: return "https://rpc.primearc.etrid.org"
            case .ethereum: return "https://eth.llamarpc.com"
            case .bnbChain: return "https://bsc-dataseed.binance.org"
            case .polygon: return "https://polygon-rpc.com"
            case .arbitrum: return "https://arb1.arbitrum.io/rpc"
            case .optimism: return "https://mainnet.optimism.io"
            case .base: return "https://mainnet.base.org"
            case .solana: return "https://api.mainnet-beta.solana.com"
            }
        }

        public var explorerURL: String {
            switch self {
            case .primearc: return "https://explorer.primearc.etrid.org"
            case .ethereum: return "https://etherscan.io"
            case .bnbChain: return "https://bscscan.com"
            case .polygon: return "https://polygonscan.com"
            case .arbitrum: return "https://arbiscan.io"
            case .optimism: return "https://optimistic.etherscan.io"
            case .base: return "https://basescan.org"
            case .solana: return "https://solscan.io"
            }
        }
    }

    // MARK: - Token Contracts

    /// Wrapped ETR (wETR) contract addresses on each chain
    /// DEPLOYED CONTRACTS - LIVE ON MAINNET
    public static let wrappedETR: [Network: String] = [
        .solana: "CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4",       // Solana SPL Token
        .bnbChain: "0xcc9b37fed77a01329502f8844620577742eb0dc6",       // BSC BEP-20
        .polygon: "0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb",        // Polygon
        .ethereum: "0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb",       // Ethereum
        .arbitrum: "0x1A065196152C2A70e54AC06D3a3433e3D8606eF3",       // Arbitrum
    ]

    /// EDSC (ËTRID Stablecoin) contract addresses
    public static let edsc: [Network: String] = [
        .primearc: "native",  // Native on Primearc
        .ethereum: "0x0000000000000000000000000000000000000000",
        .bnbChain: "0x0000000000000000000000000000000000000000",
    ]

    // MARK: - DeFi Contracts

    /// MasterChef (Yield Farming) contract addresses
    public static let masterChef: [Network: String] = [
        .primearc: "0x0000000000000000000000000000000000000000",   // TODO: Deploy
        .ethereum: "0x0000000000000000000000000000000000000000",
        .bnbChain: "0x0000000000000000000000000000000000000000",
    ]

    /// PrimeSwap Router (DEX) contract addresses
    public static let primeSwapRouter: [Network: String] = [
        .primearc: "0x0000000000000000000000000000000000000000",
        .ethereum: "0x0000000000000000000000000000000000000000",
        .bnbChain: "0x0000000000000000000000000000000000000000",
    ]

    /// PrimeSwap Factory contract addresses
    public static let primeSwapFactory: [Network: String] = [
        .primearc: "0x0000000000000000000000000000000000000000",
    ]

    // MARK: - Bridge Contracts

    /// ETH-PBC Bridge Adapter addresses
    public static let ethPBCBridge: [Network: String] = [
        .ethereum: "0x0000000000000000000000000000000000000000",
        .primearc: "0x0000000000000000000000000000000000000000",
    ]

    /// Token Messenger (cross-chain messaging)
    public static let tokenMessenger: [Network: String] = [
        .ethereum: "0x0000000000000000000000000000000000000000",
        .bnbChain: "0x0000000000000000000000000000000000000000",
        .primearc: "0x0000000000000000000000000000000000000000",
    ]

    // MARK: - LP Token Addresses

    /// Known LP token pairs
    public struct LPToken {
        public let address: String
        public let token0: String
        public let token1: String
        public let name: String
        public let network: Network

        // LP Token pools - addresses will be updated once liquidity pools are deployed
        public static let knownPools: [LPToken] = [
            LPToken(
                address: "0x0000000000000000000000000000000000000000",
                token0: "ETR",
                token1: "USDT",
                name: "ETR-USDT LP",
                network: .primearc
            ),
            LPToken(
                address: "0x0000000000000000000000000000000000000000",
                token0: "ETR",
                token1: "EDSC",
                name: "ETR-EDSC LP",
                network: .primearc
            ),
            // wETR pairs on external chains (wETR addresses are deployed)
            LPToken(
                address: "0x0000000000000000000000000000000000000000",  // LP pair address TBD
                token0: "0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb",  // wETR on ETH
                token1: "ETH",
                name: "wETR-ETH LP",
                network: .ethereum
            ),
            LPToken(
                address: "0x0000000000000000000000000000000000000000",  // LP pair address TBD
                token0: "0xcc9b37fed77a01329502f8844620577742eb0dc6",  // wETR on BSC
                token1: "BNB",
                name: "wETR-BNB LP",
                network: .bnbChain
            ),
            LPToken(
                address: "0x0000000000000000000000000000000000000000",  // LP pair address TBD
                token0: "0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb",  // wETR on Polygon
                token1: "MATIC",
                name: "wETR-MATIC LP",
                network: .polygon
            ),
            LPToken(
                address: "0x0000000000000000000000000000000000000000",  // LP pair address TBD
                token0: "0x1A065196152C2A70e54AC06D3a3433e3D8606eF3",  // wETR on Arbitrum
                token1: "ETH",
                name: "wETR-ETH LP (Arbitrum)",
                network: .arbitrum
            ),
        ]
    }

    // MARK: - Helper Methods

    public static func getAddress(for contract: ContractType, on network: Network) -> String? {
        switch contract {
        case .wrappedETR:
            return wrappedETR[network]
        case .edsc:
            return edsc[network]
        case .masterChef:
            return masterChef[network]
        case .primeSwapRouter:
            return primeSwapRouter[network]
        case .ethPBCBridge:
            return ethPBCBridge[network]
        case .tokenMessenger:
            return tokenMessenger[network]
        }
    }

    public enum ContractType {
        case wrappedETR
        case edsc
        case masterChef
        case primeSwapRouter
        case ethPBCBridge
        case tokenMessenger
    }
}

// MARK: - ABI Definitions (Minimal for read operations)

public struct ContractABI {

    // ERC20 standard methods
    public static let erc20 = """
    [
        {"constant":true,"inputs":[{"name":"account","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint8"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"type":"function"}
    ]
    """

    // MasterChef read methods
    public static let masterChef = """
    [
        {"constant":true,"inputs":[],"name":"poolLength","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[{"name":"","type":"uint256"}],"name":"poolInfo","outputs":[{"name":"lpToken","type":"address"},{"name":"allocPoint","type":"uint256"},{"name":"lastRewardBlock","type":"uint256"},{"name":"accRewardPerShare","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[{"name":"_pid","type":"uint256"},{"name":"_user","type":"address"}],"name":"pendingReward","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[{"name":"","type":"uint256"},{"name":"","type":"address"}],"name":"userInfo","outputs":[{"name":"amount","type":"uint256"},{"name":"rewardDebt","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"rewardPerBlock","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"totalAllocPoint","outputs":[{"name":"","type":"uint256"}],"type":"function"}
    ]
    """

    // WrappedETR methods
    public static let wrappedETR = """
    [
        {"constant":true,"inputs":[{"name":"account","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"MAX_SUPPLY","outputs":[{"name":"","type":"uint256"}],"type":"function"}
    ]
    """
}
