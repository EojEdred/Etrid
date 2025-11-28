//
//  WelcomeView.swift
//  EtridWallet
//
//  Welcome/onboarding screen
//

import SwiftUI

struct WelcomeView: View {
    @State private var isAnimating = false

    var body: some View {
        NavigationStack {
            ZStack {
                // Background gradient
                LinearGradient(
                    colors: [Color.etridBlue, Color.etridPurple],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                VStack(spacing: 40) {
                    Spacer()

                    // Logo and title
                    VStack(spacing: 20) {
                        Image(systemName: "bitcoinsign.circle.fill")
                            .resizable()
                            .frame(width: 100, height: 100)
                            .foregroundColor(.white)
                            .scaleEffect(isAnimating ? 1.0 : 0.8)
                            .animation(.easeInOut(duration: 1.5).repeatForever(autoreverses: true), value: isAnimating)

                        Text("Ëtrid Wallet")
                            .font(.system(size: 42, weight: .bold))
                            .foregroundColor(.white)

                        Text("Your gateway to Web3")
                            .font(.title3)
                            .foregroundColor(.white.opacity(0.9))
                    }

                    Spacer()

                    // Features
                    VStack(alignment: .leading, spacing: 20) {
                        FeatureRow(icon: "lock.shield.fill", title: "Secure", description: "Your keys, your crypto")
                        FeatureRow(icon: "network", title: "Multi-chain", description: "Support for all major networks")
                        FeatureRow(icon: "bolt.fill", title: "Fast", description: "Lightning-fast transactions")
                    }
                    .padding(.horizontal, 30)

                    Spacer()

                    // Action buttons
                    VStack(spacing: 16) {
                        NavigationLink(destination: CreateWalletView()) {
                            Text("Create New Wallet")
                                .font(.headline)
                                .foregroundColor(.etridBlue)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.white)
                                .cornerRadius(16)
                        }

                        NavigationLink(destination: ImportWalletView()) {
                            Text("Import Wallet")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .overlay(
                                    RoundedRectangle(cornerRadius: 16)
                                        .stroke(Color.white, lineWidth: 2)
                                )
                        }
                    }
                    .padding(.horizontal, 30)
                    .padding(.bottom, 40)
                }
            }
        }
        .onAppear {
            isAnimating = true
        }
    }
}

private struct FeatureRow: View {
    let icon: String
    let title: String
    let description: String

    var body: some View {
        HStack(spacing: 15) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(.white)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.headline)
                    .foregroundColor(.white)

                Text(description)
                    .font(.subheadline)
                    .foregroundColor(.white.opacity(0.8))
            }
        }
    }
}

#Preview {
    WelcomeView()
}
