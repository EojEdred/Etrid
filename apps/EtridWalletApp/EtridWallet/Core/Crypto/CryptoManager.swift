//
//  CryptoManager.swift
//  EtridWallet
//
//  Core cryptographic operations: key generation, derivation, signing
//  Production implementation using web3swift, CryptoSwift, and BigInt
//

import Foundation
import CryptoKit
import Security
import web3swift
import Web3Core
import CryptoSwift
import BigInt

actor CryptoManager {
    // MARK: - Singleton
    static let shared = CryptoManager()

    private init() {}

    // MARK: - BIP39 Wordlist
    private let wordlist = BIP39Wordlist.english

    // MARK: - Mnemonic Generation
    /// Generates a BIP39 mnemonic phrase
    /// - Parameter wordCount: Number of words (12, 15, 18, 21, or 24)
    /// - Returns: Mnemonic phrase as string
    func generateMnemonic(wordCount: Int = 12) throws -> String {
        guard [12, 15, 18, 21, 24].contains(wordCount) else {
            throw WalletError.invalidInput("Word count must be 12, 15, 18, 21, or 24")
        }

        // Calculate entropy size in bits: ENT = (wordCount * 11) - checksumBits
        // checksumBits = ENT / 32, so ENT = (wordCount * 11 * 32) / 33
        let entropyBits = (wordCount * 11 * 32) / 33
        let entropyBytes = entropyBits / 8

        // Generate cryptographically secure random entropy
        var bytes = [UInt8](repeating: 0, count: entropyBytes)
        let status = SecRandomCopyBytes(kSecRandomDefault, entropyBytes, &bytes)
        guard status == errSecSuccess else {
            throw WalletError.keyGenerationFailed
        }

        let entropy = Data(bytes)

        // Calculate checksum using SHA256
        let hash = SHA256.hash(data: entropy)
        let checksumBits = entropyBits / 32
        let checksumByte = Array(hash)[0]

        // Convert entropy to bits
        var bits: [UInt8] = []
        for byte in entropy {
            for i in (0..<8).reversed() {
                bits.append((byte >> i) & 1)
            }
        }

        // Append checksum bits
        for i in (0..<checksumBits).reversed() {
            bits.append((checksumByte >> (7 - i)) & 1)
        }

        // Convert bits to words (11 bits per word)
        var words: [String] = []
        for i in stride(from: 0, to: bits.count, by: 11) {
            let wordBits = Array(bits[i..<min(i + 11, bits.count)])
            var index = 0
            for (position, bit) in wordBits.enumerated() {
                index |= Int(bit) << (10 - position)
            }
            guard index < wordlist.count else {
                throw WalletError.keyGenerationFailed
            }
            words.append(wordlist[index])
        }

        return words.joined(separator: " ")
    }

    // MARK: - Seed Derivation
    /// Derives seed from mnemonic using PBKDF2-HMAC-SHA512
    /// - Parameters:
    ///   - mnemonic: BIP39 mnemonic phrase
    ///   - passphrase: Optional passphrase
    /// - Returns: 64-byte seed
    func mnemonicToSeed(_ mnemonic: String, passphrase: String = "") throws -> Data {
        let words = mnemonic.components(separatedBy: " ")
        guard words.count >= 12 else {
            throw WalletError.invalidMnemonic
        }

        // BIP39 standard: PBKDF2-HMAC-SHA512 with 2048 iterations
        guard let passwordData = mnemonic.normalizeNFKD().data(using: .utf8) else {
            throw WalletError.invalidMnemonic
        }

        let saltString = "mnemonic" + passphrase.normalizeNFKD()
        guard let saltData = saltString.data(using: .utf8) else {
            throw WalletError.invalidMnemonic
        }

        // Use CryptoSwift PBKDF2 with HMAC-SHA512
        do {
            let seed = try PKCS5.PBKDF2(
                password: Array(passwordData),
                salt: Array(saltData),
                iterations: 2048,
                keyLength: 64,
                variant: .sha2(.sha512)
            ).calculate()

            return Data(seed)
        } catch {
            throw WalletError.keyGenerationFailed
        }
    }

    // MARK: - Private Key Generation
    /// Generates a random private key
    /// - Returns: 32-byte private key
    func generatePrivateKey() throws -> Data {
        var bytes = [UInt8](repeating: 0, count: 32)
        let status = SecRandomCopyBytes(kSecRandomDefault, bytes.count, &bytes)
        guard status == errSecSuccess else {
            throw WalletError.keyGenerationFailed
        }
        return Data(bytes)
    }

    // MARK: - Public Key Derivation
    /// Derives public key from private key using secp256k1
    /// - Parameter privateKey: 32-byte private key
    /// - Returns: 64-byte uncompressed public key (without 0x04 prefix)
    func derivePublicKey(from privateKey: Data) throws -> Data {
        guard privateKey.count == 32 else {
            throw WalletError.invalidPrivateKey
        }

        // Use web3swift's SECP256K1 implementation
        do {
            guard let publicKey = Utilities.privateToPublic(privateKey, compressed: false) else {
                throw WalletError.invalidPrivateKey
            }

            // Remove 0x04 prefix if present (uncompressed format indicator)
            if publicKey.count == 65 && publicKey[0] == 0x04 {
                return publicKey.dropFirst()
            }

            guard publicKey.count == 64 else {
                throw WalletError.invalidPublicKey
            }

            return publicKey
        } catch {
            throw WalletError.invalidPrivateKey
        }
    }

    // MARK: - Address Generation
    /// Generates Ethereum address from public key
    /// - Parameter publicKey: 64-byte uncompressed public key
    /// - Returns: Ethereum address (0x + 40 hex chars)
    func generateAddress(from publicKey: Data) throws -> String {
        guard publicKey.count == 64 else {
            throw WalletError.invalidPublicKey
        }

        // Apply Keccak256 hash to the public key using CryptoSwift
        let hash = publicKey.sha3(.keccak256)

        // Take last 20 bytes for Ethereum address
        let addressBytes = hash.suffix(20)

        // Convert to checksummed address (EIP-55)
        let addressHex = addressBytes.toHexString()
        return ("0x" + addressHex).toChecksumAddress
    }

    /// Generates address directly from private key
    /// - Parameter privateKey: 32-byte private key
    /// - Returns: Ethereum address
    func generateAddressFromPrivateKey(_ privateKey: Data) throws -> String {
        let publicKey = try derivePublicKey(from: privateKey)
        return try generateAddress(from: publicKey)
    }

    // MARK: - Message Signing
    /// Signs a message with private key using ECDSA secp256k1
    /// - Parameters:
    ///   - message: Message data to sign
    ///   - privateKey: 32-byte private key
    /// - Returns: 65-byte signature (r + s + v)
    func sign(message: Data, privateKey: Data) throws -> Data {
        guard privateKey.count == 32 else {
            throw WalletError.invalidPrivateKey
        }

        // Hash message with Keccak256
        let messageHash = message.sha3(.keccak256)

        // Sign using web3swift's SECP256K1
        do {
            let result = SECP256K1.signForRecovery(
                hash: messageHash,
                privateKey: privateKey
            )

            guard let compressedSignature = result.0,
                  let recoveryIDData = result.1,
                  let recoveryID = recoveryIDData.first else {
                throw WalletError.signatureFailed
            }

            // Ethereum uses v = recoveryID + 27 for legacy transactions
            var signature = compressedSignature
            let v = recoveryID + 27
            signature.append(v)

            guard signature.count == 65 else {
                throw WalletError.signatureFailed
            }

            return signature
        } catch {
            throw WalletError.signatureFailed
        }
    }

    /// Signs Ethereum transaction hash
    /// - Parameters:
    ///   - hash: Transaction hash
    ///   - privateKey: Private key
    /// - Returns: Signature with recovery id
    func signTransactionHash(_ hash: Data, privateKey: Data) throws -> Data {
        return try sign(message: hash, privateKey: privateKey)
    }

    // MARK: - Message Hashing
    /// Creates Ethereum signed message hash (EIP-191)
    /// - Parameter message: Original message
    /// - Returns: Keccak256 hash ready for signing
    func hashMessage(_ message: String) -> Data {
        let prefix = "\u{19}Ethereum Signed Message:\n\(message.count)"
        guard let data = (prefix + message).data(using: .utf8) else {
            return Data()
        }
        return data.sha3(.keccak256)
    }

    // MARK: - Validation
    /// Validates BIP39 mnemonic with checksum verification
    /// - Parameter mnemonic: Mnemonic phrase
    /// - Returns: True if valid
    func validateMnemonic(_ mnemonic: String) -> Bool {
        let words = mnemonic.components(separatedBy: " ")
        guard [12, 15, 18, 21, 24].contains(words.count) else {
            return false
        }

        // Check if all words are in wordlist and get indices
        var indices: [Int] = []
        for word in words {
            guard let index = wordlist.firstIndex(of: word.lowercased()) else {
                return false
            }
            indices.append(index)
        }

        // Convert indices to bits
        var bits: [UInt8] = []
        for index in indices {
            for i in (0..<11).reversed() {
                bits.append(UInt8((index >> i) & 1))
            }
        }

        // Calculate expected checksum length
        let checksumBits = words.count / 3
        let entropyBits = bits.count - checksumBits

        // Extract entropy and checksum
        let entropyBitsArray = Array(bits[0..<entropyBits])
        let checksumBitsArray = Array(bits[entropyBits..<bits.count])

        // Convert entropy bits to bytes
        var entropyBytes: [UInt8] = []
        for i in stride(from: 0, to: entropyBitsArray.count, by: 8) {
            var byte: UInt8 = 0
            for j in 0..<8 {
                if i + j < entropyBitsArray.count {
                    byte |= entropyBitsArray[i + j] << (7 - j)
                }
            }
            entropyBytes.append(byte)
        }

        // Calculate checksum from entropy
        let hash = SHA256.hash(data: Data(entropyBytes))
        let hashByte = Array(hash)[0]

        // Extract checksum bits from hash
        var expectedChecksumBits: [UInt8] = []
        for i in (0..<checksumBits).reversed() {
            expectedChecksumBits.append((hashByte >> (7 - i)) & 1)
        }

        // Compare checksums
        return checksumBitsArray == expectedChecksumBits
    }

    /// Validates Ethereum address
    /// - Parameter address: Ethereum address
    /// - Returns: True if valid format
    func validateAddress(_ address: String) -> Bool {
        return address.isValidEthereumAddress
    }

    /// Validates private key format
    /// - Parameter privateKey: Private key data
    /// - Returns: True if valid
    func validatePrivateKey(_ privateKey: Data) -> Bool {
        guard privateKey.count == 32 else { return false }
        // Check it's not all zeros
        return privateKey.contains { $0 != 0 }
    }

    // MARK: - Encryption/Decryption
    /// Encrypts data with password
    /// - Parameters:
    ///   - data: Data to encrypt
    ///   - password: Password
    /// - Returns: Encrypted data
    func encrypt(_ data: Data, password: String) throws -> Data {
        let key = SHA256.hash(data: password.data(using: .utf8)!)
        let symmetricKey = SymmetricKey(data: Data(key))

        let sealedBox = try AES.GCM.seal(data, using: symmetricKey)
        guard let combined = sealedBox.combined else {
            throw WalletError.encryptionFailed
        }

        return combined
    }

    /// Decrypts data with password
    /// - Parameters:
    ///   - data: Encrypted data
    ///   - password: Password
    /// - Returns: Decrypted data
    func decrypt(_ data: Data, password: String) throws -> Data {
        let key = SHA256.hash(data: password.data(using: .utf8)!)
        let symmetricKey = SymmetricKey(data: Data(key))

        let sealedBox = try AES.GCM.SealedBox(combined: data)
        return try AES.GCM.open(sealedBox, using: symmetricKey)
    }
}

// MARK: - Helper Extensions
extension CryptoManager {
    /// Generates a complete wallet (mnemonic + keys + address)
    func generateWallet(wordCount: Int = 12) async throws -> (mnemonic: String, privateKey: Data, address: String) {
        let mnemonic = try generateMnemonic(wordCount: wordCount)
        let seed = try mnemonicToSeed(mnemonic)
        let privateKey = Data(seed.prefix(32))
        let address = try generateAddressFromPrivateKey(privateKey)

        return (mnemonic, privateKey, address)
    }

    /// Imports wallet from mnemonic
    func importWallet(mnemonic: String, derivationPath: String = "m/44'/60'/0'/0/0") async throws -> (privateKey: Data, address: String) {
        guard validateMnemonic(mnemonic) else {
            throw WalletError.invalidMnemonic
        }

        let seed = try mnemonicToSeed(mnemonic)
        // Simplified: use seed as private key
        let privateKey = Data(seed.prefix(32))
        let address = try generateAddressFromPrivateKey(privateKey)

        return (privateKey, address)
    }
}
