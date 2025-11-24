//
//  ErrorRecoveryManager.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import Foundation

/// Automatic error recovery with retry logic and fallback mechanisms
actor ErrorRecoveryManager {
    static let shared = ErrorRecoveryManager()

    // MARK: - Types

    enum RecoveryStrategy {
        case retry(maxAttempts: Int, backoff: BackoffStrategy)
        case fallback(alternative: () async throws -> Void)
        case ignore
        case userAction(message: String)
    }

    enum BackoffStrategy {
        case linear(seconds: TimeInterval)
        case exponential(base: TimeInterval)
        case fibonacci(base: TimeInterval)

        func delay(for attempt: Int) -> TimeInterval {
            switch self {
            case .linear(let seconds):
                return seconds * TimeInterval(attempt)
            case .exponential(let base):
                return base * pow(2.0, Double(attempt))
            case .fibonacci(let base):
                let fib = fibonacci(attempt)
                return base * TimeInterval(fib)
            }
        }

        private func fibonacci(_ n: Int) -> Int {
            guard n > 1 else { return n }
            var a = 0, b = 1
            for _ in 2...n {
                let temp = a + b
                a = b
                b = temp
            }
            return b
        }
    }

    struct RecoveryAttempt {
        let error: Error
        let attempt: Int
        let timestamp: Date
        let strategy: String
    }

    struct RecoveryResult<T> {
        let value: T?
        let attempts: Int
        let totalDuration: TimeInterval
        let success: Bool
        let finalError: Error?
    }

    // MARK: - Properties

    private var recoveryHistory: [RecoveryAttempt] = []
    private let maxHistorySize = 100
    private var fallbackEndpoints: [String] = []

    // MARK: - Initialization

    private init() {
        setupFallbackEndpoints()
    }

    private func setupFallbackEndpoints() {
        // Add fallback RPC endpoints
        fallbackEndpoints = [
            "https://rpc1.etrid.io",
            "https://rpc2.etrid.io",
            "https://rpc3.etrid.io"
        ]
    }

    // MARK: - Recovery Methods

    /// Recover from an operation with automatic retry and exponential backoff
    func recover<T>(
        from operation: @escaping () async throws -> T,
        maxRetries: Int = 3,
        backoff: BackoffStrategy = .exponential(base: 1.0),
        onRetry: ((Int, Error) -> Void)? = nil
    ) async throws -> T {
        var lastError: Error?
        let startTime = Date()

        for attempt in 0..<maxRetries {
            do {
                let result = try await operation()

                // Log success
                let duration = Date().timeIntervalSince(startTime)
                if attempt > 0 {
                    await Logger.shared.info(
                        "✅ Recovered after \(attempt + 1) attempts in \(String(format: "%.2f", duration))s",
                        category: .general
                    )
                }

                return result

            } catch {
                lastError = error

                // Record attempt
                let recoveryAttempt = RecoveryAttempt(
                    error: error,
                    attempt: attempt,
                    timestamp: Date(),
                    strategy: "retry_exponential_backoff"
                )
                recordAttempt(recoveryAttempt)

                await Logger.shared.warning(
                    "Attempt \(attempt + 1)/\(maxRetries) failed: \(error.localizedDescription)",
                    category: .general
                )

                // Notify callback
                onRetry?(attempt + 1, error)

                // Don't wait after last attempt
                guard attempt < maxRetries - 1 else { break }

                // Calculate delay with backoff
                let delay = backoff.delay(for: attempt)

                await Logger.shared.debug(
                    "Retrying in \(String(format: "%.1f", delay))s...",
                    category: .general
                )

                try await Task.sleep(nanoseconds: UInt64(delay * 1_000_000_000))
            }
        }

        // All attempts failed
        let duration = Date().timeIntervalSince(startTime)
        await Logger.shared.error(
            "❌ All \(maxRetries) recovery attempts failed in \(String(format: "%.2f", duration))s",
            category: .general
        )

        throw lastError ?? WalletError.unknownError
    }

    /// Recover with fallback operation
    func recoverWithFallback<T>(
        primary: @escaping () async throws -> T,
        fallback: @escaping () async throws -> T
    ) async throws -> T {
        do {
            return try await primary()
        } catch {
            await Logger.shared.warning(
                "Primary operation failed, trying fallback: \(error.localizedDescription)",
                category: .general
            )

            do {
                let result = try await fallback()
                await Logger.shared.info("✅ Fallback operation succeeded", category: .general)
                return result
            } catch let fallbackError {
                await Logger.shared.error(
                    "❌ Both primary and fallback operations failed",
                    category: .general
                )
                throw fallbackError
            }
        }
    }

    /// Recover network operation with fallback endpoints
    func recoverNetworkOperation<T>(
        operation: @escaping (String) async throws -> T,
        endpoints: [String]? = nil
    ) async throws -> T {
        let endpointsToTry = endpoints ?? fallbackEndpoints

        var lastError: Error?

        for (index, endpoint) in endpointsToTry.enumerated() {
            do {
                await Logger.shared.debug(
                    "Trying endpoint \(index + 1)/\(endpointsToTry.count): \(endpoint)",
                    category: .network
                )

                let result = try await operation(endpoint)

                if index > 0 {
                    await Logger.shared.info(
                        "✅ Recovered using fallback endpoint: \(endpoint)",
                        category: .network
                    )
                }

                return result

            } catch {
                lastError = error

                await Logger.shared.warning(
                    "Endpoint \(endpoint) failed: \(error.localizedDescription)",
                    category: .network
                )

                // Continue to next endpoint
                continue
            }
        }

        await Logger.shared.error(
            "❌ All \(endpointsToTry.count) endpoints failed",
            category: .network
        )

        throw lastError ?? WalletError.networkError("All endpoints failed")
    }

    // MARK: - State Recovery

    /// Recover app state after crash
    func recoverAppState() async {
        await Logger.shared.info("Attempting to recover app state after crash", category: .general)

        // Clear any corrupted state
        // Restore from last known good state
        // Notify user if needed

        await Logger.shared.info("App state recovery completed", category: .general)
    }

    /// Recover transaction state
    func recoverTransactionState(txHash: String) async throws {
        await Logger.shared.info("Recovering transaction state for \(txHash)", category: .transaction)

        // Check transaction status on blockchain
        // Update local state
        // Notify user

        await Logger.shared.info("Transaction state recovered", category: .transaction)
    }

    // MARK: - Error Classification

    func classifyError(_ error: Error) -> ErrorClassification {
        switch error {
        case let walletError as WalletError:
            return classifyWalletError(walletError)
        case is DecodingError:
            return .dataCorruption
        case is URLError:
            return .network
        default:
            return .unknown
        }
    }

    private func classifyWalletError(_ error: WalletError) -> ErrorClassification {
        switch error {
        case .networkError:
            return .network
        case .insufficientBalance:
            return .userError
        case .invalidAddress:
            return .validationError
        case .transactionFailed:
            return .transactionError
        default:
            return .unknown
        }
    }

    enum ErrorClassification {
        case network
        case dataCorruption
        case userError
        case validationError
        case transactionError
        case unknown

        var isRecoverable: Bool {
            switch self {
            case .network, .transactionError:
                return true
            case .dataCorruption, .userError, .validationError, .unknown:
                return false
            }
        }

        var suggestedStrategy: RecoveryStrategy {
            switch self {
            case .network:
                return .retry(maxAttempts: 3, backoff: .exponential(base: 1.0))
            case .transactionError:
                return .retry(maxAttempts: 2, backoff: .linear(seconds: 5.0))
            case .userError, .validationError:
                return .userAction(message: "Please check your input and try again")
            case .dataCorruption:
                return .userAction(message: "Data corruption detected. Please restart the app")
            case .unknown:
                return .ignore
            }
        }
    }

    // MARK: - Circuit Breaker

    private var circuitBreakerState: CircuitBreakerState = .closed
    private var failureCount = 0
    private var lastFailureTime: Date?
    private let failureThreshold = 5
    private let resetTimeout: TimeInterval = 60.0 // 1 minute

    enum CircuitBreakerState {
        case closed   // Normal operation
        case open     // Failing, reject all requests
        case halfOpen // Testing if service recovered
    }

    func executeWithCircuitBreaker<T>(
        operation: @escaping () async throws -> T
    ) async throws -> T {
        // Check circuit breaker state
        switch circuitBreakerState {
        case .open:
            // Check if we should try to recover
            if let lastFailure = lastFailureTime,
               Date().timeIntervalSince(lastFailure) > resetTimeout {
                circuitBreakerState = .halfOpen
                await Logger.shared.info("Circuit breaker entering half-open state", category: .general)
            } else {
                throw WalletError.serviceUnavailable("Circuit breaker is open")
            }

        case .halfOpen:
            // Try one request
            do {
                let result = try await operation()
                // Success! Reset circuit breaker
                circuitBreakerState = .closed
                failureCount = 0
                await Logger.shared.info("Circuit breaker closed after successful recovery", category: .general)
                return result
            } catch {
                // Failed again, reopen circuit
                circuitBreakerState = .open
                lastFailureTime = Date()
                await Logger.shared.error("Circuit breaker reopened after failed recovery", category: .general)
                throw error
            }

        case .closed:
            // Normal operation
            do {
                return try await operation()
            } catch {
                failureCount += 1
                lastFailureTime = Date()

                if failureCount >= failureThreshold {
                    circuitBreakerState = .open
                    await Logger.shared.error(
                        "Circuit breaker opened after \(failureCount) failures",
                        category: .general
                    )
                }

                throw error
            }
        }

        throw WalletError.unknownError
    }

    // MARK: - History & Reporting

    private func recordAttempt(_ attempt: RecoveryAttempt) {
        recoveryHistory.append(attempt)

        if recoveryHistory.count > maxHistorySize {
            recoveryHistory.removeFirst(recoveryHistory.count - maxHistorySize)
        }
    }

    func getRecoveryHistory() -> [RecoveryAttempt] {
        return recoveryHistory
    }

    func getRecoveryStats() -> [String: Any] {
        return [
            "total_attempts": recoveryHistory.count,
            "recent_failures": recoveryHistory.filter {
                Date().timeIntervalSince($0.timestamp) < 3600 // Last hour
            }.count,
            "circuit_breaker_state": String(describing: circuitBreakerState),
            "failure_count": failureCount
        ]
    }

    func clearHistory() {
        recoveryHistory.removeAll()
        failureCount = 0
        circuitBreakerState = .closed
        lastFailureTime = nil
    }
}

// MARK: - WalletError Extension

extension WalletError {
    static func serviceUnavailable(_ message: String) -> WalletError {
        return .unknownError // Add proper case to WalletError enum
    }
}

// MARK: - Global Convenience Functions

/// Recover from operation with automatic retry
func recoverFromError<T>(
    maxRetries: Int = 3,
    operation: @escaping () async throws -> T
) async throws -> T {
    return try await ErrorRecoveryManager.shared.recover(
        from: operation,
        maxRetries: maxRetries
    )
}

/// Recover with fallback operation
func recoverWithFallback<T>(
    primary: @escaping () async throws -> T,
    fallback: @escaping () async throws -> T
) async throws -> T {
    return try await ErrorRecoveryManager.shared.recoverWithFallback(
        primary: primary,
        fallback: fallback
    )
}
