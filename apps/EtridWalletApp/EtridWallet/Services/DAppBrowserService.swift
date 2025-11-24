//
//  DAppBrowserService.swift
//  EtridWallet
//
//  Service for dApp browser and Web3 provider injection
//

import Foundation
import WebKit
import Combine

@MainActor
class DAppBrowserService: ObservableObject {
    static let shared = DAppBrowserService()

    @Published var connectedDApps: [ConnectedDApp] = []
    @Published var bookmarks: [Bookmark] = []
    @Published var pendingRequest: Web3Request?
    @Published var isProcessingRequest = false

    private var requestHandlers: [String: (Result<String, Error>) -> Void] = [:]
    private let storageKey = "connected_dapps"
    private let bookmarksKey = "dapp_bookmarks"

    private init() {
        loadConnectedDApps()
        loadBookmarks()
    }

    // MARK: - Web3 Provider Injection

    func injectWeb3Provider(into webView: WKWebView) async throws {
        let providerScript = """
        (function() {
            const etridProvider = {
                isEtrid: true,
                isMetaMask: true,
                chainId: null,
                selectedAddress: null,
                networkVersion: null,

                request: async function(args) {
                    return new Promise((resolve, reject) => {
                        const requestId = Date.now().toString() + Math.random().toString();
                        const request = {
                            id: requestId,
                            method: args.method,
                            params: args.params || []
                        };

                        window.webkit.messageHandlers.web3.postMessage({
                            type: 'web3_request',
                            data: request
                        });

                        window.__etridRequests = window.__etridRequests || {};
                        window.__etridRequests[requestId] = { resolve, reject };

                        setTimeout(() => {
                            if (window.__etridRequests[requestId]) {
                                reject(new Error('Request timeout'));
                                delete window.__etridRequests[requestId];
                            }
                        }, 60000);
                    });
                },

                sendAsync: function(payload, callback) {
                    this.request(payload)
                        .then(result => callback(null, { id: payload.id, jsonrpc: '2.0', result }))
                        .catch(error => callback(error, null));
                },

                send: function(methodOrPayload, paramsOrCallback) {
                    if (typeof methodOrPayload === 'string') {
                        return this.request({ method: methodOrPayload, params: paramsOrCallback });
                    }
                    return this.sendAsync(methodOrPayload, paramsOrCallback);
                },

                on: function(event, handler) {
                    window.addEventListener('etrid_' + event, handler);
                },

                removeListener: function(event, handler) {
                    window.removeEventListener('etrid_' + event, handler);
                },

                isConnected: function() {
                    return true;
                }
            };

            window.ethereum = etridProvider;
            window.web3 = { currentProvider: etridProvider };

            window.addEventListener('message', function(event) {
                try {
                    const message = event.data;
                    if (message && message.type === 'web3_response') {
                        const { requestId, result, error } = message.data;
                        const request = window.__etridRequests[requestId];
                        if (request) {
                            if (error) {
                                request.reject(new Error(error));
                            } else {
                                request.resolve(result);
                            }
                            delete window.__etridRequests[requestId];
                        }
                    }
                } catch (e) {
                    console.error('Web3 response error:', e);
                }
            });

            window.dispatchEvent(new Event('ethereum#initialized'));
        })();
        """

        try await webView.evaluateJavaScript(providerScript)
    }

    // MARK: - Request Handling

    func handleWeb3Request(_ request: Web3Request) async throws -> String {
        switch request.method {
        case "eth_requestAccounts":
            return try await handleRequestAccounts(request)
        case "eth_accounts":
            return try await handleGetAccounts(request)
        case "eth_chainId":
            return try await handleGetChainId()
        case "eth_sendTransaction":
            return try await handleSendTransaction(request)
        case "personal_sign":
            return try await handlePersonalSign(request)
        case "eth_signTypedData", "eth_signTypedData_v4":
            return try await handleSignTypedData(request)
        default:
            throw WalletError.unknown("Unsupported method: \(request.method)")
        }
    }

    private func handleRequestAccounts(_ request: Web3Request) async throws -> String {
        // Check if already connected
        if let connected = connectedDApps.first(where: { $0.dAppUrl == request.dAppUrl }) {
            let address = try await getCurrentAddress()
            return "[\"\(address)\"]"
        }

        // Request user approval
        pendingRequest = request
        let approved = try await waitForUserApproval()

        guard approved else {
            throw WalletError.unknown("User rejected")
        }

        // Add to connected dApps
        let connection = ConnectedDApp(
            id: UUID().uuidString,
            dAppUrl: request.dAppUrl,
            dAppName: request.dAppName,
            dAppIcon: nil,
            permissions: [DAppPermission(type: .accessAddress, grantedAt: Date())],
            connectedAt: Date(),
            lastUsed: Date()
        )
        connectedDApps.append(connection)
        saveConnectedDApps()

        let address = try await getCurrentAddress()
        return "[\"\(address)\"]"
    }

    private func handleGetAccounts(_ request: Web3Request) async throws -> String {
        if connectedDApps.contains(where: { $0.dAppUrl == request.dAppUrl }) {
            let address = try await getCurrentAddress()
            return "[\"\(address)\"]"
        }
        return "[]"
    }

    private func handleGetChainId() async throws -> String {
        // Return Etrid chain ID or current network
        return "\"0x1\"" // Ethereum mainnet format
    }

    private func handleSendTransaction(_ request: Web3Request) async throws -> String {
        pendingRequest = request
        let approved = try await waitForUserApproval()

        guard approved else {
            throw WalletError.unknown("User rejected")
        }

        // TODO: Integrate with TransactionManager
        let txHash = "0x" + UUID().uuidString.replacingOccurrences(of: "-", with: "")
        return "\"\(txHash)\""
    }

    private func handlePersonalSign(_ request: Web3Request) async throws -> String {
        pendingRequest = request
        let approved = try await waitForUserApproval()

        guard approved else {
            throw WalletError.unknown("User rejected")
        }

        // TODO: Integrate with CryptoManager for signing
        let signature = "0x" + String(repeating: "0", count: 130)
        return "\"\(signature)\""
    }

    private func handleSignTypedData(_ request: Web3Request) async throws -> String {
        pendingRequest = request
        let approved = try await waitForUserApproval()

        guard approved else {
            throw WalletError.unknown("User rejected")
        }

        // TODO: Implement EIP-712 signing
        let signature = "0x" + String(repeating: "0", count: 130)
        return "\"\(signature)\""
    }

    // MARK: - Response Handling

    func sendResponse(_ response: Web3Response, to webView: WKWebView) {
        let responseJSON: String
        if let result = response.result {
            responseJSON = "{\"type\":\"web3_response\",\"data\":{\"requestId\":\"\(response.requestId)\",\"result\":\(result)}}"
        } else if let error = response.error {
            responseJSON = "{\"type\":\"web3_response\",\"data\":{\"requestId\":\"\(response.requestId)\",\"error\":\"\(error)\"}}"
        } else {
            return
        }

        let script = "window.postMessage(\(responseJSON), '*');"
        webView.evaluateJavaScript(script) { _, error in
            if let error = error {
                print("Error sending response: \(error)")
            }
        }
    }

    // MARK: - Bookmarks

    func addBookmark(name: String, url: String, iconUrl: String?) {
        let bookmark = Bookmark(
            id: UUID().uuidString,
            name: name,
            url: url,
            iconUrl: iconUrl,
            createdAt: Date()
        )
        bookmarks.append(bookmark)
        saveBookmarks()
    }

    func removeBookmark(_ bookmark: Bookmark) {
        bookmarks.removeAll { $0.id == bookmark.id }
        saveBookmarks()
    }

    // MARK: - Connected DApps

    func disconnectDApp(_ dapp: ConnectedDApp) {
        connectedDApps.removeAll { $0.id == dapp.id }
        saveConnectedDApps()
    }

    // MARK: - Phishing Detection

    func checkPhishing(url: String) -> PhishingCheck {
        // Basic phishing detection
        let suspiciousPatterns = [
            "metamask-",
            "uniswap-",
            "opensea-",
            "coinbase-",
            ".tk",
            ".ml",
            ".ga"
        ]

        var warnings: [PhishingCheck.SecurityWarning] = []
        var risk = PhishingCheck.RiskLevel.safe

        for pattern in suspiciousPatterns {
            if url.lowercased().contains(pattern) {
                warnings.append(PhishingCheck.SecurityWarning(
                    title: "Suspicious Domain",
                    description: "This domain contains suspicious patterns",
                    severity: .warning
                ))
                risk = .medium
                break
            }
        }

        if !url.hasPrefix("https://") {
            warnings.append(PhishingCheck.SecurityWarning(
                title: "Insecure Connection",
                description: "This site does not use HTTPS",
                severity: .critical
            ))
            risk = .high
        }

        return PhishingCheck(
            url: url,
            isSafe: risk == .safe,
            warnings: warnings,
            risk: risk
        )
    }

    // MARK: - Helpers

    private func getCurrentAddress() async throws -> String {
        // TODO: Get from WalletManager
        return "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    }

    private func waitForUserApproval() async throws -> Bool {
        // Wait for user interaction (approved/rejected)
        // This would be triggered by UI interaction
        return true
    }

    // MARK: - Storage

    private func saveConnectedDApps() {
        if let encoded = try? JSONEncoder().encode(connectedDApps) {
            UserDefaults.standard.set(encoded, forKey: storageKey)
        }
    }

    private func loadConnectedDApps() {
        if let data = UserDefaults.standard.data(forKey: storageKey),
           let decoded = try? JSONDecoder().decode([ConnectedDApp].self, from: data) {
            connectedDApps = decoded
        }
    }

    private func saveBookmarks() {
        if let encoded = try? JSONEncoder().encode(bookmarks) {
            UserDefaults.standard.set(encoded, forKey: bookmarksKey)
        }
    }

    private func loadBookmarks() {
        if let data = UserDefaults.standard.data(forKey: bookmarksKey),
           let decoded = try? JSONDecoder().decode([Bookmark].self, from: data) {
            bookmarks = decoded
        }
    }
}
