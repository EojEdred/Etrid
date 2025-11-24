//
//  CreateMultiSigView.swift
//  EtridWallet
//
//  Multi-sig wallet creation wizard
//

import SwiftUI

struct CreateMultiSigView: View {
    @StateObject private var service = MultiSigService.shared
    @Environment(\.dismiss) private var dismiss

    @State private var name = ""
    @State private var description = ""
    @State private var owners: [String] = []
    @State private var threshold = 2
    @State private var newOwnerAddress = ""
    @State private var chainId = "1"

    @State private var isCreating = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationView {
            Form {
                Section("Wallet Information") {
                    TextField("Name", text: $name)
                    TextField("Description (optional)", text: $description)
                }

                Section {
                    ForEach(Array(owners.enumerated()), id: \.offset) { index, owner in
                        HStack {
                            VStack(alignment: .leading) {
                                Text("Owner \(index + 1)")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                                Text(owner)
                                    .font(.caption)
                                    .lineLimit(1)
                            }
                            Spacer()
                            Button(action: { owners.remove(at: index) }) {
                                Image(systemName: "minus.circle.fill")
                                    .foregroundColor(.red)
                            }
                        }
                    }

                    HStack {
                        TextField("Owner Address", text: $newOwnerAddress)
                            .autocapitalization(.none)
                        Button(action: addOwner) {
                            Image(systemName: "plus.circle.fill")
                                .foregroundColor(.green)
                        }
                        .disabled(newOwnerAddress.isEmpty)
                    }
                } header: {
                    Text("Owners (\(owners.count))")
                } footer: {
                    Text("Add Ethereum addresses for wallet owners")
                }

                Section {
                    Stepper("Required Signatures: \(threshold)", value: $threshold, in: 1...max(1, owners.count))

                    Text("\(threshold) out of \(owners.count) owners must approve transactions")
                        .font(.caption)
                        .foregroundColor(.secondary)
                } header: {
                    Text("Signature Threshold")
                }

                Section {
                    Picker("Network", selection: $chainId) {
                        Text("Ethereum").tag("1")
                        Text("Polygon").tag("137")
                        Text("Arbitrum").tag("42161")
                    }
                }

                if let error = errorMessage {
                    Section {
                        Text(error)
                            .foregroundColor(.red)
                            .font(.caption)
                    }
                }
            }
            .navigationTitle("Create Multi-Sig")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Create") {
                        createWallet()
                    }
                    .disabled(!isValid || isCreating)
                }
            }
            .overlay {
                if isCreating {
                    ProgressView("Creating Wallet...")
                        .padding()
                        .background(Color(.systemBackground))
                        .cornerRadius(12)
                        .shadow(radius: 10)
                }
            }
            .task {
                // Add current user's address as first owner
                do {
                    let currentAddress = try await getCurrentAddress()
                    if !owners.contains(currentAddress) {
                        owners.insert(currentAddress, at: 0)
                    }
                } catch {
                    print("Error getting address: \(error)")
                }
            }
        }
    }

    private var isValid: Bool {
        let params = CreateMultiSigParams(
            name: name,
            description: description.isEmpty ? nil : description,
            owners: owners,
            threshold: threshold,
            chainId: chainId,
            saltNonce: nil
        )
        return params.isValid
    }

    private func addOwner() {
        guard !newOwnerAddress.isEmpty else { return }
        owners.append(newOwnerAddress)
        newOwnerAddress = ""

        // Adjust threshold if needed
        if threshold > owners.count {
            threshold = owners.count
        }
    }

    private func createWallet() {
        let params = CreateMultiSigParams(
            name: name,
            description: description.isEmpty ? nil : description,
            owners: owners,
            threshold: threshold,
            chainId: chainId,
            saltNonce: nil
        )

        guard params.isValid else {
            errorMessage = params.validationError
            return
        }

        isCreating = true
        errorMessage = nil

        Task {
            do {
                _ = try await service.createWallet(params)
                await MainActor.run {
                    dismiss()
                }
            } catch {
                await MainActor.run {
                    errorMessage = error.localizedDescription
                    isCreating = false
                }
            }
        }
    }

    private func getCurrentAddress() async throws -> String {
        // TODO: Get from WalletManager
        return "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
    }
}

#Preview {
    CreateMultiSigView()
}
