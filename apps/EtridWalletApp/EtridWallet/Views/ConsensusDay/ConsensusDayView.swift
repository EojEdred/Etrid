//
//  ConsensusDayView.swift
//  EtridWallet
//
//  Main Consensus Day governance interface
//

import SwiftUI

struct ConsensusDayView: View {
    @StateObject private var viewModel = ConsensusDayViewModel()
    @State private var selectedCategory: ProposalCategory?
    @State private var showVotingPower = false
    @State private var showRewards = false
    @State private var showCreateProposal = false

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
                    VStack(spacing: 20) {
                        // Countdown Card
                        countdownCard

                        // Phase Indicator
                        phaseIndicator

                        // Stats Card
                        statsCard

                        // Category Filter
                        categoryFilter

                        // Active Proposals
                        proposalsSection

                        // Director Election
                        if viewModel.consensusState.currentPhase == .voting {
                            directorElectionSection
                        }

                        // Participation Rewards
                        rewardsSection
                    }
                    .padding()
                }
            }
            .navigationTitle("Consensus Day")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button {
                        showCreateProposal = true
                    } label: {
                        Label("New Proposal", systemImage: "plus.circle.fill")
                            .foregroundColor(.orange)
                    }
                }

                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showVotingPower = true
                    } label: {
                        Image(systemName: "chart.bar.fill")
                            .foregroundColor(.white)
                    }
                }
            }
            .sheet(isPresented: $showVotingPower) {
                VotingPowerDetailView(info: viewModel.votingPower)
            }
            .sheet(isPresented: $showRewards) {
                RewardsHistoryView(rewards: viewModel.rewards)
            }
            .sheet(isPresented: $showCreateProposal) {
                CreateProposalView(viewModel: viewModel)
            }
        }
    }

    private var countdownCard: some View {
        VStack(spacing: 12) {
            HStack {
                Image(systemName: "calendar.badge.clock")
                    .font(.title2)
                    .foregroundColor(.purple)

                Text("Next Consensus Day")
                    .font(.headline)
                    .foregroundColor(.white)

                Spacer()
            }

            HStack(spacing: 20) {
                VStack(spacing: 4) {
                    Text("\(viewModel.consensusState.daysUntilConsensusDay)")
                        .font(.system(size: 48, weight: .bold))
                        .foregroundColor(.white)

                    Text("Days")
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                VStack(alignment: .leading, spacing: 8) {
                    Text("December 1st")
                        .font(.title3)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)

                    Text("Annual governance event")
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var phaseIndicator: some View {
        HStack(spacing: 12) {
            Image(systemName: viewModel.consensusState.currentPhase.icon)
                .font(.title2)
                .foregroundColor(Color(viewModel.consensusState.currentPhase.color))

            VStack(alignment: .leading, spacing: 4) {
                Text("Current Phase")
                    .font(.caption)
                    .foregroundColor(.gray)

                Text(viewModel.consensusState.currentPhase.rawValue)
                    .font(.headline)
                    .foregroundColor(.white)
            }

            Spacer()

            if viewModel.consensusState.currentPhase != .inactive {
                VStack(alignment: .trailing, spacing: 4) {
                    Text("Time Remaining")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(viewModel.timeUntilNextPhase)
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundColor(.white)
                }
            }
        }
        .padding()
        .background(
            LinearGradient(
                colors: [Color(viewModel.consensusState.currentPhase.color).opacity(0.2), Color.clear],
                startPoint: .leading,
                endPoint: .trailing
            )
        )
        .cornerRadius(16)
    }

    private var statsCard: some View {
        HStack(spacing: 16) {
            StatItem(
                title: "Your Voting Power",
                value: viewModel.votingPower.formattedPower,
                icon: "bolt.fill",
                color: .orange
            )

            Divider()
                .background(Color.white.opacity(0.3))
                .frame(height: 40)

            StatItem(
                title: "Active Proposals",
                value: "\(viewModel.activeProposalsCount)",
                icon: "doc.text.fill",
                color: .blue
            )

            Divider()
                .background(Color.white.opacity(0.3))
                .frame(height: 40)

            StatItem(
                title: "Your Votes",
                value: "\(viewModel.userVotesCount)",
                icon: "checkmark.circle.fill",
                color: .green
            )
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var categoryFilter: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 12) {
                CDCategoryChip(
                    category: nil,
                    isSelected: selectedCategory == nil,
                    label: "All"
                ) {
                    selectedCategory = nil
                }

                ForEach(ProposalCategory.allCases) { category in
                    CDCategoryChip(
                        category: category,
                        isSelected: selectedCategory == category,
                        label: category.rawValue
                    ) {
                        selectedCategory = category
                    }
                }
            }
            .padding(.horizontal)
        }
    }

    private var proposalsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Active Proposals")
                    .font(.title3)
                    .fontWeight(.bold)
                    .foregroundColor(.white)

                Spacer()

                Text("\(filteredProposals.count)")
                    .font(.subheadline)
                    .foregroundColor(.gray)
            }

            if filteredProposals.isEmpty {
                emptyProposalsView
            } else {
                ForEach(filteredProposals) { proposal in
                    NavigationLink {
                        CDProposalDetailView(proposal: proposal, viewModel: viewModel)
                    } label: {
                        ProposalCard(proposal: proposal)
                    }
                }
            }
        }
    }

    private var directorElectionSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "person.3.fill")
                    .foregroundColor(.indigo)

                Text("Director Election")
                    .font(.title3)
                    .fontWeight(.bold)
                    .foregroundColor(.white)

                Spacer()
            }

            Text("Vote for up to 7 directors to serve on the Foundation Board")
                .font(.caption)
                .foregroundColor(.gray)

            ForEach(viewModel.candidates) { candidate in
                NavigationLink {
                    DirectorCandidateDetailView(candidate: candidate, viewModel: viewModel)
                } label: {
                    CandidateCard(candidate: candidate)
                }
            }
        }
    }

    private var rewardsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Participation Rewards")
                    .font(.title3)
                    .fontWeight(.bold)
                    .foregroundColor(.white)

                Spacer()

                Button {
                    showRewards = true
                } label: {
                    Text("View All")
                        .font(.caption)
                        .foregroundColor(.blue)
                }
            }

            if let latestReward = viewModel.rewards.first {
                RewardCard(reward: latestReward) {
                    viewModel.claimReward(latestReward)
                }
            }
        }
    }

    private var emptyProposalsView: some View {
        VStack(spacing: 12) {
            Image(systemName: "doc.text.magnifyingglass")
                .font(.system(size: 48))
                .foregroundColor(.gray)

            Text("No Active Proposals")
                .font(.headline)
                .foregroundColor(.white)

            Text("Check back during the voting phase")
                .font(.caption)
                .foregroundColor(.gray)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 40)
    }

    private var filteredProposals: [GovernanceProposal] {
        if let category = selectedCategory {
            return viewModel.proposals.filter { $0.category == category }
        }
        return viewModel.proposals
    }
}

// MARK: - Category Chip
private struct CDCategoryChip: View {
    let category: ProposalCategory?
    let isSelected: Bool
    let label: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 6) {
                if let category = category {
                    Image(systemName: category.icon)
                        .font(.caption)
                }

                Text(label)
                    .font(.caption)
                    .fontWeight(.medium)
            }
            .padding(.horizontal, 16)
            .padding(.vertical, 8)
            .foregroundColor(isSelected ? .white : .gray)
            .background(
                isSelected ?
                    AnyView(LinearGradient(
                        colors: category != nil ? [Color(category!.color), Color(category!.color).opacity(0.7)] : [Color.blue, Color.purple],
                        startPoint: .leading,
                        endPoint: .trailing
                    )) :
                    AnyView(Color.white.opacity(0.1))
            )
            .cornerRadius(20)
        }
    }
}

// MARK: - Proposal Card
private struct ProposalCard: View {
    let proposal: GovernanceProposal

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                Image(systemName: proposal.category.icon)
                    .foregroundColor(Color(proposal.category.color))

                Text(proposal.category.rawValue)
                    .font(.caption)
                    .foregroundColor(Color(proposal.category.color))

                Spacer()

                HStack(spacing: 4) {
                    Image(systemName: "clock")
                        .font(.caption2)
                    Text(proposal.timeRemaining)
                        .font(.caption2)
                }
                .foregroundColor(.gray)
            }

            // Title
            Text(proposal.title)
                .font(.headline)
                .foregroundColor(.white)
                .lineLimit(2)

            // Description
            Text(proposal.description)
                .font(.caption)
                .foregroundColor(.gray)
                .lineLimit(2)

            // Voting Progress
            VStack(spacing: 8) {
                HStack {
                    HStack(spacing: 4) {
                        Image(systemName: "hand.thumbsup.fill")
                            .font(.caption2)
                            .foregroundColor(.green)
                        Text(String(format: "%.1f%%", proposal.forPercentage))
                            .font(.caption)
                            .foregroundColor(.white)
                    }

                    Spacer()

                    HStack(spacing: 4) {
                        Image(systemName: "hand.thumbsdown.fill")
                            .font(.caption2)
                            .foregroundColor(.red)
                        Text(String(format: "%.1f%%", proposal.againstPercentage))
                            .font(.caption)
                            .foregroundColor(.white)
                    }
                }

                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        RoundedRectangle(cornerRadius: 4)
                            .fill(Color.white.opacity(0.1))
                            .frame(height: 6)

                        RoundedRectangle(cornerRadius: 4)
                            .fill(proposal.isPassing ? Color.green : Color.red)
                            .frame(width: geometry.size.width * CGFloat(proposal.forPercentage / 100), height: 6)
                    }
                }
                .frame(height: 6)
            }

            // Status Badge
            HStack {
                Image(systemName: proposal.isPassing ? "checkmark.circle.fill" : "xmark.circle.fill")
                    .font(.caption)
                Text(proposal.isPassing ? "Passing" : "Not Passing")
                    .font(.caption)
                    .fontWeight(.medium)
            }
            .foregroundColor(proposal.isPassing ? .green : .orange)
            .padding(.horizontal, 10)
            .padding(.vertical, 4)
            .background((proposal.isPassing ? Color.green : Color.orange).opacity(0.2))
            .cornerRadius(8)
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

// MARK: - Candidate Card
private struct CandidateCard: View {
    let candidate: DirectorCandidate

    var body: some View {
        HStack(spacing: 12) {
            // Avatar
            Circle()
                .fill(Color.indigo.opacity(0.2))
                .frame(width: 50, height: 50)
                .overlay(
                    Text(String(candidate.displayName.prefix(1)))
                        .font(.title3)
                        .fontWeight(.bold)
                        .foregroundColor(.indigo)
                )

            // Info
            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(candidate.displayName)
                        .font(.headline)
                        .foregroundColor(.white)

                    if candidate.isIncumbent {
                        Text("Incumbent")
                            .font(.caption2)
                            .foregroundColor(.blue)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(Color.blue.opacity(0.2))
                            .cornerRadius(4)
                    }
                }

                if let bio = candidate.bio {
                    Text(bio)
                        .font(.caption)
                        .foregroundColor(.gray)
                        .lineLimit(2)
                }

                HStack(spacing: 4) {
                    Image(systemName: "person.fill.checkmark")
                        .font(.caption2)
                    Text(candidate.formattedVotes)
                        .font(.caption)
                }
                .foregroundColor(.indigo)
            }

            Spacer()

            Image(systemName: "chevron.right")
                .font(.caption)
                .foregroundColor(.gray)
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

// MARK: - Reward Card
private struct RewardCard: View {
    let reward: ParticipationReward
    let onClaim: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "gift.fill")
                    .foregroundColor(.green)

                Text("Consensus Day \(reward.consensusDayDate.formatted(date: .abbreviated, time: .omitted))")
                    .font(.headline)
                    .foregroundColor(.white)

                Spacer()
            }

            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Participation")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text("\(reward.votedProposals)/\(reward.totalProposals) proposals")
                        .font(.subheadline)
                        .foregroundColor(.white)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Reward")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(reward.formattedReward)
                        .font(.headline)
                        .foregroundColor(.green)
                }
            }

            // Progress Bar
            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    RoundedRectangle(cornerRadius: 4)
                        .fill(Color.white.opacity(0.1))
                        .frame(height: 6)

                    RoundedRectangle(cornerRadius: 4)
                        .fill(Color.green)
                        .frame(width: geometry.size.width * CGFloat(reward.completionPercentage / 100), height: 6)
                }
            }
            .frame(height: 6)

            if !reward.claimed {
                Button(action: onClaim) {
                    Text("Claim Reward")
                        .font(.subheadline)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 10)
                        .background(
                            LinearGradient(
                                colors: [Color.green, Color.blue],
                                startPoint: .leading,
                                endPoint: .trailing
                            )
                        )
                        .cornerRadius(10)
                }
            } else {
                HStack {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                    Text("Claimed")
                        .font(.subheadline)
                        .foregroundColor(.gray)
                }
                .frame(maxWidth: .infinity)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

// MARK: - Helper Views
private struct StatItem: View {
    let title: String
    let value: String
    let icon: String
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundColor(color)

            Text(value)
                .font(.headline)
                .foregroundColor(.white)

            Text(title)
                .font(.caption2)
                .foregroundColor(.gray)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
    }
}

