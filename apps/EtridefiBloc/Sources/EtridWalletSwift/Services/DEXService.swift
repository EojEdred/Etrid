import Foundation
import Combine

// MARK: - DEX Aggregation Service

/// Service for fetching swap quotes and building transactions across DEXes
@MainActor
public class DEXService: ObservableObject {
    public static let shared = DEXService()

    // MARK: - Published Properties

    @Published public var currentQuote: SwapQuote?
    @Published public var isLoadingQuote = false
    @Published public var quoteError: String?

    // MARK: - Private Properties

    private let session: URLSession

    // API Endpoints
    private let oneInchBaseURL = "https://api.1inch.dev/swap/v6.0"
    private let jupiterBaseURL = "https://quote-api.jup.ag/v6"

    // 0x API key for Ëtridefi Bloc
    private let zeroXAPIKey = "bbd6ae09-7c0b-47b9-b6b5-e2682fd7d5c4"

    // 1inch API key (using 0x key for now - can be replaced with actual 1inch key)
    private let oneInchAPIKey = "bbd6ae09-7c0b-47b9-b6b5-e2682fd7d5c4"

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 15
        self.session = URLSession(configuration: config)
    }

    // MARK: - Public Methods

    /// Get swap quote for token exchange
    public func getQuote(
        fromToken: String,
        toToken: String,
        amount: Decimal,
        chain: ContractAddresses.Network,
        slippage: Double = 0.5
    ) async throws -> SwapQuote {
        isLoadingQuote = true
        quoteError = nil

        defer { isLoadingQuote = false }

        do {
            let quote: SwapQuote
            if chain == .solana {
                quote = try await getJupiterQuote(
                    fromToken: fromToken,
                    toToken: toToken,
                    amount: amount,
                    slippage: slippage
                )
            } else {
                quote = try await get1inchQuote(
                    fromToken: fromToken,
                    toToken: toToken,
                    amount: amount,
                    chain: chain,
                    slippage: slippage
                )
            }

            currentQuote = quote
            return quote
        } catch {
            quoteError = error.localizedDescription
            throw error
        }
    }

    /// Build swap transaction for execution
    public func buildSwapTransaction(
        quote: SwapQuote,
        fromAddress: String,
        slippage: Double = 0.5
    ) async throws -> SwapTransaction {
        if quote.chain == .solana {
            return try await buildJupiterSwapTx(quote: quote, fromAddress: fromAddress, slippage: slippage)
        } else {
            return try await build1inchSwapTx(quote: quote, fromAddress: fromAddress, slippage: slippage)
        }
    }

    /// Get available DEX routes for a token pair
    public func getAvailableRoutes(
        fromToken: String,
        toToken: String,
        chain: ContractAddresses.Network
    ) async throws -> [DEXRoute] {
        // Return known DEXes for each chain
        switch chain {
        case .ethereum:
            return [
                DEXRoute(dex: "Uniswap V3", name: "Uniswap", logoURL: nil),
                DEXRoute(dex: "SushiSwap", name: "SushiSwap", logoURL: nil),
                DEXRoute(dex: "Curve", name: "Curve", logoURL: nil)
            ]
        case .bnbChain:
            return [
                DEXRoute(dex: "PancakeSwap V3", name: "PancakeSwap", logoURL: nil),
                DEXRoute(dex: "BiSwap", name: "BiSwap", logoURL: nil)
            ]
        case .polygon:
            return [
                DEXRoute(dex: "QuickSwap", name: "QuickSwap", logoURL: nil),
                DEXRoute(dex: "Uniswap V3", name: "Uniswap", logoURL: nil)
            ]
        case .arbitrum:
            return [
                DEXRoute(dex: "Uniswap V3", name: "Uniswap", logoURL: nil),
                DEXRoute(dex: "Camelot", name: "Camelot", logoURL: nil),
                DEXRoute(dex: "SushiSwap", name: "SushiSwap", logoURL: nil)
            ]
        case .solana:
            return [
                DEXRoute(dex: "Raydium", name: "Raydium", logoURL: nil),
                DEXRoute(dex: "Orca", name: "Orca", logoURL: nil),
                DEXRoute(dex: "Jupiter", name: "Jupiter Aggregator", logoURL: nil)
            ]
        default:
            return []
        }
    }

    // MARK: - 1inch Integration (EVM Chains)

    private func get1inchQuote(
        fromToken: String,
        toToken: String,
        amount: Decimal,
        chain: ContractAddresses.Network,
        slippage: Double
    ) async throws -> SwapQuote {
        let chainId = chain.chainId
        let amountWei = (amount * pow(10, 18)).description.split(separator: ".").first.map(String.init) ?? "0"

        // Use 0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE for native tokens
        let fromTokenAddress = fromToken.hasPrefix("0x") ? fromToken : "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"
        let toTokenAddress = toToken.hasPrefix("0x") ? toToken : "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"

        guard var components = URLComponents(string: "\(oneInchBaseURL)/\(chainId)/quote") else {
            throw DEXError.invalidURL
        }

        components.queryItems = [
            URLQueryItem(name: "src", value: fromTokenAddress),
            URLQueryItem(name: "dst", value: toTokenAddress),
            URLQueryItem(name: "amount", value: amountWei)
        ]

        guard let url = components.url else {
            throw DEXError.invalidURL
        }

        var request = URLRequest(url: url)
        request.addValue("application/json", forHTTPHeaderField: "Accept")

        if !oneInchAPIKey.isEmpty {
            request.addValue("Bearer \(oneInchAPIKey)", forHTTPHeaderField: "Authorization")
        }

        // For demo purposes, return a simulated quote if API key is not set
        if oneInchAPIKey.isEmpty {
            return simulatedEVMQuote(fromToken: fromToken, toToken: toToken, amount: amount, chain: chain)
        }

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw DEXError.networkError
        }

        if httpResponse.statusCode == 429 {
            throw DEXError.rateLimited
        }

        guard httpResponse.statusCode == 200 else {
            throw DEXError.apiError(httpResponse.statusCode)
        }

        guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw DEXError.parseError
        }

        guard let toAmountStr = json["toAmount"] as? String,
              let toAmount = Decimal(string: toAmountStr) else {
            throw DEXError.parseError
        }

        let outputAmount = toAmount / pow(10, 18)
        let rate = outputAmount / amount
        let priceImpact = (json["estimatedGas"] as? Double).map { _ in 0.1 } ?? 0.0 // Simplified

        return SwapQuote(
            id: UUID().uuidString,
            chain: chain,
            fromToken: fromToken,
            toToken: toToken,
            fromAmount: amount,
            toAmount: outputAmount,
            rate: rate,
            priceImpact: priceImpact,
            route: "1inch Aggregator",
            estimatedGas: json["estimatedGas"] as? Int ?? 150000,
            timestamp: Date()
        )
    }

    private func build1inchSwapTx(
        quote: SwapQuote,
        fromAddress: String,
        slippage: Double
    ) async throws -> SwapTransaction {
        // For demo, return a simulated transaction
        return SwapTransaction(
            id: UUID().uuidString,
            quote: quote,
            fromAddress: fromAddress,
            to: "0x1111111254EEB25477B68fb85Ed929f73A960582", // 1inch router
            data: "0x...", // Swap calldata
            value: quote.fromToken.hasPrefix("0x") ? "0" : quote.fromAmount.description,
            gasLimit: quote.estimatedGas,
            gasPrice: nil, // Use network default
            nonce: nil // Fetch from network
        )
    }

    // MARK: - Jupiter Integration (Solana)

    private func getJupiterQuote(
        fromToken: String,
        toToken: String,
        amount: Decimal,
        slippage: Double
    ) async throws -> SwapQuote {
        // SOL mint address
        let solMint = "So11111111111111111111111111111111111111112"
        let wETRMint = ContractAddresses.wrappedETR[.solana] ?? ""

        let inputMint = fromToken == "SOL" ? solMint : fromToken
        let outputMint = toToken == "SOL" ? solMint : toToken

        // Convert to lamports (9 decimals for SOL, 9 for wETR on Solana)
        let scaledAmount = amount * pow(10, 9)
        let amountLamports = Int(truncating: scaledAmount as NSDecimalNumber)

        guard var components = URLComponents(string: "\(jupiterBaseURL)/quote") else {
            throw DEXError.invalidURL
        }

        components.queryItems = [
            URLQueryItem(name: "inputMint", value: inputMint),
            URLQueryItem(name: "outputMint", value: outputMint),
            URLQueryItem(name: "amount", value: String(amountLamports)),
            URLQueryItem(name: "slippageBps", value: String(Int(slippage * 100)))
        ]

        guard let url = components.url else {
            throw DEXError.invalidURL
        }

        var request = URLRequest(url: url)
        request.addValue("application/json", forHTTPHeaderField: "Accept")

        do {
            let (data, response) = try await session.data(for: request)

            guard let httpResponse = response as? HTTPURLResponse,
                  httpResponse.statusCode == 200 else {
                // Fallback to simulated quote
                return simulatedSolanaQuote(fromToken: fromToken, toToken: toToken, amount: amount)
            }

            guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any],
                  let outAmountStr = json["outAmount"] as? String,
                  let outAmount = Decimal(string: outAmountStr) else {
                throw DEXError.parseError
            }

            let outputAmount = outAmount / pow(10, 9)
            let rate = outputAmount / amount
            let priceImpact = (json["priceImpactPct"] as? String).flatMap { Double($0) } ?? 0.0

            return SwapQuote(
                id: UUID().uuidString,
                chain: .solana,
                fromToken: fromToken,
                toToken: toToken,
                fromAmount: amount,
                toAmount: outputAmount,
                rate: rate,
                priceImpact: priceImpact,
                route: (json["routePlan"] as? [[String: Any]])?.first?["swapInfo"] as? String ?? "Jupiter",
                estimatedGas: 5000, // Solana compute units
                timestamp: Date()
            )
        } catch {
            // Fallback to simulated quote
            return simulatedSolanaQuote(fromToken: fromToken, toToken: toToken, amount: amount)
        }
    }

    private func buildJupiterSwapTx(
        quote: SwapQuote,
        fromAddress: String,
        slippage: Double
    ) async throws -> SwapTransaction {
        // In production, call Jupiter's /swap endpoint
        return SwapTransaction(
            id: UUID().uuidString,
            quote: quote,
            fromAddress: fromAddress,
            to: "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4", // Jupiter program
            data: "", // Base64 encoded transaction
            value: "0",
            gasLimit: 200000,
            gasPrice: nil,
            nonce: nil
        )
    }

    // MARK: - Simulated Quotes (Demo)

    private func simulatedEVMQuote(
        fromToken: String,
        toToken: String,
        amount: Decimal,
        chain: ContractAddresses.Network
    ) -> SwapQuote {
        // Simulate exchange rates based on known prices
        let rate: Decimal
        if toToken.lowercased().contains(ContractAddresses.wrappedETR[chain]?.lowercased() ?? "") {
            // Buying wETR with native token
            rate = Decimal(string: "6.67")! // ~$0.15 wETR, assuming native ~$1
        } else {
            // Selling wETR
            rate = Decimal(string: "0.15")!
        }

        return SwapQuote(
            id: UUID().uuidString,
            chain: chain,
            fromToken: fromToken,
            toToken: toToken,
            fromAmount: amount,
            toAmount: amount * rate,
            rate: rate,
            priceImpact: 0.15,
            route: "Simulated DEX Route",
            estimatedGas: 150000,
            timestamp: Date()
        )
    }

    private func simulatedSolanaQuote(
        fromToken: String,
        toToken: String,
        amount: Decimal
    ) -> SwapQuote {
        let rate = Decimal(string: "0.0015")! // wETR/SOL rate (~$0.15 / ~$100)

        let outputAmount: Decimal
        if toToken == ContractAddresses.wrappedETR[.solana] {
            outputAmount = amount / rate // SOL -> wETR
        } else {
            outputAmount = amount * rate // wETR -> SOL
        }

        return SwapQuote(
            id: UUID().uuidString,
            chain: .solana,
            fromToken: fromToken,
            toToken: toToken,
            fromAmount: amount,
            toAmount: outputAmount,
            rate: outputAmount / amount,
            priceImpact: 0.1,
            route: "Raydium (Simulated)",
            estimatedGas: 5000,
            timestamp: Date()
        )
    }
}

// MARK: - Supporting Types

public struct SwapQuote: Identifiable, Codable {
    public let id: String
    public let chain: ContractAddresses.Network
    public let fromToken: String
    public let toToken: String
    public let fromAmount: Decimal
    public let toAmount: Decimal
    public let rate: Decimal
    public let priceImpact: Double
    public let route: String
    public let estimatedGas: Int
    public let timestamp: Date

    public var formattedRate: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 6
        return formatter.string(from: rate as NSDecimalNumber) ?? "0"
    }

    public var formattedPriceImpact: String {
        return String(format: "%.2f%%", priceImpact)
    }

    public var isHighPriceImpact: Bool {
        priceImpact > 1.0
    }
}

public struct SwapTransaction: Identifiable, Codable {
    public let id: String
    public let quote: SwapQuote
    public let fromAddress: String
    public let to: String
    public let data: String
    public let value: String
    public let gasLimit: Int
    public let gasPrice: String?
    public let nonce: Int?
}

public struct DEXRoute: Identifiable, Codable {
    public var id: String { dex }
    public let dex: String
    public let name: String
    public let logoURL: String?
}

public enum DEXError: LocalizedError {
    case invalidURL
    case networkError
    case parseError
    case noLiquidity
    case rateLimited
    case apiError(Int)
    case insufficientBalance
    case slippageTooHigh

    public var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid API URL"
        case .networkError:
            return "Network error occurred"
        case .parseError:
            return "Failed to parse response"
        case .noLiquidity:
            return "Insufficient liquidity for this trade"
        case .rateLimited:
            return "Rate limited, try again later"
        case .apiError(let code):
            return "API error: \(code)"
        case .insufficientBalance:
            return "Insufficient balance"
        case .slippageTooHigh:
            return "Price impact too high"
        }
    }
}

// MARK: - Swap Preview Helper

public extension DEXService {

    /// Calculate minimum received amount with slippage
    func calculateMinimumReceived(quote: SwapQuote, slippage: Double) -> Decimal {
        return quote.toAmount * Decimal(1 - slippage / 100)
    }

    /// Format swap preview for display
    func formatSwapPreview(quote: SwapQuote) -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 6

        let from = formatter.string(from: quote.fromAmount as NSDecimalNumber) ?? "0"
        let to = formatter.string(from: quote.toAmount as NSDecimalNumber) ?? "0"

        return "\(from) -> \(to)"
    }
}
