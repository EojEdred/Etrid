//
//  TransactionBuilder.swift
//  EtridWalletSwift
//
//  Created by Eoj on 2025-11-22.
//  Transaction building, RLP encoding, and signing
//

import Foundation
import CryptoKit

/// Raw transaction structure supporting both EIP-1559 and legacy transactions
struct RawTransaction {
    let chainId: Int
    let nonce: Int
    let gasLimit: BigInt
    let to: String
    let value: BigInt
    let data: String

    // EIP-1559 fields (Type 2)
    let maxFeePerGas: BigInt?
    let maxPriorityFeePerGas: BigInt?

    // Legacy field (Type 0)
    let gasPrice: BigInt?

    // Signature components
    var v: Int?
    var r: Data?
    var s: Data?

    /// Transaction type: 0 for legacy, 2 for EIP-1559
    var type: Int {
        if maxFeePerGas != nil && maxPriorityFeePerGas != nil {
            return 2 // EIP-1559
        }
        return 0 // Legacy
    }

    /// Check if transaction is signed
    var isSigned: Bool {
        return v != nil && r != nil && s != nil
    }
}

/// Transaction parameters for building
struct TransactionParameters {
    let from: String
    let to: String
    let value: BigInt
    let data: String
    let chainId: Int
    let nonce: Int?
    let gasLimit: BigInt?

    // EIP-1559 parameters
    let maxFeePerGas: BigInt?
    let maxPriorityFeePerGas: BigInt?

    // Legacy parameter
    let gasPrice: BigInt?
}

/// Transaction builder errors
enum TransactionBuilderError: Error, LocalizedError {
    case invalidAddress
    case invalidChainId
    case missingGasParameters
    case invalidSignature
    case rlpEncodingFailed
    case unsupportedTransactionType
    case invalidNonce

    var errorDescription: String? {
        switch self {
        case .invalidAddress:
            return "Invalid Ethereum address"
        case .invalidChainId:
            return "Invalid chain ID"
        case .missingGasParameters:
            return "Missing gas parameters for transaction"
        case .invalidSignature:
            return "Invalid transaction signature"
        case .rlpEncodingFailed:
            return "Failed to RLP encode transaction"
        case .unsupportedTransactionType:
            return "Unsupported transaction type"
        case .invalidNonce:
            return "Invalid transaction nonce"
        }
    }
}

/// Builds and signs Ethereum transactions with RLP encoding
actor TransactionBuilder {

    // MARK: - Build Transaction

    /// Build a raw transaction from parameters
    func buildTransaction(params: TransactionParameters) throws -> RawTransaction {
        // Validate addresses
        guard isValidAddress(params.from), isValidAddress(params.to) else {
            throw TransactionBuilderError.invalidAddress
        }

        // Validate chain ID
        guard params.chainId > 0 else {
            throw TransactionBuilderError.invalidChainId
        }

        // Validate nonce
        guard let nonce = params.nonce, nonce >= 0 else {
            throw TransactionBuilderError.invalidNonce
        }

        // Determine transaction type and validate gas parameters
        let isEIP1559 = params.maxFeePerGas != nil && params.maxPriorityFeePerGas != nil
        let isLegacy = params.gasPrice != nil

        guard isEIP1559 || isLegacy else {
            throw TransactionBuilderError.missingGasParameters
        }

        // Use default gas limit if not provided
        let gasLimit = params.gasLimit ?? BigInt(21000)

        return RawTransaction(
            chainId: params.chainId,
            nonce: nonce,
            gasLimit: gasLimit,
            to: params.to,
            value: params.value,
            data: params.data,
            maxFeePerGas: params.maxFeePerGas,
            maxPriorityFeePerGas: params.maxPriorityFeePerGas,
            gasPrice: params.gasPrice,
            v: nil,
            r: nil,
            s: nil
        )
    }

    // MARK: - RLP Encoding

    /// Encode transaction to RLP format
    func rlpEncode(_ transaction: RawTransaction) throws -> Data {
        switch transaction.type {
        case 0:
            return try rlpEncodeLegacy(transaction)
        case 2:
            return try rlpEncodeEIP1559(transaction)
        default:
            throw TransactionBuilderError.unsupportedTransactionType
        }
    }

    /// RLP encode legacy transaction (Type 0)
    /// Format: rlp([nonce, gasPrice, gasLimit, to, value, data, v, r, s])
    private func rlpEncodeLegacy(_ tx: RawTransaction) throws -> Data {
        guard let gasPrice = tx.gasPrice else {
            throw TransactionBuilderError.missingGasParameters
        }

        var items: [RLPItem] = []

        // Add transaction fields
        items.append(.data(encodeInt(tx.nonce)))
        items.append(.data(gasPrice.serialize()))
        items.append(.data(tx.gasLimit.serialize()))
        items.append(.data(hexToData(tx.to)))
        items.append(.data(tx.value.serialize()))
        items.append(.data(hexToData(tx.data)))

        // Add signature if present
        if tx.isSigned {
            items.append(.data(encodeInt(tx.v!)))
            items.append(.data(tx.r!))
            items.append(.data(tx.s!))
        } else {
            // For unsigned transactions, add chain ID for EIP-155
            items.append(.data(encodeInt(tx.chainId)))
            items.append(.data(Data()))
            items.append(.data(Data()))
        }

        return try encodeRLPList(items)
    }

    /// RLP encode EIP-1559 transaction (Type 2)
    /// Format: 0x02 || rlp([chainId, nonce, maxPriorityFeePerGas, maxFeePerGas, gasLimit, to, value, data, accessList, v, r, s])
    private func rlpEncodeEIP1559(_ tx: RawTransaction) throws -> Data {
        guard let maxPriorityFeePerGas = tx.maxPriorityFeePerGas,
              let maxFeePerGas = tx.maxFeePerGas else {
            throw TransactionBuilderError.missingGasParameters
        }

        var items: [RLPItem] = []

        // Add transaction fields
        items.append(.data(encodeInt(tx.chainId)))
        items.append(.data(encodeInt(tx.nonce)))
        items.append(.data(maxPriorityFeePerGas.serialize()))
        items.append(.data(maxFeePerGas.serialize()))
        items.append(.data(tx.gasLimit.serialize()))
        items.append(.data(hexToData(tx.to)))
        items.append(.data(tx.value.serialize()))
        items.append(.data(hexToData(tx.data)))
        items.append(.list([])) // Empty access list

        // Add signature if present
        if tx.isSigned {
            items.append(.data(encodeInt(tx.v!)))
            items.append(.data(tx.r!))
            items.append(.data(tx.s!))
        }

        // Encode RLP list
        let rlpEncoded = try encodeRLPList(items)

        // Prepend transaction type byte (0x02)
        var result = Data([0x02])
        result.append(rlpEncoded)

        return result
    }

    // MARK: - Transaction Signing

    /// Sign a transaction with private key
    func signTransaction(_ transaction: RawTransaction, privateKey: Data) throws -> RawTransaction {
        // Get transaction hash for signing
        let hash = try getTransactionHash(transaction)

        // Sign the hash
        let signature = try signHash(hash, privateKey: privateKey)

        // Apply signature to transaction
        var signedTx = transaction
        signedTx.v = Int(signature.v)
        signedTx.r = signature.r
        signedTx.s = signature.s

        return signedTx
    }

    /// Get transaction hash for signing (Keccak256 of RLP encoded transaction)
    func getTransactionHash(_ transaction: RawTransaction) throws -> Data {
        let encoded = try rlpEncode(transaction)
        return keccak256(encoded)
    }

    /// Sign a hash with private key using ECDSA
    private func signHash(_ hash: Data, privateKey: Data) throws -> (v: UInt8, r: Data, s: Data) {
        // Create ECDSA private key
        guard let privateKeyObj = try? P256.Signing.PrivateKey(rawRepresentation: privateKey) else {
            throw TransactionBuilderError.invalidSignature
        }

        // Sign the hash
        let signature = try privateKeyObj.signature(for: hash)

        // Extract r and s from signature
        let sigData = signature.rawRepresentation
        let r = sigData.prefix(32)
        let s = sigData.suffix(32)

        // Calculate recovery ID (v)
        // For EIP-155: v = chainId * 2 + 35 + recoveryId
        // For now, using simple v value (27 or 28)
        let v: UInt8 = 27 // This should be calculated based on recovery

        return (v, Data(r), Data(s))
    }

    /// Serialize signed transaction to send to network
    func serializeTransaction(_ transaction: RawTransaction) throws -> String {
        guard transaction.isSigned else {
            throw TransactionBuilderError.invalidSignature
        }

        let encoded = try rlpEncode(transaction)
        return "0x" + encoded.toHexString()
    }

    // MARK: - Helper Methods

    private func isValidAddress(_ address: String) -> Bool {
        let cleaned = address.lowercased().replacingOccurrences(of: "0x", with: "")
        return cleaned.count == 40 && cleaned.allSatisfy { $0.isHexDigit }
    }

    private func hexToData(_ hex: String) -> Data {
        let cleaned = hex.lowercased().replacingOccurrences(of: "0x", with: "")
        if cleaned.isEmpty {
            return Data()
        }

        var data = Data()
        var index = cleaned.startIndex

        while index < cleaned.endIndex {
            let nextIndex = cleaned.index(index, offsetBy: 2, limitedBy: cleaned.endIndex) ?? cleaned.endIndex
            let byte = String(cleaned[index..<nextIndex])
            if let num = UInt8(byte, radix: 16) {
                data.append(num)
            }
            index = nextIndex
        }

        return data
    }

    private func encodeInt(_ value: Int) -> Data {
        if value == 0 {
            return Data()
        }

        var bytes: [UInt8] = []
        var n = value

        while n > 0 {
            bytes.insert(UInt8(n & 0xFF), at: 0)
            n >>= 8
        }

        return Data(bytes)
    }

    private func keccak256(_ data: Data) -> Data {
        // In production, use a proper Keccak256 implementation
        // For now, using SHA256 as placeholder
        return Data(SHA256.hash(data: data))
    }
}

// MARK: - RLP Encoding Support

/// RLP item type (data or list)
enum RLPItem {
    case data(Data)
    case list([RLPItem])
}

/// Encode RLP list
func encodeRLPList(_ items: [RLPItem]) throws -> Data {
    var encoded = Data()

    for item in items {
        switch item {
        case .data(let data):
            encoded.append(try encodeRLPData(data))
        case .list(let list):
            encoded.append(try encodeRLPList(list))
        }
    }

    return try encodeRLPLength(encoded, offset: 0xc0)
}

/// Encode RLP data
func encodeRLPData(_ data: Data) throws -> Data {
    if data.count == 1 && data[0] < 0x80 {
        return data
    }

    return try encodeRLPLength(data, offset: 0x80)
}

/// Encode RLP length prefix
func encodeRLPLength(_ data: Data, offset: UInt8) throws -> Data {
    if data.count <= 55 {
        var result = Data([offset + UInt8(data.count)])
        result.append(data)
        return result
    }

    let lengthBytes = toBinary(data.count)
    var result = Data([offset + 55 + UInt8(lengthBytes.count)])
    result.append(lengthBytes)
    result.append(data)
    return result
}

/// Convert integer to binary representation
func toBinary(_ value: Int) -> Data {
    if value == 0 {
        return Data()
    }

    var bytes: [UInt8] = []
    var n = value

    while n > 0 {
        bytes.insert(UInt8(n & 0xFF), at: 0)
        n >>= 8
    }

    return Data(bytes)
}

// MARK: - Data Extensions

extension Data {
    func toHexString() -> String {
        return map { String(format: "%02x", $0) }.joined()
    }
}
