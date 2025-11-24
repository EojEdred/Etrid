//
//  RewardsHistoryView.swift
//  EtridWallet
//
//  History of participation rewards
//

import SwiftUI

struct RewardsHistoryView: View {
    let rewards: [ParticipationReward]
    @Environment(\.dismiss) private var dismiss

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
                        // Summary Card
                        summaryCard

                        // Rewards List
                        VStack(alignment: .leading, spacing: 12) {
                            Text("Reward History")
                                .font(.title3)
                                .fontWeight(.bold)
                                .foregroundColor(.white)

                            if rewards.isEmpty {
                                emptyStateView
                            } else {
                                ForEach(rewards) { reward in
                                    RewardHistoryCard(reward: reward)
                                }
                            }
                        }
                    }
                    .padding()
                }
            }
            .navigationTitle("Participation Rewards")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }

    private var summaryCard: some View {
        VStack(spacing: 16) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Total Earned")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(totalEarnedFormatted)
                        .font(.system(size: 32, weight: .bold))
                        .foregroundColor(.green)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Unclaimed")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(totalUnclaimedFormatted)
                        .font(.title3)
                        .fontWeight(.bold)
                        .foregroundColor(.orange)
                }
            }

            Divider()
                .background(Color.white.opacity(0.3))

            HStack {
                StatBox(
                    title: "Events",
                    value: "\(rewards.count)",
                    icon: "calendar",
                    color: .blue
                )

                StatBox(
                    title: "Claimed",
                    value: "\(claimedCount)",
                    icon: "checkmark.circle",
                    color: .green
                )

                StatBox(
                    title: "Pending",
                    value: "\(unclaimedCount)",
                    icon: "clock",
                    color: .orange
                )
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var emptyStateView: some View {
        VStack(spacing: 16) {
            Image(systemName: "gift")
                .font(.system(size: 48))
                .foregroundColor(.gray)

            Text("No Rewards Yet")
                .font(.headline)
                .foregroundColor(.white)

            Text("Participate in Consensus Day to earn rewards")
                .font(.caption)
                .foregroundColor(.gray)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 40)
    }

    private var totalEarnedFormatted: String {
        let total = rewards.compactMap { Double($0.totalReward) }.reduce(0, +)
        return String(format: "%.4f ËTR", total / 1e18)
    }

    private var totalUnclaimedFormatted: String {
        let total = rewards
            .filter { !$0.claimed }
            .compactMap { Double($0.totalReward) }
            .reduce(0, +)
        return String(format: "%.4f ËTR", total / 1e18)
    }

    private var claimedCount: Int {
        rewards.filter { $0.claimed }.count
    }

    private var unclaimedCount: Int {
        rewards.filter { !$0.claimed }.count
    }
}

// MARK: - Reward History Card
private struct RewardHistoryCard: View {
    let reward: ParticipationReward

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Consensus Day")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(reward.consensusDayDate.formatted(date: .long, time: .omitted))
                        .font(.headline)
                        .foregroundColor(.white)
                }

                Spacer()

                if reward.claimed {
                    HStack(spacing: 4) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.green)
                        Text("Claimed")
                            .font(.caption)
                            .foregroundColor(.green)
                    }
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(Color.green.opacity(0.2))
                    .cornerRadius(6)
                } else {
                    HStack(spacing: 4) {
                        Image(systemName: "clock.fill")
                            .foregroundColor(.orange)
                        Text("Pending")
                            .font(.caption)
                            .foregroundColor(.orange)
                    }
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(Color.orange.opacity(0.2))
                    .cornerRadius(6)
                }
            }

            // Participation Stats
            HStack(spacing: 20) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Participation")
                        .font(.caption)
                        .foregroundColor(.gray)

                    HStack(spacing: 4) {
                        Image(systemName: "doc.text")
                            .font(.caption)
                        Text("\(reward.votedProposals)/\(reward.totalProposals)")
                            .font(.subheadline)
                            .fontWeight(.medium)
                    }
                    .foregroundColor(.white)
                }

                VStack(alignment: .leading, spacing: 4) {
                    Text("Completion")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(String(format: "%.0f%%", reward.completionPercentage))
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundColor(.white)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Reward")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(reward.formattedReward)
                        .font(.headline)
                        .fontWeight(.bold)
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
                        .fill(
                            reward.hasCompletenessBonus ?
                                LinearGradient(
                                    colors: [Color.green, Color.blue],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                ) :
                                LinearGradient(
                                    colors: [Color.orange, Color.yellow],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                        )
                        .frame(width: geometry.size.width * CGFloat(reward.completionPercentage / 100), height: 6)
                }
            }
            .frame(height: 6)

            // Bonus Badge
            if reward.hasCompletenessBonus {
                HStack(spacing: 6) {
                    Image(systemName: "star.fill")
                        .font(.caption2)
                    Text("Completeness Bonus +20%")
                        .font(.caption)
                        .fontWeight(.medium)
                }
                .foregroundColor(.yellow)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(Color.yellow.opacity(0.2))
                .cornerRadius(6)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

// MARK: - Stat Box
private struct StatBox: View {
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
        }
        .frame(maxWidth: .infinity)
    }
}

#Preview {
    RewardsHistoryView(
        rewards: [
            ParticipationReward(
                id: "1",
                consensusDayDate: Date().addingTimeInterval(-365 * 86400),
                votedProposals: 10,
                totalProposals: 10,
                baseReward: String(UInt64(1 * 1e18)),
                completenessBonus: String(UInt64(1 * 1e17)),
                totalReward: String(UInt64(1 * 1e18)),
                claimed: true
            ),
            ParticipationReward(
                id: "2",
                consensusDayDate: Date().addingTimeInterval(-730 * 86400),
                votedProposals: 8,
                totalProposals: 10,
                baseReward: String(UInt64(1 * 1e18)),
                completenessBonus: "0",
                totalReward: String(UInt64(1 * 1e18)),
                claimed: false
            )
        ]
    )
}
