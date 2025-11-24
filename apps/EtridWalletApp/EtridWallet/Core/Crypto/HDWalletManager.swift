//
//  HDWalletManager.swift
//  EtridWallet
//
//  Hierarchical Deterministic (HD) wallet implementation
//  Supports BIP32/BIP44 key derivation
//  Production implementation using web3swift and CryptoSwift
//

import Foundation
import CryptoKit
import web3swift
import Web3Core
import CryptoSwift
import BigInt

actor HDWalletManager {
    // MARK: - Singleton
    static let shared = HDWalletManager()

    private let crypto = CryptoManager.shared

    private init() {}

    // MARK: - BIP44 Derivation Paths
    struct DerivationPath {
        static let ethereum = "m/44'/60'/0'/0"
        static let ethereumLedger = "m/44'/60'/0'"
        static let ethereumClassic = "m/44'/61'/0'/0"
        static let bitcoin = "m/44'/0'/0'/0"
        static let binanceSmartChain = "m/44'/60'/0'/0" // Same as Ethereum
        static let polygon = "m/44'/60'/0'/0" // Same as Ethereum

        static func ethereum(account: Int = 0, index: Int = 0) -> String {
            "m/44'/60'/\(account)'/0/\(index)"
        }

        static func custom(coinType: Int, account: Int = 0, change: Int = 0, index: Int = 0) -> String {
            "m/44'/\(coinType)'/\(account)'/\(change)/\(index)"
        }
    }

    // MARK: - HD Wallet Creation
    /// Creates an HD wallet from mnemonic
    /// - Parameters:
    ///   - mnemonic: BIP39 mnemonic phrase
    ///   - passphrase: Optional passphrase
    /// - Returns: Master seed
    func createHDWallet(mnemonic: String, passphrase: String = "") async throws -> Data {
        return try await crypto.mnemonicToSeed(mnemonic, passphrase: passphrase)
    }

    // MARK: - Key Derivation
    /// Derives child key from parent using BIP32
    /// - Parameters:
    ///   - parentKey: Parent private key
    ///   - parentChainCode: Parent chain code
    ///   - index: Child index
    ///   - hardened: Whether to use hardened derivation
    /// - Returns: Tuple of (child key, child chain code)
    func deriveChildKey(
        from parentKey: Data,
        parentChainCode: Data,
        index: UInt32,
        hardened: Bool = false
    ) throws -> (key: Data, chainCode: Data) {
        guard parentKey.count == 32, parentChainCode.count == 32 else {
            throw WalletError.invalidPrivateKey
        }

        let hardenedOffset: UInt32 = 0x80000000
        let childIndex = hardened ? (index | hardenedOffset) : index

        var data = Data()

        if hardened {
            // Hardened derivation: HMAC-SHA512(chainCode, 0x00 || parentKey || index)
            data.append(0x00)
            data.append(parentKey)
        } else {
            // Normal derivation: HMAC-SHA512(chainCode, publicKey || index)
            guard let publicKey = Utilities.privateToPublic(parentKey, compressed: true) else {
                throw WalletError.invalidPrivateKey
            }
            data.append(publicKey)
        }

        // Append index as big-endian
        data.append(contentsOf: withUnsafeBytes(of: childIndex.bigEndian) { Array($0) })

        // HMAC-SHA512 using parent chain code as key
        let hmac = try HMAC(key: Array(parentChainCode), variant: .sha2(.sha512)).authenticate(Array(data))
        let hmacData = Data(hmac)

        // Split into left and right halves
        let leftHalf = hmacData.prefix(32)
        let rightHalf = hmacData.suffix(32)

        // Child key = (leftHalf + parentKey) mod n (secp256k1 order)
        let secp256k1Order = BigUInt("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", radix: 16)!
        let leftBigInt = BigUInt(leftHalf)
        let parentBigInt = BigUInt(parentKey)

        let childKeyBigInt = (leftBigInt + parentBigInt) % secp256k1Order

        // Convert back to Data with proper padding
        var childKey = childKeyBigInt.serialize()
        while childKey.count < 32 {
            childKey.insert(0, at: 0)
        }

        // Chain code is the right half
        let childChainCode = rightHalf

        return (Data(childKey), childChainCode)
    }

    /// Derives key at specific BIP44 path
    /// - Parameters:
    ///   - masterSeed: Master seed from mnemonic
    ///   - path: BIP44 derivation path (e.g., "m/44'/60'/0'/0/0")
    /// - Returns: Derived private key
    func deriveKey(from masterSeed: Data, path: String) throws -> Data {
        guard masterSeed.count == 64 else {
            throw WalletError.invalidPrivateKey
        }

        // Parse path
        let components = path.components(separatedBy: "/").dropFirst() // Remove 'm'

        // Derive master key and chain code from seed
        // BIP32: HMAC-SHA512(key="Bitcoin seed", data=seed)
        let hmacKey = "Bitcoin seed".data(using: .utf8)!
        let hmac = try HMAC(key: Array(hmacKey), variant: .sha2(.sha512)).authenticate(Array(masterSeed))
        let hmacData = Data(hmac)

        var currentKey = hmacData.prefix(32)
        var currentChainCode = hmacData.suffix(32)

        // Derive through path
        for component in components {
            let isHardened = component.hasSuffix("'")
            let indexString = isHardened ? String(component.dropLast()) : component

            guard let index = UInt32(indexString) else {
                throw WalletError.invalidDerivationPath
            }

            let (childKey, childChainCode) = try deriveChildKey(
                from: currentKey,
                parentChainCode: currentChainCode,
                index: index,
                hardened: isHardened
            )

            currentKey = childKey
            currentChainCode = childChainCode
        }

        return currentKey
    }

    // MARK: - Address Derivation
    /// Derives multiple addresses from HD wallet
    /// - Parameters:
    ///   - mnemonic: BIP39 mnemonic
    ///   - count: Number of addresses to derive
    ///   - startIndex: Starting index
    ///   - basePath: Base derivation path
    /// - Returns: Array of (address, privateKey, index) tuples
    func deriveAddresses(
        from mnemonic: String,
        count: Int = 1,
        startIndex: Int = 0,
        basePath: String = DerivationPath.ethereum
    ) async throws -> [(address: String, privateKey: Data, index: Int)] {
        let seed = try await crypto.mnemonicToSeed(mnemonic)

        var results: [(String, Data, Int)] = []

        for i in startIndex..<(startIndex + count) {
            let path = "\(basePath)/\(i)"
            let privateKey = try deriveKey(from: seed, path: path)
            let address = try await crypto.generateAddress(from: privateKey)

            results.append((address, privateKey, i))
        }

        return results
    }

    /// Derives single address at index
    /// - Parameters:
    ///   - mnemonic: BIP39 mnemonic
    ///   - index: Address index
    ///   - basePath: Base derivation path
    /// - Returns: Tuple of (address, privateKey)
    func deriveAddress(
        from mnemonic: String,
        at index: Int = 0,
        basePath: String = DerivationPath.ethereum
    ) async throws -> (address: String, privateKey: Data) {
        let seed = try await crypto.mnemonicToSeed(mnemonic)
        let path = "\(basePath)/\(index)"
        let privateKey = try deriveKey(from: seed, path: path)
        let address = try await crypto.generateAddress(from: privateKey)

        return (address, privateKey)
    }

    // MARK: - Account Management
    /// Creates multiple accounts from single mnemonic
    /// - Parameters:
    ///   - mnemonic: BIP39 mnemonic
    ///   - count: Number of accounts
    /// - Returns: Array of wallet accounts
    func createAccounts(from mnemonic: String, count: Int = 1) async throws -> [HDAccount] {
        let addresses = try await deriveAddresses(from: mnemonic, count: count)

        return addresses.map { address, privateKey, index in
            HDAccount(
                index: index,
                address: address,
                privateKey: privateKey,
                derivationPath: DerivationPath.ethereum(account: 0, index: index)
            )
        }
    }

    // MARK: - Extended Keys
    /// Derives extended public key (xpub)
    /// - Parameters:
    ///   - masterSeed: Master seed
    ///   - path: Derivation path
    /// - Returns: Extended public key (uncompressed, 64 bytes)
    func deriveExtendedPublicKey(from masterSeed: Data, path: String) async throws -> Data {
        let privateKey = try deriveKey(from: masterSeed, path: path)
        return try await crypto.derivePublicKey(from: privateKey)
    }

    /// Derives compressed public key from private key
    /// - Parameter privateKey: 32-byte private key
    /// - Returns: Compressed public key (33 bytes)
    func deriveCompressedPublicKey(from privateKey: Data) throws -> Data {
        guard let compressedKey = Utilities.privateToPublic(privateKey, compressed: true) else {
            throw WalletError.invalidPrivateKey
        }
        return compressedKey
    }

    // MARK: - Path Validation
    /// Validates BIP44 derivation path
    /// - Parameter path: Path to validate
    /// - Returns: True if valid
    func validatePath(_ path: String) -> Bool {
        let pattern = "^m(/[0-9]+'?)+$"
        guard let regex = try? NSRegularExpression(pattern: pattern) else {
            return false
        }

        let range = NSRange(location: 0, length: path.utf16.count)
        return regex.firstMatch(in: path, range: range) != nil
    }

    /// Parses derivation path into components
    /// - Parameter path: BIP44 path
    /// - Returns: Path components
    func parsePath(_ path: String) -> PathComponents? {
        guard validatePath(path) else { return nil }

        let components = path.components(separatedBy: "/").dropFirst()
        guard components.count >= 5 else { return nil }

        let parts = Array(components)

        let purpose = parsePathComponent(parts[0])
        let coinType = parsePathComponent(parts[1])
        let account = parsePathComponent(parts[2])
        let change = parsePathComponent(parts[3])
        let index = parsePathComponent(parts[4])

        return PathComponents(
            purpose: purpose.index,
            coinType: coinType.index,
            account: account.index,
            change: change.index,
            addressIndex: index.index,
            purposeHardened: purpose.hardened,
            coinTypeHardened: coinType.hardened,
            accountHardened: account.hardened
        )
    }

    private func parsePathComponent(_ component: String) -> (index: UInt32, hardened: Bool) {
        let isHardened = component.hasSuffix("'")
        let indexString = isHardened ? String(component.dropLast()) : component
        let index = UInt32(indexString) ?? 0
        return (index, isHardened)
    }
}

// MARK: - Supporting Types
struct HDAccount {
    let index: Int
    let address: String
    let privateKey: Data
    let derivationPath: String

    var formattedIndex: String {
        "Account #\(index + 1)"
    }
}

struct PathComponents {
    let purpose: UInt32
    let coinType: UInt32
    let account: UInt32
    let change: UInt32
    let addressIndex: UInt32
    let purposeHardened: Bool
    let coinTypeHardened: Bool
    let accountHardened: Bool

    var description: String {
        let p = purposeHardened ? "'" : ""
        let c = coinTypeHardened ? "'" : ""
        let a = accountHardened ? "'" : ""
        return "m/\(purpose)\(p)/\(coinType)\(c)/\(account)\(a)/\(change)/\(addressIndex)"
    }
}

// MARK: - Coin Types (BIP44)
extension HDWalletManager {
    enum CoinType {
        case bitcoin
        case testnet
        case litecoin
        case dogecoin
        case ethereum
        case ethereumClassic
        case cosmos
        case binance
        case polygon  // Uses Ethereum's coin type
        case avalanche  // Uses Ethereum's coin type

        var rawValue: UInt32 {
            switch self {
            case .bitcoin: return 0
            case .testnet: return 1
            case .litecoin: return 2
            case .dogecoin: return 3
            case .ethereum, .polygon, .avalanche: return 60  // EVM chains share coin type
            case .ethereumClassic: return 61
            case .cosmos: return 118
            case .binance: return 714
            }
        }

        var symbol: String {
            switch self {
            case .bitcoin: return "BTC"
            case .testnet: return "TEST"
            case .litecoin: return "LTC"
            case .dogecoin: return "DOGE"
            case .ethereum: return "ETH"
            case .ethereumClassic: return "ETC"
            case .cosmos: return "ATOM"
            case .binance: return "BNB"
            case .polygon: return "MATIC"
            case .avalanche: return "AVAX"
            }
        }
    }
}
