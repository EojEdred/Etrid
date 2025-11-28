//
//  HapticService.swift
//  EtridWallet
//
//  Haptic feedback service for user interactions
//

import UIKit

/// Haptic feedback service for enhanced user experience
@MainActor
class HapticService {
    static let shared = HapticService()

    private let impactLight = UIImpactFeedbackGenerator(style: .light)
    private let impactMedium = UIImpactFeedbackGenerator(style: .medium)
    private let impactHeavy = UIImpactFeedbackGenerator(style: .heavy)
    private let selection = UISelectionFeedbackGenerator()
    private let notification = UINotificationFeedbackGenerator()

    private init() {
        // Prepare generators for reduced latency
        impactLight.prepare()
        impactMedium.prepare()
        impactHeavy.prepare()
        selection.prepare()
        notification.prepare()
    }

    // MARK: - Impact Feedback

    /// Light impact feedback (e.g., button tap)
    func tap() {
        impactLight.impactOccurred()
        impactLight.prepare()
    }

    /// Medium impact feedback (e.g., swipe action)
    func impact() {
        impactMedium.impactOccurred()
        impactMedium.prepare()
    }

    /// Heavy impact feedback (e.g., delete action)
    func heavyImpact() {
        impactHeavy.impactOccurred()
        impactHeavy.prepare()
    }

    // MARK: - Selection Feedback

    /// Selection change feedback (e.g., picker scroll)
    func selectionChanged() {
        selection.selectionChanged()
        selection.prepare()
    }

    // MARK: - Notification Feedback

    /// Success notification feedback
    func success() {
        notification.notificationOccurred(.success)
        notification.prepare()
    }

    /// Warning notification feedback
    func warning() {
        notification.notificationOccurred(.warning)
        notification.prepare()
    }

    /// Error notification feedback
    func error() {
        notification.notificationOccurred(.error)
        notification.prepare()
    }

    // MARK: - Context-Specific Feedback

    /// Transaction sent feedback
    func transactionSent() {
        impactMedium.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            self.notification.notificationOccurred(.success)
            self.notification.prepare()
        }
    }

    /// Transaction failed feedback
    func transactionFailed() {
        impactHeavy.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
            self.notification.notificationOccurred(.error)
            self.notification.prepare()
        }
    }

    /// Wallet unlocked feedback
    func walletUnlocked() {
        impactLight.impactOccurred()
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) {
            self.impactLight.impactOccurred()
            self.impactLight.prepare()
        }
    }

    /// QR code scanned feedback
    func qrCodeScanned() {
        impactMedium.impactOccurred()
        notification.notificationOccurred(.success)
        notification.prepare()
    }

    /// Copy to clipboard feedback
    func copied() {
        impactLight.impactOccurred()
        impactLight.prepare()
    }
}
