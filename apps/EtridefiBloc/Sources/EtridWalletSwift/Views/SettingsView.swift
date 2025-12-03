import SwiftUI
import LocalAuthentication
#if os(iOS)
import UIKit
#elseif os(macOS)
import AppKit
#endif

// MARK: - Settings View

struct SettingsView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var showBackupWarning = false
    @State private var showDeleteConfirmation = false
    @State private var showMnemonic = false

    var body: some View {
        NavigationStack {
            List {
                // Wallet Section
                Section("Wallet") {
                    if let wallet = walletManager.currentWallet {
                        HStack {
                            EtridLogo(size: 44)

                            VStack(alignment: .leading, spacing: 4) {
                                Text(wallet.name)
                                    .font(.headline)
                                Text("\(wallet.accounts.count) account(s)")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }
                        .padding(.vertical, 4)

                        NavigationLink(destination: AccountsListView()) {
                            Label("Manage Accounts", systemImage: "person.2.fill")
                        }

                        Button(action: { showMnemonic = true }) {
                            Label("View Recovery Phrase", systemImage: "key.fill")
                                .foregroundColor(.primary)
                        }

                        if !wallet.isBackedUp {
                            Button(action: { showBackupWarning = true }) {
                                HStack {
                                    Label("Backup Wallet", systemImage: "exclamationmark.shield.fill")
                                    Spacer()
                                    Image(systemName: "exclamationmark.circle.fill")
                                        .foregroundColor(.orange)
                                }
                            }
                        }
                    }
                }

                // Security Section
                Section("Security") {
                    NavigationLink(destination: SecuritySettingsView()) {
                        Label("Security Settings", systemImage: "lock.shield.fill")
                    }

                    NavigationLink(destination: BiometricSettingsView()) {
                        Label("Biometrics", systemImage: "faceid")
                    }

                    NavigationLink(destination: PasscodeSettingsView()) {
                        Label("Change Passcode", systemImage: "lock.fill")
                    }
                }

                // Network Section
                Section("Network") {
                    NavigationLink(destination: NetworkSettingsView()) {
                        HStack {
                            Label("Network", systemImage: "network")
                            Spacer()
                            Text("Mainnet")
                                .foregroundColor(.secondary)
                        }
                    }

                    NavigationLink(destination: RPCSettingsView()) {
                        Label("RPC Settings", systemImage: "server.rack")
                    }
                }

                // Preferences Section
                Section("Preferences") {
                    NavigationLink(destination: CurrencySettingsView()) {
                        HStack {
                            Label("Currency", systemImage: "dollarsign.circle.fill")
                            Spacer()
                            Text("USD")
                                .foregroundColor(.secondary)
                        }
                    }

                    NavigationLink(destination: NotificationSettingsView()) {
                        Label("Notifications", systemImage: "bell.fill")
                    }

                    NavigationLink(destination: AppearanceSettingsView()) {
                        Label("Appearance", systemImage: "paintbrush.fill")
                    }
                }

                // About Section
                Section("About") {
                    NavigationLink(destination: AboutView()) {
                        Label("About ETRID", systemImage: "info.circle.fill")
                    }

                    Link(destination: URL(string: "https://etrid.network")!) {
                        HStack {
                            Label("Website", systemImage: "globe")
                            Spacer()
                            Image(systemName: "arrow.up.forward.square")
                                .foregroundColor(.secondary)
                        }
                    }

                    Link(destination: URL(string: "https://docs.etrid.network")!) {
                        HStack {
                            Label("Documentation", systemImage: "book.fill")
                            Spacer()
                            Image(systemName: "arrow.up.forward.square")
                                .foregroundColor(.secondary)
                        }
                    }

                    NavigationLink(destination: TermsView()) {
                        Label("Terms of Service", systemImage: "doc.text.fill")
                    }

                    NavigationLink(destination: PrivacyView()) {
                        Label("Privacy Policy", systemImage: "hand.raised.fill")
                    }
                }

                // Danger Zone
                Section {
                    Button(role: .destructive, action: { showDeleteConfirmation = true }) {
                        Label("Delete Wallet", systemImage: "trash.fill")
                    }
                }
            }
            .navigationTitle("Settings")
            .alert("Backup Required", isPresented: $showBackupWarning) {
                Button("Backup Now") { showMnemonic = true }
                Button("Later", role: .cancel) { }
            } message: {
                Text("Your wallet is not backed up. If you lose your device, you will lose access to your funds.")
            }
            .alert("Delete Wallet", isPresented: $showDeleteConfirmation) {
                Button("Cancel", role: .cancel) { }
                Button("Delete", role: .destructive) {
                    Task { try? await walletManager.deleteWallet() }
                }
            } message: {
                Text("Are you sure you want to delete your wallet? This action cannot be undone. Make sure you have backed up your recovery phrase.")
            }
            .sheet(isPresented: $showMnemonic) {
                RecoveryPhraseView()
            }
        }
    }
}

// MARK: - Accounts List View

struct AccountsListView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var isAddingAccount = false

    var body: some View {
        List {
            ForEach(walletManager.accounts) { account in
                AccountRow(
                    account: account,
                    isSelected: walletManager.selectedAccount?.id == account.id,
                    onSelect: { walletManager.selectAccount(account) }
                )
            }

            Button(action: addAccount) {
                Label("Add Account", systemImage: "plus.circle.fill")
                    .foregroundColor(EtridColors.primary)
            }
        }
        .navigationTitle("Accounts")
    }

    private func addAccount() {
        isAddingAccount = true
        Task {
            do {
                _ = try await walletManager.addAccount()
            } catch {
                print("Failed to add account: \(error)")
            }
            isAddingAccount = false
        }
    }
}

struct AccountRow: View {
    let account: WalletAccountInfo
    let isSelected: Bool
    let onSelect: () -> Void

    var body: some View {
        Button(action: onSelect) {
            HStack {
                Circle()
                    .fill(EtridColors.primary)
                    .frame(width: 40, height: 40)
                    .overlay(
                        Text(String(account.address.suffix(2)))
                            .font(.caption)
                            .fontWeight(.bold)
                            .foregroundColor(.white)
                    )

                VStack(alignment: .leading, spacing: 4) {
                    Text(truncatedAddress(account.address))
                        .font(.headline)
                    Text(account.chain.rawValue)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                if isSelected {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundColor(EtridColors.primary)
                }
            }
        }
        .buttonStyle(.plain)
    }

    private func truncatedAddress(_ address: String) -> String {
        guard address.count > 12 else { return address }
        return "\(address.prefix(6))...\(address.suffix(4))"
    }
}

// MARK: - Recovery Phrase View

struct RecoveryPhraseView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @Environment(\.dismiss) var dismiss
    @State private var mnemonic: String?
    @State private var isAuthenticated = false
    @State private var showWords = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                if !isAuthenticated {
                    AuthenticationPrompt(onAuthenticated: {
                        isAuthenticated = true
                        Task { await loadMnemonic() }
                    })
                } else if let mnemonic = mnemonic {
                    MnemonicDisplay(mnemonic: mnemonic, showWords: $showWords)
                } else {
                    ProgressView("Loading...")
                }
            }
            .padding()
            .navigationTitle("Recovery Phrase")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") { dismiss() }
                }
            }
        }
    }

    private func loadMnemonic() async {
        guard let wallet = walletManager.currentWallet else { return }
        do {
            mnemonic = try await walletManager.getMnemonic(for: wallet.id)
        } catch {
            print("Failed to load mnemonic: \(error)")
        }
    }
}

struct AuthenticationPrompt: View {
    let onAuthenticated: () -> Void
    @State private var isAuthenticating = false
    @State private var authError: String?

    var body: some View {
        VStack(spacing: 24) {
            Image(systemName: "lock.shield.fill")
                .font(.system(size: 64))
                .foregroundColor(EtridColors.primary)

            Text("Authentication Required")
                .font(.title2)
                .fontWeight(.bold)

            Text("Authenticate to view your recovery phrase")
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)

            if let error = authError {
                Text(error)
                    .font(.caption)
                    .foregroundColor(.red)
                    .multilineTextAlignment(.center)
            }

            Button(action: authenticate) {
                HStack {
                    if isAuthenticating {
                        ProgressView()
                            .tint(.white)
                    } else {
                        Image(systemName: biometricIcon)
                        Text("Authenticate")
                    }
                }
                .font(.headline)
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(EtridColors.primary)
                .cornerRadius(12)
            }
            .disabled(isAuthenticating)
        }
    }

    private var biometricIcon: String {
        #if os(iOS)
        let context = LAContext()
        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil) {
            switch context.biometryType {
            case .faceID: return "faceid"
            case .touchID: return "touchid"
            case .opticID: return "opticid"
            default: return "lock.fill"
            }
        }
        #endif
        return "lock.fill"
    }

    private func authenticate() {
        isAuthenticating = true
        authError = nil

        #if os(iOS)
        let context = LAContext()
        context.localizedCancelTitle = "Cancel"

        var error: NSError?
        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) {
            context.evaluatePolicy(
                .deviceOwnerAuthenticationWithBiometrics,
                localizedReason: "Authenticate to access your recovery phrase"
            ) { success, authenticationError in
                DispatchQueue.main.async {
                    isAuthenticating = false
                    if success {
                        onAuthenticated()
                    } else if let error = authenticationError as? LAError {
                        switch error.code {
                        case .userCancel:
                            authError = nil // User cancelled, no error message
                        case .biometryLockout:
                            authError = "Too many failed attempts. Use device passcode."
                        case .biometryNotEnrolled:
                            authError = "Biometric authentication not set up on this device."
                        default:
                            authError = "Authentication failed. Please try again."
                        }
                    }
                }
            }
        } else if context.canEvaluatePolicy(.deviceOwnerAuthentication, error: &error) {
            // Fall back to device passcode
            context.evaluatePolicy(
                .deviceOwnerAuthentication,
                localizedReason: "Authenticate to access your recovery phrase"
            ) { success, authenticationError in
                DispatchQueue.main.async {
                    isAuthenticating = false
                    if success {
                        onAuthenticated()
                    } else {
                        authError = "Authentication failed. Please try again."
                    }
                }
            }
        } else {
            isAuthenticating = false
            authError = "No authentication method available on this device."
        }
        #else
        // macOS or other platforms - just succeed for now
        isAuthenticating = false
        onAuthenticated()
        #endif
    }
}

struct MnemonicDisplay: View {
    let mnemonic: String
    @Binding var showWords: Bool

    var words: [String] {
        mnemonic.split(separator: " ").map(String.init)
    }

    private var gridColumns: [GridItem] {
        [GridItem(.flexible()), GridItem(.flexible()), GridItem(.flexible())]
    }

    var body: some View {
        VStack(spacing: 24) {
            warningBanner
            visibilityToggle
            wordsGrid
            copyButton
        }
    }

    private var warningBanner: some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.orange)
            Text("Never share your recovery phrase with anyone")
                .font(.caption)
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .cornerRadius(12)
    }

    private var visibilityToggle: some View {
        Button(action: { showWords.toggle() }) {
            HStack {
                Image(systemName: showWords ? "eye.slash.fill" : "eye.fill")
                Text(showWords ? "Hide Words" : "Show Words")
            }
            .font(.subheadline)
            .foregroundColor(EtridColors.primary)
        }
    }

    @ViewBuilder
    private var wordsGrid: some View {
        LazyVGrid(columns: gridColumns, spacing: 8) {
            ForEach(Array(words.enumerated()), id: \.offset) { index, word in
                wordCell(index: index, word: showWords ? word : "****")
            }
        }
    }

    private func wordCell(index: Int, word: String) -> some View {
        HStack {
            Text("\(index + 1).")
                .font(.caption)
                .foregroundColor(.secondary)
                .frame(width: 20)
            Text(word)
                .font(.system(.body, design: .monospaced))
        }
        .padding(8)
        .background(EtridColors.cardBackground)
        .cornerRadius(8)
    }

    @ViewBuilder
    private var copyButton: some View {
        if showWords {
            Button(action: copyMnemonic) {
                HStack {
                    Image(systemName: "doc.on.doc")
                    Text("Copy to Clipboard")
                }
                .font(.subheadline)
                .foregroundColor(EtridColors.primary)
            }
        }
    }

    private func copyMnemonic() {
        #if os(iOS)
        UIPasteboard.general.string = mnemonic
        #elseif os(macOS)
        NSPasteboard.general.clearContents()
        NSPasteboard.general.setString(mnemonic, forType: .string)
        #endif
    }
}

// MARK: - Placeholder Settings Views

struct SecuritySettingsView: View {
    var body: some View {
        List {
            Section("Auto-Lock") {
                Toggle("Auto-Lock", isOn: .constant(true))
                Picker("Lock After", selection: .constant(1)) {
                    Text("Immediately").tag(0)
                    Text("1 minute").tag(1)
                    Text("5 minutes").tag(5)
                }
            }

            Section("Transaction Security") {
                Toggle("Require Authentication for Transactions", isOn: .constant(true))
                Toggle("Show Transaction Preview", isOn: .constant(true))
            }
        }
        .navigationTitle("Security")
    }
}

struct BiometricSettingsView: View {
    @State private var biometricsEnabled = true

    var body: some View {
        List {
            Section {
                Toggle("Enable Face ID", isOn: $biometricsEnabled)
            } footer: {
                Text("Use Face ID to unlock your wallet and confirm transactions")
            }
        }
        .navigationTitle("Biometrics")
    }
}

struct PasscodeSettingsView: View {
    @State private var currentPasscode = ""
    @State private var newPasscode = ""
    @State private var confirmPasscode = ""

    var body: some View {
        Form {
            Section("Current Passcode") {
                SecureField("Enter current passcode", text: $currentPasscode)
            }

            Section("New Passcode") {
                SecureField("Enter new passcode", text: $newPasscode)
                SecureField("Confirm new passcode", text: $confirmPasscode)
            }

            Section {
                Button("Change Passcode") {
                    // Change passcode
                }
                .disabled(newPasscode.count < 6 || newPasscode != confirmPasscode)
            }
        }
        .navigationTitle("Change Passcode")
    }
}

struct NetworkSettingsView: View {
    @State private var selectedNetwork = 0

    var body: some View {
        List {
            Section {
                Picker("Network", selection: $selectedNetwork) {
                    Text("ETRID Mainnet").tag(0)
                    Text("ETRID Testnet").tag(1)
                }
                .pickerStyle(.inline)
            } footer: {
                Text("Switching networks will change the tokens and balances displayed")
            }
        }
        .navigationTitle("Network")
    }
}

struct RPCSettingsView: View {
    @State private var customRPC = ""

    var body: some View {
        List {
            Section("Default RPC") {
                Text("https://rpc.etrid.network")
                    .foregroundColor(.secondary)
            }

            Section("Custom RPC") {
                TextField("Enter custom RPC URL", text: $customRPC)
                    #if os(iOS)
                    .textInputAutocapitalization(.never)
                    #endif
                    .autocorrectionDisabled(true)
            }
        }
        .navigationTitle("RPC Settings")
    }
}

struct CurrencySettingsView: View {
    @State private var selectedCurrency = "USD"

    let currencies = ["USD", "EUR", "GBP", "JPY", "CNY", "KRW"]

    var body: some View {
        List {
            ForEach(currencies, id: \.self) { currency in
                Button(action: { selectedCurrency = currency }) {
                    HStack {
                        Text(currency)
                        Spacer()
                        if selectedCurrency == currency {
                            Image(systemName: "checkmark")
                                .foregroundColor(EtridColors.primary)
                        }
                    }
                }
                .foregroundColor(.primary)
            }
        }
        .navigationTitle("Currency")
    }
}

struct NotificationSettingsView: View {
    @State private var transactionNotifications = true
    @State private var priceAlerts = true
    @State private var governanceNotifications = true

    var body: some View {
        List {
            Section {
                Toggle("Transaction Notifications", isOn: $transactionNotifications)
                Toggle("Price Alerts", isOn: $priceAlerts)
                Toggle("Governance Notifications", isOn: $governanceNotifications)
            }
        }
        .navigationTitle("Notifications")
    }
}

struct AppearanceSettingsView: View {
    @State private var selectedTheme = 0

    var body: some View {
        List {
            Section {
                Picker("Theme", selection: $selectedTheme) {
                    Text("System").tag(0)
                    Text("Light").tag(1)
                    Text("Dark").tag(2)
                }
                .pickerStyle(.inline)
            }
        }
        .navigationTitle("Appearance")
    }
}

struct AboutView: View {
    var body: some View {
        List {
            Section {
                HStack {
                    Spacer()
                    VStack(spacing: 12) {
                        EtridLogo(size: 80)
                        Text("Ëtridefi Bloc")
                            .font(.title2)
                            .fontWeight(.bold)
                        Text("Version 1.0.0")
                            .foregroundColor(.secondary)
                    }
                    Spacer()
                }
                .padding(.vertical)
            }

            Section("About ETRID") {
                Text("ETRID is a next-generation blockchain platform featuring multi-chain bridges, democratic governance, and innovative consensus mechanisms.")
                    .font(.subheadline)
            }

            Section("Features") {
                Label("Multi-chain Bridge Support", systemImage: "arrow.left.arrow.right")
                Label("Democratic Governance", systemImage: "building.columns")
                Label("Consensus Day Participation", systemImage: "calendar")
                Label("Secure Key Management", systemImage: "key")
            }
        }
        .navigationTitle("About")
    }
}

struct TermsView: View {
    var body: some View {
        ScrollView {
            Text("Terms of Service content would go here...")
                .padding()
        }
        .navigationTitle("Terms of Service")
    }
}

struct PrivacyView: View {
    var body: some View {
        ScrollView {
            Text("Privacy Policy content would go here...")
                .padding()
        }
        .navigationTitle("Privacy Policy")
    }
}
