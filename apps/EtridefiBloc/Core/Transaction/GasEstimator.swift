//
//  GasEstimator.swift
//  EtridWalletSwift
//
//  Created by Eoj on 2025-11-22.
//  Smart gas estimation with EIP-1559 support
//

import Foundation

/// Gas speed options for transactions
enum GasSpeed: String, CaseIterable {
    case slow = "Slow"
    case normal = "Normal"
    case fast = "Fast"
    case instant = "Instant"

    /// Priority fee multiplier for this speed
    var priorityFeeMultiplier: Double {
        switch self {
        case .slow: return 0.8
        case .normal: return 1.0
        case .fast: return 1.5
        case .instant: return 2.0
        }
    }

    /// Estimated confirmation time in seconds
    var estimatedTime: Int {
        switch self {
        case .slow: return 180      // ~3 minutes
        case .normal: return 60      // ~1 minute
        case .fast: return 30        // ~30 seconds
        case .instant: return 15     // ~15 seconds
        }
    }
}

/// Gas estimate result
struct GasEstimate {
    let gasLimit: BigInt
    let maxFeePerGas: BigInt
    let maxPriorityFeePerGas: BigInt
    let baseFee: BigInt
    let estimatedCost: BigInt
    let speed: GasSpeed

    /// Convert to Wei
    var totalCostInWei: BigInt {
        return gasLimit * maxFeePerGas
    }

    /// Convert to ETH
    var totalCostInEth: Double {
        return Double(totalCostInWei.description) ?? 0.0 / 1e18
    }
}

/// Legacy gas estimate (for non-EIP-1559 networks)
struct LegacyGasEstimate {
    let gasLimit: BigInt
    let gasPrice: BigInt
    let speed: GasSpeed

    var totalCostInWei: BigInt {
        return gasLimit * gasPrice
    }

    var totalCostInEth: Double {
        return Double(totalCostInWei.description) ?? 0.0 / 1e18
    }
}

/// Gas estimation errors
enum GasEstimatorError: Error, LocalizedError {
    case failedToGetBaseFee
    case failedToEstimateGasLimit
    case networkNotSupported
    case insufficientFundsForGas
    case gasPriceTooHigh

    var errorDescription: String? {
        switch self {
        case .failedToGetBaseFee:
            return "Failed to retrieve current base fee"
        case .failedToEstimateGasLimit:
            return "Failed to estimate gas limit"
        case .networkNotSupported:
            return "Network does not support EIP-1559"
        case .insufficientFundsForGas:
            return "Insufficient funds to cover gas cost"
        case .gasPriceTooHigh:
            return "Gas price is unusually high"
        }
    }
}

/// Historical gas data for analysis
struct GasHistoryData {
    let timestamp: Date
    let baseFee: BigInt
    let priorityFee: BigInt
    let gasUsed: BigInt
}

/// Smart gas estimator with fallbacks and historical analysis
actor GasEstimator {
    private let web3Service: Web3Service
    private var gasHistory: [GasHistoryData] = []
    private let maxHistorySize = 100

    // Gas limit safety multipliers
    private let gasLimitSafetyBuffer: Double = 1.2  // 20% buffer
    private let minGasLimit = BigInt(21000)         // Minimum for ETH transfer

    init(web3Service: Web3Service) {
        self.web3Service = web3Service
    }

    // MARK: - Gas Estimation

    /// Estimate gas for a transaction with specified speed
    func estimateGas(
        for params: TransactionParameters,
        speed: GasSpeed = .normal,
        balanceCheck: BigInt? = nil
    ) async throws -> GasEstimate {
        // Check if network supports EIP-1559
        let supportsEIP1559 = try await checkEIP1559Support()

        guard supportsEIP1559 else {
            throw GasEstimatorError.networkNotSupported
        }

        // Get current base fee
        let baseFee = try await getBaseFee()

        // Calculate priority fee based on speed and historical data
        let priorityFee = try await calculatePriorityFee(speed: speed)

        // Calculate max fee per gas (base fee buffer + priority fee)
        let maxFeePerGas = calculateMaxFeePerGas(baseFee: baseFee, priorityFee: priorityFee)

        // Estimate gas limit
        let gasLimit = try await estimateGasLimit(for: params)

        // Calculate total cost
        let estimatedCost = gasLimit * maxFeePerGas

        // Check if user has sufficient balance
        if let balance = balanceCheck {
            let totalRequired = params.value + estimatedCost
            guard balance >= totalRequired else {
                throw GasEstimatorError.insufficientFundsForGas
            }
        }

        // Store historical data
        await storeGasData(baseFee: baseFee, priorityFee: priorityFee, gasUsed: gasLimit)

        return GasEstimate(
            gasLimit: gasLimit,
            maxFeePerGas: maxFeePerGas,
            maxPriorityFeePerGas: priorityFee,
            baseFee: baseFee,
            estimatedCost: estimatedCost,
            speed: speed
        )
    }

    /// Estimate gas for legacy networks (non-EIP-1559)
    func estimateLegacyGas(
        for params: TransactionParameters,
        speed: GasSpeed = .normal
    ) async throws -> LegacyGasEstimate {
        // Get current gas price
        let baseGasPrice = try await getLegacyGasPrice()

        // Adjust for speed
        let multiplier = speed.priorityFeeMultiplier
        let gasPrice = BigInt(Double(baseGasPrice.description)! * multiplier)

        // Estimate gas limit
        let gasLimit = try await estimateGasLimit(for: params)

        return LegacyGasEstimate(
            gasLimit: gasLimit,
            gasPrice: gasPrice,
            speed: speed
        )
    }

    /// Get estimates for all speed options
    func estimateAllSpeeds(for params: TransactionParameters) async throws -> [GasSpeed: GasEstimate] {
        var estimates: [GasSpeed: GasEstimate] = [:]

        for speed in GasSpeed.allCases {
            do {
                estimates[speed] = try await estimateGas(for: params, speed: speed)
            } catch {
                // Continue with other speeds if one fails
                continue
            }
        }

        return estimates
    }

    // MARK: - Gas Price Calculation

    /// Get current base fee from latest block
    private func getBaseFee() async throws -> BigInt {
        do {
            let latestBlock = try await web3Service.getLatestBlock()

            guard let baseFee = latestBlock.baseFeePerGas else {
                throw GasEstimatorError.failedToGetBaseFee
            }

            return baseFee
        } catch {
            // Fallback to historical average if latest block fails
            return try await getFallbackBaseFee()
        }
    }

    /// Calculate priority fee based on speed and network conditions
    private func calculatePriorityFee(speed: GasSpeed) async throws -> BigInt {
        // Get suggested priority fee from network
        let suggestedPriorityFee = try await getSuggestedPriorityFee()

        // Apply speed multiplier
        let multiplier = speed.priorityFeeMultiplier
        let priorityFee = BigInt(Double(suggestedPriorityFee.description)! * multiplier)

        // Ensure minimum priority fee (1 Gwei)
        let minPriorityFee = BigInt(1_000_000_000)
        return max(priorityFee, minPriorityFee)
    }

    /// Calculate max fee per gas with buffer for base fee fluctuations
    private func calculateMaxFeePerGas(baseFee: BigInt, priorityFee: BigInt) -> BigInt {
        // Max fee = (base fee * 2) + priority fee
        // This accounts for base fee potentially doubling in next block
        return (baseFee * BigInt(2)) + priorityFee
    }

    /// Get suggested priority fee from network
    private func getSuggestedPriorityFee() async throws -> BigInt {
        // Try to get from network oracle
        if let networkPriorityFee = try? await web3Service.getMaxPriorityFeePerGas() {
            return networkPriorityFee
        }

        // Fallback to historical analysis
        return calculateHistoricalPriorityFee()
    }

    /// Calculate priority fee from historical data
    private func calculateHistoricalPriorityFee() -> BigInt {
        guard !gasHistory.isEmpty else {
            return BigInt(2_000_000_000) // Default 2 Gwei
        }

        // Get recent priority fees
        let recentFees = gasHistory.suffix(20).map { $0.priorityFee }

        // Calculate median
        let sortedFees = recentFees.sorted()
        let median = sortedFees[sortedFees.count / 2]

        return median
    }

    // MARK: - Gas Limit Estimation

    /// Estimate gas limit for transaction
    private func estimateGasLimit(for params: TransactionParameters) async throws -> BigInt {
        do {
            // Try to get estimate from network
            let estimate = try await web3Service.estimateGas(
                from: params.from,
                to: params.to,
                value: params.value,
                data: params.data
            )

            // Add safety buffer (20%)
            let withBuffer = BigInt(Double(estimate.description)! * gasLimitSafetyBuffer)

            // Ensure minimum gas limit
            return max(withBuffer, minGasLimit)

        } catch {
            // Fallback to conservative estimates
            return getFallbackGasLimit(for: params)
        }
    }

    /// Get fallback gas limit based on transaction type
    private func getFallbackGasLimit(for params: TransactionParameters) -> BigInt {
        // Check if it's a contract interaction
        if params.data.count > 2 { // "0x" + data
            // Contract interaction - use higher limit
            return BigInt(100_000)
        } else if params.to.lowercased() == "0x0000000000000000000000000000000000000000" {
            // Contract deployment
            return BigInt(500_000)
        } else {
            // Simple ETH transfer
            return minGasLimit
        }
    }

    // MARK: - Network Detection

    /// Check if network supports EIP-1559
    private func checkEIP1559Support() async throws -> Bool {
        do {
            let latestBlock = try await web3Service.getLatestBlock()
            return latestBlock.baseFeePerGas != nil
        } catch {
            return false
        }
    }

    /// Get legacy gas price for non-EIP-1559 networks
    private func getLegacyGasPrice() async throws -> BigInt {
        return try await web3Service.getGasPrice()
    }

    // MARK: - Historical Data Management

    /// Store gas data for historical analysis
    private func storeGasData(baseFee: BigInt, priorityFee: BigInt, gasUsed: BigInt) async {
        let data = GasHistoryData(
            timestamp: Date(),
            baseFee: baseFee,
            priorityFee: priorityFee,
            gasUsed: gasUsed
        )

        gasHistory.append(data)

        // Trim to max size
        if gasHistory.count > maxHistorySize {
            gasHistory.removeFirst(gasHistory.count - maxHistorySize)
        }
    }

    /// Get fallback base fee from historical data
    private func getFallbackBaseFee() async throws -> BigInt {
        guard !gasHistory.isEmpty else {
            return BigInt(30_000_000_000) // Default 30 Gwei
        }

        // Calculate average from recent history
        let recentFees = gasHistory.suffix(10).map { $0.baseFee }
        let sum = recentFees.reduce(BigInt(0), +)
        return sum / BigInt(recentFees.count)
    }

    // MARK: - Network Congestion Analysis

    /// Detect network congestion level
    func getNetworkCongestion() async throws -> NetworkCongestion {
        let baseFee = try await getBaseFee()
        let baseFeeGwei = Double(baseFee.description)! / 1e9

        // Determine congestion based on base fee
        if baseFeeGwei < 20 {
            return .low
        } else if baseFeeGwei < 50 {
            return .medium
        } else if baseFeeGwei < 100 {
            return .high
        } else {
            return .extreme
        }
    }

    /// Calculate gas price percentile for transaction confirmation probability
    func calculateConfirmationProbability(priorityFee: BigInt) -> Double {
        guard !gasHistory.isEmpty else {
            return 0.5 // Unknown, assume 50%
        }

        let recentFees = gasHistory.suffix(20).map { $0.priorityFee }
        let lowerCount = recentFees.filter { $0 <= priorityFee }.count

        return Double(lowerCount) / Double(recentFees.count)
    }
}

/// Network congestion level
enum NetworkCongestion: String {
    case low = "Low"
    case medium = "Medium"
    case high = "High"
    case extreme = "Extreme"

    var description: String {
        switch self {
        case .low:
            return "Network is clear, transactions confirm quickly"
        case .medium:
            return "Normal network activity"
        case .high:
            return "Network is busy, higher fees recommended"
        case .extreme:
            return "Network is congested, expect delays and high fees"
        }
    }
}
