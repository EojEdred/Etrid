//
//  SecuritySettingsView.swift
//  EtridWallet
//
//  Comprehensive security settings
//

import SwiftUI

struct SecuritySettingsView: View {
    @StateObject private var securityManager = SecurityManager.shared
    @StateObject private var biometricService = BiometricService.shared
    @State private var showJailbreakWarning = false
    @State private var showSecurityTips = false

    var body: some View {
        List {
            // Security Overview
            Section {
                HStack {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Security Level")
                            .font(.headline)

                        HStack {
                            Circle()
                                .fill(securityLevelColor)
                                .frame(width: 12, height: 12)

                            Text(currentSecurityLevel.description)
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                        }
                    }

                    Spacer()

                    Image(systemName: "shield.fill")
                        .font(.largeTitle)
                        .foregroundColor(securityLevelColor)
                }
                .padding(.vertical, 4)
            }

            // Device Security Status
            Section {
                SecurityStatusRow(
                    title: "Device Secure",
                    isSecure: securityManager.isDeviceSecure,
                    icon: "checkmark.shield.fill"
                )

                SecurityStatusRow(
                    title: "Biometrics Available",
                    isSecure: biometricService.isAvailable,
                    icon: biometricService.biometricIcon
                )

                SecurityStatusRow(
                    title: "Device Integrity",
                    isSecure: !securityManager.isJailbroken,
                    icon: "lock.shield.fill"
                )

                if securityManager.isJailbroken {
                    Button {
                        showJailbreakWarning = true
                    } label: {
                        HStack {
                            Image(systemName: "exclamationmark.triangle.fill")
                                .foregroundColor(.red)
                            Text("Jailbreak Detected")
                                .foregroundColor(.red)
                        }
                    }
                }
            } header: {
                Text("Device Status")
            }

            // Authentication Settings
            Section {
                NavigationLink {
                    BiometricSettingsView()
                } label: {
                    HStack {
                        Image(systemName: biometricService.biometricIcon)
                            .foregroundColor(.blue)
                            .frame(width: 30)

                        VStack(alignment: .leading, spacing: 2) {
                            Text("Biometric Authentication")
                            Text(biometricService.isEnabled ? "Enabled" : "Disabled")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }

                        Spacer()

                        Image(systemName: biometricService.isEnabled ? "checkmark.circle.fill" : "circle")
                            .foregroundColor(biometricService.isEnabled ? .green : .gray)
                    }
                }

                HStack {
                    Image(systemName: "lock.rotation")
                        .foregroundColor(.blue)
                        .frame(width: 30)

                    VStack(alignment: .leading, spacing: 2) {
                        Text("Auto-Lock")
                        if securityManager.autoLockEnabled {
                            Text(autoLockDescription)
                                .font(.caption)
                                .foregroundColor(.secondary)
                        } else {
                            Text("Disabled")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }
                    }

                    Spacer()

                    Toggle("", isOn: $securityManager.autoLockEnabled)
                        .labelsHidden()
                        .onChange(of: securityManager.autoLockEnabled) { oldValue, newValue in
                            securityManager.toggleAutoLock(newValue)
                        }
                }
            } header: {
                Text("Authentication")
            }

            // Wallet Security
            Section {
                NavigationLink {
                    BackupSecurityView()
                } label: {
                    SecurityActionRow(
                        title: "Backup & Recovery",
                        description: "Secure your wallet backup",
                        icon: "arrow.clockwise.circle.fill",
                        iconColor: .orange
                    )
                }

                NavigationLink {
                    PrivateKeySecurityView()
                } label: {
                    SecurityActionRow(
                        title: "Private Keys",
                        description: "Manage private key access",
                        icon: "key.fill",
                        iconColor: .purple
                    )
                }

                NavigationLink {
                    WalletConnectSecurityView()
                } label: {
                    SecurityActionRow(
                        title: "WalletConnect Sessions",
                        description: "Manage connected dApps",
                        icon: "link.circle.fill",
                        iconColor: .blue
                    )
                }
            } header: {
                Text("Wallet Security")
            }

            // Privacy Settings
            Section {
                Toggle("Hide Balances on Launch", isOn: .constant(false))

                Toggle("Require Auth for Transactions", isOn: $biometricService.requireForTransactions)
                    .onChange(of: biometricService.requireForTransactions) { oldValue, newValue in
                        biometricService.toggleTransactionAuthentication(newValue)
                    }

                Toggle("Secure Screenshots", isOn: .constant(false))
            } header: {
                Text("Privacy")
            } footer: {
                Text("Additional privacy features to protect your wallet.")
            }

            // Advanced Security
            Section {
                Button {
                    showSecurityTips = true
                } label: {
                    HStack {
                        Image(systemName: "info.circle.fill")
                            .foregroundColor(.blue)
                        Text("Security Tips")
                        Spacer()
                        Image(systemName: "chevron.right")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                Button {
                    // Security audit
                } label: {
                    HStack {
                        Image(systemName: "checkmark.seal.fill")
                            .foregroundColor(.green)
                        Text("Security Audit")
                        Spacer()
                        Image(systemName: "chevron.right")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }
            } header: {
                Text("Advanced")
            }

            // Emergency Options
            Section {
                Button(role: .destructive) {
                    // Lock wallet immediately
                    securityManager.lock()
                    HapticService.shared.error()
                } label: {
                    HStack {
                        Image(systemName: "lock.fill")
                        Text("Lock Wallet Now")
                    }
                }
            } header: {
                Text("Emergency")
            }
        }
        .navigationTitle("Security")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Jailbreak Detected", isPresented: $showJailbreakWarning) {
            Button("I Understand", role: .cancel) {}
        } message: {
            Text("Your device appears to be jailbroken. This may compromise the security of your wallet. Please use this wallet on a secure, non-jailbroken device.")
        }
        .sheet(isPresented: $showSecurityTips) {
            SecurityTipsView()
        }
    }

    // MARK: - Computed Properties
    private var currentSecurityLevel: SecurityLevel {
        if biometricService.isEnabled &&
           biometricService.requireForAppLaunch &&
           securityManager.autoLockEnabled {
            return .high
        } else if biometricService.isEnabled {
            return .medium
        } else {
            return .low
        }
    }

    private var securityLevelColor: Color {
        switch currentSecurityLevel {
        case .low:
            return .orange
        case .medium:
            return .yellow
        case .high:
            return .green
        }
    }

    private var autoLockDescription: String {
        let timeout = securityManager.autoLockTimeout
        if timeout == .infinity {
            return "Never"
        } else if timeout < 60 {
            return "\(Int(timeout)) seconds"
        } else if timeout < 3600 {
            return "\(Int(timeout / 60)) minutes"
        } else {
            return "\(Int(timeout / 3600)) hours"
        }
    }
}

// MARK: - Security Status Row
struct SecurityStatusRow: View {
    let title: String
    let isSecure: Bool
    let icon: String

    var body: some View {
        HStack {
            Image(systemName: icon)
                .foregroundColor(isSecure ? .green : .red)
                .frame(width: 30)

            Text(title)

            Spacer()

            Image(systemName: isSecure ? "checkmark.circle.fill" : "xmark.circle.fill")
                .foregroundColor(isSecure ? .green : .red)
        }
    }
}

// MARK: - Security Action Row
struct SecurityActionRow: View {
    let title: String
    let description: String
    let icon: String
    let iconColor: Color

    var body: some View {
        HStack {
            Image(systemName: icon)
                .foregroundColor(iconColor)
                .frame(width: 30)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.body)

                Text(description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

// MARK: - Placeholder Views
struct BackupSecurityView: View {
    var body: some View {
        Text("Backup & Recovery Settings")
            .navigationTitle("Backup & Recovery")
            .navigationBarTitleDisplayMode(.inline)
    }
}

struct PrivateKeySecurityView: View {
    var body: some View {
        Text("Private Key Management")
            .navigationTitle("Private Keys")
            .navigationBarTitleDisplayMode(.inline)
    }
}

struct WalletConnectSecurityView: View {
    var body: some View {
        Text("WalletConnect Sessions")
            .navigationTitle("WalletConnect")
            .navigationBarTitleDisplayMode(.inline)
    }
}

struct SecurityTipsView: View {
    @Environment(\.dismiss) var dismiss

    var body: some View {
        NavigationStack {
            List {
                Section {
                    SecurityTipRow(
                        icon: "shield.checkered",
                        title: "Enable Biometrics",
                        description: "Use Face ID or Touch ID for additional security"
                    )

                    SecurityTipRow(
                        icon: "lock.rotation",
                        title: "Enable Auto-Lock",
                        description: "Automatically lock your wallet when inactive"
                    )

                    SecurityTipRow(
                        icon: "arrow.clockwise.circle",
                        title: "Backup Your Wallet",
                        description: "Keep your recovery phrase safe and offline"
                    )

                    SecurityTipRow(
                        icon: "network.slash",
                        title: "Be Careful with Public WiFi",
                        description: "Avoid sensitive transactions on public networks"
                    )

                    SecurityTipRow(
                        icon: "checkmark.seal",
                        title: "Verify Addresses",
                        description: "Always double-check recipient addresses"
                    )

                    SecurityTipRow(
                        icon: "exclamationmark.triangle",
                        title: "Watch for Phishing",
                        description: "Never share your recovery phrase or private keys"
                    )
                }
            }
            .navigationTitle("Security Tips")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

struct SecurityTipRow: View {
    let icon: String
    let title: String
    let description: String

    var body: some View {
        HStack(alignment: .top, spacing: 12) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(.blue)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.headline)

                Text(description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 4)
    }
}

#Preview {
    NavigationStack {
        SecuritySettingsView()
    }
}
