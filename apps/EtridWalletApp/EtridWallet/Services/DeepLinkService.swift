//
//  DeepLinkService.swift
//  EtridWallet
//
//  Deep link handling and navigation coordination
//

import Foundation
import SwiftUI
import Combine

@MainActor
class DeepLinkService: ObservableObject {
    // MARK: - Published Properties
    @Published var currentRoute: DeepLinkRoute?
    @Published var pendingRoute: DeepLinkRoute?
    @Published var isProcessingLink = false

    // MARK: - Private Properties
    private var cancellables = Set<AnyCancellable>()
    private var navigationHandler: ((DeepLinkRoute) -> Void)?

    // MARK: - Singleton
    static let shared = DeepLinkService()

    private init() {
        setupObservers()
    }

    // MARK: - Setup
    private func setupObservers() {
        // Observe route changes
        $currentRoute
            .compactMap { $0 }
            .sink { [weak self] route in
                self?.handleRoute(route)
            }
            .store(in: &cancellables)
    }

    // MARK: - Public Methods
    /// Handles incoming URL
    func handleURL(_ url: URL) -> Bool {
        guard let route = DeepLinkParser.parse(url) else {
            print("Failed to parse deep link: \(url)")
            return false
        }

        print("Deep link parsed: \(route)")

        // If navigation handler is not set yet, store as pending
        if navigationHandler == nil {
            pendingRoute = route
        } else {
            currentRoute = route
        }

        return true
    }

    /// Sets navigation handler
    func setNavigationHandler(_ handler: @escaping (DeepLinkRoute) -> Void) {
        navigationHandler = handler

        // Process pending route if exists
        if let pending = pendingRoute {
            pendingRoute = nil
            currentRoute = pending
        }
    }

    /// Clears current route
    func clearRoute() {
        currentRoute = nil
    }

    // MARK: - Route Handling
    private func handleRoute(_ route: DeepLinkRoute) {
        isProcessingLink = true

        // Add slight delay for UI transitions
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) { [weak self] in
            self?.navigationHandler?(route)
            self?.isProcessingLink = false
        }
    }

    // MARK: - Universal Links
    /// Handles universal link
    func handleUniversalLink(_ userActivity: NSUserActivity) -> Bool {
        guard userActivity.activityType == NSUserActivityTypeBrowsingWeb,
              let url = userActivity.webpageURL else {
            return false
        }

        return handleURL(url)
    }

    // MARK: - URL Validation
    /// Validates if URL is a supported deep link
    func isValidDeepLink(_ url: URL) -> Bool {
        guard let scheme = url.scheme else { return false }

        let supportedSchemes = [
            CustomURLScheme.etrid.scheme,
            CustomURLScheme.ethereum.scheme,
            CustomURLScheme.walletConnect.scheme,
            "https",
            "http"
        ]

        guard supportedSchemes.contains(scheme) else {
            return false
        }

        // For http(s) schemes, check if it's a supported domain
        if scheme == "https" || scheme == "http" {
            guard let host = url.host else { return false }
            let supportedDomains = [
                UniversalLinkDomain.production.host,
                UniversalLinkDomain.staging.host,
                UniversalLinkDomain.development.host
            ]
            return supportedDomains.contains(host)
        }

        return true
    }

    // MARK: - Link Generation
    /// Generates shareable payment link
    func generatePaymentLink(
        address: String,
        amount: String? = nil,
        token: String? = nil,
        chainId: Int? = nil,
        message: String? = nil
    ) -> URL? {
        return DeepLinkBuilder.createPaymentLink(
            address: address,
            amount: amount,
            token: token,
            chainId: chainId,
            message: message
        )
    }

    /// Generates WalletConnect pairing link
    func generateWalletConnectLink(uri: String) -> URL? {
        return DeepLinkBuilder.createWalletConnectLink(uri: uri)
    }

    /// Generates token detail link
    func generateTokenLink(address: String) -> URL? {
        return DeepLinkBuilder.createTokenLink(address: address)
    }

    /// Generates transaction detail link
    func generateTransactionLink(hash: String) -> URL? {
        return DeepLinkBuilder.createTransactionLink(hash: hash)
    }

    // MARK: - QR Code Integration
    /// Generates QR code data from payment request
    func generatePaymentQRCode(
        address: String,
        amount: String? = nil,
        token: String? = nil
    ) -> Data? {
        guard let url = generatePaymentLink(address: address, amount: amount, token: token) else {
            return nil
        }

        return generateQRCode(from: url.absoluteString)
    }

    private func generateQRCode(from string: String) -> Data? {
        let data = string.data(using: .utf8)

        guard let filter = CIFilter(name: "CIQRCodeGenerator") else {
            return nil
        }

        filter.setValue(data, forKey: "inputMessage")
        filter.setValue("H", forKey: "inputCorrectionLevel")

        guard let outputImage = filter.outputImage else {
            return nil
        }

        let transform = CGAffineTransform(scaleX: 10, y: 10)
        let scaledImage = outputImage.transformed(by: transform)

        let context = CIContext()
        guard let cgImage = context.createCGImage(scaledImage, from: scaledImage.extent) else {
            return nil
        }

        #if os(iOS)
        let uiImage = UIImage(cgImage: cgImage)
        return uiImage.pngData()
        #else
        return nil
        #endif
    }

    // MARK: - Share Sheet
    /// Creates share items for payment request
    func createShareItems(for paymentRequest: DeepLinkPaymentRequest) -> [Any] {
        var items: [Any] = []

        // Add URL
        if let url = DeepLinkBuilder.createPaymentLink(
            address: paymentRequest.address,
            amount: paymentRequest.amount,
            token: paymentRequest.token,
            chainId: paymentRequest.chainId,
            message: paymentRequest.message
        ) {
            items.append(url)
        }

        // Add message
        var message = "Send \(paymentRequest.displayAmount) to \(paymentRequest.address)"
        if let msg = paymentRequest.message {
            message += "\n\(msg)"
        }
        items.append(message)

        return items
    }
}

// MARK: - Navigation Coordinator
@MainActor
class DeepLinkNavigationCoordinator: ObservableObject {
    @Published var activeSheet: SheetDestination?
    @Published var navigationPath = [DeepLinkRoute]()

    private let deepLinkService = DeepLinkService.shared

    init() {
        setupDeepLinkHandler()
    }

    private func setupDeepLinkHandler() {
        deepLinkService.setNavigationHandler { [weak self] route in
            self?.navigate(to: route)
        }
    }

    func navigate(to route: DeepLinkRoute) {
        switch route {
        case .home:
            navigationPath.removeAll()
            activeSheet = nil

        case .send(let address, let amount, let token):
            activeSheet = .send(address: address, amount: amount, token: token)

        case .receive:
            activeSheet = .receive

        case .transaction(let hash):
            activeSheet = .transaction(hash: hash)

        case .token(let address):
            navigationPath.append(route)

        case .nft(let contract, let tokenId):
            navigationPath.append(route)

        case .dapp(let url):
            activeSheet = .dapp(url: url)

        case .walletConnect(let uri):
            activeSheet = .walletConnect(uri: uri)

        case .governance(let proposalId):
            if let proposalId = proposalId {
                navigationPath.append(route)
            } else {
                activeSheet = .governance
            }

        case .settings:
            activeSheet = .settings

        case .backup:
            activeSheet = .backup

        case .paymentRequest(let request):
            activeSheet = .paymentRequest(request)
        }
    }

    func dismissSheet() {
        activeSheet = nil
    }

    func popToRoot() {
        navigationPath.removeAll()
    }
}

// MARK: - Sheet Destinations
enum SheetDestination: Identifiable {
    case send(address: String?, amount: String?, token: String?)
    case receive
    case transaction(hash: String)
    case dapp(url: URL)
    case walletConnect(uri: String)
    case governance
    case settings
    case backup
    case paymentRequest(DeepLinkPaymentRequest)

    var id: String {
        switch self {
        case .send:
            return "send"
        case .receive:
            return "receive"
        case .transaction(let hash):
            return "transaction_\(hash)"
        case .dapp:
            return "dapp"
        case .walletConnect:
            return "walletConnect"
        case .governance:
            return "governance"
        case .settings:
            return "settings"
        case .backup:
            return "backup"
        case .paymentRequest:
            return "paymentRequest"
        }
    }
}
