//
//  BiometricService.swift
//  EtridWallet
//
//  Enhanced biometric authentication with transaction signing
//

import Foundation
import UIKit
import LocalAuthentication
import Security

@MainActor
class BiometricService: ObservableObject {
    // MARK: - Published Properties
    @Published var isEnabled = true
    @Published var requireForTransactions = true
    @Published var requireForAppLaunch = true
    @Published var requireForSensitiveData = true

    // MARK: - Private Properties
    private let securityManager = SecurityManager.shared
    private let context = LAContext()

    // MARK: - Singleton
    static let shared = BiometricService()

    private init() {
        loadSettings()
    }

    // MARK: - Biometric Availability
    var isAvailable: Bool {
        return securityManager.canUseBiometrics
    }

    var biometricType: BiometricType {
        return securityManager.biometricType
    }

    var biometricIcon: String {
        return biometricType.iconName
    }

    var biometricName: String {
        return biometricType.displayName
    }

    // MARK: - Authentication Methods
    /// Authenticates user for app unlock
    func authenticateForAppUnlock() async throws {
        guard isEnabled && requireForAppLaunch else { return }

        try await securityManager.authenticateWithBiometrics(reason: "Unlock Ëtrid Wallet")
    }

    /// Authenticates user for transaction signing
    func authenticateForTransaction(amount: String, to: String) async throws {
        guard isEnabled && requireForTransactions else { return }

        let reason = "Confirm transaction of \(amount) to \(formatAddress(to))"

        do {
            let success = try await securityManager.authenticateWithBiometrics(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        } catch {
            // If biometric fails, try passcode
            let success = try await securityManager.authenticateWithPasscode(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        }
    }

    /// Authenticates user for viewing sensitive data (private keys, seed phrase)
    func authenticateForSensitiveData(dataType: String = "sensitive data") async throws {
        guard isEnabled && requireForSensitiveData else { return }

        let reason = "Access \(dataType)"

        do {
            let success = try await securityManager.authenticateWithBiometrics(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        } catch {
            // If biometric fails, try passcode
            let success = try await securityManager.authenticateWithPasscode(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        }
    }

    /// Authenticates for wallet backup/recovery
    func authenticateForBackup() async throws {
        let reason = "Access wallet backup"

        do {
            let success = try await securityManager.authenticateWithBiometrics(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        } catch {
            let success = try await securityManager.authenticateWithPasscode(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        }
    }

    /// Authenticates for settings changes
    func authenticateForSettings() async throws {
        guard isEnabled else { return }

        let reason = "Modify security settings"

        do {
            let success = try await securityManager.authenticateWithBiometrics(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        } catch {
            let success = try await securityManager.authenticateWithPasscode(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        }
    }

    /// Authenticates for WalletConnect session approval
    func authenticateForWalletConnect(dappName: String) async throws {
        guard isEnabled else { return }

        let reason = "Connect to \(dappName)"

        do {
            let success = try await securityManager.authenticateWithBiometrics(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        } catch {
            let success = try await securityManager.authenticateWithPasscode(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        }
    }

    /// Generic authentication with custom reason
    func authenticate(reason: String) async throws {
        guard isEnabled else { return }

        do {
            let success = try await securityManager.authenticateWithBiometrics(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        } catch {
            let success = try await securityManager.authenticateWithPasscode(reason: reason)
            if !success {
                throw WalletError.authenticationFailed
            }
        }
    }

    // MARK: - Settings Management
    func toggleBiometrics(_ enabled: Bool) {
        isEnabled = enabled
        securityManager.toggleBiometrics(enabled)
        saveSettings()
    }

    func toggleTransactionAuthentication(_ enabled: Bool) {
        requireForTransactions = enabled
        saveSettings()
    }

    func toggleAppLaunchAuthentication(_ enabled: Bool) {
        requireForAppLaunch = enabled
        saveSettings()
    }

    func toggleSensitiveDataAuthentication(_ enabled: Bool) {
        requireForSensitiveData = enabled
        saveSettings()
    }

    // MARK: - Haptic Feedback
    private func provideHapticFeedback(success: Bool) {
        #if os(iOS)
        if success {
            let generator = UINotificationFeedbackGenerator()
            generator.notificationOccurred(.success)
        } else {
            let generator = UINotificationFeedbackGenerator()
            generator.notificationOccurred(.error)
        }
        #endif
    }

    // MARK: - Helper Methods
    private func formatAddress(_ address: String) -> String {
        guard address.count > 10 else { return address }
        let start = address.prefix(6)
        let end = address.suffix(4)
        return "\(start)...\(end)"
    }

    // MARK: - Persistence
    private func saveSettings() {
        UserDefaults.standard.set(isEnabled, forKey: "biometricServiceEnabled")
        UserDefaults.standard.set(requireForTransactions, forKey: "biometricRequireForTransactions")
        UserDefaults.standard.set(requireForAppLaunch, forKey: "biometricRequireForAppLaunch")
        UserDefaults.standard.set(requireForSensitiveData, forKey: "biometricRequireForSensitiveData")
    }

    private func loadSettings() {
        isEnabled = UserDefaults.standard.object(forKey: "biometricServiceEnabled") as? Bool ?? true
        requireForTransactions = UserDefaults.standard.object(forKey: "biometricRequireForTransactions") as? Bool ?? true
        requireForAppLaunch = UserDefaults.standard.object(forKey: "biometricRequireForAppLaunch") as? Bool ?? true
        requireForSensitiveData = UserDefaults.standard.object(forKey: "biometricRequireForSensitiveData") as? Bool ?? true
    }

    // MARK: - Transaction Signature Verification
    /// Verifies biometric authentication before signing transaction
    func verifyAndSignTransaction<T>(
        amount: String,
        to: String,
        signAction: () async throws -> T
    ) async throws -> T {
        // Authenticate first
        try await authenticateForTransaction(amount: amount, to: to)

        // Provide haptic feedback
        provideHapticFeedback(success: true)

        // Execute signing action
        do {
            let result = try await signAction()
            return result
        } catch {
            provideHapticFeedback(success: false)
            throw error
        }
    }

    /// Securely accesses keychain with biometric protection
    func secureKeychainAccess<T>(
        dataType: String,
        accessAction: () async throws -> T
    ) async throws -> T {
        // Authenticate first
        try await authenticateForSensitiveData(dataType: dataType)

        // Provide haptic feedback
        provideHapticFeedback(success: true)

        // Execute access action
        do {
            let result = try await accessAction()
            return result
        } catch {
            provideHapticFeedback(success: false)
            throw error
        }
    }
}

// MARK: - Biometric Policy
struct BiometricPolicy {
    var requireForTransactions: Bool = true
    var requireForAppLaunch: Bool = true
    var requireForSensitiveData: Bool = true
    var requireForWalletConnect: Bool = true
    var requireForBackup: Bool = true

    var minimumTransactionAmount: Double? = nil // Require biometric only for amounts above this

    func shouldRequireAuthentication(for transactionAmount: Double?) -> Bool {
        guard requireForTransactions else { return false }

        if let amount = transactionAmount,
           let minimum = minimumTransactionAmount {
            return amount >= minimum
        }

        return true
    }
}
