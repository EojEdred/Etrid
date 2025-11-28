//
//  WalletModels.swift
//  EtridWallet
//
//  Core data models for wallet functionality
//

import Foundation

// MARK: - Wallet Account
struct WalletAccount: Identifiable, Codable, Equatable {
    let id: UUID
    let name: String
    let address: String
    var balance: String
    var tokens: [Token]
    let createdAt: Date
    var isHD: Bool // Hierarchical Deterministic wallet
    var derivationPath: String? // BIP44 path if HD wallet

    init(id: UUID = UUID(),
         name: String,
         address: String,
         balance: String = "0",
         tokens: [Token] = [],
         createdAt: Date = Date(),
         isHD: Bool = true,
         derivationPath: String? = nil) {
        self.id = id
        self.name = name
        self.address = address
        self.balance = balance
        self.tokens = tokens
        self.createdAt = createdAt
        self.isHD = isHD
        self.derivationPath = derivationPath
    }

    var formattedBalance: String {
        let value = Double(balance) ?? 0
        return String(format: "%.6f", value / 1e18)
    }
}

// MARK: - Token
struct Token: Identifiable, Codable, Equatable, Hashable {
    let id: UUID
    let symbol: String
    let name: String
    let contractAddress: String?
    var balance: String
    let decimals: Int
    let logoURL: String?
    let isNative: Bool

    init(id: UUID = UUID(),
         symbol: String,
         name: String,
         contractAddress: String? = nil,
         balance: String = "0",
         decimals: Int = 18,
         logoURL: String? = nil,
         isNative: Bool = false) {
        self.id = id
        self.symbol = symbol
        self.name = name
        self.contractAddress = contractAddress
        self.balance = balance
        self.decimals = decimals
        self.logoURL = logoURL
        self.isNative = isNative
    }

    var formattedBalance: String {
        let value = Double(balance) ?? 0
        let divisor = pow(10.0, Double(decimals))
        return String(format: "%.6f", value / divisor)
    }

    var displayBalance: String {
        let value = Double(balance) ?? 0
        let divisor = pow(10.0, Double(decimals))
        let amount = value / divisor

        if amount < 0.000001 {
            return "0"
        } else if amount < 1 {
            return String(format: "%.6f", amount)
        } else if amount < 1000 {
            return String(format: "%.4f", amount)
        } else {
            return String(format: "%.2f", amount)
        }
    }
}

// MARK: - Network
struct Network: Identifiable, Codable, Equatable, Hashable {
    let id: UUID
    let name: String
    let chainId: Int
    let symbol: String
    let rpcURL: String
    let explorerURL: String
    let isTestnet: Bool
    let iconName: String?

    init(id: UUID = UUID(),
         name: String,
         chainId: Int,
         symbol: String,
         rpcURL: String,
         explorerURL: String,
         isTestnet: Bool = false,
         iconName: String? = nil) {
        self.id = id
        self.name = name
        self.chainId = chainId
        self.symbol = symbol
        self.rpcURL = rpcURL
        self.explorerURL = explorerURL
        self.isTestnet = isTestnet
        self.iconName = iconName
    }

    func hash(into hasher: inout Hasher) {
        hasher.combine(chainId)
    }

    static func == (lhs: Network, rhs: Network) -> Bool {
        lhs.chainId == rhs.chainId
    }
}

// MARK: - Wallet Type
enum WalletType: String, Codable {
    case hd = "Hierarchical Deterministic"
    case imported = "Imported"
    case watchOnly = "Watch Only"
}

// MARK: - Mnemonic
struct Mnemonic: Codable {
    let words: [String]
    let language: MnemonicLanguage

    var phrase: String {
        words.joined(separator: " ")
    }

    var wordCount: Int {
        words.count
    }
}

enum MnemonicLanguage: String, Codable {
    case english = "english"
    case spanish = "spanish"
    case french = "french"
    case italian = "italian"
    case japanese = "japanese"
    case korean = "korean"
    case chinese_simplified = "chinese_simplified"
    case chinese_traditional = "chinese_traditional"
}

// MARK: - Wallet Configuration
struct WalletConfiguration: Codable {
    var biometricsEnabled: Bool
    var autoLockTimeout: Int // seconds
    var selectedCurrency: String
    var notificationsEnabled: Bool
    var analyticsEnabled: Bool

    static let `default` = WalletConfiguration(
        biometricsEnabled: true,
        autoLockTimeout: 300,
        selectedCurrency: "USD",
        notificationsEnabled: true,
        analyticsEnabled: false
    )
}

// MARK: - Price Data
struct PriceData: Codable {
    let symbol: String
    let price: Double
    let change24h: Double
    let lastUpdated: Date

    var formattedPrice: String {
        String(format: "$%.2f", price)
    }

    var formattedChange: String {
        let sign = change24h >= 0 ? "+" : ""
        return String(format: "%@%.2f%%", sign, change24h)
    }

    var isPositiveChange: Bool {
        change24h >= 0
    }
}

// MARK: - Gas Estimate
struct GasEstimate: Codable {
    let gasLimit: String
    let gasPrice: String
    let maxFeePerGas: String?
    let maxPriorityFeePerGas: String?

    var totalCost: String {
        let limit = UInt64(gasLimit) ?? 0
        let price = UInt64(gasPrice) ?? 0
        return String(limit * price)
    }

    var formattedTotalCost: String {
        let value = Double(totalCost) ?? 0
        return String(format: "%.6f", value / 1e18)
    }
}

// MARK: - Address Book Entry
struct AddressBookEntry: Identifiable, Codable {
    let id: UUID
    let name: String
    let address: String
    let network: String?
    let notes: String?
    let createdAt: Date

    init(id: UUID = UUID(),
         name: String,
         address: String,
         network: String? = nil,
         notes: String? = nil,
         createdAt: Date = Date()) {
        self.id = id
        self.name = name
        self.address = address
        self.network = network
        self.notes = notes
        self.createdAt = createdAt
    }
}
