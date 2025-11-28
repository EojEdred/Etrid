//
//  HyperledgerBridgeService.swift
//  EtridWallet
//
//  Hyperledger Fabric Bridge Service
//

import Foundation

class HyperledgerBridgeService: ObservableObject {
    @Published var isLoading = false
    @Published var error: String?
    @Published var connectedNetwork: FabricNetwork?

    private let baseURL = "https://api.etrid.org/v1/hyperledger" // Replace with actual API

    // MARK: - Connect to Network
    func connectToNetwork(networkId: String, credentials: NetworkCredentials? = nil) async throws -> ConnectNetworkResponse {
        isLoading = true
        defer { isLoading = false }

        let request = ConnectNetworkRequest(networkId: networkId, credentials: credentials)

        guard let url = URL(string: "\(baseURL)/networks/connect") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let encoder = JSONEncoder()
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw HyperledgerServiceError.connectionFailed(message: errorMessage)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let connectResponse = try decoder.decode(ConnectNetworkResponse.self, from: data)

        if connectResponse.connected {
            connectedNetwork = connectResponse.network
        }

        return connectResponse
    }

    // MARK: - Get Available Networks
    func getAvailableNetworks() async throws -> [FabricNetwork] {
        isLoading = true
        defer { isLoading = false }

        guard let url = URL(string: "\(baseURL)/networks") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw HyperledgerServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        let networks = try decoder.decode([FabricNetwork].self, from: data)

        return networks
    }

    // MARK: - Bridge to Fabric
    func bridgeToFabric(networkId: String, token: String, amount: String, recipient: String, metadata: [String: String]? = nil) async throws -> BridgeToFabricResponse {
        isLoading = true
        defer { isLoading = false }

        let request = BridgeToFabricRequest(networkId: networkId, token: token, amount: amount, recipient: recipient, metadata: metadata)

        guard let url = URL(string: "\(baseURL)/bridge/to-fabric") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let encoder = JSONEncoder()
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw HyperledgerServiceError.bridgeFailed(message: errorMessage)
        }

        let decoder = JSONDecoder()
        let bridgeResponse = try decoder.decode(BridgeToFabricResponse.self, from: data)

        return bridgeResponse
    }

    // MARK: - Bridge from Fabric
    func bridgeFromFabric(networkId: String, token: String, amount: String, recipient: String, fabricTxId: String, proof: String) async throws -> BridgeFromFabricResponse {
        isLoading = true
        defer { isLoading = false }

        let request = BridgeFromFabricRequest(networkId: networkId, token: token, amount: amount, recipient: recipient, fabricTxId: fabricTxId, proof: proof)

        guard let url = URL(string: "\(baseURL)/bridge/from-fabric") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let encoder = JSONEncoder()
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw HyperledgerServiceError.bridgeFailed(message: errorMessage)
        }

        let decoder = JSONDecoder()
        let bridgeResponse = try decoder.decode(BridgeFromFabricResponse.self, from: data)

        return bridgeResponse
    }

    // MARK: - Get Bridge History
    func getBridgeHistory(address: String, networkId: String? = nil, direction: BridgeDirection? = nil, status: BridgeStatus? = nil, page: Int = 1, limit: Int = 20) async throws -> BridgeHistoryResponse {
        isLoading = true
        defer { isLoading = false }

        var urlComponents = URLComponents(string: "\(baseURL)/bridge/history")!
        var queryItems: [URLQueryItem] = [
            URLQueryItem(name: "address", value: address),
            URLQueryItem(name: "page", value: "\(page)"),
            URLQueryItem(name: "limit", value: "\(limit)")
        ]

        if let networkId = networkId {
            queryItems.append(URLQueryItem(name: "networkId", value: networkId))
        }

        if let direction = direction {
            queryItems.append(URLQueryItem(name: "direction", value: direction.rawValue))
        }

        if let status = status {
            queryItems.append(URLQueryItem(name: "status", value: status.rawValue))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw HyperledgerServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let historyResponse = try decoder.decode(BridgeHistoryResponse.self, from: data)

        return historyResponse
    }

    // MARK: - Verify Transaction
    func verifyTransaction(txHash: String, networkId: String) async throws -> VerifyTransactionResponse {
        isLoading = true
        defer { isLoading = false }

        let request = VerifyTransactionRequest(txHash: txHash, networkId: networkId)

        guard let url = URL(string: "\(baseURL)/bridge/verify") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let encoder = JSONEncoder()
        urlRequest.httpBody = try encoder.encode(request)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw HyperledgerServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let verifyResponse = try decoder.decode(VerifyTransactionResponse.self, from: data)

        return verifyResponse
    }

    // MARK: - Get Transaction Details
    func getTransactionDetails(transactionId: String) async throws -> BridgeTransaction {
        isLoading = true
        defer { isLoading = false }

        guard let url = URL(string: "\(baseURL)/bridge/transactions/\(transactionId)") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw HyperledgerServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let transaction = try decoder.decode(BridgeTransaction.self, from: data)

        return transaction
    }

    // MARK: - Get Network Statistics
    func getNetworkStatistics(networkId: String) async throws -> NetworkStatistics {
        isLoading = true
        defer { isLoading = false }

        guard let url = URL(string: "\(baseURL)/networks/\(networkId)/statistics") else {
            throw HyperledgerServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw HyperledgerServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw HyperledgerServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        let statistics = try decoder.decode(NetworkStatistics.self, from: data)

        return statistics
    }

    // MARK: - Disconnect from Network
    func disconnect() {
        connectedNetwork = nil
    }
}

// MARK: - Hyperledger Service Errors
enum HyperledgerServiceError: LocalizedError {
    case invalidURL
    case invalidResponse
    case httpError(statusCode: Int)
    case connectionFailed(message: String)
    case bridgeFailed(message: String)
    case verificationFailed(message: String)
    case networkError(Error)

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid URL"
        case .invalidResponse:
            return "Invalid response from server"
        case .httpError(let statusCode):
            return "HTTP error: \(statusCode)"
        case .connectionFailed(let message):
            return "Connection failed: \(message)"
        case .bridgeFailed(let message):
            return "Bridge failed: \(message)"
        case .verificationFailed(let message):
            return "Verification failed: \(message)"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        }
    }
}
