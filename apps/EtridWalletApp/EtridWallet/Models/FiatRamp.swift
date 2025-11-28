//
//  FiatRamp.swift
//  EtridWallet
//
//  Models for fiat on/off ramp functionality
//

import Foundation

// MARK: - Fiat Provider
struct FiatProvider: Identifiable, Codable, Equatable {
    let id: UUID
    let name: String
    let logo: String
    let supportedCountries: [String]
    let supportedPaymentMethods: [PaymentMethodType]
    let fees: FeeStructure
    let limits: TransactionLimits
    let rating: Double
    let processingTime: String
    let isAvailable: Bool

    init(id: UUID = UUID(),
         name: String,
         logo: String,
         supportedCountries: [String],
         supportedPaymentMethods: [PaymentMethodType],
         fees: FeeStructure,
         limits: TransactionLimits,
         rating: Double,
         processingTime: String,
         isAvailable: Bool = true) {
        self.id = id
        self.name = name
        self.logo = logo
        self.supportedCountries = supportedCountries
        self.supportedPaymentMethods = supportedPaymentMethods
        self.fees = fees
        self.limits = limits
        self.rating = rating
        self.processingTime = processingTime
        self.isAvailable = isAvailable
    }

    var formattedRating: String {
        String(format: "%.1f", rating)
    }

    var feeSummary: String {
        if fees.percentageFee > 0 && fees.fixedFee > 0 {
            return "\(String(format: "%.2f", fees.percentageFee))% + $\(String(format: "%.2f", fees.fixedFee))"
        } else if fees.percentageFee > 0 {
            return "\(String(format: "%.2f", fees.percentageFee))%"
        } else if fees.fixedFee > 0 {
            return "$\(String(format: "%.2f", fees.fixedFee))"
        } else {
            return "Free"
        }
    }
}

// MARK: - Payment Method
struct PaymentMethod: Identifiable, Codable, Equatable {
    let id: UUID
    let type: PaymentMethodType
    var details: PaymentMethodDetails
    let isDefault: Bool
    let isVerified: Bool
    let addedAt: Date

    init(id: UUID = UUID(),
         type: PaymentMethodType,
         details: PaymentMethodDetails,
         isDefault: Bool = false,
         isVerified: Bool = false,
         addedAt: Date = Date()) {
        self.id = id
        self.type = type
        self.details = details
        self.isDefault = isDefault
        self.isVerified = isVerified
        self.addedAt = addedAt
    }

    var displayName: String {
        switch details {
        case .card(let last4, _):
            return "\(type.rawValue) •••• \(last4)"
        case .bankAccount(let accountNumber, _):
            return "\(type.rawValue) •••• \(accountNumber.suffix(4))"
        case .other(let name):
            return name
        }
    }

    var iconName: String {
        type.iconName
    }
}

// MARK: - Payment Method Type
enum PaymentMethodType: String, Codable, CaseIterable {
    case creditCard = "Credit Card"
    case debitCard = "Debit Card"
    case bankTransfer = "Bank Transfer"
    case applePay = "Apple Pay"
    case googlePay = "Google Pay"
    case sepa = "SEPA"
    case wire = "Wire Transfer"
    case ach = "ACH"

    var iconName: String {
        switch self {
        case .creditCard, .debitCard:
            return "creditcard.fill"
        case .bankTransfer, .sepa, .wire, .ach:
            return "building.columns.fill"
        case .applePay:
            return "apple.logo"
        case .googlePay:
            return "g.circle.fill"
        }
    }

    var processingTime: String {
        switch self {
        case .creditCard, .debitCard, .applePay, .googlePay:
            return "Instant - 10 minutes"
        case .bankTransfer, .ach:
            return "1-3 business days"
        case .sepa:
            return "1-2 business days"
        case .wire:
            return "1-5 business days"
        }
    }
}

// MARK: - Payment Method Details
enum PaymentMethodDetails: Codable, Equatable {
    case card(last4: String, expiry: String)
    case bankAccount(accountNumber: String, routingNumber: String)
    case other(name: String)

    enum CodingKeys: String, CodingKey {
        case type, last4, expiry, accountNumber, routingNumber, name
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        let type = try container.decode(String.self, forKey: .type)

        switch type {
        case "card":
            let last4 = try container.decode(String.self, forKey: .last4)
            let expiry = try container.decode(String.self, forKey: .expiry)
            self = .card(last4: last4, expiry: expiry)
        case "bank":
            let accountNumber = try container.decode(String.self, forKey: .accountNumber)
            let routingNumber = try container.decode(String.self, forKey: .routingNumber)
            self = .bankAccount(accountNumber: accountNumber, routingNumber: routingNumber)
        default:
            let name = try container.decode(String.self, forKey: .name)
            self = .other(name: name)
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)

        switch self {
        case .card(let last4, let expiry):
            try container.encode("card", forKey: .type)
            try container.encode(last4, forKey: .last4)
            try container.encode(expiry, forKey: .expiry)
        case .bankAccount(let accountNumber, let routingNumber):
            try container.encode("bank", forKey: .type)
            try container.encode(accountNumber, forKey: .accountNumber)
            try container.encode(routingNumber, forKey: .routingNumber)
        case .other(let name):
            try container.encode("other", forKey: .type)
            try container.encode(name, forKey: .name)
        }
    }
}

// MARK: - Fiat Transaction
struct FiatTransaction: Identifiable, Codable, Equatable {
    let id: UUID
    let type: FiatTransactionType
    let fiatAmount: Double
    let fiatCurrency: String
    let cryptoAmount: String
    let cryptoAsset: Token
    let provider: String
    let paymentMethod: PaymentMethodType
    let fee: Double
    let exchangeRate: Double
    let timestamp: Date
    var status: FiatTransactionStatus
    let orderId: String?

    init(id: UUID = UUID(),
         type: FiatTransactionType,
         fiatAmount: Double,
         fiatCurrency: String,
         cryptoAmount: String,
         cryptoAsset: Token,
         provider: String,
         paymentMethod: PaymentMethodType,
         fee: Double,
         exchangeRate: Double,
         timestamp: Date = Date(),
         status: FiatTransactionStatus = .pending,
         orderId: String? = nil) {
        self.id = id
        self.type = type
        self.fiatAmount = fiatAmount
        self.fiatCurrency = fiatCurrency
        self.cryptoAmount = cryptoAmount
        self.cryptoAsset = cryptoAsset
        self.provider = provider
        self.paymentMethod = paymentMethod
        self.fee = fee
        self.exchangeRate = exchangeRate
        self.timestamp = timestamp
        self.status = status
        self.orderId = orderId
    }

    var formattedFiatAmount: String {
        let symbol = fiatCurrency == "USD" ? "$" : fiatCurrency
        return "\(symbol)\(String(format: "%.2f", fiatAmount))"
    }

    var formattedCryptoAmount: String {
        guard let amount = Double(cryptoAmount) else { return "0" }
        return String(format: "%.6f %@", amount, cryptoAsset.symbol)
    }

    var formattedFee: String {
        let symbol = fiatCurrency == "USD" ? "$" : fiatCurrency
        return "\(symbol)\(String(format: "%.2f", fee))"
    }

    var totalAmount: Double {
        type == .buy ? fiatAmount + fee : fiatAmount - fee
    }

    var formattedTotal: String {
        let symbol = fiatCurrency == "USD" ? "$" : fiatCurrency
        return "\(symbol)\(String(format: "%.2f", totalAmount))"
    }
}

// MARK: - Fiat Transaction Type
enum FiatTransactionType: String, Codable {
    case buy = "Buy"
    case sell = "Sell"

    var actionText: String {
        switch self {
        case .buy:
            return "Buy Crypto"
        case .sell:
            return "Sell Crypto"
        }
    }
}

// MARK: - Fiat Transaction Status
enum FiatTransactionStatus: String, Codable {
    case pending = "Pending"
    case processing = "Processing"
    case completed = "Completed"
    case failed = "Failed"
    case cancelled = "Cancelled"

    var color: String {
        switch self {
        case .pending, .processing:
            return "orange"
        case .completed:
            return "green"
        case .failed, .cancelled:
            return "red"
        }
    }
}

// MARK: - Fee Structure
struct FeeStructure: Codable, Equatable {
    let percentageFee: Double
    let fixedFee: Double
    let networkFee: Double

    init(percentageFee: Double = 0,
         fixedFee: Double = 0,
         networkFee: Double = 0) {
        self.percentageFee = percentageFee
        self.fixedFee = fixedFee
        self.networkFee = networkFee
    }

    func calculateFee(for amount: Double) -> Double {
        let percentageCost = amount * (percentageFee / 100)
        return percentageCost + fixedFee + networkFee
    }

    func totalCost(for amount: Double) -> Double {
        amount + calculateFee(for: amount)
    }
}

// MARK: - Transaction Limits
struct TransactionLimits: Codable, Equatable {
    let minAmount: Double
    let maxAmount: Double
    let dailyLimit: Double
    let monthlyLimit: Double

    init(minAmount: Double,
         maxAmount: Double,
         dailyLimit: Double,
         monthlyLimit: Double) {
        self.minAmount = minAmount
        self.maxAmount = maxAmount
        self.dailyLimit = dailyLimit
        self.monthlyLimit = monthlyLimit
    }

    var formattedMin: String {
        String(format: "$%.2f", minAmount)
    }

    var formattedMax: String {
        String(format: "$%.2f", maxAmount)
    }

    var formattedDaily: String {
        String(format: "$%.2f", dailyLimit)
    }

    var formattedMonthly: String {
        String(format: "$%.2f", monthlyLimit)
    }
}

// MARK: - Quote
struct FiatQuote: Codable, Equatable {
    let provider: String
    let fiatAmount: Double
    let cryptoAmount: String
    let exchangeRate: Double
    let fee: Double
    let total: Double
    let expiresAt: Date

    init(provider: String,
         fiatAmount: Double,
         cryptoAmount: String,
         exchangeRate: Double,
         fee: Double,
         total: Double,
         expiresAt: Date) {
        self.provider = provider
        self.fiatAmount = fiatAmount
        self.cryptoAmount = cryptoAmount
        self.exchangeRate = exchangeRate
        self.fee = fee
        self.total = total
        self.expiresAt = expiresAt
    }

    var isExpired: Bool {
        Date() > expiresAt
    }

    var timeUntilExpiry: TimeInterval {
        expiresAt.timeIntervalSince(Date())
    }
}

// MARK: - Mock Data
extension FiatProvider {
    static let mockProviders: [FiatProvider] = [
        FiatProvider(
            name: "MoonPay",
            logo: "moonpay",
            supportedCountries: ["US", "UK", "EU", "CA", "AU"],
            supportedPaymentMethods: [.creditCard, .debitCard, .applePay, .googlePay, .bankTransfer],
            fees: FeeStructure(percentageFee: 3.5, fixedFee: 0, networkFee: 0),
            limits: TransactionLimits(minAmount: 20, maxAmount: 10000, dailyLimit: 10000, monthlyLimit: 50000),
            rating: 4.5,
            processingTime: "5-10 minutes"
        ),
        FiatProvider(
            name: "Transak",
            logo: "transak",
            supportedCountries: ["US", "UK", "EU", "IN", "BR"],
            supportedPaymentMethods: [.creditCard, .debitCard, .bankTransfer],
            fees: FeeStructure(percentageFee: 2.99, fixedFee: 0.5, networkFee: 0),
            limits: TransactionLimits(minAmount: 30, maxAmount: 20000, dailyLimit: 20000, monthlyLimit: 100000),
            rating: 4.3,
            processingTime: "10-15 minutes"
        ),
        FiatProvider(
            name: "Ramp",
            logo: "ramp",
            supportedCountries: ["US", "UK", "EU"],
            supportedPaymentMethods: [.creditCard, .debitCard, .applePay, .bankTransfer],
            fees: FeeStructure(percentageFee: 2.9, fixedFee: 0, networkFee: 0),
            limits: TransactionLimits(minAmount: 50, maxAmount: 15000, dailyLimit: 15000, monthlyLimit: 75000),
            rating: 4.7,
            processingTime: "5-10 minutes"
        ),
        FiatProvider(
            name: "Wyre",
            logo: "wyre",
            supportedCountries: ["US", "EU", "UK", "AU"],
            supportedPaymentMethods: [.creditCard, .debitCard, .applePay, .bankTransfer, .wire],
            fees: FeeStructure(percentageFee: 3.0, fixedFee: 0, networkFee: 0),
            limits: TransactionLimits(minAmount: 25, maxAmount: 25000, dailyLimit: 25000, monthlyLimit: 150000),
            rating: 4.2,
            processingTime: "Instant"
        )
    ]
}

extension PaymentMethod {
    static let mockMethods: [PaymentMethod] = [
        PaymentMethod(
            type: .creditCard,
            details: .card(last4: "4242", expiry: "12/25"),
            isDefault: true,
            isVerified: true
        ),
        PaymentMethod(
            type: .bankTransfer,
            details: .bankAccount(accountNumber: "123456789", routingNumber: "987654321"),
            isDefault: false,
            isVerified: true
        ),
        PaymentMethod(
            type: .applePay,
            details: .other(name: "Apple Pay"),
            isDefault: false,
            isVerified: true
        )
    ]
}
