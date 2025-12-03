//
//  ErrorView.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import SwiftUI

// MARK: - Error View

/// User-friendly error display with retry and support options
struct ErrorView: View {
    let error: Error
    let onRetry: (() -> Void)?
    let onDismiss: (() -> Void)?

    @State private var showDetails = false

    init(
        error: Error,
        onRetry: (() -> Void)? = nil,
        onDismiss: (() -> Void)? = nil
    ) {
        self.error = error
        self.onRetry = onRetry
        self.onDismiss = onDismiss
    }

    var body: some View {
        VStack(spacing: 24) {
            // Error Icon
            Image(systemName: errorType.icon)
                .font(.system(size: 60))
                .foregroundColor(errorType.color)
                .padding(.top, 40)

            // Error Content
            VStack(spacing: 12) {
                Text(errorType.title)
                    .font(.title2)
                    .fontWeight(.semibold)

                Text(errorMessage)
                    .font(.body)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 32)
            }

            // Actions
            VStack(spacing: 12) {
                if let onRetry = onRetry {
                    Button(action: {
                        Haptics.selection()
                        onRetry()
                    }) {
                        HStack {
                            Image(systemName: "arrow.clockwise")
                            Text("Try Again")
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.blue)
                        .cornerRadius(12)
                    }
                    .padding(.horizontal, 32)
                }

                Button(action: {
                    showDetails.toggle()
                }) {
                    HStack {
                        Image(systemName: "info.circle")
                        Text(showDetails ? "Hide Details" : "Show Details")
                    }
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                }
            }

            // Error Details
            if showDetails {
                VStack(alignment: .leading, spacing: 8) {
                    Text("Error Details")
                        .font(.headline)
                        .padding(.bottom, 4)

                    Text("Type: \(String(describing: type(of: error)))")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text("Message: \(error.localizedDescription)")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    if let walletError = error as? WalletError {
                        Text("Code: \(walletError.errorDescription ?? "Unknown")")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(8)
                .padding(.horizontal, 32)
            }

            Spacer()

            // Support Options
            VStack(spacing: 8) {
                Text("Need help?")
                    .font(.subheadline)
                    .foregroundColor(.secondary)

                HStack(spacing: 16) {
                    Button(action: openSupport) {
                        HStack {
                            Image(systemName: "questionmark.circle")
                            Text("Support")
                        }
                        .font(.subheadline)
                    }

                    Divider()
                        .frame(height: 20)

                    Button(action: reportBug) {
                        HStack {
                            Image(systemName: "exclamationmark.bubble")
                            Text("Report Bug")
                        }
                        .font(.subheadline)
                    }
                }
            }
            .padding(.bottom, 32)

            // Dismiss Button
            if let onDismiss = onDismiss {
                Button(action: {
                    Haptics.selection()
                    onDismiss()
                }) {
                    Text("Dismiss")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                .padding(.bottom, 16)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(.systemBackground))
    }

    // MARK: - Error Type

    private var errorType: ErrorType {
        if let walletError = error as? WalletError {
            return ErrorType.fromWalletError(walletError)
        } else if error is URLError {
            return .network
        } else if error is DecodingError {
            return .data
        } else {
            return .unknown
        }
    }

    private var errorMessage: String {
        if let walletError = error as? WalletError {
            return walletError.userFriendlyMessage
        } else {
            return error.localizedDescription
        }
    }

    // MARK: - Actions

    private func openSupport() {
        Haptics.selection()
        if let url = URL(string: "https://support.etrid.org") {
            UIApplication.shared.open(url)
        }
        logInfo("Support page opened from error view", category: .ui)
    }

    private func reportBug() {
        Haptics.selection()
        let subject = "Bug Report: \(String(describing: type(of: error)))"
        let body = """
        Error Type: \(String(describing: type(of: error)))
        Error Message: \(error.localizedDescription)
        App Version: \(AppConfig.shared.fullVersion)

        Please describe what you were doing when this error occurred:

        """

        if let url = URL(string: "mailto:gizzi_io@proton.me?subject=\(subject.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? "")&body=\(body.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? "")") {
            UIApplication.shared.open(url)
        }
        logInfo("Bug report initiated from error view", category: .ui)
    }
}

// MARK: - Error Type

enum ErrorType {
    case network
    case authentication
    case validation
    case data
    case transaction
    case security
    case unknown

    var icon: String {
        switch self {
        case .network:
            return "wifi.exclamationmark"
        case .authentication:
            return "lock.shield"
        case .validation:
            return "exclamationmark.triangle"
        case .data:
            return "doc.badge.exclamationmark"
        case .transaction:
            return "arrow.left.arrow.right.circle"
        case .security:
            return "shield.slash"
        case .unknown:
            return "exclamationmark.circle"
        }
    }

    var color: Color {
        switch self {
        case .network:
            return .orange
        case .authentication, .security:
            return .red
        case .validation:
            return .yellow
        case .data:
            return .blue
        case .transaction:
            return .purple
        case .unknown:
            return .gray
        }
    }

    var title: String {
        switch self {
        case .network:
            return "Connection Error"
        case .authentication:
            return "Authentication Failed"
        case .validation:
            return "Invalid Input"
        case .data:
            return "Data Error"
        case .transaction:
            return "Transaction Error"
        case .security:
            return "Security Error"
        case .unknown:
            return "Something Went Wrong"
        }
    }

    static func fromWalletError(_ error: WalletError) -> ErrorType {
        switch error {
        case .networkError:
            return .network
        case .invalidAddress, .invalidAmount:
            return .validation
        case .transactionFailed, .insufficientBalance:
            return .transaction
        case .authenticationFailed:
            return .authentication
        default:
            return .unknown
        }
    }
}

// MARK: - Inline Error View

struct InlineErrorView: View {
    let message: String
    let onDismiss: (() -> Void)?

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.primary)

            Spacer()

            if let onDismiss = onDismiss {
                Button(action: {
                    Haptics.selection()
                    onDismiss()
                }) {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .cornerRadius(8)
    }
}

// MARK: - Error Banner

struct ErrorBanner: View {
    let message: String
    let type: BannerType
    @Binding var isShowing: Bool

    enum BannerType {
        case error
        case warning
        case info
        case success

        var color: Color {
            switch self {
            case .error: return .red
            case .warning: return .orange
            case .info: return .blue
            case .success: return .green
            }
        }

        var icon: String {
            switch self {
            case .error: return "xmark.circle.fill"
            case .warning: return "exclamationmark.triangle.fill"
            case .info: return "info.circle.fill"
            case .success: return "checkmark.circle.fill"
            }
        }
    }

    var body: some View {
        if isShowing {
            VStack {
                HStack(spacing: 12) {
                    Image(systemName: type.icon)
                        .foregroundColor(.white)

                    Text(message)
                        .font(.subheadline)
                        .foregroundColor(.white)

                    Spacer()

                    Button(action: {
                        withAnimation {
                            isShowing = false
                        }
                    }) {
                        Image(systemName: "xmark")
                            .foregroundColor(.white)
                    }
                }
                .padding()
                .background(type.color)
                .cornerRadius(8)
                .shadow(radius: 4)

                Spacer()
            }
            .padding()
            .transition(.move(edge: .top).combined(with: .opacity))
            .onAppear {
                // Auto-dismiss after 5 seconds
                DispatchQueue.main.asyncAfter(deadline: .now() + 5) {
                    withAnimation {
                        isShowing = false
                    }
                }
            }
        }
    }
}

// MARK: - Alert Error View

struct AlertErrorView: View {
    let error: Error
    @Binding var isPresented: Bool

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "exclamationmark.triangle.fill")
                .font(.system(size: 50))
                .foregroundColor(.orange)

            Text("Error")
                .font(.title2)
                .fontWeight(.semibold)

            Text(error.localizedDescription)
                .font(.body)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            Button(action: {
                isPresented = false
            }) {
                Text("OK")
                    .fontWeight(.semibold)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.blue)
                    .cornerRadius(12)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(16)
        .shadow(radius: 20)
        .padding(40)
    }
}

// MARK: - WalletError Extension

extension WalletError {
    var userFriendlyMessage: String {
        switch self {
        case .networkError(let message):
            return "Unable to connect to the network. \(message)"
        case .invalidAddress:
            return "The wallet address you entered is not valid. Please check and try again."
        case .invalidAmount:
            return "The amount you entered is not valid."
        case .insufficientBalance:
            return "You don't have enough balance to complete this transaction."
        case .transactionFailed(let reason):
            return "Transaction failed: \(reason)"
        case .authenticationFailed:
            return "Authentication failed. Please try again."
        case .invalidPrivateKey:
            return "The private key is invalid."
        case .walletNotFound:
            return "Wallet not found. Please create or import a wallet first."
        case .seedPhraseInvalid:
            return "The recovery phrase is invalid. Please check and try again."
        case .unknown:
            return "An unexpected error occurred. Please try again."
        default:
            return self.localizedDescription
        }
    }
}

// MARK: - Error Alert Modifier

struct ErrorAlert: ViewModifier {
    @Binding var error: Error?

    func body(content: Content) -> some View {
        content
            .alert("Error", isPresented: Binding(
                get: { error != nil },
                set: { if !$0 { error = nil } }
            )) {
                Button("OK") {
                    error = nil
                }
            } message: {
                if let error = error {
                    Text(error.localizedDescription)
                }
            }
    }
}

extension View {
    func errorAlert(_ error: Binding<Error?>) -> some View {
        modifier(ErrorAlert(error: error))
    }
}

// MARK: - Preview

#if DEBUG
struct ErrorView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            ErrorView(
                error: WalletError.networkError("Connection timeout"),
                onRetry: {},
                onDismiss: {}
            )
            .previewDisplayName("Network Error")

            ErrorView(
                error: WalletError.insufficientBalance,
                onRetry: {}
            )
            .previewDisplayName("Transaction Error")

            InlineErrorView(
                message: "Invalid address format",
                onDismiss: {}
            )
            .padding()
            .previewDisplayName("Inline Error")

            VStack {
                ErrorBanner(
                    message: "Transaction failed",
                    type: .error,
                    isShowing: .constant(true)
                )
                Spacer()
            }
            .previewDisplayName("Error Banner")
        }
    }
}
#endif
