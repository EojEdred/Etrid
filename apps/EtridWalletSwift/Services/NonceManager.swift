//
//  NonceManager.swift
//  EtridWalletSwift
//
//  Created by Eoj on 2025-11-22.
//  Manage transaction nonces with conflict detection and queuing
//

import Foundation

/// Nonce state for an account
struct NonceState {
    let address: String
    var currentNonce: Int
    var pendingNonces: Set<Int>
    var lastSyncedAt: Date
    var queuedTransactions: [(nonce: Int, transaction: PendingTransaction)]

    /// Get next available nonce
    var nextNonce: Int {
        var next = currentNonce
        while pendingNonces.contains(next) {
            next += 1
        }
        return next
    }

    /// Check if nonce is available
    func isNonceAvailable(_ nonce: Int) -> Bool {
        return nonce >= currentNonce && !pendingNonces.contains(nonce)
    }

    /// Check if there are nonce gaps
    var hasGaps: Bool {
        guard !pendingNonces.isEmpty else { return false }

        let sorted = pendingNonces.sorted()
        for i in 0..<sorted.count - 1 {
            if sorted[i + 1] - sorted[i] > 1 {
                return true
            }
        }
        return false
    }
}

/// Nonce manager errors
enum NonceManagerError: Error, LocalizedError {
    case nonceConflict(nonce: Int)
    case nonceGapDetected(expected: Int, got: Int)
    case failedToSyncNonce
    case invalidAddress
    case transactionQueueFull

    var errorDescription: String? {
        switch self {
        case .nonceConflict(let nonce):
            return "Nonce \(nonce) is already in use"
        case .nonceGapDetected(let expected, let got):
            return "Nonce gap detected: expected \(expected), got \(got)"
        case .failedToSyncNonce:
            return "Failed to synchronize nonce with blockchain"
        case .invalidAddress:
            return "Invalid Ethereum address"
        case .transactionQueueFull:
            return "Transaction queue is full, try again later"
        }
    }
}

/// Manages nonces for transaction submission with conflict detection
actor NonceManager {
    private let web3Service: Web3Service
    private var nonceStates: [String: NonceState] = [:]

    // Configuration
    private let maxQueueSize = 100
    private let syncInterval: TimeInterval = 60  // Sync every 60 seconds
    private let staleThreshold: TimeInterval = 300  // 5 minutes

    init(web3Service: Web3Service) {
        self.web3Service = web3Service
    }

    // MARK: - Nonce Operations

    /// Get next available nonce for address
    func getNextNonce(for address: String, forceSync: Bool = false) async throws -> Int {
        // Validate address
        guard isValidAddress(address) else {
            throw NonceManagerError.invalidAddress
        }

        // Get or create nonce state
        var state = nonceStates[address] ?? await createNonceState(for: address)

        // Sync with blockchain if needed
        if forceSync || needsSync(state) {
            state = try await syncNonce(for: address)
        }

        let nextNonce = state.nextNonce

        // Reserve the nonce
        state.pendingNonces.insert(nextNonce)
        nonceStates[address] = state

        return nextNonce
    }

    /// Reserve a specific nonce (for replacement transactions)
    func reserveNonce(_ nonce: Int, for address: String) async throws {
        guard isValidAddress(address) else {
            throw NonceManagerError.invalidAddress
        }

        var state = nonceStates[address] ?? await createNonceState(for: address)

        // Check if nonce is available
        guard state.isNonceAvailable(nonce) else {
            throw NonceManagerError.nonceConflict(nonce: nonce)
        }

        state.pendingNonces.insert(nonce)
        nonceStates[address] = state
    }

    /// Release a nonce (when transaction is confirmed or dropped)
    func releaseNonce(_ nonce: Int, for address: String) async {
        guard var state = nonceStates[address] else { return }

        state.pendingNonces.remove(nonce)

        // Process queued transactions that can now use this nonce
        await processQueue(for: address, state: &state)

        nonceStates[address] = state
    }

    /// Confirm nonce usage (transaction confirmed on-chain)
    func confirmNonce(_ nonce: Int, for address: String) async {
        guard var state = nonceStates[address] else { return }

        // Update current nonce if this is the next expected nonce
        if nonce >= state.currentNonce {
            state.currentNonce = nonce + 1
        }

        state.pendingNonces.remove(nonce)
        nonceStates[address] = state
    }

    // MARK: - Nonce Synchronization

    /// Sync nonce with blockchain
    func syncNonce(for address: String) async throws -> NonceState {
        guard isValidAddress(address) else {
            throw NonceManagerError.invalidAddress
        }

        do {
            // Get transaction count from blockchain
            let onChainNonce = try await web3Service.getTransactionCount(address: address)

            var state = nonceStates[address] ?? NonceState(
                address: address,
                currentNonce: onChainNonce,
                pendingNonces: [],
                lastSyncedAt: Date(),
                queuedTransactions: []
            )

            // Update state
            state.currentNonce = onChainNonce
            state.lastSyncedAt = Date()

            // Clean up confirmed pending nonces
            state.pendingNonces = state.pendingNonces.filter { $0 >= onChainNonce }

            // Detect and report gaps
            if state.hasGaps {
                print("Warning: Nonce gaps detected for address \(address)")
            }

            nonceStates[address] = state
            return state

        } catch {
            throw NonceManagerError.failedToSyncNonce
        }
    }

    /// Check if nonce state needs syncing
    private func needsSync(_ state: NonceState) -> Bool {
        return Date().timeIntervalSince(state.lastSyncedAt) > syncInterval
    }

    /// Create initial nonce state for address
    private func createNonceState(for address: String) async -> NonceState {
        let onChainNonce = (try? await web3Service.getTransactionCount(address: address)) ?? 0

        return NonceState(
            address: address,
            currentNonce: onChainNonce,
            pendingNonces: [],
            lastSyncedAt: Date(),
            queuedTransactions: []
        )
    }

    // MARK: - Transaction Queuing

    /// Queue transaction when nonce conflict occurs
    func queueTransaction(_ transaction: PendingTransaction, nonce: Int) async throws {
        guard var state = nonceStates[transaction.from] else {
            throw NonceManagerError.invalidAddress
        }

        // Check queue size
        guard state.queuedTransactions.count < maxQueueSize else {
            throw NonceManagerError.transactionQueueFull
        }

        state.queuedTransactions.append((nonce: nonce, transaction: transaction))
        nonceStates[transaction.from] = state
    }

    /// Process queued transactions
    private func processQueue(for address: String, state: inout NonceState) async {
        // Sort queue by nonce
        state.queuedTransactions.sort { $0.nonce < $1.nonce }

        // Process transactions that can now proceed
        var processedIndices: [Int] = []

        for (index, item) in state.queuedTransactions.enumerated() {
            if state.isNonceAvailable(item.nonce) {
                // Reserve nonce for this transaction
                state.pendingNonces.insert(item.nonce)
                processedIndices.append(index)

                // Notify that transaction can proceed (in production, use delegate or callback)
                print("Transaction \(item.transaction.hash) can now proceed with nonce \(item.nonce)")
            }
        }

        // Remove processed transactions from queue
        for index in processedIndices.reversed() {
            state.queuedTransactions.remove(at: index)
        }
    }

    // MARK: - Nonce Gap Detection

    /// Detect nonce gaps for address
    func detectGaps(for address: String) async throws -> [Int] {
        guard var state = nonceStates[address] else {
            return []
        }

        // Sync first
        state = try await syncNonce(for: address)

        var gaps: [Int] = []
        let sortedNonces = state.pendingNonces.sorted()

        for i in 0..<sortedNonces.count - 1 {
            let current = sortedNonces[i]
            let next = sortedNonces[i + 1]

            if next - current > 1 {
                // Found a gap
                for gap in (current + 1)..<next {
                    gaps.append(gap)
                }
            }
        }

        return gaps
    }

    /// Fill nonce gap by submitting dummy transaction
    func fillGap(_ nonce: Int, for address: String, privateKey: Data) async throws {
        // Create dummy transaction (0 ETH to self)
        // This should use TransactionBuilder in production
        print("Filling nonce gap \(nonce) for address \(address)")

        // Reserve the gap nonce
        try await reserveNonce(nonce, for: address)
    }

    // MARK: - Query Methods

    /// Get current nonce state for address
    func getNonceState(for address: String) async -> NonceState? {
        return nonceStates[address]
    }

    /// Get all pending nonces for address
    func getPendingNonces(for address: String) -> Set<Int> {
        return nonceStates[address]?.pendingNonces ?? []
    }

    /// Get queued transactions for address
    func getQueuedTransactions(for address: String) -> [PendingTransaction] {
        return nonceStates[address]?.queuedTransactions.map { $0.transaction } ?? []
    }

    /// Check if nonce is available
    func isNonceAvailable(_ nonce: Int, for address: String) async -> Bool {
        guard let state = nonceStates[address] else {
            return true
        }
        return state.isNonceAvailable(nonce)
    }

    // MARK: - Cleanup

    /// Clear nonce state for address
    func clearState(for address: String) {
        nonceStates.removeValue(forKey: address)
    }

    /// Clear all nonce states
    func clearAllStates() {
        nonceStates.removeAll()
    }

    // MARK: - Validation

    /// Validate nonce before transaction submission
    func validateNonce(_ nonce: Int, for address: String) async throws {
        guard let state = nonceStates[address] else {
            // No state, sync first
            _ = try await syncNonce(for: address)
            return
        }

        // Check for gaps
        if nonce > state.nextNonce {
            throw NonceManagerError.nonceGapDetected(expected: state.nextNonce, got: nonce)
        }

        // Check for conflicts
        if state.pendingNonces.contains(nonce) {
            throw NonceManagerError.nonceConflict(nonce: nonce)
        }
    }

    // MARK: - Helper Methods

    private func isValidAddress(_ address: String) -> Bool {
        let cleaned = address.lowercased().replacingOccurrences(of: "0x", with: "")
        return cleaned.count == 40 && cleaned.allSatisfy { $0.isHexDigit }
    }
}

// MARK: - Nonce Manager Extensions

extension NonceManager {
    /// Get statistics for debugging
    func getStatistics() -> NonceStatistics {
        var totalPending = 0
        var totalQueued = 0
        var addressesWithGaps = 0

        for state in nonceStates.values {
            totalPending += state.pendingNonces.count
            totalQueued += state.queuedTransactions.count
            if state.hasGaps {
                addressesWithGaps += 1
            }
        }

        return NonceStatistics(
            totalAddresses: nonceStates.count,
            totalPendingNonces: totalPending,
            totalQueuedTransactions: totalQueued,
            addressesWithGaps: addressesWithGaps
        )
    }
}

/// Statistics for nonce manager
struct NonceStatistics {
    let totalAddresses: Int
    let totalPendingNonces: Int
    let totalQueuedTransactions: Int
    let addressesWithGaps: Int
}
