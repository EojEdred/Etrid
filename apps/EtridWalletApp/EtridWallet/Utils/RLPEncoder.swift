//
//  RLPEncoder.swift
//  EtridWallet
//
//  Recursive Length Prefix (RLP) encoding for Ethereum transactions
//

import Foundation
import CryptoSwift

struct RLPEncoder {
    // MARK: - RLP Encoding

    /// Encodes data using RLP
    static func encode(_ data: Any) -> Data {
        if let bytes = data as? Data {
            return encodeData(bytes)
        } else if let string = data as? String {
            if string.hasPrefix("0x") {
                guard let bytes = string.hexToData else {
                    return encodeData(Data())
                }
                return encodeData(bytes)
            }
            return encodeData(Data(string.utf8))
        } else if let number = data as? Int {
            return encodeInt(UInt64(number))
        } else if let number = data as? UInt64 {
            return encodeInt(number)
        } else if let array = data as? [Any] {
            return encodeList(array)
        } else {
            return encodeData(Data())
        }
    }

    /// Encodes byte array
    private static func encodeData(_ data: Data) -> Data {
        if data.count == 1 && data[0] < 0x80 {
            // Single byte < 128
            return data
        } else if data.count < 56 {
            // Short string (< 56 bytes)
            var result = Data([UInt8(0x80 + data.count)])
            result.append(data)
            return result
        } else {
            // Long string (>= 56 bytes)
            let lengthBytes = encodeLength(data.count)
            var result = Data([UInt8(0xb7 + lengthBytes.count)])
            result.append(lengthBytes)
            result.append(data)
            return result
        }
    }

    /// Encodes integer
    private static func encodeInt(_ value: UInt64) -> Data {
        if value == 0 {
            return Data([0x80])
        }

        var bytes = Data()
        var val = value
        while val > 0 {
            bytes.insert(UInt8(val & 0xff), at: 0)
            val >>= 8
        }

        return encodeData(bytes)
    }

    /// Encodes list
    private static func encodeList(_ items: [Any]) -> Data {
        var encoded = Data()
        for item in items {
            encoded.append(encode(item))
        }

        if encoded.count < 56 {
            // Short list
            var result = Data([UInt8(0xc0 + encoded.count)])
            result.append(encoded)
            return result
        } else {
            // Long list
            let lengthBytes = encodeLength(encoded.count)
            var result = Data([UInt8(0xf7 + lengthBytes.count)])
            result.append(lengthBytes)
            result.append(encoded)
            return result
        }
    }

    /// Encodes length as big-endian bytes
    private static func encodeLength(_ length: Int) -> Data {
        var bytes = Data()
        var len = length
        while len > 0 {
            bytes.insert(UInt8(len & 0xff), at: 0)
            len >>= 8
        }
        return bytes
    }

    // MARK: - Transaction Encoding

    /// Encodes EIP-1559 transaction
    static func encodeEIP1559Transaction(
        chainId: Int,
        nonce: Int,
        maxPriorityFeePerGas: String,
        maxFeePerGas: String,
        gasLimit: String,
        to: String,
        value: String,
        data: String?
    ) -> Data {
        let items: [Any] = [
            UInt64(chainId),
            UInt64(nonce),
            maxPriorityFeePerGas,
            maxFeePerGas,
            gasLimit,
            to,
            value,
            data ?? "0x"
        ]

        let encoded = encodeList(items)

        // Add EIP-2718 transaction type (0x02 for EIP-1559)
        var result = Data([0x02])
        result.append(encoded)

        return result
    }

    /// Encodes legacy transaction
    static func encodeLegacyTransaction(
        nonce: Int,
        gasPrice: String,
        gasLimit: String,
        to: String,
        value: String,
        data: String?,
        chainId: Int
    ) -> Data {
        let items: [Any] = [
            UInt64(nonce),
            gasPrice,
            gasLimit,
            to,
            value,
            data ?? "0x",
            UInt64(chainId),
            UInt64(0),
            UInt64(0)
        ]

        return encodeList(items)
    }

    /// Encodes signed EIP-1559 transaction
    static func encodeSignedEIP1559Transaction(
        chainId: Int,
        nonce: Int,
        maxPriorityFeePerGas: String,
        maxFeePerGas: String,
        gasLimit: String,
        to: String,
        value: String,
        data: String?,
        v: UInt8,
        r: Data,
        s: Data
    ) -> Data {
        let items: [Any] = [
            UInt64(chainId),
            UInt64(nonce),
            maxPriorityFeePerGas,
            maxFeePerGas,
            gasLimit,
            to,
            value,
            data ?? "0x",
            [] as [Any], // Access list (empty for now)
            UInt64(v),
            r,
            s
        ]

        let encoded = encodeList(items)

        // Add EIP-2718 transaction type
        var result = Data([0x02])
        result.append(encoded)

        return result
    }

    /// Encodes signed legacy transaction
    static func encodeSignedLegacyTransaction(
        nonce: Int,
        gasPrice: String,
        gasLimit: String,
        to: String,
        value: String,
        data: String?,
        v: UInt8,
        r: Data,
        s: Data
    ) -> Data {
        let items: [Any] = [
            UInt64(nonce),
            gasPrice,
            gasLimit,
            to,
            value,
            data ?? "0x",
            UInt64(v),
            r,
            s
        ]

        return encodeList(items)
    }
}

// MARK: - Keccak256 Helper
extension Data {
    /// Computes Keccak256 hash
    /// NOTE: This is a placeholder using SHA256
    /// In production, use a proper Keccak256 implementation like:
    /// - CryptoSwift library
    /// - web3swift library
    /// - or a native C implementation
    var keccak256Hash: Data {
        return self.sha3(.keccak256)
    }
}
