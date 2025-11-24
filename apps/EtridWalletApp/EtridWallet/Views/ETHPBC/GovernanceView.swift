//
//  GovernanceView.swift
//  EtridWallet
//
//  Governance Voting View
//

import SwiftUI

struct GovernanceView: View {
    @StateObject private var service = ETHPBCService()
    @State private var proposals: [ETHPBCGovernanceProposal] = []
    @State private var selectedStatus: ETHPBCGovernanceProposal.ProposalStatus?
    @State private var selectedProposal: ETHPBCGovernanceProposal?

    var body: some View {
        VStack(spacing: 0) {
            // Status Filter
            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 12) {
                    StatusButton(title: "All", status: nil, selectedStatus: $selectedStatus, onTap: loadProposals)
                    StatusButton(title: "Active", status: .active, selectedStatus: $selectedStatus, onTap: loadProposals)
                    StatusButton(title: "Pending", status: .pending, selectedStatus: $selectedStatus, onTap: loadProposals)
                    StatusButton(title: "Succeeded", status: .succeeded, selectedStatus: $selectedStatus, onTap: loadProposals)
                    StatusButton(title: "Executed", status: .executed, selectedStatus: $selectedStatus, onTap: loadProposals)
                }
                .padding()
            }

            // Proposals List
            if service.isLoading {
                Spacer()
                ProgressView("Loading proposals...")
                Spacer()
            } else if proposals.isEmpty {
                Spacer()
                VStack(spacing: 16) {
                    Image(systemName: "doc.text")
                        .font(.system(size: 60))
                        .foregroundColor(.gray)
                    Text("No Proposals Found")
                        .font(.title2)
                        .foregroundColor(.gray)
                    Text("Governance proposals will appear here")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }
                Spacer()
            } else {
                ScrollView {
                    LazyVStack(spacing: 12) {
                        ForEach(proposals) { proposal in
                            ProposalCard(proposal: proposal)
                                .onTapGesture {
                                    selectedProposal = proposal
                                }
                        }
                    }
                    .padding()
                }
            }
        }
        .navigationTitle("Governance")
        .navigationBarTitleDisplayMode(.inline)
        .sheet(item: $selectedProposal) { proposal in
            ProposalDetailView(proposal: proposal)
        }
        .onAppear(perform: loadProposals)
    }

    private func loadProposals() {
        Task {
            do {
                proposals = try await service.getProposals(status: selectedStatus)
            } catch {
                print("Error loading proposals: \(error)")
            }
        }
    }
}

// MARK: - Status Button
private struct StatusButton: View {
    let title: String
    let status: ETHPBCGovernanceProposal.ProposalStatus?
    @Binding var selectedStatus: ETHPBCGovernanceProposal.ProposalStatus?
    let onTap: () -> Void

    var isSelected: Bool {
        if title == "All" {
            return selectedStatus == nil
        }
        return selectedStatus == status
    }

    var body: some View {
        Button(action: {
            selectedStatus = status
            onTap()
        }) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(isSelected ? .white : .blue)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(isSelected ? Color.blue : Color.blue.opacity(0.1))
                .cornerRadius(20)
        }
    }
}

// MARK: - Proposal Card
private struct ProposalCard: View {
    let proposal: ETHPBCGovernanceProposal

    var statusColor: Color {
        Color(hex: proposal.status.color) ?? .gray
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                Text("Proposal #\(proposal.proposalId)")
                    .font(.caption)
                    .foregroundColor(.gray)

                Spacer()

                Text(proposal.status.rawValue.capitalized)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(statusColor)
                    .cornerRadius(12)
            }

            Text(proposal.title)
                .font(.headline)
                .lineLimit(2)

            Text(proposal.description)
                .font(.subheadline)
                .foregroundColor(.gray)
                .lineLimit(3)

            // Voting Progress
            VStack(spacing: 8) {
                HStack {
                    Text("For: \(String(format: "%.1f%%", proposal.forPercentage))")
                        .font(.caption)
                        .foregroundColor(.green)
                    Spacer()
                    Text("Against: \(String(format: "%.1f%%", proposal.againstPercentage))")
                        .font(.caption)
                        .foregroundColor(.red)
                }

                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color.red.opacity(0.3))
                            .frame(height: 8)
                            .cornerRadius(4)

                        Rectangle()
                            .fill(Color.green)
                            .frame(width: geometry.size.width * CGFloat(proposal.forPercentage / 100), height: 8)
                            .cornerRadius(4)
                    }
                }
                .frame(height: 8)
            }

            // Blocks Remaining
            if proposal.isActive {
                HStack {
                    Image(systemName: "clock")
                        .foregroundColor(.blue)
                        .font(.caption)
                    Text("\(proposal.blocksRemaining) blocks remaining")
                        .font(.caption)
                        .foregroundColor(.gray)
                }
            }

            // Proposer
            HStack {
                Image(systemName: "person.circle")
                    .foregroundColor(.gray)
                Text(proposal.proposerDisplay)
                    .font(.caption)
                    .foregroundColor(.gray)
                Spacer()
                Text(proposal.createdAt, style: .relative)
                    .font(.caption)
                    .foregroundColor(.gray)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
    }
}

// MARK: - Proposal Detail View
private struct ProposalDetailView: View {
    let proposal: ETHPBCGovernanceProposal
    @StateObject private var service = ETHPBCService()
    @State private var selectedChoice: VoteChoice?
    @State private var isVoting = false
    @State private var showSuccess = false
    @State private var errorMessage: String?
    @Environment(\.presentationMode) var presentationMode

    var statusColor: Color {
        Color(hex: proposal.status.color) ?? .gray
    }

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(alignment: .leading, spacing: 20) {
                    // Header
                    VStack(alignment: .leading, spacing: 12) {
                        HStack {
                            Text("Proposal #\(proposal.proposalId)")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Spacer()
                            Text(proposal.status.rawValue.capitalized)
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundColor(.white)
                                .padding(.horizontal, 12)
                                .padding(.vertical, 6)
                                .background(statusColor)
                                .cornerRadius(12)
                        }

                        Text(proposal.title)
                            .font(.title2)
                            .fontWeight(.bold)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)

                    // Description
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Description")
                            .font(.headline)
                        Text(proposal.description)
                            .font(.subheadline)
                            .foregroundColor(.gray)
                    }

                    // Voting Results
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Voting Results")
                            .font(.headline)

                        VStack(spacing: 12) {
                            VoteResultRow(
                                choice: "For",
                                votes: proposal.votesFor,
                                percentage: proposal.forPercentage,
                                color: .green
                            )

                            VoteResultRow(
                                choice: "Against",
                                votes: proposal.votesAgainst,
                                percentage: proposal.againstPercentage,
                                color: .red
                            )

                            VoteResultRow(
                                choice: "Abstain",
                                votes: proposal.votesAbstain,
                                percentage: proposal.abstainPercentage,
                                color: .gray
                            )
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }

                    // Vote Section (if active)
                    if proposal.isActive {
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Cast Your Vote")
                                .font(.headline)

                            VStack(spacing: 12) {
                                VoteChoiceButton(
                                    choice: .forVote,
                                    selectedChoice: $selectedChoice
                                )

                                VoteChoiceButton(
                                    choice: .against,
                                    selectedChoice: $selectedChoice
                                )

                                VoteChoiceButton(
                                    choice: .abstain,
                                    selectedChoice: $selectedChoice
                                )
                            }

                            if let error = errorMessage {
                                Text(error)
                                    .font(.subheadline)
                                    .foregroundColor(.red)
                                    .padding()
                                    .background(Color.red.opacity(0.1))
                                    .cornerRadius(8)
                            }

                            Button(action: castVote) {
                                if isVoting {
                                    ProgressView()
                                        .progressViewStyle(CircularProgressViewStyle(tint: .white))
                                } else {
                                    HStack {
                                        Image(systemName: "checkmark.circle.fill")
                                        Text("Submit Vote")
                                    }
                                    .font(.headline)
                                }
                            }
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(selectedChoice != nil ? Color.blue : Color.gray)
                            .cornerRadius(12)
                            .disabled(selectedChoice == nil || isVoting)
                        }
                    }

                    // Details
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Details")
                            .font(.headline)

                        VStack(spacing: 12) {
                            DetailInfoRow(label: "Proposer", value: proposal.proposerDisplay)
                            DetailInfoRow(label: "Start Block", value: "\(proposal.startBlock)")
                            DetailInfoRow(label: "End Block", value: "\(proposal.endBlock)")
                            DetailInfoRow(label: "Current Block", value: "\(proposal.currentBlock)")

                            if proposal.isActive {
                                DetailInfoRow(label: "Blocks Remaining", value: "\(proposal.blocksRemaining)")
                            }

                            DetailInfoRow(label: "Created", value: proposal.createdAt.formatted())
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }
                }
                .padding()
            }
            .navigationTitle("Proposal Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        presentationMode.wrappedValue.dismiss()
                    }
                }
            }
            .alert(isPresented: $showSuccess) {
                Alert(
                    title: Text("Vote Submitted!"),
                    message: Text("Your vote has been recorded on-chain."),
                    dismissButton: .default(Text("OK")) {
                        presentationMode.wrappedValue.dismiss()
                    }
                )
            }
        }
    }

    private func castVote() {
        guard let choice = selectedChoice else { return }

        isVoting = true
        errorMessage = nil

        Task {
            do {
                _ = try await service.voteOnProposal(proposalId: proposal.proposalId, choice: choice)
                showSuccess = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isVoting = false
        }
    }
}

// MARK: - Vote Result Row
private struct VoteResultRow: View {
    let choice: String
    let votes: String
    let percentage: Double
    let color: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text(choice)
                    .font(.subheadline)
                    .fontWeight(.medium)
                Spacer()
                Text(String(format: "%.1f%%", percentage))
                    .font(.subheadline)
                    .fontWeight(.bold)
                    .foregroundColor(color)
            }

            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    Rectangle()
                        .fill(color.opacity(0.2))
                        .frame(height: 8)
                        .cornerRadius(4)

                    Rectangle()
                        .fill(color)
                        .frame(width: geometry.size.width * CGFloat(percentage / 100), height: 8)
                        .cornerRadius(4)
                }
            }
            .frame(height: 8)

            Text(votes)
                .font(.caption)
                .foregroundColor(.gray)
        }
    }
}

// MARK: - Vote Choice Button
private struct VoteChoiceButton: View {
    let choice: VoteChoice
    @Binding var selectedChoice: VoteChoice?

    var isSelected: Bool {
        selectedChoice == choice
    }

    var body: some View {
        Button(action: {
            selectedChoice = choice
        }) {
            HStack {
                Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                    .foregroundColor(isSelected ? .blue : .gray)

                Text(choice.displayName)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(.primary)

                Spacer()
            }
            .padding()
            .background(isSelected ? Color.blue.opacity(0.1) : Color(.systemGray6))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? Color.blue : Color.clear, lineWidth: 2)
            )
        }
    }
}

// MARK: - Detail Info Row
private struct DetailInfoRow: View {
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
                .fontWeight(.medium)
        }
    }
}
