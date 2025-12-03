//
//  NFCPaymentView.swift
//  EtridWallet
//
//  NFC payment simulation with animations
//

import SwiftUI

struct NFCPaymentView: View {
    let card: CryptoCard
    let onComplete: () -> Void

    @Environment(\.dismiss) var dismiss
    @State private var merchantName = ""
    @State private var amount = ""
    @State private var selectedCategory = "Shopping"
    @State private var isProcessing = false
    @State private var paymentStatus: PaymentStatus = .idle
    @State private var nfcAnimationProgress: CGFloat = 0
    @State private var showSuccess = false
    @State private var showError = false
    @State private var errorMessage = ""

    private let categories = ["Shopping", "Restaurant", "Grocery", "Gas", "Entertainment", "Travel", "Health", "Other"]

    var body: some View {
        NavigationView {
            ZStack {
                // Background
                LinearGradient(
                    gradient: Gradient(colors: [Color(hex: "0F172A"), Color(hex: "1E293B")]),
                    startPoint: .top,
                    endPoint: .bottom
                )
                .ignoresSafeArea()

                ScrollView {
                    VStack(spacing: 24) {
                        // NFC Animation
                        NFCAnimationView(
                            isAnimating: $isProcessing,
                            status: $paymentStatus
                        )
                        .padding(.top, 40)

                        if paymentStatus == .idle || paymentStatus == .scanning {
                            // Payment form
                            PaymentFormView(
                                merchantName: $merchantName,
                                amount: $amount,
                                selectedCategory: $selectedCategory,
                                categories: categories,
                                card: card
                            )
                            .padding(.horizontal)

                            // Pay button
                            Button(action: processPayment) {
                                HStack {
                                    Image(systemName: "wave.3.right")
                                    Text("Pay with NFC")
                                        .fontWeight(.semibold)
                                }
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding(.vertical, 16)
                                .background(isPaymentValid ? Color.blue : Color.gray)
                                .cornerRadius(12)
                            }
                            .disabled(!isPaymentValid || isProcessing)
                            .padding(.horizontal)
                        } else if paymentStatus == .success {
                            // Success state
                            PaymentSuccessView(
                                merchantName: merchantName,
                                amount: Double(amount) ?? 0,
                                card: card
                            )
                            .padding(.horizontal)

                            Button("Done") {
                                onComplete()
                                dismiss()
                            }
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 16)
                            .background(Color.blue)
                            .cornerRadius(12)
                            .padding(.horizontal)
                        } else if paymentStatus == .failed {
                            // Error state
                            PaymentErrorView(errorMessage: errorMessage)
                                .padding(.horizontal)

                            Button("Try Again") {
                                paymentStatus = .idle
                                errorMessage = ""
                            }
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 16)
                            .background(Color.blue)
                            .cornerRadius(12)
                            .padding(.horizontal)
                        }
                    }
                    .padding(.bottom, 40)
                }
            }
            .navigationTitle("NFC Payment")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                    .foregroundColor(.white)
                }
            }
        }
    }

    private var isPaymentValid: Bool {
        !merchantName.isEmpty &&
        !amount.isEmpty &&
        (Double(amount) ?? 0) > 0 &&
        (Double(amount) ?? 0) <= card.collateral.availableAmount
    }

    private func processPayment() {
        guard let paymentAmount = Double(amount), paymentAmount > 0 else { return }

        isProcessing = true
        paymentStatus = .scanning

        // Simulate NFC scanning
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) {
            paymentStatus = .processing

            // Process payment
            Task {
                do {
                    let request = NFCPaymentRequest(
                        cardId: card.id,
                        merchantId: "merchant_\(UUID().uuidString.prefix(8))",
                        merchantName: merchantName,
                        amount: paymentAmount,
                        currency: "USD",
                        merchantCategory: selectedCategory,
                        location: nil,
                        timestamp: Date(),
                        nonce: UUID().uuidString
                    )

                    let response = try await CardService.shared.processNFCPayment(request: request)

                    DispatchQueue.main.async {
                        if response.success {
                            paymentStatus = .success
                            // Haptic feedback
                            let generator = UINotificationFeedbackGenerator()
                            generator.notificationOccurred(.success)
                        } else {
                            paymentStatus = .failed
                            errorMessage = response.message
                            // Haptic feedback
                            let generator = UINotificationFeedbackGenerator()
                            generator.notificationOccurred(.error)
                        }
                        isProcessing = false
                    }
                } catch {
                    DispatchQueue.main.async {
                        paymentStatus = .failed
                        errorMessage = error.localizedDescription
                        isProcessing = false
                        // Haptic feedback
                        let generator = UINotificationFeedbackGenerator()
                        generator.notificationOccurred(.error)
                    }
                }
            }
        }
    }
}

// MARK: - Payment Status

enum PaymentStatus {
    case idle
    case scanning
    case processing
    case success
    case failed
}

// MARK: - NFC Animation

struct NFCAnimationView: View {
    @Binding var isAnimating: Bool
    @Binding var status: PaymentStatus

    @State private var pulseScale: CGFloat = 1.0
    @State private var waveOpacity: Double = 0.0
    @State private var rotationDegrees: Double = 0

    var body: some View {
        ZStack {
            // Pulsing waves
            if status == .scanning || status == .processing {
                ForEach(0..<3) { index in
                    Circle()
                        .stroke(statusColor.opacity(0.3), lineWidth: 2)
                        .scaleEffect(pulseScale + CGFloat(index) * 0.3)
                        .opacity(waveOpacity)
                }
            }

            // Center icon
            ZStack {
                Circle()
                    .fill(statusColor.opacity(0.2))
                    .frame(width: 120, height: 120)

                Image(systemName: statusIcon)
                    .font(.system(size: 50))
                    .foregroundColor(statusColor)
                    .rotationEffect(.degrees(rotationDegrees))
            }

            // Success checkmark
            if status == .success {
                Image(systemName: "checkmark.circle.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.green)
                    .transition(.scale.combined(with: .opacity))
            }

            // Error icon
            if status == .failed {
                Image(systemName: "xmark.circle.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.red)
                    .transition(.scale.combined(with: .opacity))
            }
        }
        .frame(height: 200)
        .onChange(of: status) { newStatus in
            updateAnimation(for: newStatus)
        }
        .onAppear {
            startIdleAnimation()
        }
    }

    private var statusIcon: String {
        switch status {
        case .idle: return "wave.3.right"
        case .scanning: return "wave.3.right"
        case .processing: return "bolt.fill"
        case .success: return "checkmark"
        case .failed: return "xmark"
        }
    }

    private var statusColor: Color {
        switch status {
        case .idle: return .blue
        case .scanning: return .blue
        case .processing: return .orange
        case .success: return .green
        case .failed: return .red
        }
    }

    private func startIdleAnimation() {
        withAnimation(.easeInOut(duration: 2).repeatForever(autoreverses: true)) {
            pulseScale = 1.2
        }
    }

    private func updateAnimation(for status: PaymentStatus) {
        switch status {
        case .scanning:
            withAnimation(.easeInOut(duration: 1).repeatForever(autoreverses: true)) {
                pulseScale = 1.5
                waveOpacity = 0.6
            }
        case .processing:
            withAnimation(.linear(duration: 1).repeatForever(autoreverses: false)) {
                rotationDegrees = 360
            }
            withAnimation(.easeInOut(duration: 0.5).repeatForever(autoreverses: true)) {
                pulseScale = 1.3
                waveOpacity = 0.8
            }
        case .success, .failed:
            withAnimation(.spring()) {
                pulseScale = 1.0
                waveOpacity = 0.0
                rotationDegrees = 0
            }
        case .idle:
            startIdleAnimation()
        }
    }
}

// MARK: - Payment Form

struct PaymentFormView: View {
    @Binding var merchantName: String
    @Binding var amount: String
    @Binding var selectedCategory: String
    let categories: [String]
    let card: CryptoCard

    var body: some View {
        VStack(spacing: 16) {
            // Merchant name
            VStack(alignment: .leading, spacing: 8) {
                Text("Merchant")
                    .font(.system(size: 12, weight: .medium))
                    .foregroundColor(.white.opacity(0.6))

                TextField("Enter merchant name", text: $merchantName)
                    .textFieldStyle(CustomTextFieldStyle())
            }

            // Amount
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text("Amount")
                        .font(.system(size: 12, weight: .medium))
                        .foregroundColor(.white.opacity(0.6))

                    Spacer()

                    Text("Available: $\(card.collateral.availableAmount, specifier: "%.2f")")
                        .font(.system(size: 12))
                        .foregroundColor(.green.opacity(0.8))
                }

                HStack {
                    Text("$")
                        .font(.system(size: 24, weight: .semibold))
                        .foregroundColor(.white)

                    TextField("0.00", text: $amount)
                        .keyboardType(.decimalPad)
                        .font(.system(size: 24, weight: .semibold))
                        .foregroundColor(.white)
                        .textFieldStyle(PlainTextFieldStyle())
                }
                .padding()
                .background(Color.white.opacity(0.1))
                .cornerRadius(12)
            }

            // Category
            VStack(alignment: .leading, spacing: 8) {
                Text("Category")
                    .font(.system(size: 12, weight: .medium))
                    .foregroundColor(.white.opacity(0.6))

                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 8) {
                        ForEach(categories, id: \.self) { category in
                            CategoryChip(
                                title: category,
                                isSelected: selectedCategory == category
                            ) {
                                selectedCategory = category
                            }
                        }
                    }
                }
            }

            // Spending limits warning
            if let amountValue = Double(amount), amountValue > card.limits.dailyRemaining {
                HStack(spacing: 8) {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .foregroundColor(.orange)
                    Text("Exceeds daily limit")
                        .font(.system(size: 12))
                        .foregroundColor(.orange)
                    Spacer()
                }
                .padding()
                .background(Color.orange.opacity(0.1))
                .cornerRadius(8)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

struct CustomTextFieldStyle: TextFieldStyle {
    func _body(configuration: TextField<Self._Label>) -> some View {
        configuration
            .padding()
            .background(Color.white.opacity(0.1))
            .cornerRadius(12)
            .foregroundColor(.white)
    }
}

struct CategoryChip: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.system(size: 12, weight: .medium))
                .foregroundColor(isSelected ? .white : .white.opacity(0.6))
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(isSelected ? Color.blue : Color.white.opacity(0.1))
                .cornerRadius(20)
        }
    }
}

// MARK: - Payment Success

struct PaymentSuccessView: View {
    let merchantName: String
    let amount: Double
    let card: CryptoCard

    var body: some View {
        VStack(spacing: 24) {
            // Success animation
            LottieSuccessView()

            VStack(spacing: 8) {
                Text("Payment Successful!")
                    .font(.system(size: 24, weight: .bold))
                    .foregroundColor(.white)

                Text("Your payment has been processed")
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.6))
            }

            // Transaction details
            VStack(spacing: 16) {
                DetailRow(label: "Merchant", value: merchantName)
                DetailRow(label: "Amount", value: String(format: "$%.2f", amount))
                DetailRow(label: "Payment Method", value: "••••  \(card.cardNumber)")
                DetailRow(label: "Date", value: Date().formatted(date: .abbreviated, time: .shortened))
            }
            .padding()
            .background(Color.white.opacity(0.05))
            .cornerRadius(16)
        }
    }
}

struct DetailRow: View {
    let label: String
    let value: String

    var body: some View {
        HStack {
            Text(label)
                .font(.system(size: 14))
                .foregroundColor(.white.opacity(0.6))

            Spacer()

            Text(value)
                .font(.system(size: 14, weight: .semibold))
                .foregroundColor(.white)
        }
    }
}

// MARK: - Payment Error

struct PaymentErrorView: View {
    let errorMessage: String

    var body: some View {
        VStack(spacing: 24) {
            // Error icon
            ZStack {
                Circle()
                    .fill(Color.red.opacity(0.2))
                    .frame(width: 100, height: 100)

                Image(systemName: "xmark.circle.fill")
                    .font(.system(size: 60))
                    .foregroundColor(.red)
            }

            VStack(spacing: 8) {
                Text("Payment Failed")
                    .font(.system(size: 24, weight: .bold))
                    .foregroundColor(.white)

                Text(errorMessage)
                    .font(.system(size: 14))
                    .foregroundColor(.white.opacity(0.6))
                    .multilineTextAlignment(.center)
                    .padding(.horizontal)
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }
}

// MARK: - Lottie Success Animation (Placeholder)

struct LottieSuccessView: View {
    @State private var checkmarkScale: CGFloat = 0

    var body: some View {
        ZStack {
            Circle()
                .fill(Color.green.opacity(0.2))
                .frame(width: 100, height: 100)

            Image(systemName: "checkmark")
                .font(.system(size: 50, weight: .bold))
                .foregroundColor(.green)
                .scaleEffect(checkmarkScale)
        }
        .onAppear {
            withAnimation(.spring(response: 0.6, dampingFraction: 0.6)) {
                checkmarkScale = 1.0
            }
        }
    }
}
