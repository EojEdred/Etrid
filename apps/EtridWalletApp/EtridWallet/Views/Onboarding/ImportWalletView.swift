//
//  ImportWalletView.swift
//  EtridWallet
//
//  Import existing wallet screen
//

import SwiftUI

struct ImportWalletView: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel = ImportWalletViewModel()

    var body: some View {
        ScrollView {
            VStack(spacing: 30) {
                // Header
                VStack(spacing: 12) {
                    Image(systemName: "arrow.down.circle.fill")
                        .font(.system(size: 60))
                        .foregroundColor(.etridBlue)

                    Text("Import Wallet")
                        .font(.largeTitle)
                        .fontWeight(.bold)

                    Text("Enter your 12 or 24 word recovery phrase")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
                .padding(.top, 20)

                // Import method selector
                Picker("Import Method", selection: $viewModel.importMethod) {
                    Text("Recovery Phrase").tag(ImportMethod.mnemonic)
                    Text("Private Key").tag(ImportMethod.privateKey)
                }
                .pickerStyle(.segmented)

                // Input area
                VStack(spacing: 16) {
                    if viewModel.importMethod == .mnemonic {
                        // Mnemonic input
                        TextEditor(text: $viewModel.mnemonicInput)
                            .frame(height: 150)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                            .autocorrectionDisabled()
                            .textInputAutocapitalization(.never)

                        Text("\(viewModel.wordCount) words")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    } else {
                        // Private key input
                        TextField("Enter private key (0x...)", text: $viewModel.privateKeyInput)
                            .textFieldStyle(.roundedBorder)
                            .autocorrectionDisabled()
                            .textInputAutocapitalization(.never)
                            .padding(.horizontal)
                    }
                }

                // Import button
                Button(action: {
                    Task {
                        await viewModel.importWallet()
                    }
                }) {
                    if viewModel.isImporting {
                        ProgressView()
                            .tint(.white)
                    } else {
                        Text("Import Wallet")
                            .font(.headline)
                    }
                }
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(viewModel.canImport ? Color.etridBlue : Color.gray)
                .cornerRadius(16)
                .disabled(!viewModel.canImport || viewModel.isImporting)

                if let error = viewModel.error {
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                        .padding()
                        .background(Color.red.opacity(0.1))
                        .cornerRadius(8)
                }

                // Tips
                VStack(alignment: .leading, spacing: 12) {
                    Text("Tips:")
                        .font(.headline)

                    TipRow(icon: "checkmark.circle.fill", text: "Make sure words are spelled correctly")
                    TipRow(icon: "checkmark.circle.fill", text: "Words should be separated by spaces")
                    TipRow(icon: "checkmark.circle.fill", text: "Order matters")
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(12)

                Spacer()
            }
            .padding()
        }
        .navigationBarTitleDisplayMode(.inline)
        .alert("Wallet Imported!", isPresented: $viewModel.showSuccess) {
            Button("Continue") {
                dismiss()
            }
        } message: {
            Text("Your wallet has been imported successfully!")
        }
    }
}

struct TipRow: View {
    let icon: String
    let text: String

    var body: some View {
        HStack(spacing: 10) {
            Image(systemName: icon)
                .foregroundColor(.etridGreen)
                .font(.caption)

            Text(text)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
}

enum ImportMethod {
    case mnemonic
    case privateKey
}

@MainActor
class ImportWalletViewModel: ObservableObject {
    @Published var importMethod: ImportMethod = .mnemonic
    @Published var mnemonicInput = ""
    @Published var privateKeyInput = ""
    @Published var isImporting = false
    @Published var error: String?
    @Published var showSuccess = false

    var wordCount: Int {
        mnemonicInput.trimmingCharacters(in: .whitespacesAndNewlines)
            .components(separatedBy: .whitespaces)
            .filter { !$0.isEmpty }
            .count
    }

    var canImport: Bool {
        if importMethod == .mnemonic {
            let count = wordCount
            return [12, 24].contains(count)
        } else {
            return !privateKeyInput.isEmpty && privateKeyInput.isValidEthereumAddress
        }
    }

    func importWallet() async {
        isImporting = true
        error = nil

        do {
            if importMethod == .mnemonic {
                try await importFromMnemonic()
            } else {
                try await importFromPrivateKey()
            }

            // Notify that wallet was created/imported
            NotificationCenter.default.post(name: .walletCreated, object: nil)

            showSuccess = true
        } catch {
            self.error = error.localizedDescription
        }

        isImporting = false
    }

    private func importFromMnemonic() async throws {
        let mnemonic = mnemonicInput.trimmingCharacters(in: .whitespacesAndNewlines)

        guard await CryptoManager.shared.validateMnemonic(mnemonic) else {
            throw WalletError.invalidMnemonic
        }

        let (privateKey, address) = try await CryptoManager.shared.importWallet(mnemonic: mnemonic)

        let account = WalletAccount(
            name: "Imported Account",
            address: address,
            isHD: true,
            derivationPath: "m/44'/60'/0'/0/0"
        )

        try await WalletStorage.shared.saveAccounts([account])
        try await WalletStorage.shared.savePrivateKey(privateKey, for: address)
        try await WalletStorage.shared.saveMnemonic(mnemonic)
    }

    private func importFromPrivateKey() async throws {
        let privateKeyHex = privateKeyInput.removeHexPrefix

        guard let privateKey = privateKeyHex.hexToData else {
            throw WalletError.invalidPrivateKey
        }

        let address = try await CryptoManager.shared.generateAddress(from: privateKey)

        let account = WalletAccount(
            name: "Imported Account",
            address: address,
            isHD: false
        )

        try await WalletStorage.shared.saveAccounts([account])
        try await WalletStorage.shared.savePrivateKey(privateKey, for: address)
    }
}

#Preview {
    NavigationStack {
        ImportWalletView()
    }
}
