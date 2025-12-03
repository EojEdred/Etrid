//
//  Haptics.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import UIKit
import SwiftUI

/// Haptic feedback system with accessibility awareness
struct Haptics {

    // MARK: - Notification Feedback

    /// Success haptic feedback
    static func success() {
        guard isEnabled else { return }

        let generator = UINotificationFeedbackGenerator()
        generator.notificationOccurred(.success)

        logDebug("Haptic: Success", category: .ui)
    }

    /// Error haptic feedback
    static func error() {
        guard isEnabled else { return }

        let generator = UINotificationFeedbackGenerator()
        generator.notificationOccurred(.error)

        logDebug("Haptic: Error", category: .ui)
    }

    /// Warning haptic feedback
    static func warning() {
        guard isEnabled else { return }

        let generator = UINotificationFeedbackGenerator()
        generator.notificationOccurred(.warning)

        logDebug("Haptic: Warning", category: .ui)
    }

    // MARK: - Impact Feedback

    /// Light impact feedback
    static func light() {
        impact(.light)
    }

    /// Medium impact feedback
    static func medium() {
        impact(.medium)
    }

    /// Heavy impact feedback
    static func heavy() {
        impact(.heavy)
    }

    /// Soft impact feedback (iOS 13+)
    static func soft() {
        impact(.soft)
    }

    /// Rigid impact feedback (iOS 13+)
    static func rigid() {
        impact(.rigid)
    }

    /// Generic impact feedback with style
    static func impact(_ style: UIImpactFeedbackGenerator.FeedbackStyle = .medium) {
        guard isEnabled else { return }

        let generator = UIImpactFeedbackGenerator(style: style)
        generator.prepare()
        generator.impactOccurred()

        logDebug("Haptic: Impact (\(style))", category: .ui)
    }

    // MARK: - Selection Feedback

    /// Selection changed feedback
    static func selection() {
        guard isEnabled else { return }

        let generator = UISelectionFeedbackGenerator()
        generator.prepare()
        generator.selectionChanged()

        logDebug("Haptic: Selection", category: .ui)
    }

    // MARK: - Context-Specific Haptics

    /// Button tap feedback
    static func buttonTap() {
        impact(.light)
    }

    /// Toggle switch feedback
    static func toggle() {
        impact(.medium)
    }

    /// Slider value changed feedback
    static func sliderChanged() {
        selection()
    }

    /// Transaction sent feedback
    static func transactionSent() {
        success()
    }

    /// Transaction confirmed feedback
    static func transactionConfirmed() {
        // Double success pattern
        success()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
            success()
        }
    }

    /// Transaction failed feedback
    static func transactionFailed() {
        // Triple error pattern
        error()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            error()
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
            error()
        }
    }

    /// Wallet created feedback
    static func walletCreated() {
        // Success + medium impact
        success()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.15) {
            impact(.medium)
        }
    }

    /// Copied to clipboard feedback
    static func copied() {
        impact(.light)
    }

    /// QR code scanned feedback
    static func qrScanned() {
        success()
    }

    /// Biometric authentication success
    static func biometricSuccess() {
        success()
    }

    /// Biometric authentication failed
    static func biometricFailed() {
        error()
    }

    /// Pull to refresh feedback
    static func pullToRefresh() {
        impact(.medium)
    }

    /// Swipe action feedback
    static func swipe() {
        selection()
    }

    /// Long press feedback
    static func longPress() {
        impact(.heavy)
    }

    /// Delete/remove feedback
    static func delete() {
        warning()
    }

    /// Navigation feedback
    static func navigate() {
        impact(.light)
    }

    /// Modal presented feedback
    static func modalPresented() {
        impact(.medium)
    }

    /// Modal dismissed feedback
    static func modalDismissed() {
        impact(.light)
    }

    // MARK: - Advanced Patterns

    /// Custom haptic pattern
    static func customPattern(impacts: [(UIImpactFeedbackGenerator.FeedbackStyle, TimeInterval)]) {
        guard isEnabled else { return }

        for (index, impact) in impacts.enumerated() {
            let delay = index == 0 ? 0 : impact.1
            DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
                self.impact(impact.0)
            }
        }
    }

    /// Celebration pattern (multiple haptics)
    static func celebrate() {
        success()

        let pattern: [(UIImpactFeedbackGenerator.FeedbackStyle, TimeInterval)] = [
            (.light, 0.1),
            (.medium, 0.2),
            (.heavy, 0.3),
            (.medium, 0.4),
            (.light, 0.5)
        ]

        customPattern(impacts: pattern)
    }

    /// Alert pattern (attention-grabbing)
    static func alert() {
        warning()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.15) {
            warning()
        }
    }

    /// Countdown feedback (3, 2, 1, go!)
    static func countdown(count: Int = 3, completion: @escaping () -> Void) {
        guard isEnabled else {
            completion()
            return
        }

        var remaining = count

        func tick() {
            if remaining > 0 {
                impact(.medium)
                remaining -= 1
                DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) {
                    tick()
                }
            } else {
                success()
                completion()
            }
        }

        tick()
    }

    // MARK: - Configuration

    /// Check if haptics are enabled
    static var isEnabled: Bool {
        // Check user preference
        guard AppConfig.shared.enableHaptics else { return false }

        // Check device capability
        guard UIDevice.current.userInterfaceIdiom == .phone else { return false }

        // Respect reduce motion accessibility setting
        guard !UIAccessibilityIsReduceMotionEnabled() else { return false }

        return true
    }

    /// Prepare haptic generator for immediate use
    static func prepare() {
        guard isEnabled else { return }

        let notificationGenerator = UINotificationFeedbackGenerator()
        notificationGenerator.prepare()

        let impactGenerator = UIImpactFeedbackGenerator()
        impactGenerator.prepare()

        let selectionGenerator = UISelectionFeedbackGenerator()
        selectionGenerator.prepare()
    }

    // MARK: - Testing

    /// Test all haptic types (for debugging)
    static func testAllHaptics() {
        print("Testing haptic feedback...")

        success()
        Thread.sleep(forTimeInterval: 0.5)

        error()
        Thread.sleep(forTimeInterval: 0.5)

        warning()
        Thread.sleep(forTimeInterval: 0.5)

        light()
        Thread.sleep(forTimeInterval: 0.5)

        medium()
        Thread.sleep(forTimeInterval: 0.5)

        heavy()
        Thread.sleep(forTimeInterval: 0.5)

        selection()
        Thread.sleep(forTimeInterval: 0.5)

        print("Haptic test complete!")
    }
}

// MARK: - SwiftUI View Modifier

struct HapticFeedback: ViewModifier {
    let haptic: () -> Void

    func body(content: Content) -> some View {
        content
            .simultaneousGesture(
                TapGesture()
                    .onEnded { _ in
                        haptic()
                    }
            )
    }
}

extension View {
    /// Add haptic feedback to tap gesture
    func hapticFeedback(_ haptic: @escaping () -> Void = Haptics.buttonTap) -> some View {
        modifier(HapticFeedback(haptic: haptic))
    }

    /// Add success haptic on tap
    func successHaptic() -> some View {
        hapticFeedback(Haptics.success)
    }

    /// Add selection haptic on tap
    func selectionHaptic() -> some View {
        hapticFeedback(Haptics.selection)
    }

    /// Add button tap haptic
    func buttonHaptic() -> some View {
        hapticFeedback(Haptics.buttonTap)
    }
}

// MARK: - Button Extension

extension Button {
    /// Create button with haptic feedback
    init(
        haptic: @escaping () -> Void = Haptics.buttonTap,
        action: @escaping () -> Void,
        @ViewBuilder label: () -> Label
    ) where Label: View {
        self.init(action: {
            haptic()
            action()
        }, label: label)
    }
}

// MARK: - Async Haptics

extension Haptics {
    /// Play haptic feedback asynchronously
    @MainActor
    static func playAsync(_ feedback: @escaping () -> Void) {
        Task { @MainActor in
            feedback()
        }
    }

    /// Play success haptic asynchronously
    @MainActor
    static func successAsync() {
        playAsync(success)
    }

    /// Play error haptic asynchronously
    @MainActor
    static func errorAsync() {
        playAsync(error)
    }

    /// Play selection haptic asynchronously
    @MainActor
    static func selectionAsync() {
        playAsync(selection)
    }
}

// MARK: - Haptic Manager (for complex sequences)

@MainActor
class HapticManager: ObservableObject {
    static let shared = HapticManager()

    @Published var isEnabled = true

    private var sequences: [String: Task<Void, Never>] = [:]

    private init() {
        isEnabled = AppConfig.shared.enableHaptics
    }

    /// Play a named haptic sequence
    func playSequence(_ name: String) {
        // Cancel existing sequence with same name
        sequences[name]?.cancel()

        sequences[name] = Task {
            switch name {
            case "transaction_sent":
                Haptics.transactionSent()
            case "transaction_confirmed":
                Haptics.transactionConfirmed()
            case "transaction_failed":
                Haptics.transactionFailed()
            case "wallet_created":
                Haptics.walletCreated()
            case "celebrate":
                Haptics.celebrate()
            default:
                Haptics.success()
            }

            sequences.removeValue(forKey: name)
        }
    }

    /// Cancel all haptic sequences
    func cancelAll() {
        for (_, task) in sequences {
            task.cancel()
        }
        sequences.removeAll()
    }

    /// Update enabled state
    func setEnabled(_ enabled: Bool) {
        isEnabled = enabled
        AppConfig.shared.enableHaptics = enabled

        if !enabled {
            cancelAll()
        }

        logInfo("Haptic feedback \(enabled ? "enabled" : "disabled")", category: .ui)
    }
}

// MARK: - Preview Helpers

#if DEBUG
struct HapticsTestView: View {
    var body: some View {
        List {
            Section("Notification Feedback") {
                Button("Success") { Haptics.success() }
                Button("Error") { Haptics.error() }
                Button("Warning") { Haptics.warning() }
            }

            Section("Impact Feedback") {
                Button("Light") { Haptics.light() }
                Button("Medium") { Haptics.medium() }
                Button("Heavy") { Haptics.heavy() }
                Button("Soft") { Haptics.soft() }
                Button("Rigid") { Haptics.rigid() }
            }

            Section("Selection") {
                Button("Selection") { Haptics.selection() }
            }

            Section("Context-Specific") {
                Button("Transaction Sent") { Haptics.transactionSent() }
                Button("Transaction Confirmed") { Haptics.transactionConfirmed() }
                Button("Transaction Failed") { Haptics.transactionFailed() }
                Button("Wallet Created") { Haptics.walletCreated() }
                Button("Celebrate") { Haptics.celebrate() }
            }

            Section("Other") {
                Button("Test All Haptics") {
                    Task {
                        Haptics.testAllHaptics()
                    }
                }
            }
        }
        .navigationTitle("Haptics Test")
    }
}

struct HapticsTestView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            HapticsTestView()
        }
    }
}
#endif
