//
//  RLPEncoder.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import Foundation
import BigInt

/// RLP (Recursive Length Prefix) encoding implementation
/// Used for Ethereum transaction serialization
struct RLPEncoder {
    /// Encode value to RLP format
    static func encode(_ value: Any) throws -> Data {
        if let data = value as? Data {
            return encodeData(data)
        } else if let string = value as? String {
            return encodeData(Data(string.utf8))
        } else if let number = value as? Int {
            return encodeInt(number)
        } else if let bigInt = value as? BigUInt {
            return encodeBigUInt(bigInt)
        } else if let array = value as? [Any] {
            return try encodeList(array)
        } else {
            throw RLPError.unsupportedType
        }
    }

    // MARK: - Private Methods

    private static func encodeData(_ data: Data) -> Data {
        if data.count == 1 && data[0] < 0x80 {
            // Single byte less than 128
            return data
        } else if data.count <= 55 {
            // Short string (0-55 bytes)
            var result = Data([UInt8(0x80 + data.count)])
            result.append(data)
            return result
        } else {
            // Long string (more than 55 bytes)
            let lengthBytes = encodeLength(data.count)
            var result = Data([UInt8(0xb7 + lengthBytes.count)])
            result.append(lengthBytes)
            result.append(data)
            return result
        }
    }

    private static func encodeInt(_ value: Int) -> Data {
        if value == 0 {
            return Data([0x80])
        }

        let bigInt = BigUInt(value)
        return encodeBigUInt(bigInt)
    }

    private static func encodeBigUInt(_ value: BigUInt) -> Data {
        if value == 0 {
            return Data([0x80])
        }

        let bytes = value.serialize()
        return encodeData(bytes)
    }

    private static func encodeList(_ list: [Any]) throws -> Data {
        var encodedItems = Data()

        for item in list {
            let encoded = try encode(item)
            encodedItems.append(encoded)
        }

        if encodedItems.count <= 55 {
            // Short list
            var result = Data([UInt8(0xc0 + encodedItems.count)])
            result.append(encodedItems)
            return result
        } else {
            // Long list
            let lengthBytes = encodeLength(encodedItems.count)
            var result = Data([UInt8(0xf7 + lengthBytes.count)])
            result.append(lengthBytes)
            result.append(encodedItems)
            return result
        }
    }

    private static func encodeLength(_ length: Int) -> Data {
        if length == 0 {
            return Data()
        }

        var lengthBytes = Data()
        var remaining = length

        while remaining > 0 {
            lengthBytes.insert(UInt8(remaining & 0xff), at: 0)
            remaining >>= 8
        }

        return lengthBytes
    }
}

// MARK: - RLP Decoder

struct RLPDecoder {
    /// Decode RLP-encoded data
    static func decode(_ data: Data) throws -> Any {
        if data.isEmpty {
            throw RLPError.emptyData
        }

        let (decoded, _) = try decodeItem(data, offset: 0)
        return decoded
    }

    // MARK: - Private Methods

    private static func decodeItem(_ data: Data, offset: Int) throws -> (Any, Int) {
        guard offset < data.count else {
            throw RLPError.invalidOffset
        }

        let prefix = data[offset]

        if prefix <= 0x7f {
            // Single byte
            return (Data([prefix]), offset + 1)

        } else if prefix <= 0xb7 {
            // Short string
            let length = Int(prefix - 0x80)
            let endIndex = offset + 1 + length

            guard endIndex <= data.count else {
                throw RLPError.invalidLength
            }

            let stringData = data.subdata(in: (offset + 1)..<endIndex)
            return (stringData, endIndex)

        } else if prefix <= 0xbf {
            // Long string
            let lengthOfLength = Int(prefix - 0xb7)
            let lengthEndIndex = offset + 1 + lengthOfLength

            guard lengthEndIndex <= data.count else {
                throw RLPError.invalidLength
            }

            let lengthBytes = data.subdata(in: (offset + 1)..<lengthEndIndex)
            let length = try decodeLength(lengthBytes)
            let endIndex = lengthEndIndex + length

            guard endIndex <= data.count else {
                throw RLPError.invalidLength
            }

            let stringData = data.subdata(in: lengthEndIndex..<endIndex)
            return (stringData, endIndex)

        } else if prefix <= 0xf7 {
            // Short list
            let length = Int(prefix - 0xc0)
            let endIndex = offset + 1 + length

            guard endIndex <= data.count else {
                throw RLPError.invalidLength
            }

            let listData = data.subdata(in: (offset + 1)..<endIndex)
            let list = try decodeList(listData)
            return (list, endIndex)

        } else {
            // Long list
            let lengthOfLength = Int(prefix - 0xf7)
            let lengthEndIndex = offset + 1 + lengthOfLength

            guard lengthEndIndex <= data.count else {
                throw RLPError.invalidLength
            }

            let lengthBytes = data.subdata(in: (offset + 1)..<lengthEndIndex)
            let length = try decodeLength(lengthBytes)
            let endIndex = lengthEndIndex + length

            guard endIndex <= data.count else {
                throw RLPError.invalidLength
            }

            let listData = data.subdata(in: lengthEndIndex..<endIndex)
            let list = try decodeList(listData)
            return (list, endIndex)
        }
    }

    private static func decodeList(_ data: Data) throws -> [Any] {
        var items: [Any] = []
        var offset = 0

        while offset < data.count {
            let (item, newOffset) = try decodeItem(data, offset: offset)
            items.append(item)
            offset = newOffset
        }

        return items
    }

    private static func decodeLength(_ data: Data) throws -> Int {
        guard !data.isEmpty else {
            throw RLPError.invalidLength
        }

        var length = 0
        for byte in data {
            length = (length << 8) | Int(byte)
        }

        return length
    }
}

// MARK: - Errors

enum RLPError: LocalizedError {
    case unsupportedType
    case emptyData
    case invalidOffset
    case invalidLength
    case invalidFormat

    var errorDescription: String? {
        switch self {
        case .unsupportedType:
            return "Unsupported type for RLP encoding"
        case .emptyData:
            return "Cannot decode empty data"
        case .invalidOffset:
            return "Invalid offset in RLP data"
        case .invalidLength:
            return "Invalid length in RLP encoding"
        case .invalidFormat:
            return "Invalid RLP format"
        }
    }
}

// MARK: - Extensions

extension BigUInt {
    func serialize() -> Data {
        // Serialize BigUInt to bytes (big-endian)
        let string = String(self, radix: 16)
        let paddedString = string.count % 2 == 0 ? string : "0" + string

        var data = Data()
        var index = paddedString.startIndex

        while index < paddedString.endIndex {
            let nextIndex = paddedString.index(index, offsetBy: 2)
            let byteString = paddedString[index..<nextIndex]

            if let byte = UInt8(byteString, radix: 16) {
                data.append(byte)
            }

            index = nextIndex
        }

        return data
    }
}
