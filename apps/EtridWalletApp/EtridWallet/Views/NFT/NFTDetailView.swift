//
//  NFTDetailView.swift
//  EtridWallet
//
//  NFT detail screen
//

import SwiftUI

struct NFTDetailView: View {
    @Environment(\.dismiss) private var dismiss
    let nft: NFT

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // NFT Image
                AsyncImage(url: URL(string: nft.imageURL)) { image in
                    image
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                } placeholder: {
                    Rectangle()
                        .fill(Color.gray.opacity(0.3))
                        .overlay(ProgressView())
                }
                .frame(maxWidth: .infinity)
                .frame(height: 350)
                .cornerRadius(20)
                .padding(.horizontal)

                // NFT Info
                VStack(alignment: .leading, spacing: 16) {
                    // Name & Collection
                    VStack(alignment: .leading, spacing: 8) {
                        Text(nft.name)
                            .font(.title2)
                            .fontWeight(.bold)

                        HStack {
                            Image(systemName: "square.grid.2x2")
                            Text(nft.collection)
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                        }
                    }

                    Divider()

                    // Description
                    if let description = nft.description {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Description")
                                .font(.headline)

                            Text(description)
                                .font(.body)
                                .foregroundColor(.secondary)
                        }

                        Divider()
                    }

                    // Price & Chain
                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Price")
                                .font(.caption)
                                .foregroundColor(.secondary)

                            Text(nft.displayPrice)
                                .font(.title3)
                                .fontWeight(.bold)
                                .foregroundColor(.etridGreen)
                        }

                        Spacer()

                        VStack(alignment: .trailing, spacing: 4) {
                            Text("Chain")
                                .font(.caption)
                                .foregroundColor(.secondary)

                            HStack {
                                Image(systemName: "link.circle.fill")
                                Text(nft.chain)
                            }
                            .font(.subheadline)
                            .foregroundColor(.etridBlue)
                        }
                    }

                    // Attributes
                    if let attributes = nft.attributes, !attributes.isEmpty {
                        Divider()

                        VStack(alignment: .leading, spacing: 12) {
                            Text("Attributes")
                                .font(.headline)

                            LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 12) {
                                ForEach(attributes) { attribute in
                                    AttributeCard(attribute: attribute)
                                }
                            }
                        }
                    }

                    // Actions
                    HStack(spacing: 12) {
                        Button(action: {}) {
                            Label("Transfer", systemImage: "arrow.up.right")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.etridBlue)
                                .cornerRadius(16)
                        }

                        Button(action: {}) {
                            Label("Sell", systemImage: "tag")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.etridGreen)
                                .cornerRadius(16)
                        }
                    }
                }
                .padding(.horizontal)
            }
            .padding(.vertical)
        }
        .navigationBarTitleDisplayMode(.inline)
    }
}

struct AttributeCard: View {
    let attribute: NFTAttribute

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(attribute.traitType)
                .font(.caption)
                .foregroundColor(.secondary)
                .textCase(.uppercase)

            Text(attribute.value)
                .font(.subheadline)
                .fontWeight(.semibold)

            if let rarity = attribute.rarity {
                Text("\(Int(rarity * 100))% Rare")
                    .font(.caption2)
                    .foregroundColor(.etridBlue)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

#Preview {
    NavigationStack {
        NFTDetailView(nft: NFT.mockNFTs[0])
    }
}
