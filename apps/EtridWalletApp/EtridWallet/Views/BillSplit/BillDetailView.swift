//
//  BillDetailView.swift
//  EtridWallet
//
//  Detailed view of a bill split with payment tracking
//

import SwiftUI

struct BillDetailView: View {
    @Environment(\.dismiss) var dismiss
    let bill: SplitBill
    @ObservedObject var viewModel: BillSplitViewModel

    @State private var showingPaymentConfirmation = false
    @State private var selectedParticipant: Participant?
    @State private var showingShareSheet = false

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

                        // Progress Section
                        progressSection

                        // Participants List
                        participantsSection

                        // Payment Actions
                        if !bill.isCompleted {
                            paymentActionsSection
                        }

                        // Details Section
                        detailsSection
                    }
                    .padding()
                }
            }
            .navigationTitle("Bill Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Close") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Menu {
                        Button {
                            showingShareSheet = true
                        } label: {
                            Label("Share Bill", systemImage: "square.and.arrow.up")
                        }

                        Button {
                            sendReminders()
                        } label: {
                            Label("Send Reminders", systemImage: "bell")
                        }

                        Button(role: .destructive) {
                            deleteBill()
                        } label: {
                            Label("Delete Bill", systemImage: "trash")
                        }
                    } label: {
                        Image(systemName: "ellipsis")
                            .foregroundColor(.white)
                    }
                }
            }
        }
    }

    private var headerCard: some View {
        VStack(spacing: 16) {
            // Category Icon
            if let category = bill.category {
                ZStack {
                    Circle()
                        .fill(Color(category.color).opacity(0.2))
                        .frame(width: 60, height: 60)

                    Image(systemName: category.icon)
                        .font(.title)
                        .foregroundColor(Color(category.color))
                }
            }

            // Title
            Text(bill.title)
                .font(.title2)
                .fontWeight(.bold)
                .foregroundColor(.white)

            if let description = bill.description {
                Text(description)
                    .font(.subheadline)
                    .foregroundColor(.gray)
                    .multilineTextAlignment(.center)
            }

            // Amount
            VStack(spacing: 4) {
                Text("Total Amount")
                    .font(.caption)
                    .foregroundColor(.gray)

                Text("\(bill.formattedTotalAmount) \(bill.tokenSymbol)")
                    .font(.system(size: 32, weight: .bold))
                    .foregroundColor(.white)
            }
            .padding()
            .frame(maxWidth: .infinity)
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)

            // Status Badge
            HStack(spacing: 12) {
                StatusBadge(
                    text: bill.isCompleted ? "Completed" : "Active",
                    color: bill.isCompleted ? .green : .blue
                )

                if bill.isExpired {
                    StatusBadge(text: "Expired", color: .red)
                }

                Text("•")
                    .foregroundColor(.gray)

                Text(bill.splitType.rawValue)
                    .font(.caption)
                    .foregroundColor(.gray)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var progressSection: some View {
        VStack(spacing: 16) {
            // Progress Stats
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Payment Progress")
                        .font(.headline)
                        .foregroundColor(.white)

                    Text("\(bill.paidCount) of \(bill.participants.count) paid")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }

                Spacer()

                Text("\(Int(bill.completionPercentage))%")
                    .font(.title2)
                    .fontWeight(.bold)
                    .foregroundColor(.white)
            }

            // Progress Bar
            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    RoundedRectangle(cornerRadius: 8)
                        .fill(Color.white.opacity(0.1))
                        .frame(height: 12)

                    RoundedRectangle(cornerRadius: 8)
                        .fill(
                            LinearGradient(
                                colors: [Color.green, Color.blue],
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .frame(width: geometry.size.width * CGFloat(bill.completionPercentage / 100), height: 12)
                }
            }
            .frame(height: 12)

            // Amount Breakdown
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Paid")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text("\((Double(bill.totalPaid) ?? 0) / 1e18, specifier: "%.4f") \(bill.tokenSymbol)")
                        .font(.body)
                        .foregroundColor(.green)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Pending")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text("\((Double(bill.totalPending) ?? 0) / 1e18, specifier: "%.4f") \(bill.tokenSymbol)")
                        .font(.body)
                        .foregroundColor(.orange)
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var participantsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Participants")
                .font(.headline)
                .foregroundColor(.white)

            ForEach(bill.participants) { participant in
                ParticipantDetailRow(participant: participant, tokenSymbol: bill.tokenSymbol) {
                    selectedParticipant = participant
                    if participant.status == .pending {
                        showingPaymentConfirmation = true
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var paymentActionsSection: some View {
        VStack(spacing: 12) {
            if let myParticipation = bill.participants.first(where: { $0.address.lowercased() == viewModel.userAddress.lowercased() }),
               myParticipation.status == .pending {
                Button {
                    selectedParticipant = myParticipation
                    showingPaymentConfirmation = true
                } label: {
                    HStack {
                        Image(systemName: "creditcard.fill")
                        Text("Pay My Share (\(myParticipation.formattedAmount) \(bill.tokenSymbol))")
                            .fontWeight(.semibold)
                    }
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(
                        LinearGradient(
                            colors: [Color.green, Color.blue],
                            startPoint: .leading,
                            endPoint: .trailing
                        )
                    )
                    .cornerRadius(12)
                }
            }
        }
        .padding(.horizontal)
    }

    private var detailsSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Details")
                .font(.headline)
                .foregroundColor(.white)

            DetailRow(label: "Created by", value: bill.creatorAddress.prefix(10) + "..." + bill.creatorAddress.suffix(4))
            DetailRow(label: "Created on", value: bill.createdAt.formatted(date: .abbreviated, time: .shortened))

            if let expiresAt = bill.expiresAt {
                DetailRow(label: "Expires", value: expiresAt.formatted(date: .abbreviated, time: .shortened))
            }

            DetailRow(label: "Split method", value: bill.splitType.rawValue)

            if let category = bill.category {
                DetailRow(label: "Category", value: category.rawValue)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private func sendReminders() {
        // Send payment reminders to pending participants
    }

    private func deleteBill() {
        viewModel.deleteBill(bill)
        dismiss()
    }
}

// MARK: - Participant Detail Row
private struct ParticipantDetailRow: View {
    let participant: Participant
    let tokenSymbol: String
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            HStack {
                // Avatar
                ZStack {
                    Circle()
                        .fill(statusColor.opacity(0.2))
                        .frame(width: 50, height: 50)

                    Text(participant.name.prefix(1).uppercased())
                        .font(.title3)
                        .foregroundColor(statusColor)
                }

                // Info
                VStack(alignment: .leading, spacing: 4) {
                    Text(participant.name)
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)

                    Text(participant.shortAddress)
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()

                // Amount and Status
                VStack(alignment: .trailing, spacing: 4) {
                    Text("\(participant.formattedAmount) \(tokenSymbol)")
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)

                    HStack(spacing: 4) {
                        Circle()
                            .fill(statusColor)
                            .frame(width: 6, height: 6)

                        Text(participant.status.rawValue)
                            .font(.caption)
                            .foregroundColor(statusColor)
                    }
                }
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(12)
        }
        .buttonStyle(PlainButtonStyle())
    }

    private var statusColor: Color {
        switch participant.status {
        case .paid: return .green
        case .pending: return .orange
        case .declined: return .red
        case .expired: return .gray
        }
    }
}

// MARK: - Status Badge
private struct StatusBadge: View {
    let text: String
    let color: Color

    var body: some View {
        Text(text)
            .font(.caption)
            .fontWeight(.semibold)
            .foregroundColor(color)
            .padding(.horizontal, 10)
            .padding(.vertical, 6)
            .background(color.opacity(0.2))
            .cornerRadius(8)
    }
}

// MARK: - Detail Row
private struct DetailRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.gray)

            Spacer()

            Text(value)
                .font(.subheadline)
                .foregroundColor(.white)
        }
    }
}

struct BillDetailView_Previews: PreviewProvider {
    static var previews: some View {
        BillDetailView(
            bill: SplitBill(
                title: "Dinner at Restaurant",
                description: "Team dinner celebration",
                totalAmount: String(UInt64(10 * 1e18)),
                tokenSymbol: "ETR",
                splitType: .even,
                participants: [
                    Participant(name: "Alice", address: "0x1234...5678", amount: String(UInt64(5 * 1e18)), status: .paid),
                    Participant(name: "Bob", address: "0x5678...9012", amount: String(UInt64(5 * 1e18)), status: .pending)
                ],
                creatorAddress: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
                category: .food
            ),
            viewModel: BillSplitViewModel()
        )
    }
}
