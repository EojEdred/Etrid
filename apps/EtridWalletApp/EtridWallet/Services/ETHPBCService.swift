//
//  ETHPBCService.swift
//  EtridWallet
//
//  Ethereum PBC (Precompiled Blockchain) Service
//

import Foundation

class ETHPBCService: ObservableObject {
    @Published var isLoading = false
    @Published var error: String?

    private let baseURL = "https://api.etrid.io/v1/ethpbc" // Replace with actual API
    private let rpcURL = "https://rpc.ethpbc.etrid.io" // Replace with actual RPC

    // MARK: - Call Precompile
    func callPrecompile(address: PrecompileAddress, method: String, params: [String: Any], gasLimit: String? = nil, value: String? = nil) async throws -> PrecompileResponse {
        isLoading = true
        defer { isLoading = false }

        let call = PrecompileCall(precompile: address, method: method, params: params, gasLimit: gasLimit, value: value)

        guard let url = URL(string: "\(baseURL)/precompile/call") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        // Encode the call (simplified version)
        let requestBody: [String: Any] = [
            "address": address.rawValue,
            "method": method,
            "params": params,
            "gasLimit": gasLimit ?? "100000",
            "value": value ?? "0"
        ]

        urlRequest.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            let errorMessage = String(data: data, encoding: .utf8) ?? "Unknown error"
            throw ETHPBCServiceError.precompileCallFailed(message: errorMessage)
        }

        let decoder = JSONDecoder()
        let precompileResponse = try decoder.decode(PrecompileResponse.self, from: data)

        return precompileResponse
    }

    // MARK: - Oracle: Get Price
    func getOraclePrice(token: String) async throws -> OraclePrice {
        let params: [String: Any] = ["token": token]
        let response = try await callPrecompile(address: .oracle, method: "getPrice", params: params)

        guard response.success, let result = response.result else {
            throw ETHPBCServiceError.precompileCallFailed(message: response.error ?? "Unknown error")
        }

        // Parse the result into OraclePrice
        guard let url = URL(string: "\(baseURL)/oracle/price/\(token)") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, urlResponse) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = urlResponse as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let price = try decoder.decode(OraclePrice.self, from: data)

        return price
    }

    // MARK: - Governance: Vote on Proposal
    func voteOnProposal(proposalId: String, choice: VoteChoice, votingPower: String? = nil) async throws -> PrecompileResponse {
        var params: [String: Any] = [
            "proposalId": proposalId,
            "choice": choice.rawValue
        ]

        if let power = votingPower {
            params["votingPower"] = power
        }

        return try await callPrecompile(address: .governance, method: "vote", params: params)
    }

    // MARK: - Governance: Get Proposals
    func getProposals(status: ETHPBCGovernanceProposal.ProposalStatus? = nil, page: Int = 1, limit: Int = 20) async throws -> [ETHPBCGovernanceProposal] {
        var urlComponents = URLComponents(string: "\(baseURL)/governance/proposals")!
        var queryItems: [URLQueryItem] = [
            URLQueryItem(name: "page", value: "\(page)"),
            URLQueryItem(name: "limit", value: "\(limit)")
        ]

        if let status = status {
            queryItems.append(URLQueryItem(name: "status", value: status.rawValue))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let proposals = try decoder.decode([ETHPBCGovernanceProposal].self, from: data)

        return proposals
    }

    // MARK: - Staking: Stake Tokens
    func stakeTokens(amount: String, lockPeriod: Int? = nil) async throws -> PrecompileResponse {
        var params: [String: Any] = ["amount": amount]

        if let period = lockPeriod {
            params["lockPeriod"] = period
        }

        return try await callPrecompile(address: .staking, method: "stake", params: params, value: amount)
    }

    // MARK: - Staking: Unstake Tokens
    func unstakeTokens(amount: String) async throws -> PrecompileResponse {
        let params: [String: Any] = ["amount": amount]
        return try await callPrecompile(address: .staking, method: "unstake", params: params)
    }

    // MARK: - Staking: Get Staking Info
    func getStakingInfo(address: String) async throws -> StakingInfo {
        guard let url = URL(string: "\(baseURL)/staking/info/\(address)") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        let stakingInfo = try decoder.decode(StakingInfo.self, from: data)

        return stakingInfo
    }

    // MARK: - ETH Wrap: Wrap ETH
    func wrapETH(amount: String) async throws -> PrecompileResponse {
        let params: [String: Any] = ["amount": amount]
        return try await callPrecompile(address: .ethWrap, method: "wrap", params: params, value: amount)
    }

    // MARK: - ETH Wrap: Unwrap ETH
    func unwrapETH(amount: String) async throws -> PrecompileResponse {
        let params: [String: Any] = ["amount": amount]
        return try await callPrecompile(address: .ethWrap, method: "unwrap", params: params)
    }

    // MARK: - ETH Wrap: Get Wrapped ETH Info
    func getWrappedETHInfo(address: String) async throws -> WrappedETH {
        guard let url = URL(string: "\(baseURL)/wrap/info/\(address)") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        let wrappedETH = try decoder.decode(WrappedETH.self, from: data)

        return wrappedETH
    }

    // MARK: - Token Registry: Query Token
    func queryToken(address: String) async throws -> TokenInfo {
        guard let url = URL(string: "\(baseURL)/registry/token/\(address)") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        let tokenInfo = try decoder.decode(TokenInfo.self, from: data)

        return tokenInfo
    }

    // MARK: - State Proof: Verify Proof
    func verifyStateProof(blockHash: String, stateRoot: String, proof: [String], key: String, value: String) async throws -> StateProof {
        let params: [String: Any] = [
            "blockHash": blockHash,
            "stateRoot": stateRoot,
            "proof": proof,
            "key": key,
            "value": value
        ]

        let response = try await callPrecompile(address: .stateProof, method: "verify", params: params)

        guard response.success else {
            throw ETHPBCServiceError.verificationFailed(message: response.error ?? "Unknown error")
        }

        guard let url = URL(string: "\(baseURL)/stateproof/verify") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let requestBody: [String: Any] = [
            "blockHash": blockHash,
            "stateRoot": stateRoot,
            "proof": proof,
            "key": key,
            "value": value
        ]

        urlRequest.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (data, urlResponse) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = urlResponse as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        let stateProof = try decoder.decode(StateProof.self, from: data)

        return stateProof
    }

    // MARK: - Get Transaction History
    func getTransactionHistory(address: String, precompile: PrecompileAddress? = nil, page: Int = 1, limit: Int = 20) async throws -> [PrecompileTransaction] {
        var urlComponents = URLComponents(string: "\(baseURL)/transactions")!
        var queryItems: [URLQueryItem] = [
            URLQueryItem(name: "address", value: address),
            URLQueryItem(name: "page", value: "\(page)"),
            URLQueryItem(name: "limit", value: "\(limit)")
        ]

        if let precompile = precompile {
            queryItems.append(URLQueryItem(name: "precompile", value: precompile.rawValue))
        }

        urlComponents.queryItems = queryItems

        guard let url = urlComponents.url else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "GET"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        let transactions = try decoder.decode([PrecompileTransaction].self, from: data)

        return transactions
    }

    // MARK: - Estimate Gas
    func estimateGas(address: PrecompileAddress, method: String, params: [String: Any]) async throws -> String {
        guard let url = URL(string: "\(baseURL)/gas/estimate") else {
            throw ETHPBCServiceError.invalidURL
        }

        var urlRequest = URLRequest(url: url)
        urlRequest.httpMethod = "POST"
        urlRequest.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let requestBody: [String: Any] = [
            "address": address.rawValue,
            "method": method,
            "params": params
        ]

        urlRequest.httpBody = try JSONSerialization.data(withJSONObject: requestBody)

        let (data, response) = try await URLSession.shared.data(for: urlRequest)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw ETHPBCServiceError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw ETHPBCServiceError.httpError(statusCode: httpResponse.statusCode)
        }

        let json = try JSONSerialization.jsonObject(with: data) as? [String: Any]
        guard let gasEstimate = json?["gasEstimate"] as? String else {
            throw ETHPBCServiceError.invalidResponse
        }

        return gasEstimate
    }
}

// MARK: - ETH PBC Service Errors
enum ETHPBCServiceError: LocalizedError {
    case invalidURL
    case invalidResponse
    case httpError(statusCode: Int)
    case precompileCallFailed(message: String)
    case verificationFailed(message: String)
    case insufficientBalance
    case networkError(Error)

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid URL"
        case .invalidResponse:
            return "Invalid response from server"
        case .httpError(let statusCode):
            return "HTTP error: \(statusCode)"
        case .precompileCallFailed(let message):
            return "Precompile call failed: \(message)"
        case .verificationFailed(let message):
            return "Verification failed: \(message)"
        case .insufficientBalance:
            return "Insufficient balance"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        }
    }
}
