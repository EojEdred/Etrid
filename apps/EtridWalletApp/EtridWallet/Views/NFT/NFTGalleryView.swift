//
//  NFTGalleryView.swift
//  EtridWallet
//
//  NFT Gallery screen
//

import SwiftUI

struct NFTGalleryView: View {
    @StateObject private var viewModel = NFTGalleryViewModel()
    @State private var selectedTab: NFTTab = .owned

    enum NFTTab {
        case owned, created
    }

    var body: some View {
        NavigationStack {
            ZStack {
                // Gradient background
                LinearGradient(
                    colors: [Color.etridBlue.opacity(0.3), Color.etridPurple.opacity(0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 20) {
                        // Stats Cards
                        HStack(spacing: 12) {
                            StatCard(label: "Total NFTs", value: "\(viewModel.nfts.count)")
                            StatCard(label: "Total Value", value: "24.5 ÉTR")
                            StatCard(label: "Collections", value: "5")
                        }
                        .padding(.horizontal)

                        // Tabs
                        HStack(spacing: 0) {
                            TabButton(title: "Owned", isSelected: selectedTab == .owned) {
                                selectedTab = .owned
                            }
                            TabButton(title: "Created", isSelected: selectedTab == .created) {
                                selectedTab = .created
                            }
                        }
                        .padding(.horizontal)

                        // NFT Grid
                        LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 16) {
                            ForEach(viewModel.nfts) { nft in
                                NavigationLink(destination: NFTDetailView(nft: nft)) {
                                    NFTCard(nft: nft)
                                }
                            }
                        }
                        .padding(.horizontal)
                    }
                    .padding(.vertical)
                }
            }
            .navigationTitle("NFT Gallery")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    HStack(spacing: 16) {
                        NavigationLink(destination: NFTMarketplaceView()) {
                            Image(systemName: "cart")
                        }
                        Button(action: {}) {
                            Image(systemName: "plus.circle")
                        }
                    }
                }
            }
        }
        .task {
            await viewModel.loadNFTs()
        }
    }
}

private struct NFTCard: View {
    let nft: NFT

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            // NFT Image
            AsyncImage(url: URL(string: nft.imageURL)) { image in
                image
                    .resizable()
                    .aspectRatio(contentMode: .fill)
            } placeholder: {
                Rectangle()
                    .fill(Color.gray.opacity(0.3))
                    .overlay(ProgressView())
            }
            .frame(height: 160)
            .clipped()
            .cornerRadius(12)

            // NFT Info
            VStack(alignment: .leading, spacing: 4) {
                Text(nft.name)
                    .font(.subheadline)
                    .fontWeight(.semibold)
                    .foregroundColor(.primary)
                    .lineLimit(1)

                Text(nft.collection)
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .lineLimit(1)

                HStack {
                    Image(systemName: "link.circle.fill")
                        .font(.caption2)
                    Text(nft.chain)
                        .font(.caption2)
                }
                .foregroundColor(.etridBlue)

                Text(nft.displayPrice)
                    .font(.caption)
                    .fontWeight(.bold)
                    .foregroundColor(.etridGreen)
            }
            .padding(.horizontal, 8)
            .padding(.bottom, 8)
        }
        .background(Color(.systemBackground))
        .cornerRadius(16)
        .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
    }
}

private struct StatCard: View {
    let label: String
    let value: String

    var body: some View {
        VStack(spacing: 4) {
            Text(value)
                .font(.headline)
                .fontWeight(.bold)
                .foregroundColor(.primary)

            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 12)
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(color: Color.black.opacity(0.05), radius: 3)
    }
}

private struct TabButton: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(isSelected ? .white : .secondary)
                .frame(maxWidth: .infinity)
                .padding(.vertical, 12)
                .background(isSelected ? Color.etridBlue : Color(.systemGray6))
                .cornerRadius(8)
        }
    }
}

@MainActor
class NFTGalleryViewModel: ObservableObject {
    @Published var nfts: [NFT] = []
    @Published var isLoading = false

    func loadNFTs() async {
        isLoading = true
        // Simulate API call
        try? await Task.sleep(nanoseconds: 500_000_000)
        nfts = NFT.mockNFTs
        isLoading = false
    }
}

#Preview {
    NFTGalleryView()
}
