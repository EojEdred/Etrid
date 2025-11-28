//
//  WalletConnectService.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import Foundation
import Combine
import WalletConnectSwift

/// Manages WalletConnect v2 protocol integration
@MainActor
final class WalletConnectService: ObservableObject {
    // MARK: - Published Properties

    @Published private(set) var activeSessions: [WalletConnectSession] = []
    @Published private(set) var pendingProposals: [SessionProposal] = []
    @Published private(set) var isInitialized: Bool = false
    @Published var currentRequest: SessionRequest?
    @Published var currentProposal: SessionProposal?

    // MARK: - Private Properties

    private var client: WalletConnectClient?
    private var requestHandler: WalletConnectRequestHandler?
    private var cancellables = Set<AnyCancellable>()
    private let sessionStore = WalletConnectSessionStore()
    private let projectId: String
    private let logger = Logger(subsystem: "io.etrid.wallet", category: "WalletConnect")

    // MARK: - Dependencies

    private let keychainManager: KeychainManager
    private let networkManager: NetworkManager

    // MARK: - Constants

    private struct Constants {
        static let relayHost = "relay.walletconnect.com"
        static let appName = "Ëtrid Wallet"
        static let appDescription = "Secure multi-chain wallet for Ëtrid ecosystem"
        static let appURL = "https://etrid.org"
        static let appIcon = "https://etrid.org/assets/icon.png"
    }

    // MARK: - Initialization

    init(
        projectId: String,
        keychainManager: KeychainManager,
        networkManager: NetworkManager
    ) {
        self.projectId = projectId
        self.keychainManager = keychainManager
        self.networkManager = networkManager
    }

    // MARK: - Public Methods

    /// Initialize WalletConnect client
    func initialize() async throws {
        guard !isInitialized else {
            logger.info("WalletConnect already initialized")
            return
        }

        do {
            // Setup metadata
            let metadata = AppMetadata(
                name: Constants.appName,
                description: Constants.appDescription,
                url: Constants.appURL,
                icons: [Constants.appIcon],
                redirect: try buildRedirectURL()
            )

            // Create client
            let clientConfig = WalletConnectClientConfig(
                metadata: metadata,
                projectId: projectId,
                relayHost: Constants.relayHost,
                socketConnectionType: .automatic
            )

            client = try await WalletConnectClient.create(config: clientConfig)

            // Initialize request handler
            requestHandler = WalletConnectRequestHandler(
                keychainManager: keychainManager,
                networkManager: networkManager,
                walletConnectService: self
            )

            // Setup event listeners
            setupListeners()

            // Restore existing sessions
            await restoreSessions()

            isInitialized = true
            logger.info("WalletConnect initialized successfully")

        } catch {
            logger.error("Failed to initialize WalletConnect: \(error.localizedDescription)")
            throw WalletConnectError.initializationFailed(error)
        }
    }

    /// Pair with dApp using URI
    func pair(uri: String) async throws {
        guard let client = client else {
            throw WalletConnectError.clientNotInitialized
        }

        do {
            try await client.pair(uri: uri)
            logger.info("Successfully paired with URI")
        } catch {
            logger.error("Failed to pair: \(error.localizedDescription)")
            throw WalletConnectError.pairingFailed(error)
        }
    }

    /// Pair with dApp using WalletConnect URI from deep link
    func handleDeepLink(url: URL) async throws {
        guard url.scheme == "wc" || url.absoluteString.contains("wc?uri=") else {
            throw WalletConnectError.invalidDeepLink
        }

        let uri: String
        if url.scheme == "wc" {
            uri = url.absoluteString
        } else if let uriParam = url.queryParameters["uri"] {
            uri = uriParam
        } else {
            throw WalletConnectError.invalidDeepLink
        }

        try await pair(uri: uri)
    }

    /// Approve session proposal
    func approveSession(
        _ proposal: SessionProposal,
        accounts: [Account]
    ) async throws {
        guard let client = client else {
            throw WalletConnectError.clientNotInitialized
        }

        do {
            // Build session namespaces
            let namespaces = try buildSessionNamespaces(
                from: proposal.requiredNamespaces,
                accounts: accounts
            )

            // Approve session
            let session = try await client.approve(
                proposalId: proposal.id,
                namespaces: namespaces
            )

            // Create and store session
            let wcSession = WalletConnectSession(
                id: session.topic,
                topic: session.topic,
                dAppMetadata: DAppMetadata(
                    name: proposal.proposer.name,
                    description: proposal.proposer.description,
                    url: proposal.proposer.url,
                    icons: proposal.proposer.icons
                ),
                accounts: accounts.map { $0.address },
                chains: namespaces.flatMap { $0.value.chains ?? [] },
                createdAt: Date(),
                expiresAt: session.expiryDate
            )

            activeSessions.append(wcSession)
            try await sessionStore.save(wcSession)

            // Remove from pending
            pendingProposals.removeAll { $0.id == proposal.id }
            currentProposal = nil

            logger.info("Session approved: \(session.topic)")

        } catch {
            logger.error("Failed to approve session: \(error.localizedDescription)")
            throw WalletConnectError.sessionApprovalFailed(error)
        }
    }

    /// Reject session proposal
    func rejectSession(_ proposal: SessionProposal) async throws {
        guard let client = client else {
            throw WalletConnectError.clientNotInitialized
        }

        do {
            try await client.reject(
                proposalId: proposal.id,
                reason: .userRejected
            )

            pendingProposals.removeAll { $0.id == proposal.id }
            currentProposal = nil

            logger.info("Session rejected: \(proposal.id)")

        } catch {
            logger.error("Failed to reject session: \(error.localizedDescription)")
            throw WalletConnectError.sessionRejectionFailed(error)
        }
    }

    /// Disconnect session
    func disconnectSession(_ session: WalletConnectSession) async throws {
        guard let client = client else {
            throw WalletConnectError.clientNotInitialized
        }

        do {
            try await client.disconnect(topic: session.topic)

            activeSessions.removeAll { $0.id == session.id }
            try await sessionStore.delete(session)

            logger.info("Session disconnected: \(session.topic)")

        } catch {
            logger.error("Failed to disconnect session: \(error.localizedDescription)")
            throw WalletConnectError.disconnectionFailed(error)
        }
    }

    /// Respond to session request
    func respondToRequest(
        _ request: SessionRequest,
        response: JSONRPCResponse
    ) async throws {
        guard let client = client else {
            throw WalletConnectError.clientNotInitialized
        }

        do {
            try await client.respond(
                topic: request.topic,
                requestId: request.id,
                response: response
            )

            currentRequest = nil
            logger.info("Responded to request: \(request.id)")

        } catch {
            logger.error("Failed to respond to request: \(error.localizedDescription)")
            throw WalletConnectError.responseFailed(error)
        }
    }

    /// Get active session by topic
    func getSession(topic: String) -> WalletConnectSession? {
        return activeSessions.first { $0.topic == topic }
    }

    // MARK: - Private Methods

    private func setupListeners() {
        guard let client = client else { return }

        // Session proposal listener
        client.sessionProposalPublisher
            .receive(on: DispatchQueue.main)
            .sink { [weak self] proposal in
                Task { @MainActor [weak self] in
                    await self?.handleSessionProposal(proposal)
                }
            }
            .store(in: &cancellables)

        // Session request listener
        client.sessionRequestPublisher
            .receive(on: DispatchQueue.main)
            .sink { [weak self] request in
                Task { @MainActor [weak self] in
                    await self?.handleSessionRequest(request)
                }
            }
            .store(in: &cancellables)

        // Session delete listener
        client.sessionDeletePublisher
            .receive(on: DispatchQueue.main)
            .sink { [weak self] session in
                Task { @MainActor [weak self] in
                    await self?.handleSessionDelete(session)
                }
            }
            .store(in: &cancellables)

        // Session update listener
        client.sessionUpdatePublisher
            .receive(on: DispatchQueue.main)
            .sink { [weak self] update in
                Task { @MainActor [weak self] in
                    await self?.handleSessionUpdate(update)
                }
            }
            .store(in: &cancellables)
    }

    private func handleSessionProposal(_ proposal: SessionProposal) async {
        logger.info("Received session proposal from: \(proposal.proposer.name)")

        pendingProposals.append(proposal)
        currentProposal = proposal

        // Post notification for UI
        NotificationCenter.default.post(
            name: .walletConnectSessionProposal,
            object: proposal
        )
    }

    private func handleSessionRequest(_ request: SessionRequest) async {
        logger.info("Received session request: \(request.method)")

        guard let requestHandler = requestHandler else {
            logger.error("Request handler not initialized")
            return
        }

        // Validate session exists
        guard let session = getSession(topic: request.topic) else {
            logger.error("Session not found for request")
            do {
                try await respondToRequest(
                    request,
                    response: .error(.sessionNotFound)
                )
            } catch {
                logger.error("Failed to send error response: \(error.localizedDescription)")
            }
            return
        }

        currentRequest = request

        // Handle request based on method
        do {
            let response = try await requestHandler.handleRequest(request, session: session)
            try await respondToRequest(request, response: response)
        } catch {
            logger.error("Request handling failed: \(error.localizedDescription)")

            let errorResponse: JSONRPCResponse
            if let wcError = error as? WalletConnectError {
                errorResponse = .error(wcError.toJSONRPCError())
            } else {
                errorResponse = .error(.internalError)
            }

            do {
                try await respondToRequest(request, response: errorResponse)
            } catch {
                logger.error("Failed to send error response: \(error.localizedDescription)")
            }
        }
    }

    private func handleSessionDelete(_ session: Session) async {
        logger.info("Session deleted: \(session.topic)")

        activeSessions.removeAll { $0.topic == session.topic }

        if let wcSession = activeSessions.first(where: { $0.topic == session.topic }) {
            try? await sessionStore.delete(wcSession)
        }

        NotificationCenter.default.post(
            name: .walletConnectSessionDeleted,
            object: session.topic
        )
    }

    private func handleSessionUpdate(_ update: SessionUpdate) async {
        logger.info("Session updated: \(update.topic)")

        // Update session in storage
        if let index = activeSessions.firstIndex(where: { $0.topic == update.topic }) {
            // Update session data
            try? await sessionStore.save(activeSessions[index])
        }
    }

    private func restoreSessions() async {
        do {
            let sessions = try await sessionStore.loadAll()

            // Filter out expired sessions
            let now = Date()
            let validSessions = sessions.filter { $0.expiresAt > now }

            activeSessions = validSessions

            logger.info("Restored \(validSessions.count) active sessions")

            // Clean up expired sessions
            let expiredSessions = sessions.filter { $0.expiresAt <= now }
            for session in expiredSessions {
                try? await sessionStore.delete(session)
            }

        } catch {
            logger.error("Failed to restore sessions: \(error.localizedDescription)")
        }
    }

    private func buildSessionNamespaces(
        from requiredNamespaces: [String: ProposalNamespace],
        accounts: [Account]
    ) throws -> [String: SessionNamespace] {
        var namespaces: [String: SessionNamespace] = [:]

        for (key, proposalNamespace) in requiredNamespaces {
            // Build account strings (CAIP-10 format: namespace:chainId:address)
            let accountStrings = proposalNamespace.chains.flatMap { chain in
                accounts.map { account in
                    "\(key):\(chain):\(account.address)"
                }
            }

            namespaces[key] = SessionNamespace(
                accounts: Set(accountStrings),
                methods: Set(proposalNamespace.methods),
                events: Set(proposalNamespace.events),
                chains: proposalNamespace.chains
            )
        }

        return namespaces
    }

    private func buildRedirectURL() throws -> String {
        guard let bundleId = Bundle.main.bundleIdentifier else {
            throw WalletConnectError.invalidConfiguration
        }
        return "\(bundleId)://walletconnect"
    }
}

// MARK: - Session Store

private actor WalletConnectSessionStore {
    private let userDefaults = UserDefaults.standard
    private let key = "io.etrid.wallet.wc.sessions"

    func save(_ session: WalletConnectSession) throws {
        var sessions = try loadAll()
        sessions.removeAll { $0.id == session.id }
        sessions.append(session)

        let data = try JSONEncoder().encode(sessions)
        userDefaults.set(data, forKey: key)
    }

    func loadAll() throws -> [WalletConnectSession] {
        guard let data = userDefaults.data(forKey: key) else {
            return []
        }
        return try JSONDecoder().decode([WalletConnectSession].self, from: data)
    }

    func delete(_ session: WalletConnectSession) throws {
        var sessions = try loadAll()
        sessions.removeAll { $0.id == session.id }

        let data = try JSONEncoder().encode(sessions)
        userDefaults.set(data, forKey: key)
    }
}

// MARK: - URL Extension

private extension URL {
    var queryParameters: [String: String] {
        guard let components = URLComponents(url: self, resolvingAgainstBaseURL: false),
              let queryItems = components.queryItems else {
            return [:]
        }

        return Dictionary(
            queryItems.compactMap { item -> (String, String)? in
                guard let value = item.value else { return nil }
                return (item.name, value)
            },
            uniquingKeysWith: { first, _ in first }
        )
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let walletConnectSessionProposal = Notification.Name("walletConnectSessionProposal")
    static let walletConnectSessionDeleted = Notification.Name("walletConnectSessionDeleted")
    static let walletConnectRequestReceived = Notification.Name("walletConnectRequestReceived")
}
