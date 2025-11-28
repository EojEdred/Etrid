//
//  Networks.swift
//  EtridWallet
//
//  Network configurations for supported blockchains
//

import Foundation

struct Networks {
    // MARK: - Mainnet Networks
    static let ethereum = Network(
        name: "Ethereum",
        chainId: 1,
        symbol: "ETH",
        rpcURL: "https://eth.llamarpc.com",
        explorerURL: "https://etherscan.io",
        isTestnet: false,
        iconName: "eth"
    )

    static let polygon = Network(
        name: "Polygon",
        chainId: 137,
        symbol: "MATIC",
        rpcURL: "https://polygon-rpc.com",
        explorerURL: "https://polygonscan.com",
        isTestnet: false,
        iconName: "polygon"
    )

    static let bsc = Network(
        name: "BNB Smart Chain",
        chainId: 56,
        symbol: "BNB",
        rpcURL: "https://bsc-dataseed1.binance.org",
        explorerURL: "https://bscscan.com",
        isTestnet: false,
        iconName: "bnb"
    )

    static let arbitrum = Network(
        name: "Arbitrum One",
        chainId: 42161,
        symbol: "ETH",
        rpcURL: "https://arb1.arbitrum.io/rpc",
        explorerURL: "https://arbiscan.io",
        isTestnet: false,
        iconName: "arbitrum"
    )

    static let optimism = Network(
        name: "Optimism",
        chainId: 10,
        symbol: "ETH",
        rpcURL: "https://mainnet.optimism.io",
        explorerURL: "https://optimistic.etherscan.io",
        isTestnet: false,
        iconName: "optimism"
    )

    static let avalanche = Network(
        name: "Avalanche C-Chain",
        chainId: 43114,
        symbol: "AVAX",
        rpcURL: "https://api.avax.network/ext/bc/C/rpc",
        explorerURL: "https://snowtrace.io",
        isTestnet: false,
        iconName: "avalanche"
    )

    static let etrid = Network(
        name: "Ëtrid Network",
        chainId: 8888,
        symbol: "ETRID",
        rpcURL: "https://rpc.etrid.org/primearc",
        explorerURL: "https://explorer.etrid.org",
        isTestnet: false,
        iconName: "etrid"
    )

    // MARK: - Testnet Networks
    static let goerli = Network(
        name: "Goerli Testnet",
        chainId: 5,
        symbol: "ETH",
        rpcURL: "https://rpc.ankr.com/eth_goerli",
        explorerURL: "https://goerli.etherscan.io",
        isTestnet: true,
        iconName: "eth"
    )

    static let sepolia = Network(
        name: "Sepolia Testnet",
        chainId: 11155111,
        symbol: "ETH",
        rpcURL: "https://rpc.sepolia.org",
        explorerURL: "https://sepolia.etherscan.io",
        isTestnet: true,
        iconName: "eth"
    )

    static let mumbai = Network(
        name: "Mumbai Testnet",
        chainId: 80001,
        symbol: "MATIC",
        rpcURL: "https://rpc-mumbai.maticvigil.com",
        explorerURL: "https://mumbai.polygonscan.com",
        isTestnet: true,
        iconName: "polygon"
    )

    static let bscTestnet = Network(
        name: "BSC Testnet",
        chainId: 97,
        symbol: "BNB",
        rpcURL: "https://data-seed-prebsc-1-s1.binance.org:8545",
        explorerURL: "https://testnet.bscscan.com",
        isTestnet: true,
        iconName: "bnb"
    )

    // MARK: - All Networks
    static let allMainnets: [Network] = [
        ethereum,
        polygon,
        bsc,
        arbitrum,
        optimism,
        avalanche,
        etrid
    ]

    static let allTestnets: [Network] = [
        goerli,
        sepolia,
        mumbai,
        bscTestnet
    ]

    static let allNetworks: [Network] = allMainnets + allTestnets

    // MARK: - Helper Functions
    static func network(for chainId: Int) -> Network? {
        allNetworks.first { $0.chainId == chainId }
    }

    static func network(for name: String) -> Network? {
        allNetworks.first { $0.name.lowercased() == name.lowercased() }
    }

    static func isTestnet(_ chainId: Int) -> Bool {
        allTestnets.contains { $0.chainId == chainId }
    }
}

// MARK: - Network Presets
extension Network {
    static var ethereum: Network { Networks.ethereum }
    static var polygon: Network { Networks.polygon }
    static var bsc: Network { Networks.bsc }
    static var arbitrum: Network { Networks.arbitrum }
    static var optimism: Network { Networks.optimism }
    static var avalanche: Network { Networks.avalanche }
    static var etrid: Network { Networks.etrid }

    var isEthereumBased: Bool {
        [1, 5, 11155111, 137, 80001, 56, 97, 42161, 10, 43114].contains(chainId)
    }

    var supportsEIP1559: Bool {
        [1, 5, 11155111, 137, 80001, 42161, 10].contains(chainId)
    }

    var blockTime: TimeInterval {
        switch chainId {
        case 1, 5, 11155111: return 12 // Ethereum
        case 137, 80001: return 2 // Polygon
        case 56, 97: return 3 // BSC
        case 42161: return 0.25 // Arbitrum
        case 10: return 2 // Optimism
        case 43114: return 2 // Avalanche
        default: return 12
        }
    }

    var defaultGasLimit: String {
        switch chainId {
        case 1, 5, 11155111: return "21000" // Ethereum
        case 137, 80001: return "21000" // Polygon
        case 56, 97: return "21000" // BSC
        case 42161: return "21000" // Arbitrum
        case 10: return "21000" // Optimism
        case 43114: return "21000" // Avalanche
        default: return "21000"
        }
    }
}
