//
//  DeepLink.swift
//  EtridWallet
//
//  Deep link models and routing
//

import Foundation

// MARK: - Deep Link Route
enum DeepLinkRoute: Equatable {
    case home
    case send(address: String?, amount: String?, token: String?)
    case receive
    case transaction(hash: String)
    case token(address: String)
    case nft(contractAddress: String, tokenId: String)
    case dapp(url: URL)
    case walletConnect(uri: String)
    case governance(proposalId: String?)
    case settings
    case backup
    case paymentRequest(DeepLinkPaymentRequest)

    var path: String {
        switch self {
        case .home:
            return "/"
        case .send:
            return "/send"
        case .receive:
            return "/receive"
        case .transaction(let hash):
            return "/transaction/\(hash)"
        case .token(let address):
            return "/token/\(address)"
        case .nft(let contract, let tokenId):
            return "/nft/\(contract)/\(tokenId)"
        case .dapp(let url):
            return "/dapp?url=\(url.absoluteString)"
        case .walletConnect(let uri):
            return "/wc?uri=\(uri)"
        case .governance(let proposalId):
            return proposalId != nil ? "/governance/\(proposalId!)" : "/governance"
        case .settings:
            return "/settings"
        case .backup:
            return "/backup"
        case .paymentRequest:
            return "/pay"
        }
    }
}

// MARK: - Payment Request
struct DeepLinkPaymentRequest: Equatable, Codable {
    let address: String
    let amount: String?
    let token: String? // Contract address or symbol
    let chainId: Int?
    let message: String?
    let callbackUrl: String?

    init(address: String,
         amount: String? = nil,
         token: String? = nil,
         chainId: Int? = nil,
         message: String? = nil,
         callbackUrl: String? = nil) {
        self.address = address
        self.amount = amount
        self.token = token
        self.chainId = chainId
        self.message = message
        self.callbackUrl = callbackUrl
    }

    var displayAmount: String {
        guard let amount = amount else { return "Unknown" }
        let tokenSymbol = token ?? "ETH"
        return "\(amount) \(tokenSymbol)"
    }
}

// MARK: - Deep Link Parameters
struct DeepLinkParameters {
    let scheme: String
    let host: String?
    let path: String
    let queryItems: [URLQueryItem]

    init?(url: URL) {
        guard let scheme = url.scheme else { return nil }

        self.scheme = scheme
        self.host = url.host
        self.path = url.path

        if let components = URLComponents(url: url, resolvingAgainstBaseURL: true) {
            self.queryItems = components.queryItems ?? []
        } else {
            self.queryItems = []
        }
    }

    func queryValue(for key: String) -> String? {
        queryItems.first { $0.name == key }?.value
    }

    func allQueryValues() -> [String: String] {
        var dict = [String: String]()
        for item in queryItems {
            dict[item.name] = item.value
        }
        return dict
    }
}

// MARK: - Universal Link Domain
enum UniversalLinkDomain: String {
    case production = "etrid.app"
    case staging = "staging.etrid.app"
    case development = "dev.etrid.app"

    var host: String {
        return rawValue
    }
}

// MARK: - Custom URL Scheme
enum CustomURLScheme: String {
    case etrid = "etrid"
    case ethereum = "ethereum"
    case walletConnect = "wc"

    var scheme: String {
        return rawValue
    }
}

// MARK: - Deep Link Parser
struct DeepLinkParser {
    /// Parses URL into DeepLinkRoute
    static func parse(_ url: URL) -> DeepLinkRoute? {
        guard let params = DeepLinkParameters(url: url) else {
            return nil
        }

        // Handle different schemes
        switch params.scheme {
        case CustomURLScheme.etrid.scheme:
            return parseEtridScheme(params)
        case CustomURLScheme.ethereum.scheme:
            return parseEthereumScheme(params)
        case CustomURLScheme.walletConnect.scheme:
            return parseWalletConnectScheme(params)
        case "https", "http":
            return parseUniversalLink(params)
        default:
            return nil
        }
    }

    // MARK: - Scheme Parsers
    private static func parseEtridScheme(_ params: DeepLinkParameters) -> DeepLinkRoute? {
        let path = params.path.trimmingCharacters(in: CharacterSet(charactersIn: "/"))
        let components = path.split(separator: "/").map(String.init)

        guard let host = params.host else { return .home }

        switch host {
        case "send", "pay":
            return parseSendRoute(params)
        case "receive":
            return .receive
        case "transaction", "tx":
            if let hash = components.first {
                return .transaction(hash: hash)
            }
        case "token":
            if let address = components.first {
                return .token(address: address)
            }
        case "nft":
            if components.count >= 2 {
                return .nft(contractAddress: components[0], tokenId: components[1])
            }
        case "dapp":
            if let urlString = params.queryValue(for: "url"),
               let dappUrl = URL(string: urlString) {
                return .dapp(url: dappUrl)
            }
        case "wc":
            if let uri = params.queryValue(for: "uri") {
                return .walletConnect(uri: uri)
            }
        case "governance", "dao":
            let proposalId = components.first
            return .governance(proposalId: proposalId)
        case "settings":
            return .settings
        case "backup":
            return .backup
        default:
            break
        }

        return .home
    }

    private static func parseEthereumScheme(_ params: DeepLinkParameters) -> DeepLinkRoute? {
        // ethereum:0x1234...?value=1.5&chainId=1
        guard let host = params.host else { return nil }

        let address = host
        let amount = params.queryValue(for: "value")
        let chainId = params.queryValue(for: "chainId").flatMap { Int($0) }
        let message = params.queryValue(for: "message")

        let paymentRequest = DeepLinkPaymentRequest(
            address: address,
            amount: amount,
            chainId: chainId,
            message: message
        )

        return .paymentRequest(paymentRequest)
    }

    private static func parseWalletConnectScheme(_ params: DeepLinkParameters) -> DeepLinkRoute? {
        // wc:a13aef...?bridge=https%3A%2F%2Fbridge.walletconnect.org&key=41791102...
        guard let host = params.host else { return nil }

        // WalletConnect URI format
        let uri = "wc:\(host)@\(params.path.trimmingCharacters(in: CharacterSet(charactersIn: "/")))"
        return .walletConnect(uri: uri)
    }

    private static func parseUniversalLink(_ params: DeepLinkParameters) -> DeepLinkRoute? {
        // https://etrid.app/send/0x1234...
        let path = params.path.trimmingCharacters(in: CharacterSet(charactersIn: "/"))
        let components = path.split(separator: "/").map(String.init)

        guard let firstComponent = components.first else { return .home }

        switch firstComponent {
        case "send", "pay":
            return parseSendRoute(params)
        case "receive":
            return .receive
        case "transaction", "tx":
            if components.count > 1 {
                return .transaction(hash: components[1])
            }
        case "token":
            if components.count > 1 {
                return .token(address: components[1])
            }
        case "nft":
            if components.count >= 3 {
                return .nft(contractAddress: components[1], tokenId: components[2])
            }
        case "wc":
            if let uri = params.queryValue(for: "uri") {
                return .walletConnect(uri: uri)
            }
        case "governance", "dao":
            let proposalId = components.count > 1 ? components[1] : nil
            return .governance(proposalId: proposalId)
        case "settings":
            return .settings
        case "backup":
            return .backup
        default:
            break
        }

        return .home
    }

    private static func parseSendRoute(_ params: DeepLinkParameters) -> DeepLinkRoute {
        let address = params.queryValue(for: "address") ?? params.queryValue(for: "to")
        let amount = params.queryValue(for: "amount") ?? params.queryValue(for: "value")
        let token = params.queryValue(for: "token") ?? params.queryValue(for: "contract")

        if let addr = address {
            let chainId = params.queryValue(for: "chainId").flatMap { Int($0) }
            let message = params.queryValue(for: "message")
            let callbackUrl = params.queryValue(for: "callback")

            let paymentRequest = DeepLinkPaymentRequest(
                address: addr,
                amount: amount,
                token: token,
                chainId: chainId,
                message: message,
                callbackUrl: callbackUrl
            )
            return .paymentRequest(paymentRequest)
        }

        return .send(address: address, amount: amount, token: token)
    }
}

// MARK: - Deep Link Builder
struct DeepLinkBuilder {
    /// Creates a payment request deep link
    static func createPaymentLink(
        address: String,
        amount: String? = nil,
        token: String? = nil,
        chainId: Int? = nil,
        message: String? = nil
    ) -> URL? {
        var components = URLComponents()
        components.scheme = CustomURLScheme.etrid.scheme
        components.host = "pay"

        var queryItems = [URLQueryItem(name: "address", value: address)]
        if let amount = amount {
            queryItems.append(URLQueryItem(name: "amount", value: amount))
        }
        if let token = token {
            queryItems.append(URLQueryItem(name: "token", value: token))
        }
        if let chainId = chainId {
            queryItems.append(URLQueryItem(name: "chainId", value: String(chainId)))
        }
        if let message = message {
            queryItems.append(URLQueryItem(name: "message", value: message))
        }

        components.queryItems = queryItems
        return components.url
    }

    /// Creates a WalletConnect pairing link
    static func createWalletConnectLink(uri: String) -> URL? {
        var components = URLComponents()
        components.scheme = CustomURLScheme.etrid.scheme
        components.host = "wc"
        components.queryItems = [URLQueryItem(name: "uri", value: uri)]
        return components.url
    }

    /// Creates a token detail link
    static func createTokenLink(address: String) -> URL? {
        var components = URLComponents()
        components.scheme = CustomURLScheme.etrid.scheme
        components.host = "token"
        components.path = "/\(address)"
        return components.url
    }

    /// Creates a transaction detail link
    static func createTransactionLink(hash: String) -> URL? {
        var components = URLComponents()
        components.scheme = CustomURLScheme.etrid.scheme
        components.host = "transaction"
        components.path = "/\(hash)"
        return components.url
    }
}
