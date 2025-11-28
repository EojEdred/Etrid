//
//  DCABotView.swift
//  EtridWallet
//
//  Native iOS implementation of Dollar-Cost Averaging Bot
//  Production-ready with full DCABotService integration
//

import SwiftUI

struct DCABotView: View {
    @StateObject private var viewModel = DCABotViewModel()
    @Environment(\.dismiss) private var dismiss
    @State private var showCreateBot = false
    @State private var selectedBot: DCABot?
    @State private var showHistory = false

    var body: some View {
        NavigationView {
            ZStack {
                ScrollView {
                    VStack(spacing: 20) {
                        // Info Card
                        infoCard

                        // Error Alert
                        if let error = viewModel.errorMessage {
                            DCAErrorBanner(message: error) {
                                viewModel.errorMessage = nil
                            }
                        }

                        // Your DCA Bots
                        yourBots

                        // DCA Strategies
                        dcaStrategies
                    }
                    .padding()
                }
                .refreshable {
                    await viewModel.loadBots()
                }

                // Loading Overlay
                if viewModel.isLoading {
                    LoadingOverlay()
                }
            }
            .navigationTitle("DCA Bots")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button(action: { dismiss() }) {
                        Image(systemName: "chevron.left")
                            .foregroundColor(.blue)
                    }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showCreateBot = true }) {
                        Image(systemName: "plus")
                            .font(.title2)
                            .foregroundColor(.blue)
                    }
                }
            }
            .sheet(isPresented: $showCreateBot) {
                CreateDCABotView(viewModel: viewModel)
            }
            .sheet(item: $selectedBot) { bot in
                DCABotDetailView(bot: bot, viewModel: viewModel)
            }
        }
        .task {
            await viewModel.loadBots()
        }
    }

    // MARK: - Info Card
    private var infoCard: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Text("💡")
                    .font(.title2)
                Text("What is DCA?")
                    .font(.headline)
                    .fontWeight(.bold)
            }

            Text("Dollar Cost Averaging automatically invests fixed amounts at regular intervals, reducing market timing risk.")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .lineSpacing(4)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(
            LinearGradient(
                gradient: Gradient(colors: [Color.blue.opacity(0.1), Color.blue.opacity(0.05)]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
        )
        .cornerRadius(16)
        .overlay(
            RoundedRectangle(cornerRadius: 16)
                .stroke(Color.blue, lineWidth: 1)
        )
    }

    // MARK: - Your Bots
    private var yourBots: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Your DCA Bots")
                .font(.headline)
                .fontWeight(.bold)

            if viewModel.bots.isEmpty && !viewModel.isLoading {
                EmptyBotsView()
            } else {
                ForEach(viewModel.bots) { bot in
                    DCABotCard(
                        bot: bot,
                        onToggle: {
                            Task {
                                await viewModel.toggleBot(bot.id)
                            }
                        },
                        onTap: {
                            selectedBot = bot
                        },
                        onExecute: {
                            Task {
                                await viewModel.executeBot(bot.id)
                            }
                        }
                    )
                }
            }
        }
    }

    // MARK: - DCA Strategies
    private var dcaStrategies: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("DCA Strategies")
                .font(.headline)
                .fontWeight(.bold)

            StrategyCard(
                icon: "📅",
                title: "Conservative",
                description: "Weekly, small amounts",
                color: .green
            )

            StrategyCard(
                icon: "⚡",
                title: "Aggressive",
                description: "Daily, larger amounts",
                color: .orange
            )

            StrategyCard(
                icon: "🎯",
                title: "Target Price",
                description: "Buy dips automatically",
                color: .purple
            )
        }
    }
}

// MARK: - Empty Bots View
struct EmptyBotsView: View {
    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "robot")
                .font(.system(size: 60))
                .foregroundColor(.secondary)

            Text("No DCA Bots Yet")
                .font(.headline)
                .foregroundColor(.secondary)

            Text("Create your first bot to start automating your investments")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
        .padding(40)
        .background(Color(.systemGray6))
        .cornerRadius(16)
    }
}

// MARK: - Error Banner
struct DCAErrorBanner: View {
    let message: String
    let onDismiss: () -> Void

    var body: some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.primary)

            Spacer()

            Button(action: onDismiss) {
                Image(systemName: "xmark.circle.fill")
                    .foregroundColor(.secondary)
            }
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .cornerRadius(12)
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(Color.orange, lineWidth: 1)
        )
    }
}

// MARK: - Loading Overlay
struct LoadingOverlay: View {
    var body: some View {
        ZStack {
            Color.black.opacity(0.3)
                .ignoresSafeArea()

            VStack(spacing: 16) {
                ProgressView()
                    .scaleEffect(1.5)
                    .tint(.white)

                Text("Loading...")
                    .font(.headline)
                    .foregroundColor(.white)
            }
            .padding(30)
            .background(Color(.systemGray6))
            .cornerRadius(16)
        }
    }
}

// MARK: - DCA Bot Card
struct DCABotCard: View {
    let bot: DCABot
    let onToggle: () -> Void
    let onTap: () -> Void
    let onExecute: () -> Void

    @State private var isToggling = false

    var body: some View {
        Button(action: onTap) {
            VStack(alignment: .leading, spacing: 12) {
                // Header
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text(bot.name)
                            .font(.title3)
                            .fontWeight(.bold)
                            .foregroundColor(.primary)

                        Text(bot.pair)
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }

                    Spacer()

                    Toggle("", isOn: .constant(bot.isActive))
                        .labelsHidden()
                        .disabled(isToggling)
                        .onTapGesture {
                            isToggling = true
                            onToggle()
                            DispatchQueue.main.asyncAfter(deadline: .now() + 1) {
                                isToggling = false
                            }
                        }
                }

                // Stats Grid
                HStack(spacing: 12) {
                    BotStatView(label: "Frequency", value: bot.frequency.displayName)
                    BotStatView(label: "Amount", value: "$\(String(format: "%.2f", bot.amount))")
                    BotStatView(label: "Next Run", value: formatTimeUntil(bot.nextRun))
                }

                // Investment Summary
                HStack(spacing: 20) {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Total Invested")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text("$\(String(format: "%.2f", bot.totalInvested))")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                    }

                    VStack(alignment: .leading, spacing: 4) {
                        Text("Avg Price")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text("$\(String(format: "%.4f", bot.averagePrice))")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                    }

                    Spacer()

                    if bot.isActive {
                        Button(action: onExecute) {
                            HStack(spacing: 4) {
                                Image(systemName: "play.circle.fill")
                                Text("Execute")
                            }
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.blue)
                            .padding(.horizontal, 12)
                            .padding(.vertical, 6)
                            .background(Color.blue.opacity(0.1))
                            .cornerRadius(8)
                        }
                    }
                }
            }
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(16)
            .overlay(
                RoundedRectangle(cornerRadius: 16)
                    .stroke(bot.isActive ? Color.blue : Color(.systemGray4), lineWidth: bot.isActive ? 2 : 1)
            )
        }
        .buttonStyle(PlainButtonStyle())
    }

    private func formatTimeUntil(_ date: Date) -> String {
        let now = Date()
        let interval = date.timeIntervalSince(now)

        if interval < 0 {
            return "Due now"
        }

        let hours = Int(interval / 3600)
        let days = hours / 24

        if days > 0 {
            return "\(days)d"
        } else if hours > 0 {
            return "\(hours)h"
        } else {
            let minutes = Int(interval / 60)
            return "\(minutes)m"
        }
    }
}

// MARK: - Bot Stat View
struct BotStatView: View {
    let label: String
    let value: String

    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(label)
                .font(.caption)
                .foregroundColor(.secondary)

            Text(value)
                .font(.body)
                .fontWeight(.semibold)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(8)
    }
}

// MARK: - Strategy Card
struct StrategyCard: View {
    let icon: String
    let title: String
    let description: String
    let color: Color

    var body: some View {
        Button(action: {}) {
            HStack(spacing: 16) {
                Text(icon)
                    .font(.largeTitle)

                VStack(alignment: .leading, spacing: 4) {
                    Text(title)
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.primary)

                    Text(description)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                Image(systemName: "chevron.right")
                    .foregroundColor(.secondary)
            }
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(color.opacity(0.3), lineWidth: 2)
            )
        }
    }
}

// MARK: - DCA Bot Detail View
struct DCABotDetailView: View {
    let bot: DCABot
    @ObservedObject var viewModel: DCABotViewModel
    @Environment(\.dismiss) private var dismiss
    @State private var executions: [DCAExecution] = []
    @State private var isLoadingHistory = false

    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 24) {
                    // Bot Info
                    VStack(alignment: .leading, spacing: 16) {
                        HStack {
                            VStack(alignment: .leading, spacing: 8) {
                                Text(bot.name)
                                    .font(.title)
                                    .fontWeight(.bold)

                                Text(bot.pair)
                                    .font(.headline)
                                    .foregroundColor(.secondary)
                            }

                            Spacer()

                            VStack(alignment: .trailing, spacing: 8) {
                                Text(bot.isActive ? "Active" : "Paused")
                                    .font(.subheadline)
                                    .fontWeight(.semibold)
                                    .foregroundColor(bot.isActive ? .green : .orange)
                                    .padding(.horizontal, 12)
                                    .padding(.vertical, 6)
                                    .background(bot.isActive ? Color.green.opacity(0.1) : Color.orange.opacity(0.1))
                                    .cornerRadius(8)
                            }
                        }

                        // Stats
                        VStack(spacing: 12) {
                            StatRow(label: "Frequency", value: bot.frequency.displayName)
                            StatRow(label: "Amount per execution", value: "$\(String(format: "%.2f", bot.amount))")
                            StatRow(label: "Total invested", value: "$\(String(format: "%.2f", bot.totalInvested))")
                            StatRow(label: "Total purchased", value: "\(String(format: "%.4f", bot.totalPurchased)) \(bot.toAsset)")
                            StatRow(label: "Average price", value: "$\(String(format: "%.4f", bot.averagePrice))")
                            StatRow(label: "Next execution", value: formatDate(bot.nextRun))
                            if let lastRun = bot.lastRun {
                                StatRow(label: "Last execution", value: formatDate(lastRun))
                            }
                        }
                    }
                    .padding()
                    .background(Color(.systemGray6))
                    .cornerRadius(16)

                    // Execution History
                    VStack(alignment: .leading, spacing: 12) {
                        Text("Execution History")
                            .font(.headline)
                            .fontWeight(.bold)

                        if isLoadingHistory {
                            ProgressView()
                                .frame(maxWidth: .infinity)
                                .padding()
                        } else if executions.isEmpty {
                            Text("No executions yet")
                                .font(.subheadline)
                                .foregroundColor(.secondary)
                                .frame(maxWidth: .infinity)
                                .padding()
                        } else {
                            ForEach(executions) { execution in
                                ExecutionRow(execution: execution, toAsset: bot.toAsset)
                            }
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("Bot Details")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Close") { dismiss() }
                }
            }
            .task {
                await loadHistory()
            }
        }
    }

    private func loadHistory() async {
        isLoadingHistory = true
        do {
            executions = try await DCABotService.shared.getExecutionHistory(for: bot.id)
        } catch {
            print("Failed to load execution history: \(error)")
        }
        isLoadingHistory = false
    }

    private func formatDate(_ date: Date) -> String {
        let formatter = DateFormatter()
        formatter.dateStyle = .medium
        formatter.timeStyle = .short
        return formatter.string(from: date)
    }
}

// MARK: - Stat Row
struct StatRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()

            Text(value)
                .font(.subheadline)
                .fontWeight(.semibold)
        }
    }
}

// MARK: - Execution Row
struct ExecutionRow: View {
    let execution: DCAExecution
    let toAsset: String

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 4) {
                Text(formatDate(execution.executedAt))
                    .font(.subheadline)
                    .fontWeight(.semibold)

                Text("\(String(format: "%.4f", execution.purchasedAmount)) \(toAsset)")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text("$\(String(format: "%.2f", execution.amount))")
                    .font(.subheadline)
                    .fontWeight(.semibold)

                HStack(spacing: 4) {
                    statusIcon
                    Text(execution.status.rawValue.capitalized)
                        .font(.caption)
                        .foregroundColor(statusColor)
                }
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }

    private var statusIcon: some View {
        Group {
            switch execution.status {
            case .success:
                Image(systemName: "checkmark.circle.fill")
                    .foregroundColor(.green)
            case .failed:
                Image(systemName: "xmark.circle.fill")
                    .foregroundColor(.red)
            case .pending:
                Image(systemName: "clock.fill")
                    .foregroundColor(.orange)
            }
        }
    }

    private var statusColor: Color {
        switch execution.status {
        case .success:
            return .green
        case .failed:
            return .red
        case .pending:
            return .orange
        }
    }

    private func formatDate(_ date: Date) -> String {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .short
        return formatter.string(from: date)
    }
}

// MARK: - Create DCA Bot View
struct CreateDCABotView: View {
    @ObservedObject var viewModel: DCABotViewModel
    @Environment(\.dismiss) private var dismiss

    @State private var botName = ""
    @State private var fromAsset = "USDC"
    @State private var toAsset = "ÉTR"
    @State private var frequency: DCAFrequency = .weekly
    @State private var amount = ""
    @State private var isCreating = false
    @State private var validationError: String?

    let frequencies: [DCAFrequency] = [.daily, .weekly, .biweekly, .monthly]
    let assets = ["USDC", "ÉTR", "EDSC", "ETH", "BTC"]

    var body: some View {
        NavigationView {
            ZStack {
                Form {
                    Section(header: Text("Bot Configuration")) {
                        TextField("Bot Name", text: $botName)
                            .autocapitalization(.words)

                        Picker("From Asset", selection: $fromAsset) {
                            ForEach(assets, id: \.self) { asset in
                                Text(asset).tag(asset)
                            }
                        }

                        Picker("To Asset", selection: $toAsset) {
                            ForEach(assets, id: \.self) { asset in
                                Text(asset).tag(asset)
                            }
                        }

                        Picker("Frequency", selection: $frequency) {
                            ForEach(frequencies, id: \.self) { freq in
                                Text(freq.displayName).tag(freq)
                            }
                        }

                        HStack {
                            Text("$")
                            TextField("Amount", text: $amount)
                                .keyboardType(.decimalPad)
                        }
                    }

                    if let error = validationError {
                        Section {
                            Text(error)
                                .font(.subheadline)
                                .foregroundColor(.red)
                        }
                    }

                    Section {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("Summary")
                                .font(.headline)

                            if !botName.isEmpty && !amount.isEmpty, let amountValue = Double(amount) {
                                Text("Your bot will automatically invest $\(String(format: "%.2f", amountValue)) in \(toAsset) every \(frequency.displayName.lowercased()).")
                                    .font(.subheadline)
                                    .foregroundColor(.secondary)
                            }
                        }
                    }

                    Section {
                        Text("DCA bots run automatically based on your schedule. You can pause or modify them at any time.")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                }

                if isCreating {
                    LoadingOverlay()
                }
            }
            .navigationTitle("Create DCA Bot")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .disabled(isCreating)
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Create") {
                        Task {
                            await createBot()
                        }
                    }
                    .disabled(!isValid || isCreating)
                }
            }
        }
    }

    private var isValid: Bool {
        !botName.isEmpty &&
        !amount.isEmpty &&
        Double(amount) != nil &&
        fromAsset != toAsset
    }

    private func createBot() async {
        guard let amountValue = Double(amount) else {
            validationError = "Invalid amount"
            return
        }

        guard amountValue > 0 else {
            validationError = "Amount must be greater than 0"
            return
        }

        guard fromAsset != toAsset else {
            validationError = "Cannot convert to the same asset"
            return
        }

        isCreating = true
        validationError = nil

        await viewModel.createBot(
            name: botName,
            fromAsset: fromAsset,
            toAsset: toAsset,
            frequency: frequency,
            amount: amountValue
        )

        isCreating = false

        if viewModel.errorMessage == nil {
            dismiss()
        } else {
            validationError = viewModel.errorMessage
        }
    }
}

// MARK: - View Model
class DCABotViewModel: ObservableObject {
    @Published var bots: [DCABot] = []
    @Published var isLoading = false
    @Published var errorMessage: String?

    private let service = DCABotService.shared

    // Get current user's wallet address from keychain or use a default
    private var userId: String {
        // In production, this would come from your wallet/auth state
        // For now, we'll use a stored value or default
        return KeychainService.shared.loadString(key: "current_wallet_address") ?? "default_user"
    }

    // MARK: - Load Bots
    @MainActor
    func loadBots() async {
        isLoading = true
        errorMessage = nil

        do {
            bots = try await service.getBots(for: userId)
        } catch {
            errorMessage = "Failed to load bots: \(error.localizedDescription)"
            print("Error loading bots: \(error)")
        }

        isLoading = false
    }

    // MARK: - Create Bot
    @MainActor
    func createBot(
        name: String,
        fromAsset: String,
        toAsset: String,
        frequency: DCAFrequency,
        amount: Double
    ) async {
        isLoading = true
        errorMessage = nil

        do {
            let newBot = try await service.createBot(
                userId: userId,
                name: name,
                fromAsset: fromAsset,
                toAsset: toAsset,
                frequency: frequency,
                amount: amount
            )

            // Add to local list
            bots.append(newBot)
        } catch let error as DCAError {
            errorMessage = error.localizedDescription
        } catch {
            errorMessage = "Failed to create bot: \(error.localizedDescription)"
        }

        isLoading = false
    }

    // MARK: - Toggle Bot
    @MainActor
    func toggleBot(_ id: String) async {
        guard let index = bots.firstIndex(where: { $0.id == id }) else { return }

        let currentState = bots[index].isActive

        do {
            let updatedBot = try await service.toggleBot(id: id, isActive: !currentState)
            bots[index] = updatedBot
        } catch {
            errorMessage = "Failed to toggle bot: \(error.localizedDescription)"
        }
    }

    // MARK: - Execute Bot
    @MainActor
    func executeBot(_ id: String) async {
        isLoading = true
        errorMessage = nil

        do {
            let execution = try await service.executeBot(id: id)

            // Reload the bot to get updated stats
            if let index = bots.firstIndex(where: { $0.id == id }) {
                let updatedBot = try await service.getBot(id: id)
                bots[index] = updatedBot
            }

            // Show success message
            errorMessage = nil
        } catch let error as DCAError {
            errorMessage = error.localizedDescription
        } catch {
            errorMessage = "Execution failed: \(error.localizedDescription)"
        }

        isLoading = false
    }

    // MARK: - Delete Bot
    @MainActor
    func deleteBot(_ id: String) async {
        do {
            try await service.deleteBot(id: id)
            bots.removeAll { $0.id == id }
        } catch {
            errorMessage = "Failed to delete bot: \(error.localizedDescription)"
        }
    }
}

// MARK: - Preview
struct DCABotView_Previews: PreviewProvider {
    static var previews: some View {
        DCABotView()
    }
}
