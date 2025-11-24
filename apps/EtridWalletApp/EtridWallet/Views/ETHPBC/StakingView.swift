//
//  StakingView.swift
//  EtridWallet
//
//  Staking View
//

import SwiftUI

struct StakingView: View {
    @StateObject private var service = ETHPBCService()
    @State private var stakingInfo: StakingInfo?
    @State private var isStaking = true
    @State private var amount = ""
    @State private var lockPeriod = 30 // days
    @State private var isProcessing = false
    @State private var showSuccess = false
    @State private var errorMessage: String?

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // Staking Overview
                if let info = stakingInfo {
                    VStack(spacing: 16) {
                        // Total Staked
                        VStack(spacing: 8) {
                            Text("Total Staked")
                                .font(.subheadline)
                                .foregroundColor(.gray)
                            Text(info.stakedDisplay)
                                .font(.system(size: 36, weight: .bold))
                                .foregroundColor(.blue)
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)

                        // Stats Grid
                        HStack(spacing: 12) {
                            StatCard(
                                title: "Rewards",
                                value: info.rewardsDisplay,
                                icon: "gift.fill",
                                color: .green
                            )

                            StatCard(
                                title: "APR",
                                value: info.aprDisplay,
                                icon: "percent",
                                color: .orange
                            )
                        }
                    }
                } else {
                    ProgressView("Loading staking info...")
                        .padding()
                }

                // Stake/Unstake Toggle
                VStack(alignment: .leading, spacing: 12) {
                    Text("Action")
                        .font(.headline)

                    HStack(spacing: 12) {
                        ActionButton(
                            title: "Stake",
                            icon: "lock.fill",
                            isSelected: isStaking,
                            action: { isStaking = true }
                        )

                        ActionButton(
                            title: "Unstake",
                            icon: "lock.open.fill",
                            isSelected: !isStaking,
                            action: { isStaking = false }
                        )
                    }
                }

                // Amount Input
                VStack(alignment: .leading, spacing: 12) {
                    HStack {
                        Text("Amount")
                            .font(.headline)
                        Spacer()
                        Button("Max") {
                            if let info = stakingInfo {
                                amount = isStaking ? "1000" : info.stakedAmount // Simplified
                            }
                        }
                        .font(.subheadline)
                        .foregroundColor(.blue)
                    }

                    HStack {
                        TextField("0.0", text: $amount)
                            .keyboardType(.decimalPad)
                            .font(.title2)

                        Text("ETR")
                            .font(.headline)
                            .foregroundColor(.gray)
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(12)
                }

                // Lock Period (for staking only)
                if isStaking {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Lock Period")
                            .font(.headline)

                        VStack(spacing: 8) {
                            Text("\(lockPeriod) days")
                                .font(.title3)
                                .fontWeight(.bold)
                                .foregroundColor(.blue)

                            Slider(value: Binding(
                                get: { Double(lockPeriod) },
                                set: { lockPeriod = Int($0) }
                            ), in: 7...365, step: 1)

                            HStack {
                                Text("7 days")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                                Spacer()
                                Text("365 days")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)

                        // Lock period info
                        HStack {
                            Image(systemName: "info.circle.fill")
                                .foregroundColor(.blue)
                            Text("Longer lock periods earn higher APR")
                                .font(.caption)
                                .foregroundColor(.gray)
                        }
                        .padding()
                        .background(Color.blue.opacity(0.1))
                        .cornerRadius(8)
                    }
                }

                // Rewards Calculator (for staking)
                if isStaking, !amount.isEmpty, let value = Double(amount), let info = stakingInfo {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Estimated Rewards")
                            .font(.headline)

                        VStack(spacing: 12) {
                            HStack {
                                Text("Daily")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)
                                Spacer()
                                Text(String(format: "%.4f ETR", value * info.apr / 100 / 365))
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }

                            HStack {
                                Text("Monthly")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)
                                Spacer()
                                Text(String(format: "%.4f ETR", value * info.apr / 100 / 12))
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                            }

                            HStack {
                                Text("Yearly")
                                    .font(.subheadline)
                                    .foregroundColor(.gray)
                                Spacer()
                                Text(String(format: "%.4f ETR", value * info.apr / 100))
                                    .font(.subheadline)
                                    .fontWeight(.bold)
                                    .foregroundColor(.green)
                            }
                        }
                        .padding()
                        .background(Color(.systemGray6))
                        .cornerRadius(12)
                    }
                }

                // Can Unstake Check
                if !isStaking, let info = stakingInfo, !info.canUnstake {
                    HStack {
                        Image(systemName: "lock.fill")
                            .foregroundColor(.orange)
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Tokens are locked")
                                .font(.subheadline)
                                .fontWeight(.medium)
                            if let blocksRemaining = info.blocksUntilUnlock {
                                Text("Unlock in approximately \(blocksRemaining) blocks")
                                    .font(.caption)
                                    .foregroundColor(.gray)
                            }
                        }
                        Spacer()
                    }
                    .padding()
                    .background(Color.orange.opacity(0.1))
                    .cornerRadius(12)
                }

                // Error Message
                if let error = errorMessage {
                    Text(error)
                        .font(.subheadline)
                        .foregroundColor(.red)
                        .padding()
                        .background(Color.red.opacity(0.1))
                        .cornerRadius(8)
                }

                // Execute Button
                Button(action: executeStaking) {
                    if isProcessing {
                        ProgressView()
                            .progressViewStyle(CircularProgressViewStyle(tint: .white))
                    } else {
                        HStack {
                            Image(systemName: isStaking ? "lock.fill" : "lock.open.fill")
                            Text(isStaking ? "Stake Tokens" : "Unstake Tokens")
                        }
                        .font(.headline)
                    }
                }
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(canExecute ? Color.blue : Color.gray)
                .cornerRadius(12)
                .disabled(!canExecute || isProcessing)
            }
            .padding()
        }
        .navigationTitle("Staking")
        .navigationBarTitleDisplayMode(.inline)
        .onAppear(perform: loadStakingInfo)
        .alert(isPresented: $showSuccess) {
            Alert(
                title: Text("Success!"),
                message: Text(isStaking ? "Successfully staked \(amount) ETR" : "Successfully unstaked \(amount) ETR"),
                dismissButton: .default(Text("OK")) {
                    amount = ""
                    loadStakingInfo()
                }
            )
        }
    }

    private var canExecute: Bool {
        guard !amount.isEmpty, let value = Double(amount), value > 0 else {
            return false
        }

        if !isStaking, let info = stakingInfo {
            return info.canUnstake
        }

        return true
    }

    private func loadStakingInfo() {
        Task {
            do {
                // Replace with actual wallet address
                let address = "0x1234567890123456789012345678901234567890"
                stakingInfo = try await service.getStakingInfo(address: address)
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }

    private func executeStaking() {
        guard canExecute else { return }

        isProcessing = true
        errorMessage = nil

        Task {
            do {
                if isStaking {
                    // Convert days to blocks (assuming 6 second block time)
                    let blocks = lockPeriod * 24 * 60 * 10
                    _ = try await service.stakeTokens(amount: amount, lockPeriod: blocks)
                } else {
                    _ = try await service.unstakeTokens(amount: amount)
                }
                showSuccess = true
            } catch {
                errorMessage = error.localizedDescription
            }
            isProcessing = false
        }
    }
}

// MARK: - Stat Card
private struct StatCard: View {
    let title: String
    let value: String
    let icon: String
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(color)

            Text(title)
                .font(.caption)
                .foregroundColor(.gray)

            Text(value)
                .font(.title3)
                .fontWeight(.bold)
                .foregroundColor(color)
        }
        .frame(maxWidth: .infinity)
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

// MARK: - Action Button
private struct ActionButton: View {
    let title: String
    let icon: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundColor(isSelected ? .white : .blue)

                Text(title)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(isSelected ? .white : .primary)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(isSelected ? Color.blue : Color(.systemGray6))
            .cornerRadius(12)
        }
    }
}

struct StakingView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            StakingView()
        }
    }
}
