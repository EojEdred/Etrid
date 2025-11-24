//
//  MultiSig.swift
//  EtridWallet
//
//  Models for multi-signature wallet operations
//

import Foundation

// MARK: - MultiSig Wallet

struct MultiSigWallet: Identifiable, Codable, Hashable {
    let id: String
    let address: String
    let name: String
    let description: String?
    var owners: [String]
    var threshold: Int
    let chainId: String
    var balance: String
    var pendingTransactionCount: Int
    let createdAt: Date
    var userRole: OwnerRole

    enum OwnerRole: String, Codable {
        case creator = "Creator"
        case owner = "Owner"
        case viewer = "Viewer"
    }

    var thresholdDescription: String {
        "\(threshold) of \(owners.count)"
    }

    var isUserOwner: Bool {
        userRole == .creator || userRole == .owner
    }
}

// MARK: - Pending Transaction

struct MultiSigPendingTransaction: Identifiable, Codable {
    let id: String
    let walletId: String
    let walletAddress: String
    let nonce: Int
    let to: String
    let value: String
    let data: String?
    let operation: OperationType
    let proposer: String
    let proposerName: String?
    var signatures: [TransactionSignature]
    let requiredSignatures: Int
    let createdAt: Date
    var expiresAt: Date?
    var status: MultiSigTransactionStatus
    var executedAt: Date?
    var executionTxHash: String?

    enum OperationType: String, Codable {
        case call = "Call"
        case delegateCall = "Delegate Call"
        case create = "Create"
    }

    enum MultiSigTransactionStatus: String, Codable {
        case pending = "Pending"
        case readyToExecute = "Ready to Execute"
        case executed = "Executed"
        case rejected = "Rejected"
        case expired = "Expired"
    }

    var signatureProgress: Double {
        guard requiredSignatures > 0 else { return 0 }
        return Double(signatures.count) / Double(requiredSignatures)
    }

    var isReadyToExecute: Bool {
        signatures.count >= requiredSignatures && status == .pending
    }

    var isExpired: Bool {
        if let expiresAt = expiresAt {
            return Date() > expiresAt
        }
        return false
    }

    var valueInEth: String {
        guard let valueInt = Double(value) else { return "0" }
        let eth = valueInt / 1_000_000_000_000_000_000
        return String(format: "%.6f", eth)
    }
}

// MARK: - Transaction Signature

struct TransactionSignature: Identifiable, Codable, Hashable {
    let id: String
    let transactionId: String
    let signer: String
    let signerName: String?
    let signature: String
    let signedAt: Date

    var shortSignature: String {
        if signature.count > 16 {
            return String(signature.prefix(10)) + "..." + String(signature.suffix(6))
        }
        return signature
    }
}

// MARK: - MultiSig Owner

struct MultiSigOwner: Identifiable, Codable {
    let id: String
    let address: String
    let name: String?
    let addedAt: Date
    var transactionsSigned: Int
    var transactionsProposed: Int

    var displayName: String {
        name ?? shortAddress
    }

    var shortAddress: String {
        if address.count > 16 {
            return String(address.prefix(6)) + "..." + String(address.suffix(4))
        }
        return address
    }
}

// MARK: - MultiSig Transaction History

struct MultiSigTransactionHistory: Identifiable, Codable {
    let id: String
    let walletId: String
    let type: HistoryType
    let to: String?
    let value: String?
    let operation: MultiSigPendingTransaction.OperationType?
    let proposer: String
    let executedBy: String?
    let signatures: [TransactionSignature]
    let createdAt: Date
    let executedAt: Date?
    let txHash: String?

    enum HistoryType: String, Codable {
        case executed = "Executed"
        case rejected = "Rejected"
        case expired = "Expired"
        case ownerAdded = "Owner Added"
        case ownerRemoved = "Owner removed"
        case thresholdChanged = "Threshold Changed"
    }
}

// MARK: - Create MultiSig Parameters

struct CreateMultiSigParams {
    let name: String
    let description: String?
    let owners: [String]
    let threshold: Int
    let chainId: String
    let saltNonce: String?

    var isValid: Bool {
        guard !name.isEmpty else { return false }
        guard owners.count >= threshold else { return false }
        guard threshold >= 1 else { return false }
        guard owners.count >= 2 else { return false }
        return true
    }

    var validationError: String? {
        if name.isEmpty {
            return "Name cannot be empty"
        }
        if owners.count < 2 {
            return "At least 2 owners required"
        }
        if threshold < 1 {
            return "Threshold must be at least 1"
        }
        if owners.count < threshold {
            return "Threshold cannot exceed number of owners"
        }
        return nil
    }
}

// MARK: - Propose Transaction Parameters

struct ProposeTransactionParams {
    let walletId: String
    let to: String
    let value: String
    let data: String?
    let operation: MultiSigPendingTransaction.OperationType
    let description: String?
    let expiryHours: Int?

    var valueInEth: String {
        guard let valueInt = Double(value) else { return "0" }
        let eth = valueInt / 1_000_000_000_000_000_000
        return String(format: "%.6f", eth)
    }
}

// MARK: - MultiSig Activity

struct MultiSigActivity: Identifiable {
    let id: String
    let walletId: String
    let type: ActivityType
    let description: String
    let actor: String
    let actorName: String?
    let timestamp: Date
    let txHash: String?

    enum ActivityType {
        case created
        case transactionProposed
        case transactionSigned
        case transactionExecuted
        case transactionRejected
        case ownerAdded
        case ownerRemoved
        case thresholdChanged
        case deposit
        case withdrawal
    }

    var icon: String {
        switch type {
        case .created: return "plus.circle"
        case .transactionProposed: return "doc.badge.plus"
        case .transactionSigned: return "signature"
        case .transactionExecuted: return "checkmark.circle.fill"
        case .transactionRejected: return "xmark.circle"
        case .ownerAdded: return "person.badge.plus"
        case .ownerRemoved: return "person.badge.minus"
        case .thresholdChanged: return "slider.horizontal.3"
        case .deposit: return "arrow.down.circle"
        case .withdrawal: return "arrow.up.circle"
        }
    }

    var color: String {
        switch type {
        case .created, .transactionExecuted, .ownerAdded, .deposit:
            return "green"
        case .transactionProposed, .transactionSigned, .thresholdChanged:
            return "blue"
        case .transactionRejected, .ownerRemoved:
            return "red"
        case .withdrawal:
            return "orange"
        }
    }
}

// MARK: - MultiSig Stats

struct MultiSigStats: Codable {
    let totalTransactions: Int
    let executedTransactions: Int
    let pendingTransactions: Int
    let rejectedTransactions: Int
    let totalValue: String
    let averageSigningTime: TimeInterval

    var executionRate: Double {
        guard totalTransactions > 0 else { return 0 }
        return Double(executedTransactions) / Double(totalTransactions) * 100
    }
}
