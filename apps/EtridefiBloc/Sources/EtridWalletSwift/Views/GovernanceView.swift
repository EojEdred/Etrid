import SwiftUI
#if os(iOS)
import UIKit
#elseif os(macOS)
import AppKit
#endif

// MARK: - Governance View

struct GovernanceView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var dashboard: GovernanceDashboard?
    @State private var isLoading = true
    @State private var selectedTab: GovernanceTab = .proposals
    @State private var showSubmitProposal = false
    @State private var currentPhase: ConsensusDayPhase = .inactive

    enum GovernanceTab: String, CaseIterable {
        case proposals = "Proposals"
        case directors = "Directors"
        case staking = "Staking"
        case rewards = "Rewards"
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                // Calendar-based Consensus Day Phase Banner (always shown)
                CalendarPhaseBanner(phase: currentPhase)

                // Voting Power Card
                if let dashboard = dashboard {
                    VotingPowerCard(dashboard: dashboard)
                        .padding()
                }

                // Tab Selector
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 12) {
                        ForEach(GovernanceTab.allCases, id: \.self) { tab in
                            CategoryChip(
                                title: tab.rawValue,
                                isSelected: selectedTab == tab,
                                action: { selectedTab = tab }
                            )
                        }
                    }
                    .padding(.horizontal)
                }
                .padding(.vertical, 8)

                // Content
                TabView(selection: $selectedTab) {
                    ProposalsListView(proposals: dashboard?.activeProposals ?? [], userVotes: dashboard?.userVotes ?? [])
                        .tag(GovernanceTab.proposals)

                    DirectorsListView(directors: dashboard?.directors ?? [])
                        .tag(GovernanceTab.directors)

                    StakingView()
                        .tag(GovernanceTab.staking)

                    RewardsListView(rewards: dashboard?.pendingRewards ?? [])
                        .tag(GovernanceTab.rewards)
                }
                #if os(iOS)
                .tabViewStyle(.page(indexDisplayMode: .never))
                #endif
            }
            .navigationTitle("Consënsus")
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .topBarTrailing) {
                    Button(action: { showSubmitProposal = true }) {
                        Image(systemName: "plus.circle.fill")
                    }
                }
                ToolbarItem(placement: .topBarTrailing) {
                    Button(action: { Task { await loadDashboard() } }) {
                        Image(systemName: "arrow.clockwise")
                    }
                }
                #else
                ToolbarItem(placement: .automatic) {
                    Button(action: { showSubmitProposal = true }) {
                        Image(systemName: "plus.circle.fill")
                    }
                }
                ToolbarItem(placement: .automatic) {
                    Button(action: { Task { await loadDashboard() } }) {
                        Image(systemName: "arrow.clockwise")
                    }
                }
                #endif
            }
            .sheet(isPresented: $showSubmitProposal) {
                SubmitProposalView()
            }
            .task {
                await loadDashboard()
            }
            .refreshable {
                await loadDashboard()
            }
            .overlay {
                if isLoading {
                    ProgressView("Loading governance data...")
                }
            }
        }
    }

    private func loadDashboard() async {
        isLoading = true

        // Set current phase from calendar (authoritative source)
        currentPhase = ConsensusDayPhase.currentPhase()

        do {
            dashboard = try await walletManager.getGovernanceDashboard()
        } catch {
            print("Failed to load governance dashboard: \(error)")
        }
        isLoading = false
    }
}

// MARK: - Calendar Phase Banner

struct CalendarPhaseBanner: View {
    let phase: ConsensusDayPhase

    var body: some View {
        VStack(spacing: 8) {
            HStack {
                Image(systemName: phase.icon)
                    .font(.title2)

                VStack(alignment: .leading, spacing: 2) {
                    Text(phaseTitle)
                        .font(.headline)
                        .fontWeight(.bold)

                    Text(phase.rawValue)
                        .font(.subheadline)
                }

                Spacer()

                VStack(alignment: .trailing) {
                    Text(countdownText)
                        .font(.caption)
                        .fontWeight(.semibold)
                    Text(countdownSubtext)
                        .font(.caption2)
                        .opacity(0.8)
                }
            }

            // Phase progress indicator
            HStack(spacing: 4) {
                ForEach(phases, id: \.0) { (name, isCurrent) in
                    Rectangle()
                        .fill(isCurrent ? Color.white : Color.white.opacity(0.3))
                        .frame(height: 4)
                        .cornerRadius(2)
                }
            }
        }
        .padding()
        .background(bannerGradient)
        .foregroundColor(.white)
    }

    private var phaseTitle: String {
        let year = Calendar.current.component(.year, from: Date())
        switch phase {
        case .voting:
            return "Consensus Day \(year)"
        case .registration:
            return "Registration Period"
        case .execution:
            return "Execution Period"
        case .distribution, .minting:
            return "Distribution Day"
        case .inactive:
            return "Governance"
        }
    }

    private var countdownText: String {
        let daysUntil = ConsensusDayPhase.daysUntilConsensusDay()
        switch phase {
        case .voting:
            return "TODAY"
        case .registration:
            return "\(daysUntil) days"
        case .execution:
            let daysLeft = 30 - Calendar.current.component(.day, from: Date())
            return "\(daysLeft) days"
        case .distribution, .minting:
            return "FINAL DAY"
        case .inactive:
            return "--"
        }
    }

    private var countdownSubtext: String {
        switch phase {
        case .voting:
            return "Vote now!"
        case .registration:
            return "until Consensus Day"
        case .execution:
            return "until completion"
        case .distribution, .minting:
            return "of the year"
        case .inactive:
            return ""
        }
    }

    private var phases: [(String, Bool)] {
        [
            ("Registration", phase == .registration),
            ("Voting", phase == .voting),
            ("Execution", phase == .execution),
            ("Distribution", phase == .distribution || phase == .minting)
        ]
    }

    private var bannerGradient: LinearGradient {
        let colors: [Color]
        switch phase {
        case .voting:
            colors = [.orange, .red]
        case .registration:
            colors = [EtridColors.primary, EtridColors.secondary]
        case .execution:
            colors = [.purple, .blue]
        case .distribution, .minting:
            colors = [.green, .teal]
        case .inactive:
            colors = [.gray, .gray.opacity(0.7)]
        }
        return LinearGradient(colors: colors, startPoint: .leading, endPoint: .trailing)
    }
}

// MARK: - Consensus Day Banner (Legacy - RPC based)

struct ConsensusDayBanner: View {
    let state: ConsensusDayState

    var body: some View {
        VStack(spacing: 8) {
            HStack {
                Image(systemName: state.phase.icon)
                    .font(.title2)

                VStack(alignment: .leading, spacing: 2) {
                    Text("Consensus Day \(state.currentYear)")
                        .font(.headline)
                        .fontWeight(.bold)

                    Text(state.phase.rawValue)
                        .font(.subheadline)
                }

                Spacer()

                VStack(alignment: .trailing) {
                    Text(timeRemaining)
                        .font(.caption)
                        .fontWeight(.semibold)
                    Text("remaining")
                        .font(.caption2)
                        .foregroundColor(.secondary)
                }
            }

            // Progress Bar
            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    Rectangle()
                        .fill(Color.white.opacity(0.3))
                        .frame(height: 4)
                        .cornerRadius(2)

                    Rectangle()
                        .fill(Color.white)
                        .frame(width: geometry.size.width * state.phaseProgress, height: 4)
                        .cornerRadius(2)
                }
            }
            .frame(height: 4)
        }
        .padding()
        .background(
            LinearGradient(
                colors: [EtridColors.primary, EtridColors.secondary],
                startPoint: .leading,
                endPoint: .trailing
            )
        )
        .foregroundColor(.white)
    }

    private var timeRemaining: String {
        let remaining = state.timeRemainingInPhase
        let hours = Int(remaining) / 3600
        let minutes = (Int(remaining) % 3600) / 60

        if hours > 0 {
            return "\(hours)h \(minutes)m"
        } else {
            return "\(minutes)m"
        }
    }
}

// MARK: - Voting Power Card

struct VotingPowerCard: View {
    let dashboard: GovernanceDashboard

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Your Voting Power")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    Text("\((dashboard.effectiveVotingPower as NSDecimalNumber).doubleValue, specifier: "%.2f")")
                        .font(.title)
                        .fontWeight(.bold)
                        .foregroundColor(EtridColors.primary)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    HStack(spacing: 4) {
                        Image(systemName: "clock.fill")
                            .font(.caption2)
                        Text("+\(Int((dashboard.stakeDurationMultiplier - 1) * 100))%")
                            .font(.caption)
                    }
                    .foregroundColor(.green)

                    HStack(spacing: 4) {
                        Image(systemName: "star.fill")
                            .font(.caption2)
                        Text("+\(Int((dashboard.historyMultiplier - 1) * 100))%")
                            .font(.caption)
                    }
                    .foregroundColor(.orange)
                }
            }

            Divider()

            HStack {
                StatItem(title: "Staked", value: String(format: "%.0f ETR", (dashboard.userStake as NSDecimalNumber).doubleValue))
                Spacer()
                StatItem(title: "Proposals Voted", value: "\(dashboard.proposalsUserVotedOn.count)")
                Spacer()
                StatItem(title: "Rewards", value: String(format: "%.2f ETR", (dashboard.totalUnclaimedRewards as NSDecimalNumber).doubleValue))
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
    }
}

struct StatItem: View {
    let title: String
    let value: String

    var body: some View {
        VStack(spacing: 4) {
            Text(value)
                .font(.headline)
                .fontWeight(.semibold)
            Text(title)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
}

// MARK: - Proposals List View

struct ProposalsListView: View {
    let proposals: [GovernanceProposal]
    let userVotes: [VoteRecord]

    var body: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                if proposals.isEmpty {
                    EmptyProposalsView()
                } else {
                    ForEach(proposals) { proposal in
                        NavigationLink(destination: ProposalDetailView(proposal: proposal, userVote: userVote(for: proposal))) {
                            ProposalCard(proposal: proposal, hasVoted: hasVoted(proposal))
                        }
                        .buttonStyle(.plain)
                    }
                }
            }
            .padding()
        }
    }

    private func hasVoted(_ proposal: GovernanceProposal) -> Bool {
        userVotes.contains { $0.proposalId == proposal.id }
    }

    private func userVote(for proposal: GovernanceProposal) -> VoteRecord? {
        userVotes.first { $0.proposalId == proposal.id }
    }
}

struct EmptyProposalsView: View {
    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "doc.text.magnifyingglass")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("No Active Proposals")
                .font(.headline)

            Text("Proposals will appear here during Consensus Day")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .padding(40)
    }
}

struct ProposalCard: View {
    let proposal: GovernanceProposal
    let hasVoted: Bool

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                // Category Badge
                HStack(spacing: 4) {
                    Image(systemName: proposal.category.icon)
                    Text(proposal.category.rawValue)
                }
                .font(.caption)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(EtridColors.primary.opacity(0.2))
                .cornerRadius(8)

                Spacer()

                // Status Badge
                Text(proposal.status.rawValue)
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(statusColor)

                if hasVoted {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(.green)
                }
            }

            Text(proposal.title)
                .font(.headline)
                .lineLimit(2)

            Text(proposal.description)
                .font(.caption)
                .foregroundColor(.secondary)
                .lineLimit(2)

            // Vote Progress
            VStack(spacing: 4) {
                HStack {
                    Text("For: \(proposal.votesFor)")
                        .font(.caption)
                        .foregroundColor(.green)
                    Spacer()
                    Text("Against: \(proposal.votesAgainst)")
                        .font(.caption)
                        .foregroundColor(.red)
                }

                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color.red.opacity(0.3))
                            .frame(height: 6)
                            .cornerRadius(3)

                        Rectangle()
                            .fill(Color.green)
                            .frame(width: geometry.size.width * (proposal.approvalPercentage / 100), height: 6)
                            .cornerRadius(3)
                    }
                }
                .frame(height: 6)

                Text("\(proposal.approvalPercentage, specifier: "%.1f")% approval")
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }

            if let timeRemaining = proposal.timeRemaining {
                Text(timeRemaining)
                    .font(.caption)
                    .foregroundColor(.orange)
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }

    private var statusColor: Color {
        switch proposal.status {
        case .active: return .blue
        case .approved: return .green
        case .rejected: return .red
        case .executed: return .purple
        default: return .gray
        }
    }
}

// MARK: - Proposal Detail View

struct ProposalDetailView: View {
    let proposal: GovernanceProposal
    let userVote: VoteRecord?
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var isVoting = false
    @State private var selectedBallot: Ballot?
    @State private var showVoteConfirmation = false
    @State private var errorMessage: String?

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                // Header
                VStack(alignment: .leading, spacing: 8) {
                    HStack {
                        Image(systemName: proposal.category.icon)
                        Text(proposal.category.rawValue)
                    }
                    .font(.subheadline)
                    .foregroundColor(EtridColors.primary)

                    Text(proposal.title)
                        .font(.title2)
                        .fontWeight(.bold)

                    Text("Proposed by \(truncatedAddress(proposal.proposer))")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Divider()

                // Description
                Text(proposal.description)
                    .font(.body)

                // Budget Info (if applicable)
                if let amount = proposal.budgetAmount, let category = proposal.budgetCategory {
                    BudgetInfoCard(amount: amount, category: category)
                }

                Divider()

                // Vote Stats
                VoteStatsView(proposal: proposal)

                // User's Vote
                if let vote = userVote {
                    UserVoteCard(vote: vote)
                }

                // Voting Buttons
                if proposal.isVotingActive && userVote == nil {
                    VotingButtons(
                        selectedBallot: $selectedBallot,
                        isVoting: isVoting,
                        onVote: { showVoteConfirmation = true }
                    )
                }

                if let error = errorMessage {
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                }
            }
            .padding()
        }
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        #endif
        .alert("Confirm Vote", isPresented: $showVoteConfirmation) {
            Button("Cancel", role: .cancel) { }
            Button("Vote") { Task { await submitVote() } }
        } message: {
            Text("Vote \(selectedBallot?.rawValue ?? "") on this proposal?")
        }
    }

    private func truncatedAddress(_ address: String) -> String {
        guard address.count > 12 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }

    private func submitVote() async {
        guard let ballot = selectedBallot,
              let governance = walletManager.getGovernanceService(),
              let address = walletManager.selectedAccount?.address else { return }

        isVoting = true
        errorMessage = nil

        do {
            _ = try await governance.castVote(
                proposalId: proposal.id,
                ballot: ballot,
                fromAddress: address
            )
        } catch {
            errorMessage = error.localizedDescription
        }

        isVoting = false
    }
}

struct BudgetInfoCard: View {
    let amount: Decimal
    let category: BudgetCategory

    var body: some View {
        HStack {
            Image(systemName: category.icon)
                .font(.title2)
                .foregroundColor(EtridColors.primary)

            VStack(alignment: .leading, spacing: 4) {
                Text("Budget Request")
                    .font(.caption)
                    .foregroundColor(.secondary)
                Text("\((amount as NSDecimalNumber).doubleValue, specifier: "%.0f") ETR")
                    .font(.headline)
                Text(category.rawValue)
                    .font(.caption)
            }

            Spacer()
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

struct VoteStatsView: View {
    let proposal: GovernanceProposal

    var body: some View {
        VStack(spacing: 16) {
            Text("Vote Results")
                .font(.headline)

            HStack(spacing: 20) {
                VoteStatCircle(label: "For", count: Int(proposal.votesFor), color: .green)
                VoteStatCircle(label: "Against", count: Int(proposal.votesAgainst), color: .red)
            }

            // Approval threshold indicator
            VStack(spacing: 4) {
                GeometryReader { geometry in
                    ZStack(alignment: .leading) {
                        Rectangle()
                            .fill(Color.gray.opacity(0.3))
                            .frame(height: 12)
                            .cornerRadius(6)

                        Rectangle()
                            .fill(proposal.approvalPercentage >= proposal.category.requiredApproval ? Color.green : Color.orange)
                            .frame(width: geometry.size.width * (proposal.approvalPercentage / 100), height: 12)
                            .cornerRadius(6)

                        // Threshold marker
                        Rectangle()
                            .fill(Color.white)
                            .frame(width: 2, height: 16)
                            .offset(x: geometry.size.width * (proposal.category.requiredApproval / 100))
                    }
                }
                .frame(height: 16)

                HStack {
                    Text("\(proposal.approvalPercentage, specifier: "%.1f")% approval")
                    Spacer()
                    Text("Required: \(proposal.category.requiredApproval, specifier: "%.0f")%")
                        .foregroundColor(.secondary)
                }
                .font(.caption)
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

struct VoteStatCircle: View {
    let label: String
    let count: Int
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            ZStack {
                Circle()
                    .stroke(color.opacity(0.3), lineWidth: 8)
                    .frame(width: 80, height: 80)

                Circle()
                    .trim(from: 0, to: 0.75)
                    .stroke(color, lineWidth: 8)
                    .frame(width: 80, height: 80)
                    .rotationEffect(.degrees(-90))

                Text("\(count)")
                    .font(.title2)
                    .fontWeight(.bold)
            }
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
}

struct UserVoteCard: View {
    let vote: VoteRecord

    var body: some View {
        HStack {
            Image(systemName: vote.ballot.icon)
                .font(.title2)
                .foregroundColor(ballotColor)

            VStack(alignment: .leading, spacing: 4) {
                Text("Your Vote")
                    .font(.caption)
                    .foregroundColor(.secondary)
                Text(vote.ballot.rawValue)
                    .font(.headline)
                    .foregroundColor(ballotColor)
            }

            Spacer()

            Text("Voting Power: \((vote.votingPower as NSDecimalNumber).doubleValue, specifier: "%.0f")")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .padding()
        .background(ballotColor.opacity(0.1))
        .cornerRadius(12)
    }

    private var ballotColor: Color {
        switch vote.ballot {
        case .yes: return .green
        case .no: return .red
        case .abstain: return .gray
        }
    }
}

struct VotingButtons: View {
    @Binding var selectedBallot: Ballot?
    let isVoting: Bool
    let onVote: () -> Void

    var body: some View {
        VStack(spacing: 12) {
            Text("Cast Your Vote")
                .font(.headline)

            HStack(spacing: 12) {
                ForEach(Ballot.allCases, id: \.self) { ballot in
                    BallotButton(
                        ballot: ballot,
                        isSelected: selectedBallot == ballot,
                        onSelect: { selectedBallot = ballot }
                    )
                }
            }

            Button(action: onVote) {
                HStack {
                    if isVoting {
                        ProgressView()
                            .tint(.white)
                    } else {
                        Image(systemName: "checkmark.seal.fill")
                        Text("Submit Vote")
                    }
                }
                .font(.headline)
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(selectedBallot != nil ? EtridColors.primary : Color.gray)
                .cornerRadius(12)
            }
            .disabled(selectedBallot == nil || isVoting)
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
    }
}

struct BallotButton: View {
    let ballot: Ballot
    let isSelected: Bool
    let onSelect: () -> Void

    var body: some View {
        Button(action: onSelect) {
            VStack(spacing: 8) {
                Image(systemName: ballot.icon)
                    .font(.title)
                Text(ballot.rawValue)
                    .font(.caption)
                    .fontWeight(.medium)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(isSelected ? ballotColor.opacity(0.2) : EtridColors.cardBackground)
            .foregroundColor(isSelected ? ballotColor : .primary)
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? ballotColor : Color.clear, lineWidth: 2)
            )
        }
    }

    private var ballotColor: Color {
        switch ballot {
        case .yes: return .green
        case .no: return .red
        case .abstain: return .gray
        }
    }
}

// MARK: - Directors List View

struct DirectorsListView: View {
    let directors: [DirectorCandidate]

    var body: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                if directors.isEmpty {
                    Text("No elected directors")
                        .foregroundColor(.secondary)
                        .padding(40)
                } else {
                    ForEach(directors) { director in
                        DirectorCard(director: director)
                    }
                }
            }
            .padding()
        }
    }
}

struct DirectorCard: View {
    let director: DirectorCandidate

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Circle()
                    .fill(EtridColors.primary)
                    .frame(width: 50, height: 50)
                    .overlay(
                        Text(director.name.prefix(1))
                            .font(.title2)
                            .fontWeight(.bold)
                            .foregroundColor(.white)
                    )

                VStack(alignment: .leading, spacing: 4) {
                    Text(director.name)
                        .font(.headline)
                    Text(truncatedAddress(director.address))
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                if director.status == .elected {
                    Image(systemName: "checkmark.seal.fill")
                        .foregroundColor(.green)
                }
            }

            Text(director.manifesto)
                .font(.caption)
                .foregroundColor(.secondary)
                .lineLimit(3)

            HStack {
                Label("\(director.votesReceived) votes", systemImage: "hand.raised.fill")
                Spacer()
                Label("\((director.votingPowerReceived as NSDecimalNumber).doubleValue, specifier: "%.0f") VP", systemImage: "bolt.fill")
            }
            .font(.caption)
            .foregroundColor(.secondary)
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }

    private func truncatedAddress(_ address: String) -> String {
        guard address.count > 12 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }
}

// MARK: - Staking View

struct StakingView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var stakeInfo: StakeInfo?
    @State private var stakeAmount = ""
    @State private var isStaking = false
    @State private var showStakeSheet = false

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Current Stake Card
                if let info = stakeInfo {
                    CurrentStakeCard(info: info)
                } else {
                    NoStakeCard(onStake: { showStakeSheet = true })
                }

                // Stake/Unstake Actions
                HStack(spacing: 12) {
                    Button(action: { showStakeSheet = true }) {
                        HStack {
                            Image(systemName: "lock.fill")
                            Text("Stake")
                        }
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(EtridColors.primary)
                        .cornerRadius(12)
                    }

                    if stakeInfo != nil {
                        Button(action: { /* Show unstake */ }) {
                            HStack {
                                Image(systemName: "lock.open.fill")
                                Text("Unstake")
                            }
                            .font(.headline)
                            .foregroundColor(EtridColors.primary)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(EtridColors.cardBackground)
                            .cornerRadius(12)
                        }
                    }
                }

                // Staking Benefits
                StakingBenefitsCard()
            }
            .padding()
        }
        .task {
            await loadStakeInfo()
        }
        .sheet(isPresented: $showStakeSheet) {
            StakeAmountSheet(currentBalance: currentETRBalance)
        }
    }

    private var currentETRBalance: Decimal {
        walletManager.selectedAccount?.balances.first(where: { $0.token == .etrid })?.amount ?? 0
    }

    private func loadStakeInfo() async {
        guard let governance = walletManager.getGovernanceService(),
              let address = walletManager.selectedAccount?.address else { return }

        do {
            stakeInfo = try await governance.getStakingInfo(address: address)
        } catch {
            print("Failed to load stake info: \(error)")
        }
    }
}

struct CurrentStakeCard: View {
    let info: StakeInfo

    var body: some View {
        VStack(spacing: 16) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Staked Amount")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("\((info.stakedAmount as NSDecimalNumber).doubleValue, specifier: "%.2f") ETR")
                        .font(.title)
                        .fontWeight(.bold)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Rewards")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("\((info.rewards as NSDecimalNumber).doubleValue, specifier: "%.4f") ETR")
                        .font(.headline)
                        .foregroundColor(.green)
                }
            }

            Divider()

            HStack {
                VStack(alignment: .leading) {
                    Text("Validator")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text(info.validatorName)
                        .font(.subheadline)
                }

                Spacer()

                VStack(alignment: .trailing) {
                    Text("Multiplier")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("\(info.multiplier, specifier: "%.2f")x")
                        .font(.subheadline)
                        .foregroundColor(EtridColors.primary)
                }
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
    }
}

struct NoStakeCard: View {
    let onStake: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "lock.shield")
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text("Start Staking")
                .font(.headline)

            Text("Stake ETR to earn rewards and increase your voting power")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)

            Button(action: onStake) {
                Text("Stake Now")
                    .font(.headline)
                    .foregroundColor(.white)
                    .padding(.horizontal, 32)
                    .padding(.vertical, 12)
                    .background(EtridColors.primary)
                    .cornerRadius(12)
            }
        }
        .padding(24)
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
    }
}

struct StakingBenefitsCard: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Staking Benefits")
                .font(.headline)

            BenefitRow(icon: "bolt.fill", title: "Voting Power", description: "Increase your governance influence")
            BenefitRow(icon: "gift.fill", title: "Rewards", description: "Earn ETR from network fees")
            BenefitRow(icon: "clock.fill", title: "Duration Bonus", description: "Up to +20% for long-term staking")
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(16)
    }
}

struct BenefitRow: View {
    let icon: String
    let title: String
    let description: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(EtridColors.primary)
                .frame(width: 32)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.subheadline)
                    .fontWeight(.medium)
                Text(description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

struct StakeAmountSheet: View {
    let currentBalance: Decimal
    @Environment(\.dismiss) var dismiss
    @State private var amount = ""

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                AmountInputView(
                    amount: $amount,
                    token: .etrid,
                    balance: currentBalance
                )

                Spacer()

                Button(action: { /* Execute stake */ dismiss() }) {
                    Text("Stake ETR")
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(EtridColors.primary)
                        .cornerRadius(12)
                }
            }
            .padding()
            .navigationTitle("Stake ETR")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
        }
    }
}

// MARK: - Rewards List View

struct RewardsListView: View {
    let rewards: [ParticipationReward]

    var body: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                if rewards.isEmpty {
                    Text("No pending rewards")
                        .foregroundColor(.secondary)
                        .padding(40)
                } else {
                    ForEach(rewards) { reward in
                        RewardCard(reward: reward)
                    }
                }
            }
            .padding()
        }
    }
}

struct RewardCard: View {
    let reward: ParticipationReward

    var body: some View {
        HStack {
            Image(systemName: reward.type.icon)
                .font(.title2)
                .foregroundColor(EtridColors.primary)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 4) {
                Text(reward.type.rawValue)
                    .font(.subheadline)
                    .fontWeight(.medium)
                Text("Consensus Day \(reward.consensusDayYear)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text("\((reward.amount as NSDecimalNumber).doubleValue, specifier: "%.4f") ETR")
                    .font(.headline)
                    .foregroundColor(.green)

                if reward.isClaimed {
                    Text("Claimed")
                        .font(.caption)
                        .foregroundColor(.secondary)
                } else {
                    Button("Claim") {
                        // Claim reward
                    }
                    .font(.caption)
                    .foregroundColor(EtridColors.primary)
                }
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

// MARK: - Submit Proposal View

struct SubmitProposalView: View {
    @Environment(\.dismiss) var dismiss
    @EnvironmentObject var walletManager: EtridWalletManager

    @State private var title = ""
    @State private var description = ""
    @State private var category: ProposalCategory = .parameterChange
    @State private var budgetAmount = ""
    @State private var budgetCategory: BudgetCategory = .infrastructure
    @State private var isSubmitting = false
    @State private var errorMessage: String?
    @State private var isLoading = true
    @State private var userBalance: Decimal = 0
    @State private var consensusPhase: ConsensusDayPhase = .inactive
    @State private var isConsensusDayActive = false

    private let requiredBond: Decimal = 10_000

    var body: some View {
        NavigationStack {
            Form {
                // Requirements Status Section
                Section("Requirements") {
                    RequirementRow(
                        title: "Wallet Connected",
                        isMet: walletManager.selectedAccount != nil,
                        metText: walletManager.selectedAccount?.address.prefix(12).description ?? "Connected",
                        unmetText: "Connect wallet first"
                    )

                    RequirementRow(
                        title: "Bond Available",
                        isMet: userBalance >= requiredBond,
                        metText: String(format: "%.0f ETR available", (userBalance as NSDecimalNumber).doubleValue),
                        unmetText: String(format: "Need %.0f ETR (have %.0f)", (requiredBond as NSDecimalNumber).doubleValue, (userBalance as NSDecimalNumber).doubleValue)
                    )

                    RequirementRow(
                        title: "Registration Phase",
                        isMet: consensusPhase.allowsProposalSubmission,
                        metText: "Jan 1 - Nov 30: Proposals open",
                        unmetText: phaseStatusText
                    )
                }

                // Current Phase Info
                Section("Consensus Day Schedule") {
                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Image(systemName: consensusPhase.icon)
                                .foregroundColor(EtridColors.primary)
                            Text("Current Phase: \(consensusPhase.rawValue)")
                                .font(.headline)
                        }

                        Text(consensusPhase.description)
                            .font(.caption)
                            .foregroundColor(.secondary)

                        Divider()

                        PhaseScheduleRow(phase: "Registration", dates: "Jan 1 - Nov 30", isCurrent: consensusPhase == .registration)
                        PhaseScheduleRow(phase: "Voting", dates: "Dec 1", isCurrent: consensusPhase == .voting)
                        PhaseScheduleRow(phase: "Execution", dates: "Dec 2-30", isCurrent: consensusPhase == .execution)
                        PhaseScheduleRow(phase: "Distribution", dates: "Dec 31", isCurrent: consensusPhase == .distribution)

                        if consensusPhase != .voting {
                            let daysUntil = ConsensusDayPhase.daysUntilConsensusDay()
                            Text("\(daysUntil) days until next Consensus Day (Dec 1)")
                                .font(.caption)
                                .foregroundColor(.orange)
                                .padding(.top, 4)
                        }
                    }
                }

                Section("Proposal Details") {
                    TextField("Title (min 10 chars)", text: $title)

                    Picker("Category", selection: $category) {
                        ForEach(ProposalCategory.allCases, id: \.self) { cat in
                            Label(cat.rawValue, systemImage: cat.icon).tag(cat)
                        }
                    }

                    VStack(alignment: .leading) {
                        Text("Description (min 50 chars)")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        TextEditor(text: $description)
                            .frame(minHeight: 100)
                        Text("\(description.count)/50 characters")
                            .font(.caption2)
                            .foregroundColor(description.count >= 50 ? .green : .orange)
                    }
                }

                if category == .budgetAllocation {
                    Section("Budget Request") {
                        TextField("Amount (ETR)", text: $budgetAmount)
                            #if os(iOS)
                            .keyboardType(.decimalPad)
                            #endif

                        Picker("Budget Category", selection: $budgetCategory) {
                            ForEach(BudgetCategory.allCases, id: \.self) { cat in
                                Label(cat.rawValue, systemImage: cat.icon).tag(cat)
                            }
                        }
                    }
                }

                Section {
                    VStack(alignment: .leading, spacing: 8) {
                        Label("Bond: 10,000 ETR", systemImage: userBalance >= requiredBond ? "checkmark.circle.fill" : "exclamationmark.triangle.fill")
                            .foregroundColor(userBalance >= requiredBond ? .green : .orange)
                        Text("Bond is locked until proposal ends. Refunded if approved or returns 90% if rejected.")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                if let error = errorMessage {
                    Section {
                        Label(error, systemImage: "xmark.circle.fill")
                            .foregroundColor(.red)
                    }
                }
            }
            .navigationTitle("Submit Proposal")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    if isSubmitting {
                        ProgressView()
                    } else {
                        Button("Submit") {
                            Task { await submitProposal() }
                        }
                        .disabled(!canSubmit)
                    }
                }
            }
            .task {
                await loadRequirements()
            }
        }
    }

    private var isFormValid: Bool {
        title.count >= 10 && description.count >= 50
    }

    private var canSubmit: Bool {
        isFormValid &&
        walletManager.selectedAccount != nil &&
        userBalance >= requiredBond &&
        consensusPhase.allowsProposalSubmission &&
        !isSubmitting
    }

    private var phaseStatusText: String {
        switch consensusPhase {
        case .voting:
            return "Voting in progress (Dec 1) - proposals closed"
        case .execution:
            return "Execution phase (Dec 2-30) - proposals closed"
        case .distribution, .minting:
            return "Distribution phase (Dec 31) - proposals closed"
        case .inactive:
            return "System inactive"
        case .registration:
            return "Registration open (Jan 1 - Nov 30)"
        }
    }

    private func loadRequirements() async {
        isLoading = true

        // Get user balance
        if let account = walletManager.selectedAccount {
            userBalance = account.balances.first(where: { $0.token == .etrid })?.amount ?? 0
        }

        // Determine consensus phase based on calendar date
        // This is the authoritative source - no RPC needed for phase detection
        consensusPhase = ConsensusDayPhase.currentPhase()
        isConsensusDayActive = consensusPhase != .inactive

        isLoading = false
    }

    private func submitProposal() async {
        guard let governance = walletManager.getGovernanceService(),
              let address = walletManager.selectedAccount?.address else {
            errorMessage = "No wallet connected"
            return
        }

        guard userBalance >= requiredBond else {
            errorMessage = "Insufficient balance for 10,000 ETR bond"
            return
        }

        guard consensusPhase == .registration else {
            errorMessage = "Proposals can only be submitted during Registration phase"
            return
        }

        isSubmitting = true
        errorMessage = nil

        do {
            _ = try await governance.submitProposal(
                title: title,
                description: description,
                category: category,
                budgetAmount: category == .budgetAllocation ? Decimal(string: budgetAmount) : nil,
                budgetCategory: category == .budgetAllocation ? budgetCategory : nil,
                fromAddress: address
            )
            dismiss()
        } catch {
            errorMessage = error.localizedDescription
        }

        isSubmitting = false
    }
}

// MARK: - Requirement Row

struct RequirementRow: View {
    let title: String
    let isMet: Bool
    let metText: String
    let unmetText: String

    var body: some View {
        HStack {
            Image(systemName: isMet ? "checkmark.circle.fill" : "xmark.circle.fill")
                .foregroundColor(isMet ? .green : .red)
            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.subheadline)
                Text(isMet ? metText : unmetText)
                    .font(.caption)
                    .foregroundColor(isMet ? .green : .red)
            }
            Spacer()
        }
    }
}

// MARK: - Phase Schedule Row

struct PhaseScheduleRow: View {
    let phase: String
    let dates: String
    let isCurrent: Bool

    var body: some View {
        HStack {
            Image(systemName: isCurrent ? "circle.fill" : "circle")
                .font(.caption2)
                .foregroundColor(isCurrent ? EtridColors.primary : .secondary)
            Text(phase)
                .font(.caption)
                .foregroundColor(isCurrent ? .primary : .secondary)
            Spacer()
            Text(dates)
                .font(.caption)
                .foregroundColor(isCurrent ? EtridColors.primary : .secondary)
        }
    }
}
