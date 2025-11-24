//
//  VotingPowerDetailView.swift
//  EtridWallet
//
//  Detailed view of voting power calculation
//

import SwiftUI

struct VotingPowerDetailView: View {
    let info: VotingPowerInfo
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(alignment: .leading, spacing: 24) {
                    // Total Voting Power
                    VStack(spacing: 12) {
                        Text("Your Voting Power")
                            .font(.headline)
                            .foregroundColor(.gray)

                        Text(info.formattedPower)
                            .font(.system(size: 48, weight: .bold))
                            .foregroundColor(.white)

                        Text("Total voting power for governance")
                            .font(.caption)
                            .foregroundColor(.gray)
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(
                        LinearGradient(
                            colors: [Color.orange, Color.red],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    )
                    .cornerRadius(16)

                    // Calculation Breakdown
                    VStack(alignment: .leading, spacing: 16) {
                        Text("How It's Calculated")
                            .font(.title3)
                            .fontWeight(.bold)
                            .foregroundColor(.white)

                        // Base Stake
                        CalculationRow(
                            icon: "dollarsign.circle.fill",
                            title: "Base Stake",
                            value: info.formattedStake,
                            color: .blue,
                            multiplier: nil
                        )

                        // Duration Multiplier
                        CalculationRow(
                            icon: "clock.fill",
                            title: "Duration Bonus",
                            value: "\(info.stakeDuration) days",
                            color: .green,
                            multiplier: info.durationMultiplier
                        )

                        Text("Stake for 90+ days: 1.1× | 180+ days: 1.2×")
                            .font(.caption)
                            .foregroundColor(.gray)
                            .padding(.leading, 44)

                        // History Multiplier
                        CalculationRow(
                            icon: "chart.line.uptrend.xyaxis",
                            title: "Participation History",
                            value: "\(info.participationHistory) events",
                            color: .purple,
                            multiplier: info.historyMultiplier
                        )

                        Text("Each past participation: +2% (max 10%)")
                            .font(.caption)
                            .foregroundColor(.gray)
                            .padding(.leading, 44)

                        Divider()
                            .background(Color.white.opacity(0.3))

                        // Total Multiplier
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text("Total Multiplier")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)

                                Text(String(format: "%.2f×", info.powerMultiplier))
                                    .font(.title2)
                                    .fontWeight(.bold)
                                    .foregroundColor(.white)
                            }

                            Spacer()

                            VStack(alignment: .trailing, spacing: 4) {
                                Text("Final Power")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)

                                Text(info.formattedPower)
                                    .font(.title2)
                                    .fontWeight(.bold)
                                    .foregroundColor(.orange)
                            }
                        }
                        .padding()
                        .background(Color.white.opacity(0.05))
                        .cornerRadius(12)
                    }
                    .padding()
                    .background(Color.white.opacity(0.05))
                    .cornerRadius(16)

                    // Formula Explanation
                    VStack(alignment: .leading, spacing: 12) {
                        HStack(spacing: 8) {
                            Image(systemName: "function")
                                .foregroundColor(.blue)

                            Text("Voting Power Formula")
                                .font(.headline)
                                .foregroundColor(.white)
                        }

                        VStack(alignment: .leading, spacing: 8) {
                            Text("Base Power = √(Stake × Coinage)")
                                .font(.system(.subheadline, design: .monospaced))
                                .foregroundColor(.gray)

                            Text("Duration Multiplier:")
                                .font(.caption)
                                .foregroundColor(.gray)
                            Text("• < 90 days: 1.0×")
                                .font(.caption)
                                .foregroundColor(.gray)
                            Text("• 90-179 days: 1.1×")
                                .font(.caption)
                                .foregroundColor(.gray)
                            Text("• 180+ days: 1.2×")
                                .font(.caption)
                                .foregroundColor(.gray)

                            Text("History Multiplier:")
                                .font(.caption)
                                .foregroundColor(.gray)
                                .padding(.top, 4)
                            Text("• +2% per past Consensus Day")
                                .font(.caption)
                                .foregroundColor(.gray)
                            Text("• Maximum 10% bonus")
                                .font(.caption)
                                .foregroundColor(.gray)
                        }
                    }
                    .padding()
                    .background(Color.blue.opacity(0.1))
                    .cornerRadius(12)

                    // Tips
                    VStack(alignment: .leading, spacing: 12) {
                        HStack(spacing: 8) {
                            Image(systemName: "lightbulb.fill")
                                .foregroundColor(.yellow)

                            Text("Increase Your Voting Power")
                                .font(.headline)
                                .foregroundColor(.white)
                        }

                        VStack(alignment: .leading, spacing: 8) {
                            CDTipRow(
                                icon: "plus.circle",
                                text: "Stake more ËTR tokens"
                            )

                            CDTipRow(
                                icon: "lock.fill",
                                text: "Keep tokens staked for longer periods"
                            )

                            CDTipRow(
                                icon: "calendar.badge.checkmark",
                                text: "Participate in every Consensus Day"
                            )
                        }
                    }
                    .padding()
                    .background(Color.yellow.opacity(0.1))
                    .cornerRadius(12)
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
            .navigationTitle("Voting Power")
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
}

// MARK: - Calculation Row
private struct CalculationRow: View {
    let icon: String
    let title: String
    let value: String
    let color: Color
    let multiplier: Double?

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundColor(color)
                .frame(width: 32)

            VStack(alignment: .leading, spacing: 4) {
                Text(title)
                    .font(.subheadline)
                    .foregroundColor(.gray)

                Text(value)
                    .font(.body)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
            }

            Spacer()

            if let multiplier = multiplier {
                Text(String(format: "%.2f×", multiplier))
                    .font(.headline)
                    .fontWeight(.bold)
                    .foregroundColor(color)
            }
        }
    }
}

// MARK: - Tip Row
private struct CDTipRow: View {
    let icon: String
    let text: String

    var body: some View {
        HStack(spacing: 8) {
            Image(systemName: icon)
                .font(.caption)
                .foregroundColor(.yellow)
                .frame(width: 20)

            Text(text)
                .font(.caption)
                .foregroundColor(.gray)

            Spacer()
        }
    }
}

#Preview {
    VotingPowerDetailView(
        info: VotingPowerInfo(
            stakedAmount: String(UInt64(10 * 1e18)),
            stakeDuration: 180,
            participationHistory: 3,
            calculatedPower: String(UInt64(12 * 1e18)),
            durationMultiplier: 1.1,
            historyMultiplier: 1.05
        )
    )
}
