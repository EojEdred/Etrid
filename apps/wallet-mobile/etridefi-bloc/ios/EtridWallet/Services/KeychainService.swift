//
//  KeychainService.swift
//  EtridWallet
//
//  Secure storage service using iOS Keychain
//

import Foundation
import Security

class KeychainService {
    static let shared = KeychainService()

    private init() {}

    // MARK: - Save
    func save(key: String, data: Data) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: key,
            kSecValueData as String: data
        ]

        SecItemDelete(query as CFDictionary)

        let status = SecItemAdd(query as CFDictionary, nil)
        return status == errSecSuccess
    }

    func save(key: String, string: String) -> Bool {
        guard let data = string.data(using: .utf8) else { return false }
        return save(key: key, data: data)
    }

    // MARK: - Load
    func load(key: String) -> Data? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        return status == errSecSuccess ? result as? Data : nil
    }

    func loadString(key: String) -> String? {
        guard let data = load(key: key) else { return nil }
        return String(data: data, encoding: .utf8)
    }

    // MARK: - Delete
    func delete(key: String) -> Bool {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: key
        ]

        let status = SecItemDelete(query as CFDictionary)
        return status == errSecSuccess
    }

    // MARK: - Convenience Methods
    func saveAuthToken(_ token: String) -> Bool {
        return save(key: "auth_token", string: token)
    }

    func getAuthToken() -> String? {
        return loadString(key: "auth_token")
    }

    func deleteAuthToken() -> Bool {
        return delete(key: "auth_token")
    }

    func savePrivateKey(_ key: String, for address: String) -> Bool {
        return save(key: "private_key_\(address)", string: key)
    }

    func getPrivateKey(for address: String) -> String? {
        return loadString(key: "private_key_\(address)")
    }

    func saveSeedPhrase(_ phrase: String) -> Bool {
        return save(key: "seed_phrase", string: phrase)
    }

    func getSeedPhrase() -> String? {
        return loadString(key: "seed_phrase")
    }

    func deleteSeedPhrase() -> Bool {
        return delete(key: "seed_phrase")
    }
}
