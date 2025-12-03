import Foundation
import CoreLocation

// MARK: - DeFi Hub Feature Types

/// Main DeFi Hub features available in the wallet
public enum DeFiHubFeature: String, CaseIterable, Identifiable {
    case virtualCard = "Virtual Card"
    case atmLocator = "ATM Locator"
    case cryptoLoans = "Crypto Loans"
    case yieldFarming = "Yield Farming"

    public var id: String { rawValue }

    public var icon: String {
        switch self {
        case .virtualCard: return "creditcard.fill"
        case .atmLocator: return "mappin.and.ellipse"
        case .cryptoLoans: return "dollarsign.arrow.circlepath"
        case .yieldFarming: return "leaf.fill"
        }
    }

    public var isAvailable: Bool {
        switch self {
        case .atmLocator: return true
        case .virtualCard, .cryptoLoans, .yieldFarming: return false
        }
    }

    public var comingSoonDate: String? {
        switch self {
        case .virtualCard: return "Q2 2026"
        case .cryptoLoans: return "Q3 2026"
        case .yieldFarming: return "Q3 2026"
        case .atmLocator: return nil
        }
    }

    public var description: String {
        switch self {
        case .virtualCard:
            return "Spend your crypto anywhere with a virtual Visa/Mastercard backed by your holdings"
        case .atmLocator:
            return "Find nearby crypto ATMs and withdraw cash using your ËTRID wallet"
        case .cryptoLoans:
            return "Borrow against your crypto without selling - maintain exposure while accessing liquidity"
        case .yieldFarming:
            return "Earn passive income by providing liquidity to ËTRID DeFi protocols"
        }
    }
}

// MARK: - Virtual Card Models

/// Virtual card waitlist registration
public struct VirtualCardWaitlistEntry: Codable, Identifiable {
    public let id: UUID
    public let email: String
    public let walletAddress: String
    public let preferredCardType: VirtualCardType
    public let estimatedMonthlySpend: SpendingTier
    public let registeredAt: Date
    public var position: Int?

    public init(
        id: UUID = UUID(),
        email: String,
        walletAddress: String,
        preferredCardType: VirtualCardType,
        estimatedMonthlySpend: SpendingTier,
        registeredAt: Date = Date(),
        position: Int? = nil
    ) {
        self.id = id
        self.email = email
        self.walletAddress = walletAddress
        self.preferredCardType = preferredCardType
        self.estimatedMonthlySpend = estimatedMonthlySpend
        self.registeredAt = registeredAt
        self.position = position
    }
}

public enum VirtualCardType: String, Codable, CaseIterable {
    case standard = "Standard"
    case premium = "Premium"
    case elite = "Elite"

    public var collateralRatio: Double {
        switch self {
        case .standard: return 1.5 // 150% collateral
        case .premium: return 1.25 // 125% collateral
        case .elite: return 1.1 // 110% collateral
        }
    }

    public var monthlyLimit: Decimal {
        switch self {
        case .standard: return 5000
        case .premium: return 25000
        case .elite: return 100000
        }
    }

    public var cashbackRate: Double {
        switch self {
        case .standard: return 0.01 // 1%
        case .premium: return 0.02 // 2%
        case .elite: return 0.03 // 3%
        }
    }

    public var minimumStake: Decimal {
        switch self {
        case .standard: return 0
        case .premium: return 10000
        case .elite: return 100000
        }
    }

    public var color: String {
        switch self {
        case .standard: return "gray"
        case .premium: return "gold"
        case .elite: return "black"
        }
    }
}

public enum SpendingTier: String, Codable, CaseIterable {
    case low = "Under $500"
    case medium = "$500 - $2,000"
    case high = "$2,000 - $10,000"
    case veryHigh = "Over $10,000"
}

/// Planned virtual card features for marketing display
public struct VirtualCardFeature: Identifiable {
    public let id = UUID()
    public let icon: String
    public let title: String
    public let description: String

    public static let plannedFeatures: [VirtualCardFeature] = [
        VirtualCardFeature(
            icon: "creditcard.and.123",
            title: "Crypto-Collateralized",
            description: "Spend without selling - your crypto backs your purchases"
        ),
        VirtualCardFeature(
            icon: "arrow.counterclockwise",
            title: "Auto-Repay",
            description: "Automatically repay from staking rewards or yield"
        ),
        VirtualCardFeature(
            icon: "percent",
            title: "Cashback Rewards",
            description: "Earn up to 3% back in ETR on all purchases"
        ),
        VirtualCardFeature(
            icon: "globe",
            title: "Global Acceptance",
            description: "Use anywhere Visa/Mastercard is accepted worldwide"
        ),
        VirtualCardFeature(
            icon: "lock.shield",
            title: "Self-Custody",
            description: "Your keys, your crypto - we never hold your assets"
        ),
        VirtualCardFeature(
            icon: "bolt",
            title: "Instant Issuance",
            description: "Get your virtual card number instantly in-app"
        )
    ]
}

// MARK: - ATM Models

/// Crypto ATM location data
public struct CryptoATM: Identifiable, Codable, Equatable {
    public let id: String
    public let name: String
    public let operator_name: String
    public let address: ATMAddress
    public let location: ATMLocation
    public let supportedCryptos: [ATMCrypto]
    public let operationType: ATMOperationType
    public let fees: ATMFees
    public let limits: ATMLimits
    public let operatingHours: OperatingHours
    public let rating: Double?
    public let reviewCount: Int
    public let verified: Bool
    public let lastUpdated: Date

    public init(
        id: String,
        name: String,
        operator_name: String,
        address: ATMAddress,
        location: ATMLocation,
        supportedCryptos: [ATMCrypto],
        operationType: ATMOperationType,
        fees: ATMFees,
        limits: ATMLimits,
        operatingHours: OperatingHours,
        rating: Double? = nil,
        reviewCount: Int = 0,
        verified: Bool = false,
        lastUpdated: Date = Date()
    ) {
        self.id = id
        self.name = name
        self.operator_name = operator_name
        self.address = address
        self.location = location
        self.supportedCryptos = supportedCryptos
        self.operationType = operationType
        self.fees = fees
        self.limits = limits
        self.operatingHours = operatingHours
        self.rating = rating
        self.reviewCount = reviewCount
        self.verified = verified
        self.lastUpdated = lastUpdated
    }

    /// Calculate distance from user's current location
    public func distance(from userLocation: CLLocation) -> CLLocationDistance {
        let atmLocation = CLLocation(latitude: location.latitude, longitude: location.longitude)
        return userLocation.distance(from: atmLocation)
    }

    /// Formatted distance string
    public func formattedDistance(from userLocation: CLLocation) -> String {
        let meters = distance(from: userLocation)
        if meters < 1000 {
            return "\(Int(meters)) m"
        } else {
            let km = meters / 1000
            return String(format: "%.1f km", km)
        }
    }

    /// Check if ATM supports a specific crypto
    public func supports(crypto: ATMCrypto) -> Bool {
        supportedCryptos.contains(crypto)
    }
}

public struct ATMAddress: Codable, Equatable {
    public let street: String
    public let city: String
    public let state: String
    public let postalCode: String
    public let country: String

    public init(street: String, city: String, state: String, postalCode: String, country: String) {
        self.street = street
        self.city = city
        self.state = state
        self.postalCode = postalCode
        self.country = country
    }

    public var fullAddress: String {
        "\(street), \(city), \(state) \(postalCode)"
    }

    public var shortAddress: String {
        "\(street), \(city)"
    }
}

public struct ATMLocation: Codable, Equatable {
    public let latitude: Double
    public let longitude: Double

    public init(latitude: Double, longitude: Double) {
        self.latitude = latitude
        self.longitude = longitude
    }

    public var coordinate: CLLocationCoordinate2D {
        CLLocationCoordinate2D(latitude: latitude, longitude: longitude)
    }
}

public enum ATMCrypto: String, Codable, CaseIterable, Equatable {
    case bitcoin = "BTC"
    case ethereum = "ETH"
    case litecoin = "LTC"
    case dogecoin = "DOGE"
    case usdt = "USDT"
    case etrid = "ETR"

    public var name: String {
        switch self {
        case .bitcoin: return "Bitcoin"
        case .ethereum: return "Ethereum"
        case .litecoin: return "Litecoin"
        case .dogecoin: return "Dogecoin"
        case .usdt: return "Tether USD"
        case .etrid: return "ËTRID"
        }
    }

    public var icon: String {
        switch self {
        case .bitcoin: return "bitcoinsign.circle.fill"
        case .ethereum: return "diamond.fill"
        case .litecoin: return "l.circle.fill"
        case .dogecoin: return "d.circle.fill"
        case .usdt: return "dollarsign.circle.fill"
        case .etrid: return "e.circle.fill"
        }
    }
}

public enum ATMOperationType: String, Codable, Equatable {
    case buyOnly = "Buy Only"
    case sellOnly = "Sell Only"
    case buyAndSell = "Buy & Sell"

    public var canWithdrawCash: Bool {
        self == .sellOnly || self == .buyAndSell
    }

    public var canBuyCrypto: Bool {
        self == .buyOnly || self == .buyAndSell
    }
}

public struct ATMFees: Codable, Equatable {
    public let buyPercentage: Double
    public let sellPercentage: Double
    public let flatFee: Decimal?

    public init(buyPercentage: Double, sellPercentage: Double, flatFee: Decimal? = nil) {
        self.buyPercentage = buyPercentage
        self.sellPercentage = sellPercentage
        self.flatFee = flatFee
    }

    public var formattedBuyFee: String {
        String(format: "%.1f%%", buyPercentage)
    }

    public var formattedSellFee: String {
        String(format: "%.1f%%", sellPercentage)
    }
}

public struct ATMLimits: Codable, Equatable {
    public let minBuy: Decimal
    public let maxBuy: Decimal
    public let minSell: Decimal
    public let maxSell: Decimal
    public let dailyLimit: Decimal?

    public init(minBuy: Decimal, maxBuy: Decimal, minSell: Decimal, maxSell: Decimal, dailyLimit: Decimal? = nil) {
        self.minBuy = minBuy
        self.maxBuy = maxBuy
        self.minSell = minSell
        self.maxSell = maxSell
        self.dailyLimit = dailyLimit
    }
}

public struct OperatingHours: Codable, Equatable {
    public let is24Hours: Bool
    public let schedule: [DaySchedule]?

    public init(is24Hours: Bool = true, schedule: [DaySchedule]? = nil) {
        self.is24Hours = is24Hours
        self.schedule = schedule
    }

    public var displayText: String {
        is24Hours ? "Open 24/7" : "See hours"
    }
}

public struct DaySchedule: Codable, Equatable {
    public let day: String
    public let open: String
    public let close: String
    public let isClosed: Bool

    public init(day: String, open: String, close: String, isClosed: Bool = false) {
        self.day = day
        self.open = open
        self.close = close
        self.isClosed = isClosed
    }
}

// MARK: - ATM Withdrawal Models

/// ATM withdrawal request
public struct ATMWithdrawalRequest: Identifiable, Codable {
    public let id: UUID
    public let atmId: String
    public let walletAddress: String
    public let cryptoAmount: Decimal
    public let cryptoCurrency: ATMCrypto
    public let fiatAmount: Decimal
    public let fiatCurrency: String
    public let fee: Decimal
    public let status: ATMWithdrawalStatus
    public let qrCodeData: String?
    public let expiresAt: Date
    public let createdAt: Date
    public var completedAt: Date?

    public init(
        id: UUID = UUID(),
        atmId: String,
        walletAddress: String,
        cryptoAmount: Decimal,
        cryptoCurrency: ATMCrypto,
        fiatAmount: Decimal,
        fiatCurrency: String = "USD",
        fee: Decimal,
        status: ATMWithdrawalStatus = .pending,
        qrCodeData: String? = nil,
        expiresAt: Date,
        createdAt: Date = Date(),
        completedAt: Date? = nil
    ) {
        self.id = id
        self.atmId = atmId
        self.walletAddress = walletAddress
        self.cryptoAmount = cryptoAmount
        self.cryptoCurrency = cryptoCurrency
        self.fiatAmount = fiatAmount
        self.fiatCurrency = fiatCurrency
        self.fee = fee
        self.status = status
        self.qrCodeData = qrCodeData
        self.expiresAt = expiresAt
        self.createdAt = createdAt
        self.completedAt = completedAt
    }

    public var isExpired: Bool {
        Date() > expiresAt
    }

    public var timeRemaining: TimeInterval {
        expiresAt.timeIntervalSince(Date())
    }

    public var formattedTimeRemaining: String {
        let remaining = max(0, timeRemaining)
        let minutes = Int(remaining) / 60
        let seconds = Int(remaining) % 60
        return String(format: "%02d:%02d", minutes, seconds)
    }
}

public enum ATMWithdrawalStatus: String, Codable {
    case pending = "Pending"
    case cryptoSent = "Crypto Sent"
    case confirming = "Confirming"
    case readyForPickup = "Ready for Pickup"
    case completed = "Completed"
    case expired = "Expired"
    case failed = "Failed"
    case cancelled = "Cancelled"

    public var icon: String {
        switch self {
        case .pending: return "clock"
        case .cryptoSent: return "arrow.up.circle"
        case .confirming: return "hourglass"
        case .readyForPickup: return "checkmark.circle"
        case .completed: return "checkmark.seal.fill"
        case .expired: return "clock.badge.xmark"
        case .failed: return "xmark.circle"
        case .cancelled: return "xmark.circle"
        }
    }

    public var color: String {
        switch self {
        case .pending, .cryptoSent, .confirming: return "orange"
        case .readyForPickup: return "blue"
        case .completed: return "green"
        case .expired, .failed, .cancelled: return "red"
        }
    }
}

// MARK: - ATM Search & Filter

public struct ATMSearchFilter {
    public var operationType: ATMOperationType?
    public var supportedCryptos: Set<ATMCrypto>
    public var maxDistance: Double? // in meters
    public var minRating: Double?
    public var verifiedOnly: Bool
    public var openNow: Bool

    public init(
        operationType: ATMOperationType? = nil,
        supportedCryptos: Set<ATMCrypto> = [],
        maxDistance: Double? = nil,
        minRating: Double? = nil,
        verifiedOnly: Bool = false,
        openNow: Bool = false
    ) {
        self.operationType = operationType
        self.supportedCryptos = supportedCryptos
        self.maxDistance = maxDistance
        self.minRating = minRating
        self.verifiedOnly = verifiedOnly
        self.openNow = openNow
    }

    public static let `default` = ATMSearchFilter()

    public static let withdrawalOnly = ATMSearchFilter(
        operationType: .sellOnly,
        supportedCryptos: [.bitcoin, .ethereum, .etrid]
    )
}

// MARK: - Sample/Mock Data for Development

extension CryptoATM {
    /// Sample ATMs for development and UI testing
    public static let sampleATMs: [CryptoATM] = [
        CryptoATM(
            id: "atm-001",
            name: "Bitcoin Depot - Chevron Gas Station",
            operator_name: "Bitcoin Depot",
            address: ATMAddress(
                street: "1234 Main Street",
                city: "Los Angeles",
                state: "CA",
                postalCode: "90001",
                country: "USA"
            ),
            location: ATMLocation(latitude: 34.0522, longitude: -118.2437),
            supportedCryptos: [.bitcoin, .ethereum, .litecoin, .etrid],
            operationType: .buyAndSell,
            fees: ATMFees(buyPercentage: 6.5, sellPercentage: 4.5),
            limits: ATMLimits(minBuy: 20, maxBuy: 7500, minSell: 20, maxSell: 5000),
            operatingHours: OperatingHours(is24Hours: true),
            rating: 4.2,
            reviewCount: 156,
            verified: true
        ),
        CryptoATM(
            id: "atm-002",
            name: "CoinFlip - 7-Eleven",
            operator_name: "CoinFlip",
            address: ATMAddress(
                street: "5678 Hollywood Blvd",
                city: "Los Angeles",
                state: "CA",
                postalCode: "90028",
                country: "USA"
            ),
            location: ATMLocation(latitude: 34.1016, longitude: -118.3267),
            supportedCryptos: [.bitcoin, .ethereum, .dogecoin, .usdt],
            operationType: .buyAndSell,
            fees: ATMFees(buyPercentage: 5.5, sellPercentage: 3.5),
            limits: ATMLimits(minBuy: 10, maxBuy: 10000, minSell: 20, maxSell: 8000),
            operatingHours: OperatingHours(is24Hours: true),
            rating: 4.5,
            reviewCount: 234,
            verified: true
        ),
        CryptoATM(
            id: "atm-003",
            name: "GENERAL BYTES - Crypto Corner",
            operator_name: "GENERAL BYTES",
            address: ATMAddress(
                street: "999 Sunset Blvd",
                city: "Los Angeles",
                state: "CA",
                postalCode: "90069",
                country: "USA"
            ),
            location: ATMLocation(latitude: 34.0901, longitude: -118.3868),
            supportedCryptos: [.bitcoin, .litecoin, .etrid],
            operationType: .sellOnly,
            fees: ATMFees(buyPercentage: 0, sellPercentage: 3.0),
            limits: ATMLimits(minBuy: 0, maxBuy: 0, minSell: 50, maxSell: 3000),
            operatingHours: OperatingHours(is24Hours: false, schedule: [
                DaySchedule(day: "Mon-Fri", open: "9:00 AM", close: "9:00 PM", isClosed: false),
                DaySchedule(day: "Sat-Sun", open: "10:00 AM", close: "6:00 PM", isClosed: false)
            ]),
            rating: 4.8,
            reviewCount: 89,
            verified: true
        ),
        CryptoATM(
            id: "atm-004",
            name: "Coinme - Coinstar Kiosk",
            operator_name: "Coinme",
            address: ATMAddress(
                street: "456 Wilshire Blvd",
                city: "Santa Monica",
                state: "CA",
                postalCode: "90401",
                country: "USA"
            ),
            location: ATMLocation(latitude: 34.0195, longitude: -118.4912),
            supportedCryptos: [.bitcoin, .ethereum],
            operationType: .buyOnly,
            fees: ATMFees(buyPercentage: 4.0, sellPercentage: 0),
            limits: ATMLimits(minBuy: 20, maxBuy: 2500, minSell: 0, maxSell: 0),
            operatingHours: OperatingHours(is24Hours: true),
            rating: 3.9,
            reviewCount: 67,
            verified: false
        ),
        CryptoATM(
            id: "atm-005",
            name: "RockitCoin - Shell Station",
            operator_name: "RockitCoin",
            address: ATMAddress(
                street: "789 Venice Blvd",
                city: "Los Angeles",
                state: "CA",
                postalCode: "90034",
                country: "USA"
            ),
            location: ATMLocation(latitude: 34.0138, longitude: -118.4051),
            supportedCryptos: [.bitcoin, .ethereum, .litecoin, .dogecoin, .etrid],
            operationType: .buyAndSell,
            fees: ATMFees(buyPercentage: 5.0, sellPercentage: 4.0, flatFee: 1.99),
            limits: ATMLimits(minBuy: 20, maxBuy: 15000, minSell: 50, maxSell: 10000, dailyLimit: 25000),
            operatingHours: OperatingHours(is24Hours: true),
            rating: 4.6,
            reviewCount: 312,
            verified: true
        )
    ]
}
