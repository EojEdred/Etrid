import SwiftUI
#if os(iOS)
import UIKit
#elseif os(macOS)
import AppKit
#endif

// MARK: - Onboarding Flow

struct OnboardingView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var currentStep: OnboardingStep = .welcome
    @State private var walletName = "My Wallet"
    @State private var generatedMnemonic: String?
    @State private var importedMnemonic = ""
    @State private var passcode = ""
    @State private var confirmPasscode = ""
    @State private var enableBiometrics = true
    @State private var verificationWords: [Int: String] = [:]
    @State private var isCreating = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationStack {
            VStack {
                // Progress Indicator
                ProgressBar(currentStep: currentStep.rawValue, totalSteps: OnboardingStep.allCases.count - 1)
                    .padding(.horizontal)

                // Content
                Group {
                    switch currentStep {
                    case .welcome:
                        WelcomeStepView(onContinue: { currentStep = .createOrImport })

                    case .createOrImport:
                        CreateOrImportStepView(
                            onCreate: {
                                Task { await createNewWallet() }
                            },
                            onImport: { currentStep = .createMnemonic }
                        )

                    case .createMnemonic:
                        if let mnemonic = generatedMnemonic {
                            MnemonicDisplayStepView(
                                mnemonic: mnemonic,
                                onContinue: { currentStep = .verifyMnemonic }
                            )
                        } else {
                            ImportMnemonicStepView(
                                mnemonic: $importedMnemonic,
                                onContinue: {
                                    generatedMnemonic = importedMnemonic
                                    currentStep = .setPasscode
                                }
                            )
                        }

                    case .verifyMnemonic:
                        VerifyMnemonicStepView(
                            mnemonic: generatedMnemonic ?? "",
                            verificationWords: $verificationWords,
                            onContinue: { currentStep = .setPasscode },
                            onBack: { currentStep = .createMnemonic }
                        )

                    case .setPasscode:
                        SetPasscodeStepView(
                            passcode: $passcode,
                            confirmPasscode: $confirmPasscode,
                            onContinue: { currentStep = .enableBiometrics }
                        )

                    case .enableBiometrics:
                        BiometricsStepView(
                            enableBiometrics: $enableBiometrics,
                            onContinue: { Task { await completeOnboarding() } }
                        )

                    case .complete:
                        CompletionStepView()
                    }
                }
                .transition(.asymmetric(insertion: .move(edge: .trailing), removal: .move(edge: .leading)))
                .animation(.easeInOut, value: currentStep)

                if let error = errorMessage {
                    Text(error)
                        .font(.caption)
                        .foregroundColor(.red)
                        .padding()
                }
            }
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
        }
    }

    private func createNewWallet() async {
        isCreating = true
        errorMessage = nil

        do {
            let result = try await walletManager.createNewWallet(name: walletName)
            generatedMnemonic = result.mnemonic
            currentStep = .createMnemonic
        } catch {
            errorMessage = error.localizedDescription
        }

        isCreating = false
    }

    private func completeOnboarding() async {
        isCreating = true
        errorMessage = nil

        do {
            // Save passcode
            try await walletManager.setPasscode(passcode)

            // If importing, create wallet from mnemonic
            if generatedMnemonic == nil && !importedMnemonic.isEmpty {
                _ = try await walletManager.importWallet(mnemonic: importedMnemonic, name: walletName)
            }

            currentStep = .complete

            // Brief delay then unlock
            try await Task.sleep(nanoseconds: 2_000_000_000)
            // Wallet manager will automatically transition to unlocked state
        } catch {
            errorMessage = error.localizedDescription
        }

        isCreating = false
    }
}

// MARK: - Progress Bar

struct ProgressBar: View {
    let currentStep: Int
    let totalSteps: Int

    var body: some View {
        GeometryReader { geometry in
            ZStack(alignment: .leading) {
                Rectangle()
                    .fill(Color.gray.opacity(0.3))
                    .frame(height: 4)
                    .cornerRadius(2)

                Rectangle()
                    .fill(EtridColors.primary)
                    .frame(width: geometry.size.width * progress, height: 4)
                    .cornerRadius(2)
            }
        }
        .frame(height: 4)
    }

    private var progress: Double {
        guard totalSteps > 0 else { return 0 }
        return Double(currentStep) / Double(totalSteps)
    }
}

// MARK: - Welcome Step

struct WelcomeStepView: View {
    let onContinue: () -> Void

    var body: some View {
        VStack(spacing: 32) {
            Spacer()

            // Logo
            EtridLogo(size: 120)

            // Title
            VStack(spacing: 8) {
                Text("Welcome to")
                    .font(.title2)
                    .foregroundColor(.secondary)

                Text("Ëtridefi Bloc")
                    .font(.largeTitle)
                    .fontWeight(.bold)
            }

            // Features
            VStack(alignment: .leading, spacing: 16) {
                FeatureRow(icon: "lock.shield.fill", title: "Secure", description: "Your keys, your crypto")
                FeatureRow(icon: "arrow.left.arrow.right.circle.fill", title: "Multi-chain", description: "Bridge tokens across blockchains")
                FeatureRow(icon: "building.columns.fill", title: "Governance", description: "Participate in network decisions")
            }
            .padding(.horizontal, 32)

            Spacer()

            // Continue Button
            Button(action: onContinue) {
                Text("Get Started")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(EtridColors.primary)
                    .cornerRadius(16)
            }
            .padding(.horizontal)
            .padding(.bottom, 32)
        }
    }
}

struct FeatureRow: View {
    let icon: String
    let title: String
    let description: String

    var body: some View {
        HStack(spacing: 16) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(EtridColors.primary)
                .frame(width: 40)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(.headline)
                Text(description)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
    }
}

// MARK: - Create or Import Step

struct CreateOrImportStepView: View {
    let onCreate: () -> Void
    let onImport: () -> Void

    var body: some View {
        VStack(spacing: 32) {
            Spacer()

            VStack(spacing: 8) {
                Text("Set Up Your Wallet")
                    .font(.title)
                    .fontWeight(.bold)

                Text("Create a new wallet or import an existing one")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
            }

            Spacer()

            VStack(spacing: 16) {
                // Create New Wallet
                Button(action: onCreate) {
                    HStack {
                        Image(systemName: "plus.circle.fill")
                            .font(.title2)

                        VStack(alignment: .leading, spacing: 4) {
                            Text("Create New Wallet")
                                .font(.headline)
                            Text("Generate a new recovery phrase")
                                .font(.caption)
                                .foregroundColor(.white.opacity(0.8))
                        }

                        Spacer()

                        Image(systemName: "chevron.right")
                    }
                    .foregroundColor(.white)
                    .padding()
                    .background(EtridColors.primary)
                    .cornerRadius(16)
                }

                // Import Wallet
                Button(action: onImport) {
                    HStack {
                        Image(systemName: "square.and.arrow.down.fill")
                            .font(.title2)

                        VStack(alignment: .leading, spacing: 4) {
                            Text("Import Existing Wallet")
                                .font(.headline)
                            Text("Use your recovery phrase")
                                .font(.caption)
                                .foregroundColor(.secondary)
                        }

                        Spacer()

                        Image(systemName: "chevron.right")
                    }
                    .foregroundColor(.primary)
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(16)
                }
            }
            .padding(.horizontal)
            .padding(.bottom, 32)
        }
    }
}

// MARK: - Mnemonic Display Step

struct MnemonicDisplayStepView: View {
    let mnemonic: String
    let onContinue: () -> Void
    @State private var showWords = false
    @State private var confirmed = false

    var words: [String] {
        mnemonic.split(separator: " ").map(String.init)
    }

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                VStack(spacing: 8) {
                    Text("Your Recovery Phrase")
                        .font(.title2)
                        .fontWeight(.bold)

                    Text("Write down these 24 words in order. This is the only way to recover your wallet.")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
                .padding(.horizontal)

                // Warning
                HStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text("Never share your recovery phrase")
                        .font(.caption)
                        .fontWeight(.medium)
                }
                .padding()
                .background(Color.orange.opacity(0.1))
                .cornerRadius(12)

                // Show/Hide Toggle
                Button(action: { showWords.toggle() }) {
                    HStack {
                        Image(systemName: showWords ? "eye.slash.fill" : "eye.fill")
                        Text(showWords ? "Hide Words" : "Reveal Words")
                    }
                    .foregroundColor(EtridColors.primary)
                }

                // Words Grid
                LazyVGrid(columns: [
                    GridItem(.flexible()),
                    GridItem(.flexible()),
                    GridItem(.flexible())
                ], spacing: 8) {
                    ForEach(Array(words.enumerated()), id: \.offset) { index, word in
                        HStack {
                            Text("\(index + 1).")
                                .font(.caption)
                                .foregroundColor(.secondary)
                                .frame(width: 24)
                            Text(showWords ? word : "****")
                                .font(.system(.body, design: .monospaced))
                        }
                        .padding(8)
                        .frame(maxWidth: .infinity)
                        .background(EtridColors.cardBackground)
                        .cornerRadius(8)
                    }
                }
                .padding(.horizontal)

                // Confirmation
                Toggle(isOn: $confirmed) {
                    Text("I have written down my recovery phrase")
                        .font(.subheadline)
                }
                .padding()

                // Continue Button
                Button(action: onContinue) {
                    Text("Continue")
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(confirmed ? EtridColors.primary : Color.gray)
                        .cornerRadius(16)
                }
                .disabled(!confirmed)
                .padding(.horizontal)
            }
            .padding(.vertical)
        }
    }
}

// MARK: - Import Mnemonic Step

struct ImportMnemonicStepView: View {
    @Binding var mnemonic: String
    let onContinue: () -> Void
    @State private var wordCount = 0

    var body: some View {
        VStack(spacing: 24) {
            VStack(spacing: 8) {
                Text("Import Your Wallet")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("Enter your 12 or 24 word recovery phrase")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            VStack(alignment: .leading, spacing: 8) {
                Text("Recovery Phrase")
                    .font(.subheadline)
                    .foregroundColor(.secondary)

                TextEditor(text: $mnemonic)
                    .font(.system(.body, design: .monospaced))
                    .frame(minHeight: 120)
                    .padding(8)
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)
                    .onChange(of: mnemonic) { _, newValue in
                        wordCount = newValue.split(separator: " ").count
                    }

                Text("\(wordCount) words")
                    .font(.caption)
                    .foregroundColor(isValidWordCount ? .green : .secondary)
            }
            .padding(.horizontal)

            Spacer()

            Button(action: onContinue) {
                Text("Import Wallet")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(isValidWordCount ? EtridColors.primary : Color.gray)
                    .cornerRadius(16)
            }
            .disabled(!isValidWordCount)
            .padding(.horizontal)
            .padding(.bottom)
        }
    }

    private var isValidWordCount: Bool {
        [12, 15, 18, 21, 24].contains(wordCount)
    }
}

// MARK: - Verify Mnemonic Step

struct VerifyMnemonicStepView: View {
    let mnemonic: String
    @Binding var verificationWords: [Int: String]
    let onContinue: () -> Void
    let onBack: () -> Void

    var words: [String] {
        mnemonic.split(separator: " ").map(String.init)
    }

    @State private var verificationIndices: [Int] = []

    var body: some View {
        VStack(spacing: 24) {
            VStack(spacing: 8) {
                Text("Verify Your Phrase")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("Enter the requested words to verify your backup")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            VStack(spacing: 16) {
                ForEach(verificationIndices, id: \.self) { index in
                    HStack {
                        Text("Word #\(index + 1)")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                            .frame(width: 80)

                        TextField("Enter word", text: Binding(
                            get: { verificationWords[index] ?? "" },
                            set: { verificationWords[index] = $0.lowercased() }
                        ))
                        .font(.system(.body, design: .monospaced))
                        #if os(iOS)
                        .textInputAutocapitalization(.never)
                        #endif
                        .autocorrectionDisabled(true)
                        .padding()
                        .background(EtridColors.cardBackground)
                        .cornerRadius(12)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(verificationStatus(for: index), lineWidth: 2)
                        )
                    }
                }
            }
            .padding(.horizontal)

            Spacer()

            HStack(spacing: 16) {
                Button(action: onBack) {
                    Text("Back")
                        .font(.headline)
                        .foregroundColor(EtridColors.primary)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(EtridColors.cardBackground)
                        .cornerRadius(16)
                }

                Button(action: onContinue) {
                    Text("Continue")
                        .font(.headline)
                        .foregroundColor(.white)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(isVerified ? EtridColors.primary : Color.gray)
                        .cornerRadius(16)
                }
                .disabled(!isVerified)
            }
            .padding(.horizontal)
            .padding(.bottom)
        }
        .onAppear {
            // Select 3 random indices to verify
            verificationIndices = Array(0..<words.count).shuffled().prefix(3).sorted()
        }
    }

    private func verificationStatus(for index: Int) -> Color {
        guard let entered = verificationWords[index], !entered.isEmpty else {
            return .clear
        }
        return entered == words[index] ? .green : .red
    }

    private var isVerified: Bool {
        for index in verificationIndices {
            guard let entered = verificationWords[index],
                  entered == words[index] else {
                return false
            }
        }
        return true
    }
}

// MARK: - Set Passcode Step

struct SetPasscodeStepView: View {
    @Binding var passcode: String
    @Binding var confirmPasscode: String
    let onContinue: () -> Void

    var body: some View {
        VStack(spacing: 24) {
            VStack(spacing: 8) {
                Text("Create a Passcode")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("This passcode will be used to unlock your wallet")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            VStack(spacing: 16) {
                SecureField("Enter passcode (6+ characters)", text: $passcode)
                    .font(.system(.body, design: .monospaced))
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)

                SecureField("Confirm passcode", text: $confirmPasscode)
                    .font(.system(.body, design: .monospaced))
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)
                    .overlay(
                        RoundedRectangle(cornerRadius: 12)
                            .stroke(confirmationColor, lineWidth: 2)
                    )

                if !confirmPasscode.isEmpty && passcode != confirmPasscode {
                    Text("Passcodes do not match")
                        .font(.caption)
                        .foregroundColor(.red)
                }
            }
            .padding(.horizontal)

            Spacer()

            Button(action: onContinue) {
                Text("Continue")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(isValid ? EtridColors.primary : Color.gray)
                    .cornerRadius(16)
            }
            .disabled(!isValid)
            .padding(.horizontal)
            .padding(.bottom)
        }
    }

    private var confirmationColor: Color {
        guard !confirmPasscode.isEmpty else { return .clear }
        return passcode == confirmPasscode ? .green : .red
    }

    private var isValid: Bool {
        passcode.count >= 6 && passcode == confirmPasscode
    }
}

// MARK: - Biometrics Step

struct BiometricsStepView: View {
    @Binding var enableBiometrics: Bool
    let onContinue: () -> Void

    var body: some View {
        VStack(spacing: 32) {
            Spacer()

            Image(systemName: "faceid")
                .font(.system(size: 80))
                .foregroundColor(EtridColors.primary)

            VStack(spacing: 8) {
                Text("Enable Face ID")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("Use Face ID for quick and secure access to your wallet")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
            }

            Toggle("Enable Face ID", isOn: $enableBiometrics)
                .padding()
                .background(EtridColors.cardBackground)
                .cornerRadius(12)
                .padding(.horizontal)

            Spacer()

            Button(action: onContinue) {
                Text("Complete Setup")
                    .font(.headline)
                    .foregroundColor(.white)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(EtridColors.primary)
                    .cornerRadius(16)
            }
            .padding(.horizontal)
            .padding(.bottom)
        }
    }
}

// MARK: - Completion Step

struct CompletionStepView: View {
    @State private var showCheckmark = false

    var body: some View {
        VStack(spacing: 32) {
            Spacer()

            ZStack {
                Circle()
                    .fill(EtridColors.primary.opacity(0.2))
                    .frame(width: 120, height: 120)

                if showCheckmark {
                    Image(systemName: "checkmark.circle.fill")
                        .font(.system(size: 80))
                        .foregroundColor(EtridColors.primary)
                        .transition(.scale.combined(with: .opacity))
                }
            }

            VStack(spacing: 8) {
                Text("You're All Set!")
                    .font(.title)
                    .fontWeight(.bold)

                Text("Your wallet is ready to use")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            Spacer()

            ProgressView()
                .padding()
        }
        .onAppear {
            withAnimation(.spring(response: 0.5, dampingFraction: 0.6)) {
                showCheckmark = true
            }
        }
    }
}

// MARK: - Lock Screen View

struct LockScreenView: View {
    @EnvironmentObject var walletManager: EtridWalletManager
    @State private var passcode = ""
    @State private var errorMessage: String?
    @State private var isAuthenticating = false

    var body: some View {
        VStack(spacing: 32) {
            Spacer()

            EtridLogo(size: 100)

            Text("ETRID Wallet")
                .font(.title)
                .fontWeight(.bold)

            Spacer()

            // Biometric Button
            Button(action: authenticateWithBiometrics) {
                HStack {
                    if isAuthenticating {
                        ProgressView()
                            .tint(.white)
                    } else {
                        Image(systemName: "faceid")
                        Text("Unlock with Face ID")
                    }
                }
                .font(.headline)
                .foregroundColor(.white)
                .frame(maxWidth: .infinity)
                .padding()
                .background(EtridColors.primary)
                .cornerRadius(16)
            }
            .disabled(isAuthenticating)
            .padding(.horizontal)

            // Passcode Entry
            VStack(spacing: 12) {
                SecureField("Enter passcode", text: $passcode)
                    .font(.system(.body, design: .monospaced))
                    .multilineTextAlignment(.center)
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)

                Button(action: unlockWithPasscode) {
                    Text("Unlock")
                        .font(.subheadline)
                        .foregroundColor(EtridColors.primary)
                }
                .disabled(passcode.isEmpty)
            }
            .padding(.horizontal)

            if let error = errorMessage {
                Text(error)
                    .font(.caption)
                    .foregroundColor(.red)
            }

            Spacer()

            // Development/Testing option
            #if DEBUG
            Button("Reset Wallet (Dev Only)") {
                resetForDev()
            }
            .font(.caption)
            .foregroundColor(.secondary)
            .padding(.bottom, 20)
            #endif
        }
    }

    private func authenticateWithBiometrics() {
        isAuthenticating = true
        errorMessage = nil

        Task {
            do {
                try await walletManager.unlockWithBiometrics()
            } catch {
                errorMessage = error.localizedDescription
            }
            isAuthenticating = false
        }
    }

    private func unlockWithPasscode() {
        errorMessage = nil

        Task {
            do {
                try await walletManager.unlockWithPasscode(passcode)
            } catch {
                errorMessage = error.localizedDescription
                passcode = ""
            }
        }
    }

    #if DEBUG
    private func resetForDev() {
        Task {
            try? await walletManager.deleteWallet()
        }
    }
    #endif
}
