//
//  CryptoCard.swift
//  EtridWallet
//
//  Crypto-backed virtual card for NFC payments
//

import Foundation

// MARK: - Card Models

struct CryptoCard: Identifiable, Codable {
    let id: String
    let userId: String
    let cardNumber: String // Last 4 digits visible
    let cardholderName: String
    let expiryDate: Date
    let cvv: String // Encrypted
    let status: CardStatus
    let cardType: CardType
    let collateral: CollateralInfo
    let limits: SpendingLimits
    let createdAt: Date
    let updatedAt: Date

    // Computed properties
    var isActive: Bool {
        status == .active
    }

    var isFrozen: Bool {
        status == .frozen
    }

    var maskedCardNumber: String {
        "**** **** **** \(cardNumber)"
    }

    var expiryString: String {
        let formatter = DateFormatter()
        formatter.dateFormat = "MM/yy"
        return formatter.string(from: expiryDate)
    }

    var availableBalance: Double {
        collateral.availableAmount
    }
}

enum CardStatus: String, Codable {
    case active = "active"
    case frozen = "frozen"
    case blocked = "blocked"
    case expired = "expired"
    case pending = "pending"
}

enum CardType: String, Codable {
    case virtual = "virtual"
    case physical = "physical"
}

struct CollateralInfo: Codable {
    let cryptoAsset: String // e.g., "ETH", "BTC", "USDC"
    let collateralAmount: Double // Amount locked as collateral
    let collateralValueUSD: Double // USD value of collateral
    let availableAmount: Double // Available spending limit
    let ltv: Double // Loan-to-value ratio (e.g., 0.5 = 50%)
    let liquidationThreshold: Double // Price threshold for liquidation
}

struct SpendingLimits: Codable {
    let dailyLimit: Double
    let monthlyLimit: Double
    let perTransactionLimit: Double
    let dailySpent: Double
    let monthlySpent: Double

    var dailyRemaining: Double {
        max(0, dailyLimit - dailySpent)
    }

    var monthlyRemaining: Double {
        max(0, monthlyLimit - monthlySpent)
    }
}

// MARK: - Transaction Models

struct CardTransaction: Identifiable, Codable {
    let id: String
    let cardId: String
    let merchantName: String
    let merchantCategory: String
    let amount: Double
    let currency: String
    let status: CardTransactionStatus
    let timestamp: Date
    let location: TransactionLocation?
    let receiptURL: String?
    let description: String?
    let nfcTransaction: Bool

    var amountString: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = currency
        return formatter.string(from: NSNumber(value: amount)) ?? "\(amount) \(currency)"
    }

    var icon: String {
        switch merchantCategory.lowercased() {
        case "restaurant", "food": return "fork.knife"
        case "grocery": return "cart"
        case "gas", "fuel": return "fuelpump"
        case "shopping": return "bag"
        case "entertainment": return "tv"
        case "travel": return "airplane"
        case "health": return "cross.case"
        default: return "creditcard"
        }
    }
}

enum CardTransactionStatus: String, Codable {
    case pending = "pending"
    case completed = "completed"
    case declined = "declined"
    case refunded = "refunded"
}

struct TransactionLocation: Codable {
    let latitude: Double
    let longitude: Double
    let address: String?
    let city: String?
    let country: String?
}

// MARK: - Card Control Models

struct CardControls: Codable {
    var onlineTransactionsEnabled: Bool
    var contactlessEnabled: Bool
    var atmWithdrawalsEnabled: Bool
    var foreignTransactionsEnabled: Bool
    var allowedCategories: Set<String>
    var blockedCategories: Set<String>
}

struct PINChangeRequest: Codable {
    let cardId: String
    let currentPIN: String
    let newPIN: String
}

struct CardActivationRequest: Codable {
    let cardId: String
    let activationCode: String
}

// MARK: - NFC Payment Models

struct NFCPaymentRequest: Codable {
    let cardId: String
    let merchantId: String
    let merchantName: String
    let amount: Double
    let currency: String
    let merchantCategory: String
    let location: TransactionLocation?
    let timestamp: Date
    let nonce: String // For security
}

struct NFCPaymentResponse: Codable {
    let success: Bool
    let transactionId: String?
    let message: String
    let newBalance: Double?
    let declineReason: String?
}

enum PaymentDeclineReason: String, Codable {
    case insufficientFunds = "insufficient_funds"
    case cardFrozen = "card_frozen"
    case cardBlocked = "card_blocked"
    case limitExceeded = "limit_exceeded"
    case invalidMerchant = "invalid_merchant"
    case securityBlock = "security_block"
    case networkError = "network_error"
}

// MARK: - Apple Wallet Integration

struct AppleWalletPass: Codable {
    let passTypeIdentifier: String
    let serialNumber: String
    let cardId: String
    let passURL: String?
    let isProvisioned: Bool
}
