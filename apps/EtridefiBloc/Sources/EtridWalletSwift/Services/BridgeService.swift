import Foundation
import Combine

// MARK: - Bridge Service

public actor BridgeService {

    // Bridge configuration per chain
    private let bridgeConfigs: [SupportedChain: BridgeConfig] = [
        .solana: BridgeConfig(
            chain: .solana,
            bridgeAddress: "ETRid1111111111111111111111111111111111111",
            requiredConfirmations: 31,
            estimatedTime: 60, // seconds
            feePercent: 0.1,
            minAmount: Decimal(string: "0.01")!,
            maxAmount: Decimal(string: "10000")!
        ),
        .bnbChain: BridgeConfig(
            chain: .bnbChain,
            bridgeAddress: "0xETRidBridgeBNB000000000000000000000000",
            requiredConfirmations: 15,
            estimatedTime: 45,
            feePercent: 0.1,
            minAmount: Decimal(string: "0.001")!,
            maxAmount: Decimal(string: "1000")!
        ),
        .xrp: BridgeConfig(
            chain: .xrp,
            bridgeAddress: "rETRidBridgeXRP11111111111111111",
            requiredConfirmations: 1, // Instant finality
            estimatedTime: 5,
            feePercent: 0.1,
            minAmount: Decimal(string: "1")!,
            maxAmount: Decimal(string: "100000")!
        ),
        .tron: BridgeConfig(
            chain: .tron,
            bridgeAddress: "TETRidBridgeTRX11111111111111111",
            requiredConfirmations: 19,
            estimatedTime: 60,
            feePercent: 0.1,
            minAmount: Decimal(string: "1")!,
            maxAmount: Decimal(string: "100000")!
        ),
        .cardano: BridgeConfig(
            chain: .cardano,
            bridgeAddress: "addr1_etrid_bridge_cardano",
            requiredConfirmations: 15,
            estimatedTime: 300, // 5 minutes
            feePercent: 0.15,
            minAmount: Decimal(string: "1")!,
            maxAmount: Decimal(string: "100000")!
        ),
        .stellar: BridgeConfig(
            chain: .stellar,
            bridgeAddress: "GETRIDBRIDGESTELLAR111111111111111111",
            requiredConfirmations: 1,
            estimatedTime: 5,
            feePercent: 0.1,
            minAmount: Decimal(string: "1")!,
            maxAmount: Decimal(string: "100000")!
        ),
        .dogecoin: BridgeConfig(
            chain: .dogecoin,
            bridgeAddress: "DETRidBridgeDOGE1111111111111111",
            requiredConfirmations: 6,
            estimatedTime: 60,
            feePercent: 0.1,
            minAmount: Decimal(string: "10")!,
            maxAmount: Decimal(string: "1000000")!
        )
    ]

    private let rpcService: RPCService
    private var pendingDeposits: [String: BridgeDeposit] = [:]
    private var pendingWithdrawals: [String: BridgeWithdrawal] = [:]

    public init(rpcService: RPCService) {
        self.rpcService = rpcService
    }

    // MARK: - Bridge Information

    /// Get bridge configuration for a specific chain
    public func getBridgeConfig(for chain: SupportedChain) -> BridgeConfig? {
        return bridgeConfigs[chain]
    }

    /// Get all supported bridge chains
    public func supportedChains() -> [SupportedChain] {
        return Array(bridgeConfigs.keys)
    }

    /// Calculate bridge fee for a given amount
    public func calculateFee(amount: Decimal, chain: SupportedChain) -> BridgeFeeEstimate? {
        guard let config = bridgeConfigs[chain] else { return nil }

        let feeAmount = amount * Decimal(config.feePercent) / 100
        let netAmount = amount - feeAmount

        return BridgeFeeEstimate(
            grossAmount: amount,
            feeAmount: feeAmount,
            feePercent: config.feePercent,
            netAmount: netAmount,
            estimatedTime: config.estimatedTime,
            requiredConfirmations: config.requiredConfirmations
        )
    }

    /// Validate deposit amount
    public func validateDepositAmount(_ amount: Decimal, chain: SupportedChain) -> BridgeValidationResult {
        guard let config = bridgeConfigs[chain] else {
            return .failure("Unsupported chain")
        }

        if amount < config.minAmount {
            return .failure("Minimum deposit is \(config.minAmount) \(chain.symbol)")
        }

        if amount > config.maxAmount {
            return .failure("Maximum deposit is \(config.maxAmount) \(chain.symbol)")
        }

        return .success
    }

    // MARK: - Deposit Flow

    /// Get bridge deposit address for a specific chain
    public func getDepositAddress(for chain: SupportedChain, etridAddress: String) async throws -> BridgeDepositInfo {
        guard let config = bridgeConfigs[chain] else {
            throw BridgeError.unsupportedChain
        }

        // In production, this would generate a unique deposit address or memo
        // linked to the user's ËTRID address
        let depositInfo = BridgeDepositInfo(
            chain: chain,
            depositAddress: config.bridgeAddress,
            memo: generateDepositMemo(etridAddress: etridAddress),
            etridRecipient: etridAddress,
            requiredConfirmations: config.requiredConfirmations,
            estimatedTime: config.estimatedTime,
            feePercent: config.feePercent,
            minAmount: config.minAmount,
            maxAmount: config.maxAmount
        )

        return depositInfo
    }

    /// Initiate a deposit (called after user sends funds to bridge address)
    public func initiateDeposit(
        sourceChain: SupportedChain,
        sourceTxHash: String,
        sourceAddress: String,
        targetAddress: String,
        amount: Decimal
    ) async throws -> BridgeDeposit {
        guard let config = bridgeConfigs[sourceChain] else {
            throw BridgeError.unsupportedChain
        }

        // Validate amount
        let validation = validateDepositAmount(amount, chain: sourceChain)
        if case .failure(let message) = validation {
            throw BridgeError.invalidAmount(message)
        }

        // Create deposit record
        let deposit = BridgeDeposit(
            sourceChain: sourceChain,
            sourceAddress: sourceAddress,
            sourceTxHash: sourceTxHash,
            targetAddress: targetAddress,
            amount: amount,
            token: bridgedToken(for: sourceChain),
            status: .pending,
            confirmations: 0,
            requiredConfirmations: config.requiredConfirmations
        )

        pendingDeposits[deposit.id] = deposit

        // Submit to ËTRID bridge pallet
        try await submitDepositToChain(deposit)

        return deposit
    }

    /// Monitor deposit confirmation status
    public func checkDepositStatus(_ depositId: String) async throws -> BridgeDeposit {
        guard var deposit = pendingDeposits[depositId] else {
            throw BridgeError.depositNotFound
        }

        // Query current confirmations from source chain
        let confirmations = try await queryConfirmations(
            chain: deposit.sourceChain,
            txHash: deposit.sourceTxHash
        )

        // Update deposit status
        var updatedDeposit = deposit
        // Note: BridgeDeposit is a struct with let properties, so we create new instance
        updatedDeposit = BridgeDeposit(
            id: deposit.id,
            sourceChain: deposit.sourceChain,
            sourceAddress: deposit.sourceAddress,
            sourceTxHash: deposit.sourceTxHash,
            targetAddress: deposit.targetAddress,
            amount: deposit.amount,
            token: deposit.token,
            status: determineDepositStatus(confirmations: confirmations, required: deposit.requiredConfirmations),
            confirmations: confirmations,
            requiredConfirmations: deposit.requiredConfirmations,
            createdAt: deposit.createdAt,
            completedAt: confirmations >= deposit.requiredConfirmations ? Date() : nil,
            etridTxHash: deposit.etridTxHash
        )

        pendingDeposits[depositId] = updatedDeposit
        return updatedDeposit
    }

    // MARK: - Withdrawal Flow

    /// Request withdrawal from ËTRID to external chain
    public func requestWithdrawal(
        targetChain: SupportedChain,
        targetAddress: String,
        amount: Decimal,
        fromAddress: String
    ) async throws -> BridgeWithdrawal {
        guard bridgeConfigs[targetChain] != nil else {
            throw BridgeError.unsupportedChain
        }

        // Validate target address format
        guard validateAddress(targetAddress, for: targetChain) else {
            throw BridgeError.invalidAddress
        }

        // Calculate fee
        guard let feeEstimate = calculateFee(amount: amount, chain: targetChain) else {
            throw BridgeError.feeCalculationFailed
        }

        // Submit withdrawal request to ËTRID chain
        let txHash = try await submitWithdrawalToChain(
            targetChain: targetChain,
            targetAddress: targetAddress,
            amount: feeEstimate.netAmount,
            fromAddress: fromAddress
        )

        let withdrawal = BridgeWithdrawal(
            targetChain: targetChain,
            targetAddress: targetAddress,
            amount: feeEstimate.netAmount,
            token: bridgedToken(for: targetChain),
            status: .pending,
            etridTxHash: txHash
        )

        pendingWithdrawals[withdrawal.id] = withdrawal
        return withdrawal
    }

    /// Check withdrawal status
    public func checkWithdrawalStatus(_ withdrawalId: String) async throws -> BridgeWithdrawal {
        guard let withdrawal = pendingWithdrawals[withdrawalId] else {
            throw BridgeError.withdrawalNotFound
        }

        // Query status from ËTRID chain
        // In production, this queries the bridge pallet storage
        return withdrawal
    }

    // MARK: - Token Purchase

    /// Get available tokens for purchase
    public func getAvailableTokens() -> [Token] {
        return Token.allTokens
    }

    /// Get exchange rate for token swap
    public func getExchangeRate(from: Token, to: Token) async throws -> ExchangeRate {
        // Query current exchange rates
        // In production, this would query price oracles
        let rate: Decimal

        if from == .etrid && to.isBridged {
            // ETR -> Bridged token
            rate = Decimal(string: "1.0")! // Placeholder
        } else if from.isBridged && to == .etrid {
            // Bridged token -> ETR
            rate = Decimal(string: "1.0")! // Placeholder
        } else {
            throw BridgeError.unsupportedSwapPair
        }

        return ExchangeRate(
            fromToken: from,
            toToken: to,
            rate: rate ?? 1,
            timestamp: Date(),
            validFor: 60 // 60 seconds
        )
    }

    /// Execute token swap
    public func swapTokens(
        from: Token,
        to: Token,
        amount: Decimal,
        walletAddress: String,
        slippageTolerance: Double = 0.5
    ) async throws -> SwapResult {
        let exchangeRate = try await getExchangeRate(from: from, to: to)

        let expectedAmount = amount * exchangeRate.rate
        let minAmount = expectedAmount * (1 - Decimal(slippageTolerance / 100))

        // Execute swap on chain
        let txHash = try await executeSwap(
            fromToken: from,
            toToken: to,
            amount: amount,
            minReceive: minAmount,
            walletAddress: walletAddress
        )

        return SwapResult(
            transactionHash: txHash,
            fromToken: from,
            toToken: to,
            amountIn: amount,
            amountOut: expectedAmount,
            exchangeRate: exchangeRate.rate,
            timestamp: Date()
        )
    }

    // MARK: - Private Helpers

    private func generateDepositMemo(etridAddress: String) -> String? {
        // Generate unique memo to identify the deposit recipient
        // This is used for chains like XRP and Stellar that support memos
        return String(etridAddress.prefix(32))
    }

    private func bridgedToken(for chain: SupportedChain) -> Token {
        switch chain {
        case .solana: return .bridgedSOL
        case .bnbChain: return .bridgedBNB
        case .xrp: return .bridgedXRP
        case .tron: return .bridgedTRX
        case .cardano: return .bridgedADA
        case .stellar: return .bridgedXLM
        case .dogecoin: return .bridgedDOGE
        default: return .etrid
        }
    }

    private func determineDepositStatus(confirmations: Int, required: Int) -> BridgeStatus {
        if confirmations == 0 {
            return .pending
        } else if confirmations < required {
            return .confirming
        } else {
            return .confirmed
        }
    }

    private func validateAddress(_ address: String, for chain: SupportedChain) -> Bool {
        switch chain {
        case .solana:
            return address.count >= 32 && address.count <= 44
        case .bnbChain, .ethereum, .etrid:
            return address.hasPrefix("0x") && address.count == 42
        case .xrp:
            return address.hasPrefix("r") && address.count >= 25 && address.count <= 35
        case .tron:
            return address.hasPrefix("T") && address.count == 34
        case .cardano:
            return address.hasPrefix("addr")
        case .stellar:
            return address.hasPrefix("G") && address.count == 56
        case .dogecoin:
            return address.hasPrefix("D") && address.count >= 26 && address.count <= 35
        }
    }

    // RPC Methods (stubs - implement with actual RPC calls)

    private func submitDepositToChain(_ deposit: BridgeDeposit) async throws {
        // Call ËTRID bridge pallet: initiate_<coin>_deposit
        _ = try await rpcService.call(
            method: "bridge_initiateDeposit",
            params: [
                "chain": deposit.sourceChain.rawValue,
                "txHash": deposit.sourceTxHash,
                "amount": deposit.amount.description,
                "targetAddress": deposit.targetAddress
            ]
        )
    }

    private func queryConfirmations(chain: SupportedChain, txHash: String) async throws -> Int {
        // Query source chain for transaction confirmations
        // This would use chain-specific RPC
        return 0 // Placeholder
    }

    private func submitWithdrawalToChain(
        targetChain: SupportedChain,
        targetAddress: String,
        amount: Decimal,
        fromAddress: String
    ) async throws -> String {
        // Call ËTRID bridge pallet: request_<coin>_withdrawal
        let result = try await rpcService.call(
            method: "bridge_requestWithdrawal",
            params: [
                "chain": targetChain.rawValue,
                "targetAddress": targetAddress,
                "amount": amount.description,
                "fromAddress": fromAddress
            ]
        )
        return result["txHash"] as? String ?? ""
    }

    private func executeSwap(
        fromToken: Token,
        toToken: Token,
        amount: Decimal,
        minReceive: Decimal,
        walletAddress: String
    ) async throws -> String {
        // Execute swap via DEX or bridge
        let result = try await rpcService.call(
            method: "dex_swap",
            params: [
                "fromToken": fromToken.id,
                "toToken": toToken.id,
                "amount": amount.description,
                "minReceive": minReceive.description,
                "address": walletAddress
            ]
        )
        return result["txHash"] as? String ?? ""
    }
}

// MARK: - Supporting Types

public struct BridgeConfig {
    public let chain: SupportedChain
    public let bridgeAddress: String
    public let requiredConfirmations: Int
    public let estimatedTime: Int // seconds
    public let feePercent: Double
    public let minAmount: Decimal
    public let maxAmount: Decimal
}

public struct BridgeDepositInfo {
    public let chain: SupportedChain
    public let depositAddress: String
    public let memo: String?
    public let etridRecipient: String
    public let requiredConfirmations: Int
    public let estimatedTime: Int
    public let feePercent: Double
    public let minAmount: Decimal
    public let maxAmount: Decimal

    public var instructions: String {
        var text = "Send \(chain.symbol) to:\n\(depositAddress)"
        if let memo = memo {
            text += "\n\nMemo/Tag: \(memo)"
        }
        text += "\n\nMinimum: \(minAmount) \(chain.symbol)"
        text += "\nFee: \(feePercent)%"
        text += "\nConfirmations: \(requiredConfirmations)"
        return text
    }
}

public struct BridgeFeeEstimate {
    public let grossAmount: Decimal
    public let feeAmount: Decimal
    public let feePercent: Double
    public let netAmount: Decimal
    public let estimatedTime: Int
    public let requiredConfirmations: Int
}

public enum BridgeValidationResult {
    case success
    case failure(String)
}

public struct ExchangeRate {
    public let fromToken: Token
    public let toToken: Token
    public let rate: Decimal
    public let timestamp: Date
    public let validFor: Int // seconds

    public var isExpired: Bool {
        return Date().timeIntervalSince(timestamp) > Double(validFor)
    }
}

public struct SwapResult {
    public let transactionHash: String
    public let fromToken: Token
    public let toToken: Token
    public let amountIn: Decimal
    public let amountOut: Decimal
    public let exchangeRate: Decimal
    public let timestamp: Date
}

public enum BridgeError: LocalizedError {
    case unsupportedChain
    case invalidAmount(String)
    case invalidAddress
    case depositNotFound
    case withdrawalNotFound
    case feeCalculationFailed
    case unsupportedSwapPair
    case insufficientBalance
    case transactionFailed(String)

    public var errorDescription: String? {
        switch self {
        case .unsupportedChain:
            return "This chain is not supported for bridging"
        case .invalidAmount(let message):
            return message
        case .invalidAddress:
            return "Invalid destination address"
        case .depositNotFound:
            return "Deposit not found"
        case .withdrawalNotFound:
            return "Withdrawal not found"
        case .feeCalculationFailed:
            return "Failed to calculate bridge fee"
        case .unsupportedSwapPair:
            return "This token pair is not supported for swapping"
        case .insufficientBalance:
            return "Insufficient balance"
        case .transactionFailed(let message):
            return "Transaction failed: \(message)"
        }
    }
}

// MARK: - RPC Service

public actor RPCService {
    private let baseURL: URL
    private let session: URLSession

    public init(baseURL: URL, session: URLSession = .shared) {
        self.baseURL = baseURL
        self.session = session
    }

    public func call(method: String, params: [String: Any]) async throws -> [String: Any] {
        var request = URLRequest(url: baseURL)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let body: [String: Any] = [
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        ]

        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw RPCError.httpError
        }

        guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
              let result = json["result"] as? [String: Any] else {
            if let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
               let error = json["error"] as? [String: Any],
               let message = error["message"] as? String,
               let code = error["code"] as? Int {
                throw RPCError.rpcError(code: code, message: message)
            }
            throw RPCError.noResult
        }

        return result
    }
}

// Note: RPCError is defined in NetworkConfig.swift
