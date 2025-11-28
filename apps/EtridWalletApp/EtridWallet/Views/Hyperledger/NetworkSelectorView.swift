//
//  NetworkSelectorView.swift
//  EtridWallet
//
//  Hyperledger Network Selector
//

import SwiftUI

struct NetworkSelectorView: View {
    @Binding var selectedNetwork: FabricNetwork?
    @ObservedObject var service: HyperledgerBridgeService
    @State private var networks: [FabricNetwork] = []
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        NavigationView {
            VStack {
                if service.isLoading {
                    Spacer()
                    ProgressView("Loading networks...")
                    Spacer()
                } else if networks.isEmpty {
                    Spacer()
                    VStack(spacing: 16) {
                        Image(systemName: "network")
                            .font(.system(size: 60))
                            .foregroundColor(.gray)
                        Text("No Networks Available")
                            .font(.title2)
                            .foregroundColor(.gray)
                    }
                    Spacer()
                } else {
                    ScrollView {
                        LazyVStack(spacing: 12) {
                            ForEach(networks) { network in
                                NetworkCardView(
                                    network: network,
                                    isSelected: selectedNetwork?.id == network.id,
                                    onSelect: {
                                        selectedNetwork = network
                                        connectToNetwork(network)
                                    }
                                )
                            }
                        }
                        .padding()
                    }
                }
            }
            .navigationTitle("Select Network")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
            .onAppear(perform: loadNetworks)
        }
    }

    private func loadNetworks() {
        Task {
            do {
                networks = try await service.getAvailableNetworks()
            } catch {
                print("Error loading networks: \(error)")
            }
        }
    }

    private func connectToNetwork(_ network: FabricNetwork) {
        Task {
            do {
                _ = try await service.connectToNetwork(networkId: network.id)
                presentationMode.wrappedValue.dismiss()
            } catch {
                print("Error connecting to network: \(error)")
            }
        }
    }
}

// MARK: - Network Card View
struct NetworkCardView: View {
    let network: FabricNetwork
    let isSelected: Bool
    let onSelect: () -> Void

    var statusColor: Color {
        Color(hex: network.status.color) ?? .gray
    }

    var body: some View {
        Button(action: onSelect) {
            VStack(alignment: .leading, spacing: 12) {
                // Header
                HStack {
                    Text(network.name)
                        .font(.headline)
                        .foregroundColor(.primary)

                    Spacer()

                    HStack(spacing: 6) {
                        Circle()
                            .fill(statusColor)
                            .frame(width: 8, height: 8)
                        Text(network.statusText)
                            .font(.caption)
                            .foregroundColor(statusColor)
                    }
                }

                // Description
                Text(network.description)
                    .font(.subheadline)
                    .foregroundColor(.gray)
                    .lineLimit(2)

                // Network Info
                HStack(spacing: 16) {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Channel")
                            .font(.caption)
                            .foregroundColor(.gray)
                        Text(network.channelName)
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    Divider()
                        .frame(height: 30)

                    VStack(alignment: .leading, spacing: 4) {
                        Text("Endpoints")
                            .font(.caption)
                            .foregroundColor(.gray)
                        Text("\(network.endpointCount)")
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }

                    Divider()
                        .frame(height: 30)

                    VStack(alignment: .leading, spacing: 4) {
                        Text("Tokens")
                            .font(.caption)
                            .foregroundColor(.gray)
                        Text("\(network.supportedTokens.count)")
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }
                }

                // Selection Indicator
                if isSelected {
                    HStack {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.blue)
                        Text("Connected")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(.blue)
                    }
                }
            }
            .padding()
            .background(Color(.systemBackground))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color.blue : Color.gray.opacity(0.2), lineWidth: isSelected ? 2 : 1)
            )
            .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
        }
    }
}
