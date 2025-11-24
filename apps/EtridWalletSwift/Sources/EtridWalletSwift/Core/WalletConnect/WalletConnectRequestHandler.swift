//
//  WalletConnectRequestHandler.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import Foundation
import BigInt

/// Handles incoming WalletConnect requests from dApps
actor WalletConnectRequestHandler {
    // MARK: - Dependencies

    private let keychainManager: KeychainManager
    private let networkManager: NetworkManager
    private let signatureHandler: SignatureHandler
    private weak var walletConnectService: WalletConnectService?
    private let logger = Logger(subsystem: "io.etrid.wallet", category: "WCRequestHandler")

    // MARK: - State

    private var pendingApprovals: [String: CheckedContinuation<Bool, Never>] = [:]

    // MARK: - Initialization

    init(
        keychainManager: KeychainManager,
        networkManager: NetworkManager,
        walletConnectService: WalletConnectService
    ) {
        self.keychainManager = keychainManager
        self.networkManager = networkManager
        self.walletConnectService = walletConnectService
        self.signatureHandler = SignatureHandler(
            keychainManager: keychainManager,
            networkManager: networkManager
        )
    }

    // MARK: - Public Methods

    /// Handle incoming session request
    func handleRequest(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        logger.info("Handling request: \(request.method)")

        // Validate request
        try validateRequest(request, session: session)

        // Route to appropriate handler
        switch request.method {
        case "eth_sendTransaction":
            return try await handleSendTransaction(request, session: session)

        case "eth_signTransaction":
            return try await handleSignTransaction(request, session: session)

        case "personal_sign":
            return try await handlePersonalSign(request, session: session)

        case "eth_sign":
            return try await handleEthSign(request, session: session)

        case "eth_signTypedData", "eth_signTypedData_v1":
            return try await handleSignTypedDataV1(request, session: session)

        case "eth_signTypedData_v3":
            return try await handleSignTypedDataV3(request, session: session)

        case "eth_signTypedData_v4":
            return try await handleSignTypedDataV4(request, session: session)

        case "wallet_switchEthereumChain":
            return try await handleSwitchChain(request, session: session)

        case "wallet_addEthereumChain":
            return try await handleAddChain(request, session: session)

        case "wallet_watchAsset":
            return try await handleWatchAsset(request, session: session)

        case "eth_accounts":
            return try await handleGetAccounts(request, session: session)

        case "eth_chainId":
            return try await handleGetChainId(request, session: session)

        default:
            throw WalletConnectError.unsupportedMethod(request.method)
        }
    }

    // MARK: - Request Handlers

    private func handleSendTransaction(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        guard let params = request.params as? [[String: Any]],
              let txParams = params.first else {
            throw WalletConnectError.invalidParams
        }

        // Parse transaction parameters
        guard let from = txParams["from"] as? String,
              let to = txParams["to"] as? String else {
            throw WalletConnectError.invalidParams
        }

        // Validate addresses
        guard from.isValidEthereumAddress, to.isValidEthereumAddress else {
            throw WalletConnectError.invalidAddress
        }

        // Validate sender is in session
        guard session.accounts.contains(from.lowercased()) ||
              session.accounts.contains(from) else {
            throw WalletConnectError.addressNotOwned
        }

        // Parse optional parameters
        let value = txParams["value"] as? String ?? "0x0"
        let data = txParams["data"] as? String ?? "0x"
        let gas = txParams["gas"] as? String
        let gasPrice = txParams["gasPrice"] as? String
        let nonce = txParams["nonce"] as? String

        // Build transaction
        let transaction = EthereumTransaction(
            from: from,
            to: to,
            value: BigUInt(hex: value) ?? 0,
            data: Data(hex: data),
            gas: gas.flatMap { BigUInt(hex: $0) },
            gasPrice: gasPrice.flatMap { BigUInt(hex: $0) },
            nonce: nonce.flatMap { BigUInt(hex: $0) },
            chainId: try parseChainId(from: request)
        )

        // Request user approval
        let approved = try await requestUserApproval(
            transaction: transaction,
            dApp: session.dAppMetadata,
            requestId: request.id
        )

        guard approved else {
            throw WalletConnectError.userRejected
        }

        // Get network
        let network = try await networkManager.getNetwork(chainId: transaction.chainId)

        // Estimate gas if not provided
        var finalTransaction = transaction
        if finalTransaction.gas == nil {
            let estimatedGas = try await networkManager.estimateGas(
                transaction: transaction,
                network: network
            )
            finalTransaction.gas = estimatedGas
        }

        // Get current gas price if not provided
        if finalTransaction.gasPrice == nil {
            let currentGasPrice = try await networkManager.getGasPrice(network: network)
            finalTransaction.gasPrice = currentGasPrice
        }

        // Get nonce if not provided
        if finalTransaction.nonce == nil {
            let currentNonce = try await networkManager.getTransactionCount(
                address: from,
                network: network
            )
            finalTransaction.nonce = currentNonce
        }

        // Sign and send transaction
        let txHash = try await signAndSendTransaction(
            finalTransaction,
            from: from,
            network: network
        )

        return JSONRPCResponse(id: request.id, result: txHash)
    }

    private func handleSignTransaction(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        guard let params = request.params as? [[String: Any]],
              let txParams = params.first else {
            throw WalletConnectError.invalidParams
        }

        guard let from = txParams["from"] as? String,
              let to = txParams["to"] as? String else {
            throw WalletConnectError.invalidParams
        }

        guard from.isValidEthereumAddress, to.isValidEthereumAddress else {
            throw WalletConnectError.invalidAddress
        }

        guard session.accounts.contains(from.lowercased()) ||
              session.accounts.contains(from) else {
            throw WalletConnectError.addressNotOwned
        }

        let value = txParams["value"] as? String ?? "0x0"
        let data = txParams["data"] as? String ?? "0x"
        let gas = txParams["gas"] as? String
        let gasPrice = txParams["gasPrice"] as? String
        let nonce = txParams["nonce"] as? String

        let transaction = EthereumTransaction(
            from: from,
            to: to,
            value: BigUInt(hex: value) ?? 0,
            data: Data(hex: data),
            gas: gas.flatMap { BigUInt(hex: $0) },
            gasPrice: gasPrice.flatMap { BigUInt(hex: $0) },
            nonce: nonce.flatMap { BigUInt(hex: $0) },
            chainId: try parseChainId(from: request)
        )

        let approved = try await requestUserApproval(
            transaction: transaction,
            dApp: session.dAppMetadata,
            requestId: request.id
        )

        guard approved else {
            throw WalletConnectError.userRejected
        }

        let signedTx = try await signTransaction(transaction, from: from)

        return JSONRPCResponse(id: request.id, result: signedTx)
    }

    private func handlePersonalSign(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        // personal_sign has params in format [message, address] or [address, message]
        guard let params = request.params as? [String],
              params.count >= 2 else {
            throw WalletConnectError.invalidParams
        }

        // Try to determine which param is the address
        let (message, address) = try parsePersonalSignParams(params)

        // Validate address
        guard address.isValidEthereumAddress else {
            throw WalletConnectError.invalidAddress
        }

        guard session.accounts.contains(address.lowercased()) ||
              session.accounts.contains(address) else {
            throw WalletConnectError.addressNotOwned
        }

        // Decode message
        let decodedMessage = try decodeMessage(message)

        // Check for dangerous patterns
        let securityWarnings = signatureHandler.checkSecurityWarnings(message: decodedMessage)

        // Request user approval
        let signRequest = SignRequest(
            id: request.id,
            type: .personalSign,
            message: decodedMessage,
            rawMessage: message,
            address: address,
            chainId: try parseChainId(from: request),
            dAppMetadata: session.dAppMetadata,
            securityWarnings: securityWarnings
        )

        let approved = try await requestSignatureApproval(signRequest)

        guard approved else {
            throw WalletConnectError.userRejected
        }

        // Sign message
        let signature = try await signatureHandler.signPersonalMessage(
            message: message,
            address: address
        )

        return JSONRPCResponse(id: request.id, result: signature)
    }

    private func handleEthSign(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        // eth_sign is dangerous and should show extra warnings
        guard let params = request.params as? [String],
              params.count >= 2 else {
            throw WalletConnectError.invalidParams
        }

        let address = params[0]
        let message = params[1]

        guard address.isValidEthereumAddress else {
            throw WalletConnectError.invalidAddress
        }

        guard session.accounts.contains(address.lowercased()) ||
              session.accounts.contains(address) else {
            throw WalletConnectError.addressNotOwned
        }

        let decodedMessage = try decodeMessage(message)

        let signRequest = SignRequest(
            id: request.id,
            type: .ethSign,
            message: decodedMessage,
            rawMessage: message,
            address: address,
            chainId: try parseChainId(from: request),
            dAppMetadata: session.dAppMetadata,
            securityWarnings: [
                "eth_sign is a dangerous method that can sign arbitrary data",
                "Only sign if you completely trust this dApp"
            ]
        )

        let approved = try await requestSignatureApproval(signRequest)

        guard approved else {
            throw WalletConnectError.userRejected
        }

        let signature = try await signatureHandler.signEthMessage(
            message: message,
            address: address
        )

        return JSONRPCResponse(id: request.id, result: signature)
    }

    private func handleSignTypedDataV4(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        guard let params = request.params as? [String],
              params.count >= 2 else {
            throw WalletConnectError.invalidParams
        }

        let address = params[0]
        let typedDataString = params[1]

        guard address.isValidEthereumAddress else {
            throw WalletConnectError.invalidAddress
        }

        guard session.accounts.contains(address.lowercased()) ||
              session.accounts.contains(address) else {
            throw WalletConnectError.addressNotOwned
        }

        // Parse typed data
        let typedData = try TypedData.parse(json: typedDataString)

        // Check for dangerous patterns
        let securityWarnings = signatureHandler.checkTypedDataWarnings(typedData: typedData)

        // Format for display
        let formattedMessage = try signatureHandler.formatTypedData(typedData)

        let signRequest = SignRequest(
            id: request.id,
            type: .typedDataV4,
            message: formattedMessage,
            rawMessage: typedDataString,
            address: address,
            chainId: try parseChainId(from: request),
            dAppMetadata: session.dAppMetadata,
            securityWarnings: securityWarnings,
            typedData: typedData
        )

        let approved = try await requestSignatureApproval(signRequest)

        guard approved else {
            throw WalletConnectError.userRejected
        }

        let signature = try await signatureHandler.signTypedData(
            typedData: typedData,
            address: address
        )

        return JSONRPCResponse(id: request.id, result: signature)
    }

    private func handleSignTypedDataV3(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        // V3 is similar to V4
        return try await handleSignTypedDataV4(request, session: session)
    }

    private func handleSignTypedDataV1(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        // V1 has a different format, simpler than V4
        guard let params = request.params as? [String],
              params.count >= 2 else {
            throw WalletConnectError.invalidParams
        }

        let address = params[0]
        let typedDataString = params[1]

        guard address.isValidEthereumAddress else {
            throw WalletConnectError.invalidAddress
        }

        guard session.accounts.contains(address.lowercased()) ||
              session.accounts.contains(address) else {
            throw WalletConnectError.addressNotOwned
        }

        let typedData = try TypedData.parseV1(json: typedDataString)
        let formattedMessage = try signatureHandler.formatTypedData(typedData)

        let signRequest = SignRequest(
            id: request.id,
            type: .typedDataV1,
            message: formattedMessage,
            rawMessage: typedDataString,
            address: address,
            chainId: try parseChainId(from: request),
            dAppMetadata: session.dAppMetadata,
            securityWarnings: [],
            typedData: typedData
        )

        let approved = try await requestSignatureApproval(signRequest)

        guard approved else {
            throw WalletConnectError.userRejected
        }

        let signature = try await signatureHandler.signTypedDataV1(
            typedData: typedData,
            address: address
        )

        return JSONRPCResponse(id: request.id, result: signature)
    }

    private func handleSwitchChain(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        guard let params = request.params as? [[String: Any]],
              let chainParam = params.first,
              let chainIdHex = chainParam["chainId"] as? String else {
            throw WalletConnectError.invalidParams
        }

        guard let chainId = BigUInt(hex: chainIdHex) else {
            throw WalletConnectError.invalidParams
        }

        // Check if chain is supported
        let isSupported = try await networkManager.isChainSupported(chainId: Int(chainId))

        guard isSupported else {
            throw WalletConnectError.unsupportedChain(Int(chainId))
        }

        // Switch chain in network manager
        try await networkManager.switchChain(chainId: Int(chainId))

        return JSONRPCResponse(id: request.id, result: nil)
    }

    private func handleAddChain(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        guard let params = request.params as? [[String: Any]],
              let chainParam = params.first else {
            throw WalletConnectError.invalidParams
        }

        guard let chainIdHex = chainParam["chainId"] as? String,
              let chainName = chainParam["chainName"] as? String,
              let rpcUrls = chainParam["rpcUrls"] as? [String],
              let nativeCurrency = chainParam["nativeCurrency"] as? [String: Any] else {
            throw WalletConnectError.invalidParams
        }

        guard let chainId = BigUInt(hex: chainIdHex) else {
            throw WalletConnectError.invalidParams
        }

        // Parse native currency
        guard let currencyName = nativeCurrency["name"] as? String,
              let currencySymbol = nativeCurrency["symbol"] as? String,
              let currencyDecimals = nativeCurrency["decimals"] as? Int else {
            throw WalletConnectError.invalidParams
        }

        // Request user approval to add chain
        let approved = try await requestAddChainApproval(
            chainId: Int(chainId),
            chainName: chainName,
            rpcUrls: rpcUrls,
            currencyName: currencyName,
            currencySymbol: currencySymbol,
            dApp: session.dAppMetadata
        )

        guard approved else {
            throw WalletConnectError.userRejected
        }

        // Add chain to network manager
        try await networkManager.addCustomChain(
            chainId: Int(chainId),
            name: chainName,
            rpcUrls: rpcUrls,
            nativeCurrency: (currencyName, currencySymbol, currencyDecimals)
        )

        return JSONRPCResponse(id: request.id, result: nil)
    }

    private func handleWatchAsset(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        guard let params = request.params as? [String: Any],
              let type = params["type"] as? String,
              let options = params["options"] as? [String: Any],
              let address = options["address"] as? String else {
            throw WalletConnectError.invalidParams
        }

        guard type.lowercased() == "erc20" else {
            throw WalletConnectError.unsupportedAssetType
        }

        let symbol = options["symbol"] as? String ?? "Unknown"
        let decimals = options["decimals"] as? Int ?? 18

        // Request user approval
        let approved = try await requestWatchAssetApproval(
            address: address,
            symbol: symbol,
            decimals: decimals,
            dApp: session.dAppMetadata
        )

        return JSONRPCResponse(id: request.id, result: approved)
    }

    private func handleGetAccounts(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        return JSONRPCResponse(id: request.id, result: session.accounts)
    }

    private func handleGetChainId(
        _ request: SessionRequest,
        session: WalletConnectSession
    ) async throws -> JSONRPCResponse {
        let currentChainId = await networkManager.getCurrentChainId()
        let chainIdHex = String(format: "0x%x", currentChainId)
        return JSONRPCResponse(id: request.id, result: chainIdHex)
    }

    // MARK: - Helper Methods

    private func validateRequest(_ request: SessionRequest, session: WalletConnectSession) throws {
        // Validate session is active
        guard !session.isExpired else {
            throw WalletConnectError.sessionExpired
        }

        // Validate method is supported
        guard session.supportedMethods.contains(request.method) else {
            throw WalletConnectError.unsupportedMethod(request.method)
        }
    }

    private func parseChainId(from request: SessionRequest) throws -> Int {
        // Try to parse from request chainId
        if let chainIdString = request.chainId?.split(separator: ":").last,
           let chainId = Int(chainIdString) {
            return chainId
        }

        // Fallback to current network
        return await networkManager.getCurrentChainId()
    }

    private func parsePersonalSignParams(_ params: [String]) throws -> (message: String, address: String) {
        // Standard is [message, address], but some use [address, message]
        if params[0].isValidEthereumAddress {
            return (params[1], params[0])
        } else if params[1].isValidEthereumAddress {
            return (params[0], params[1])
        } else {
            throw WalletConnectError.invalidParams
        }
    }

    private func decodeMessage(_ hexMessage: String) throws -> String {
        let hex = hexMessage.hasPrefix("0x") ? String(hexMessage.dropFirst(2)) : hexMessage

        if let data = Data(hexString: hex),
           let string = String(data: data, encoding: .utf8) {
            return string
        }

        // If not valid UTF-8, return hex
        return hexMessage
    }

    private func signAndSendTransaction(
        _ transaction: EthereumTransaction,
        from address: String,
        network: Network
    ) async throws -> String {
        // Get private key
        let privateKey = try await keychainManager.getPrivateKey(for: address)

        // Sign transaction
        let signedTx = try await signatureHandler.signTransaction(
            transaction: transaction,
            privateKey: privateKey
        )

        // Send to network
        let txHash = try await networkManager.sendRawTransaction(
            signedTransaction: signedTx,
            network: network
        )

        return txHash
    }

    private func signTransaction(
        _ transaction: EthereumTransaction,
        from address: String
    ) async throws -> String {
        let privateKey = try await keychainManager.getPrivateKey(for: address)

        let signedTx = try await signatureHandler.signTransaction(
            transaction: transaction,
            privateKey: privateKey
        )

        return signedTx
    }

    // MARK: - User Approval Methods

    private func requestUserApproval(
        transaction: EthereumTransaction,
        dApp: DAppMetadata,
        requestId: Int64
    ) async throws -> Bool {
        return await withCheckedContinuation { continuation in
            Task { @MainActor in
                // Post notification to show UI
                NotificationCenter.default.post(
                    name: .walletConnectTransactionRequest,
                    object: TransactionApprovalRequest(
                        id: requestId,
                        transaction: transaction,
                        dApp: dApp,
                        continuation: continuation
                    )
                )
            }
        }
    }

    private func requestSignatureApproval(_ signRequest: SignRequest) async throws -> Bool {
        return await withCheckedContinuation { continuation in
            Task { @MainActor in
                NotificationCenter.default.post(
                    name: .walletConnectSignRequest,
                    object: SignatureApprovalRequest(
                        signRequest: signRequest,
                        continuation: continuation
                    )
                )
            }
        }
    }

    private func requestAddChainApproval(
        chainId: Int,
        chainName: String,
        rpcUrls: [String],
        currencyName: String,
        currencySymbol: String,
        dApp: DAppMetadata
    ) async throws -> Bool {
        return await withCheckedContinuation { continuation in
            Task { @MainActor in
                NotificationCenter.default.post(
                    name: .walletConnectAddChainRequest,
                    object: AddChainRequest(
                        chainId: chainId,
                        chainName: chainName,
                        rpcUrls: rpcUrls,
                        currencyName: currencyName,
                        currencySymbol: currencySymbol,
                        dApp: dApp,
                        continuation: continuation
                    )
                )
            }
        }
    }

    private func requestWatchAssetApproval(
        address: String,
        symbol: String,
        decimals: Int,
        dApp: DAppMetadata
    ) async throws -> Bool {
        return await withCheckedContinuation { continuation in
            Task { @MainActor in
                NotificationCenter.default.post(
                    name: .walletConnectWatchAssetRequest,
                    object: WatchAssetRequest(
                        address: address,
                        symbol: symbol,
                        decimals: decimals,
                        dApp: dApp,
                        continuation: continuation
                    )
                )
            }
        }
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let walletConnectTransactionRequest = Notification.Name("walletConnectTransactionRequest")
    static let walletConnectSignRequest = Notification.Name("walletConnectSignRequest")
    static let walletConnectAddChainRequest = Notification.Name("walletConnectAddChainRequest")
    static let walletConnectWatchAssetRequest = Notification.Name("walletConnectWatchAssetRequest")
}

// MARK: - Supporting Types

struct TransactionApprovalRequest {
    let id: Int64
    let transaction: EthereumTransaction
    let dApp: DAppMetadata
    let continuation: CheckedContinuation<Bool, Never>
}

struct SignatureApprovalRequest {
    let signRequest: SignRequest
    let continuation: CheckedContinuation<Bool, Never>
}

struct AddChainRequest {
    let chainId: Int
    let chainName: String
    let rpcUrls: [String]
    let currencyName: String
    let currencySymbol: String
    let dApp: DAppMetadata
    let continuation: CheckedContinuation<Bool, Never>
}

struct WatchAssetRequest {
    let address: String
    let symbol: String
    let decimals: Int
    let dApp: DAppMetadata
    let continuation: CheckedContinuation<Bool, Never>
}
