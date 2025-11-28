//
//  SignRequestView.swift
//  EtridWalletSwift
//
//  Created by Etrid Team on 2025-11-22.
//  Copyright © 2025 Etrid. All rights reserved.
//

import SwiftUI
import LocalAuthentication

/// View for displaying signature request from dApps
struct SignRequestView: View {
    // MARK: - Properties

    let signRequest: SignRequest
    let onApprove: () async -> Void
    let onReject: () async -> Void

    @Environment(\.dismiss) private var dismiss
    @State private var isProcessing = false
    @State private var showError = false
    @State private var errorMessage = ""
    @State private var expandedMessage = false
    @State private var biometricAuthenticated = false

    // MARK: - Body

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // Header
                    headerSection

                    // Security Warning (if dangerous)
                    if signRequest.isDangerous {
                        dangerWarningSection
                    }

                    // dApp Information
                    dAppInfoSection

                    // Signature Type
                    signatureTypeSection

                    // Message Content
                    messageSection

                    // Security Warnings
                    if !signRequest.securityWarnings.isEmpty {
                        securityWarningsSection
                    }

                    // Account Information
                    accountSection

                    // Action Buttons
                    actionButtons
                }
                .padding()
            }
            .navigationTitle("Signature Request")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        Task {
                            await reject()
                        }
                    }
                    .disabled(isProcessing)
                }
            }
            .alert("Error", isPresented: $showError) {
                Button("OK", role: .cancel) {}
            } message: {
                Text(errorMessage)
            }
        }
    }

    // MARK: - View Components

    private var headerSection: some View {
        VStack(spacing: 16) {
            // Icon
            Image(systemName: signRequest.isDangerous ? "exclamationmark.shield.fill" : "signature")
                .font(.system(size: 50))
                .foregroundColor(signRequest.isDangerous ? .red : .blue)

            Text("Signature Request")
                .font(.title2)
                .fontWeight(.bold)

            Text("Review this signature request carefully before approving")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
    }

    private var dangerWarningSection: some View {
        VStack(spacing: 12) {
            HStack {
                Image(systemName: "exclamationmark.triangle.fill")
                    .foregroundColor(.white)

                Text("Dangerous Signature Request")
                    .font(.headline)
                    .foregroundColor(.white)

                Spacer()
            }

            Text("This signature request is potentially dangerous. Only sign if you completely trust this dApp and understand what you're signing.")
                .font(.subheadline)
                .foregroundColor(.white)
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color.red)
        )
    }

    private var dAppInfoSection: some View {
        VStack(spacing: 12) {
            // dApp Icon
            AsyncImage(url: signRequest.dAppMetadata.iconURL) { phase in
                switch phase {
                case .empty:
                    ProgressView()
                case .success(let image):
                    image
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                case .failure:
                    Image(systemName: "app.badge")
                        .foregroundColor(.secondary)
                @unknown default:
                    EmptyView()
                }
            }
            .frame(width: 50, height: 50)
            .clipShape(RoundedRectangle(cornerRadius: 10))

            Text(signRequest.dAppMetadata.name)
                .font(.headline)

            HStack {
                Image(systemName: "globe")
                    .font(.caption)

                Text(signRequest.dAppMetadata.displayURL)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .frame(maxWidth: .infinity)
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color(.secondarySystemBackground))
        )
    }

    private var signatureTypeSection: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text("Signature Type")
                    .font(.caption)
                    .foregroundColor(.secondary)

                Text(signRequest.displayType)
                    .font(.subheadline)
                    .fontWeight(.medium)
            }

            Spacer()

            if signRequest.type == .ethSign {
                Image(systemName: "exclamationmark.triangle.fill")
                    .foregroundColor(.orange)
            }
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 12)
                .fill(Color(.secondarySystemBackground))
        )
    }

    private var messageSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Message to Sign")
                    .font(.headline)
                    .fontWeight(.semibold)

                Spacer()

                Button {
                    withAnimation {
                        expandedMessage.toggle()
                    }
                } label: {
                    Image(systemName: expandedMessage ? "chevron.up" : "chevron.down")
                        .foregroundColor(.blue)
                }
            }

            if expandedMessage || signRequest.message.count < 200 {
                ScrollView {
                    Text(signRequest.message)
                        .font(.system(.caption, design: .monospaced))
                        .padding()
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(
                            RoundedRectangle(cornerRadius: 8)
                                .fill(Color(.tertiarySystemBackground))
                        )
                }
                .frame(maxHeight: expandedMessage ? 300 : nil)
            } else {
                Text(signRequest.message.prefix(200) + "...")
                    .font(.system(.caption, design: .monospaced))
                    .padding()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(
                        RoundedRectangle(cornerRadius: 8)
                            .fill(Color(.tertiarySystemBackground))
                    )
                    .lineLimit(5)
            }

            // Show raw message button
            if signRequest.type != .personalSign {
                Button {
                    // Show raw message in modal
                } label: {
                    HStack {
                        Image(systemName: "doc.text")
                        Text("View Raw Message")
                    }
                    .font(.caption)
                    .foregroundColor(.blue)
                }
            }
        }
    }

    private var securityWarningsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "shield.lefthalf.filled")
                    .foregroundColor(.orange)

                Text("Security Warnings")
                    .font(.headline)
                    .fontWeight(.semibold)
            }

            VStack(alignment: .leading, spacing: 8) {
                ForEach(signRequest.securityWarnings, id: \.self) { warning in
                    HStack(alignment: .top, spacing: 8) {
                        Image(systemName: "exclamationmark.circle.fill")
                            .foregroundColor(.orange)
                            .font(.caption)

                        Text(warning)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 8)
                    .fill(Color.orange.opacity(0.1))
            )
        }
    }

    private var accountSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Signing Account")
                .font(.headline)
                .fontWeight(.semibold)

            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Address")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text(formatAddress(signRequest.address))
                        .font(.system(.subheadline, design: .monospaced))
                }

                Spacer()

                Button {
                    UIPasteboard.general.string = signRequest.address
                } label: {
                    Image(systemName: "doc.on.doc")
                        .foregroundColor(.blue)
                }
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 8)
                    .fill(Color(.tertiarySystemBackground))
            )

            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Network")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text(networkName(chainId: signRequest.chainId))
                        .font(.subheadline)
                }

                Spacer()
            }
            .padding()
            .background(
                RoundedRectangle(cornerRadius: 8)
                    .fill(Color(.tertiarySystemBackground))
            )
        }
    }

    private var actionButtons: some View {
        VStack(spacing: 12) {
            Button {
                Task {
                    await approve()
                }
            } label: {
                HStack {
                    if isProcessing {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                    } else {
                        Image(systemName: biometricIcon())
                        Text("Sign with \(biometricType())")
                            .fontWeight(.semibold)
                    }
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(signRequest.isDangerous ? Color.orange : Color.blue)
                )
                .foregroundColor(.white)
            }
            .disabled(isProcessing)

            Button {
                Task {
                    await reject()
                }
            } label: {
                Text("Reject")
                    .fontWeight(.medium)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(
                        RoundedRectangle(cornerRadius: 12)
                            .stroke(Color.red, lineWidth: 2)
                    )
                    .foregroundColor(.red)
            }
            .disabled(isProcessing)
        }
        .padding(.top, 8)
    }

    // MARK: - Actions

    private func approve() async {
        isProcessing = true

        // Require biometric authentication
        let authenticated = await authenticateWithBiometrics()

        guard authenticated else {
            await MainActor.run {
                errorMessage = "Biometric authentication failed"
                showError = true
                isProcessing = false
            }
            return
        }

        await onApprove()

        await MainActor.run {
            dismiss()
        }
    }

    private func reject() async {
        isProcessing = true
        await onReject()

        await MainActor.run {
            dismiss()
        }
    }

    // MARK: - Biometric Authentication

    private func authenticateWithBiometrics() async -> Bool {
        let context = LAContext()
        var error: NSError?

        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            return false
        }

        do {
            let reason = "Authenticate to sign this message"
            let success = try await context.evaluatePolicy(
                .deviceOwnerAuthenticationWithBiometrics,
                localizedReason: reason
            )
            return success
        } catch {
            return false
        }
    }

    private func biometricType() -> String {
        let context = LAContext()
        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil) {
            switch context.biometryType {
            case .faceID:
                return "Face ID"
            case .touchID:
                return "Touch ID"
            default:
                return "Passcode"
            }
        }
        return "Passcode"
    }

    private func biometricIcon() -> String {
        let context = LAContext()
        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil) {
            switch context.biometryType {
            case .faceID:
                return "faceid"
            case .touchID:
                return "touchid"
            default:
                return "lock.fill"
            }
        }
        return "lock.fill"
    }

    // MARK: - Helper Methods

    private func formatAddress(_ address: String) -> String {
        guard address.count >= 42 else { return address }
        return String(address.prefix(10)) + "..." + String(address.suffix(8))
    }

    private func networkName(chainId: Int) -> String {
        switch chainId {
        case 1:
            return "Ethereum Mainnet"
        case 5:
            return "Goerli Testnet"
        case 137:
            return "Polygon"
        case 56:
            return "Binance Smart Chain"
        case 43114:
            return "Avalanche C-Chain"
        default:
            return "Chain ID: \(chainId)"
        }
    }
}

// MARK: - Preview

struct SignRequestView_Previews: PreviewProvider {
    static var previews: some View {
        SignRequestView(
            signRequest: SignRequest(
                id: 1,
                type: .personalSign,
                message: "Welcome to OpenSea!\n\nClick to sign in and accept the OpenSea Terms of Service.\n\nThis request will not trigger a blockchain transaction or cost any gas fees.",
                rawMessage: "0x57656c636f6d6520746f204f70656e536561",
                address: "0x1234567890123456789012345678901234567890",
                chainId: 1,
                dAppMetadata: DAppMetadata(
                    name: "OpenSea",
                    description: "The largest NFT marketplace",
                    url: "https://opensea.io",
                    icons: ["https://opensea.io/static/images/logos/opensea.svg"]
                ),
                securityWarnings: []
            ),
            onApprove: {},
            onReject: {}
        )
    }
}
