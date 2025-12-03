// KeychainManager.swift
// ËTRID Wallet - Secure Keychain Storage

import Foundation
import Security

/// Secure keychain storage manager for sensitive wallet data
public final class KeychainManager: Sendable {

    enum KeychainError: Error {
        case duplicateItem
        case itemNotFound
        case unexpectedStatus(OSStatus)
        case invalidData
    }

    private let service = "com.etrid.wallet"

    // MARK: - Save Data

    func save(_ data: Data, forKey key: String) throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly
        ]

        // Delete any existing item
        SecItemDelete(query as CFDictionary)

        let status = SecItemAdd(query as CFDictionary, nil)

        guard status == errSecSuccess else {
            throw KeychainError.unexpectedStatus(status)
        }
    }

    func save(_ string: String, forKey key: String) throws {
        guard let data = string.data(using: .utf8) else {
            throw KeychainError.invalidData
        }
        try save(data, forKey: key)
    }

    // MARK: - Read Data

    func read(forKey key: String) throws -> Data {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        guard status == errSecSuccess else {
            if status == errSecItemNotFound {
                throw KeychainError.itemNotFound
            }
            throw KeychainError.unexpectedStatus(status)
        }

        guard let data = result as? Data else {
            throw KeychainError.invalidData
        }

        return data
    }

    func readString(forKey key: String) throws -> String {
        let data = try read(forKey: key)
        guard let string = String(data: data, encoding: .utf8) else {
            throw KeychainError.invalidData
        }
        return string
    }

    // MARK: - Delete Data

    func delete(forKey key: String) throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: key
        ]

        let status = SecItemDelete(query as CFDictionary)

        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.unexpectedStatus(status)
        }
    }

    // MARK: - Check Existence

    func exists(forKey key: String) -> Bool {
        do {
            _ = try read(forKey: key)
            return true
        } catch {
            return false
        }
    }

    // MARK: - Alias Methods (for WalletCore compatibility)

    func store(_ data: Data, for key: String) throws {
        try save(data, forKey: key)
    }

    func retrieve(for key: String) throws -> Data {
        try read(forKey: key)
    }

    func delete(for key: String) throws {
        try delete(forKey: key)
    }

    // MARK: - Clear All

    func clearAll() throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service
        ]

        let status = SecItemDelete(query as CFDictionary)

        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.unexpectedStatus(status)
        }
    }

    // MARK: - Convenience Keys

    struct Keys {
        static let mnemonic = "wallet.mnemonic"
        static let privateKey = "wallet.privateKey"
        static let passcode = "wallet.passcode"
        static let biometricEnabled = "wallet.biometric"
    }

    // MARK: - Wallet-Specific Methods

    func saveMnemonic(_ mnemonic: String) throws {
        try save(mnemonic, forKey: Keys.mnemonic)
    }

    func loadMnemonic() throws -> String {
        try readString(forKey: Keys.mnemonic)
    }

    func savePrivateKey(_ key: Data) throws {
        try save(key, forKey: Keys.privateKey)
    }

    func loadPrivateKey() throws -> Data {
        try read(forKey: Keys.privateKey)
    }

    func savePasscodeHash(_ hash: String) throws {
        try save(hash, forKey: Keys.passcode)
    }

    func loadPasscodeHash() throws -> String {
        try readString(forKey: Keys.passcode)
    }

    func setBiometricEnabled(_ enabled: Bool) throws {
        try save(enabled ? "true" : "false", forKey: Keys.biometricEnabled)
    }

    func isBiometricEnabled() -> Bool {
        (try? readString(forKey: Keys.biometricEnabled)) == "true"
    }
}
