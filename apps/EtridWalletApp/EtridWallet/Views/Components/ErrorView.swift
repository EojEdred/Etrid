//
//  ErrorView.swift
//  EtridWallet
//
//  Error display components
//

import SwiftUI

struct ErrorView: View {
    let error: Error
    let retryAction: (() -> Void)?

    init(error: Error, retryAction: (() -> Void)? = nil) {
        self.error = error
        self.retryAction = retryAction
    }

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "exclamationmark.triangle.fill")
                .font(.system(size: 50))
                .foregroundColor(.etridOrange)

            Text("Something went wrong")
                .font(.headline)

            Text(error.localizedDescription)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            if let retryAction = retryAction {
                Button(action: retryAction) {
                    Text("Try Again")
                        .font(.headline)
                        .foregroundColor(.white)
                        .padding(.horizontal, 32)
                        .padding(.vertical, 12)
                        .background(Color.etridBlue)
                        .cornerRadius(12)
                }
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }
}

struct InlineErrorView: View {
    let message: String
    let dismissAction: (() -> Void)?

    init(message: String, dismissAction: (() -> Void)? = nil) {
        self.message = message
        self.dismissAction = dismissAction
    }

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: "exclamationmark.circle.fill")
                .foregroundColor(.etridRed)

            Text(message)
                .font(.caption)
                .foregroundColor(.etridRed)

            Spacer()

            if let dismissAction = dismissAction {
                Button(action: dismissAction) {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding()
        .background(Color.etridRed.opacity(0.1))
        .cornerRadius(8)
    }
}

struct WarningView: View {
    let message: String
    let icon: String

    init(message: String, icon: String = "exclamationmark.triangle.fill") {
        self.message = message
        self.icon = icon
    }

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.orange)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.orange.opacity(0.1))
        .cornerRadius(12)
    }
}

struct InfoView: View {
    let message: String
    let icon: String

    init(message: String, icon: String = "info.circle.fill") {
        self.message = message
        self.icon = icon
    }

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.etridBlue)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .padding()
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.etridBlue.opacity(0.1))
        .cornerRadius(12)
    }
}

struct EmptyStateView: View {
    let icon: String
    let title: String
    let message: String
    let actionTitle: String?
    let action: (() -> Void)?

    init(
        icon: String,
        title: String,
        message: String,
        actionTitle: String? = nil,
        action: (() -> Void)? = nil
    ) {
        self.icon = icon
        self.title = title
        self.message = message
        self.actionTitle = actionTitle
        self.action = action
    }

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: icon)
                .font(.system(size: 60))
                .foregroundColor(.secondary)

            Text(title)
                .font(.headline)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            if let actionTitle = actionTitle, let action = action {
                Button(action: action) {
                    Text(actionTitle)
                        .font(.headline)
                        .foregroundColor(.white)
                        .padding(.horizontal, 32)
                        .padding(.vertical, 12)
                        .background(Color.etridBlue)
                        .cornerRadius(12)
                }
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }
}

#Preview {
    VStack(spacing: 20) {
        InlineErrorView(message: "Invalid address format")

        WarningView(message: "This action cannot be undone")

        InfoView(message: "Transaction is being processed")

        Divider()

        EmptyStateView(
            icon: "tray",
            title: "No Items",
            message: "You don't have any items yet",
            actionTitle: "Add Item",
            action: {}
        )
    }
    .padding()
}
