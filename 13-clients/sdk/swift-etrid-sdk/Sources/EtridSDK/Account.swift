/// Account management and cryptographic operations

import Foundation
import Crypto

/// Ëtrid account
public struct Account {
    private let privateKey: Curve25519.Signing.PrivateKey

    private init(privateKey: Curve25519.Signing.PrivateKey) {
        self.privateKey = privateKey
    }

    /// Create account from mnemonic phrase
    /// - Parameter mnemonic: 12 or 24 word mnemonic phrase
    /// - Returns: New Account instance
    public static func fromMnemonic(_ mnemonic: String) throws -> Account {
        // Simplified implementation - real version would use BIP39
        let privateKey = Curve25519.Signing.PrivateKey()
        return Account(privateKey: privateKey)
    }

    /// Generate a new random account
    /// - Returns: New Account instance
    public static func generate() -> Account {
        let privateKey = Curve25519.Signing.PrivateKey()
        return Account(privateKey: privateKey)
    }

    /// Get the account address (SS58 format)
    public var address: String {
        // Placeholder - real implementation would encode properly
        return publicKey
    }

    /// Get the public key (hex format)
    public var publicKey: String {
        privateKey.publicKey.rawRepresentation.map { String(format: "%02x", $0) }.joined()
    }

    /// Sign a message
    /// - Parameter message: Message to sign
    /// - Returns: Signature data
    public func sign(_ message: Data) throws -> Data {
        try privateKey.signature(for: message)
    }
}
