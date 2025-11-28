//
//  DApp.swift
//  EtridWallet
//
//  Models for dApp browser and Web3 interactions
//

import Foundation

// MARK: - DApp Models

struct DApp: Identifiable, Codable, Hashable {
    let id: String
    let name: String
    let description: String
    let url: String
    let iconUrl: String?
    let category: DAppCategory
    var isFavorite: Bool
    var lastVisited: Date?

    enum DAppCategory: String, Codable, CaseIterable {
        case defi = "DeFi"
        case nft = "NFT"
        case gaming = "Gaming"
        case dao = "DAO"
        case social = "Social"
        case marketplace = "Marketplace"
        case bridge = "Bridge"
        case other = "Other"
    }
}

struct Bookmark: Identifiable, Codable {
    let id: String
    let name: String
    let url: String
    let iconUrl: String?
    let createdAt: Date
}

struct ConnectedDApp: Identifiable, Codable {
    let id: String
    let dAppUrl: String
    let dAppName: String
    let dAppIcon: String?
    var permissions: [DAppPermission]
    let connectedAt: Date
    var lastUsed: Date
}

struct DAppPermission: Codable, Hashable {
    let type: PermissionType
    let grantedAt: Date

    enum PermissionType: String, Codable {
        case accessAddress = "access_address"
        case signTransaction = "sign_transaction"
        case signMessage = "sign_message"
        case signTypedData = "sign_typed_data"
        case switchChain = "switch_chain"
    }
}

// MARK: - Web3 Request/Response

struct Web3Request: Identifiable, Equatable {
    let id: String
    let method: String
    let params: [Any]
    let dAppUrl: String
    let dAppName: String
    let requestedAt: Date

    static func == (lhs: Web3Request, rhs: Web3Request) -> Bool {
        lhs.id == rhs.id &&
        lhs.method == rhs.method &&
        lhs.dAppUrl == rhs.dAppUrl &&
        lhs.dAppName == rhs.dAppName &&
        lhs.requestedAt == rhs.requestedAt
    }

    var methodDescription: String {
        switch method {
        case "eth_requestAccounts": return "Request Account Access"
        case "eth_sendTransaction": return "Send Transaction"
        case "eth_signTransaction": return "Sign Transaction"
        case "personal_sign": return "Sign Message"
        case "eth_signTypedData", "eth_signTypedData_v4": return "Sign Typed Data"
        case "wallet_switchEthereumChain": return "Switch Network"
        default: return method
        }
    }
}

struct Web3Response: Codable {
    let requestId: String
    let result: String?
    let error: String?
}

// MARK: - Transaction Request

struct DAppTransactionRequest: Codable {
    let from: String
    let to: String?
    let value: String?
    let data: String?
    let gas: String?
    let gasPrice: String?
    let nonce: String?
    let chainId: String?

    var estimatedGasFee: String {
        guard let gas = gas, let gasPrice = gasPrice,
              let gasInt = Int(gas.replacingOccurrences(of: "0x", with: ""), radix: 16),
              let gasPriceInt = Int(gasPrice.replacingOccurrences(of: "0x", with: ""), radix: 16) else {
            return "0"
        }

        let totalGas = Double(gasInt * gasPriceInt) / 1_000_000_000_000_000_000
        return String(format: "%.6f", totalGas)
    }
}

struct TransactionSimulation: Codable {
    let success: Bool
    let gasEstimate: String
    let balanceChanges: [BalanceChange]
    let warnings: [String]

    struct BalanceChange: Codable {
        let asset: String
        let amount: String
        let direction: ChangeDirection

        enum ChangeDirection: String, Codable {
            case incoming = "incoming"
            case outgoing = "outgoing"
        }
    }
}

// MARK: - Phishing Detection

struct PhishingCheck {
    let url: String
    let isSafe: Bool
    let warnings: [SecurityWarning]
    let risk: RiskLevel

    enum RiskLevel: String {
        case safe = "Safe"
        case low = "Low Risk"
        case medium = "Medium Risk"
        case high = "High Risk"
        case critical = "Critical"
    }

    struct SecurityWarning {
        let title: String
        let description: String
        let severity: WarningLevel

        enum WarningLevel {
            case info
            case warning
            case critical
        }
    }
}

// MARK: - Popular DApps

extension DApp {
    static let popularDApps: [DApp] = [
        DApp(
            id: "uniswap",
            name: "Uniswap",
            description: "Leading decentralized exchange",
            url: "https://app.uniswap.org",
            iconUrl: "https://app.uniswap.org/favicon.ico",
            category: .defi,
            isFavorite: false,
            lastVisited: nil
        ),
        DApp(
            id: "opensea",
            name: "OpenSea",
            description: "World's largest NFT marketplace",
            url: "https://opensea.io",
            iconUrl: "https://opensea.io/static/images/logos/opensea-logo.svg",
            category: .nft,
            isFavorite: false,
            lastVisited: nil
        ),
        DApp(
            id: "aave",
            name: "Aave",
            description: "Non-custodial liquidity protocol",
            url: "https://app.aave.com",
            iconUrl: "https://app.aave.com/favicon.ico",
            category: .defi,
            isFavorite: false,
            lastVisited: nil
        ),
        DApp(
            id: "snapshot",
            name: "Snapshot",
            description: "Decentralized voting platform",
            url: "https://snapshot.org",
            iconUrl: "https://snapshot.org/favicon.ico",
            category: .dao,
            isFavorite: false,
            lastVisited: nil
        )
    ]
}
