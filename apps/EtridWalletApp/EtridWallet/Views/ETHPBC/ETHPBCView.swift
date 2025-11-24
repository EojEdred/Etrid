//
//  ETHPBCView.swift
//  EtridWallet
//
//  ETH PBC Precompile Dashboard
//

import SwiftUI

struct ETHPBCView: View {
    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 16) {
                    // Header
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Ethereum PBC")
                            .font(.title)
                            .fontWeight(.bold)
                        Text("Access precompiled contracts for advanced blockchain operations with zero gas fees")
                            .font(.subheadline)
                            .foregroundColor(.gray)
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Precompile Services Grid
                    LazyVGrid(columns: [GridItem(.flexible()), GridItem(.flexible())], spacing: 16) {
                        PrecompileServiceCard(
                            icon: "chart.line.uptrend.xyaxis",
                            title: "Oracle",
                            description: "Real-time price feeds",
                            color: .blue,
                            destination: AnyView(OracleView())
                        )

                        PrecompileServiceCard(
                            icon: "person.3.fill",
                            title: "Governance",
                            description: "Vote on proposals",
                            color: .purple,
                            destination: AnyView(GovernanceView())
                        )

                        PrecompileServiceCard(
                            icon: "lock.fill",
                            title: "Staking",
                            description: "Stake for rewards",
                            color: .green,
                            destination: AnyView(StakingView())
                        )

                        PrecompileServiceCard(
                            icon: "arrow.triangle.2.circlepath",
                            title: "Wrap/Unwrap",
                            description: "ETH ↔ wETH",
                            color: .orange,
                            destination: AnyView(WrapUnwrapView())
                        )

                        PrecompileServiceCard(
                            icon: "arrow.left.arrow.right",
                            title: "Bridge",
                            description: "Cross-chain assets",
                            color: .red,
                            destination: AnyView(Text("Bridge View"))
                        )

                        PrecompileServiceCard(
                            icon: "list.bullet.rectangle",
                            title: "Token Registry",
                            description: "Query token info",
                            color: .teal,
                            destination: AnyView(Text("Token Registry"))
                        )
                    }

                    // Features
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Key Features")
                            .font(.headline)

                        FeatureRow(icon: "bolt.fill", title: "Zero Gas Fees", description: "All precompile calls require no gas")
                        FeatureRow(icon: "shield.fill", title: "Secure", description: "Native blockchain integration")
                        FeatureRow(icon: "speedometer", title: "Fast", description: "Optimized execution")
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                }
                .padding()
            }
            .navigationTitle("ETH PBC")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
}

// MARK: - Precompile Service Card
private struct PrecompileServiceCard: View {
    let icon: String
    let title: String
    let description: String
    let color: Color
    let destination: AnyView

    var body: some View {
        NavigationLink(destination: destination) {
            VStack(spacing: 12) {
                Image(systemName: icon)
                    .font(.system(size: 32))
                    .foregroundColor(color)

                Text(title)
                    .font(.headline)
                    .foregroundColor(.primary)

                Text(description)
                    .font(.caption)
                    .foregroundColor(.gray)
                    .multilineTextAlignment(.center)
                    .lineLimit(2)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color(.systemBackground))
            .cornerRadius(12)
            .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
        }
    }
}

// MARK: - Feature Row
private struct FeatureRow: View {
    let icon: String
    let title: String
    let description: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .font(.title3)
                .frame(width: 32)

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

struct ETHPBCView_Previews: PreviewProvider {
    static var previews: some View {
        ETHPBCView()
    }
}
