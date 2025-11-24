//
//  WalletConnect.swift
//  EtridWallet
//
//  Models for WalletConnect v2 protocol
//

import Foundation

// MARK: - WalletConnect Session

struct WCSession: Identifiable, Codable, Hashable {
    let id: String
    let topic: String
    let dAppUrl: String
    let dAppName: String
    let dAppIcon: String?
    let dAppDescription: String?
    var permissions: [String]
    var chains: [String]
    let createdAt: Date
    let expiresAt: Date
    var lastUsed: Date

    var isExpired: Bool {
        Date() > expiresAt
    }

    var daysUntilExpiry: Int {
        let calendar = Calendar.current
        let components = calendar.dateComponents([.day], from: Date(), to: expiresAt)
        return max(0, components.day ?? 0)
    }
}

// MARK: - WalletConnect Proposal

struct WCProposal: Identifiable, Codable {
    let id: String
    let proposer: WCProposer
    let permissions: WCPermissions
    let expiresAt: Date

    var isExpired: Bool {
        Date() > expiresAt
    }
}

struct WCProposer: Codable {
    let name: String
    let description: String
    let url: String
    let icons: [String]

    var iconUrl: String? {
        icons.first
    }
}

struct WCPermissions: Codable {
    let chains: [String]
    let methods: [String]
    let events: [String]

    var methodsDescription: String {
        methods.joined(separator: ", ")
    }

    var chainsDescription: String {
        chains.map { chain in
            // Convert eip155:1 to "Ethereum"
            let chainId = chain.split(separator: ":").last ?? ""
            switch chainId {
            case "1": return "Ethereum"
            case "137": return "Polygon"
            case "42161": return "Arbitrum"
            case "10": return "Optimism"
            default: return "Chain \(chainId)"
            }
        }.joined(separator: ", ")
    }
}

// MARK: - WalletConnect Request

struct WCRequest: Identifiable {
    let id: String
    let topic: String
    let method: String
    let params: Any
    let chainId: String?
    let sessionInfo: WCSession

    var methodDescription: String {
        switch method {
        case "eth_sendTransaction": return "Send Transaction"
        case "personal_sign": return "Sign Message"
        case "eth_sign": return "Sign Message"
        case "eth_signTypedData": return "Sign Typed Data"
        case "eth_signTypedData_v4": return "Sign Typed Data (v4)"
        case "wallet_addEthereumChain": return "Add Network"
        case "wallet_switchEthereumChain": return "Switch Network"
        default: return method
        }
    }

    var requestType: RequestType {
        switch method {
        case "eth_sendTransaction": return .transaction
        case "personal_sign", "eth_sign": return .message
        case "eth_signTypedData", "eth_signTypedData_v4": return .typedData
        case "wallet_addEthereumChain", "wallet_switchEthereumChain": return .network
        default: return .other
        }
    }

    enum RequestType {
        case transaction
        case message
        case typedData
        case network
        case other
    }
}

// MARK: - Pairing Data

struct PairingData: Codable {
    let uri: String
    let topic: String?
    let relay: RelayProtocol?

    struct RelayProtocol: Codable {
        let protocolName: String
        let data: String?
    }

    var isValid: Bool {
        uri.hasPrefix("wc:")
    }
}

// MARK: - WalletConnect Events

enum WCEvent {
    case sessionProposal(WCProposal)
    case sessionRequest(WCRequest)
    case sessionDelete(String) // topic
    case sessionUpdate(WCSession)
    case pairingExpired(String) // topic
}

// MARK: - WalletConnect Error

enum WCError: Error, LocalizedError {
    case invalidURI
    case sessionNotFound
    case proposalNotFound
    case userRejected
    case unsupportedMethod(String)
    case invalidParams
    case connectionFailed

    var errorDescription: String? {
        switch self {
        case .invalidURI:
            return "Invalid WalletConnect URI"
        case .sessionNotFound:
            return "Session not found"
        case .proposalNotFound:
            return "Proposal not found"
        case .userRejected:
            return "User rejected the request"
        case .unsupportedMethod(let method):
            return "Unsupported method: \(method)"
        case .invalidParams:
            return "Invalid parameters"
        case .connectionFailed:
            return "Failed to connect"
        }
    }
}

// MARK: - Session State

enum WCSessionState: String {
    case connecting = "Connecting"
    case connected = "Connected"
    case disconnecting = "Disconnecting"
    case disconnected = "Disconnected"
    case error = "Error"
}

// MARK: - Helpers

extension WCSession {
    var shortTopic: String {
        if topic.count > 16 {
            return String(topic.prefix(8)) + "..." + String(topic.suffix(8))
        }
        return topic
    }

    func supportsMethod(_ method: String) -> Bool {
        permissions.contains(method)
    }
}
