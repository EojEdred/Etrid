//
//  SocialTabView.swift
//  EtridWallet
//
//  Main social tab view with navigation to Contacts, Bills, and Feed
//

import SwiftUI

struct SocialTabView: View {
    @State private var selectedSection = 0

    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                VStack(spacing: 0) {
                    // Top Navigation Cards
                    ScrollView(.horizontal, showsIndicators: false) {
                        HStack(spacing: 12) {
                            QuickAccessCard(
                                title: "Contacts",
                                icon: "person.crop.circle.fill",
                                color: .blue,
                                destination: AnyView(ContactsView())
                            )

                            QuickAccessCard(
                                title: "Bill Split",
                                icon: "doc.text.fill",
                                color: .purple,
                                destination: AnyView(BillSplitView())
                            )

                            QuickAccessCard(
                                title: "Request Payment",
                                icon: "arrow.down.circle.fill",
                                color: .green,
                                destination: AnyView(PaymentRequestView())
                            )
                        }
                        .padding(.horizontal)
                    }
                    .padding(.top)

                    // Main Feed
                    SocialFeedView()
                }
            }
            .navigationTitle("Social")
            .navigationBarTitleDisplayMode(.large)
        }
    }
}

// MARK: - Quick Access Card
struct QuickAccessCard: View {
    let title: String
    let icon: String
    let color: Color
    let destination: AnyView

    var body: some View {
        NavigationLink(destination: destination) {
            VStack(spacing: 12) {
                ZStack {
                    Circle()
                        .fill(color.opacity(0.2))
                        .frame(width: 60, height: 60)

                    Image(systemName: icon)
                        .font(.title2)
                        .foregroundColor(color)
                }

                Text(title)
                    .font(.caption)
                    .fontWeight(.semibold)
                    .foregroundColor(.white)
            }
            .frame(width: 100)
            .padding(.vertical, 16)
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)
        }
    }
}

struct SocialTabView_Previews: PreviewProvider {
    static var previews: some View {
        SocialTabView()
    }
}
