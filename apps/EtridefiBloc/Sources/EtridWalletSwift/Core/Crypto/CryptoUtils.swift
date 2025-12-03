import Foundation
import CryptoKit
import P256K
import CryptoSwift
import Blake2
import Base58Swift

// MARK: - Production-Ready Multichain Address Generation

/// Handles cryptographic operations for multiple blockchain networks
public actor CryptoUtils {

    public static let shared = CryptoUtils()

    private init() {}

    // MARK: - Secp256k1 Public Key Generation

    /// Generate compressed secp256k1 public key from 32-byte private key
    public func derivePublicKey(from privateKey: Data) throws -> Data {
        guard privateKey.count == 32 else {
            throw CryptoError.invalidPrivateKeyLength
        }

        let signingKey = try P256K.Signing.PrivateKey(dataRepresentation: privateKey)
        return signingKey.publicKey.dataRepresentation
    }

    /// Generate uncompressed secp256k1 public key (65 bytes) for Ethereum
    public func deriveUncompressedPublicKey(from privateKey: Data) throws -> Data {
        guard privateKey.count == 32 else {
            throw CryptoError.invalidPrivateKeyLength
        }

        // Create with uncompressed format
        let signingKey = try P256K.Signing.PrivateKey(dataRepresentation: privateKey, format: .uncompressed)
        return signingKey.publicKey.dataRepresentation
    }

    // MARK: - ËTRID / Substrate SS58 Address

    /// SS58 prefix for ËTRID network (using 42 as generic Substrate prefix)
    /// In production, ËTRID should have its own registered prefix
    public static let etridSS58Prefix: UInt16 = 42

    /// Generate SS58 address for ËTRID (Substrate-based)
    public func generateEtridAddress(from publicKey: Data) throws -> String {
        // Substrate uses the raw 32-byte public key (without prefix)
        // For secp256k1, we need to hash the compressed public key
        let accountId: Data

        if publicKey.count == 33 {
            // Compressed secp256k1 - hash with Blake2b-256
            accountId = try blake2b256(publicKey)
        } else if publicKey.count == 32 {
            // Already 32 bytes (Ed25519 or pre-hashed)
            accountId = publicKey
        } else {
            throw CryptoError.invalidPublicKeyLength
        }

        return try ss58Encode(accountId: accountId, prefix: Self.etridSS58Prefix)
    }

    /// SS58 encode with checksum (Blake2b)
    private func ss58Encode(accountId: Data, prefix: UInt16) throws -> String {
        var payload = Data()

        // Add prefix (simple encoding for prefix < 64)
        if prefix < 64 {
            payload.append(UInt8(prefix))
        } else {
            // Multi-byte prefix encoding
            let first = UInt8(((prefix & 0x00FC) >> 2) | 0x40)
            let second = UInt8((prefix >> 8) | ((prefix & 0x0003) << 6))
            payload.append(first)
            payload.append(second)
        }

        // Add account ID (32 bytes)
        payload.append(accountId)

        // Calculate checksum using Blake2b-512
        let ss58Prefix = "SS58PRE".data(using: .utf8)!
        var checksumInput = ss58Prefix
        checksumInput.append(payload)

        let fullHash = try blake2b512(checksumInput)
        let checksum = fullHash.prefix(2)

        // Append checksum
        payload.append(checksum)

        // Base58 encode
        return Base58.base58Encode(Array(payload))
    }

    /// Blake2b-256 hash
    private func blake2b256(_ data: Data) throws -> Data {
        var hasher = try Blake2b(size: 32)
        try hasher.update(data)
        return Data(try hasher.finalize())
    }

    /// Blake2b-512 hash
    private func blake2b512(_ data: Data) throws -> Data {
        var hasher = try Blake2b(size: 64)
        try hasher.update(data)
        return Data(try hasher.finalize())
    }

    // MARK: - Ethereum Address (Keccak256)

    /// Generate Ethereum address from private key
    public func generateEthereumAddress(from privateKey: Data) throws -> String {
        // Get uncompressed public key (65 bytes: 0x04 + x + y)
        let uncompressedPubKey = try deriveUncompressedPublicKey(from: privateKey)

        // Remove 0x04 prefix, keep only x and y coordinates (64 bytes)
        let pubKeyWithoutPrefix = uncompressedPubKey.dropFirst()

        // Keccak256 hash
        let hash = pubKeyWithoutPrefix.sha3(.keccak256)

        // Take last 20 bytes
        let addressBytes = hash.suffix(20)

        // Convert to hex with checksum (EIP-55)
        return checksumAddress(Data(addressBytes))
    }

    /// EIP-55 checksum address encoding
    private func checksumAddress(_ addressBytes: Data) -> String {
        let hexAddress = addressBytes.map { String(format: "%02x", $0) }.joined()
        let hashHex = hexAddress.data(using: .utf8)!.sha3(.keccak256).toHexString()

        var checksummed = "0x"
        for (i, char) in hexAddress.enumerated() {
            let hashChar = hashHex[hashHex.index(hashHex.startIndex, offsetBy: i)]
            if let hashInt = Int(String(hashChar), radix: 16), hashInt >= 8 {
                checksummed.append(char.uppercased())
            } else {
                checksummed.append(char)
            }
        }

        return checksummed
    }

    // MARK: - Bitcoin Address

    /// Generate Bitcoin address (P2PKH - Legacy) from private key
    public func generateBitcoinAddress(from privateKey: Data, testnet: Bool = false) throws -> String {
        // Get compressed public key
        let compressedPubKey = try derivePublicKey(from: privateKey)

        // SHA256
        let sha256Hash = Data(SHA256.hash(data: compressedPubKey))

        // RIPEMD160
        let ripemd160Hash = ripemd160(sha256Hash)

        // Add version byte (0x00 for mainnet, 0x6F for testnet)
        var versionedPayload = Data()
        versionedPayload.append(testnet ? 0x6F : 0x00)
        versionedPayload.append(ripemd160Hash)

        // Double SHA256 for checksum
        let checksum1 = Data(SHA256.hash(data: versionedPayload))
        let checksum2 = Data(SHA256.hash(data: checksum1))
        let checksum = checksum2.prefix(4)

        // Append checksum
        versionedPayload.append(checksum)

        // Base58 encode
        return Base58.base58Encode(Array(versionedPayload))
    }

    /// Generate Bitcoin SegWit address (P2WPKH - Bech32)
    public func generateBitcoinSegWitAddress(from privateKey: Data, testnet: Bool = false) throws -> String {
        // Get compressed public key
        let compressedPubKey = try derivePublicKey(from: privateKey)

        // SHA256 + RIPEMD160 (Hash160)
        let sha256Hash = Data(SHA256.hash(data: compressedPubKey))
        let hash160 = ripemd160(sha256Hash)

        // Bech32 encode
        let hrp = testnet ? "tb" : "bc"
        return try bech32Encode(hrp: hrp, witnessVersion: 0, program: hash160)
    }

    /// RIPEMD-160 hash implementation
    private func ripemd160(_ data: Data) -> Data {
        // Using CryptoSwift's RIPEMD160
        return Data(RIPEMD160.hash(data: data.bytes))
    }

    /// Bech32 encoding for SegWit addresses
    private func bech32Encode(hrp: String, witnessVersion: Int, program: Data) throws -> String {
        let charset = "qpzry9x8gf2tvdw0s3jn54khce6mua7l"

        // Convert to 5-bit groups
        var values: [UInt8] = [UInt8(witnessVersion)]
        values.append(contentsOf: convertBits(data: Array(program), fromBits: 8, toBits: 5, pad: true))

        // Create checksum
        let checksum = bech32Checksum(hrp: hrp, values: values)
        values.append(contentsOf: checksum)

        // Encode
        var result = hrp + "1"
        for value in values {
            result.append(charset[charset.index(charset.startIndex, offsetBy: Int(value))])
        }

        return result
    }

    private func convertBits(data: [UInt8], fromBits: Int, toBits: Int, pad: Bool) -> [UInt8] {
        var acc = 0
        var bits = 0
        var result: [UInt8] = []
        let maxv = (1 << toBits) - 1

        for value in data {
            acc = (acc << fromBits) | Int(value)
            bits += fromBits
            while bits >= toBits {
                bits -= toBits
                result.append(UInt8((acc >> bits) & maxv))
            }
        }

        if pad && bits > 0 {
            result.append(UInt8((acc << (toBits - bits)) & maxv))
        }

        return result
    }

    private func bech32Checksum(hrp: String, values: [UInt8]) -> [UInt8] {
        let generator: [UInt32] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3]

        func polymod(_ values: [UInt8]) -> UInt32 {
            var chk: UInt32 = 1
            for value in values {
                let top = chk >> 25
                chk = (chk & 0x1ffffff) << 5 ^ UInt32(value)
                for i in 0..<5 {
                    if ((top >> i) & 1) != 0 {
                        chk ^= generator[i]
                    }
                }
            }
            return chk
        }

        func hrpExpand(_ hrp: String) -> [UInt8] {
            var result: [UInt8] = []
            for char in hrp {
                result.append(UInt8(char.asciiValue! >> 5))
            }
            result.append(0)
            for char in hrp {
                result.append(UInt8(char.asciiValue! & 31))
            }
            return result
        }

        var combined = hrpExpand(hrp)
        combined.append(contentsOf: values)
        combined.append(contentsOf: [0, 0, 0, 0, 0, 0])

        let mod = polymod(combined) ^ 1
        var checksum: [UInt8] = []
        for i in 0..<6 {
            checksum.append(UInt8((mod >> (5 * (5 - i))) & 31))
        }

        return checksum
    }

    // MARK: - Solana Address (Ed25519)

    /// Generate Solana address from Ed25519 seed (32 bytes)
    public func generateSolanaAddress(from seed: Data) throws -> String {
        guard seed.count == 32 else {
            throw CryptoError.invalidSeedLength
        }

        // Generate Ed25519 key pair from seed
        let privateKey = try Curve25519.Signing.PrivateKey(rawRepresentation: seed)
        let publicKey = privateKey.publicKey.rawRepresentation

        // Solana address is Base58 encoded public key
        return Base58.base58Encode(Array(publicKey))
    }

    // MARK: - BIP32 Key Addition (for HD derivation)

    /// Add two 32-byte private keys mod secp256k1 order
    public func addPrivateKeys(_ a: Data, _ b: Data) throws -> Data {
        guard a.count == 32, b.count == 32 else {
            throw CryptoError.invalidPrivateKeyLength
        }

        // secp256k1 curve order
        let curveOrder: [UInt8] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
            0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B,
            0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41
        ]

        // Add with carry
        var result = [UInt8](repeating: 0, count: 32)
        var carry: UInt16 = 0

        let aBytes = Array(a)
        let bBytes = Array(b)

        for i in (0..<32).reversed() {
            let sum = UInt16(aBytes[i]) + UInt16(bBytes[i]) + carry
            result[i] = UInt8(sum & 0xFF)
            carry = sum >> 8
        }

        // Reduce modulo curve order if needed
        if carry > 0 || compareBytes(result, curveOrder) >= 0 {
            result = subtractBytes(result, curveOrder)
        }

        return Data(result)
    }

    private func compareBytes(_ a: [UInt8], _ b: [UInt8]) -> Int {
        for i in 0..<min(a.count, b.count) {
            if a[i] < b[i] { return -1 }
            if a[i] > b[i] { return 1 }
        }
        return 0
    }

    private func subtractBytes(_ a: [UInt8], _ b: [UInt8]) -> [UInt8] {
        var result = [UInt8](repeating: 0, count: 32)
        var borrow: Int16 = 0

        for i in (0..<32).reversed() {
            let diff = Int16(a[i]) - Int16(b[i]) - borrow
            if diff < 0 {
                result[i] = UInt8((diff + 256) & 0xFF)
                borrow = 1
            } else {
                result[i] = UInt8(diff & 0xFF)
                borrow = 0
            }
        }

        return result
    }
}

// MARK: - Errors

public enum CryptoError: LocalizedError {
    case invalidPrivateKeyLength
    case invalidPublicKeyLength
    case invalidSeedLength
    case keyDerivationFailed
    case addressGenerationFailed
    case checksumMismatch

    public var errorDescription: String? {
        switch self {
        case .invalidPrivateKeyLength:
            return "Private key must be 32 bytes"
        case .invalidPublicKeyLength:
            return "Invalid public key length"
        case .invalidSeedLength:
            return "Seed must be 32 bytes"
        case .keyDerivationFailed:
            return "Key derivation failed"
        case .addressGenerationFailed:
            return "Address generation failed"
        case .checksumMismatch:
            return "Checksum verification failed"
        }
    }
}

// MARK: - RIPEMD160 Implementation

/// RIPEMD-160 hash function
struct RIPEMD160 {
    static func hash(data: [UInt8]) -> [UInt8] {
        var h0: UInt32 = 0x67452301
        var h1: UInt32 = 0xEFCDAB89
        var h2: UInt32 = 0x98BADCFE
        var h3: UInt32 = 0x10325476
        var h4: UInt32 = 0xC3D2E1F0

        // Pre-processing: adding padding bits
        var message = data
        let originalLength = UInt64(data.count * 8)

        message.append(0x80)
        while (message.count % 64) != 56 {
            message.append(0x00)
        }

        // Append length in bits as 64-bit little-endian
        for i in 0..<8 {
            message.append(UInt8((originalLength >> (i * 8)) & 0xFF))
        }

        // Process each 512-bit block
        for chunkStart in stride(from: 0, to: message.count, by: 64) {
            var X = [UInt32](repeating: 0, count: 16)
            for i in 0..<16 {
                let offset = chunkStart + i * 4
                X[i] = UInt32(message[offset]) |
                       (UInt32(message[offset + 1]) << 8) |
                       (UInt32(message[offset + 2]) << 16) |
                       (UInt32(message[offset + 3]) << 24)
            }

            var A1 = h0, B1 = h1, C1 = h2, D1 = h3, E1 = h4
            var A2 = h0, B2 = h1, C2 = h2, D2 = h3, E2 = h4

            // Round functions
            func f(_ j: Int, _ x: UInt32, _ y: UInt32, _ z: UInt32) -> UInt32 {
                switch j {
                case 0..<16: return x ^ y ^ z
                case 16..<32: return (x & y) | (~x & z)
                case 32..<48: return (x | ~y) ^ z
                case 48..<64: return (x & z) | (y & ~z)
                default: return x ^ (y | ~z)
                }
            }

            let K1: [UInt32] = [0x00000000, 0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xA953FD4E]
            let K2: [UInt32] = [0x50A28BE6, 0x5C4DD124, 0x6D703EF3, 0x7A6D76E9, 0x00000000]

            let r1: [Int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                            7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8,
                            3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12,
                            1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
                            4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13]

            let r2: [Int] = [5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12,
                            6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2,
                            15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13,
                            8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10, 14,
                            12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11]

            let s1: [Int] = [11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8,
                            7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12,
                            11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5,
                            11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12,
                            9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6]

            let s2: [Int] = [8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6,
                            9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11,
                            9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5,
                            15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8,
                            8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11]

            for j in 0..<80 {
                let round = j / 16

                let T1 = A1 &+ f(j, B1, C1, D1) &+ X[r1[j]] &+ K1[round]
                let rotated1 = (T1 << s1[j]) | (T1 >> (32 - s1[j]))
                let newA1 = E1 &+ rotated1
                A1 = E1; E1 = D1; D1 = (C1 << 10) | (C1 >> 22); C1 = B1; B1 = newA1

                let T2 = A2 &+ f(79 - j, B2, C2, D2) &+ X[r2[j]] &+ K2[round]
                let rotated2 = (T2 << s2[j]) | (T2 >> (32 - s2[j]))
                let newA2 = E2 &+ rotated2
                A2 = E2; E2 = D2; D2 = (C2 << 10) | (C2 >> 22); C2 = B2; B2 = newA2
            }

            let T = h1 &+ C1 &+ D2
            h1 = h2 &+ D1 &+ E2
            h2 = h3 &+ E1 &+ A2
            h3 = h4 &+ A1 &+ B2
            h4 = h0 &+ B1 &+ C2
            h0 = T
        }

        // Produce final hash value (little-endian)
        var result = [UInt8](repeating: 0, count: 20)
        for (i, h) in [h0, h1, h2, h3, h4].enumerated() {
            result[i * 4] = UInt8(h & 0xFF)
            result[i * 4 + 1] = UInt8((h >> 8) & 0xFF)
            result[i * 4 + 2] = UInt8((h >> 16) & 0xFF)
            result[i * 4 + 3] = UInt8((h >> 24) & 0xFF)
        }

        return result
    }
}
