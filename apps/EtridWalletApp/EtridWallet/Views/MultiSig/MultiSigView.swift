//
//  MultiSigView.swift
//  EtridWallet
//
//  Multi-signature wallet management
//

import SwiftUI

struct MultiSigView: View {
    @StateObject private var service = MultiSigService.shared
    @State private var showCreateWallet = false

    var body: some View {
        NavigationView {
            List {
                if service.wallets.isEmpty {
                    EmptyMultiSigView {
                        showCreateWallet = true
                    }
                } else {
                    ForEach(service.wallets) { wallet in
                        NavigationLink {
                            MultiSigWalletDetailView(wallet: wallet)
                        } label: {
                            MultiSigWalletRow(wallet: wallet)
                        }
                    }
                }
            }
            .navigationTitle("Multi-Sig Wallets")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showCreateWallet = true }) {
                        Image(systemName: "plus")
                    }
                }
            }
            .sheet(isPresented: $showCreateWallet) {
                CreateMultiSigView()
            }
        }
    }
}

struct MultiSigWalletRow: View {
    let wallet: MultiSigWallet

    var body: some View {
        HStack(spacing: 12) {
            // Icon
            Circle()
                .fill(Color.blue.opacity(0.1))
                .frame(width: 50, height: 50)
                .overlay(
                    Image(systemName: "person.2.badge.key")
                        .foregroundColor(.blue)
                )

            // Info
            VStack(alignment: .leading, spacing: 4) {
                Text(wallet.name)
                    .font(.headline)

                Text(wallet.thresholdDescription + " signatures")
                    .font(.caption)
                    .foregroundColor(.secondary)

                HStack(spacing: 12) {
                    Label(wallet.balance + " ETH", systemImage: "banknote")
                    if wallet.pendingTransactionCount > 0 {
                        Label("\(wallet.pendingTransactionCount)", systemImage: "clock.badge.exclamationmark")
                            .foregroundColor(.orange)
                    }
                }
                .font(.caption2)
                .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding(.vertical, 4)
    }
}

struct EmptyMultiSigView: View {
    let onCreate: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "person.2.badge.key")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("No Multi-Sig Wallets")
                .font(.headline)

            Text("Create a multi-signature wallet for enhanced security")
                .font(.caption)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            Button("Create Wallet") {
                onCreate()
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
        .frame(maxWidth: .infinity)
    }
}

#Preview {
    MultiSigView()
}
