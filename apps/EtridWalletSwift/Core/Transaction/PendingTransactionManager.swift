//
//  PendingTransactionManager.swift
//  EtridWalletSwift
//
//  Created by Eoj on 2025-11-22.
//  Manage pending transactions, cancellation, and speed-up
//

import Foundation
import Combine

/// Pending transaction state
struct PendingTransaction: Codable, Identifiable {
    let id: String  // Transaction hash
    let hash: String
    let from: String
    let to: String
    let value: BigInt
    let nonce: Int
    let gasLimit: BigInt
    let gasPrice: BigInt
    let maxFeePerGas: BigInt?
    let maxPriorityFeePerGas: BigInt?
    let data: String
    let submittedAt: Date
    var lastCheckedAt: Date
    var status: TransactionStatus
    var confirmations: Int
    var replacementHash: String?  // Hash of replacement transaction (speedup/cancel)

    /// Check if transaction is stuck (pending > 10 minutes)
    var isStuck: Bool {
        guard status == .pending else { return false }
        return Date().timeIntervalSince(submittedAt) > 600  // 10 minutes
    }

    /// Check if transaction is very old (pending > 1 hour)
    var isVeryOld: Bool {
        guard status == .pending else { return false }
        return Date().timeIntervalSince(submittedAt) > 3600  // 1 hour
    }

    /// Duration pending in seconds
    var pendingDuration: TimeInterval {
        return Date().timeIntervalSince(submittedAt)
    }
}

/// Transaction replacement type
enum TransactionReplacementType {
    case cancel      // Cancel by sending 0 ETH to self with same nonce
    case speedUp     // Speed up by increasing gas price with same nonce
}

/// Pending transaction events
enum PendingTransactionEvent {
    case submitted(hash: String)
    case confirmed(hash: String, confirmations: Int)
    case failed(hash: String, error: Error)
    case stuck(hash: String, duration: TimeInterval)
    case replaced(oldHash: String, newHash: String)
    case dropped(hash: String)
}

/// Pending transaction manager errors
enum PendingTransactionError: Error, LocalizedError {
    case transactionNotFound
    case cannotReplaceConfirmedTransaction
    case nonceAlreadyUsed
    case gasPriceTooLow
    case insufficientFundsForReplacement
    case replacementFailed

    var errorDescription: String? {
        switch self {
        case .transactionNotFound:
            return "Transaction not found in pending list"
        case .cannotReplaceConfirmedTransaction:
            return "Cannot replace a confirmed transaction"
        case .nonceAlreadyUsed:
            return "Nonce has already been used"
        case .gasPriceTooLow:
            return "Gas price must be at least 10% higher than original"
        case .insufficientFundsForReplacement:
            return "Insufficient funds for replacement transaction"
        case .replacementFailed:
            return "Failed to submit replacement transaction"
        }
    }
}

/// Manages pending transactions with monitoring and replacement capabilities
actor PendingTransactionManager {
    private let web3Service: Web3Service
    private let gasEstimator: GasEstimator
    private let nonceManager: NonceManager
    private let transactionBuilder: TransactionBuilder

    private var pendingTransactions: [String: PendingTransaction] = [:]
    private var monitoringTasks: [String: Task<Void, Never>] = [:]

    // Event publisher
    private let eventSubject = PassthroughSubject<PendingTransactionEvent, Never>()
    var events: AnyPublisher<PendingTransactionEvent, Never> {
        eventSubject.eraseToAnyPublisher()
    }

    // Monitoring configuration
    private let checkInterval: TimeInterval = 5.0  // Check every 5 seconds
    private let stuckThreshold: TimeInterval = 600  // 10 minutes
    private let minGasPriceIncrease: Double = 1.1   // 10% minimum increase

    init(
        web3Service: Web3Service,
        gasEstimator: GasEstimator,
        nonceManager: NonceManager,
        transactionBuilder: TransactionBuilder
    ) {
        self.web3Service = web3Service
        self.gasEstimator = gasEstimator
        self.nonceManager = nonceManager
        self.transactionBuilder = transactionBuilder
    }

    // MARK: - Transaction Submission

    /// Add transaction to pending list and start monitoring
    func trackTransaction(_ transaction: PendingTransaction) async {
        pendingTransactions[transaction.hash] = transaction

        // Start monitoring
        await startMonitoring(hash: transaction.hash)

        // Notify
        eventSubject.send(.submitted(hash: transaction.hash))
    }

    // MARK: - Transaction Monitoring

    /// Start monitoring a transaction
    private func startMonitoring(hash: String) async {
        // Cancel existing monitoring task if any
        monitoringTasks[hash]?.cancel()

        // Create new monitoring task
        let task = Task {
            await monitorTransactionStatus(hash: hash)
        }

        monitoringTasks[hash] = task
    }

    /// Monitor transaction status continuously
    private func monitorTransactionStatus(hash: String) async {
        while !Task.isCancelled {
            do {
                guard var transaction = pendingTransactions[hash] else {
                    return  // Transaction removed from pending list
                }

                // Check transaction status
                let receipt = try? await web3Service.getTransactionReceipt(hash: hash)

                if let receipt = receipt {
                    // Transaction is mined
                    let currentBlock = try await web3Service.getBlockNumber()
                    let confirmations = receipt.blockNumber > 0 ? currentBlock - receipt.blockNumber + 1 : 0

                    transaction.confirmations = confirmations
                    transaction.lastCheckedAt = Date()

                    if receipt.isSuccess {
                        transaction.status = .confirmed
                        eventSubject.send(.confirmed(hash: hash, confirmations: confirmations))

                        // Remove from pending after 12 confirmations (finalized)
                        if confirmations >= 12 {
                            await removeTransaction(hash: hash)
                            return
                        }
                    } else {
                        transaction.status = .failed
                        eventSubject.send(.failed(hash: hash, error: NSError(domain: "Transaction failed", code: -1)))
                        await removeTransaction(hash: hash)
                        return
                    }

                    pendingTransactions[hash] = transaction

                } else {
                    // Transaction still pending
                    transaction.lastCheckedAt = Date()

                    // Check if stuck
                    if transaction.isStuck && !transaction.isVeryOld {
                        eventSubject.send(.stuck(hash: hash, duration: transaction.pendingDuration))
                    }

                    // Check if dropped (very old and not found)
                    if transaction.isVeryOld {
                        // Verify if nonce was used by another transaction
                        let currentNonce = try? await web3Service.getTransactionCount(address: transaction.from)
                        if let currentNonce = currentNonce, currentNonce > transaction.nonce {
                            transaction.status = .dropped
                            eventSubject.send(.dropped(hash: hash))
                            await removeTransaction(hash: hash)
                            return
                        }
                    }

                    pendingTransactions[hash] = transaction
                }

                // Wait before next check
                try await Task.sleep(nanoseconds: UInt64(checkInterval * 1_000_000_000))

            } catch {
                // Continue monitoring on error
                try? await Task.sleep(nanoseconds: UInt64(checkInterval * 1_000_000_000))
            }
        }
    }

    // MARK: - Transaction Replacement

    /// Cancel a pending transaction by sending 0 ETH to self with same nonce
    func cancelTransaction(hash: String, privateKey: Data) async throws -> String {
        guard let transaction = pendingTransactions[hash] else {
            throw PendingTransactionError.transactionNotFound
        }

        guard transaction.status == .pending else {
            throw PendingTransactionError.cannotReplaceConfirmedTransaction
        }

        // Create cancellation transaction (0 ETH to self with higher gas)
        let newGasPrice = calculateReplacementGasPrice(original: transaction.gasPrice)

        let cancelParams = TransactionParameters(
            from: transaction.from,
            to: transaction.from,  // Send to self
            value: BigInt(0),      // 0 ETH
            data: "0x",
            chainId: 1,  // Should be from config
            nonce: transaction.nonce,  // Same nonce
            gasLimit: BigInt(21000),
            maxFeePerGas: transaction.maxFeePerGas != nil ? calculateReplacementFee(transaction.maxFeePerGas!) : nil,
            maxPriorityFeePerGas: transaction.maxPriorityFeePerGas != nil ? calculateReplacementFee(transaction.maxPriorityFeePerGas!) : nil,
            gasPrice: transaction.maxFeePerGas == nil ? newGasPrice : nil
        )

        // Build and sign transaction
        let rawTx = try await transactionBuilder.buildTransaction(params: cancelParams)
        let signedTx = try await transactionBuilder.signTransaction(rawTx, privateKey: privateKey)
        let serialized = try await transactionBuilder.serializeTransaction(signedTx)

        // Submit to network
        let newHash = try await web3Service.sendRawTransaction(serialized)

        // Update original transaction
        var updatedTx = transaction
        updatedTx.replacementHash = newHash
        pendingTransactions[hash] = updatedTx

        // Track new transaction
        let cancelTx = PendingTransaction(
            id: newHash,
            hash: newHash,
            from: transaction.from,
            to: transaction.from,
            value: BigInt(0),
            nonce: transaction.nonce,
            gasLimit: BigInt(21000),
            gasPrice: newGasPrice,
            maxFeePerGas: cancelParams.maxFeePerGas,
            maxPriorityFeePerGas: cancelParams.maxPriorityFeePerGas,
            data: "0x",
            submittedAt: Date(),
            lastCheckedAt: Date(),
            status: .pending,
            confirmations: 0
        )

        await trackTransaction(cancelTx)

        // Notify
        eventSubject.send(.replaced(oldHash: hash, newHash: newHash))

        return newHash
    }

    /// Speed up a pending transaction by resubmitting with higher gas
    func speedUpTransaction(hash: String, privateKey: Data, newSpeed: GasSpeed = .fast) async throws -> String {
        guard let transaction = pendingTransactions[hash] else {
            throw PendingTransactionError.transactionNotFound
        }

        guard transaction.status == .pending else {
            throw PendingTransactionError.cannotReplaceConfirmedTransaction
        }

        // Get new gas estimate with higher speed
        let speedUpParams = TransactionParameters(
            from: transaction.from,
            to: transaction.to,
            value: transaction.value,
            data: transaction.data,
            chainId: 1,  // Should be from config
            nonce: transaction.nonce,  // Same nonce
            gasLimit: transaction.gasLimit,
            maxFeePerGas: nil,
            maxPriorityFeePerGas: nil,
            gasPrice: nil
        )

        let gasEstimate = try await gasEstimator.estimateGas(for: speedUpParams, speed: newSpeed)

        // Ensure new gas price is at least 10% higher
        let newGasPrice = max(
            calculateReplacementGasPrice(original: transaction.gasPrice),
            gasEstimate.maxFeePerGas
        )

        // Build speed-up transaction
        let speedUpTx = TransactionParameters(
            from: transaction.from,
            to: transaction.to,
            value: transaction.value,
            data: transaction.data,
            chainId: 1,
            nonce: transaction.nonce,
            gasLimit: transaction.gasLimit,
            maxFeePerGas: gasEstimate.maxFeePerGas,
            maxPriorityFeePerGas: gasEstimate.maxPriorityFeePerGas,
            gasPrice: transaction.maxFeePerGas == nil ? newGasPrice : nil
        )

        // Build and sign
        let rawTx = try await transactionBuilder.buildTransaction(params: speedUpTx)
        let signedTx = try await transactionBuilder.signTransaction(rawTx, privateKey: privateKey)
        let serialized = try await transactionBuilder.serializeTransaction(signedTx)

        // Submit to network
        let newHash = try await web3Service.sendRawTransaction(serialized)

        // Update original transaction
        var updatedTx = transaction
        updatedTx.replacementHash = newHash
        pendingTransactions[hash] = updatedTx

        // Track new transaction
        let newTx = PendingTransaction(
            id: newHash,
            hash: newHash,
            from: transaction.from,
            to: transaction.to,
            value: transaction.value,
            nonce: transaction.nonce,
            gasLimit: transaction.gasLimit,
            gasPrice: newGasPrice,
            maxFeePerGas: speedUpTx.maxFeePerGas,
            maxPriorityFeePerGas: speedUpTx.maxPriorityFeePerGas,
            data: transaction.data,
            submittedAt: Date(),
            lastCheckedAt: Date(),
            status: .pending,
            confirmations: 0
        )

        await trackTransaction(newTx)

        // Notify
        eventSubject.send(.replaced(oldHash: hash, newHash: newHash))

        return newHash
    }

    // MARK: - Query Methods

    /// Get all pending transactions
    func getAllPendingTransactions() -> [PendingTransaction] {
        return Array(pendingTransactions.values)
    }

    /// Get pending transactions for address
    func getPendingTransactions(for address: String) -> [PendingTransaction] {
        return pendingTransactions.values.filter {
            $0.from.lowercased() == address.lowercased()
        }
    }

    /// Get stuck transactions
    func getStuckTransactions() -> [PendingTransaction] {
        return pendingTransactions.values.filter { $0.isStuck }
    }

    /// Get transaction by hash
    func getTransaction(hash: String) -> PendingTransaction? {
        return pendingTransactions[hash]
    }

    // MARK: - Cleanup

    /// Remove transaction from pending list
    private func removeTransaction(hash: String) async {
        pendingTransactions.removeValue(forKey: hash)
        monitoringTasks[hash]?.cancel()
        monitoringTasks.removeValue(forKey: hash)
    }

    /// Clear all pending transactions
    func clearAll() async {
        for task in monitoringTasks.values {
            task.cancel()
        }
        monitoringTasks.removeAll()
        pendingTransactions.removeAll()
    }

    // MARK: - Helper Methods

    /// Calculate replacement gas price (10% higher minimum)
    private func calculateReplacementGasPrice(original: BigInt) -> BigInt {
        return BigInt(Double(original.description)! * minGasPriceIncrease)
    }

    /// Calculate replacement fee (10% higher minimum)
    private func calculateReplacementFee(_ original: BigInt) -> BigInt {
        return BigInt(Double(original.description)! * minGasPriceIncrease)
    }
}
