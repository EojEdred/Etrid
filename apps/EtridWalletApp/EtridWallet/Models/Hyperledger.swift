//
//  Hyperledger.swift
//  EtridWallet
//
//  Hyperledger Fabric Bridge Models
//

import Foundation

// MARK: - Fabric Network
struct FabricNetwork: Codable, Identifiable {
    let id: String
    let name: String
    let description: String
    let channelName: String
    let chaincodeName: String
    let mspId: String
    let peerEndpoints: [String]
    let ordererEndpoints: [String]
    let caEndpoint: String
    let tlsEnabled: Bool
    let status: NetworkStatus
    let supportedTokens: [String]

    enum NetworkStatus: String, Codable {
        case online = "online"
        case offline = "offline"
        case maintenance = "maintenance"

        var color: String {
            switch self {
            case .online: return "#00FF00"
            case .offline: return "#FF0000"
            case .maintenance: return "#FFA500"
            }
        }
    }

    var statusText: String {
        status.rawValue.capitalized
    }

    var endpointCount: Int {
        peerEndpoints.count + ordererEndpoints.count
    }
}

// MARK: - Bridge Status
enum BridgeStatus: String, Codable {
    case initiated = "initiated"
    case locked = "locked"
    case proofGenerated = "proof_generated"
    case minting = "minting"
    case burning = "burning"
    case unlocking = "unlocking"
    case completed = "completed"
    case failed = "failed"
    case cancelled = "cancelled"

    var color: String {
        switch self {
        case .initiated, .locked, .proofGenerated, .minting, .burning, .unlocking:
            return "#FFA500"
        case .completed:
            return "#00FF00"
        case .failed, .cancelled:
            return "#FF0000"
        }
    }

    var stepNumber: Int {
        switch self {
        case .initiated: return 1
        case .locked: return 2
        case .proofGenerated: return 3
        case .minting, .burning: return 4
        case .unlocking: return 4
        case .completed: return 5
        case .failed, .cancelled: return 0
        }
    }

    var description: String {
        switch self {
        case .initiated:
            return "Bridge transaction initiated"
        case .locked:
            return "Assets locked on source chain"
        case .proofGenerated:
            return "Cryptographic proof generated"
        case .minting:
            return "Minting assets on Fabric"
        case .burning:
            return "Burning assets on Fabric"
        case .unlocking:
            return "Unlocking assets on Etrid"
        case .completed:
            return "Bridge transaction completed"
        case .failed:
            return "Transaction failed"
        case .cancelled:
            return "Transaction cancelled"
        }
    }
}

// MARK: - Bridge Direction
enum BridgeDirection: String, Codable {
    case toFabric = "to_fabric"
    case fromFabric = "from_fabric"

    var displayName: String {
        switch self {
        case .toFabric: return "Etrid → Fabric"
        case .fromFabric: return "Fabric → Etrid"
        }
    }
}

// MARK: - Endorsement
struct Endorsement: Codable, Identifiable {
    let id: String
    let mspId: String
    let signature: String
    let endorser: String
    let timestamp: Date
    let valid: Bool

    var displayName: String {
        let prefix = endorser.prefix(8)
        let suffix = endorser.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var statusIcon: String {
        valid ? "checkmark.circle.fill" : "xmark.circle.fill"
    }

    var statusColor: String {
        valid ? "#00FF00" : "#FF0000"
    }
}

// MARK: - Bridge Transaction
struct BridgeTransaction: Codable, Identifiable {
    let id: String
    let txHash: String
    let direction: BridgeDirection
    let networkId: String
    let sourceChain: String
    let destinationChain: String
    let token: String
    let amount: String
    let sender: String
    let recipient: String
    let status: BridgeStatus
    let lockTxHash: String?
    let mintTxHash: String?
    let burnTxHash: String?
    let unlockTxHash: String?
    let proof: String?
    let endorsements: [Endorsement]?
    let createdAt: Date
    let updatedAt: Date
    let completedAt: Date?
    let errorMessage: String?

    var amountDisplay: String {
        if let value = Double(amount) {
            return String(format: "%.4f %@", value, token)
        }
        return "\(amount) \(token)"
    }

    var senderDisplay: String {
        let prefix = sender.prefix(8)
        let suffix = sender.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var recipientDisplay: String {
        let prefix = recipient.prefix(8)
        let suffix = recipient.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var txHashDisplay: String {
        let prefix = txHash.prefix(8)
        let suffix = txHash.suffix(6)
        return "\(prefix)...\(suffix)"
    }

    var isComplete: Bool {
        status == .completed
    }

    var isFailed: Bool {
        status == .failed || status == .cancelled
    }

    var isInProgress: Bool {
        !isComplete && !isFailed
    }

    var endorsementCount: Int {
        endorsements?.count ?? 0
    }

    var validEndorsementCount: Int {
        endorsements?.filter { $0.valid }.count ?? 0
    }

    var duration: String {
        if let completed = completedAt {
            let interval = completed.timeIntervalSince(createdAt)
            let minutes = Int(interval / 60)
            let seconds = Int(interval.truncatingRemainder(dividingBy: 60))
            return "\(minutes)m \(seconds)s"
        }
        return "In progress"
    }
}

// MARK: - Bridge Asset Request
struct BridgeToFabricRequest: Codable {
    let networkId: String
    let token: String
    let amount: String
    let recipient: String
    let metadata: [String: String]?
}

// MARK: - Bridge Asset Response
struct BridgeToFabricResponse: Codable {
    let transactionId: String
    let lockTxHash: String
    let estimatedTime: Int // seconds
    let message: String
}

// MARK: - Bridge From Fabric Request
struct BridgeFromFabricRequest: Codable {
    let networkId: String
    let token: String
    let amount: String
    let recipient: String
    let fabricTxId: String
    let proof: String
}

// MARK: - Bridge From Fabric Response
struct BridgeFromFabricResponse: Codable {
    let transactionId: String
    let burnTxHash: String
    let estimatedTime: Int // seconds
    let message: String
}

// MARK: - Verify Transaction Request
struct VerifyTransactionRequest: Codable {
    let txHash: String
    let networkId: String
}

// MARK: - Verify Transaction Response
struct VerifyTransactionResponse: Codable {
    let valid: Bool
    let transaction: BridgeTransaction?
    let endorsements: [Endorsement]
    let message: String
}

// MARK: - Bridge History Request
struct BridgeHistoryRequest: Codable {
    let address: String
    let networkId: String?
    let direction: BridgeDirection?
    let status: BridgeStatus?
    let page: Int
    let limit: Int

    init(address: String, networkId: String? = nil, direction: BridgeDirection? = nil, status: BridgeStatus? = nil, page: Int = 1, limit: Int = 20) {
        self.address = address
        self.networkId = networkId
        self.direction = direction
        self.status = status
        self.page = page
        self.limit = limit
    }
}

// MARK: - Bridge History Response
struct BridgeHistoryResponse: Codable {
    let transactions: [BridgeTransaction]
    let total: Int
    let page: Int
    let totalPages: Int
}

// MARK: - Network Statistics
struct NetworkStatistics: Codable {
    let totalBridgeVolume: String
    let totalTransactions: Int
    let activeEndorsers: Int
    let averageConfirmationTime: Int // seconds
    let successRate: Double // percentage

    var volumeDisplay: String {
        if let value = Double(totalBridgeVolume) {
            return String(format: "%.2f ETR", value)
        }
        return totalBridgeVolume
    }

    var successRateDisplay: String {
        String(format: "%.1f%%", successRate)
    }

    var avgTimeDisplay: String {
        let minutes = averageConfirmationTime / 60
        let seconds = averageConfirmationTime % 60
        return "\(minutes)m \(seconds)s"
    }
}

// MARK: - Connect Network Request
struct ConnectNetworkRequest: Codable {
    let networkId: String
    let credentials: NetworkCredentials?
}

// MARK: - Network Credentials
struct NetworkCredentials: Codable {
    let certificate: String
    let privateKey: String
    let userId: String
}

// MARK: - Connect Network Response
struct ConnectNetworkResponse: Codable {
    let connected: Bool
    let network: FabricNetwork
    let statistics: NetworkStatistics?
    let message: String
}
