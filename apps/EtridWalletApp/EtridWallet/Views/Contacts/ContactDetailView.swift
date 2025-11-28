//
//  ContactDetailView.swift
//  EtridWallet
//
//  Detailed view of a contact with transaction history
//

import SwiftUI

struct ContactDetailView: View {
    @Environment(\.dismiss) var dismiss
    let contact: Contact
    @ObservedObject var viewModel: ContactsViewModel

    @State private var showingEditContact = false
    @State private var showingSendView = false
    @State private var showingRequestView = false
    @State private var selectedTab = 0
    @State private var transactions: [ContactTransaction] = []

    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 24) {
                        // Header Card
                        headerCard

                        // Action Buttons
                        actionButtons

                        // Tabs
                        Picker("View", selection: $selectedTab) {
                            Text("Info").tag(0)
                            Text("Transactions").tag(1)
                        }
                        .pickerStyle(.segmented)
                        .padding(.horizontal)

                        // Content
                        if selectedTab == 0 {
                            infoSection
                        } else {
                            transactionsSection
                        }
                    }
                    .padding()
                }
            }
            .navigationTitle("Contact Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Close") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showingEditContact = true
                    } label: {
                        Image(systemName: "pencil")
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $showingEditContact) {
                AddContactView(viewModel: viewModel, contact: contact)
            }
        }
        .onAppear {
            loadTransactions()
        }
    }

    private var headerCard: some View {
        VStack(spacing: 16) {
            // Avatar
            ZStack {
                Circle()
                    .fill(
                        LinearGradient(
                            colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .frame(width: 100, height: 100)

                if let avatar = contact.avatar, avatar.count == 1 {
                    Text(avatar)
                        .font(.system(size: 50))
                } else {
                    Text(contact.initials)
                        .font(.system(size: 40))
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                }
            }

            // Name
            HStack {
                Text(contact.name)
                    .font(.title2)
                    .fontWeight(.bold)
                    .foregroundColor(.white)

                if contact.isFavorite {
                    Image(systemName: "star.fill")
                        .foregroundColor(.yellow)
                }
            }

            // Address
            HStack {
                Text(contact.shortAddress)
                    .font(.subheadline)
                    .foregroundColor(.gray)

                Button {
                    UIPasteboard.general.string = contact.address
                } label: {
                    Image(systemName: "doc.on.doc")
                        .font(.caption)
                        .foregroundColor(.blue)
                }
            }

            // Tags
            if !contact.tags.isEmpty {
                FlowLayout(spacing: 8) {
                    ForEach(contact.tags, id: \.self) { tag in
                        Text(tag)
                            .font(.caption)
                            .foregroundColor(.white)
                            .padding(.horizontal, 10)
                            .padding(.vertical, 6)
                            .background(Color.blue.opacity(0.4))
                            .cornerRadius(12)
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var actionButtons: some View {
        HStack(spacing: 16) {
            ActionButton(
                title: "Send",
                icon: "arrow.up.circle.fill",
                color: .blue
            ) {
                showingSendView = true
            }

            ActionButton(
                title: "Request",
                icon: "arrow.down.circle.fill",
                color: .green
            ) {
                showingRequestView = true
            }

            ActionButton(
                title: "Share",
                icon: "square.and.arrow.up",
                color: .purple
            ) {
                shareContact()
            }
        }
        .padding(.horizontal)
    }

    private var infoSection: some View {
        VStack(spacing: 16) {
            if let email = contact.email {
                InfoRow(label: "Email", value: email, icon: "envelope")
            }

            if let phone = contact.phone {
                InfoRow(label: "Phone", value: phone, icon: "phone")
            }

            if let network = contact.network {
                InfoRow(label: "Network", value: network, icon: "network")
            }

            InfoRow(
                label: "Added",
                value: contact.createdAt.formatted(date: .abbreviated, time: .omitted),
                icon: "calendar"
            )

            if let lastInteraction = contact.lastInteraction {
                InfoRow(
                    label: "Last Interaction",
                    value: lastInteraction.formatted(date: .abbreviated, time: .shortened),
                    icon: "clock"
                )
            }

            if let notes = contact.notes, !notes.isEmpty {
                VStack(alignment: .leading, spacing: 8) {
                    HStack {
                        Image(systemName: "note.text")
                            .foregroundColor(.blue)
                        Text("Notes")
                            .font(.subheadline)
                            .foregroundColor(.gray)
                    }

                    Text(notes)
                        .font(.body)
                        .foregroundColor(.white)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var transactionsSection: some View {
        VStack(spacing: 12) {
            if transactions.isEmpty {
                VStack(spacing: 16) {
                    Image(systemName: "arrow.left.arrow.right")
                        .font(.system(size: 50))
                        .foregroundColor(.gray)

                    Text("No transaction history")
                        .font(.headline)
                        .foregroundColor(.white)

                    Text("Send or receive funds to see your transaction history")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                        .multilineTextAlignment(.center)
                }
                .frame(maxWidth: .infinity)
                .padding(40)
                .background(Color.white.opacity(0.05))
                .cornerRadius(16)
            } else {
                ForEach(transactions) { transaction in
                    TransactionRow(transaction: transaction)
                }
            }
        }
    }

    private func loadTransactions() {
        // Load transaction history for this contact
        // Placeholder data
        transactions = [
            ContactTransaction(
                id: UUID(),
                contactId: contact.id,
                transactionHash: "0x1234...5678",
                type: .sent,
                amount: String(UInt64(1e18)),
                tokenSymbol: "ETR",
                timestamp: Date().addingTimeInterval(-86400),
                status: .confirmed
            ),
            ContactTransaction(
                id: UUID(),
                contactId: contact.id,
                transactionHash: "0x5678...9012",
                type: .received,
                amount: String(UInt64(2e18)),
                tokenSymbol: "ETR",
                timestamp: Date().addingTimeInterval(-172800),
                status: .confirmed
            )
        ]
    }

    private func shareContact() {
        // Share contact information
    }
}

// MARK: - Action Button
private struct ActionButton: View {
    let title: String
    let icon: String
    let color: Color
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundColor(color)

                Text(title)
                    .font(.caption)
                    .foregroundColor(.white)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
    }
}

// MARK: - Info Row
private struct InfoRow: View {
    let label: String
    let value: String
    let icon: String

    var body: some View {
        HStack {
            Image(systemName: icon)
                .foregroundColor(.blue)
                .frame(width: 30)

            VStack(alignment: .leading, spacing: 4) {
                Text(label)
                    .font(.caption)
                    .foregroundColor(.gray)

                Text(value)
                    .font(.body)
                    .foregroundColor(.white)
            }

            Spacer()
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }
}

// MARK: - Transaction Row
private struct TransactionRow: View {
    let transaction: ContactTransaction

    var body: some View {
        HStack {
            ZStack {
                Circle()
                    .fill(transaction.type == .sent ? Color.red.opacity(0.2) : Color.green.opacity(0.2))
                    .frame(width: 40, height: 40)

                Image(systemName: transaction.type == .sent ? "arrow.up" : "arrow.down")
                    .foregroundColor(transaction.type == .sent ? .red : .green)
            }

            VStack(alignment: .leading, spacing: 4) {
                Text(transaction.type == .sent ? "Sent" : "Received")
                    .font(.headline)
                    .foregroundColor(.white)

                Text(transaction.timestamp.formatted(date: .abbreviated, time: .shortened))
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(transaction.formattedAmount)
                    .font(.headline)
                    .foregroundColor(.white)

                statusBadge
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(12)
    }

    private var statusBadge: some View {
        HStack(spacing: 4) {
            Circle()
                .fill(statusColor)
                .frame(width: 6, height: 6)

            Text(transaction.status.rawValue)
                .font(.caption2)
                .foregroundColor(.gray)
        }
    }

    private var statusColor: Color {
        switch transaction.status {
        case .confirmed: return .green
        case .pending: return .orange
        case .failed: return .red
        case .dropped: return .gray
        case .replaced: return .blue
        }
    }
}

struct ContactDetailView_Previews: PreviewProvider {
    static var previews: some View {
        ContactDetailView(
            contact: Contact(
                name: "Alice Johnson",
                address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
                avatar: "👩",
                isFavorite: true,
                tags: ["Friend", "ETR"]
            ),
            viewModel: ContactsViewModel()
        )
    }
}
