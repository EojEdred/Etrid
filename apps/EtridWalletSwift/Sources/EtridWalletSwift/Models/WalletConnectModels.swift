//
//  WalletConnectModels.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import Foundation
import BigInt

// MARK: - WalletConnect Session

struct WalletConnectSession: Codable, Identifiable, Equatable {
    let id: String
    let topic: String
    let dAppMetadata: DAppMetadata
    let accounts: [String]
    let chains: [String]
    let createdAt: Date
    let expiresAt: Date

    var isExpired: Bool {
        return Date() > expiresAt
    }

    var supportedMethods: [String] {
        return [
            "eth_sendTransaction",
            "eth_signTransaction",
            "eth_sign",
            "personal_sign",
            "eth_signTypedData",
            "eth_signTypedData_v1",
            "eth_signTypedData_v3",
            "eth_signTypedData_v4",
            "wallet_switchEthereumChain",
            "wallet_addEthereumChain",
            "wallet_watchAsset",
            "eth_accounts",
            "eth_chainId"
        ]
    }

    static func == (lhs: WalletConnectSession, rhs: WalletConnectSession) -> Bool {
        return lhs.id == rhs.id
    }
}

// MARK: - DApp Metadata

struct DAppMetadata: Codable, Equatable {
    let name: String
    let description: String
    let url: String
    let icons: [String]

    var iconURL: URL? {
        guard let iconString = icons.first else { return nil }
        return URL(string: iconString)
    }

    var displayURL: String {
        // Extract domain from URL
        if let url = URL(string: url) {
            return url.host ?? url.absoluteString
        }
        return url
    }
}

// MARK: - Session Proposal

struct SessionProposal: Identifiable {
    let id: String
    let proposer: DAppMetadata
    let requiredNamespaces: [String: ProposalNamespace]
    let optionalNamespaces: [String: ProposalNamespace]
    let createdAt: Date

    var requestedChains: [String] {
        return requiredNamespaces.flatMap { $0.value.chains }
    }

    var requestedMethods: [String] {
        return requiredNamespaces.flatMap { $0.value.methods }
    }

    var requestedEvents: [String] {
        return requiredNamespaces.flatMap { $0.value.events }
    }

    var permissions: [String] {
        var perms: [String] = []

        if requestedMethods.contains(where: { $0.contains("sendTransaction") }) {
            perms.append("Send transactions")
        }

        if requestedMethods.contains(where: { $0.contains("sign") }) {
            perms.append("Sign messages")
        }

        if requestedMethods.contains(where: { $0.contains("signTypedData") }) {
            perms.append("Sign structured data")
        }

        perms.append("View account addresses")
        perms.append("View account balances")

        return perms
    }
}

struct ProposalNamespace {
    let chains: [String]
    let methods: [String]
    let events: [String]
}

struct SessionNamespace {
    let accounts: Set<String>
    let methods: Set<String>
    let events: Set<String>
    let chains: [String]?
}

// MARK: - Session Request

struct SessionRequest: Identifiable {
    let id: Int64
    let topic: String
    let method: String
    let params: Any
    let chainId: String?

    var displayMethod: String {
        switch method {
        case "eth_sendTransaction":
            return "Send Transaction"
        case "eth_signTransaction":
            return "Sign Transaction"
        case "personal_sign":
            return "Sign Message"
        case "eth_sign":
            return "Sign Data"
        case "eth_signTypedData", "eth_signTypedData_v1", "eth_signTypedData_v3", "eth_signTypedData_v4":
            return "Sign Typed Data"
        case "wallet_switchEthereumChain":
            return "Switch Network"
        case "wallet_addEthereumChain":
            return "Add Network"
        case "wallet_watchAsset":
            return "Add Token"
        default:
            return method
        }
    }
}

// MARK: - Sign Request

struct SignRequest: Identifiable {
    let id: Int64
    let type: SignType
    let message: String
    let rawMessage: String
    let address: String
    let chainId: Int
    let dAppMetadata: DAppMetadata
    let securityWarnings: [String]
    let typedData: TypedData?

    init(
        id: Int64,
        type: SignType,
        message: String,
        rawMessage: String,
        address: String,
        chainId: Int,
        dAppMetadata: DAppMetadata,
        securityWarnings: [String],
        typedData: TypedData? = nil
    ) {
        self.id = id
        self.type = type
        self.message = message
        self.rawMessage = rawMessage
        self.address = address
        self.chainId = chainId
        self.dAppMetadata = dAppMetadata
        self.securityWarnings = securityWarnings
        self.typedData = typedData
    }

    var isDangerous: Bool {
        return !securityWarnings.isEmpty || type == .ethSign
    }

    var displayType: String {
        switch type {
        case .personalSign:
            return "Personal Message"
        case .ethSign:
            return "Raw Data (Dangerous)"
        case .typedDataV1:
            return "Typed Data v1"
        case .typedDataV3:
            return "Typed Data v3"
        case .typedDataV4:
            return "Typed Data v4"
        }
    }
}

enum SignType {
    case personalSign
    case ethSign
    case typedDataV1
    case typedDataV3
    case typedDataV4
}

// MARK: - Transaction Request

struct TransactionRequest: Identifiable {
    let id: Int64
    let transaction: EthereumTransaction
    let dAppMetadata: DAppMetadata
    let estimatedGas: BigUInt?
    let gasPrice: BigUInt?

    var totalCost: BigUInt? {
        guard let gas = estimatedGas, let price = gasPrice else {
            return nil
        }
        return gas * price + transaction.value
    }
}

// MARK: - Ethereum Transaction

struct EthereumTransaction {
    let from: String
    let to: String
    let value: BigUInt
    let data: Data
    var gas: BigUInt?
    var gasPrice: BigUInt?
    var nonce: BigUInt?
    let chainId: Int?

    var isContractInteraction: Bool {
        return data.count > 0 && data != Data([0x00])
    }

    var valueInEther: Decimal {
        return Decimal(string: value.description) ?? 0 / pow(10, 18)
    }

    var displayValue: String {
        let etherValue = valueInEther
        return String(format: "%.6f ETH", NSDecimalNumber(decimal: etherValue).doubleValue)
    }
}

// MARK: - TypedData (EIP-712)

struct TypedData {
    let types: [String: [TypedDataField]]
    let primaryType: String
    let domain: [String: Any]
    let message: [String: Any]

    static func parse(json: String) throws -> TypedData {
        guard let data = json.data(using: .utf8) else {
            throw TypedDataError.invalidJSON
        }

        guard let jsonObject = try JSONSerialization.jsonObject(with: data) as? [String: Any] else {
            throw TypedDataError.invalidFormat
        }

        guard let typesJSON = jsonObject["types"] as? [String: Any] else {
            throw TypedDataError.missingTypes
        }

        var types: [String: [TypedDataField]] = [:]
        for (typeName, typeFields) in typesJSON {
            guard let fieldsArray = typeFields as? [[String: Any]] else {
                throw TypedDataError.invalidTypeDefinition
            }

            types[typeName] = try fieldsArray.map { fieldDict in
                guard let name = fieldDict["name"] as? String,
                      let type = fieldDict["type"] as? String else {
                    throw TypedDataError.invalidFieldDefinition
                }
                return TypedDataField(name: name, type: type)
            }
        }

        guard let primaryType = jsonObject["primaryType"] as? String else {
            throw TypedDataError.missingPrimaryType
        }

        guard let domain = jsonObject["domain"] as? [String: Any] else {
            throw TypedDataError.missingDomain
        }

        guard let message = jsonObject["message"] as? [String: Any] else {
            throw TypedDataError.missingMessage
        }

        return TypedData(
            types: types,
            primaryType: primaryType,
            domain: domain,
            message: message
        )
    }

    static func parseV1(json: String) throws -> TypedData {
        guard let data = json.data(using: .utf8) else {
            throw TypedDataError.invalidJSON
        }

        guard let jsonArray = try JSONSerialization.jsonObject(with: data) as? [[String: Any]] else {
            throw TypedDataError.invalidFormat
        }

        // V1 format is simpler - array of {type, name, value}
        var types: [String: [TypedDataField]] = [:]
        var message: [String: Any] = [:]

        var fields: [TypedDataField] = []
        for item in jsonArray {
            guard let name = item["name"] as? String,
                  let type = item["type"] as? String,
                  let value = item["value"] else {
                throw TypedDataError.invalidFieldDefinition
            }

            fields.append(TypedDataField(name: name, type: type))
            message[name] = value
        }

        types["Message"] = fields

        return TypedData(
            types: types,
            primaryType: "Message",
            domain: [:],
            message: message
        )
    }
}

struct TypedDataField {
    let name: String
    let type: String
}

enum TypedDataError: LocalizedError {
    case invalidJSON
    case invalidFormat
    case missingTypes
    case missingPrimaryType
    case missingDomain
    case missingMessage
    case invalidTypeDefinition
    case invalidFieldDefinition

    var errorDescription: String? {
        switch self {
        case .invalidJSON:
            return "Invalid JSON format"
        case .invalidFormat:
            return "Invalid typed data format"
        case .missingTypes:
            return "Missing types definition"
        case .missingPrimaryType:
            return "Missing primary type"
        case .missingDomain:
            return "Missing domain"
        case .missingMessage:
            return "Missing message"
        case .invalidTypeDefinition:
            return "Invalid type definition"
        case .invalidFieldDefinition:
            return "Invalid field definition"
        }
    }
}

// MARK: - Connection Proposal

struct ConnectionProposal: Identifiable {
    let id: String
    let dApp: DAppMetadata
    let requestedPermissions: [String]
    let requestedChains: [String]
    let requestedAccounts: [String]

    var displayChains: String {
        requestedChains.compactMap { chainId in
            if chainId == "eip155:1" {
                return "Ethereum"
            } else if chainId == "eip155:137" {
                return "Polygon"
            } else if chainId == "eip155:56" {
                return "BSC"
            }
            return chainId
        }.joined(separator: ", ")
    }
}

// MARK: - JSON-RPC Response

struct JSONRPCResponse {
    let id: Int64
    let result: Any?
    let error: JSONRPCError?

    init(id: Int64, result: Any?) {
        self.id = id
        self.result = result
        self.error = nil
    }

    static func error(_ error: JSONRPCError) -> JSONRPCResponse {
        return JSONRPCResponse(id: 0, result: nil, error: error)
    }

    private init(id: Int64, result: Any?, error: JSONRPCError?) {
        self.id = id
        self.result = result
        self.error = error
    }
}

struct JSONRPCError: Codable {
    let code: Int
    let message: String

    static let userRejected = JSONRPCError(code: 4001, message: "User rejected the request")
    static let unauthorized = JSONRPCError(code: 4100, message: "Unauthorized")
    static let unsupportedMethod = JSONRPCError(code: 4200, message: "Unsupported method")
    static let disconnected = JSONRPCError(code: 4900, message: "Disconnected")
    static let chainDisconnected = JSONRPCError(code: 4901, message: "Chain disconnected")
    static let invalidParams = JSONRPCError(code: -32602, message: "Invalid params")
    static let internalError = JSONRPCError(code: -32603, message: "Internal error")
    static let sessionNotFound = JSONRPCError(code: 5000, message: "Session not found")
}

// MARK: - WalletConnect Error

enum WalletConnectError: LocalizedError {
    case clientNotInitialized
    case initializationFailed(Error)
    case pairingFailed(Error)
    case invalidDeepLink
    case sessionApprovalFailed(Error)
    case sessionRejectionFailed(Error)
    case disconnectionFailed(Error)
    case responseFailed(Error)
    case unsupportedMethod(String)
    case invalidParams
    case invalidAddress
    case addressNotOwned
    case unsupportedNetwork
    case sessionExpired
    case userRejected
    case invalidConfiguration
    case unsupportedChain(Int)
    case unsupportedAssetType

    var errorDescription: String? {
        switch self {
        case .clientNotInitialized:
            return "WalletConnect client not initialized"
        case .initializationFailed(let error):
            return "Initialization failed: \(error.localizedDescription)"
        case .pairingFailed(let error):
            return "Pairing failed: \(error.localizedDescription)"
        case .invalidDeepLink:
            return "Invalid WalletConnect deep link"
        case .sessionApprovalFailed(let error):
            return "Session approval failed: \(error.localizedDescription)"
        case .sessionRejectionFailed(let error):
            return "Session rejection failed: \(error.localizedDescription)"
        case .disconnectionFailed(let error):
            return "Disconnection failed: \(error.localizedDescription)"
        case .responseFailed(let error):
            return "Response failed: \(error.localizedDescription)"
        case .unsupportedMethod(let method):
            return "Unsupported method: \(method)"
        case .invalidParams:
            return "Invalid parameters"
        case .invalidAddress:
            return "Invalid Ethereum address"
        case .addressNotOwned:
            return "Address not owned by this wallet"
        case .unsupportedNetwork:
            return "Unsupported network"
        case .sessionExpired:
            return "Session has expired"
        case .userRejected:
            return "User rejected the request"
        case .invalidConfiguration:
            return "Invalid WalletConnect configuration"
        case .unsupportedChain(let chainId):
            return "Unsupported chain: \(chainId)"
        case .unsupportedAssetType:
            return "Unsupported asset type"
        }
    }

    func toJSONRPCError() -> JSONRPCError {
        switch self {
        case .userRejected:
            return .userRejected
        case .invalidParams, .invalidAddress:
            return .invalidParams
        case .unsupportedMethod:
            return .unsupportedMethod
        case .sessionExpired:
            return .sessionNotFound
        default:
            return .internalError
        }
    }
}

// MARK: - Session Update

struct SessionUpdate {
    let topic: String
    let namespaces: [String: SessionNamespace]
}

// MARK: - Session Delete

struct Session {
    let topic: String
    let expiryDate: Date
}

// MARK: - App Metadata

struct AppMetadata {
    let name: String
    let description: String
    let url: String
    let icons: [String]
    let redirect: String
}

// MARK: - WalletConnect Client Config

struct WalletConnectClientConfig {
    let metadata: AppMetadata
    let projectId: String
    let relayHost: String
    let socketConnectionType: SocketConnectionType
}

enum SocketConnectionType {
    case automatic
    case manual
}

// MARK: - Rejection Reason

enum RejectionReason {
    case userRejected

    var message: String {
        switch self {
        case .userRejected:
            return "User rejected the session"
        }
    }
}

// MARK: - String Extension for Ethereum Address Validation

extension String {
    var isValidEthereumAddress: Bool {
        let pattern = "^0x[a-fA-F0-9]{40}$"
        let regex = try? NSRegularExpression(pattern: pattern)
        let range = NSRange(location: 0, length: self.utf16.count)
        return regex?.firstMatch(in: self, range: range) != nil
    }
}

// MARK: - BigUInt Extension

extension BigUInt {
    init?(hex: String) {
        let cleanHex = hex.replacingOccurrences(of: "0x", with: "")
        self.init(cleanHex, radix: 16)
    }
}

// MARK: - Data Extension

extension Data {
    init?(hexString: String) {
        let cleanHex = hexString.replacingOccurrences(of: "0x", with: "")

        guard cleanHex.count % 2 == 0 else {
            return nil
        }

        var data = Data()
        var index = cleanHex.startIndex

        while index < cleanHex.endIndex {
            let nextIndex = cleanHex.index(index, offsetBy: 2)
            let byteString = cleanHex[index..<nextIndex]

            guard let byte = UInt8(byteString, radix: 16) else {
                return nil
            }

            data.append(byte)
            index = nextIndex
        }

        self = data
    }

    var hexString: String {
        return map { String(format: "%02x", $0) }.joined()
    }
}

extension Data {
    init(hex: String) {
        self.init(hexString: hex) ?? Data()
    }
}
