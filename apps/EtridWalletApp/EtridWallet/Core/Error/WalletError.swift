//
//  WalletError.swift
//  EtridWallet
//
//  Comprehensive error types for wallet operations
//

import Foundation

// MARK: - Wallet Error
enum WalletError: LocalizedError {
    // Authentication & Security
    case biometricsNotAvailable
    case biometricsNotEnrolled
    case authenticationFailed
    case securityError(String)
    case keychainError(String)

    // Crypto Operations
    case keyGenerationFailed
    case invalidPrivateKey
    case invalidPublicKey
    case invalidMnemonic
    case invalidDerivationPath
    case signatureFailed
    case encryptionFailed
    case decryptionFailed

    // Network & RPC
    case networkError(String)
    case rpcError(Int, String)
    case connectionFailed
    case timeout
    case invalidResponse
    case rateLimitExceeded

    // Transactions
    case insufficientBalance
    case insufficientGas
    case invalidAddress
    case invalidAmount
    case transactionFailed(String)
    case nonceTooLow
    case gasEstimationFailed
    case transactionNotFound

    // Storage
    case storageError(String)
    case dataCorrupted
    case fileNotFound
    case encodingFailed
    case decodingFailed

    // Validation
    case invalidInput(String)
    case validationFailed(String)
    case checksumMismatch

    // General
    case unknown(String)
    case notImplemented
    case unsupportedOperation

    var errorDescription: String? {
        switch self {
        // Authentication & Security
        case .biometricsNotAvailable:
            return "Biometric authentication is not available on this device"
        case .biometricsNotEnrolled:
            return "No biometric data enrolled. Please set up Face ID or Touch ID"
        case .authenticationFailed:
            return "Authentication failed. Please try again"
        case .securityError(let message):
            return "Security error: \(message)"
        case .keychainError(let message):
            return "Keychain error: \(message)"

        // Crypto Operations
        case .keyGenerationFailed:
            return "Failed to generate cryptographic keys"
        case .invalidPrivateKey:
            return "Invalid private key format"
        case .invalidPublicKey:
            return "Invalid public key format"
        case .invalidMnemonic:
            return "Invalid mnemonic phrase. Please check your seed words"
        case .invalidDerivationPath:
            return "Invalid BIP44 derivation path"
        case .signatureFailed:
            return "Failed to sign transaction"
        case .encryptionFailed:
            return "Failed to encrypt data"
        case .decryptionFailed:
            return "Failed to decrypt data"

        // Network & RPC
        case .networkError(let message):
            return "Network error: \(message)"
        case .rpcError(let code, let message):
            return "RPC error (\(code)): \(message)"
        case .connectionFailed:
            return "Failed to connect to network. Please check your internet connection"
        case .timeout:
            return "Request timed out. Please try again"
        case .invalidResponse:
            return "Invalid response from server"
        case .rateLimitExceeded:
            return "Rate limit exceeded. Please try again later"

        // Transactions
        case .insufficientBalance:
            return "Insufficient balance to complete this transaction"
        case .insufficientGas:
            return "Insufficient gas to complete this transaction"
        case .invalidAddress:
            return "Invalid address format"
        case .invalidAmount:
            return "Invalid amount"
        case .transactionFailed(let reason):
            return "Transaction failed: \(reason)"
        case .nonceTooLow:
            return "Nonce too low. Transaction already processed"
        case .gasEstimationFailed:
            return "Failed to estimate gas for transaction"
        case .transactionNotFound:
            return "Transaction not found"

        // Storage
        case .storageError(let message):
            return "Storage error: \(message)"
        case .dataCorrupted:
            return "Data is corrupted and cannot be read"
        case .fileNotFound:
            return "Required file not found"
        case .encodingFailed:
            return "Failed to encode data"
        case .decodingFailed:
            return "Failed to decode data"

        // Validation
        case .invalidInput(let field):
            return "Invalid input for: \(field)"
        case .validationFailed(let reason):
            return "Validation failed: \(reason)"
        case .checksumMismatch:
            return "Checksum mismatch. Address may be invalid"

        // General
        case .unknown(let message):
            return "Unknown error: \(message)"
        case .notImplemented:
            return "This feature is not yet implemented"
        case .unsupportedOperation:
            return "This operation is not supported"
        }
    }

    var recoverySuggestion: String? {
        switch self {
        case .biometricsNotAvailable, .biometricsNotEnrolled:
            return "Go to Settings to enable biometric authentication"
        case .authenticationFailed:
            return "Try authenticating again or use your passcode"
        case .invalidMnemonic:
            return "Check that you've entered all 12 or 24 words correctly"
        case .connectionFailed:
            return "Check your internet connection and try again"
        case .insufficientBalance:
            return "Add more funds to your wallet before proceeding"
        case .insufficientGas:
            return "Add more funds to cover gas fees"
        case .invalidAddress:
            return "Double-check the address and try again"
        case .rateLimitExceeded:
            return "Wait a few moments before trying again"
        default:
            return nil
        }
    }

    var isRecoverable: Bool {
        switch self {
        case .biometricsNotAvailable, .biometricsNotEnrolled:
            return false
        case .dataCorrupted, .checksumMismatch:
            return false
        case .invalidPrivateKey, .invalidPublicKey:
            return false
        default:
            return true
        }
    }

    var shouldRetry: Bool {
        switch self {
        case .networkError, .connectionFailed, .timeout:
            return true
        case .rateLimitExceeded:
            return true
        case .rpcError:
            return true
        default:
            return false
        }
    }
}

// MARK: - Result Extensions
extension Result where Failure == WalletError {
    var value: Success? {
        switch self {
        case .success(let value):
            return value
        case .failure:
            return nil
        }
    }

    var error: WalletError? {
        switch self {
        case .success:
            return nil
        case .failure(let error):
            return error
        }
    }
}

// MARK: - Error Logging
struct ErrorLogger {
    static func log(_ error: Error, context: String? = nil) {
        let message = context.map { "[\($0)] " } ?? ""
        print("ERROR: \(message)\(error.localizedDescription)")

        if let walletError = error as? WalletError {
            if let recovery = walletError.recoverySuggestion {
                print("RECOVERY: \(recovery)")
            }
        }
    }

    static func logWarning(_ message: String, context: String? = nil) {
        let prefix = context.map { "[\($0)] " } ?? ""
        print("WARNING: \(prefix)\(message)")
    }

    static func logInfo(_ message: String, context: String? = nil) {
        let prefix = context.map { "[\($0)] " } ?? ""
        print("INFO: \(prefix)\(message)")
    }
}
