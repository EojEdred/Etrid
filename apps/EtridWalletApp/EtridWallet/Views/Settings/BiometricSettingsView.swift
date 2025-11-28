//
//  BiometricSettingsView.swift
//  EtridWallet
//
//  Biometric authentication settings
//

import SwiftUI

struct BiometricSettingsView: View {
    @StateObject private var biometricService = BiometricService.shared
    @StateObject private var securityManager = SecurityManager.shared
    @State private var showAuthError = false
    @State private var authErrorMessage = ""

    var body: some View {
        List {
            // Biometric Info Section
            Section {
                HStack {
                    Image(systemName: biometricService.biometricIcon)
                        .font(.title2)
                        .foregroundColor(.blue)
                        .frame(width: 40)

                    VStack(alignment: .leading, spacing: 4) {
                        Text(biometricService.biometricName)
                            .font(.headline)

                        if biometricService.isAvailable {
                            Text("Available")
                                .font(.caption)
                                .foregroundColor(.green)
                        } else {
                            Text("Not Available")
                                .font(.caption)
                                .foregroundColor(.red)
                        }
                    }

                    Spacer()

                    Toggle("", isOn: $biometricService.isEnabled)
                        .labelsHidden()
                        .disabled(!biometricService.isAvailable)
                        .onChange(of: biometricService.isEnabled) { oldValue, newValue in
                            biometricService.toggleBiometrics(newValue)
                        }
                }
            } header: {
                Text("Biometric Authentication")
            } footer: {
                if !biometricService.isAvailable {
                    Text("Biometric authentication is not available on this device. Please set up \(biometricService.biometricName) in Settings.")
                }
            }

            // Authentication Requirements
            if biometricService.isEnabled {
                Section {
                    Toggle("Require for App Launch", isOn: $biometricService.requireForAppLaunch)
                        .onChange(of: biometricService.requireForAppLaunch) { oldValue, newValue in
                            biometricService.toggleAppLaunchAuthentication(newValue)
                        }

                    Toggle("Require for Transactions", isOn: $biometricService.requireForTransactions)
                        .onChange(of: biometricService.requireForTransactions) { oldValue, newValue in
                            biometricService.toggleTransactionAuthentication(newValue)
                        }

                    Toggle("Require for Sensitive Data", isOn: $biometricService.requireForSensitiveData)
                        .onChange(of: biometricService.requireForSensitiveData) { oldValue, newValue in
                            biometricService.toggleSensitiveDataAuthentication(newValue)
                        }
                } header: {
                    Text("Authentication Requirements")
                } footer: {
                    Text("Choose when biometric authentication is required.")
                }
            }

            // Auto-Lock Settings
            Section {
                Toggle("Auto-Lock", isOn: $securityManager.autoLockEnabled)
                    .onChange(of: securityManager.autoLockEnabled) { oldValue, newValue in
                        securityManager.toggleAutoLock(newValue)
                    }

                if securityManager.autoLockEnabled {
                    Picker("Auto-Lock Timeout", selection: Binding(
                        get: { securityManager.autoLockTimeout },
                        set: { securityManager.setAutoLockTimeout($0) }
                    )) {
                        Text("30 seconds").tag(30.0)
                        Text("1 minute").tag(60.0)
                        Text("2 minutes").tag(120.0)
                        Text("5 minutes").tag(300.0)
                        Text("10 minutes").tag(600.0)
                        Text("Never").tag(Double.infinity)
                    }
                }
            } header: {
                Text("Auto-Lock")
            } footer: {
                Text("Automatically lock the app after a period of inactivity.")
            }

            // Test Authentication
            Section {
                Button {
                    Task {
                        await testAuthentication()
                    }
                } label: {
                    HStack {
                        Image(systemName: biometricService.biometricIcon)
                        Text("Test \(biometricService.biometricName)")
                    }
                }
                .disabled(!biometricService.isAvailable)
            } header: {
                Text("Test")
            }

            // Security Level Indicator
            Section {
                HStack {
                    Text("Security Level")
                    Spacer()
                    Text(currentSecurityLevel.description)
                        .foregroundColor(securityLevelColor)
                }
            }
        }
        .navigationTitle("Biometric Settings")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Authentication Error", isPresented: $showAuthError) {
            Button("OK", role: .cancel) {}
        } message: {
            Text(authErrorMessage)
        }
    }

    // MARK: - Helper Methods
    private func testAuthentication() async {
        do {
            try await biometricService.authenticate(reason: "Test authentication")
            HapticService.shared.success()
        } catch {
            authErrorMessage = error.localizedDescription
            showAuthError = true
            HapticService.shared.error()
        }
    }

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
}

#Preview {
    NavigationStack {
        BiometricSettingsView()
    }
}
