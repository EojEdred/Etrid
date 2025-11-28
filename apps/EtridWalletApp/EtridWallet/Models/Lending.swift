//
//  Lending.swift
//  EtridWallet
//
//  Models for lending and borrowing functionality
//

import Foundation

// MARK: - Lending Pool
struct LendingPool: Identifiable, Codable, Equatable {
    let id: UUID
    let asset: Token
    let totalSupply: String
    let totalBorrow: String
    let supplyAPY: Double
    let borrowAPY: Double
    let utilizationRate: Double
    let collateralFactor: Double
    let reserveFactor: Double
    let protocolName: String

    init(id: UUID = UUID(),
         asset: Token,
         totalSupply: String,
         totalBorrow: String,
         supplyAPY: Double,
         borrowAPY: Double,
         utilizationRate: Double,
         collateralFactor: Double,
         reserveFactor: Double = 0.1,
         protocolName: String = "FlareSwap") {
        self.id = id
        self.asset = asset
        self.totalSupply = totalSupply
        self.totalBorrow = totalBorrow
        self.supplyAPY = supplyAPY
        self.borrowAPY = borrowAPY
        self.utilizationRate = utilizationRate
        self.collateralFactor = collateralFactor
        self.reserveFactor = reserveFactor
        self.protocolName = protocolName
    }

    var formattedSupplyAPY: String {
        String(format: "%.2f%%", supplyAPY)
    }

    var formattedBorrowAPY: String {
        String(format: "%.2f%%", borrowAPY)
    }

    var formattedUtilization: String {
        String(format: "%.1f%%", utilizationRate)
    }

    var availableLiquidity: String {
        guard let supply = Double(totalSupply),
              let borrow = Double(totalBorrow) else {
            return "0"
        }
        return String(supply - borrow)
    }

    var isHighUtilization: Bool {
        utilizationRate > 80
    }
}

// MARK: - Loan Position
struct LoanPosition: Identifiable, Codable, Equatable {
    let id: UUID
    let asset: Token
    let supplied: String
    let borrowed: String
    let collateralValue: Double
    let borrowLimit: Double
    let healthFactor: Double
    let interestAccrued: String
    let createdAt: Date
    var lastUpdated: Date

    init(id: UUID = UUID(),
         asset: Token,
         supplied: String,
         borrowed: String,
         collateralValue: Double,
         borrowLimit: Double,
         healthFactor: Double,
         interestAccrued: String = "0",
         createdAt: Date = Date(),
         lastUpdated: Date = Date()) {
        self.id = id
        self.asset = asset
        self.supplied = supplied
        self.borrowed = borrowed
        self.collateralValue = collateralValue
        self.borrowLimit = borrowLimit
        self.healthFactor = healthFactor
        self.interestAccrued = interestAccrued
        self.createdAt = createdAt
        self.lastUpdated = lastUpdated
    }

    var formattedSupplied: String {
        guard let value = Double(supplied) else { return "0" }
        return String(format: "%.4f %@", value, asset.symbol)
    }

    var formattedBorrowed: String {
        guard let value = Double(borrowed) else { return "0" }
        return String(format: "%.4f %@", value, asset.symbol)
    }

    var borrowLimitUsed: Double {
        guard borrowLimit > 0 else { return 0 }
        return (collateralValue - borrowLimit) / collateralValue * 100
    }

    var formattedBorrowLimit: String {
        String(format: "%.1f%%", borrowLimitUsed)
    }

    var healthStatus: HealthStatus {
        if healthFactor >= 2.0 {
            return .safe
        } else if healthFactor >= 1.3 {
            return .moderate
        } else if healthFactor >= 1.1 {
            return .risky
        } else {
            return .critical
        }
    }

    var formattedHealthFactor: String {
        String(format: "%.2f", healthFactor)
    }

    var isLiquidatable: Bool {
        healthFactor < 1.0
    }
}

// MARK: - Collateral Asset
struct CollateralAsset: Identifiable, Codable, Equatable {
    let id: UUID
    let token: Token
    var amount: String
    let value: Double
    let collateralFactor: Double
    let liquidationThreshold: Double

    init(id: UUID = UUID(),
         token: Token,
         amount: String,
         value: Double,
         collateralFactor: Double,
         liquidationThreshold: Double) {
        self.id = id
        self.token = token
        self.amount = amount
        self.value = value
        self.collateralFactor = collateralFactor
        self.liquidationThreshold = liquidationThreshold
    }

    var maxBorrowValue: Double {
        value * collateralFactor
    }

    var formattedAmount: String {
        guard let amountValue = Double(amount) else { return "0" }
        return String(format: "%.4f %@", amountValue, token.symbol)
    }

    var formattedValue: String {
        String(format: "$%.2f", value)
    }

    var formattedMaxBorrow: String {
        String(format: "$%.2f", maxBorrowValue)
    }
}

// MARK: - Interest Rate
struct InterestRate: Codable, Equatable {
    let base: Double
    let slope1: Double
    let slope2: Double
    let optimalUtilization: Double

    init(base: Double = 0,
         slope1: Double = 0.04,
         slope2: Double = 0.6,
         optimalUtilization: Double = 0.8) {
        self.base = base
        self.slope1 = slope1
        self.slope2 = slope2
        self.optimalUtilization = optimalUtilization
    }

    func calculateAPY(utilizationRate: Double) -> Double {
        if utilizationRate <= optimalUtilization {
            return base + (slope1 * utilizationRate / optimalUtilization)
        } else {
            let excessUtil = utilizationRate - optimalUtilization
            return base + slope1 + (slope2 * excessUtil / (1 - optimalUtilization))
        }
    }
}

// MARK: - Health Status
enum HealthStatus: String, Codable {
    case safe = "Safe"
    case moderate = "Moderate"
    case risky = "Risky"
    case critical = "Critical"

    var color: String {
        switch self {
        case .safe:
            return "green"
        case .moderate:
            return "yellow"
        case .risky:
            return "orange"
        case .critical:
            return "red"
        }
    }

    var description: String {
        switch self {
        case .safe:
            return "Your position is safe"
        case .moderate:
            return "Monitor your position"
        case .risky:
            return "Add collateral or repay loan"
        case .critical:
            return "Risk of liquidation!"
        }
    }
}

// MARK: - Lending Action
enum LendingAction: String, Codable {
    case supply = "Supply"
    case withdraw = "Withdraw"
    case borrow = "Borrow"
    case repay = "Repay"
}

// MARK: - Lending Transaction
struct LendingTransaction: Identifiable, Codable, Equatable {
    let id: UUID
    let action: LendingAction
    let asset: Token
    let amount: String
    let timestamp: Date
    var status: TransactionStatus
    let hash: String?

    init(id: UUID = UUID(),
         action: LendingAction,
         asset: Token,
         amount: String,
         timestamp: Date = Date(),
         status: TransactionStatus = .pending,
         hash: String? = nil) {
        self.id = id
        self.action = action
        self.asset = asset
        self.amount = amount
        self.timestamp = timestamp
        self.status = status
        self.hash = hash
    }

    var formattedAmount: String {
        guard let value = Double(amount) else { return "0" }
        return String(format: "%.4f %@", value, asset.symbol)
    }
}

// MARK: - Mock Data
extension LendingPool {
    static let mockPools: [LendingPool] = [
        LendingPool(
            asset: Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
            totalSupply: "50000000000000000000000",
            totalBorrow: "35000000000000000000000",
            supplyAPY: 3.2,
            borrowAPY: 5.8,
            utilizationRate: 70,
            collateralFactor: 0.75
        ),
        LendingPool(
            asset: Token(symbol: "USDC", name: "USD Coin", contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", decimals: 6),
            totalSupply: "100000000000000",
            totalBorrow: "85000000000000",
            supplyAPY: 5.5,
            borrowAPY: 8.2,
            utilizationRate: 85,
            collateralFactor: 0.8
        ),
        LendingPool(
            asset: Token(symbol: "DAI", name: "Dai Stablecoin", contractAddress: "0x6B175474E89094C44Da98b954EedeAC495271d0F", decimals: 18),
            totalSupply: "75000000000000000000000000",
            totalBorrow: "52500000000000000000000000",
            supplyAPY: 4.8,
            borrowAPY: 7.1,
            utilizationRate: 70,
            collateralFactor: 0.8
        ),
        LendingPool(
            asset: Token(symbol: "WBTC", name: "Wrapped Bitcoin", contractAddress: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", decimals: 8),
            totalSupply: "500000000000",
            totalBorrow: "300000000000",
            supplyAPY: 2.1,
            borrowAPY: 4.5,
            utilizationRate: 60,
            collateralFactor: 0.7
        )
    ]
}

extension LoanPosition {
    static let mock = LoanPosition(
        asset: Token(symbol: "ETH", name: "Ethereum", decimals: 18, isNative: true),
        supplied: "5.0",
        borrowed: "3000",
        collateralValue: 12500,
        borrowLimit: 9375,
        healthFactor: 2.08,
        interestAccrued: "15.50"
    )
}
