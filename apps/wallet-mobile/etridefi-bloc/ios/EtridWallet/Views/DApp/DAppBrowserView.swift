//
//  DAppBrowserView.swift
//  EtridWallet
//
//  Native iOS implementation of DApp Browser with Web3 injection
//

import SwiftUI
import WebKit

struct DAppBrowserView: View {
    @StateObject private var viewModel = DAppBrowserViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showBookmarks = true
    @State private var urlText = ""

    var body: some View {
        NavigationView {
            VStack(spacing: 0) {
                // Search/URL Bar
                searchBar

                // Content Area
                if showBookmarks && viewModel.currentURL == nil {
                    bookmarksView
                } else {
                    webView
                }
            }
            .navigationTitle("DApp Browser")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button(action: { dismiss() }) {
                        Image(systemName: "chevron.left")
                            .foregroundColor(.blue)
                    }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    HStack(spacing: 16) {
                        // Network Selector
                        Menu {
                            ForEach(Web3Network.allNetworks) { network in
                                Button(action: {
                                    viewModel.switchNetwork(network)
                                }) {
                                    HStack {
                                        Text(network.name)
                                        if network.id == viewModel.currentNetwork.id {
                                            Image(systemName: "checkmark")
                                        }
                                    }
                                }
                            }
                        } label: {
                            Image(systemName: "network")
                                .foregroundColor(.blue)
                        }

                        Button(action: { viewModel.showAddBookmark = true }) {
                            Image(systemName: "plus")
                                .foregroundColor(.blue)
                        }
                    }
                }
            }
            .sheet(isPresented: $viewModel.showAddBookmark) {
                AddBookmarkView(viewModel: viewModel)
            }
            .sheet(isPresented: $viewModel.showConnectionRequest) {
                if let request = viewModel.pendingConnectionRequest {
                    ConnectionRequestView(
                        request: request,
                        onApprove: { viewModel.approveConnection() },
                        onReject: { viewModel.rejectConnection() }
                    )
                }
            }
            .sheet(isPresented: $viewModel.showSignRequest) {
                if let request = viewModel.pendingSignRequest {
                    SignRequestView(
                        request: request,
                        onApprove: { viewModel.approveSignRequest() },
                        onReject: { viewModel.rejectSignRequest() }
                    )
                }
            }
            .sheet(isPresented: $viewModel.showTransactionRequest) {
                if let request = viewModel.pendingTransactionRequest {
                    TransactionRequestView(
                        request: request,
                        network: viewModel.currentNetwork,
                        onApprove: { viewModel.approveTransaction() },
                        onReject: { viewModel.rejectTransaction() }
                    )
                }
            }
            .alert("Error", isPresented: $viewModel.showError) {
                Button("OK", role: .cancel) { }
            } message: {
                Text(viewModel.errorMessage)
            }
        }
    }

    // MARK: - Search Bar
    private var searchBar: some View {
        HStack(spacing: 12) {
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundColor(.secondary)

                TextField("Enter DApp URL or search...", text: $urlText)
                    .autocapitalization(.none)
                    .keyboardType(.URL)
                    .disableAutocorrection(true)

                if !urlText.isEmpty {
                    Button(action: { urlText = "" }) {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundColor(.secondary)
                    }
                }
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 10)
            .background(Color(.systemGray6))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(Color(.systemGray4), lineWidth: 1)
            )

            Button(action: {
                if !urlText.isEmpty {
                    viewModel.loadURL(urlText)
                    showBookmarks = false
                }
            }) {
                Text("Go")
                    .fontWeight(.semibold)
                    .foregroundColor(.white)
                    .padding(.horizontal, 20)
                    .padding(.vertical, 10)
                    .background(Color.blue)
                    .cornerRadius(12)
            }
        }
        .padding()
    }

    // MARK: - Bookmarks View
    private var bookmarksView: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                Text("Bookmarks")
                    .font(.headline)
                    .fontWeight(.bold)
                    .padding(.horizontal)

                ForEach(viewModel.bookmarks) { bookmark in
                    BookmarkCard(bookmark: bookmark) {
                        viewModel.loadURL(bookmark.url)
                        urlText = bookmark.url
                        showBookmarks = false
                    }
                }

                // Popular DApps Section
                VStack(alignment: .leading, spacing: 12) {
                    Text("Popular DApps")
                        .font(.headline)
                        .fontWeight(.bold)
                        .padding(.horizontal)

                    ForEach(viewModel.popularDApps) { dapp in
                        BookmarkCard(bookmark: dapp) {
                            viewModel.loadURL(dapp.url)
                            urlText = dapp.url
                            showBookmarks = false
                        }
                    }
                }
                .padding(.top)
            }
            .padding(.vertical)
        }
    }

    // MARK: - Web View
    private var webView: some View {
        VStack(spacing: 0) {
            // Navigation Controls
            HStack(spacing: 20) {
                Button(action: { viewModel.goBack() }) {
                    Image(systemName: "chevron.left")
                        .font(.title3)
                        .foregroundColor(viewModel.canGoBack ? .blue : .gray)
                }
                .disabled(!viewModel.canGoBack)

                Button(action: { viewModel.goForward() }) {
                    Image(systemName: "chevron.right")
                        .font(.title3)
                        .foregroundColor(viewModel.canGoForward ? .blue : .gray)
                }
                .disabled(!viewModel.canGoForward)

                Button(action: { viewModel.reload() }) {
                    Image(systemName: "arrow.clockwise")
                        .font(.title3)
                        .foregroundColor(.blue)
                }

                Spacer()

                Button(action: {
                    showBookmarks = true
                    viewModel.currentURL = nil
                }) {
                    Image(systemName: "house")
                        .font(.title3)
                        .foregroundColor(.blue)
                }

                Button(action: { /* Share */ }) {
                    Image(systemName: "square.and.arrow.up")
                        .font(.title3)
                        .foregroundColor(.blue)
                }
            }
            .padding()
            .background(Color(.systemGray6))

            // Web Content
            WebViewRepresentable(viewModel: viewModel)

            // Loading Indicator
            if viewModel.isLoading {
                ProgressView()
                    .progressViewStyle(LinearProgressViewStyle())
            }
        }
    }
}

// MARK: - Bookmark Card
struct BookmarkCard: View {
    let bookmark: DAppBookmark
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 12) {
                Text(bookmark.icon)
                    .font(.largeTitle)

                VStack(alignment: .leading, spacing: 4) {
                    Text(bookmark.name)
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.primary)

                    Text(bookmark.url)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .lineLimit(1)
                }

                Spacer()

                Image(systemName: "chevron.right")
                    .foregroundColor(.secondary)
            }
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
            .padding(.horizontal)
        }
    }
}

// MARK: - Add Bookmark View
struct AddBookmarkView: View {
    @ObservedObject var viewModel: DAppBrowserViewModel
    @Environment(\.dismiss) private var dismiss
    @State private var name = ""
    @State private var url = ""
    @State private var icon = "🌐"

    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Bookmark Details")) {
                    TextField("Name", text: $name)
                    TextField("URL", text: $url)
                        .autocapitalization(.none)
                        .keyboardType(.URL)
                    TextField("Icon (emoji)", text: $icon)
                }
            }
            .navigationTitle("Add Bookmark")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Save") {
                        viewModel.addBookmark(name: name, url: url, icon: icon)
                        dismiss()
                    }
                    .disabled(name.isEmpty || url.isEmpty)
                }
            }
        }
    }
}

// MARK: - WebView Representable
struct WebViewRepresentable: UIViewRepresentable {
    @ObservedObject var viewModel: DAppBrowserViewModel

    func makeUIView(context: Context) -> WKWebView {
        let configuration = WKWebViewConfiguration()
        configuration.preferences.javaScriptEnabled = true

        // Add Web3 message handler
        let contentController = WKUserContentController()
        contentController.add(context.coordinator, name: "etridWallet")
        configuration.userContentController = contentController

        // Inject Web3 provider script
        let web3Script = WKUserScript(
            source: Web3Provider.providerScript,
            injectionTime: .atDocumentStart,
            forMainFrameOnly: false
        )
        contentController.addUserScript(web3Script)

        let webView = WKWebView(frame: .zero, configuration: configuration)
        webView.navigationDelegate = context.coordinator
        webView.allowsBackForwardNavigationGestures = true

        viewModel.webView = webView
        return webView
    }

    func updateUIView(_ webView: WKWebView, context: Context) {
        // Updates handled through view model
    }

    func makeCoordinator() -> Coordinator {
        Coordinator(viewModel: viewModel)
    }

    class Coordinator: NSObject, WKNavigationDelegate, WKScriptMessageHandler {
        let viewModel: DAppBrowserViewModel

        init(viewModel: DAppBrowserViewModel) {
            self.viewModel = viewModel
        }

        // MARK: - WKNavigationDelegate

        func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
            viewModel.isLoading = true
        }

        func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
            viewModel.isLoading = false
            viewModel.canGoBack = webView.canGoBack
            viewModel.canGoForward = webView.canGoForward

            // Update current domain for DApp connection tracking
            if let url = webView.url {
                viewModel.updateCurrentDomain(url)
            }

            // Inject current account and network info
            viewModel.injectWalletState()
        }

        func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
            viewModel.isLoading = false
        }

        // MARK: - WKScriptMessageHandler

        func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
            guard message.name == "etridWallet",
                  let body = message.body as? [String: Any],
                  let method = body["method"] as? String,
                  let id = body["id"] as? Int else {
                return
            }

            let params = body["params"] as? [Any] ?? []

            // Handle Web3 RPC methods
            viewModel.handleWeb3Request(method: method, params: params, id: id)
        }
    }
}

// MARK: - View Model
class DAppBrowserViewModel: ObservableObject {
    @Published var bookmarks: [DAppBookmark] = [
        DAppBookmark(id: "1", name: "Uniswap", url: "https://app.uniswap.org", icon: "🦄"),
        DAppBookmark(id: "2", name: "Aave", url: "https://app.aave.com", icon: "👻"),
        DAppBookmark(id: "3", name: "OpenSea", url: "https://opensea.io", icon: "🌊")
    ]

    @Published var popularDApps: [DAppBookmark] = [
        DAppBookmark(id: "4", name: "PancakeSwap", url: "https://pancakeswap.finance", icon: "🥞"),
        DAppBookmark(id: "5", name: "Curve Finance", url: "https://curve.fi", icon: "📈"),
        DAppBookmark(id: "6", name: "Compound", url: "https://app.compound.finance", icon: "🏦"),
        DAppBookmark(id: "7", name: "SushiSwap", url: "https://app.sushi.com", icon: "🍣"),
        DAppBookmark(id: "8", name: "Yearn Finance", url: "https://yearn.finance", icon: "💰")
    ]

    @Published var currentURL: String?
    @Published var isLoading = false
    @Published var canGoBack = false
    @Published var canGoForward = false
    @Published var showAddBookmark = false

    // Web3 State
    @Published var currentNetwork = Web3Network.mainnet
    @Published var currentAccount: String?
    @Published var connectedDApps: [ConnectedDApp] = []

    // Request States
    @Published var showConnectionRequest = false
    @Published var showSignRequest = false
    @Published var showTransactionRequest = false
    @Published var showError = false

    @Published var pendingConnectionRequest: ConnectionRequest?
    @Published var pendingSignRequest: SignRequest?
    @Published var pendingTransactionRequest: TransactionRequest?
    @Published var errorMessage = ""

    private var currentDomain: String?
    private var pendingRequestId: Int?

    var webView: WKWebView?

    init() {
        loadApprovedDApps()
        loadCurrentAccount()
    }

    // MARK: - URL Navigation

    func loadURL(_ urlString: String) {
        var urlToLoad = urlString
        if !urlToLoad.hasPrefix("http://") && !urlToLoad.hasPrefix("https://") {
            urlToLoad = "https://" + urlToLoad
        }

        guard let url = URL(string: urlToLoad) else { return }
        currentURL = urlToLoad
        let request = URLRequest(url: url)
        webView?.load(request)
    }

    func goBack() {
        webView?.goBack()
    }

    func goForward() {
        webView?.goForward()
    }

    func reload() {
        webView?.reload()
    }

    func addBookmark(name: String, url: String, icon: String) {
        let newBookmark = DAppBookmark(
            id: UUID().uuidString,
            name: name,
            url: url,
            icon: icon
        )
        bookmarks.append(newBookmark)
    }

    // MARK: - Web3 State Management

    func updateCurrentDomain(_ url: URL) {
        currentDomain = url.host
    }

    func injectWalletState() {
        guard let webView = webView,
              let account = currentAccount else { return }

        let script = """
        if (window.ethereum) {
            window.ethereum._updateState({
                accounts: ['\(account)'],
                chainId: '\(currentNetwork.chainId)',
                networkVersion: '\(currentNetwork.networkVersion)',
                isConnected: \(isDAppConnected())
            });
        }
        """

        webView.evaluateJavaScript(script) { _, error in
            if let error = error {
                print("Error injecting wallet state: \(error)")
            }
        }
    }

    func switchNetwork(_ network: Web3Network) {
        currentNetwork = network
        injectWalletState()

        // Notify DApp of network change
        notifyNetworkChange()
    }

    func switchAccount(_ account: String) {
        currentAccount = account
        injectWalletState()

        // Notify DApp of account change
        notifyAccountChange()
    }

    private func isDAppConnected() -> Bool {
        guard let domain = currentDomain else { return false }
        return connectedDApps.contains { $0.domain == domain }
    }

    // MARK: - Web3 Request Handling

    func handleWeb3Request(method: String, params: [Any], id: Int) {
        pendingRequestId = id

        switch method {
        case "eth_requestAccounts", "eth_accounts":
            handleAccountsRequest()

        case "eth_chainId":
            sendResponse(id: id, result: currentNetwork.chainId)

        case "net_version":
            sendResponse(id: id, result: currentNetwork.networkVersion)

        case "personal_sign", "eth_sign":
            handleSignRequest(params: params, id: id)

        case "eth_signTypedData", "eth_signTypedData_v3", "eth_signTypedData_v4":
            handleSignTypedDataRequest(params: params, id: id)

        case "eth_sendTransaction":
            handleSendTransactionRequest(params: params, id: id)

        case "wallet_switchEthereumChain":
            handleSwitchChainRequest(params: params, id: id)

        case "wallet_addEthereumChain":
            handleAddChainRequest(params: params, id: id)

        default:
            sendError(id: id, message: "Method not supported: \(method)")
        }
    }

    private func handleAccountsRequest() {
        guard let domain = currentDomain else {
            sendError(id: pendingRequestId ?? 0, message: "Invalid domain")
            return
        }

        // Check if DApp is already connected
        if isDAppConnected() {
            if let account = currentAccount {
                sendResponse(id: pendingRequestId ?? 0, result: [account])
            } else {
                sendResponse(id: pendingRequestId ?? 0, result: [])
            }
            return
        }

        // Show connection approval dialog
        pendingConnectionRequest = ConnectionRequest(
            id: UUID().uuidString,
            domain: domain,
            icon: getFaviconURL(domain),
            requestedAt: Date()
        )
        showConnectionRequest = true
    }

    private func handleSignRequest(params: [Any], id: Int) {
        guard isDAppConnected(),
              params.count >= 1,
              let message = params[0] as? String else {
            sendError(id: id, message: "Invalid sign request")
            return
        }

        pendingSignRequest = SignRequest(
            id: UUID().uuidString,
            domain: currentDomain ?? "Unknown",
            message: message,
            type: .personalSign,
            requestedAt: Date()
        )
        showSignRequest = true
    }

    private func handleSignTypedDataRequest(params: [Any], id: Int) {
        guard isDAppConnected(),
              params.count >= 2,
              let typedData = params[1] as? String else {
            sendError(id: id, message: "Invalid typed data request")
            return
        }

        pendingSignRequest = SignRequest(
            id: UUID().uuidString,
            domain: currentDomain ?? "Unknown",
            message: typedData,
            type: .signTypedData,
            requestedAt: Date()
        )
        showSignRequest = true
    }

    private func handleSendTransactionRequest(params: [Any], id: Int) {
        guard isDAppConnected(),
              params.count >= 1,
              let txParams = params[0] as? [String: Any] else {
            sendError(id: id, message: "Invalid transaction request")
            return
        }

        pendingTransactionRequest = TransactionRequest(
            id: UUID().uuidString,
            domain: currentDomain ?? "Unknown",
            from: txParams["from"] as? String ?? "",
            to: txParams["to"] as? String ?? "",
            value: txParams["value"] as? String ?? "0x0",
            data: txParams["data"] as? String,
            gas: txParams["gas"] as? String,
            gasPrice: txParams["gasPrice"] as? String,
            requestedAt: Date()
        )
        showTransactionRequest = true
    }

    private func handleSwitchChainRequest(params: [Any], id: Int) {
        guard params.count >= 1,
              let chainParams = params[0] as? [String: Any],
              let chainId = chainParams["chainId"] as? String else {
            sendError(id: id, message: "Invalid chain switch request")
            return
        }

        if let network = Web3Network.allNetworks.first(where: { $0.chainId == chainId }) {
            switchNetwork(network)
            sendResponse(id: id, result: nil)
        } else {
            sendError(id: id, message: "Chain not supported")
        }
    }

    private func handleAddChainRequest(params: [Any], id: Int) {
        // For now, reject custom chains
        sendError(id: id, message: "Adding custom chains not yet supported")
    }

    // MARK: - Request Approval/Rejection

    func approveConnection() {
        guard let request = pendingConnectionRequest,
              let account = currentAccount,
              let id = pendingRequestId else {
            return
        }

        // Save approved DApp
        let connectedDApp = ConnectedDApp(
            id: UUID().uuidString,
            domain: request.domain,
            icon: request.icon,
            connectedAt: Date()
        )
        connectedDApps.append(connectedDApp)
        saveApprovedDApps()

        // Send response
        sendResponse(id: id, result: [account])

        // Update state
        injectWalletState()

        // Clean up
        pendingConnectionRequest = nil
        showConnectionRequest = false
    }

    func rejectConnection() {
        guard let id = pendingRequestId else { return }

        sendError(id: id, message: "User rejected connection")

        pendingConnectionRequest = nil
        showConnectionRequest = false
    }

    func approveSignRequest() {
        guard let request = pendingSignRequest,
              let id = pendingRequestId else {
            return
        }

        Task {
            do {
                // Sign the message
                let signature = try await signMessage(request.message, type: request.type)

                await MainActor.run {
                    sendResponse(id: id, result: signature)
                    pendingSignRequest = nil
                    showSignRequest = false
                }
            } catch {
                await MainActor.run {
                    sendError(id: id, message: "Signing failed: \(error.localizedDescription)")
                    pendingSignRequest = nil
                    showSignRequest = false
                }
            }
        }
    }

    func rejectSignRequest() {
        guard let id = pendingRequestId else { return }

        sendError(id: id, message: "User rejected signature request")

        pendingSignRequest = nil
        showSignRequest = false
    }

    func approveTransaction() {
        guard let request = pendingTransactionRequest,
              let id = pendingRequestId else {
            return
        }

        Task {
            do {
                // Send transaction
                let txHash = try await sendTransaction(request)

                await MainActor.run {
                    sendResponse(id: id, result: txHash)
                    pendingTransactionRequest = nil
                    showTransactionRequest = false
                }
            } catch {
                await MainActor.run {
                    sendError(id: id, message: "Transaction failed: \(error.localizedDescription)")
                    pendingTransactionRequest = nil
                    showTransactionRequest = false
                }
            }
        }
    }

    func rejectTransaction() {
        guard let id = pendingRequestId else { return }

        sendError(id: id, message: "User rejected transaction")

        pendingTransactionRequest = nil
        showTransactionRequest = false
    }

    // MARK: - Response Handling

    private func sendResponse(id: Int, result: Any?) {
        guard let webView = webView else { return }

        let resultJSON: String
        if let result = result {
            if let stringResult = result as? String {
                resultJSON = "\"\(stringResult)\""
            } else if let arrayResult = result as? [String] {
                let items = arrayResult.map { "\"\($0)\"" }.joined(separator: ",")
                resultJSON = "[\(items)]"
            } else {
                resultJSON = "\(result)"
            }
        } else {
            resultJSON = "null"
        }

        let script = """
        if (window.ethereum && window.ethereum._handleResponse) {
            window.ethereum._handleResponse({
                id: \(id),
                result: \(resultJSON)
            });
        }
        """

        webView.evaluateJavaScript(script) { _, error in
            if let error = error {
                print("Error sending response: \(error)")
            }
        }
    }

    private func sendError(id: Int, message: String) {
        guard let webView = webView else { return }

        let script = """
        if (window.ethereum && window.ethereum._handleResponse) {
            window.ethereum._handleResponse({
                id: \(id),
                error: {
                    code: 4001,
                    message: "\(message)"
                }
            });
        }
        """

        webView.evaluateJavaScript(script) { _, error in
            if let error = error {
                print("Error sending error response: \(error)")
            }
        }
    }

    // MARK: - Event Notifications

    private func notifyAccountChange() {
        guard let webView = webView,
              let account = currentAccount else { return }

        let script = """
        if (window.ethereum && window.ethereum._emit) {
            window.ethereum._emit('accountsChanged', ['\(account)']);
        }
        """

        webView.evaluateJavaScript(script)
    }

    private func notifyNetworkChange() {
        guard let webView = webView else { return }

        let script = """
        if (window.ethereum && window.ethereum._emit) {
            window.ethereum._emit('chainChanged', '\(currentNetwork.chainId)');
            window.ethereum._emit('networkChanged', '\(currentNetwork.networkVersion)');
        }
        """

        webView.evaluateJavaScript(script)
    }

    // MARK: - Crypto Operations

    private func signMessage(_ message: String, type: SignType) async throws -> String {
        // TODO: Implement actual message signing using wallet private key
        // For now, return a mock signature
        // In production, this should:
        // 1. Get private key from keychain
        // 2. Hash the message appropriately
        // 3. Sign with the private key
        // 4. Return the signature

        return "0x" + String(repeating: "0", count: 130) // Mock signature
    }

    private func sendTransaction(_ request: TransactionRequest) async throws -> String {
        // TODO: Implement actual transaction sending
        // For now, return a mock transaction hash
        // In production, this should:
        // 1. Build the transaction
        // 2. Sign it with the private key
        // 3. Send to the network
        // 4. Return the transaction hash

        return "0x" + String(repeating: "0", count: 64) // Mock tx hash
    }

    // MARK: - Persistence

    private func loadApprovedDApps() {
        if let data = UserDefaults.standard.data(forKey: "connectedDApps"),
           let decoded = try? JSONDecoder().decode([ConnectedDApp].self, from: data) {
            connectedDApps = decoded
        }
    }

    private func saveApprovedDApps() {
        if let encoded = try? JSONEncoder().encode(connectedDApps) {
            UserDefaults.standard.set(encoded, forKey: "connectedDApps")
        }
    }

    private func loadCurrentAccount() {
        // TODO: Load from wallet service
        // For now, use a mock address
        currentAccount = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    }

    private func getFaviconURL(_ domain: String) -> String {
        return "https://\(domain)/favicon.ico"
    }
}

// MARK: - Models
struct DAppBookmark: Identifiable {
    let id: String
    let name: String
    let url: String
    let icon: String
}

struct ConnectedDApp: Identifiable, Codable {
    let id: String
    let domain: String
    let icon: String
    let connectedAt: Date
}

struct ConnectionRequest: Identifiable {
    let id: String
    let domain: String
    let icon: String
    let requestedAt: Date
}

enum SignType {
    case personalSign
    case signTypedData
}

struct SignRequest: Identifiable {
    let id: String
    let domain: String
    let message: String
    let type: SignType
    let requestedAt: Date
}

struct TransactionRequest: Identifiable {
    let id: String
    let domain: String
    let from: String
    let to: String
    let value: String
    let data: String?
    let gas: String?
    let gasPrice: String?
    let requestedAt: Date
}

struct Web3Network: Identifiable {
    let id: String
    let name: String
    let chainId: String
    let networkVersion: String
    let rpcUrl: String
    let symbol: String

    static let mainnet = Web3Network(
        id: "1",
        name: "Ethereum Mainnet",
        chainId: "0x1",
        networkVersion: "1",
        rpcUrl: "https://mainnet.infura.io/v3/",
        symbol: "ETH"
    )

    static let goerli = Web3Network(
        id: "5",
        name: "Goerli Testnet",
        chainId: "0x5",
        networkVersion: "5",
        rpcUrl: "https://goerli.infura.io/v3/",
        symbol: "ETH"
    )

    static let polygon = Web3Network(
        id: "137",
        name: "Polygon Mainnet",
        chainId: "0x89",
        networkVersion: "137",
        rpcUrl: "https://polygon-rpc.com/",
        symbol: "MATIC"
    )

    static let bsc = Web3Network(
        id: "56",
        name: "BSC Mainnet",
        chainId: "0x38",
        networkVersion: "56",
        rpcUrl: "https://bsc-dataseed.binance.org/",
        symbol: "BNB"
    )

    static let arbitrum = Web3Network(
        id: "42161",
        name: "Arbitrum One",
        chainId: "0xa4b1",
        networkVersion: "42161",
        rpcUrl: "https://arb1.arbitrum.io/rpc",
        symbol: "ETH"
    )

    static let optimism = Web3Network(
        id: "10",
        name: "Optimism",
        chainId: "0xa",
        networkVersion: "10",
        rpcUrl: "https://mainnet.optimism.io",
        symbol: "ETH"
    )

    static let allNetworks: [Web3Network] = [
        .mainnet, .goerli, .polygon, .bsc, .arbitrum, .optimism
    ]
}

// MARK: - Web3 Provider JavaScript
struct Web3Provider {
    static let providerScript = """
    (function() {
        let requestId = 0;
        const pendingRequests = new Map();

        class EtridProvider {
            constructor() {
                this.isEtrid = true;
                this.isMetaMask = true; // Compatibility
                this._state = {
                    accounts: [],
                    isConnected: false,
                    chainId: null,
                    networkVersion: null
                };
                this._events = {};
            }

            // Core RPC method
            async request(args) {
                const { method, params = [] } = args;
                const id = ++requestId;

                return new Promise((resolve, reject) => {
                    pendingRequests.set(id, { resolve, reject });

                    // Send to native
                    window.webkit.messageHandlers.etridWallet.postMessage({
                        id: id,
                        method: method,
                        params: params
                    });

                    // Timeout after 60 seconds
                    setTimeout(() => {
                        if (pendingRequests.has(id)) {
                            pendingRequests.delete(id);
                            reject(new Error('Request timeout'));
                        }
                    }, 60000);
                });
            }

            // Legacy methods for compatibility
            async send(methodOrPayload, paramsOrCallback) {
                if (typeof methodOrPayload === 'string') {
                    return this.request({
                        method: methodOrPayload,
                        params: paramsOrCallback
                    });
                }

                if (typeof paramsOrCallback === 'function') {
                    // Legacy callback style
                    try {
                        const result = await this.request(methodOrPayload);
                        paramsOrCallback(null, { result });
                    } catch (error) {
                        paramsOrCallback(error);
                    }
                } else {
                    return this.request(methodOrPayload);
                }
            }

            async sendAsync(payload, callback) {
                try {
                    const result = await this.request(payload);
                    callback(null, { result });
                } catch (error) {
                    callback(error);
                }
            }

            // Account management
            async enable() {
                return this.request({ method: 'eth_requestAccounts' });
            }

            isConnected() {
                return this._state.isConnected;
            }

            // Event handling
            on(event, callback) {
                if (!this._events[event]) {
                    this._events[event] = [];
                }
                this._events[event].push(callback);
                return this;
            }

            removeListener(event, callback) {
                if (this._events[event]) {
                    this._events[event] = this._events[event].filter(cb => cb !== callback);
                }
                return this;
            }

            // Internal methods
            _handleResponse(response) {
                const request = pendingRequests.get(response.id);
                if (!request) return;

                pendingRequests.delete(response.id);

                if (response.error) {
                    request.reject(response.error);
                } else {
                    request.resolve(response.result);
                }
            }

            _updateState(newState) {
                this._state = { ...this._state, ...newState };
            }

            _emit(event, ...args) {
                if (this._events[event]) {
                    this._events[event].forEach(callback => {
                        try {
                            callback(...args);
                        } catch (error) {
                            console.error('Error in event listener:', error);
                        }
                    });
                }
            }
        }

        // Create and expose provider
        window.ethereum = new EtridProvider();

        // Announce provider
        window.dispatchEvent(new Event('ethereum#initialized'));
    })();
    """
}

// MARK: - Request Views

struct ConnectionRequestView: View {
    let request: ConnectionRequest
    let onApprove: () -> Void
    let onReject: () -> Void

    var body: some View {
        NavigationView {
            VStack(spacing: 24) {
                Spacer()

                // Icon
                Image(systemName: "link.circle.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.blue)

                // Title
                Text("Connect to DApp")
                    .font(.title2)
                    .fontWeight(.bold)

                // Domain
                VStack(spacing: 8) {
                    Text(request.domain)
                        .font(.headline)
                        .foregroundColor(.primary)

                    Text("wants to connect to your wallet")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }

                // Permissions
                VStack(alignment: .leading, spacing: 12) {
                    Text("This app will be able to:")
                        .font(.subheadline)
                        .fontWeight(.semibold)

                    PermissionRow(
                        icon: "eye",
                        text: "View your wallet address"
                    )

                    PermissionRow(
                        icon: "signature",
                        text: "Request approval for transactions"
                    )

                    PermissionRow(
                        icon: "network",
                        text: "View your network"
                    )
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                Spacer()

                // Actions
                VStack(spacing: 12) {
                    Button(action: onApprove) {
                        Text("Connect")
                            .font(.headline)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.blue)
                            .cornerRadius(12)
                    }

                    Button(action: onReject) {
                        Text("Cancel")
                            .font(.headline)
                            .foregroundColor(.blue)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                    }
                }
            }
            .padding()
            .navigationTitle("Connection Request")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct SignRequestView: View {
    let request: SignRequest
    let onApprove: () -> Void
    let onReject: () -> Void

    var body: some View {
        NavigationView {
            VStack(spacing: 24) {
                Spacer()

                // Icon
                Image(systemName: "signature")
                    .font(.system(size: 60))
                    .foregroundColor(.orange)

                // Title
                Text("Signature Request")
                    .font(.title2)
                    .fontWeight(.bold)

                // Domain
                VStack(spacing: 8) {
                    Text(request.domain)
                        .font(.headline)
                        .foregroundColor(.primary)

                    Text("wants you to sign a message")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                // Message
                VStack(alignment: .leading, spacing: 8) {
                    Text("Message:")
                        .font(.subheadline)
                        .fontWeight(.semibold)

                    ScrollView {
                        Text(request.message)
                            .font(.system(.body, design: .monospaced))
                            .foregroundColor(.secondary)
                            .frame(maxWidth: .infinity, alignment: .leading)
                    }
                    .frame(maxHeight: 200)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                // Warning
                HStack(spacing: 12) {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)

                    Text("Only sign messages from sites you trust")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .padding()
                .background(Color.orange.opacity(0.1))
                .cornerRadius(8)

                Spacer()

                // Actions
                VStack(spacing: 12) {
                    Button(action: onApprove) {
                        Text("Sign")
                            .font(.headline)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.orange)
                            .cornerRadius(12)
                    }

                    Button(action: onReject) {
                        Text("Cancel")
                            .font(.headline)
                            .foregroundColor(.orange)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                    }
                }
            }
            .padding()
            .navigationTitle("Sign Message")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

struct TransactionRequestView: View {
    let request: TransactionRequest
    let network: Web3Network
    let onApprove: () -> Void
    let onReject: () -> Void

    var body: some View {
        NavigationView {
            VStack(spacing: 24) {
                Spacer()

                // Icon
                Image(systemName: "arrow.up.circle.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.green)

                // Title
                Text("Transaction Request")
                    .font(.title2)
                    .fontWeight(.bold)

                // Domain
                VStack(spacing: 8) {
                    Text(request.domain)
                        .font(.headline)
                        .foregroundColor(.primary)

                    Text("wants to send a transaction")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                // Transaction Details
                VStack(spacing: 12) {
                    DAppTransactionRow(label: "Network", value: network.name)
                    DAppTransactionRow(label: "To", value: formatAddress(request.to))
                    DAppTransactionRow(label: "Amount", value: formatValue(request.value, symbol: network.symbol))

                    if let gas = request.gas {
                        DAppTransactionRow(label: "Gas Limit", value: gas)
                    }

                    if let gasPrice = request.gasPrice {
                        DAppTransactionRow(label: "Gas Price", value: formatGasPrice(gasPrice))
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                // Warning
                HStack(spacing: 12) {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)

                    Text("Verify all transaction details before confirming")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .padding()
                .background(Color.orange.opacity(0.1))
                .cornerRadius(8)

                Spacer()

                // Actions
                VStack(spacing: 12) {
                    Button(action: onApprove) {
                        Text("Confirm")
                            .font(.headline)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.green)
                            .cornerRadius(12)
                    }

                    Button(action: onReject) {
                        Text("Cancel")
                            .font(.headline)
                            .foregroundColor(.green)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                    }
                }
            }
            .padding()
            .navigationTitle("Send Transaction")
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    private func formatAddress(_ address: String) -> String {
        guard address.count > 10 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }

    private func formatValue(_ hexValue: String, symbol: String) -> String {
        // Convert hex to decimal (simplified)
        let cleanHex = hexValue.replacingOccurrences(of: "0x", with: "")
        if let value = Int(cleanHex, radix: 16) {
            let ethValue = Double(value) / 1_000_000_000_000_000_000
            return String(format: "%.6f %@", ethValue, symbol)
        }
        return "\(hexValue) \(symbol)"
    }

    private func formatGasPrice(_ hexPrice: String) -> String {
        let cleanHex = hexPrice.replacingOccurrences(of: "0x", with: "")
        if let value = Int(cleanHex, radix: 16) {
            let gwei = Double(value) / 1_000_000_000
            return String(format: "%.2f Gwei", gwei)
        }
        return hexPrice
    }
}

// MARK: - Helper Views

struct PermissionRow: View {
    let icon: String
    let text: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 24)

            Text(text)
                .font(.subheadline)
                .foregroundColor(.primary)

            Spacer()
        }
    }
}

struct DAppTransactionRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(.primary)
                .lineLimit(1)
        }
    }
}

// MARK: - Preview
struct DAppBrowserView_Previews: PreviewProvider {
    static var previews: some View {
        DAppBrowserView()
    }
}
