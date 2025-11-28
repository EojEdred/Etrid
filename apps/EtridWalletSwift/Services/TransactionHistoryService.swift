//
//  TransactionHistoryService.swift
//  EtridWalletSwift
//
//  Created by Eoj on 2025-11-22.
//  Fetch and manage transaction history
//

import Foundation

/// Transaction status
enum TransactionStatus: String, Codable {
    case pending = "Pending"
    case confirmed = "Confirmed"
    case failed = "Failed"
    case dropped = "Dropped"

    var iconName: String {
        switch self {
        case .pending: return "clock.fill"
        case .confirmed: return "checkmark.circle.fill"
        case .failed: return "xmark.circle.fill"
        case .dropped: return "minus.circle.fill"
        }
    }

    var color: String {
        switch self {
        case .pending: return "orange"
        case .confirmed: return "green"
        case .failed: return "red"
        case .dropped: return "gray"
        }
    }
}

/// Transaction direction
enum TransactionDirection: String, Codable {
    case incoming = "Incoming"
    case outgoing = "Outgoing"
    case self_ = "Self"

    var symbol: String {
        switch self {
        case .incoming: return "↓"
        case .outgoing: return "↑"
        case .self_: return "↻"
        }
    }
}

/// Transaction receipt from blockchain
struct TransactionReceipt: Codable {
    let transactionHash: String
    let blockNumber: Int
    let blockHash: String
    let from: String
    let to: String?
    let gasUsed: BigInt
    let effectiveGasPrice: BigInt
    let status: Int  // 1 for success, 0 for failure
    let contractAddress: String?
    let logs: [TransactionLog]

    var isSuccess: Bool {
        return status == 1
    }
}

/// Transaction log entry
struct TransactionLog: Codable {
    let address: String
    let topics: [String]
    let data: String
    let blockNumber: Int
    let transactionHash: String
    let logIndex: Int
}

/// Complete transaction details
struct TransactionDetails: Codable, Identifiable {
    let id: String  // Transaction hash
    let hash: String
    let blockNumber: Int?
    let timestamp: Date
    let from: String
    let to: String
    let value: BigInt
    let gasLimit: BigInt
    let gasUsed: BigInt?
    let gasPrice: BigInt
    let nonce: Int
    let data: String
    let status: TransactionStatus
    let confirmations: Int
    let direction: TransactionDirection

    // EIP-1559 fields
    let maxFeePerGas: BigInt?
    let maxPriorityFeePerGas: BigInt?
    let baseFeePerGas: BigInt?

    /// Transaction type: 0 (legacy) or 2 (EIP-1559)
    var type: Int {
        return maxFeePerGas != nil ? 2 : 0
    }

    /// Total fee paid
    var totalFee: BigInt {
        if let gasUsed = gasUsed {
            return gasUsed * gasPrice
        }
        return BigInt(0)
    }

    /// Transaction fee in ETH
    var feeInEth: Double {
        return Double(totalFee.description) ?? 0.0 / 1e18
    }

    /// Transaction value in ETH
    var valueInEth: Double {
        return Double(value.description) ?? 0.0 / 1e18
    }

    /// Check if transaction is confirmed
    var isConfirmed: Bool {
        return status == .confirmed && confirmations > 0
    }

    /// Check if transaction is finalized (12+ confirmations)
    var isFinalized: Bool {
        return confirmations >= 12
    }
}

/// Transaction history pagination
struct TransactionPage {
    let transactions: [TransactionDetails]
    let hasMore: Bool
    let nextPage: Int?
}

/// Transaction history errors
enum TransactionHistoryError: Error, LocalizedError {
    case failedToFetchHistory
    case invalidAddress
    case transactionNotFound
    case receiptNotAvailable
    case networkError

    var errorDescription: String? {
        switch self {
        case .failedToFetchHistory:
            return "Failed to fetch transaction history"
        case .invalidAddress:
            return "Invalid Ethereum address"
        case .transactionNotFound:
            return "Transaction not found on blockchain"
        case .receiptNotAvailable:
            return "Transaction receipt not yet available"
        case .networkError:
            return "Network error while fetching transactions"
        }
    }
}

/// Service for fetching and managing transaction history
actor TransactionHistoryService {
    private let web3Service: Web3Service
    private var historyCache: [String: [TransactionDetails]] = [:]
    private let pageSize = 50

    init(web3Service: Web3Service) {
        self.web3Service = web3Service
    }

    // MARK: - Fetch Transaction History

    /// Fetch transaction history for an address
    func fetchTransactionHistory(
        address: String,
        page: Int = 0,
        limit: Int = 50
    ) async throws -> TransactionPage {
        // Validate address
        guard isValidAddress(address) else {
            throw TransactionHistoryError.invalidAddress
        }

        // Check cache first
        if let cached = historyCache[address], !cached.isEmpty {
            return paginateTransactions(cached, page: page, limit: limit)
        }

        // Fetch from blockchain
        do {
            let transactions = try await fetchFromBlockchain(address: address)

            // Sort by timestamp (newest first)
            let sorted = transactions.sorted { $0.timestamp > $1.timestamp }

            // Cache the results
            historyCache[address] = sorted

            return paginateTransactions(sorted, page: page, limit: limit)

        } catch {
            throw TransactionHistoryError.failedToFetchHistory
        }
    }

    /// Fetch transaction details by hash
    func fetchTransaction(hash: String) async throws -> TransactionDetails {
        do {
            // Get transaction from blockchain
            guard let tx = try await web3Service.getTransaction(hash: hash) else {
                throw TransactionHistoryError.transactionNotFound
            }

            // Get receipt if available
            let receipt = try? await web3Service.getTransactionReceipt(hash: hash)

            // Get current block number for confirmations
            let currentBlock = try await web3Service.getBlockNumber()

            // Determine status
            let status = determineStatus(tx: tx, receipt: receipt)

            // Calculate confirmations
            let confirmations = calculateConfirmations(
                txBlockNumber: tx.blockNumber,
                currentBlock: currentBlock
            )

            return TransactionDetails(
                id: tx.hash,
                hash: tx.hash,
                blockNumber: tx.blockNumber,
                timestamp: tx.timestamp ?? Date(),
                from: tx.from,
                to: tx.to ?? "0x0",
                value: tx.value,
                gasLimit: tx.gas,
                gasUsed: receipt?.gasUsed,
                gasPrice: receipt?.effectiveGasPrice ?? tx.gasPrice ?? BigInt(0),
                nonce: tx.nonce,
                data: tx.input,
                status: status,
                confirmations: confirmations,
                direction: determineDirection(tx: tx, address: ""),
                maxFeePerGas: tx.maxFeePerGas,
                maxPriorityFeePerGas: tx.maxPriorityFeePerGas,
                baseFeePerGas: nil
            )

        } catch {
            throw TransactionHistoryError.transactionNotFound
        }
    }

    /// Refresh transaction status
    func refreshTransactionStatus(hash: String) async throws -> TransactionStatus {
        let tx = try await fetchTransaction(hash: hash)
        return tx.status
    }

    // MARK: - Private Methods

    /// Fetch transactions from blockchain
    private func fetchFromBlockchain(address: String) async throws -> [TransactionDetails] {
        var transactions: [TransactionDetails] = []

        // Get latest block number
        let latestBlock = try await web3Service.getBlockNumber()

        // Scan recent blocks for transactions
        // In production, use a blockchain indexer/API like Etherscan
        let blocksToScan = min(1000, latestBlock)
        let startBlock = max(0, latestBlock - blocksToScan)

        for blockNum in stride(from: latestBlock, through: startBlock, by: -1) {
            do {
                let block = try await web3Service.getBlock(number: blockNum)

                // Filter transactions for this address
                for txHash in block.transactions {
                    if let tx = try? await web3Service.getTransaction(hash: txHash) {
                        if tx.from.lowercased() == address.lowercased() ||
                           tx.to?.lowercased() == address.lowercased() {

                            // Get receipt
                            let receipt = try? await web3Service.getTransactionReceipt(hash: txHash)

                            // Create transaction details
                            let details = createTransactionDetails(
                                tx: tx,
                                receipt: receipt,
                                address: address,
                                currentBlock: latestBlock
                            )

                            transactions.append(details)
                        }
                    }
                }
            } catch {
                // Continue scanning other blocks
                continue
            }
        }

        return transactions
    }

    /// Create transaction details from raw transaction
    private func createTransactionDetails(
        tx: Web3Transaction,
        receipt: TransactionReceipt?,
        address: String,
        currentBlock: Int
    ) -> TransactionDetails {
        let status = determineStatus(tx: tx, receipt: receipt)
        let confirmations = calculateConfirmations(
            txBlockNumber: tx.blockNumber,
            currentBlock: currentBlock
        )
        let direction = determineDirection(tx: tx, address: address)

        return TransactionDetails(
            id: tx.hash,
            hash: tx.hash,
            blockNumber: tx.blockNumber,
            timestamp: tx.timestamp ?? Date(),
            from: tx.from,
            to: tx.to ?? "0x0",
            value: tx.value,
            gasLimit: tx.gas,
            gasUsed: receipt?.gasUsed,
            gasPrice: receipt?.effectiveGasPrice ?? tx.gasPrice ?? BigInt(0),
            nonce: tx.nonce,
            data: tx.input,
            status: status,
            confirmations: confirmations,
            direction: direction,
            maxFeePerGas: tx.maxFeePerGas,
            maxPriorityFeePerGas: tx.maxPriorityFeePerGas,
            baseFeePerGas: nil
        )
    }

    /// Determine transaction status
    private func determineStatus(tx: Web3Transaction, receipt: TransactionReceipt?) -> TransactionStatus {
        guard let receipt = receipt else {
            return .pending
        }

        return receipt.isSuccess ? .confirmed : .failed
    }

    /// Calculate transaction confirmations
    private func calculateConfirmations(txBlockNumber: Int?, currentBlock: Int) -> Int {
        guard let blockNumber = txBlockNumber else {
            return 0
        }
        return max(0, currentBlock - blockNumber + 1)
    }

    /// Determine transaction direction
    private func determineDirection(tx: Web3Transaction, address: String) -> TransactionDirection {
        let fromLower = tx.from.lowercased()
        let toLower = (tx.to ?? "").lowercased()
        let addressLower = address.lowercased()

        if fromLower == addressLower && toLower == addressLower {
            return .self_
        } else if fromLower == addressLower {
            return .outgoing
        } else {
            return .incoming
        }
    }

    /// Paginate transactions
    private func paginateTransactions(
        _ transactions: [TransactionDetails],
        page: Int,
        limit: Int
    ) -> TransactionPage {
        let startIndex = page * limit
        let endIndex = min(startIndex + limit, transactions.count)

        guard startIndex < transactions.count else {
            return TransactionPage(transactions: [], hasMore: false, nextPage: nil)
        }

        let pageTransactions = Array(transactions[startIndex..<endIndex])
        let hasMore = endIndex < transactions.count
        let nextPage = hasMore ? page + 1 : nil

        return TransactionPage(
            transactions: pageTransactions,
            hasMore: hasMore,
            nextPage: nextPage
        )
    }

    /// Validate Ethereum address
    private func isValidAddress(_ address: String) -> Bool {
        let cleaned = address.lowercased().replacingOccurrences(of: "0x", with: "")
        return cleaned.count == 40 && cleaned.allSatisfy { $0.isHexDigit }
    }

    // MARK: - Cache Management

    /// Clear cache for address
    func clearCache(for address: String) {
        historyCache.removeValue(forKey: address)
    }

    /// Clear all cache
    func clearAllCache() {
        historyCache.removeAll()
    }
}

// MARK: - Supporting Types

/// Web3 transaction from blockchain
struct Web3Transaction: Codable {
    let hash: String
    let blockNumber: Int?
    let from: String
    let to: String?
    let value: BigInt
    let gas: BigInt
    let gasPrice: BigInt?
    let nonce: Int
    let input: String
    let timestamp: Date?

    // EIP-1559 fields
    let maxFeePerGas: BigInt?
    let maxPriorityFeePerGas: BigInt?
}

/// Block information
struct BlockInfo: Codable {
    let number: Int
    let hash: String
    let timestamp: Date
    let transactions: [String]
    let baseFeePerGas: BigInt?
}
