//
//  WalletConnectService.swift
//  EtridWallet
//
//  Service for WalletConnect v2 protocol integration
//

import Foundation
import Combine

@MainActor
class WalletConnectService: ObservableObject {
    static let shared = WalletConnectService()

    @Published var sessions: [WCSession] = []
    @Published var proposals: [WCProposal] = []
    @Published var pendingRequest: WCRequest?
    @Published var connectionState: WCSessionState = .disconnected
    @Published var isInitialized = false

    private let storageKey = "wc_sessions"
    private let proposalsKey = "wc_proposals"

    // WalletConnect client would be initialized here
    // private var client: WalletConnectClient?

    private init() {
        loadSessions()
        loadProposals()
    }

    // MARK: - Initialization

    func initialize() async throws {
        guard !isInitialized else { return }

        // TODO: Initialize WalletConnect v2 client
        /*
        client = try await WalletConnectClient.initialize(
            metadata: AppMetadata(
                name: "Ëtrid Wallet",
                description: "Ëtrid Mobile Wallet",
                url: "https://etrid.io",
                icons: ["https://etrid.io/icon.png"]
            )
        )

        // Register event listeners
        client?.sessionProposalPublisher
            .sink { [weak self] proposal in
                self?.handleSessionProposal(proposal)
            }
            .store(in: &cancellables)

        client?.sessionRequestPublisher
            .sink { [weak self] request in
                self?.handleSessionRequest(request)
            }
            .store(in: &cancellables)
        */

        isInitialized = true
    }

    // MARK: - Pairing

    func pair(uri: String) async throws {
        guard uri.hasPrefix("wc:") else {
            throw WCError.invalidURI
        }

        connectionState = .connecting

        // TODO: Implement actual pairing with WalletConnect client
        // try await client?.pair(uri: uri)

        // Simulate pairing delay
        try await Task.sleep(nanoseconds: 1_000_000_000)

        connectionState = .connected
        print("Pairing initiated with URI: \(uri)")
    }

    func pairFromQRCode(_ qrCode: String) async throws {
        try await pair(uri: qrCode)
    }

    // MARK: - Session Management

    func approveSession(_ proposal: WCProposal) async throws {
        guard let proposalIndex = proposals.firstIndex(where: { $0.id == proposal.id }) else {
            throw WCError.proposalNotFound
        }

        // Get current account info
        let address = try await getCurrentAddress()
        let chainId = try await getCurrentChainId()

        // TODO: Approve session with WalletConnect client
        /*
        let namespaces = [
            "eip155": SessionNamespace(
                accounts: ["eip155:\(chainId):\(address)"],
                methods: proposal.permissions.methods,
                events: proposal.permissions.events
            )
        ]

        let session = try await client?.approve(
            proposalId: proposal.id,
            namespaces: namespaces
        )
        */

        // Create session object
        let session = WCSession(
            id: proposal.id,
            topic: "topic_" + UUID().uuidString,
            dAppUrl: proposal.proposer.url,
            dAppName: proposal.proposer.name,
            dAppIcon: proposal.proposer.iconUrl,
            dAppDescription: proposal.proposer.description,
            permissions: proposal.permissions.methods,
            chains: proposal.permissions.chains,
            createdAt: Date(),
            expiresAt: Date().addingTimeInterval(7 * 24 * 60 * 60), // 7 days
            lastUsed: Date()
        )

        sessions.append(session)
        proposals.remove(at: proposalIndex)

        saveSessions()
        saveProposals()
    }

    func rejectSession(_ proposal: WCProposal) async throws {
        guard let proposalIndex = proposals.firstIndex(where: { $0.id == proposal.id }) else {
            throw WCError.proposalNotFound
        }

        // TODO: Reject with WalletConnect client
        /*
        try await client?.reject(
            proposalId: proposal.id,
            reason: .userRejected
        )
        */

        proposals.remove(at: proposalIndex)
        saveProposals()
    }

    func disconnect(_ session: WCSession) async throws {
        guard let sessionIndex = sessions.firstIndex(where: { $0.id == session.id }) else {
            throw WCError.sessionNotFound
        }

        connectionState = .disconnecting

        // TODO: Disconnect with WalletConnect client
        /*
        try await client?.disconnect(topic: session.topic)
        */

        sessions.remove(at: sessionIndex)
        saveSessions()

        connectionState = .disconnected
    }

    func disconnectAll() async throws {
        for session in sessions {
            try? await disconnect(session)
        }
    }

    // MARK: - Request Handling

    func approveRequest(_ request: WCRequest, result: String) async throws {
        // TODO: Send response via WalletConnect client
        /*
        try await client?.respond(
            topic: request.topic,
            response: .success(result)
        )
        */

        // Update last used
        if let index = sessions.firstIndex(where: { $0.topic == request.topic }) {
            sessions[index].lastUsed = Date()
            saveSessions()
        }

        pendingRequest = nil
    }

    func rejectRequest(_ request: WCRequest, error: String) async throws {
        // TODO: Send error response via WalletConnect client
        /*
        try await client?.respond(
            topic: request.topic,
            response: .error(WCError.userRejected)
        )
        */

        pendingRequest = nil
    }

    // MARK: - Transaction Methods

    func signTransaction(_ session: WCSession, transaction: TransactionRequest) async throws -> String {
        // TODO: Integrate with CryptoManager for signing
        // This is a placeholder
        let signedTx = "0x" + String(repeating: "0", count: 130)
        return signedTx
    }

    func signMessage(_ session: WCSession, message: String) async throws -> String {
        // TODO: Integrate with CryptoManager for signing
        let signature = "0x" + String(repeating: "0", count: 130)
        return signature
    }

    func signTypedData(_ session: WCSession, typedData: String) async throws -> String {
        // TODO: Implement EIP-712 signing
        let signature = "0x" + String(repeating: "0", count: 130)
        return signature
    }

    // MARK: - Event Handlers

    private func handleSessionProposal(_ proposal: Any) {
        // Convert to WCProposal and add to proposals list
        // This would be implemented with actual WalletConnect SDK

        let wcProposal = WCProposal(
            id: UUID().uuidString,
            proposer: WCProposer(
                name: "Sample dApp",
                description: "A sample dApp",
                url: "https://example.com",
                icons: []
            ),
            permissions: WCPermissions(
                chains: ["eip155:1"],
                methods: ["eth_sendTransaction", "personal_sign"],
                events: ["chainChanged", "accountsChanged"]
            ),
            expiresAt: Date().addingTimeInterval(300)
        )

        proposals.append(wcProposal)
        saveProposals()
    }

    private func handleSessionRequest(_ request: Any) {
        // Convert to WCRequest and handle
        // This would be implemented with actual WalletConnect SDK
    }

    private func handleSessionDelete(_ topic: String) {
        sessions.removeAll { $0.topic == topic }
        saveSessions()
    }

    // MARK: - Utilities

    func getSession(byTopic topic: String) -> WCSession? {
        sessions.first { $0.topic == topic }
    }

    func getSession(byId id: String) -> WCSession? {
        sessions.first { $0.id == id }
    }

    func cleanExpiredSessions() {
        let activeSessions = sessions.filter { !$0.isExpired }
        if activeSessions.count != sessions.count {
            sessions = activeSessions
            saveSessions()
        }
    }

    func cleanExpiredProposals() {
        let activeProposals = proposals.filter { !$0.isExpired }
        if activeProposals.count != proposals.count {
            proposals = activeProposals
            saveProposals()
        }
    }

    // MARK: - Helpers

    private func getCurrentAddress() async throws -> String {
        // TODO: Get from WalletManager
        return "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    }

    private func getCurrentChainId() async throws -> String {
        // TODO: Get from network configuration
        return "1"
    }

    // MARK: - Storage

    private func saveSessions() {
        if let encoded = try? JSONEncoder().encode(sessions) {
            UserDefaults.standard.set(encoded, forKey: storageKey)
        }
    }

    private func loadSessions() {
        if let data = UserDefaults.standard.data(forKey: storageKey),
           let decoded = try? JSONDecoder().decode([WCSession].self, from: data) {
            sessions = decoded.filter { !$0.isExpired }
        }
    }

    private func saveProposals() {
        if let encoded = try? JSONEncoder().encode(proposals) {
            UserDefaults.standard.set(encoded, forKey: proposalsKey)
        }
    }

    private func loadProposals() {
        if let data = UserDefaults.standard.data(forKey: proposalsKey),
           let decoded = try? JSONDecoder().decode([WCProposal].self, from: data) {
            proposals = decoded.filter { !$0.isExpired }
        }
    }
}
