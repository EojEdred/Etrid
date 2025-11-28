//
//  TransactionManager.swift
//  EtridWallet
//
//  Transaction building and signing manager
//

import Foundation

actor TransactionManager {
    // MARK: - Singleton
    static let shared = TransactionManager()

    private let crypto = CryptoManager.shared
    private let storage = WalletStorage.shared

    private init() {}

    // MARK: - Build Transaction
    /// Builds complete transaction ready for signing
    /// - Parameters:
    ///   - from: Sender address
    ///   - to: Recipient address
    ///   - value: Amount in Wei
    ///   - network: Target network
    ///   - data: Transaction data (optional)
    /// - Returns: Transaction request
    func buildTransaction(
        from: String,
        to: String,
        value: String,
        network: Network,
        data: String? = nil
    ) async throws -> TransactionRequest {
        let web3 = Web3Service(network: network)

        return try await web3.buildTransaction(
            from: from,
            to: to,
            value: value,
            data: data
        )
    }

    // MARK: - Sign Transaction
    /// Signs transaction with private key
    /// - Parameters:
    ///   - transaction: Transaction to sign
    ///   - privateKey: Private key for signing
    /// - Returns: Signed transaction data
    func signTransaction(
        _ transaction: TransactionRequest,
        privateKey: Data
    ) async throws -> SignedTransaction {
        // Build transaction hash
        let txHash = try buildTransactionHash(transaction)

        // Sign hash
        let signature = try await crypto.signTransactionHash(txHash, privateKey: privateKey)

        // Build raw transaction (RLP encoded in production)
        let rawTransaction = try buildRawTransaction(transaction, signature: signature)

        // Calculate transaction hash
        let hash = Data(SHA256.hash(data: Data(rawTransaction.utf8))).hexStringWithPrefix

        return SignedTransaction(
            rawTransaction: rawTransaction,
            hash: hash,
            transaction: transaction
        )
    }

    /// Signs transaction for account
    /// - Parameters:
    ///   - transaction: Transaction to sign
    ///   - address: Account address
    /// - Returns: Signed transaction
    func signTransaction(
        _ transaction: TransactionRequest,
        for address: String
    ) async throws -> SignedTransaction {
        guard let privateKey = try await storage.loadPrivateKey(for: address) else {
            throw WalletError.invalidPrivateKey
        }

        return try await signTransaction(transaction, privateKey: privateKey)
    }

    // MARK: - Send Transaction
    /// Signs and sends transaction
    /// - Parameters:
    ///   - transaction: Transaction to send
    ///   - address: Sender address
    ///   - network: Network to send on
    /// - Returns: Transaction hash
    func sendTransaction(
        _ transaction: TransactionRequest,
        from address: String,
        network: Network
    ) async throws -> String {
        // Sign transaction
        let signed = try await signTransaction(transaction, for: address)

        // Send to network
        let web3 = Web3Service(network: network)
        let hash = try await web3.sendRawTransaction(signed.rawTransaction)

        // Save to history
        let tx = Transaction(
            hash: hash,
            from: transaction.from,
            to: transaction.to,
            value: transaction.value,
            data: transaction.data,
            gasLimit: transaction.gasLimit ?? "21000",
            gasPrice: transaction.gasPrice ?? "0",
            nonce: transaction.nonce ?? 0,
            network: network.name
        )

        try await storage.addTransaction(tx)

        return hash
    }

    // MARK: - Transaction Monitoring
    /// Monitors transaction until confirmed
    /// - Parameters:
    ///   - hash: Transaction hash
    ///   - network: Network
    ///   - confirmations: Required confirmations
    /// - Returns: Final transaction
    func monitorTransaction(
        hash: String,
        network: Network,
        confirmations: Int = 12
    ) async throws -> Transaction {
        let web3 = Web3Service(network: network)

        // Wait for transaction to be mined
        let receipt = try await web3.waitForTransaction(hash: hash)

        // Get current block
        let currentBlock = try await web3.getBlockNumber()

        guard let blockNumberHex = receipt["blockNumber"] as? String,
              let blockNumber = BigInt.fromHex(blockNumberHex) else {
            throw WalletError.invalidResponse
        }

        let txConfirmations = currentBlock - Int(blockNumber)

        // Update transaction status
        let status: TransactionStatus
        if let statusHex = receipt["status"] as? String, statusHex == "0x1" {
            status = txConfirmations >= confirmations ? .confirmed : .pending
        } else {
            status = .failed
        }

        try await storage.updateTransactionStatus(hash, status: status)

        // Get updated transaction
        let transactions = try await storage.loadTransactions()
        guard let transaction = transactions.first(where: { $0.hash == hash }) else {
            throw WalletError.transactionNotFound
        }

        return transaction
    }

    // MARK: - Helper Methods
    private func buildTransactionHash(_ transaction: TransactionRequest) throws -> Data {
        // RLP encode transaction for signing
        let encoded: Data

        if let maxFeePerGas = transaction.maxFeePerGas,
           let maxPriorityFeePerGas = transaction.maxPriorityFeePerGas {
            // EIP-1559 transaction
            encoded = RLPEncoder.encodeEIP1559Transaction(
                chainId: transaction.chainId,
                nonce: transaction.nonce ?? 0,
                maxPriorityFeePerGas: maxPriorityFeePerGas,
                maxFeePerGas: maxFeePerGas,
                gasLimit: transaction.gasLimit ?? "21000",
                to: transaction.to,
                value: transaction.value,
                data: transaction.data
            )
        } else {
            // Legacy transaction
            let gasPrice = transaction.gasPrice ?? transaction.maxFeePerGas ?? "0x0"
            encoded = RLPEncoder.encodeLegacyTransaction(
                nonce: transaction.nonce ?? 0,
                gasPrice: gasPrice,
                gasLimit: transaction.gasLimit ?? "21000",
                to: transaction.to,
                value: transaction.value,
                data: transaction.data,
                chainId: transaction.chainId
            )
        }

        // Hash the RLP-encoded transaction
        return encoded.keccak256Hash
    }

    private func buildRawTransaction(
        _ transaction: TransactionRequest,
        signature: Data
    ) throws -> String {
        // Extract r, s, v from signature (ECDSA signature is 65 bytes: r(32) + s(32) + v(1))
        guard signature.count == 65 else {
            throw WalletError.signatureFailed
        }

        let r = signature.prefix(32)
        let s = signature.dropFirst(32).prefix(32)
        var v = signature.last!

        // Adjust v for EIP-155
        v = v + UInt8(transaction.chainId * 2 + 35)

        let encoded: Data

        if let maxFeePerGas = transaction.maxFeePerGas,
           let maxPriorityFeePerGas = transaction.maxPriorityFeePerGas {
            // EIP-1559 transaction
            encoded = RLPEncoder.encodeSignedEIP1559Transaction(
                chainId: transaction.chainId,
                nonce: transaction.nonce ?? 0,
                maxPriorityFeePerGas: maxPriorityFeePerGas,
                maxFeePerGas: maxFeePerGas,
                gasLimit: transaction.gasLimit ?? "21000",
                to: transaction.to,
                value: transaction.value,
                data: transaction.data,
                v: v,
                r: r,
                s: s
            )
        } else {
            // Legacy transaction
            let gasPrice = transaction.gasPrice ?? transaction.maxFeePerGas ?? "0x0"
            encoded = RLPEncoder.encodeSignedLegacyTransaction(
                nonce: transaction.nonce ?? 0,
                gasPrice: gasPrice,
                gasLimit: transaction.gasLimit ?? "21000",
                to: transaction.to,
                value: transaction.value,
                data: transaction.data,
                v: v,
                r: r,
                s: s
            )
        }

        return encoded.hexStringWithPrefix
    }

    // MARK: - Fee Estimation
    /// Estimates total transaction cost
    /// - Parameter transaction: Transaction request
    /// - Returns: Total cost in Wei
    func estimateTotalCost(_ transaction: TransactionRequest) -> String {
        let value = BigInt.fromHex(transaction.value) ?? 0

        guard let gasLimit = transaction.gasLimit,
              let gasPrice = transaction.gasPrice,
              let limitValue = BigInt.fromHex(gasLimit),
              let priceValue = BigInt.fromHex(gasPrice) else {
            return BigInt.toHex(value)
        }

        let gasCost = limitValue * priceValue
        let total = value + gasCost

        return BigInt.toHex(total)
    }

    /// Checks if account has sufficient balance
    /// - Parameters:
    ///   - transaction: Transaction to check
    ///   - balance: Account balance in Wei
    /// - Returns: True if sufficient
    func hasSufficientBalance(
        for transaction: TransactionRequest,
        balance: String
    ) -> Bool {
        guard let balanceValue = BigInt.fromHex(balance) else {
            return false
        }

        let totalCost = estimateTotalCost(transaction)
        guard let totalValue = BigInt.fromHex(totalCost) else {
            return false
        }

        return balanceValue >= totalValue
    }
}

// MARK: - Transaction Builder Helper
struct TransactionBuilder {
    var from: String
    var to: String
    var value: String = "0x0"
    var data: String?
    var gasLimit: String?
    var gasPrice: String?
    var maxFeePerGas: String?
    var maxPriorityFeePerGas: String?
    var nonce: Int?
    var chainId: Int

    func build() -> TransactionRequest {
        TransactionRequest(
            from: from,
            to: to,
            value: value,
            data: data,
            gasLimit: gasLimit,
            gasPrice: gasPrice,
            maxFeePerGas: maxFeePerGas,
            maxPriorityFeePerGas: maxPriorityFeePerGas,
            nonce: nonce,
            chainId: chainId
        )
    }

    mutating func setEtherValue(_ ether: Double) {
        value = WeiConverter.etherToWei(ether)
    }

    mutating func setTokenTransfer(token: String, to recipient: String, amount: String) {
        to = token
        data = ERC20ABI.encodeTransfer(to: recipient, amount: amount)
        value = "0x0"
    }
}

import CryptoKit
