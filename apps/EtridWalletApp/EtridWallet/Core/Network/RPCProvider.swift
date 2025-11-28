//
//  RPCProvider.swift
//  EtridWallet
//
//  JSON-RPC communication with blockchain nodes
//

import Foundation

actor RPCProvider {
    // MARK: - Properties
    private let url: String
    private let session: URLSession
    private var requestId: Int = 0

    // Retry configuration
    private let maxRetries: Int = 3
    private let retryDelay: TimeInterval = 1.0
    private let requestTimeout: TimeInterval = 30.0

    // MARK: - Initialization
    init(url: String, session: URLSession = .shared) {
        self.url = url

        // Configure session with timeout
        let configuration = URLSessionConfiguration.default
        configuration.timeoutIntervalForRequest = requestTimeout
        configuration.timeoutIntervalForResource = requestTimeout * 2
        configuration.requestCachePolicy = .reloadIgnoringLocalCacheData

        self.session = URLSession(configuration: configuration)
    }

    // MARK: - RPC Request
    /// Makes a JSON-RPC request with retry logic
    /// - Parameters:
    ///   - method: RPC method name
    ///   - params: Parameters array
    /// - Returns: Response result
    func request<T: Decodable>(method: String, params: [Any]) async throws -> T {
        var lastError: Error?

        for attempt in 0..<maxRetries {
            do {
                return try await performRequest(method: method, params: params)
            } catch let error as WalletError {
                lastError = error

                // Don't retry on certain errors
                switch error {
                case .invalidAddress, .invalidInput, .invalidAmount:
                    throw error
                case .rpcError(let code, _):
                    // Don't retry on user errors (nonce too low, insufficient funds, etc.)
                    if code >= -32000 && code <= -32099 {
                        throw error
                    }
                default:
                    break
                }

                // Wait before retrying (exponential backoff)
                if attempt < maxRetries - 1 {
                    let delay = retryDelay * Double(1 << attempt) // 1s, 2s, 4s
                    try await Task.sleep(nanoseconds: UInt64(delay * 1_000_000_000))
                }
            } catch {
                lastError = error
                if attempt < maxRetries - 1 {
                    try await Task.sleep(nanoseconds: UInt64(retryDelay * 1_000_000_000))
                }
            }
        }

        throw lastError ?? WalletError.networkError("Request failed after \(maxRetries) attempts")
    }

    /// Performs a single RPC request without retry
    private func performRequest<T: Decodable>(method: String, params: [Any]) async throws -> T {
        requestId += 1

        let rpcRequest = RPCRequest(
            jsonrpc: "2.0",
            method: method,
            params: params,
            id: requestId
        )

        guard let url = URL(string: url) else {
            throw WalletError.networkError("Invalid URL")
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
        urlRequest.httpBody = try JSONSerialization.data(withJSONObject: rpcRequest.toDictionary())

        let (data, response) = try await session.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw WalletError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            if httpResponse.statusCode == 429 {
                throw WalletError.rateLimitExceeded
            }
            throw WalletError.networkError("HTTP \(httpResponse.statusCode)")
        }

        let rpcResponse = try JSONDecoder().decode(RPCResponse<T>.self, from: data)

        if let error = rpcResponse.error {
            throw WalletError.rpcError(error.code, error.message)
        }

        guard let result = rpcResponse.result else {
            throw WalletError.invalidResponse
        }

        return result
    }

    /// Makes a raw JSON-RPC request returning JSON
    /// - Parameters:
    ///   - method: RPC method name
    ///   - params: Parameters array
    /// - Returns: Response as dictionary
    func requestRaw(method: String, params: [Any]) async throws -> [String: Any] {
        requestId += 1

        let rpcRequest = RPCRequest(
            jsonrpc: "2.0",
            method: method,
            params: params,
            id: requestId
        )

        guard let url = URL(string: url) else {
            throw WalletError.networkError("Invalid URL")
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")
        urlRequest.httpBody = try JSONSerialization.data(withJSONObject: rpcRequest.toDictionary())

        let (data, response) = try await session.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw WalletError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw WalletError.networkError("HTTP \(httpResponse.statusCode)")
        }

        guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw WalletError.invalidResponse
        }

        if let error = json["error"] as? [String: Any],
           let code = error["code"] as? Int,
           let message = error["message"] as? String {
            throw WalletError.rpcError(code, message)
        }

        return json
    }

    // MARK: - Batch Requests
    /// Makes multiple RPC requests in a batch
    /// - Parameter requests: Array of (method, params) tuples
    /// - Returns: Array of responses
    func batchRequest(requests: [(method: String, params: [Any])]) async throws -> [[String: Any]] {
        let rpcRequests = requests.enumerated().map { index, request in
            RPCRequest(
                jsonrpc: "2.0",
                method: request.method,
                params: request.params,
                id: requestId + index + 1
            )
        }

        requestId += requests.count

        guard let url = URL(string: url) else {
            throw WalletError.networkError("Invalid URL")
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.addValue("application/json", forHTTPHeaderField: "Content-Type")

        let batch = rpcRequests.map { $0.toDictionary() }
        urlRequest.httpBody = try JSONSerialization.data(withJSONObject: batch)

        let (data, response) = try await session.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw WalletError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw WalletError.networkError("HTTP \(httpResponse.statusCode)")
        }

        guard let json = try JSONSerialization.jsonObject(with: data) as? [[String: Any]] else {
            throw WalletError.invalidResponse
        }

        return json
    }
}

// MARK: - RPC Models
struct RPCRequest {
    let jsonrpc: String
    let method: String
    let params: [Any]
    let id: Int

    func toDictionary() -> [String: Any] {
        [
            "jsonrpc": jsonrpc,
            "method": method,
            "params": params,
            "id": id
        ]
    }
}

struct RPCResponse<T: Decodable>: Decodable {
    let jsonrpc: String
    let id: Int
    let result: T?
    let error: RPCError?
}

struct RPCError: Decodable {
    let code: Int
    let message: String
    let data: String?
}

// MARK: - Common RPC Methods
extension RPCProvider {
    /// Gets current block number
    func getBlockNumber() async throws -> String {
        try await request(method: "eth_blockNumber", params: [])
    }

    /// Gets balance of address
    func getBalance(address: String) async throws -> String {
        try await request(method: "eth_getBalance", params: [address, "latest"])
    }

    /// Gets transaction count (nonce)
    func getTransactionCount(address: String) async throws -> String {
        try await request(method: "eth_getTransactionCount", params: [address, "latest"])
    }

    /// Gets gas price
    func getGasPrice() async throws -> String {
        try await request(method: "eth_gasPrice", params: [])
    }

    /// Estimates gas for transaction
    func estimateGas(params: [String: Any]) async throws -> String {
        let response: String = try await request(method: "eth_estimateGas", params: [params])
        return response
    }

    /// Sends raw transaction
    func sendRawTransaction(signedTx: String) async throws -> String {
        try await request(method: "eth_sendRawTransaction", params: [signedTx])
    }

    /// Gets transaction by hash
    func getTransaction(hash: String) async throws -> [String: Any] {
        try await requestRaw(method: "eth_getTransactionByHash", params: [hash])
    }

    /// Gets transaction receipt
    func getTransactionReceipt(hash: String) async throws -> [String: Any]? {
        let response = try await requestRaw(method: "eth_getTransactionReceipt", params: [hash])
        return response["result"] as? [String: Any]
    }

    /// Calls contract method
    func call(to: String, data: String) async throws -> String {
        let params: [String: Any] = [
            "to": to,
            "data": data
        ]
        return try await request(method: "eth_call", params: [params, "latest"])
    }

    /// Gets chain ID
    func getChainId() async throws -> String {
        try await request(method: "eth_chainId", params: [])
    }

    /// Gets network version
    func getNetworkVersion() async throws -> String {
        try await request(method: "net_version", params: [])
    }

    /// Gets block by number
    func getBlock(number: String, fullTransactions: Bool = false) async throws -> [String: Any] {
        try await requestRaw(method: "eth_getBlockByNumber", params: [number, fullTransactions])
    }

    /// Gets EIP-1559 fee data
    func getFeeHistory(blockCount: Int, newestBlock: String, rewardPercentiles: [Int]) async throws -> [String: Any] {
        try await requestRaw(method: "eth_feeHistory", params: [blockCount, newestBlock, rewardPercentiles])
    }
}
