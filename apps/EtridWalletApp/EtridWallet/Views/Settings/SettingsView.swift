//
//  SettingsView.swift
//  EtridWallet
//
//  Settings screen
//

import SwiftUI

struct SettingsView: View {
    @StateObject private var viewModel = SettingsViewModel()
    @State private var showBackupPhrase = false
    @State private var showResetConfirmation = false

    var body: some View {
        List {
            // Security Section
            Section("Security") {
                Toggle(isOn: $viewModel.biometricsEnabled) {
                    Label("Biometric Authentication", systemImage: "faceid")
                }
                .onChange(of: viewModel.biometricsEnabled) { _, newValue in
                    viewModel.toggleBiometrics(newValue)
                }

                Toggle(isOn: $viewModel.autoLockEnabled) {
                    Label("Auto Lock", systemImage: "lock.fill")
                }
                .onChange(of: viewModel.autoLockEnabled) { _, newValue in
                    viewModel.toggleAutoLock(newValue)
                }

                if viewModel.autoLockEnabled {
                    Picker("Auto Lock Timeout", selection: $viewModel.autoLockTimeout) {
                        Text("1 minute").tag(60)
                        Text("5 minutes").tag(300)
                        Text("15 minutes").tag(900)
                        Text("30 minutes").tag(1800)
                    }
                }

                Button(action: { showBackupPhrase = true }) {
                    Label("Backup Recovery Phrase", systemImage: "key.fill")
                }
            }

            // Network Section
            Section("Network") {
                Toggle(isOn: $viewModel.showTestnets) {
                    Label("Show Testnets", systemImage: "network")
                }
                .onChange(of: viewModel.showTestnets) { _, newValue in
                    viewModel.toggleTestnets(newValue)
                }
            }

            // About Section
            Section("About") {
                HStack {
                    Text("Version")
                    Spacer()
                    Text("1.0.0")
                        .foregroundColor(.secondary)
                }

                Button(action: {}) {
                    Label("Terms of Service", systemImage: "doc.text")
                }

                Button(action: {}) {
                    Label("Privacy Policy", systemImage: "hand.raised")
                }
            }

            // Danger Zone
            Section("Danger Zone") {
                Button(role: .destructive, action: { showResetConfirmation = true }) {
                    Label("Reset Wallet", systemImage: "trash")
                }
            }
        }
        .navigationTitle("Settings")
        .navigationBarTitleDisplayMode(.inline)
        .sheet(isPresented: $showBackupPhrase) {
            BackupPhraseView()
        }
        .alert("Reset Wallet", isPresented: $showResetConfirmation) {
            Button("Cancel", role: .cancel) {}
            Button("Reset", role: .destructive) {
                viewModel.resetWallet()
            }
        } message: {
            Text("This will delete all wallet data. Make sure you have backed up your recovery phrase!")
        }
    }
}

struct BackupPhraseView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var mnemonic: String?
    @State private var isRevealed = false
    @State private var isAuthenticated = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 20) {
                if !isAuthenticated {
                    VStack(spacing: 20) {
                        Image(systemName: "lock.shield.fill")
                            .font(.system(size: 60))
                            .foregroundColor(.etridBlue)

                        Text("Authenticate to view your recovery phrase")
                            .font(.headline)
                            .multilineTextAlignment(.center)

                        Button(action: authenticate) {
                            Text("Authenticate")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.etridBlue)
                                .cornerRadius(16)
                        }
                    }
                    .padding()
                } else if let words = mnemonic?.components(separatedBy: " ") {
                    ScrollView {
                        VStack(spacing: 20) {
                            // Warning
                            HStack(spacing: 12) {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundColor(.orange)

                                Text("Never share your recovery phrase with anyone!")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)
                            }
                            .padding()
                            .background(Color.orange.opacity(0.1))
                            .cornerRadius(12)

                            // Mnemonic
                            if isRevealed {
                                LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
                                    ForEach(Array(words.enumerated()), id: \.offset) { index, word in
                                        HStack {
                                            Text("\(index + 1).")
                                                .font(.caption)
                                                .foregroundColor(.secondary)
                                                .frame(width: 25, alignment: .trailing)

                                            Text(word)
                                                .font(.body)
                                                .fontWeight(.medium)

                                            Spacer()
                                        }
                                        .padding()
                                        .background(Color(.systemGray6))
                                        .cornerRadius(8)
                                    }
                                }
                            } else {
                                Button(action: { isRevealed = true }) {
                                    Text("Tap to Reveal")
                                        .font(.headline)
                                        .foregroundColor(.white)
                                        .frame(maxWidth: .infinity)
                                        .padding()
                                        .background(Color.etridBlue)
                                        .cornerRadius(16)
                                }
                            }
                        }
                        .padding()
                    }
                }
            }
            .navigationTitle("Recovery Phrase")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
        .task {
            await loadMnemonic()
        }
    }

    func authenticate() {
        Task {
            do {
                try await SecurityManager.shared.authenticateWithBiometrics()
                await MainActor.run {
                    isAuthenticated = true
                }
            } catch {
                ErrorLogger.log(error, context: "Authenticate for backup")
            }
        }
    }

    func loadMnemonic() async {
        do {
            mnemonic = try await WalletStorage.shared.loadMnemonic()
        } catch {
            ErrorLogger.log(error, context: "Load mnemonic")
        }
    }
}

@MainActor
class SettingsViewModel: ObservableObject {
    @Published var biometricsEnabled = true
    @Published var autoLockEnabled = true
    @Published var autoLockTimeout = 300
    @Published var showTestnets = false

    init() {
        loadSettings()
    }

    func loadSettings() {
        let security = SecurityManager.shared
        biometricsEnabled = security.biometricsEnabled
        autoLockEnabled = security.autoLockEnabled
        autoLockTimeout = Int(security.autoLockTimeout)
    }

    func toggleBiometrics(_ enabled: Bool) {
        SecurityManager.shared.toggleBiometrics(enabled)
    }

    func toggleAutoLock(_ enabled: Bool) {
        SecurityManager.shared.toggleAutoLock(enabled)
    }

    func toggleTestnets(_ enabled: Bool) {
        NetworkManager.shared.toggleTestnets(enabled)
    }

    func resetWallet() {
        Task {
            do {
                try await WalletStorage.shared.clearAllData()
            } catch {
                ErrorLogger.log(error, context: "Reset wallet")
            }
        }
    }
}

#Preview {
    NavigationStack {
        SettingsView()
    }
}
