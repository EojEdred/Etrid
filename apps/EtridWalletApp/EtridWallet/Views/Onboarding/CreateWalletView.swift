//
//  CreateWalletView.swift
//  EtridWallet
//
//  Create new wallet screen
//

import SwiftUI

struct CreateWalletView: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var viewModel = CreateWalletViewModel()

    var body: some View {
        ScrollView {
            VStack(spacing: 30) {
                // Header
                VStack(spacing: 12) {
                    Image(systemName: "plus.circle.fill")
                        .font(.system(size: 60))
                        .foregroundColor(.etridBlue)

                    Text("Create New Wallet")
                        .font(.largeTitle)
                        .fontWeight(.bold)

                    Text("Generate a new wallet with a secure recovery phrase")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal)
                }
                .padding(.top, 20)

                if viewModel.isGenerating {
                    ProgressView("Generating secure keys...")
                        .padding()
                } else if let mnemonic = viewModel.mnemonic {
                    // Show mnemonic
                    VStack(spacing: 20) {
                        // Warning
                        HStack(spacing: 12) {
                            Image(systemName: "exclamationmark.triangle.fill")
                                .foregroundColor(.orange)

                            Text("Write down these words in order. You'll need them to recover your wallet.")
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                        }
                        .padding()
                        .background(Color.orange.opacity(0.1))
                        .cornerRadius(12)

                        // Mnemonic grid
                        LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
                            ForEach(Array(mnemonic.enumerated()), id: \.offset) { index, word in
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

                        // Confirmation checkbox
                        Toggle(isOn: $viewModel.hasWrittenDown) {
                            Text("I have written down my recovery phrase")
                                .font(.subheadline)
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)

                        // Create button
                        Button(action: {
                            Task {
                                await viewModel.createWallet()
                            }
                        }) {
                            Text("Create Wallet")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(viewModel.hasWrittenDown ? Color.etridBlue : Color.gray)
                                .cornerRadius(16)
                        }
                        .disabled(!viewModel.hasWrittenDown || viewModel.isCreating)

                        if viewModel.isCreating {
                            ProgressView("Creating wallet...")
                        }
                    }
                } else {
                    // Generate button
                    VStack(spacing: 20) {
                        Image(systemName: "key.fill")
                            .font(.system(size: 80))
                            .foregroundColor(.etridBlue.opacity(0.3))

                        Button(action: {
                            Task {
                                await viewModel.generateMnemonic()
                            }
                        }) {
                            Text("Generate Recovery Phrase")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.etridBlue)
                                .cornerRadius(16)
                        }
                    }
                    .padding(.top, 40)
                }

                if let error = viewModel.error {
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                        .padding()
                        .background(Color.red.opacity(0.1))
                        .cornerRadius(8)
                }
            }
            .padding()
            .padding(.bottom, 50)
        }
        .navigationBarTitleDisplayMode(.inline)
        .alert("Wallet Created!", isPresented: $viewModel.showSuccess) {
            Button("Continue") {
                dismiss()
            }
        } message: {
            Text("Your wallet has been created successfully!")
        }
    }
}

@MainActor
class CreateWalletViewModel: ObservableObject {
    @Published var mnemonic: [String]?
    @Published var hasWrittenDown = false
    @Published var isGenerating = false
    @Published var isCreating = false
    @Published var error: String?
    @Published var showSuccess = false

    func generateMnemonic() async {
        isGenerating = true
        error = nil

        do {
            let phrase = try await CryptoManager.shared.generateMnemonic(wordCount: 12)
            mnemonic = phrase.components(separatedBy: " ")
        } catch {
            self.error = error.localizedDescription
        }

        isGenerating = false
    }

    func createWallet() async {
        guard let words = mnemonic else { return }

        isCreating = true
        error = nil

        do {
            let phrase = words.joined(separator: " ")

            // Import wallet from mnemonic
            let (privateKey, address) = try await CryptoManager.shared.importWallet(mnemonic: phrase)

            // Save to storage
            let account = WalletAccount(
                name: "Account 1",
                address: address,
                isHD: true,
                derivationPath: "m/44'/60'/0'/0/0"
            )

            try await WalletStorage.shared.saveAccounts([account])
            try await WalletStorage.shared.savePrivateKey(privateKey, for: address)
            try await WalletStorage.shared.saveMnemonic(phrase)

            // Notify that wallet was created
            NotificationCenter.default.post(name: .walletCreated, object: nil)

            showSuccess = true
        } catch {
            self.error = error.localizedDescription
        }

        isCreating = false
    }
}

#Preview {
    NavigationStack {
        CreateWalletView()
    }
}
