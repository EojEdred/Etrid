//
//  RPCEndpoints.swift
//  EtridWallet
//
//  Production RPC endpoint configurations with fallbacks
//

import Foundation

struct RPCEndpoints {
    // MARK: - Ethereum Mainnet
    static let ethereumEndpoints = [
        "https://eth.llamarpc.com",
        "https://rpc.ankr.com/eth",
        "https://ethereum.publicnode.com",
        "https://1rpc.io/eth",
        "https://eth.drpc.org"
    ]

    // MARK: - Polygon
    static let polygonEndpoints = [
        "https://polygon-rpc.com",
        "https://rpc.ankr.com/polygon",
        "https://polygon.llamarpc.com",
        "https://1rpc.io/matic"
    ]

    // MARK: - BSC
    static let bscEndpoints = [
        "https://bsc-dataseed1.binance.org",
        "https://bsc-dataseed2.binance.org",
        "https://bsc.publicnode.com",
        "https://1rpc.io/bnb"
    ]

    // MARK: - Arbitrum
    static let arbitrumEndpoints = [
        "https://arb1.arbitrum.io/rpc",
        "https://rpc.ankr.com/arbitrum",
        "https://arbitrum.llamarpc.com",
        "https://1rpc.io/arb"
    ]

    // MARK: - Optimism
    static let optimismEndpoints = [
        "https://mainnet.optimism.io",
        "https://rpc.ankr.com/optimism",
        "https://optimism.llamarpc.com",
        "https://1rpc.io/op"
    ]

    // MARK: - Avalanche
    static let avalancheEndpoints = [
        "https://api.avax.network/ext/bc/C/rpc",
        "https://rpc.ankr.com/avalanche",
        "https://avalanche.public-rpc.com",
        "https://1rpc.io/avax/c"
    ]

    // MARK: - Testnets
    static let sepoliaEndpoints = [
        "https://rpc.sepolia.org",
        "https://rpc2.sepolia.org",
        "https://ethereum-sepolia.publicnode.com",
        "https://1rpc.io/sepolia"
    ]

    static let goerliEndpoints = [
        "https://rpc.ankr.com/eth_goerli",
        "https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161" // Public Infura key
    ]

    static let mumbaiEndpoints = [
        "https://rpc-mumbai.maticvigil.com",
        "https://rpc.ankr.com/polygon_mumbai",
        "https://polygon-mumbai.g.alchemy.com/v2/demo"
    ]

    // MARK: - Helper Methods

    /// Gets primary endpoint for network
    static func primaryEndpoint(for chainId: Int) -> String {
        switch chainId {
        case 1: return ethereumEndpoints[0]
        case 137: return polygonEndpoints[0]
        case 56: return bscEndpoints[0]
        case 42161: return arbitrumEndpoints[0]
        case 10: return optimismEndpoints[0]
        case 43114: return avalancheEndpoints[0]
        case 11155111: return sepoliaEndpoints[0]
        case 5: return goerliEndpoints[0]
        case 80001: return mumbaiEndpoints[0]
        default: return ethereumEndpoints[0]
        }
    }

    /// Gets all fallback endpoints for network
    static func fallbackEndpoints(for chainId: Int) -> [String] {
        switch chainId {
        case 1: return Array(ethereumEndpoints.dropFirst())
        case 137: return Array(polygonEndpoints.dropFirst())
        case 56: return Array(bscEndpoints.dropFirst())
        case 42161: return Array(arbitrumEndpoints.dropFirst())
        case 10: return Array(optimismEndpoints.dropFirst())
        case 43114: return Array(avalancheEndpoints.dropFirst())
        case 11155111: return Array(sepoliaEndpoints.dropFirst())
        case 5: return Array(goerliEndpoints.dropFirst())
        case 80001: return Array(mumbaiEndpoints.dropFirst())
        default: return []
        }
    }

    /// Gets all endpoints for network
    static func allEndpoints(for chainId: Int) -> [String] {
        switch chainId {
        case 1: return ethereumEndpoints
        case 137: return polygonEndpoints
        case 56: return bscEndpoints
        case 42161: return arbitrumEndpoints
        case 10: return optimismEndpoints
        case 43114: return avalancheEndpoints
        case 11155111: return sepoliaEndpoints
        case 5: return goerliEndpoints
        case 80001: return mumbaiEndpoints
        default: return ethereumEndpoints
        }
    }
}

// MARK: - Endpoint Quality Monitoring
actor EndpointMonitor {
    private var endpointHealth: [String: EndpointHealth] = [:]

    struct EndpointHealth {
        var successCount: Int = 0
        var failureCount: Int = 0
        var lastFailure: Date?
        var averageResponseTime: TimeInterval = 0

        var isHealthy: Bool {
            if let lastFailure = lastFailure,
               Date().timeIntervalSince(lastFailure) < 300 { // 5 minutes
                return false
            }
            return successCount > failureCount
        }
    }

    func recordSuccess(for endpoint: String, responseTime: TimeInterval) {
        var health = endpointHealth[endpoint] ?? EndpointHealth()
        health.successCount += 1

        // Update rolling average
        let totalRequests = health.successCount + health.failureCount
        health.averageResponseTime = ((health.averageResponseTime * Double(totalRequests - 1)) + responseTime) / Double(totalRequests)

        endpointHealth[endpoint] = health
    }

    func recordFailure(for endpoint: String) {
        var health = endpointHealth[endpoint] ?? EndpointHealth()
        health.failureCount += 1
        health.lastFailure = Date()
        endpointHealth[endpoint] = health
    }

    func getBestEndpoint(from endpoints: [String]) -> String {
        let sortedByHealth = endpoints.sorted { endpoint1, endpoint2 in
            let health1 = endpointHealth[endpoint1]
            let health2 = endpointHealth[endpoint2]

            // Prioritize healthy endpoints
            if health1?.isHealthy != health2?.isHealthy {
                return (health1?.isHealthy ?? true) && !(health2?.isHealthy ?? true)
            }

            // Then by response time
            return (health1?.averageResponseTime ?? 1000) < (health2?.averageResponseTime ?? 1000)
        }

        return sortedByHealth.first ?? endpoints.first ?? ""
    }
}
