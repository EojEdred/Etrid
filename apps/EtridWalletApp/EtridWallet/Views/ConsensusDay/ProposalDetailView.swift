//
//  ProposalDetailView.swift
//  EtridWallet
//
//  Detailed view of a governance proposal with voting interface
//

import SwiftUI

struct CDProposalDetailView: View {
    let proposal: GovernanceProposal
    @ObservedObject var viewModel: ConsensusDayViewModel
    @Environment(\.dismiss) private var dismiss
    @State private var showVoteConfirmation = false
    @State private var selectedVote: VoteType?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                // Category Badge
                HStack {
                    Image(systemName: proposal.category.icon)
                        .foregroundColor(Color(proposal.category.color))

                    Text(proposal.category.rawValue)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                        .foregroundColor(Color(proposal.category.color))

                    Spacer()

                    if proposal.category.requiresSupermajority {
                        Text("66% Required")
                            .font(.caption)
                            .padding(.horizontal, 8)
                            .padding(.vertical, 4)
                            .background(Color.orange.opacity(0.2))
                            .foregroundColor(.orange)
                            .cornerRadius(6)
                    }
                }

                // Title
                Text(proposal.title)
                    .font(.title2)
                    .fontWeight(.bold)
                    .foregroundColor(.white)

                // Proposer Info
                VStack(alignment: .leading, spacing: 8) {
                    Text("Proposed By")
                        .font(.caption)
                        .foregroundColor(.gray)

                    HStack {
                        Circle()
                            .fill(Color.blue.opacity(0.2))
                            .frame(width: 32, height: 32)
                            .overlay(
                                Text(String(proposal.proposerName?.prefix(1) ?? "?"))
                                    .font(.subheadline)
                                    .foregroundColor(.blue)
                            )

                        VStack(alignment: .leading, spacing: 2) {
                            Text(proposal.proposerName ?? "Unknown")
                                .font(.subheadline)
                                .foregroundColor(.white)

                            Text(proposal.proposer.formatAddress())
                                .font(.caption2)
                                .foregroundColor(.gray)
                        }
                    }
                }
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)

                // Description
                VStack(alignment: .leading, spacing: 8) {
                    Text("Description")
                        .font(.headline)
                        .foregroundColor(.white)

                    Text(proposal.description)
                        .font(.body)
                        .foregroundColor(.gray)
                }

                // Budget Request (if applicable)
                if let budget = proposal.budgetRequest {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Budget Request")
                            .font(.headline)
                            .foregroundColor(.white)

                        HStack {
                            Image(systemName: "dollarsign.circle.fill")
                                .foregroundColor(.green)

                            Text(proposal.formattedBudget)
                                .font(.title3)
                                .fontWeight(.bold)
                                .foregroundColor(.white)
                        }
                        .padding()
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(Color.green.opacity(0.1))
                        .cornerRadius(12)
                    }
                }

                // Voting Stats
                VStack(alignment: .leading, spacing: 16) {
                    Text("Voting Results")
                        .font(.headline)
                        .foregroundColor(.white)

                    // Vote Breakdown
                    VStack(spacing: 12) {
                        VoteBar(
                            type: .yes,
                            percentage: proposal.forPercentage,
                            votes: WeiConverter.weiToEther(proposal.votesFor)
                        )

                        VoteBar(
                            type: .no,
                            percentage: proposal.againstPercentage,
                            votes: WeiConverter.weiToEther(proposal.votesAgainst)
                        )

                        VoteBar(
                            type: .abstain,
                            percentage: proposal.abstainPercentage,
                            votes: WeiConverter.weiToEther(proposal.votesAbstain)
                        )
                    }

                    // Total Votes
                    HStack {
                        Text("Total Votes")
                            .font(.subheadline)
                            .foregroundColor(.gray)

                        Spacer()

                        Text(String(format: "%.2f ËTR", proposal.totalVotes / 1e18))
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                    }
                    .padding()
                    .background(Color.white.opacity(0.05))
                    .cornerRadius(10)
                }

                // Status & Time
                HStack(spacing: 16) {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Status")
                            .font(.caption)
                            .foregroundColor(.gray)

                        HStack(spacing: 6) {
                            Circle()
                                .fill(Color(proposal.status.color))
                                .frame(width: 8, height: 8)

                            Text(proposal.status.rawValue)
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundColor(.white)
                        }
                    }

                    Divider()
                        .frame(height: 30)

                    VStack(alignment: .leading, spacing: 4) {
                        Text("Time Remaining")
                            .font(.caption)
                            .foregroundColor(.gray)

                        Text(proposal.timeRemaining)
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(.white)
                    }

                    Spacer()
                }
                .padding()
                .background(Color.white.opacity(0.05))
                .cornerRadius(12)

                // Voting Actions
                if proposal.status == .active {
                    VStack(spacing: 12) {
                        Text("Cast Your Vote")
                            .font(.headline)
                            .foregroundColor(.white)

                        Text("Your voting power: \(viewModel.votingPower.formattedPower)")
                            .font(.caption)
                            .foregroundColor(.gray)

                        HStack(spacing: 12) {
                            VoteButton(
                                voteType: .yes,
                                isSelected: selectedVote == .yes
                            ) {
                                selectedVote = .yes
                                showVoteConfirmation = true
                            }

                            VoteButton(
                                voteType: .no,
                                isSelected: selectedVote == .no
                            ) {
                                selectedVote = .no
                                showVoteConfirmation = true
                            }

                            VoteButton(
                                voteType: .abstain,
                                isSelected: selectedVote == .abstain
                            ) {
                                selectedVote = .abstain
                                showVoteConfirmation = true
                            }
                        }
                    }
                    .padding(.top, 8)
                }
            }
            .padding()
        }
        .background(
            LinearGradient(
                colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()
        )
        .navigationTitle("Proposal Details")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Confirm Vote", isPresented: $showVoteConfirmation) {
            Button("Cancel", role: .cancel) {
                selectedVote = nil
            }
            Button("Confirm") {
                if let vote = selectedVote {
                    viewModel.castVote(proposal: proposal, voteType: vote)
                    dismiss()
                }
            }
        } message: {
            if let vote = selectedVote {
                Text("Cast \(vote.rawValue) vote with \(viewModel.votingPower.formattedPower)?")
            }
        }
    }
}

// MARK: - Vote Bar
private struct VoteBar: View {
    let type: VoteType
    let percentage: Double
    let votes: Double

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Image(systemName: type.icon)
                    .font(.caption)
                    .foregroundColor(Color(type.color))

                Text(type.rawValue)
                    .font(.caption)
                    .foregroundColor(.gray)

                Spacer()

                Text(String(format: "%.1f%%", percentage))
                    .font(.caption)
                    .fontWeight(.semibold)
                    .foregroundColor(.white)

                Text(String(format: "%.2f ËTR", votes))
                    .font(.caption)
                    .foregroundColor(.gray)
            }

            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    RoundedRectangle(cornerRadius: 4)
                        .fill(Color.white.opacity(0.1))
                        .frame(height: 8)

                    RoundedRectangle(cornerRadius: 4)
                        .fill(Color(type.color))
                        .frame(width: geometry.size.width * CGFloat(percentage / 100), height: 8)
                }
            }
            .frame(height: 8)
        }
    }
}

// MARK: - Vote Button
private struct VoteButton: View {
    let voteType: VoteType
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: voteType.icon)
                    .font(.title2)
                    .foregroundColor(isSelected ? .white : Color(voteType.color))

                Text(voteType.rawValue)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(isSelected ? .white : .gray)
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 16)
            .background(
                isSelected ?
                    AnyView(Color(voteType.color)) :
                    AnyView(Color.white.opacity(0.05))
            )
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(Color(voteType.color), lineWidth: isSelected ? 0 : 1)
            )
        }
    }
}

#Preview {
    NavigationView {
        CDProposalDetailView(
            proposal: GovernanceProposal.mockProposals[0],
            viewModel: ConsensusDayViewModel()
        )
    }
}
