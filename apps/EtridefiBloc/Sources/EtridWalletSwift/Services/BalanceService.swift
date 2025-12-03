import Foundation
import Combine

// MARK: - Balance Service

/// Service for fetching token balances across multiple chains
@MainActor
public class BalanceService: ObservableObject {
    public static let shared = BalanceService()

    // MARK: - Published Properties

    @Published public var balances: [String: ChainBalance] = [:]
    @Published public var isLoading = false
    @Published public var lastUpdated: Date?
    @Published public var errors: [String: String] = [:]

    // MARK: - Private Properties

    private let session: URLSession
    private var updateTimer: Timer?
    private let updateInterval: TimeInterval = 30 // 30 seconds

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 10
        self.session = URLSession(configuration: config)
    }

    deinit {
        updateTimer?.invalidate()
    }

    // MARK: - Public Methods

    /// Start automatic balance updates for an address
    public func startBalanceUpdates(for address: String, chains: [ContractAddresses.Network] = [.bnbChain, .polygon, .ethereum, .arbitrum]) {
        // Initial fetch
        Task {
            await refreshBalances(for: address, chains: chains)
        }

        // Schedule periodic updates
        updateTimer?.invalidate()
        updateTimer = Timer.scheduledTimer(withTimeInterval: updateInterval, repeats: true) { [weak self] _ in
            Task { @MainActor in
                await self?.refreshBalances(for: address, chains: chains)
            }
        }
    }

    /// Stop automatic balance updates
    public func stopBalanceUpdates() {
        updateTimer?.invalidate()
        updateTimer = nil
    }

    /// Refresh balances for a given address across specified chains
    public func refreshBalances(for address: String, chains: [ContractAddresses.Network]) async {
        isLoading = true
        defer { isLoading = false }

        errors.removeAll()

        await withTaskGroup(of: (ContractAddresses.Network, ChainBalance?, String?).self) { group in
            for chain in chains {
                group.addTask {
                    do {
                        let balance = try await self.fetchBalance(for: address, on: chain)
                        return (chain, balance, nil)
                    } catch {
                        return (chain, nil, error.localizedDescription)
                    }
                }
            }

            for await (chain, balance, error) in group {
                if let balance = balance {
                    balances[chain.rawValue] = balance
                }
                if let error = error {
                    errors[chain.rawValue] = error
                }
            }
        }

        // Also fetch Solana balance if address looks like a Solana pubkey
        if address.count >= 32 && !address.hasPrefix("0x") {
            do {
                let solBalance = try await fetchSolanaBalance(for: address)
                balances["solana"] = solBalance
            } catch {
                errors["solana"] = error.localizedDescription
            }
        }

        lastUpdated = Date()
    }

    /// Get wETR balance for a specific chain
    public func getWETRBalance(for chain: ContractAddresses.Network) -> Decimal {
        return balances[chain.rawValue]?.wETRBalance ?? 0
    }

    /// Get native token balance for a specific chain
    public func getNativeBalance(for chain: ContractAddresses.Network) -> Decimal {
        return balances[chain.rawValue]?.nativeBalance ?? 0
    }

    /// Get total wETR balance across all chains
    public var totalWETRBalance: Decimal {
        balances.values.reduce(0) { $0 + $1.wETRBalance }
    }

    // MARK: - Private Methods - EVM Chains

    private func fetchBalance(for address: String, on chain: ContractAddresses.Network) async throws -> ChainBalance {
        guard let wETRAddress = ContractAddresses.wrappedETR[chain] else {
            throw BalanceError.contractNotDeployed
        }

        // Fetch native balance and wETR balance in parallel
        async let nativeBalance = fetchNativeBalance(for: address, on: chain)
        async let wETRBalance = fetchERC20Balance(for: address, tokenContract: wETRAddress, on: chain)

        return ChainBalance(
            chain: chain,
            nativeBalance: try await nativeBalance,
            wETRBalance: try await wETRBalance,
            lastUpdated: Date()
        )
    }

    private func fetchNativeBalance(for address: String, on chain: ContractAddresses.Network) async throws -> Decimal {
        let rpcURL = chain.rpcURL
        guard let url = URL(string: rpcURL) else {
            throw BalanceError.invalidURL
        }

        let payload: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [address, "latest"],
            "id": 1
        ]

        let result = try await makeRPCCall(url: url, payload: payload)

        guard let hexBalance = result["result"] as? String else {
            throw BalanceError.parseError
        }

        return hexToDecimal(hexBalance, decimals: 18)
    }

    private func fetchERC20Balance(for address: String, tokenContract: String, on chain: ContractAddresses.Network) async throws -> Decimal {
        let rpcURL = chain.rpcURL
        guard let url = URL(string: rpcURL) else {
            throw BalanceError.invalidURL
        }

        // balanceOf(address) function selector: 0x70a08231
        let paddedAddress = address.lowercased().replacingOccurrences(of: "0x", with: "").leftPadding(toLength: 64, withPad: "0")
        let data = "0x70a08231" + paddedAddress

        let payload: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [
                [
                    "to": tokenContract,
                    "data": data
                ],
                "latest"
            ],
            "id": 1
        ]

        let result = try await makeRPCCall(url: url, payload: payload)

        guard let hexBalance = result["result"] as? String else {
            throw BalanceError.parseError
        }

        // wETR has 18 decimals on EVM chains
        return hexToDecimal(hexBalance, decimals: 18)
    }

    private func makeRPCCall(url: URL, payload: [String: Any]) async throws -> [String: Any] {
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = try JSONSerialization.data(withJSONObject: payload)

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 200 else {
            throw BalanceError.rpcError
        }

        guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw BalanceError.parseError
        }

        if let error = json["error"] as? [String: Any] {
            let message = error["message"] as? String ?? "Unknown RPC error"
            throw BalanceError.rpcErrorMessage(message)
        }

        return json
    }

    // MARK: - Private Methods - Solana

    private func fetchSolanaBalance(for address: String) async throws -> ChainBalance {
        guard let wETRMint = ContractAddresses.wrappedETR[.solana] else {
            throw BalanceError.contractNotDeployed
        }

        guard let url = URL(string: ContractAddresses.Network.solana.rpcURL) else {
            throw BalanceError.invalidURL
        }

        // Fetch SOL balance
        let solPayload: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "getBalance",
            "params": [address],
            "id": 1
        ]

        let solResult = try await makeRPCCall(url: url, payload: solPayload)
        let solLamports = (solResult["result"] as? [String: Any])?["value"] as? Int ?? 0
        let solBalance = Decimal(solLamports) / Decimal(1_000_000_000) // 9 decimals

        // Fetch wETR token account balance
        let wETRBalance = try await fetchSPLTokenBalance(for: address, mint: wETRMint)

        return ChainBalance(
            chain: .solana,
            nativeBalance: solBalance,
            wETRBalance: wETRBalance,
            lastUpdated: Date()
        )
    }

    private func fetchSPLTokenBalance(for owner: String, mint: String) async throws -> Decimal {
        guard let url = URL(string: ContractAddresses.Network.solana.rpcURL) else {
            throw BalanceError.invalidURL
        }

        // Get token accounts by owner
        let payload: [String: Any] = [
            "jsonrpc": "2.0",
            "method": "getTokenAccountsByOwner",
            "params": [
                owner,
                ["mint": mint],
                ["encoding": "jsonParsed"]
            ],
            "id": 1
        ]

        let result = try await makeRPCCall(url: url, payload: payload)

        guard let resultData = result["result"] as? [String: Any],
              let accounts = resultData["value"] as? [[String: Any]] else {
            return 0 // No token account exists
        }

        var totalBalance: Decimal = 0

        for account in accounts {
            if let accountData = account["account"] as? [String: Any],
               let data = accountData["data"] as? [String: Any],
               let parsed = data["parsed"] as? [String: Any],
               let info = parsed["info"] as? [String: Any],
               let tokenAmount = info["tokenAmount"] as? [String: Any],
               let uiAmount = tokenAmount["uiAmount"] as? Double {
                totalBalance += Decimal(uiAmount)
            }
        }

        return totalBalance
    }

    // MARK: - Helpers

    private func hexToDecimal(_ hex: String, decimals: Int) -> Decimal {
        var cleanHex = hex.lowercased()
        if cleanHex.hasPrefix("0x") {
            cleanHex = String(cleanHex.dropFirst(2))
        }

        guard !cleanHex.isEmpty, cleanHex != "0" else {
            return 0
        }

        // Convert hex to integer
        guard let intValue = UInt64(cleanHex, radix: 16) else {
            // For very large numbers, handle them differently
            if let decimalValue = Decimal(string: cleanHex, locale: nil) {
                return decimalValue / pow(10, decimals)
            }
            return 0
        }

        let decimalValue = Decimal(intValue)
        return decimalValue / pow(10, decimals)
    }
}

// MARK: - Supporting Types

public struct ChainBalance: Codable, Equatable {
    public let chain: ContractAddresses.Network
    public let nativeBalance: Decimal
    public let wETRBalance: Decimal
    public let lastUpdated: Date

    public var formattedNativeBalance: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 6
        formatter.minimumFractionDigits = 2
        return formatter.string(from: nativeBalance as NSDecimalNumber) ?? "0.00"
    }

    public var formattedWETRBalance: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 6
        formatter.minimumFractionDigits = 2
        return formatter.string(from: wETRBalance as NSDecimalNumber) ?? "0.00"
    }

    public var nativeSymbol: String {
        switch chain {
        case .primearc: return "ETR"
        case .ethereum: return "ETH"
        case .bnbChain: return "BNB"
        case .polygon: return "MATIC"
        case .arbitrum: return "ETH"
        case .optimism: return "ETH"
        case .base: return "ETH"
        case .solana: return "SOL"
        }
    }
}

public enum BalanceError: LocalizedError {
    case invalidURL
    case contractNotDeployed
    case rpcError
    case parseError
    case rpcErrorMessage(String)

    public var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid RPC URL"
        case .contractNotDeployed:
            return "Contract not deployed on this chain"
        case .rpcError:
            return "RPC call failed"
        case .parseError:
            return "Failed to parse balance"
        case .rpcErrorMessage(let message):
            return message
        }
    }
}

// MARK: - String Extension

extension String {
    func leftPadding(toLength: Int, withPad character: Character) -> String {
        let stringLength = self.count
        if stringLength < toLength {
            return String(repeatElement(character, count: toLength - stringLength)) + self
        } else {
            return String(self.suffix(toLength))
        }
    }
}
