//
//  DAppBrowserView.swift
//  EtridWallet
//
//  dApp browser with Web3 provider injection
//

import SwiftUI
import WebKit

struct DAppBrowserView: View {
    @StateObject private var service = DAppBrowserService.shared
    @State private var urlString = ""
    @State private var currentURL: URL?
    @State private var canGoBack = false
    @State private var canGoForward = false
    @State private var isLoading = false
    @State private var showBookmarks = false
    @State private var showConnectedDApps = false
    @State private var showTransactionApproval = false
    @State private var estimatedProgress: Double = 0

    var body: some View {
        VStack(spacing: 0) {
            // Navigation Bar
            navigationBar

            // Progress Bar
            if isLoading {
                ProgressView(value: estimatedProgress)
                    .progressViewStyle(.linear)
            }

            // WebView
            Web3WebView(
                url: $currentURL,
                canGoBack: $canGoBack,
                canGoForward: $canGoForward,
                isLoading: $isLoading,
                estimatedProgress: $estimatedProgress,
                onRequestReceived: handleWeb3Request
            )

            // Popular dApps (shown when no URL)
            if currentURL == nil {
                popularDAppsGrid
            }
        }
        .sheet(isPresented: $showBookmarks) {
            BookmarksView()
        }
        .sheet(isPresented: $showConnectedDApps) {
            ConnectedDAppsView()
        }
        .sheet(isPresented: $showTransactionApproval) {
            if let request = service.pendingRequest {
                TransactionApprovalView(request: request)
            }
        }
        .onChange(of: service.pendingRequest) { newValue in
            showTransactionApproval = newValue != nil
        }
    }

    private var navigationBar: some View {
        VStack(spacing: 8) {
            // Top Controls
            HStack {
                Button(action: goBack) {
                    Image(systemName: "chevron.left")
                        .foregroundColor(canGoBack ? .primary : .gray)
                }
                .disabled(!canGoBack)

                Button(action: goForward) {
                    Image(systemName: "chevron.right")
                        .foregroundColor(canGoForward ? .primary : .gray)
                }
                .disabled(!canGoForward)

                Spacer()

                Button(action: { showBookmarks = true }) {
                    Image(systemName: "book")
                }

                Button(action: { showConnectedDApps = true }) {
                    Image(systemName: "link")
                        .overlay(
                            service.connectedDApps.isEmpty ? nil :
                            Circle()
                                .fill(Color.green)
                                .frame(width: 8, height: 8)
                                .offset(x: 6, y: -6)
                        )
                }

                Button(action: reload) {
                    Image(systemName: "arrow.clockwise")
                }
            }
            .padding(.horizontal)

            // URL Bar
            HStack {
                Image(systemName: isSecure ? "lock.fill" : "lock.open.fill")
                    .foregroundColor(isSecure ? .green : .red)
                    .font(.caption)

                TextField("Enter URL or search", text: $urlString)
                    .textFieldStyle(.roundedBorder)
                    .autocapitalization(.none)
                    .keyboardType(.URL)
                    .onSubmit(navigate)

                if !urlString.isEmpty {
                    Button(action: { urlString = "" }) {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundColor(.gray)
                    }
                }
            }
            .padding(.horizontal)
        }
        .padding(.vertical, 8)
        .background(Color(.systemBackground))
    }

    private var popularDAppsGrid: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                Text("Popular dApps")
                    .font(.title2)
                    .bold()
                    .padding(.horizontal)

                LazyVGrid(columns: [
                    GridItem(.flexible()),
                    GridItem(.flexible())
                ], spacing: 16) {
                    ForEach(DApp.popularDApps) { dapp in
                        DAppCard(dapp: dapp) {
                            navigateTo(url: dapp.url)
                        }
                    }
                }
                .padding(.horizontal)
            }
            .padding(.top)
        }
    }

    private var isSecure: Bool {
        currentURL?.scheme == "https"
    }

    // MARK: - Actions

    private func navigate() {
        guard !urlString.isEmpty else { return }

        var urlToLoad = urlString
        if !urlToLoad.hasPrefix("http://") && !urlToLoad.hasPrefix("https://") {
            urlToLoad = "https://" + urlToLoad
        }

        if let url = URL(string: urlToLoad) {
            // Check phishing
            let phishingCheck = service.checkPhishing(url: url.absoluteString)
            if !phishingCheck.isSafe {
                // Show warning
                print("Warning: Potentially unsafe site")
            }

            currentURL = url
        }
    }

    private func navigateTo(url: String) {
        urlString = url
        navigate()
    }

    private func goBack() {
        // Trigger WebView goBack
    }

    private func goForward() {
        // Trigger WebView goForward
    }

    private func reload() {
        // Trigger WebView reload
    }

    private func handleWeb3Request(_ request: Web3Request) {
        Task {
            do {
                _ = try await service.handleWeb3Request(request)
                // Response will be handled by the service
            } catch {
                print("Web3 request error: \(error)")
            }
        }
    }
}

// MARK: - DApp Card

struct DAppCard: View {
    let dapp: DApp
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 12) {
                // Icon
                Circle()
                    .fill(Color.blue.opacity(0.1))
                    .frame(width: 60, height: 60)
                    .overlay(
                        Text(String(dapp.name.prefix(1)))
                            .font(.title)
                            .foregroundColor(.blue)
                    )

                VStack(spacing: 4) {
                    Text(dapp.name)
                        .font(.headline)
                        .lineLimit(1)

                    Text(dapp.category.rawValue)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Web3 WebView

struct Web3WebView: UIViewRepresentable {
    @Binding var url: URL?
    @Binding var canGoBack: Bool
    @Binding var canGoForward: Bool
    @Binding var isLoading: Bool
    @Binding var estimatedProgress: Double
    let onRequestReceived: (Web3Request) -> Void

    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }

    func makeUIView(context: Context) -> WKWebView {
        let config = WKWebViewConfiguration()

        // Add message handler for Web3 requests
        config.userContentController.add(context.coordinator, name: "web3")

        let webView = WKWebView(frame: .zero, configuration: config)
        webView.navigationDelegate = context.coordinator
        webView.allowsBackForwardNavigationGestures = true

        return webView
    }

    func updateUIView(_ webView: WKWebView, context: Context) {
        if let url = url, webView.url != url {
            let request = URLRequest(url: url)
            webView.load(request)

            // Inject Web3 provider
            Task {
                try? await DAppBrowserService.shared.injectWeb3Provider(into: webView)
            }
        }

        canGoBack = webView.canGoBack
        canGoForward = webView.canGoForward
    }

    class Coordinator: NSObject, WKNavigationDelegate, WKScriptMessageHandler {
        let parent: Web3WebView

        init(_ parent: Web3WebView) {
            self.parent = parent
        }

        func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
            parent.isLoading = true
        }

        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
            parent.isLoading = false
            parent.estimatedProgress = 1.0

            // Inject Web3 provider after page load
            Task {
                try? await DAppBrowserService.shared.injectWeb3Provider(into: webView)
            }
        }

        func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
            parent.isLoading = false
        }

        func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
            guard message.name == "web3",
                  let body = message.body as? [String: Any],
                  let type = body["type"] as? String,
                  type == "web3_request",
                  let data = body["data"] as? [String: Any],
                  let requestId = data["id"] as? String,
                  let method = data["method"] as? String else {
                return
            }

            let params = data["params"] as? [Any] ?? []
            let dAppUrl = message.webView?.url?.absoluteString ?? ""
            let dAppName = message.webView?.url?.host ?? "Unknown"

            let request = Web3Request(
                id: requestId,
                method: method,
                params: params,
                dAppUrl: dAppUrl,
                dAppName: dAppName,
                requestedAt: Date()
            )

            parent.onRequestReceived(request)
        }
    }
}

#Preview {
    DAppBrowserView()
}
