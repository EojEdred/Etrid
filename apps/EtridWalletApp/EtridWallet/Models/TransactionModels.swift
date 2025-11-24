//
//  TransactionModels.swift
//  EtridWallet
//
//  Transaction-related data models
//

import Foundation

// MARK: - Transaction
struct Transaction: Identifiable, Codable, Equatable {
    let id: UUID
    let hash: String
    let from: String
    let to: String
    let value: String
    let data: String?
    let gasLimit: String
    let gasPrice: String
    let nonce: Int
    let timestamp: Date
    let status: TransactionStatus
    let network: String
    let type: TransactionType
    var blockNumber: Int?
    var confirmations: Int?

    init(id: UUID = UUID(),
         hash: String,
         from: String,
         to: String,
         value: String,
         data: String? = nil,
         gasLimit: String,
         gasPrice: String,
         nonce: Int,
         timestamp: Date = Date(),
         status: TransactionStatus = .pending,
         network: String,
         type: TransactionType = .send,
         blockNumber: Int? = nil,
         confirmations: Int? = nil) {
        self.id = id
        self.hash = hash
        self.from = from
        self.to = to
        self.value = value
        self.data = data
        self.gasLimit = gasLimit
        self.gasPrice = gasPrice
        self.nonce = nonce
        self.timestamp = timestamp
        self.status = status
        self.network = network
        self.type = type
        self.blockNumber = blockNumber
        self.confirmations = confirmations
    }

    var formattedValue: String {
        let val = Double(value) ?? 0
        return String(format: "%.6f", val / 1e18)
    }

    var formattedGasFee: String {
        let limit = UInt64(gasLimit) ?? 0
        let price = UInt64(gasPrice) ?? 0
        let fee = Double(limit * price)
        return String(format: "%.6f", fee / 1e18)
    }

    var shortHash: String {
        guard hash.count > 10 else { return hash }
        let start = hash.prefix(6)
        let end = hash.suffix(4)
        return "\(start)...\(end)"
    }

    var isConfirmed: Bool {
        status == .confirmed
    }

    var isPending: Bool {
        status == .pending
    }

    var isFailed: Bool {
        status == .failed
    }
}

// MARK: - Transaction Status
enum TransactionStatus: String, Codable {
    case pending = "Pending"
    case confirmed = "Confirmed"
    case failed = "Failed"
    case dropped = "Dropped"
    case replaced = "Replaced"

    var iconName: String {
        switch self {
        case .pending:
            return "clock.fill"
        case .confirmed:
            return "checkmark.circle.fill"
        case .failed:
            return "xmark.circle.fill"
        case .dropped:
            return "trash.fill"
        case .replaced:
            return "arrow.triangle.2.circlepath"
        }
    }

    var color: String {
        switch self {
        case .pending:
            return "orange"
        case .confirmed:
            return "green"
        case .failed, .dropped:
            return "red"
        case .replaced:
            return "blue"
        }
    }
}

// MARK: - Transaction Type
enum TransactionType: String, Codable {
    case send = "Send"
    case receive = "Receive"
    case contractInteraction = "Contract Interaction"
    case tokenTransfer = "Token Transfer"
    case swap = "Swap"
    case approval = "Approval"

    var iconName: String {
        switch self {
        case .send:
            return "arrow.up.circle.fill"
        case .receive:
            return "arrow.down.circle.fill"
        case .contractInteraction:
            return "doc.text.fill"
        case .tokenTransfer:
            return "arrow.left.arrow.right.circle.fill"
        case .swap:
            return "arrow.triangle.swap"
        case .approval:
            return "checkmark.seal.fill"
        }
    }
}

// MARK: - Transaction Request
struct TransactionRequest: Codable {
    let from: String
    let to: String
    let value: String
    let data: String?
    var gasLimit: String?
    var gasPrice: String?
    var maxFeePerGas: String?
    var maxPriorityFeePerGas: String?
    var nonce: Int?
    let chainId: Int

    init(from: String,
         to: String,
         value: String,
         data: String? = nil,
         gasLimit: String? = nil,
         gasPrice: String? = nil,
         maxFeePerGas: String? = nil,
         maxPriorityFeePerGas: String? = nil,
         nonce: Int? = nil,
         chainId: Int) {
        self.from = from
        self.to = to
        self.value = value
        self.data = data
        self.gasLimit = gasLimit
        self.gasPrice = gasPrice
        self.maxFeePerGas = maxFeePerGas
        self.maxPriorityFeePerGas = maxPriorityFeePerGas
        self.nonce = nonce
        self.chainId = chainId
    }
}

// MARK: - Signed Transaction
struct SignedTransaction {
    let rawTransaction: String
    let hash: String
    let transaction: TransactionRequest
}

// MARK: - Transaction Receipt
struct TransactionReceipt: Codable {
    let transactionHash: String
    let blockNumber: Int
    let blockHash: String
    let from: String
    let to: String?
    let gasUsed: String
    let cumulativeGasUsed: String
    let status: String // "0x1" for success, "0x0" for failure
    let logs: [TransactionLog]

    var isSuccess: Bool {
        status == "0x1"
    }
}

// MARK: - Transaction Log
struct TransactionLog: Codable {
    let address: String
    let topics: [String]
    let data: String
    let blockNumber: String
    let transactionHash: String
    let logIndex: String
}

// MARK: - Pending Transaction
struct PendingTransaction: Identifiable, Codable {
    let id: UUID
    let hash: String
    let request: TransactionRequest
    let timestamp: Date
    var attempts: Int
    var lastChecked: Date

    init(id: UUID = UUID(),
         hash: String,
         request: TransactionRequest,
         timestamp: Date = Date(),
         attempts: Int = 0,
         lastChecked: Date = Date()) {
        self.id = id
        self.hash = hash
        self.request = request
        self.timestamp = timestamp
        self.attempts = attempts
        self.lastChecked = lastChecked
    }
}

// MARK: - EIP-1559 Transaction
struct EIP1559Transaction: Codable {
    let chainId: Int
    let nonce: Int
    let maxPriorityFeePerGas: String
    let maxFeePerGas: String
    let gasLimit: String
    let to: String
    let value: String
    let data: String
    // TODO: Implement proper AccessList struct for Codable conformance
    // var accessList: [[String: Any]]?

    var isLegacy: Bool {
        false
    }
}

// MARK: - Legacy Transaction
struct LegacyTransaction: Codable {
    let chainId: Int
    let nonce: Int
    let gasPrice: String
    let gasLimit: String
    let to: String
    let value: String
    let data: String

    var isLegacy: Bool {
        true
    }
}
