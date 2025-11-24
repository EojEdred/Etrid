//
//  Web3Service.swift
//  EtridWallet
//
//  Web3 blockchain interaction service with real RPC connectivity
//
//  Features:
//  - Real Ethereum JSON-RPC calls to public nodes
//  - Automatic retry logic with exponential backoff
//  - Timeout handling (30s per request)
//  - Rate limit detection and handling
//  - EIP-1559 and legacy transaction support
//  - Gas estimation and fee calculation
//

import Foundation

actor Web3Service {
    // MARK: - Properties
    private var rpcProvider: RPCProvider
    private let network: Network

    // MARK: - Singleton
    static let shared = Web3Service(network: .ethereum)

    init(network: Network) {
        self.network = network
        self.rpcProvider = RPCProvider(url: network.rpcURL)
    }

    // MARK: - Network Configuration
    /// Updates RPC provider for new network
    func updateNetwork(_ network: Network) {
        self.rpcProvider = RPCProvider(url: network.rpcURL)
    }

    // MARK: - Balance Operations
    /// Gets native token balance
    /// - Parameter address: Wallet address
    /// - Returns: Balance in Wei as string
    func getBalance(address: String) async throws -> String {
        guard address.isValidEthereumAddress else {
            throw WalletError.invalidAddress
        }

        let balance: String = try await rpcProvider.getBalance(address: address)
        return balance
    }

    /// Gets balance in Ether
    /// - Parameter address: Wallet address
    /// - Returns: Balance as Double
    func getBalanceInEther(address: String) async throws -> Double {
        let weiBalance = try await getBalance(address: address)
        return WeiConverter.weiToEther(weiBalance)
    }

    // MARK: - Transaction Operations
    /// Gets transaction count (nonce) for address
    /// - Parameter address: Wallet address
    /// - Returns: Nonce as integer
    func getTransactionCount(address: String) async throws -> Int {
        guard address.isValidEthereumAddress else {
            throw WalletError.invalidAddress
        }

        let nonce: String = try await rpcProvider.getTransactionCount(address: address)
        guard let nonceValue = BigInt.fromHex(nonce) else {
            throw WalletError.invalidResponse
        }

        return Int(nonceValue)
    }

    /// Gets current gas price
    /// - Returns: Gas price in Wei
    func getGasPrice() async throws -> String {
        try await rpcProvider.getGasPrice()
    }

    /// Gets EIP-1559 fee data
    /// - Returns: Tuple of (maxFeePerGas, maxPriorityFeePerGas)
    func getFeeData() async throws -> (maxFeePerGas: String, maxPriorityFeePerGas: String) {
        // Get fee history for last 10 blocks
        let feeHistory = try await rpcProvider.getFeeHistory(
            blockCount: 10,
            newestBlock: "latest",
            rewardPercentiles: [25, 50, 75]
        )

        guard let result = feeHistory["result"] as? [String: Any],
              let baseFeeArray = result["baseFeePerGas"] as? [String],
              let baseFee = baseFeeArray.last else {
            // Fallback to gas price
            let gasPrice = try await getGasPrice()
            return (gasPrice, gasPrice)
        }

        // Calculate maxFeePerGas (2x base fee + priority fee)
        guard let baseFeeValue = BigInt.fromHex(baseFee) else {
            throw WalletError.invalidResponse
        }

        let priorityFee: UInt64 = 2_000_000_000 // 2 Gwei
        let maxFeePerGas = (baseFeeValue * 2) + priorityFee
        let maxPriorityFeePerGas = priorityFee

        return (
            BigInt.toHex(maxFeePerGas),
            BigInt.toHex(maxPriorityFeePerGas)
        )
    }

    /// Estimates gas for transaction
    /// - Parameter params: Transaction parameters
    /// - Returns: Estimated gas limit
    func estimateGas(params: [String: Any]) async throws -> String {
        try await rpcProvider.estimateGas(params: params)
    }

    /// Estimates gas for simple transfer
    /// - Parameters:
    ///   - from: Sender address
    ///   - to: Recipient address
    ///   - value: Amount in Wei
    /// - Returns: Estimated gas limit
    func estimateGas(from: String, to: String, value: String) async throws -> String {
        let params: [String: Any] = [
            "from": from,
            "to": to,
            "value": value
        ]
        return try await estimateGas(params: params)
    }

    /// Sends raw signed transaction
    /// - Parameter signedTx: Signed transaction hex string
    /// - Returns: Transaction hash
    func sendRawTransaction(_ signedTx: String) async throws -> String {
        let hash: String = try await rpcProvider.sendRawTransaction(signedTx: signedTx)
        return hash
    }

    /// Gets transaction by hash
    /// - Parameter hash: Transaction hash
    /// - Returns: Transaction data
    func getTransaction(hash: String) async throws -> [String: Any] {
        guard hash.isValidTransactionHash else {
            throw WalletError.invalidInput("Invalid transaction hash")
        }

        let tx = try await rpcProvider.getTransaction(hash: hash)
        guard let result = tx["result"] as? [String: Any] else {
            throw WalletError.transactionNotFound
        }

        return result
    }

    /// Gets transaction receipt
    /// - Parameter hash: Transaction hash
    /// - Returns: Receipt data or nil if pending
    func getTransactionReceipt(hash: String) async throws -> [String: Any]? {
        guard hash.isValidTransactionHash else {
            throw WalletError.invalidInput("Invalid transaction hash")
        }

        return try await rpcProvider.getTransactionReceipt(hash: hash)
    }

    /// Waits for transaction to be mined
    /// - Parameters:
    ///   - hash: Transaction hash
    ///   - timeout: Maximum wait time in seconds
    ///   - interval: Check interval in seconds
    /// - Returns: Transaction receipt
    func waitForTransaction(hash: String, timeout: TimeInterval = 120, interval: TimeInterval = 2) async throws -> [String: Any] {
        let startTime = Date()

        while Date().timeIntervalSince(startTime) < timeout {
            if let receipt = try await getTransactionReceipt(hash: hash) {
                return receipt
            }

            try await Task.sleep(nanoseconds: UInt64(interval * 1_000_000_000))
        }

        throw WalletError.timeout
    }

    // MARK: - Contract Calls
    /// Calls contract method (read-only)
    /// - Parameters:
    ///   - to: Contract address
    ///   - data: Encoded function call
    /// - Returns: Response data
    func call(to: String, data: String) async throws -> String {
        guard to.isValidEthereumAddress else {
            throw WalletError.invalidAddress
        }

        return try await rpcProvider.call(to: to, data: data)
    }

    /// Gets ERC20 token balance
    /// - Parameters:
    ///   - tokenAddress: Token contract address
    ///   - walletAddress: Wallet address
    /// - Returns: Balance in token's smallest unit
    func getTokenBalance(tokenAddress: String, walletAddress: String) async throws -> String {
        let data = ERC20ABI.encodeBalanceOf(address: walletAddress)
        let result = try await call(to: tokenAddress, data: data)

        guard let balance = ERC20ABI.decodeBalance(data: result) else {
            throw WalletError.invalidResponse
        }

        return balance
    }

    // MARK: - Block Operations
    /// Gets current block number
    /// - Returns: Block number
    func getBlockNumber() async throws -> Int {
        let blockHex: String = try await rpcProvider.getBlockNumber()
        guard let blockNumber = BigInt.fromHex(blockHex) else {
            throw WalletError.invalidResponse
        }
        return Int(blockNumber)
    }

    /// Gets block by number
    /// - Parameters:
    ///   - number: Block number (or "latest")
    ///   - fullTransactions: Include full transaction data
    /// - Returns: Block data
    func getBlock(number: String = "latest", fullTransactions: Bool = false) async throws -> [String: Any] {
        let block = try await rpcProvider.getBlock(number: number, fullTransactions: fullTransactions)
        guard let result = block["result"] as? [String: Any] else {
            throw WalletError.invalidResponse
        }
        return result
    }

    // MARK: - Network Info
    /// Gets chain ID
    /// - Returns: Chain ID
    func getChainId() async throws -> Int {
        let chainIdHex: String = try await rpcProvider.getChainId()
        guard let chainId = BigInt.fromHex(chainIdHex) else {
            throw WalletError.invalidResponse
        }
        return Int(chainId)
    }

    /// Verifies network matches expected chain ID
    /// - Parameter expectedChainId: Expected chain ID
    /// - Returns: True if matches
    func verifyNetwork(expectedChainId: Int) async throws -> Bool {
        let chainId = try await getChainId()
        return chainId == expectedChainId
    }

    // MARK: - Transaction Building
    /// Builds transaction parameters
    /// - Parameters:
    ///   - from: Sender address
    ///   - to: Recipient address
    ///   - value: Amount in Wei
    ///   - data: Transaction data (optional)
    ///   - gasLimit: Gas limit (optional, will estimate if nil)
    /// - Returns: Complete transaction parameters
    func buildTransaction(
        from: String,
        to: String,
        value: String,
        data: String? = nil,
        gasLimit: String? = nil
    ) async throws -> TransactionRequest {
        // Get nonce
        let nonce = try await getTransactionCount(address: from)

        // Estimate gas if not provided
        let estimatedGas: String
        if let gasLimit = gasLimit {
            estimatedGas = gasLimit
        } else {
            var params: [String: Any] = [
                "from": from,
                "to": to,
                "value": value
            ]
            if let data = data {
                params["data"] = data
            }
            estimatedGas = try await estimateGas(params: params)
        }

        // Get gas price / fee data
        let (maxFeePerGas, maxPriorityFeePerGas) = try await getFeeData()

        return TransactionRequest(
            from: from,
            to: to,
            value: value,
            data: data,
            gasLimit: estimatedGas,
            maxFeePerGas: maxFeePerGas,
            maxPriorityFeePerGas: maxPriorityFeePerGas,
            nonce: nonce,
            chainId: network.chainId
        )
    }
}
