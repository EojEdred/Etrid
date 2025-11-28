//
//  SecurityManager.swift
//  EtridWallet
//
//  Biometric authentication and security features
//

import Foundation
import LocalAuthentication
import Security
import UIKit

@MainActor
class SecurityManager: ObservableObject {
    // MARK: - Published Properties
    @Published var isLocked = true
    @Published var biometricsEnabled = true
    @Published var autoLockEnabled = true
    @Published var autoLockTimeout: TimeInterval = 300 // 5 minutes

    // MARK: - Private Properties
    private var lastActiveTime: Date = Date()
    private var lockTimer: Timer?

    // MARK: - Singleton
    static let shared = SecurityManager()

    private init() {
        setupAutoLock()
        loadSettings()
    }

    // MARK: - Biometric Authentication
    /// Checks if biometric authentication is available
    var canUseBiometrics: Bool {
        let context = LAContext()
        var error: NSError?
        return context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error)
    }

    /// Returns the type of biometric authentication available
    var biometricType: BiometricType {
        let context = LAContext()
        guard canUseBiometrics else { return .none }

        switch context.biometryType {
        case .faceID:
            return .faceID
        case .touchID:
            return .touchID
        case .opticID:
            return .opticID
        case .none:
            return .none
        @unknown default:
            return .none
        }
    }

    /// Authenticates user with biometrics
    /// - Parameter reason: Reason for authentication
    /// - Returns: Success or failure
    func authenticateWithBiometrics(reason: String = "Unlock your wallet") async throws -> Bool {
        let context = LAContext()
        context.localizedCancelTitle = "Cancel"
        context.localizedFallbackTitle = "Use Passcode"

        var error: NSError?
        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            if let err = error {
                throw WalletError.securityError(err.localizedDescription)
            }
            throw WalletError.biometricsNotAvailable
        }

        do {
            let success = try await context.evaluatePolicy(
                .deviceOwnerAuthenticationWithBiometrics,
                localizedReason: reason
            )

            if success {
                isLocked = false
                updateLastActiveTime()
            }

            return success
        } catch let error as LAError {
            switch error.code {
            case .biometryNotAvailable:
                throw WalletError.biometricsNotAvailable
            case .biometryNotEnrolled:
                throw WalletError.biometricsNotEnrolled
            case .userCancel, .userFallback, .systemCancel:
                throw WalletError.authenticationFailed
            default:
                throw WalletError.securityError(error.localizedDescription)
            }
        }
    }

    /// Authenticates with device passcode
    /// - Parameter reason: Reason for authentication
    /// - Returns: Success or failure
    func authenticateWithPasscode(reason: String = "Unlock your wallet") async throws -> Bool {
        let context = LAContext()

        do {
            let success = try await context.evaluatePolicy(
                .deviceOwnerAuthentication,
                localizedReason: reason
            )

            if success {
                isLocked = false
                updateLastActiveTime()
            }

            return success
        } catch {
            throw WalletError.authenticationFailed
        }
    }

    /// Main unlock method - tries biometrics first if enabled
    func unlock() async throws {
        if biometricsEnabled && canUseBiometrics {
            _ = try await authenticateWithBiometrics()
        } else {
            _ = try await authenticateWithPasscode()
        }
    }

    /// Locks the wallet
    func lock() {
        isLocked = true
    }

    // MARK: - Auto Lock
    /// Sets up auto-lock functionality
    private func setupAutoLock() {
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(appDidEnterBackground),
            name: UIApplication.didEnterBackgroundNotification,
            object: nil
        )

        NotificationCenter.default.addObserver(
            self,
            selector: #selector(appWillEnterForeground),
            name: UIApplication.willEnterForegroundNotification,
            object: nil
        )
    }

    @objc private func appDidEnterBackground() {
        lastActiveTime = Date()
        if autoLockEnabled {
            lock()
        }
    }

    @objc private func appWillEnterForeground() {
        if autoLockEnabled {
            let elapsed = Date().timeIntervalSince(lastActiveTime)
            if elapsed > autoLockTimeout {
                lock()
            }
        }
    }

    /// Updates last active time
    func updateLastActiveTime() {
        lastActiveTime = Date()
    }

    /// Starts auto-lock timer
    func startAutoLockTimer() {
        guard autoLockEnabled else { return }

        lockTimer?.invalidate()
        lockTimer = Timer.scheduledTimer(withTimeInterval: autoLockTimeout, repeats: false) { [weak self] _ in
            self?.lock()
        }
    }

    /// Resets auto-lock timer
    func resetAutoLockTimer() {
        guard autoLockEnabled else { return }
        startAutoLockTimer()
    }

    // MARK: - Settings
    /// Toggles biometric authentication
    func toggleBiometrics(_ enabled: Bool) {
        biometricsEnabled = enabled
        saveSettings()
    }

    /// Sets auto-lock timeout
    /// - Parameter timeout: Timeout in seconds
    func setAutoLockTimeout(_ timeout: TimeInterval) {
        autoLockTimeout = timeout
        saveSettings()
        if autoLockEnabled {
            startAutoLockTimer()
        }
    }

    /// Toggles auto-lock
    func toggleAutoLock(_ enabled: Bool) {
        autoLockEnabled = enabled
        saveSettings()

        if enabled {
            startAutoLockTimer()
        } else {
            lockTimer?.invalidate()
        }
    }

    // MARK: - Persistence
    private func saveSettings() {
        UserDefaults.standard.set(biometricsEnabled, forKey: "biometricsEnabled")
        UserDefaults.standard.set(autoLockEnabled, forKey: "autoLockEnabled")
        UserDefaults.standard.set(autoLockTimeout, forKey: "autoLockTimeout")
    }

    private func loadSettings() {
        biometricsEnabled = UserDefaults.standard.bool(forKey: "biometricsEnabled")
        autoLockEnabled = UserDefaults.standard.bool(forKey: "autoLockEnabled")
        autoLockTimeout = UserDefaults.standard.double(forKey: "autoLockTimeout")

        // Set defaults if not set
        if autoLockTimeout == 0 {
            autoLockTimeout = 300
        }
    }

    // MARK: - Security Validation
    /// Validates if device is secure
    var isDeviceSecure: Bool {
        let context = LAContext()
        var error: NSError?
        return context.canEvaluatePolicy(.deviceOwnerAuthentication, error: &error)
    }

    /// Checks if device is jailbroken (basic check)
    var isJailbroken: Bool {
        #if targetEnvironment(simulator)
        return false
        #else
        let paths = [
            "/Applications/Cydia.app",
            "/Library/MobileSubstrate/MobileSubstrate.dylib",
            "/bin/bash",
            "/usr/sbin/sshd",
            "/etc/apt",
            "/private/var/lib/apt/"
        ]

        for path in paths {
            if FileManager.default.fileExists(atPath: path) {
                return true
            }
        }

        // Try to write to system directory
        let testPath = "/private/jailbreak_test.txt"
        do {
            try "test".write(toFile: testPath, atomically: true, encoding: .utf8)
            try FileManager.default.removeItem(atPath: testPath)
            return true
        } catch {
            return false
        }
        #endif
    }
}

// MARK: - Biometric Type
enum BiometricType {
    case none
    case touchID
    case faceID
    case opticID

    var displayName: String {
        switch self {
        case .none:
            return "None"
        case .touchID:
            return "Touch ID"
        case .faceID:
            return "Face ID"
        case .opticID:
            return "Optic ID"
        }
    }

    var iconName: String {
        switch self {
        case .none:
            return "lock.fill"
        case .touchID:
            return "touchid"
        case .faceID:
            return "faceid"
        case .opticID:
            return "opticid"
        }
    }
}

// MARK: - Security Level
enum SecurityLevel {
    case low
    case medium
    case high

    var description: String {
        switch self {
        case .low:
            return "Low - Passcode only"
        case .medium:
            return "Medium - Biometrics enabled"
        case .high:
            return "High - Biometrics + Auto-lock"
        }
    }
}
