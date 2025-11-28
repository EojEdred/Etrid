//
//  CryptoCard.swift
//  EtridWallet
//
//  Crypto-backed virtual debit card models
//

import Foundation

// MARK: - Crypto Card

struct CryptoCard: Identifiable, Codable {
    let id: String
    let cardNumber: String
    let cardholderName: String
    let expiryDate: Date
    let cvv: String
    var status: CardStatus
    let createdAt: Date
    var lastUsedAt: Date?

    // Collateral settings
    var collateralAssets: [CardCollateralAsset]
    var spendingLimitRatio: Double // e.g., 0.5 = can spend 50% of collateral value

    enum CardStatus: String, Codable {
        case active = "Active"
        case frozen = "Frozen"
        case suspended = "Suspended"
        case expired = "Expired"
    }

    // Computed properties
    var totalCollateralValue: Double {
        collateralAssets.reduce(0) { $0 + $1.currentValue }
    }

    var availableSpendingLimit: Double {
        totalCollateralValue * spendingLimitRatio
    }

    var formattedCardNumber: String {
        let chunks = cardNumber.enumerated().reduce(into: [String]()) { result, item in
            if item.offset % 4 == 0 {
                result.append("")
            }
            result[result.count - 1] += String(item.element)
        }
        return chunks.joined(separator: " ")
    }

    var maskedCardNumber: String {
        let last4 = String(cardNumber.suffix(4))
        return "•••• •••• •••• \(last4)"
    }

    var expiryFormatted: String {
        let formatter = DateFormatter()
        formatter.dateFormat = "MM/yy"
        return formatter.string(from: expiryDate)
    }
}

// MARK: - Collateral Asset

struct CardCollateralAsset: Identifiable, Codable {
    let id: String
    let symbol: String // ETR, BTC, ETH, etc.
    let name: String
    let amount: String // Wei format
    let lockedAt: Date
    var priceUSD: Double // Current price per token

    var amountDecimal: Double {
        guard let value = Double(amount) else { return 0 }
        return value / 1e18
    }

    var currentValue: Double {
        amountDecimal * priceUSD
    }

    var formattedAmount: String {
        String(format: "%.4f %@", amountDecimal, symbol)
    }

    var formattedValue: String {
        String(format: "$%.2f", currentValue)
    }
}

// MARK: - Card Transaction

struct CardTransaction: Identifiable, Codable {
    let id: String
    let cardId: String
    let merchantName: String
    let category: TransactionCategory
    let amount: Double // USD
    let currency: String
    let timestamp: Date
    let status: TransactionStatus
    let location: String?

    enum TransactionCategory: String, Codable, CaseIterable {
        case dining = "Dining"
        case shopping = "Shopping"
        case travel = "Travel"
        case entertainment = "Entertainment"
        case groceries = "Groceries"
        case gas = "Gas & Fuel"
        case utilities = "Utilities"
        case other = "Other"

        var icon: String {
            switch self {
            case .dining: return "fork.knife"
            case .shopping: return "bag.fill"
            case .travel: return "airplane"
            case .entertainment: return "music.note"
            case .groceries: return "cart.fill"
            case .gas: return "fuelpump.fill"
            case .utilities: return "bolt.fill"
            case .other: return "dollarsign.circle.fill"
            }
        }
    }

    enum TransactionStatus: String, Codable {
        case completed = "Completed"
        case pending = "Pending"
        case declined = "Declined"
        case refunded = "Refunded"
    }

    var formattedAmount: String {
        String(format: "$%.2f", amount)
    }

    var formattedDate: String {
        let formatter = DateFormatter()
        formatter.dateFormat = "MMM d, h:mm a"
        return formatter.string(from: timestamp)
    }
}

// MARK: - Card Application

struct CardApplication: Codable {
    let selectedAssets: [CollateralAsset]
    let spendingLimitRatio: Double
    let cardholderName: String
}

// MARK: - Card Stats

struct CardStats: Codable {
    let totalSpent: Double
    let transactionCount: Int
    let averageTransaction: Double
    let spendingByCategory: [CardTransaction.TransactionCategory: Double]
    let monthlySpending: [String: Double] // "2025-01" -> amount

    var formattedTotalSpent: String {
        String(format: "$%.2f", totalSpent)
    }

    var topCategory: CardTransaction.TransactionCategory? {
        spendingByCategory.max(by: { $0.value < $1.value })?.key
    }
}
