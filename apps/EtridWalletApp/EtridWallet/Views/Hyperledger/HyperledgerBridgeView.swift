//
//  HyperledgerBridgeView.swift
//  EtridWallet
//
//  Hyperledger Fabric Bridge Main View
//

import SwiftUI

struct HyperledgerBridgeView: View {
    @StateObject private var service = HyperledgerBridgeService()
    @State private var selectedNetwork: FabricNetwork?
    @State private var bridgeDirection: BridgeDirection = .toFabric
    @State private var selectedToken = "ETR"
    @State private var amount = ""
    @State private var recipient = ""
    @State private var showNetworkSelector = false
    @State private var showHistory = false
    @State private var isBridging = false
    @State private var bridgeSuccess = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // Network Selection
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Network")
                            .font(.headline)

                        Button(action: { showNetworkSelector = true }) {
                            HStack {
                                if let network = selectedNetwork {
                                    VStack(alignment: .leading, spacing: 4) {
                                        Text(network.name)
                                            .font(.subheadline)
                                            .fontWeight(.medium)
                                            .foregroundColor(.primary)
                                        HStack {
                                            Circle()
                                                .fill(Color(hex: network.status.color) ?? .gray)
                                                .frame(width: 8, height: 8)
                                            Text(network.statusText)
                                                .font(.caption)
                                                .foregroundColor(.gray)
                                        }
                                    }
                                } else {
                                    Text("Select Network")
                                        .foregroundColor(.gray)
                                }

                                Spacer()
                                Image(systemName: "chevron.right")
                                    .foregroundColor(.gray)
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                    }

                    // Direction Selector
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Direction")
                            .font(.headline)

                        HStack(spacing: 12) {
                            DirectionButton(
                                icon: "arrow.right.circle.fill",
                                title: "To Fabric",
                                subtitle: "Lock on Etrid",
                                direction: .toFabric,
                                selectedDirection: $bridgeDirection
                            )

                            DirectionButton(
                                icon: "arrow.left.circle.fill",
                                title: "From Fabric",
                                subtitle: "Burn on Fabric",
                                direction: .fromFabric,
                                selectedDirection: $bridgeDirection
                            )
                        }
                    }

                    // Bridge Form
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Token")
                            .font(.headline)

                        Menu {
                            Button("ETR") { selectedToken = "ETR" }
                            Button("USDT") { selectedToken = "USDT" }
                            Button("USDC") { selectedToken = "USDC" }
                        } label: {
                            HStack {
                                Text(selectedToken)
                                    .fontWeight(.medium)
                                Spacer()
                                Image(systemName: "chevron.down")
                            }
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                        }
                    }

                    VStack(alignment: .leading, spacing: 12) {
                        Text("Amount")
                            .font(.headline)

                        TextField("0.0", text: $amount)
                            .keyboardType(.decimalPad)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                    }

                    VStack(alignment: .leading, spacing: 12) {
                        Text("Recipient Address")
                            .font(.headline)

                        TextField("0x...", text: $recipient)
                            .autocapitalization(.none)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)
                    }

                    // Bridge Process Visualization
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Bridge Process")
                            .font(.headline)

                        VStack(spacing: 16) {
                            BridgeStepView(number: 1, title: "Lock/Burn", description: bridgeDirection == .toFabric ? "Lock assets on Etrid" : "Burn assets on Fabric", isActive: true)
                            BridgeStepView(number: 2, title: "Generate Proof", description: "Create cryptographic proof", isActive: false)
                            BridgeStepView(number: 3, title: "Verify", description: "Endorsers verify transaction", isActive: false)
                            BridgeStepView(number: 4, title: "Complete", description: bridgeDirection == .toFabric ? "Mint on Fabric" : "Unlock on Etrid", isActive: false)
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Error Message
                    if let error = errorMessage {
                        Text(error)
                            .font(.subheadline)
                            .foregroundColor(.red)
                            .padding()
                            .background(Color.red.opacity(0.1))
                            .cornerRadius(8)
                    }

                    // Bridge Button
                    Button(action: executeBridge) {
                        if isBridging {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            HStack {
                                Image(systemName: "arrow.left.arrow.right")
                                Text("Bridge Assets")
                            }
                            .font(.headline)
                        }
                    }
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(canBridge ? Color.blue : Color.gray)
                    .cornerRadius(12)
                    .disabled(!canBridge || isBridging)
                }
                .padding()
            }
            .navigationTitle("Hyperledger Bridge")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showHistory = true }) {
                        Image(systemName: "clock.arrow.circlepath")
                    }
                }
            }
            .sheet(isPresented: $showNetworkSelector) {
                NetworkSelectorView(selectedNetwork: $selectedNetwork, service: service)
            }
            .sheet(isPresented: $showHistory) {
                BridgeHistoryView()
            }
            .alert(isPresented: $bridgeSuccess) {
                Alert(
                    title: Text("Bridge Initiated"),
                    message: Text("Your bridge transaction has been initiated. Track progress in Bridge History."),
                    dismissButton: .default(Text("OK"))
                )
            }
        }
    }

    private var canBridge: Bool {
        selectedNetwork != nil &&
        !amount.isEmpty &&
        Double(amount) != nil &&
        !recipient.isEmpty
    }

    private func executeBridge() {
        guard canBridge, let network = selectedNetwork else { return }

        isBridging = true
        errorMessage = nil

        Task {
            do {
                if bridgeDirection == .toFabric {
                    _ = try await service.bridgeToFabric(
                        networkId: network.id,
                        token: selectedToken,
                        amount: amount,
                        recipient: recipient
                    )
                } else {
                    // For from Fabric, we need proof - simplified for now
                    _ = try await service.bridgeFromFabric(
                        networkId: network.id,
                        token: selectedToken,
                        amount: amount,
                        recipient: recipient,
                        fabricTxId: "sample-tx-id",
                        proof: "sample-proof"
                    )
                }
                bridgeSuccess = true
                // Reset form
                amount = ""
                recipient = ""
            } catch {
                errorMessage = error.localizedDescription
            }
            isBridging = false
        }
    }
}

// MARK: - Direction Button
struct DirectionButton: View {
    let icon: String
    let title: String
    let subtitle: String
    let direction: BridgeDirection
    @Binding var selectedDirection: BridgeDirection

    var isSelected: Bool {
        selectedDirection == direction
    }

    var body: some View {
        Button(action: {
            selectedDirection = direction
        }) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.title)
                    .foregroundColor(isSelected ? .blue : .gray)

                Text(title)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(.primary)

                Text(subtitle)
                    .font(.caption)
                    .foregroundColor(.gray)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(isSelected ? Color.blue.opacity(0.1) : Color(.systemGray6))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color.blue : Color.clear, lineWidth: 2)
            )
        }
    }
}

// MARK: - Bridge Step View
struct BridgeStepView: View {
    let number: Int
    let title: String
    let description: String
    let isActive: Bool

    var body: some View {
        HStack(spacing: 12) {
            ZStack {
                Circle()
                    .fill(isActive ? Color.blue : Color.gray.opacity(0.3))
                    .frame(width: 32, height: 32)

                Text("\(number)")
                    .font(.subheadline)
                    .fontWeight(.bold)
                    .foregroundColor(.white)
            }

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.subheadline)
                    .fontWeight(.medium)
                Text(description)
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            Spacer()
        }
    }
}

struct HyperledgerBridgeView_Previews: PreviewProvider {
    static var previews: some View {
        HyperledgerBridgeView()
    }
}
