//
//  SignatureHandler.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import Foundation
import CryptoKit
import BigInt

/// Handles cryptographic signing operations for WalletConnect
actor SignatureHandler {
    // MARK: - Dependencies

    private let keychainManager: KeychainManager
    private let networkManager: NetworkManager
    private let logger = Logger(subsystem: "io.etrid.wallet", category: "SignatureHandler")

    // MARK: - Constants

    private struct Constants {
        static let personalSignPrefix = "\u{19}Ethereum Signed Message:\n"
        static let eip712Prefix = Data([0x19, 0x01])
    }

    // MARK: - Initialization

    init(keychainManager: KeychainManager, networkManager: NetworkManager) {
        self.keychainManager = keychainManager
        self.networkManager = networkManager
    }

    // MARK: - Personal Sign

    /// Sign a personal message (EIP-191)
    func signPersonalMessage(message: String, address: String) async throws -> String {
        logger.info("Signing personal message for address: \(address)")

        // Get message data
        let messageData: Data
        if message.hasPrefix("0x") {
            guard let data = Data(hexString: String(message.dropFirst(2))) else {
                throw SignatureError.invalidMessageFormat
            }
            messageData = data
        } else {
            messageData = Data(message.utf8)
        }

        // Apply EIP-191 prefix
        let prefixedMessage = buildPersonalSignMessage(messageData)

        // Hash the message
        let messageHash = keccak256(prefixedMessage)

        // Get private key
        let privateKey = try await keychainManager.getPrivateKey(for: address)

        // Sign
        let signature = try signHash(messageHash, privateKey: privateKey)

        logger.info("Personal message signed successfully")
        return signature
    }

    /// Sign an eth_sign message (raw hash signing - dangerous!)
    func signEthMessage(message: String, address: String) async throws -> String {
        logger.warning("Signing eth_sign message - dangerous operation!")

        guard let messageData = Data(hexString: message.replacingOccurrences(of: "0x", with: "")) else {
            throw SignatureError.invalidMessageFormat
        }

        // eth_sign doesn't add prefix, signs the hash directly
        let messageHash = keccak256(messageData)

        let privateKey = try await keychainManager.getPrivateKey(for: address)
        let signature = try signHash(messageHash, privateKey: privateKey)

        logger.info("eth_sign message signed")
        return signature
    }

    // MARK: - Typed Data Signing

    /// Sign EIP-712 typed data v4
    func signTypedData(typedData: TypedData, address: String) async throws -> String {
        logger.info("Signing EIP-712 typed data v4")

        // Compute domain separator
        let domainSeparator = try hashStruct(
            typedData: typedData.domain,
            typeName: "EIP712Domain",
            types: typedData.types
        )

        // Compute message hash
        let messageHash = try hashStruct(
            typedData: typedData.message,
            typeName: typedData.primaryType,
            types: typedData.types
        )

        // Compute final hash: keccak256("\x19\x01" + domainSeparator + messageHash)
        let finalHash = keccak256(Constants.eip712Prefix + domainSeparator + messageHash)

        // Sign
        let privateKey = try await keychainManager.getPrivateKey(for: address)
        let signature = try signHash(finalHash, privateKey: privateKey)

        logger.info("Typed data signed successfully")
        return signature
    }

    /// Sign EIP-712 typed data v1 (legacy)
    func signTypedDataV1(typedData: TypedData, address: String) async throws -> String {
        logger.info("Signing EIP-712 typed data v1")

        // V1 uses simpler encoding
        let encodedData = try encodeTypedDataV1(typedData)
        let messageHash = keccak256(encodedData)

        let privateKey = try await keychainManager.getPrivateKey(for: address)
        let signature = try signHash(messageHash, privateKey: privateKey)

        logger.info("Typed data v1 signed successfully")
        return signature
    }

    // MARK: - Transaction Signing

    /// Sign an Ethereum transaction
    func signTransaction(transaction: EthereumTransaction, privateKey: Data) async throws -> String {
        logger.info("Signing transaction")

        // Serialize transaction for signing
        let serialized = try serializeTransaction(transaction)

        // Hash the serialized transaction
        let txHash = keccak256(serialized)

        // Sign
        let signature = try signHash(txHash, privateKey: privateKey)

        // Encode signed transaction
        let signedTx = try encodeSignedTransaction(transaction, signature: signature)

        logger.info("Transaction signed successfully")
        return signedTx
    }

    // MARK: - Security Checks

    /// Check message for dangerous patterns
    func checkSecurityWarnings(message: String) -> [String] {
        var warnings: [String] = []

        let dangerousPatterns: [(pattern: String, warning: String)] = [
            ("approve", "This message may approve token spending"),
            ("setApprovalForAll", "This message may approve unlimited access to your NFTs"),
            ("transferFrom", "This message may transfer tokens from your account"),
            ("permit", "This message may grant permission to spend your tokens"),
            ("transfer ownership", "This message may transfer ownership of a contract"),
            ("delegate", "This message may delegate voting power or permissions"),
            ("authorize", "This message may authorize access to your funds"),
        ]

        let lowerMessage = message.lowercased()

        for (pattern, warning) in dangerousPatterns {
            if lowerMessage.contains(pattern) {
                warnings.append(warning)
            }
        }

        return warnings
    }

    /// Check typed data for dangerous patterns
    func checkTypedDataWarnings(typedData: TypedData) -> [String] {
        var warnings: [String] = []

        // Check primary type
        let dangerousTypes = ["Permit", "Approve", "Transfer", "Delegate"]
        if dangerousTypes.contains(where: { typedData.primaryType.contains($0) }) {
            warnings.append("This signature may authorize token operations")
        }

        // Check message content
        let messageString = formatTypedDataForWarningCheck(typedData.message)
        let messageWarnings = checkSecurityWarnings(message: messageString)
        warnings.append(contentsOf: messageWarnings)

        // Check for unlimited approvals
        if let value = typedData.message["value"] as? String,
           value.contains("ffffffff") || value.lowercased().contains("unlimited") {
            warnings.append("This may approve UNLIMITED token spending")
        }

        return warnings
    }

    /// Format typed data for human-readable display
    func formatTypedData(_ typedData: TypedData) throws -> String {
        var formatted = "Domain:\n"

        // Format domain
        for (key, value) in typedData.domain {
            formatted += "  \(key): \(formatValue(value))\n"
        }

        formatted += "\n\(typedData.primaryType):\n"

        // Format message
        if let typeFields = typedData.types[typedData.primaryType] {
            for field in typeFields {
                if let value = typedData.message[field.name] {
                    formatted += "  \(field.name) (\(field.type)): \(formatValue(value))\n"
                }
            }
        } else {
            // Fallback to simple formatting
            for (key, value) in typedData.message {
                formatted += "  \(key): \(formatValue(value))\n"
            }
        }

        return formatted
    }

    // MARK: - Private Methods

    private func buildPersonalSignMessage(_ message: Data) -> Data {
        let prefix = Constants.personalSignPrefix + String(message.count)
        var result = Data(prefix.utf8)
        result.append(message)
        return result
    }

    private func signHash(_ hash: Data, privateKey: Data) throws -> String {
        guard hash.count == 32 else {
            throw SignatureError.invalidHashLength
        }

        // Use secp256k1 to sign
        let signature = try Secp256k1.sign(message: hash, privateKey: privateKey)

        // Format as 0x-prefixed hex with v, r, s
        return "0x" + signature.hexString
    }

    private func hashStruct(
        typedData: [String: Any],
        typeName: String,
        types: [String: [TypedDataField]]
    ) throws -> Data {
        // Get type definition
        guard let typeFields = types[typeName] else {
            throw SignatureError.typeNotFound(typeName)
        }

        // Encode type
        let typeHash = try hashType(typeName: typeName, types: types)

        // Encode data
        var encodedData = typeHash

        for field in typeFields {
            guard let value = typedData[field.name] else {
                throw SignatureError.missingField(field.name)
            }

            let encodedValue = try encodeValue(value, type: field.type, types: types)
            encodedData.append(encodedValue)
        }

        return keccak256(encodedData)
    }

    private func hashType(typeName: String, types: [String: [TypedDataField]]) throws -> Data {
        let typeString = try encodeType(typeName: typeName, types: types)
        return keccak256(Data(typeString.utf8))
    }

    private func encodeType(typeName: String, types: [String: [TypedDataField]]) throws -> String {
        guard let fields = types[typeName] else {
            throw SignatureError.typeNotFound(typeName)
        }

        var result = "\(typeName)("
        result += fields.map { "\($0.type) \($0.name)" }.joined(separator: ",")
        result += ")"

        // Find referenced types and sort alphabetically
        var referencedTypes: Set<String> = []
        for field in fields {
            let baseType = field.type.components(separatedBy: "[")[0]
            if !isPrimitiveType(baseType) && baseType != typeName {
                referencedTypes.insert(baseType)
            }
        }

        for refType in referencedTypes.sorted() {
            result += try encodeType(typeName: refType, types: types)
        }

        return result
    }

    private func encodeValue(_ value: Any, type: String, types: [String: [TypedDataField]]) throws -> Data {
        // Handle arrays
        if type.hasSuffix("]") {
            guard let array = value as? [Any] else {
                throw SignatureError.invalidValue
            }

            let elementType = String(type.split(separator: "[")[0])
            var arrayData = Data()

            for element in array {
                let encoded = try encodeValue(element, type: elementType, types: types)
                arrayData.append(encoded)
            }

            return keccak256(arrayData)
        }

        // Handle custom types
        if !isPrimitiveType(type) {
            guard let structData = value as? [String: Any] else {
                throw SignatureError.invalidValue
            }

            return try hashStruct(typedData: structData, typeName: type, types: types)
        }

        // Handle primitive types
        return try encodePrimitive(value, type: type)
    }

    private func encodePrimitive(_ value: Any, type: String) throws -> Data {
        var result = Data(count: 32) // All values are padded to 32 bytes

        switch type {
        case "string":
            guard let string = value as? String else {
                throw SignatureError.invalidValue
            }
            let hash = keccak256(Data(string.utf8))
            return hash

        case "bytes":
            guard let hexString = value as? String,
                  let bytes = Data(hexString: hexString.replacingOccurrences(of: "0x", with: "")) else {
                throw SignatureError.invalidValue
            }
            return keccak256(bytes)

        case let t where t.hasPrefix("bytes"):
            guard let hexString = value as? String,
                  let bytes = Data(hexString: hexString.replacingOccurrences(of: "0x", with: "")) else {
                throw SignatureError.invalidValue
            }
            result.replaceSubrange(0..<bytes.count, with: bytes)
            return result

        case "address":
            guard let address = value as? String,
                  let addressData = Data(hexString: address.replacingOccurrences(of: "0x", with: "")) else {
                throw SignatureError.invalidValue
            }
            result.replaceSubrange(12..<32, with: addressData)
            return result

        case "bool":
            let boolValue: Bool
            if let b = value as? Bool {
                boolValue = b
            } else if let s = value as? String {
                boolValue = s == "true"
            } else {
                throw SignatureError.invalidValue
            }
            result[31] = boolValue ? 1 : 0
            return result

        case let t where t.hasPrefix("uint") || t.hasPrefix("int"):
            let number: BigUInt
            if let n = value as? Int {
                number = BigUInt(n)
            } else if let s = value as? String {
                if s.hasPrefix("0x") {
                    number = BigUInt(hex: s) ?? 0
                } else {
                    number = BigUInt(s) ?? 0
                }
            } else {
                throw SignatureError.invalidValue
            }

            let bytes = number.serialize()
            let offset = 32 - bytes.count
            result.replaceSubrange(offset..<32, with: bytes)
            return result

        default:
            throw SignatureError.unsupportedType(type)
        }
    }

    private func isPrimitiveType(_ type: String) -> Bool {
        let primitives = ["address", "bool", "string", "bytes"]

        if primitives.contains(type) {
            return true
        }

        if type.hasPrefix("bytes") && type.count > 5 {
            return true
        }

        if type.hasPrefix("uint") || type.hasPrefix("int") {
            return true
        }

        return false
    }

    private func encodeTypedDataV1(_ typedData: TypedData) throws -> Data {
        // V1 uses simpler encoding - just concatenate values
        var encoded = Data()

        for (_, value) in typedData.message.sorted(by: { $0.key < $1.key }) {
            if let string = value as? String {
                encoded.append(Data(string.utf8))
            } else if let number = value as? Int {
                encoded.append(Data(String(number).utf8))
            }
        }

        return encoded
    }

    private func serializeTransaction(_ transaction: EthereumTransaction) throws -> Data {
        // RLP encode transaction
        var items: [Any] = []

        items.append(transaction.nonce ?? 0)
        items.append(transaction.gasPrice ?? 0)
        items.append(transaction.gas ?? 0)
        items.append(Data(hexString: transaction.to.replacingOccurrences(of: "0x", with: "")) ?? Data())
        items.append(transaction.value)
        items.append(transaction.data)

        // Add chain ID for EIP-155
        if let chainId = transaction.chainId {
            items.append(chainId)
            items.append(0)
            items.append(0)
        }

        return try RLPEncoder.encode(items)
    }

    private func encodeSignedTransaction(_ transaction: EthereumTransaction, signature: String) throws -> String {
        // Parse signature into v, r, s
        let sig = signature.replacingOccurrences(of: "0x", with: "")
        guard sig.count == 130 else {
            throw SignatureError.invalidSignature
        }

        let r = String(sig.prefix(64))
        let s = String(sig.dropFirst(64).prefix(64))
        let v = String(sig.suffix(2))

        // RLP encode with signature
        var items: [Any] = []
        items.append(transaction.nonce ?? 0)
        items.append(transaction.gasPrice ?? 0)
        items.append(transaction.gas ?? 0)
        items.append(Data(hexString: transaction.to.replacingOccurrences(of: "0x", with: "")) ?? Data())
        items.append(transaction.value)
        items.append(transaction.data)
        items.append(Data(hexString: v) ?? Data())
        items.append(Data(hexString: r) ?? Data())
        items.append(Data(hexString: s) ?? Data())

        let encoded = try RLPEncoder.encode(items)
        return "0x" + encoded.hexString
    }

    private func formatValue(_ value: Any) -> String {
        if let string = value as? String {
            return string
        } else if let number = value as? Int {
            return String(number)
        } else if let dict = value as? [String: Any] {
            return "{ " + dict.map { "\($0.key): \(formatValue($0.value))" }.joined(separator: ", ") + " }"
        } else if let array = value as? [Any] {
            return "[" + array.map { formatValue($0) }.joined(separator: ", ") + "]"
        }
        return String(describing: value)
    }

    private func formatTypedDataForWarningCheck(_ message: [String: Any]) -> String {
        message.map { "\($0.key): \(formatValue($0.value))" }.joined(separator: " ")
    }

    private func keccak256(_ data: Data) -> Data {
        // Use Keccak-256 implementation
        return Keccak256.hash(data: data)
    }
}

// MARK: - Errors

enum SignatureError: LocalizedError {
    case invalidMessageFormat
    case invalidHashLength
    case invalidSignature
    case typeNotFound(String)
    case missingField(String)
    case invalidValue
    case unsupportedType(String)

    var errorDescription: String? {
        switch self {
        case .invalidMessageFormat:
            return "Invalid message format"
        case .invalidHashLength:
            return "Invalid hash length - expected 32 bytes"
        case .invalidSignature:
            return "Invalid signature format"
        case .typeNotFound(let type):
            return "Type not found: \(type)"
        case .missingField(let field):
            return "Missing required field: \(field)"
        case .invalidValue:
            return "Invalid value for encoding"
        case .unsupportedType(let type):
            return "Unsupported type: \(type)"
        }
    }
}
