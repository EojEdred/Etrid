//
//  Swap.swift
//  EtridWallet
//
//  Models for DEX swap functionality
//

import Foundation

// MARK: - Swap Pair
struct SwapPair: Identifiable, Codable, Equatable {
    let id: UUID
    let tokenIn: Token
    let tokenOut: Token
    var amountIn: String
    var amountOut: String
    var priceImpact: Double
    var minimumReceived: String
    var route: [String]

    init(id: UUID = UUID(),
         tokenIn: Token,
         tokenOut: Token,
         amountIn: String = "0",
         amountOut: String = "0",
         priceImpact: Double = 0,
         minimumReceived: String = "0",
         route: [String] = []) {
        self.id = id
        self.tokenIn = tokenIn
        self.tokenOut = tokenOut
        self.amountIn = amountIn
        self.amountOut = amountOut
        self.priceImpact = priceImpact
        self.minimumReceived = minimumReceived
        self.route = route
    }

    var exchangeRate: String {
        guard let inAmount = Double(amountIn), inAmount > 0,
              let outAmount = Double(amountOut) else {
            return "0"
        }
        let rate = outAmount / inAmount
        return String(format: "%.6f", rate)
    }

    var priceImpactFormatted: String {
        String(format: "%.2f%%", priceImpact)
    }

    var isPriceImpactHigh: Bool {
        priceImpact > 5.0
    }
}

// MARK: - Swap Transaction
struct SwapTransaction: Identifiable, Codable, Equatable {
    let id: UUID
    let pair: SwapPair
    let timestamp: Date
    var status: TransactionStatus
    let hash: String?
    let gasUsed: String?
    let slippageTolerance: Double

    init(id: UUID = UUID(),
         pair: SwapPair,
         timestamp: Date = Date(),
         status: TransactionStatus = .pending,
         hash: String? = nil,
         gasUsed: String? = nil,
         slippageTolerance: Double = 0.5) {
        self.id = id
        self.pair = pair
        self.timestamp = timestamp
        self.status = status
        self.hash = hash
        self.gasUsed = gasUsed
        self.slippageTolerance = slippageTolerance
    }

    var statusText: String {
        switch status {
        case .pending:
            return "Pending"
        case .confirmed:
            return "Completed"
        case .failed:
            return "Failed"
        case .dropped:
            return "Dropped"
        case .replaced:
            return "Replaced"
        }
    }
}

// MARK: - Liquidity Pool
struct LiquidityPool: Identifiable, Codable, Equatable {
    let id: UUID
    let token0: Token
    let token1: Token
    let reserve0: String
    let reserve1: String
    let totalLiquidity: String
    let apy: Double
    let volume24h: Double
    let fee: Double

    init(id: UUID = UUID(),
         token0: Token,
         token1: Token,
         reserve0: String,
         reserve1: String,
         totalLiquidity: String,
         apy: Double = 0,
         volume24h: Double = 0,
         fee: Double = 0.3) {
        self.id = id
        self.token0 = token0
        self.token1 = token1
        self.reserve0 = reserve0
        self.reserve1 = reserve1
        self.totalLiquidity = totalLiquidity
        self.apy = apy
        self.volume24h = volume24h
        self.fee = fee
    }

    var pairName: String {
        "\(token0.symbol)/\(token1.symbol)"
    }

    var formattedAPY: String {
        String(format: "%.2f%%", apy)
    }

    var formattedVolume: String {
        if volume24h >= 1_000_000 {
            return String(format: "$%.2fM", volume24h / 1_000_000)
        } else if volume24h >= 1_000 {
            return String(format: "$%.2fK", volume24h / 1_000)
        } else {
            return String(format: "$%.2f", volume24h)
        }
    }
}

// MARK: - Slippage Settings
struct SlippageSettings: Codable, Equatable {
    var tolerance: Double
    var isCustom: Bool

    init(tolerance: Double = 0.5, isCustom: Bool = false) {
        self.tolerance = tolerance
        self.isCustom = isCustom
    }

    var formattedTolerance: String {
        String(format: "%.1f%%", tolerance)
    }

    static let presets: [Double] = [0.5, 1.0, 3.0]

    static let `default` = SlippageSettings(tolerance: 0.5, isCustom: false)
}

// MARK: - Swap Quote
struct SwapQuote: Codable, Equatable {
    let amountIn: String
    let amountOut: String
    let priceImpact: Double
    let fee: String
    let gasEstimate: String
    let route: [String]
    let timestamp: Date

    init(amountIn: String,
         amountOut: String,
         priceImpact: Double,
         fee: String,
         gasEstimate: String,
         route: [String],
         timestamp: Date = Date()) {
        self.amountIn = amountIn
        self.amountOut = amountOut
        self.priceImpact = priceImpact
        self.fee = fee
        self.gasEstimate = gasEstimate
        self.route = route
        self.timestamp = timestamp
    }

    var isExpired: Bool {
        Date().timeIntervalSince(timestamp) > 60 // 60 seconds
    }

    var totalCost: String {
        guard let feeValue = Double(fee),
              let gasValue = Double(gasEstimate) else {
            return "0"
        }
        return String(feeValue + gasValue)
    }
}

// MARK: - Mock Data
extension SwapPair {
    static let mock = SwapPair(
        tokenIn: Token(
            symbol: "ETH",
            name: "Ethereum",
            balance: "1000000000000000000",
            decimals: 18,
            isNative: true
        ),
        tokenOut: Token(
            symbol: "USDC",
            name: "USD Coin",
            contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
            balance: "0",
            decimals: 6
        ),
        amountIn: "1.0",
        amountOut: "2500.00",
        priceImpact: 0.15,
        minimumReceived: "2475.00",
        route: ["ETH", "USDC"]
    )
}

extension LiquidityPool {
    static let mockPools: [LiquidityPool] = [
        LiquidityPool(
            token0: Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
            token1: Token(symbol: "USDC", name: "USD Coin", contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", decimals: 6),
            reserve0: "50000000000000000000000",
            reserve1: "125000000000000",
            totalLiquidity: "125000000",
            apy: 12.5,
            volume24h: 5_280_000,
            fee: 0.3
        ),
        LiquidityPool(
            token0: Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
            token1: Token(symbol: "DAI", name: "Dai Stablecoin", contractAddress: "0x6B175474E89094C44Da98b954EedeAC495271d0F", decimals: 18),
            reserve0: "30000000000000000000000",
            reserve1: "75000000000000000000000000",
            totalLiquidity: "75000000",
            apy: 8.7,
            volume24h: 3_120_000,
            fee: 0.3
        )
    ]
}
