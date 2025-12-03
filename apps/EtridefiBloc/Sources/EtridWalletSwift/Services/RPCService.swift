import Foundation

// MARK: - RPC Service
//
// Wrapper around SubstrateRPCService for governance and general RPC calls.
// Provides a dictionary-based interface for flexible JSON-RPC communication.

public actor GovernanceRPCService {

    private let endpoint: URL
    private let session: URLSession

    /// Initialize with the Primearc Core Chain RPC endpoint
    public init(endpoint: String = NetworkConfig.primearcCoreHTTP) throws {
        guard let url = URL(string: endpoint) else {
            throw RPCError.invalidEndpoint
        }
        self.endpoint = url

        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        config.timeoutIntervalForResource = 60
        self.session = URLSession(configuration: config)
    }

    // MARK: - JSON-RPC Call

    /// Make a JSON-RPC call with dictionary params
    public func call(
        method: String,
        params: [String: Any]
    ) async throws -> [String: Any] {
        var request = URLRequest(url: endpoint)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let body: [String: Any] = [
            "jsonrpc": "2.0",
            "id": Int.random(in: 1...999999),
            "method": method,
            "params": params.isEmpty ? [] : [params]
        ]

        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw RPCError.httpError
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw RPCError.rpcError(code: httpResponse.statusCode, message: "HTTP \(httpResponse.statusCode)")
        }

        guard let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw RPCError.decodingError
        }

        if let error = json["error"] as? [String: Any] {
            let code = error["code"] as? Int ?? -1
            let message = error["message"] as? String ?? "Unknown error"
            throw RPCError.rpcError(code: code, message: message)
        }

        guard let result = json["result"] else {
            throw RPCError.noResult
        }

        if let dict = result as? [String: Any] {
            return dict
        } else if let array = result as? [[String: Any]] {
            return ["items": array]
        } else if let stringResult = result as? String {
            return ["value": stringResult]
        } else if let boolResult = result as? Bool {
            return ["value": boolResult]
        } else if let intResult = result as? Int {
            return ["value": intResult]
        }

        return [:]
    }

    // MARK: - Typed Calls

    /// Make a JSON-RPC call with array params and decode to type
    public func call<T: Decodable>(
        method: String,
        params: [Any] = []
    ) async throws -> T {
        var request = URLRequest(url: endpoint)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let body: [String: Any] = [
            "jsonrpc": "2.0",
            "id": Int.random(in: 1...999999),
            "method": method,
            "params": params
        ]

        request.httpBody = try JSONSerialization.data(withJSONObject: body)

        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw RPCError.httpError
        }

        // Parse the RPC response
        let rpcResponse = try JSONDecoder().decode(RPCResponse<T>.self, from: data)

        if let error = rpcResponse.error {
            throw RPCError.rpcError(code: error.code, message: error.message)
        }

        guard let result = rpcResponse.result else {
            throw RPCError.noResult
        }

        return result
    }

    // MARK: - Governance Convenience Methods

    /// Get all governance proposals
    public func getProposals() async throws -> [GovernanceRPCProposal] {
        let result: [String: Any] = try await call(method: "governance_getProposals", params: [:])
        guard let items = result["items"] as? [[String: Any]] else {
            if let proposals = result as? [[String: Any]] {
                return proposals.compactMap { GovernanceRPCProposal(from: $0) }
            }
            return []
        }
        return items.compactMap { GovernanceRPCProposal(from: $0) }
    }

    /// Get current Consensus Day phase
    public func getConsensusDayPhase() async throws -> ConsensusDayRPCPhase {
        let result: [String: Any] = try await call(method: "governance_getPhase", params: [:])
        return ConsensusDayRPCPhase(from: result)
    }

    /// Get governance statistics
    public func getGovernanceStats() async throws -> GovernanceRPCStats {
        let result: [String: Any] = try await call(method: "governance_getStats", params: [:])
        return GovernanceRPCStats(from: result)
    }

    /// Check if Consensus Day is active
    public func isConsensusDayActive() async throws -> Bool {
        let result: [String: Any] = try await call(method: "governance_isActive", params: [:])
        return result["value"] as? Bool ?? false
    }
}

// MARK: - RPC Response Types

/// Proposal from RPC
public struct GovernanceRPCProposal {
    public let id: UInt64
    public let proposer: String
    public let title: String
    public let description: String
    public let category: String
    public let status: String
    public let votesFor: String
    public let votesAgainst: String
    public let votesAbstain: String
    public let approved: Bool
    public let executed: Bool

    init?(from dict: [String: Any]) {
        guard let id = dict["id"] as? UInt64 ?? (dict["id"] as? Int).map({ UInt64($0) }) else {
            return nil
        }
        self.id = id
        self.proposer = dict["proposer"] as? String ?? ""
        self.title = dict["title"] as? String ?? ""
        self.description = dict["description"] as? String ?? ""
        self.category = dict["category"] as? String ?? "Unknown"
        self.status = dict["status"] as? String ?? "Unknown"
        self.votesFor = dict["votes_for"] as? String ?? "0"
        self.votesAgainst = dict["votes_against"] as? String ?? "0"
        self.votesAbstain = dict["votes_abstain"] as? String ?? "0"
        self.approved = dict["approved"] as? Bool ?? false
        self.executed = dict["executed"] as? Bool ?? false
    }
}

/// Consensus Day phase from RPC
public struct ConsensusDayRPCPhase {
    public let phase: String
    public let phaseCode: UInt8
    public let phaseStartBlock: UInt32
    public let eventStartBlock: UInt32
    public let year: UInt32
    public let blocksRemaining: UInt32
    public let isActive: Bool

    init(from dict: [String: Any]) {
        self.phase = dict["phase"] as? String ?? "Inactive"
        self.phaseCode = UInt8(dict["phase_code"] as? Int ?? 0)
        self.phaseStartBlock = UInt32(dict["phase_start_block"] as? Int ?? 0)
        self.eventStartBlock = UInt32(dict["event_start_block"] as? Int ?? 0)
        self.year = UInt32(dict["year"] as? Int ?? 0)
        self.blocksRemaining = UInt32(dict["blocks_remaining"] as? Int ?? 0)
        self.isActive = dict["is_active"] as? Bool ?? false
    }
}

/// Governance statistics from RPC
public struct GovernanceRPCStats {
    public let totalProposals: UInt64
    public let activeProposals: UInt64
    public let approvedProposals: UInt64
    public let rejectedProposals: UInt64
    public let totalVotesCast: String
    public let uniqueVoters: UInt32
    public let circulatingSupply: String
    public let quorumThresholdPercent: UInt32

    init(from dict: [String: Any]) {
        self.totalProposals = UInt64(dict["total_proposals"] as? Int ?? 0)
        self.activeProposals = UInt64(dict["active_proposals"] as? Int ?? 0)
        self.approvedProposals = UInt64(dict["approved_proposals"] as? Int ?? 0)
        self.rejectedProposals = UInt64(dict["rejected_proposals"] as? Int ?? 0)
        self.totalVotesCast = dict["total_votes_cast"] as? String ?? "0"
        self.uniqueVoters = UInt32(dict["unique_voters"] as? Int ?? 0)
        self.circulatingSupply = dict["circulating_supply"] as? String ?? "0"
        self.quorumThresholdPercent = UInt32(dict["quorum_threshold_percent"] as? Int ?? 33)
    }
}
