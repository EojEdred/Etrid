//
//  NFTMarketplaceView.swift
//  EtridWallet
//
//  NFT Marketplace screen
//

import SwiftUI

struct NFTMarketplaceView: View {
    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                Text("NFT Marketplace")
                    .font(.largeTitle)
                    .fontWeight(.bold)

                Text("Browse and purchase NFTs from various collections")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)

                // Coming soon placeholder
                VStack(spacing: 16) {
                    Image(systemName: "cart.fill")
                        .font(.system(size: 60))
                        .foregroundColor(.etridBlue)

                    Text("Marketplace Coming Soon")
                        .font(.headline)

                    Text("Discover, buy, and sell NFTs across multiple chains")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
                .padding(.vertical, 40)
            }
            .padding()
        }
        .navigationTitle("Marketplace")
        .navigationBarTitleDisplayMode(.inline)
    }
}

#Preview {
    NavigationStack {
        NFTMarketplaceView()
    }
}
