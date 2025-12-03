import SwiftUI
import CoreLocation
import MapKit
#if canImport(UIKit)
import UIKit
#endif

// MARK: - Main DeFi Hub View

struct DeFiHubView: View {
    @State private var selectedFeature: DeFiHubFeature = .virtualCard

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                // Feature Selector
                FeatureSelector(selectedFeature: $selectedFeature)
                    .padding(.horizontal)
                    .padding(.top, 8)

                // Content based on selection
                TabView(selection: $selectedFeature) {
                    VirtualCardComingSoonView()
                        .tag(DeFiHubFeature.virtualCard)

                    ATMLocatorView()
                        .tag(DeFiHubFeature.atmLocator)

                    ComingSoonFeatureView(feature: .cryptoLoans)
                        .tag(DeFiHubFeature.cryptoLoans)

                    ComingSoonFeatureView(feature: .yieldFarming)
                        .tag(DeFiHubFeature.yieldFarming)
                }
                .tabViewStyle(.page(indexDisplayMode: .never))
            }
            .background(EtridColors.background)
            .navigationTitle("DeFi Hub")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .topBarLeading) {
                    EtridLogo(size: 28)
                }
                #endif
            }
        }
    }
}

// MARK: - Feature Selector

struct FeatureSelector: View {
    @Binding var selectedFeature: DeFiHubFeature

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 12) {
                ForEach(DeFiHubFeature.allCases) { feature in
                    FeaturePill(
                        feature: feature,
                        isSelected: selectedFeature == feature
                    ) {
                        withAnimation(.spring(response: 0.3)) {
                            selectedFeature = feature
                        }
                    }
                }
            }
            .padding(.vertical, 8)
        }
    }
}

struct FeaturePill: View {
    let feature: DeFiHubFeature
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 6) {
                Image(systemName: feature.icon)
                    .font(.caption)

                Text(feature.rawValue)
                    .font(.caption)
                    .fontWeight(.medium)

                if !feature.isAvailable {
                    Text("SOON")
                        .font(.system(size: 8, weight: .bold))
                        .padding(.horizontal, 4)
                        .padding(.vertical, 2)
                        .background(Color.orange)
                        .foregroundColor(.white)
                        .cornerRadius(4)
                }
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 8)
            .background(isSelected ? EtridColors.primary : EtridColors.cardBackground)
            .foregroundColor(isSelected ? .white : .primary)
            .cornerRadius(20)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Virtual Card Coming Soon View

struct VirtualCardComingSoonView: View {
    @StateObject private var waitlistService = VirtualCardWaitlistService.shared
    @State private var showWaitlistSheet = false
    @State private var animateCard = false

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // Animated Card Preview
                VirtualCardPreview(animate: animateCard)
                    .padding(.top, 20)
                    .onAppear {
                        withAnimation(.easeInOut(duration: 2).repeatForever(autoreverses: true)) {
                            animateCard = true
                        }
                    }

                // Coming Soon Banner
                ComingSoonBanner()

                // Waitlist Status or Join Button
                if waitlistService.isRegistered {
                    WaitlistStatusCard(position: waitlistService.waitlistPosition ?? 0)
                } else {
                    JoinWaitlistButton {
                        showWaitlistSheet = true
                    }
                }

                // Features Preview
                FeaturesPreviewSection()

                // Card Tiers Preview
                CardTiersSection()
            }
            .padding()
        }
        .sheet(isPresented: $showWaitlistSheet) {
            WaitlistSignupSheet()
        }
    }
}

// MARK: - Virtual Card Preview (AU Lightning Bloc)

struct VirtualCardPreview: View {
    let animate: Bool

    var body: some View {
        ZStack {
            // Card Background - Dark with gold/lightning accents
            RoundedRectangle(cornerRadius: 20)
                .fill(
                    LinearGradient(
                        colors: [
                            Color(red: 0.05, green: 0.05, blue: 0.1),
                            Color(red: 0.1, green: 0.1, blue: 0.18)
                        ],
                        startPoint: .topLeading,
                        endPoint: .bottomTrailing
                    )
                )
                .overlay(
                    // Lightning/Gold holographic effect
                    RoundedRectangle(cornerRadius: 20)
                        .fill(
                            LinearGradient(
                                colors: [
                                    Color.yellow.opacity(animate ? 0.25 : 0.08),
                                    Color.orange.opacity(animate ? 0.15 : 0.05),
                                    Color.yellow.opacity(animate ? 0.08 : 0.25)
                                ],
                                startPoint: animate ? .topLeading : .bottomTrailing,
                                endPoint: animate ? .bottomTrailing : .topLeading
                            )
                        )
                )
                .shadow(color: Color.yellow.opacity(0.3), radius: animate ? 20 : 10, y: 5)

            // Card Content
            VStack(alignment: .leading, spacing: 0) {
                // Top Row - AU Lightning Bloc Logo
                HStack {
                    VStack(alignment: .leading, spacing: 2) {
                        HStack(spacing: 4) {
                            Image(systemName: "bolt.fill")
                                .font(.system(size: 16))
                                .foregroundColor(.yellow)
                            Text("AU")
                                .font(.system(size: 20, weight: .black, design: .rounded))
                                .foregroundColor(.yellow)
                        }
                        Text("LIGHTNING BLOC")
                            .font(.system(size: 10, weight: .bold, design: .rounded))
                            .foregroundColor(.white.opacity(0.8))
                            .tracking(2)
                    }

                    Spacer()

                    // Gold Chip
                    RoundedRectangle(cornerRadius: 4)
                        .fill(
                            LinearGradient(
                                colors: [Color(red: 0.85, green: 0.65, blue: 0.2), Color(red: 0.7, green: 0.5, blue: 0.15)],
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                        .frame(width: 42, height: 32)
                        .overlay(
                            VStack(spacing: 2) {
                                ForEach(0..<4, id: \.self) { _ in
                                    Rectangle()
                                        .fill(Color.black.opacity(0.2))
                                        .frame(height: 1)
                                }
                            }
                            .padding(6)
                        )
                }

                Spacer()

                // Card Number (Blurred)
                HStack(spacing: 16) {
                    ForEach(0..<4, id: \.self) { _ in
                        Text("****")
                            .font(.system(size: 18, weight: .medium, design: .monospaced))
                            .foregroundColor(.white.opacity(0.8))
                    }
                }

                Spacer()

                // Bottom Row
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("CARD HOLDER")
                            .font(.system(size: 8, weight: .medium))
                            .foregroundColor(.white.opacity(0.5))

                        Text("YOUR NAME")
                            .font(.system(size: 12, weight: .semibold))
                            .foregroundColor(.white)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text("VALID THRU")
                            .font(.system(size: 8, weight: .medium))
                            .foregroundColor(.white.opacity(0.5))

                        Text("**/**")
                            .font(.system(size: 12, weight: .semibold))
                            .foregroundColor(.white)
                    }

                    Spacer()

                    // Network logo
                    VStack(spacing: 2) {
                        Image(systemName: "bolt.horizontal.circle.fill")
                            .font(.system(size: 24))
                            .foregroundColor(.yellow)
                        Text("ËTRID")
                            .font(.system(size: 6, weight: .bold))
                            .foregroundColor(.white.opacity(0.6))
                    }
                }
            }
            .padding(24)
        }
        .frame(height: 200)
        .rotation3DEffect(
            .degrees(animate ? 2 : -2),
            axis: (x: 1, y: 0, z: 0)
        )
    }
}

// MARK: - Coming Soon Banner

struct ComingSoonBanner: View {
    var body: some View {
        VStack(spacing: 8) {
            HStack {
                Image(systemName: "bolt.fill")
                    .foregroundColor(.yellow)

                Text("COMING Q2 2026")
                    .font(.caption)
                    .fontWeight(.bold)
                    .foregroundColor(.yellow)

                Image(systemName: "bolt.fill")
                    .foregroundColor(.yellow)
            }

            Text("AU Lightning Bloc")
                .font(.title2)
                .fontWeight(.bold)

            Text("Crypto-collateralized virtual card. Spend your crypto anywhere without selling. Powered by the ËTRID Network.")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)
        }
        .padding(.vertical, 16)
    }
}

// MARK: - Waitlist Status Card

struct WaitlistStatusCard: View {
    let position: Int

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Image(systemName: "checkmark.circle.fill")
                    .foregroundColor(.green)
                    .font(.title2)

                Text("You're on the waitlist!")
                    .font(.headline)
                    .fontWeight(.semibold)
            }

            HStack(spacing: 4) {
                Text("Your position:")
                    .foregroundColor(.secondary)

                Text("#\(position)")
                    .font(.title)
                    .fontWeight(.bold)
                    .foregroundColor(EtridColors.primary)
            }

            Text("We'll notify you when your card is ready")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding(20)
        .background(
            RoundedRectangle(cornerRadius: 16)
                .fill(Color.green.opacity(0.1))
                .overlay(
                    RoundedRectangle(cornerRadius: 16)
                        .stroke(Color.green.opacity(0.3), lineWidth: 1)
                )
        )
    }
}

// MARK: - Join Waitlist Button

struct JoinWaitlistButton: View {
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: "bell.badge")

                Text("Join the Waitlist")
                    .fontWeight(.semibold)

                Spacer()

                Image(systemName: "chevron.right")
            }
            .foregroundColor(.white)
            .padding()
            .frame(maxWidth: .infinity)
            .background(
                LinearGradient(
                    colors: [EtridColors.primary, EtridColors.secondary],
                    startPoint: .leading,
                    endPoint: .trailing
                )
            )
            .cornerRadius(14)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Waitlist Signup Sheet

struct WaitlistSignupSheet: View {
    @Environment(\.dismiss) private var dismiss
    @StateObject private var waitlistService = VirtualCardWaitlistService.shared
    @EnvironmentObject var walletManager: EtridWalletManager

    @State private var email = ""
    @State private var selectedCardType: VirtualCardType = .standard
    @State private var selectedSpendingTier: SpendingTier = .medium
    @State private var agreedToTerms = false
    @State private var showSuccess = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    // Header
                    VStack(spacing: 8) {
                        HStack(spacing: 6) {
                            Image(systemName: "bolt.fill")
                                .font(.system(size: 40))
                                .foregroundColor(.yellow)
                            Text("AU")
                                .font(.system(size: 40, weight: .black, design: .rounded))
                                .foregroundColor(.yellow)
                        }

                        Text("Join the AU Lightning Bloc Waitlist")
                            .font(.title3)
                            .fontWeight(.bold)

                        Text("Be among the first to get the no-KYC crypto-collateralized virtual card")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                            .multilineTextAlignment(.center)
                    }
                    .padding(.top, 20)

                    // Email Input
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Email Address")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        TextField("your@email.com", text: $email)
                            .textFieldStyle(.plain)
                            .padding()
                            .background(EtridColors.cardBackground)
                            .cornerRadius(12)
                            #if os(iOS)
                            .keyboardType(.emailAddress)
                            .textInputAutocapitalization(.never)
                            .autocorrectionDisabled()
                            #endif
                    }

                    // Card Type Selection
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Preferred Card Tier")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        Picker("Card Type", selection: $selectedCardType) {
                            ForEach(VirtualCardType.allCases, id: \.self) { type in
                                Text(type.rawValue).tag(type)
                            }
                        }
                        .pickerStyle(.segmented)
                    }

                    // Card Tier Info
                    CardTierInfoCard(cardType: selectedCardType)

                    // Spending Tier Selection
                    VStack(alignment: .leading, spacing: 8) {
                        Text("Estimated Monthly Spend")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        ForEach(SpendingTier.allCases, id: \.self) { tier in
                            SpendingTierRow(
                                tier: tier,
                                isSelected: selectedSpendingTier == tier
                            ) {
                                selectedSpendingTier = tier
                            }
                        }
                    }

                    // Terms Agreement
                    Toggle(isOn: $agreedToTerms) {
                        Text("I agree to receive updates about the virtual card launch")
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .toggleStyle(CheckboxToggleStyle())

                    // Submit Button
                    Button(action: submitWaitlist) {
                        if waitlistService.isLoading {
                            ProgressView()
                                .progressViewStyle(CircularProgressViewStyle(tint: .white))
                        } else {
                            Text("Join Waitlist")
                                .fontWeight(.semibold)
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(canSubmit ? EtridColors.primary : Color.gray)
                    .foregroundColor(.white)
                    .cornerRadius(14)
                    .disabled(!canSubmit || waitlistService.isLoading)
                }
                .padding()
            }
            .navigationTitle("Waitlist Signup")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
            .alert("You're on the list!", isPresented: $showSuccess) {
                Button("OK") { dismiss() }
            } message: {
                if let position = waitlistService.waitlistPosition {
                    Text("Your waitlist position is #\(position). We'll notify you when your card is ready!")
                }
            }
        }
    }

    private var canSubmit: Bool {
        !email.isEmpty && agreedToTerms
    }

    private func submitWaitlist() {
        guard let walletAddress = walletManager.selectedAccount?.address else { return }

        Task {
            do {
                _ = try await waitlistService.registerForWaitlist(
                    email: email,
                    walletAddress: walletAddress,
                    cardType: selectedCardType,
                    spendingTier: selectedSpendingTier
                )
                showSuccess = true
            } catch {
                // Handle error
                print("Waitlist error: \(error)")
            }
        }
    }
}

// MARK: - Card Tier Info Card

struct CardTierInfoCard: View {
    let cardType: VirtualCardType

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                Text(cardType.rawValue)
                    .font(.headline)
                    .fontWeight(.bold)

                Spacer()

                Text("\(Int(cardType.cashbackRate * 100))% Cashback")
                    .font(.caption)
                    .fontWeight(.semibold)
                    .padding(.horizontal, 8)
                    .padding(.vertical, 4)
                    .background(EtridColors.primary.opacity(0.2))
                    .foregroundColor(EtridColors.primary)
                    .cornerRadius(8)
            }

            Divider()

            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Collateral Ratio")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("\(Int(cardType.collateralRatio * 100))%")
                        .font(.subheadline)
                        .fontWeight(.semibold)
                }

                Spacer()

                VStack(alignment: .center, spacing: 4) {
                    Text("Monthly Limit")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("$\(cardType.monthlyLimit as NSDecimalNumber)")
                        .font(.subheadline)
                        .fontWeight(.semibold)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("Min Stake")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("\(cardType.minimumStake as NSDecimalNumber) ETR")
                        .font(.subheadline)
                        .fontWeight(.semibold)
                }
            }
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

// MARK: - Spending Tier Row

struct SpendingTierRow: View {
    let tier: SpendingTier
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                    .foregroundColor(isSelected ? EtridColors.primary : .secondary)

                Text(tier.rawValue)
                    .foregroundColor(.primary)

                Spacer()
            }
            .padding()
            .background(isSelected ? EtridColors.primary.opacity(0.1) : EtridColors.cardBackground)
            .cornerRadius(10)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Checkbox Toggle Style

struct CheckboxToggleStyle: ToggleStyle {
    func makeBody(configuration: Configuration) -> some View {
        HStack {
            Image(systemName: configuration.isOn ? "checkmark.square.fill" : "square")
                .foregroundColor(configuration.isOn ? EtridColors.primary : .secondary)
                .onTapGesture { configuration.isOn.toggle() }

            configuration.label
        }
    }
}

// MARK: - Features Preview Section

struct FeaturesPreviewSection: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Card Features")
                .font(.headline)
                .fontWeight(.semibold)

            LazyVGrid(columns: [
                GridItem(.flexible()),
                GridItem(.flexible())
            ], spacing: 12) {
                ForEach(VirtualCardFeature.plannedFeatures) { feature in
                    FeatureCard(feature: feature)
                }
            }
        }
    }
}

struct FeatureCard: View {
    let feature: VirtualCardFeature

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Image(systemName: feature.icon)
                .font(.title2)
                .foregroundColor(EtridColors.primary)

            Text(feature.title)
                .font(.subheadline)
                .fontWeight(.semibold)

            Text(feature.description)
                .font(.caption)
                .foregroundColor(.secondary)
                .lineLimit(3)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }
}

// MARK: - Card Tiers Section

struct CardTiersSection: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Card Tiers")
                .font(.headline)
                .fontWeight(.semibold)

            ForEach(VirtualCardType.allCases, id: \.self) { tier in
                CardTierRow(tier: tier)
            }
        }
    }
}

struct CardTierRow: View {
    let tier: VirtualCardType

    var body: some View {
        HStack {
            // Card Icon
            RoundedRectangle(cornerRadius: 6)
                .fill(tierGradient)
                .frame(width: 50, height: 32)
                .overlay(
                    Text("Ë")
                        .font(.caption)
                        .fontWeight(.bold)
                        .foregroundColor(.white)
                )

            VStack(alignment: .leading, spacing: 2) {
                Text(tier.rawValue)
                    .font(.subheadline)
                    .fontWeight(.semibold)

                Text("\(Int(tier.cashbackRate * 100))% cashback • $\(tier.monthlyLimit as NSDecimalNumber)/mo limit")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Text("\(Int(tier.collateralRatio * 100))%")
                .font(.caption)
                .fontWeight(.medium)
                .foregroundColor(.secondary)
        }
        .padding()
        .background(EtridColors.cardBackground)
        .cornerRadius(12)
    }

    private var tierGradient: LinearGradient {
        switch tier {
        case .standard:
            return LinearGradient(colors: [.gray, .gray.opacity(0.7)], startPoint: .topLeading, endPoint: .bottomTrailing)
        case .premium:
            return LinearGradient(colors: [.yellow, .orange], startPoint: .topLeading, endPoint: .bottomTrailing)
        case .elite:
            return LinearGradient(colors: [.black, .gray], startPoint: .topLeading, endPoint: .bottomTrailing)
        }
    }
}

// MARK: - Coming Soon Feature View (Generic)

struct ComingSoonFeatureView: View {
    let feature: DeFiHubFeature

    var body: some View {
        VStack(spacing: 24) {
            Spacer()

            Image(systemName: feature.icon)
                .font(.system(size: 60))
                .foregroundColor(EtridColors.primary.opacity(0.5))

            VStack(spacing: 8) {
                Text(feature.rawValue)
                    .font(.title2)
                    .fontWeight(.bold)

                if let date = feature.comingSoonDate {
                    Text("Coming \(date)")
                        .font(.subheadline)
                        .foregroundColor(.orange)
                        .fontWeight(.semibold)
                }

                Text(feature.description)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 32)
            }

            Spacer()
        }
        .padding()
    }
}

// MARK: - ATM Locator View

struct ATMLocatorView: View {
    @StateObject private var atmService = ATMService.shared
    @State private var selectedATM: CryptoATM?
    @State private var showATMDetail = false
    @State private var viewMode: ATMViewMode = .list
    @State private var searchText = ""

    enum ATMViewMode: String, CaseIterable {
        case list = "List"
        case map = "Map"
    }

    var body: some View {
        VStack(spacing: 0) {
            // Search and View Toggle
            HStack {
                // Search Bar
                HStack {
                    Image(systemName: "magnifyingglass")
                        .foregroundColor(.secondary)

                    TextField("Search by address or city", text: $searchText)
                        .textFieldStyle(.plain)
                        #if os(iOS)
                        .autocorrectionDisabled()
                        #endif

                    if !searchText.isEmpty {
                        Button(action: { searchText = "" }) {
                            Image(systemName: "xmark.circle.fill")
                                .foregroundColor(.secondary)
                        }
                    }
                }
                .padding(10)
                .background(EtridColors.cardBackground)
                .cornerRadius(10)

                // View Mode Toggle
                Picker("View", selection: $viewMode) {
                    ForEach(ATMViewMode.allCases, id: \.self) { mode in
                        Image(systemName: mode == .list ? "list.bullet" : "map")
                            .tag(mode)
                    }
                }
                .pickerStyle(.segmented)
                .frame(width: 100)
            }
            .padding()

            // Content
            if atmService.isLoading {
                ProgressView("Finding nearby ATMs...")
                    .frame(maxWidth: .infinity, maxHeight: .infinity)
            } else if atmService.nearbyATMs.isEmpty {
                EmptyATMView {
                    Task { try? await atmService.fetchNearbyATMs() }
                }
            } else {
                if viewMode == .list {
                    ATMListView(
                        atms: filteredATMs,
                        selectedATM: $selectedATM,
                        showDetail: $showATMDetail
                    )
                } else {
                    ATMMapView(
                        atms: filteredATMs,
                        selectedATM: $selectedATM,
                        showDetail: $showATMDetail
                    )
                }
            }
        }
        .task {
            if atmService.nearbyATMs.isEmpty {
                try? await atmService.fetchNearbyATMs()
            }
        }
        .sheet(isPresented: $showATMDetail) {
            if let atm = selectedATM {
                ATMDetailSheet(atm: atm)
            }
        }
    }

    private var filteredATMs: [CryptoATM] {
        if searchText.isEmpty {
            return atmService.nearbyATMs
        }
        return atmService.nearbyATMs.filter { atm in
            atm.name.localizedCaseInsensitiveContains(searchText) ||
            atm.address.city.localizedCaseInsensitiveContains(searchText) ||
            atm.address.street.localizedCaseInsensitiveContains(searchText)
        }
    }
}

// MARK: - Empty ATM View

struct EmptyATMView: View {
    let onRefresh: () -> Void

    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "mappin.slash")
                .font(.system(size: 50))
                .foregroundColor(.secondary)

            Text("No ATMs Found Nearby")
                .font(.headline)

            Text("Try searching for a specific address or expand your search area")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)

            Button(action: onRefresh) {
                Label("Refresh", systemImage: "arrow.clockwise")
            }
            .buttonStyle(.borderedProminent)
            .tint(EtridColors.primary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

// MARK: - ATM List View

struct ATMListView: View {
    let atms: [CryptoATM]
    @Binding var selectedATM: CryptoATM?
    @Binding var showDetail: Bool

    var body: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                ForEach(atms) { atm in
                    ATMRowCard(atm: atm) {
                        selectedATM = atm
                        showDetail = true
                    }
                }
            }
            .padding()
        }
    }
}

// MARK: - ATM Row Card

struct ATMRowCard: View {
    let atm: CryptoATM
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            VStack(alignment: .leading, spacing: 12) {
                // Header
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        HStack {
                            Text(atm.name)
                                .font(.subheadline)
                                .fontWeight(.semibold)
                                .lineLimit(1)

                            if atm.verified {
                                Image(systemName: "checkmark.seal.fill")
                                    .font(.caption)
                                    .foregroundColor(.blue)
                            }
                        }

                        Text(atm.operator_name)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }

                    Spacer()

                    // Rating
                    if let rating = atm.rating {
                        HStack(spacing: 2) {
                            Image(systemName: "star.fill")
                                .font(.caption2)
                                .foregroundColor(.yellow)
                            Text(String(format: "%.1f", rating))
                                .font(.caption)
                                .fontWeight(.medium)
                        }
                    }
                }

                // Address
                HStack {
                    Image(systemName: "mappin")
                        .font(.caption)
                        .foregroundColor(.secondary)

                    Text(atm.address.shortAddress)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .lineLimit(1)

                    Spacer()

                    Text(atm.operatingHours.displayText)
                        .font(.caption2)
                        .foregroundColor(.green)
                }

                // Supported Cryptos & Type
                HStack {
                    // Supported cryptos
                    HStack(spacing: 4) {
                        ForEach(atm.supportedCryptos.prefix(4), id: \.self) { crypto in
                            Text(crypto.rawValue)
                                .font(.system(size: 9, weight: .semibold))
                                .padding(.horizontal, 6)
                                .padding(.vertical, 3)
                                .background(Color.secondary.opacity(0.15))
                                .cornerRadius(4)
                        }
                        if atm.supportedCryptos.count > 4 {
                            Text("+\(atm.supportedCryptos.count - 4)")
                                .font(.system(size: 9))
                                .foregroundColor(.secondary)
                        }
                    }

                    Spacer()

                    // Operation type badge
                    Text(atm.operationType.rawValue)
                        .font(.caption2)
                        .fontWeight(.medium)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(operationTypeBadgeColor.opacity(0.15))
                        .foregroundColor(operationTypeBadgeColor)
                        .cornerRadius(6)
                }

                // Fees
                HStack {
                    if atm.operationType.canBuyCrypto {
                        Label("Buy: \(atm.fees.formattedBuyFee)", systemImage: "arrow.down.circle")
                            .font(.caption2)
                            .foregroundColor(.green)
                    }

                    if atm.operationType.canWithdrawCash {
                        Label("Sell: \(atm.fees.formattedSellFee)", systemImage: "arrow.up.circle")
                            .font(.caption2)
                            .foregroundColor(.orange)
                    }

                    Spacer()

                    Image(systemName: "chevron.right")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            .padding()
            .background(EtridColors.cardBackground)
            .cornerRadius(14)
        }
        .buttonStyle(.plain)
    }

    private var operationTypeBadgeColor: Color {
        switch atm.operationType {
        case .buyOnly: return .green
        case .sellOnly: return .orange
        case .buyAndSell: return EtridColors.primary
        }
    }
}

// MARK: - ATM Map View

struct ATMMapView: View {
    let atms: [CryptoATM]
    @Binding var selectedATM: CryptoATM?
    @Binding var showDetail: Bool
    @State private var region = MKCoordinateRegion(
        center: CLLocationCoordinate2D(latitude: 34.0522, longitude: -118.2437),
        span: MKCoordinateSpan(latitudeDelta: 0.1, longitudeDelta: 0.1)
    )

    var body: some View {
        Map(coordinateRegion: $region, annotationItems: atms) { atm in
            MapAnnotation(coordinate: atm.location.coordinate) {
                ATMMapPin(atm: atm, isSelected: selectedATM?.id == atm.id) {
                    selectedATM = atm
                    showDetail = true
                }
            }
        }
        .edgesIgnoringSafeArea(.bottom)
    }
}

struct ATMMapPin: View {
    let atm: CryptoATM
    let isSelected: Bool
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            VStack(spacing: 0) {
                ZStack {
                    Circle()
                        .fill(isSelected ? EtridColors.primary : .white)
                        .frame(width: 36, height: 36)
                        .shadow(radius: 3)

                    Image(systemName: "bitcoinsign.circle.fill")
                        .font(.title3)
                        .foregroundColor(isSelected ? .white : EtridColors.primary)
                }

                // Pointer
                Triangle()
                    .fill(isSelected ? EtridColors.primary : .white)
                    .frame(width: 12, height: 8)
                    .offset(y: -2)
            }
        }
    }
}

struct Triangle: Shape {
    func path(in rect: CGRect) -> Path {
        var path = Path()
        path.move(to: CGPoint(x: rect.midX, y: rect.maxY))
        path.addLine(to: CGPoint(x: rect.minX, y: rect.minY))
        path.addLine(to: CGPoint(x: rect.maxX, y: rect.minY))
        path.closeSubpath()
        return path
    }
}

// MARK: - ATM Detail Sheet

struct ATMDetailSheet: View {
    @Environment(\.dismiss) private var dismiss
    let atm: CryptoATM
    @State private var showWithdrawalFlow = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Header
                    VStack(spacing: 8) {
                        HStack {
                            Text(atm.name)
                                .font(.title3)
                                .fontWeight(.bold)

                            if atm.verified {
                                Image(systemName: "checkmark.seal.fill")
                                    .foregroundColor(.blue)
                            }
                        }

                        Text(atm.operator_name)
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        if let rating = atm.rating {
                            HStack {
                                ForEach(0..<5) { index in
                                    Image(systemName: index < Int(rating) ? "star.fill" : "star")
                                        .foregroundColor(.yellow)
                                        .font(.caption)
                                }
                                Text("(\(atm.reviewCount) reviews)")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }
                    }

                    // Address Card
                    VStack(alignment: .leading, spacing: 8) {
                        Label("Location", systemImage: "mappin.circle.fill")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        Text(atm.address.fullAddress)
                            .font(.subheadline)

                        Text(atm.operatingHours.displayText)
                            .font(.caption)
                            .foregroundColor(.green)

                        // Open in Maps button
                        Button(action: openInMaps) {
                            Label("Open in Maps", systemImage: "map")
                                .font(.caption)
                        }
                        .buttonStyle(.bordered)
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)

                    // Supported Cryptos
                    VStack(alignment: .leading, spacing: 8) {
                        Label("Supported Cryptocurrencies", systemImage: "bitcoinsign.circle")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        LazyVGrid(columns: [GridItem(.adaptive(minimum: 80))], spacing: 8) {
                            ForEach(atm.supportedCryptos, id: \.self) { crypto in
                                HStack {
                                    Image(systemName: crypto.icon)
                                        .font(.caption)
                                    Text(crypto.rawValue)
                                        .font(.caption)
                                        .fontWeight(.medium)
                                }
                                .padding(.horizontal, 10)
                                .padding(.vertical, 6)
                                .background(EtridColors.primary.opacity(0.1))
                                .cornerRadius(8)
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)

                    // Fees & Limits
                    VStack(alignment: .leading, spacing: 12) {
                        Label("Fees & Limits", systemImage: "dollarsign.circle")
                            .font(.caption)
                            .foregroundColor(.secondary)

                        HStack(spacing: 20) {
                            if atm.operationType.canBuyCrypto {
                                VStack(alignment: .leading, spacing: 4) {
                                    Text("Buy Fee")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                    Text(atm.fees.formattedBuyFee)
                                        .font(.title3)
                                        .fontWeight(.bold)
                                        .foregroundColor(.green)
                                    Text("$\(atm.limits.minBuy as NSDecimalNumber) - $\(atm.limits.maxBuy as NSDecimalNumber)")
                                        .font(.caption2)
                                        .foregroundColor(.secondary)
                                }
                            }

                            if atm.operationType.canWithdrawCash {
                                VStack(alignment: .leading, spacing: 4) {
                                    Text("Sell Fee")
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                    Text(atm.fees.formattedSellFee)
                                        .font(.title3)
                                        .fontWeight(.bold)
                                        .foregroundColor(.orange)
                                    Text("$\(atm.limits.minSell as NSDecimalNumber) - $\(atm.limits.maxSell as NSDecimalNumber)")
                                        .font(.caption2)
                                        .foregroundColor(.secondary)
                                }
                            }
                        }
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
                    .background(EtridColors.cardBackground)
                    .cornerRadius(12)

                    // Action Buttons
                    if atm.operationType.canWithdrawCash {
                        Button(action: { showWithdrawalFlow = true }) {
                            HStack {
                                Image(systemName: "banknote")
                                Text("Withdraw Cash")
                                    .fontWeight(.semibold)
                            }
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(EtridColors.primary)
                            .foregroundColor(.white)
                            .cornerRadius(14)
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("ATM Details")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Close") { dismiss() }
                }
            }
            .sheet(isPresented: $showWithdrawalFlow) {
                ATMWithdrawalFlowView(atm: atm)
            }
        }
    }

    private func openInMaps() {
        let coordinate = atm.location.coordinate
        let mapItem = MKMapItem(placemark: MKPlacemark(coordinate: coordinate))
        mapItem.name = atm.name
        mapItem.openInMaps(launchOptions: [
            MKLaunchOptionsDirectionsModeKey: MKLaunchOptionsDirectionsModeDriving
        ])
    }
}

// MARK: - ATM Withdrawal Flow View

struct ATMWithdrawalFlowView: View {
    @Environment(\.dismiss) private var dismiss
    @EnvironmentObject var walletManager: EtridWalletManager
    @StateObject private var atmService = ATMService.shared

    let atm: CryptoATM
    @State private var step: WithdrawalStep = .selectCrypto
    @State private var selectedCrypto: ATMCrypto = .bitcoin
    @State private var cryptoAmount: String = ""
    @State private var estimatedFiat: Decimal = 0
    @State private var withdrawal: ATMWithdrawalRequest?
    @State private var showError = false
    @State private var errorMessage = ""

    enum WithdrawalStep {
        case selectCrypto
        case enterAmount
        case confirm
        case processing
        case showQR
    }

    var body: some View {
        NavigationStack {
            VStack {
                // Progress Indicator
                WithdrawalProgressIndicator(currentStep: step)
                    .padding()

                // Content based on step
                switch step {
                case .selectCrypto:
                    SelectCryptoView(
                        supportedCryptos: atm.supportedCryptos,
                        selectedCrypto: $selectedCrypto,
                        onContinue: { step = .enterAmount }
                    )

                case .enterAmount:
                    EnterAmountView(
                        crypto: selectedCrypto,
                        amount: $cryptoAmount,
                        estimatedFiat: $estimatedFiat,
                        limits: atm.limits,
                        fees: atm.fees,
                        onBack: { step = .selectCrypto },
                        onContinue: { step = .confirm }
                    )

                case .confirm:
                    ConfirmWithdrawalView(
                        atm: atm,
                        crypto: selectedCrypto,
                        cryptoAmount: Decimal(string: cryptoAmount) ?? 0,
                        estimatedFiat: estimatedFiat,
                        fee: calculateFee(),
                        onBack: { step = .enterAmount },
                        onConfirm: initiateWithdrawal
                    )

                case .processing:
                    ProcessingView()

                case .showQR:
                    if let withdrawal = withdrawal {
                        WithdrawalQRView(
                            withdrawal: withdrawal,
                            atm: atm,
                            onDone: { dismiss() }
                        )
                    }
                }
            }
            .navigationTitle("Withdraw Cash")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
            .alert("Error", isPresented: $showError) {
                Button("OK") { }
            } message: {
                Text(errorMessage)
            }
        }
    }

    private func calculateFee() -> Decimal {
        let amount = Decimal(string: cryptoAmount) ?? 0
        return amount * Decimal(atm.fees.sellPercentage / 100)
    }

    private func initiateWithdrawal() {
        step = .processing

        Task {
            do {
                guard let walletAddress = walletManager.selectedAccount?.address else {
                    throw ATMServiceError.withdrawalNotFound
                }

                let amount = Decimal(string: cryptoAmount) ?? 0
                withdrawal = try await atmService.initiateWithdrawal(
                    atm: atm,
                    cryptoAmount: amount,
                    crypto: selectedCrypto,
                    walletAddress: walletAddress
                )
                step = .showQR
            } catch {
                errorMessage = error.localizedDescription
                showError = true
                step = .confirm
            }
        }
    }
}

// MARK: - Withdrawal Progress Indicator

struct WithdrawalProgressIndicator: View {
    let currentStep: ATMWithdrawalFlowView.WithdrawalStep

    var body: some View {
        HStack(spacing: 0) {
            ForEach(0..<4) { index in
                Circle()
                    .fill(index <= stepIndex ? EtridColors.primary : Color.secondary.opacity(0.3))
                    .frame(width: 10, height: 10)

                if index < 3 {
                    Rectangle()
                        .fill(index < stepIndex ? EtridColors.primary : Color.secondary.opacity(0.3))
                        .frame(height: 2)
                }
            }
        }
        .padding(.horizontal, 40)
    }

    private var stepIndex: Int {
        switch currentStep {
        case .selectCrypto: return 0
        case .enterAmount: return 1
        case .confirm: return 2
        case .processing, .showQR: return 3
        }
    }
}

// MARK: - Select Crypto View

struct SelectCryptoView: View {
    let supportedCryptos: [ATMCrypto]
    @Binding var selectedCrypto: ATMCrypto
    let onContinue: () -> Void

    var body: some View {
        VStack(spacing: 20) {
            Text("Select Cryptocurrency")
                .font(.title2)
                .fontWeight(.bold)

            Text("Choose which crypto you want to sell for cash")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)

            ScrollView {
                VStack(spacing: 12) {
                    ForEach(supportedCryptos, id: \.self) { crypto in
                        CryptoSelectionRow(
                            crypto: crypto,
                            isSelected: selectedCrypto == crypto
                        ) {
                            selectedCrypto = crypto
                        }
                    }
                }
                .padding()
            }

            Button(action: onContinue) {
                Text("Continue")
                    .fontWeight(.semibold)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(EtridColors.primary)
                    .foregroundColor(.white)
                    .cornerRadius(14)
            }
            .padding(.horizontal)
        }
    }
}

struct CryptoSelectionRow: View {
    let crypto: ATMCrypto
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                Image(systemName: crypto.icon)
                    .font(.title2)
                    .foregroundColor(EtridColors.primary)
                    .frame(width: 40)

                VStack(alignment: .leading) {
                    Text(crypto.name)
                        .font(.headline)
                    Text(crypto.rawValue)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }

                Spacer()

                Image(systemName: isSelected ? "checkmark.circle.fill" : "circle")
                    .foregroundColor(isSelected ? EtridColors.primary : .secondary)
            }
            .padding()
            .background(isSelected ? EtridColors.primary.opacity(0.1) : EtridColors.cardBackground)
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(isSelected ? EtridColors.primary : Color.clear, lineWidth: 2)
            )
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Enter Amount View

struct EnterAmountView: View {
    let crypto: ATMCrypto
    @Binding var amount: String
    @Binding var estimatedFiat: Decimal
    let limits: ATMLimits
    let fees: ATMFees
    let onBack: () -> Void
    let onContinue: () -> Void

    var body: some View {
        VStack(spacing: 20) {
            Text("Enter Amount")
                .font(.title2)
                .fontWeight(.bold)

            VStack(spacing: 8) {
                HStack {
                    TextField("0.00", text: $amount)
                        .font(.system(size: 40, weight: .bold, design: .rounded))
                        .multilineTextAlignment(.center)
                        #if os(iOS)
                        .keyboardType(.decimalPad)
                        #endif

                    Text(crypto.rawValue)
                        .font(.title2)
                        .foregroundColor(.secondary)
                }

                Text("$\(estimatedFiat as NSDecimalNumber) USD")
                    .font(.title3)
                    .foregroundColor(.secondary)
            }
            .padding()
            .background(EtridColors.cardBackground)
            .cornerRadius(16)
            .padding(.horizontal)

            // Limits info
            VStack(spacing: 8) {
                HStack {
                    Text("Min withdrawal:")
                    Spacer()
                    Text("$\(limits.minSell as NSDecimalNumber)")
                }
                HStack {
                    Text("Max withdrawal:")
                    Spacer()
                    Text("$\(limits.maxSell as NSDecimalNumber)")
                }
                HStack {
                    Text("Fee:")
                    Spacer()
                    Text(fees.formattedSellFee)
                }
            }
            .font(.caption)
            .foregroundColor(.secondary)
            .padding(.horizontal)

            Spacer()

            HStack {
                Button(action: onBack) {
                    Text("Back")
                        .fontWeight(.medium)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .foregroundColor(.primary)
                        .cornerRadius(14)
                }

                Button(action: onContinue) {
                    Text("Continue")
                        .fontWeight(.semibold)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(isValidAmount ? EtridColors.primary : Color.gray)
                        .foregroundColor(.white)
                        .cornerRadius(14)
                }
                .disabled(!isValidAmount)
            }
            .padding(.horizontal)
        }
        .onChange(of: amount) { _, newValue in
            updateEstimatedFiat(newValue)
        }
    }

    private var isValidAmount: Bool {
        guard let value = Decimal(string: amount), value > 0 else { return false }
        return true
    }

    private func updateEstimatedFiat(_ amountString: String) {
        guard let cryptoValue = Decimal(string: amountString) else {
            estimatedFiat = 0
            return
        }
        // Mock exchange rate - in production, fetch real rates
        let rate: Decimal = {
            switch crypto {
            case .bitcoin: return 42000
            case .ethereum: return 2500
            case .litecoin: return 85
            case .dogecoin: return Decimal(string: "0.12") ?? 0
            case .usdt: return 1
            case .etrid: return Decimal(string: "0.15") ?? 0
            }
        }()
        estimatedFiat = cryptoValue * rate
    }
}

// MARK: - Confirm Withdrawal View

struct ConfirmWithdrawalView: View {
    let atm: CryptoATM
    let crypto: ATMCrypto
    let cryptoAmount: Decimal
    let estimatedFiat: Decimal
    let fee: Decimal
    let onBack: () -> Void
    let onConfirm: () -> Void

    var body: some View {
        VStack(spacing: 20) {
            Text("Confirm Withdrawal")
                .font(.title2)
                .fontWeight(.bold)

            VStack(spacing: 16) {
                // Amount details
                VStack(spacing: 8) {
                    Text("You're selling")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text("\(cryptoAmount as NSDecimalNumber) \(crypto.rawValue)")
                        .font(.title)
                        .fontWeight(.bold)
                }

                Divider()

                // Breakdown
                VStack(spacing: 8) {
                    HStack {
                        Text("Gross amount")
                        Spacer()
                        Text("$\(estimatedFiat as NSDecimalNumber)")
                    }
                    HStack {
                        Text("ATM fee (\(atm.fees.formattedSellFee))")
                        Spacer()
                        Text("-$\((estimatedFiat * Decimal(atm.fees.sellPercentage / 100)) as NSDecimalNumber)")
                            .foregroundColor(.orange)
                    }
                    Divider()
                    HStack {
                        Text("You'll receive")
                            .fontWeight(.semibold)
                        Spacer()
                        Text("$\((estimatedFiat - estimatedFiat * Decimal(atm.fees.sellPercentage / 100)) as NSDecimalNumber)")
                            .fontWeight(.bold)
                            .foregroundColor(.green)
                    }
                }
                .font(.subheadline)

                Divider()

                // ATM info
                VStack(alignment: .leading, spacing: 4) {
                    Text("ATM Location")
                        .font(.caption)
                        .foregroundColor(.secondary)
                    Text(atm.name)
                        .font(.subheadline)
                        .fontWeight(.medium)
                    Text(atm.address.shortAddress)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
            }
            .padding()
            .background(EtridColors.cardBackground)
            .cornerRadius(16)
            .padding(.horizontal)

            // Warning
            HStack {
                Image(systemName: "exclamationmark.triangle")
                    .foregroundColor(.orange)
                Text("QR code will expire in 30 minutes")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            HStack {
                Button(action: onBack) {
                    Text("Back")
                        .fontWeight(.medium)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .foregroundColor(.primary)
                        .cornerRadius(14)
                }

                Button(action: onConfirm) {
                    Text("Confirm")
                        .fontWeight(.semibold)
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(EtridColors.primary)
                        .foregroundColor(.white)
                        .cornerRadius(14)
                }
            }
            .padding(.horizontal)
        }
    }
}

// MARK: - Processing View

struct ProcessingView: View {
    var body: some View {
        VStack(spacing: 20) {
            Spacer()

            ProgressView()
                .scaleEffect(1.5)

            Text("Processing...")
                .font(.headline)

            Text("Generating your withdrawal QR code")
                .font(.subheadline)
                .foregroundColor(.secondary)

            Spacer()
        }
    }
}

// MARK: - Withdrawal QR View

struct WithdrawalQRView: View {
    let withdrawal: ATMWithdrawalRequest
    let atm: CryptoATM
    let onDone: () -> Void
    @State private var timeRemaining: TimeInterval = 0
    let timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()

    var body: some View {
        VStack(spacing: 20) {
            // Success header
            VStack(spacing: 8) {
                Image(systemName: "checkmark.circle.fill")
                    .font(.system(size: 50))
                    .foregroundColor(.green)

                Text("Withdrawal Ready!")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("Scan this QR code at the ATM")
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }

            // QR Code
            if let qrData = withdrawal.qrCodeData {
                ATMQRCodeView(data: qrData)
                    .frame(width: 200, height: 200)
                    .padding()
                    .background(Color.white)
                    .cornerRadius(16)
            }

            // Timer
            VStack(spacing: 4) {
                Text("Expires in")
                    .font(.caption)
                    .foregroundColor(.secondary)
                Text(withdrawal.formattedTimeRemaining)
                    .font(.system(size: 32, weight: .bold, design: .monospaced))
                    .foregroundColor(timeRemaining < 300 ? .orange : .primary)
            }
            .onReceive(timer) { _ in
                timeRemaining = withdrawal.timeRemaining
            }
            .onAppear {
                timeRemaining = withdrawal.timeRemaining
            }

            // Amount info
            VStack(spacing: 8) {
                Text("You'll receive")
                    .font(.caption)
                    .foregroundColor(.secondary)
                Text("$\(withdrawal.fiatAmount as NSDecimalNumber) \(withdrawal.fiatCurrency)")
                    .font(.title)
                    .fontWeight(.bold)
                    .foregroundColor(.green)
            }

            // ATM address
            VStack(spacing: 4) {
                Text(atm.name)
                    .font(.subheadline)
                    .fontWeight(.medium)
                Text(atm.address.shortAddress)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()

            Button(action: onDone) {
                Text("Done")
                    .fontWeight(.semibold)
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(EtridColors.primary)
                    .foregroundColor(.white)
                    .cornerRadius(14)
            }
            .padding(.horizontal)
        }
        .padding()
    }
}

// MARK: - ATM QR Code View

struct ATMQRCodeView: View {
    let data: String

    var body: some View {
        #if canImport(UIKit)
        if let image = generateQRCode(from: data) {
            Image(uiImage: image)
                .interpolation(.none)
                .resizable()
                .scaledToFit()
        } else {
            Image(systemName: "qrcode")
                .font(.system(size: 100))
                .foregroundColor(.gray)
        }
        #else
        Image(systemName: "qrcode")
            .font(.system(size: 100))
            .foregroundColor(.gray)
        #endif
    }

    #if canImport(UIKit)
    private func generateQRCode(from string: String) -> UIImage? {
        let data = string.data(using: .utf8)
        guard let filter = CIFilter(name: "CIQRCodeGenerator") else { return nil }
        filter.setValue(data, forKey: "inputMessage")
        filter.setValue("H", forKey: "inputCorrectionLevel")

        guard let outputImage = filter.outputImage else { return nil }

        let scaleX = 200 / outputImage.extent.size.width
        let scaleY = 200 / outputImage.extent.size.height
        let transformedImage = outputImage.transformed(by: CGAffineTransform(scaleX: scaleX, y: scaleY))

        let context = CIContext()
        guard let cgImage = context.createCGImage(transformedImage, from: transformedImage.extent) else { return nil }

        return UIImage(cgImage: cgImage)
    }
    #endif
}
