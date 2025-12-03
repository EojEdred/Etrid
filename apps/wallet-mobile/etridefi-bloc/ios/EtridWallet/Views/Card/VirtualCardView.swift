//
//  VirtualCardView.swift
//  EtridWallet
//
//  3D Virtual card UI component with animations
//

import SwiftUI

struct VirtualCardView: View {
    let card: CryptoCard
    @State private var isFlipped = false
    @State private var showDetails = false
    @State private var dragOffset: CGSize = .zero
    @State private var rotation: Double = 0

    var body: some View {
        ZStack {
            // Card Front
            CardFrontView(card: card, showDetails: showDetails)
                .opacity(isFlipped ? 0 : 1)
                .rotation3DEffect(
                    .degrees(isFlipped ? 180 : 0),
                    axis: (x: 0, y: 1, z: 0)
                )

            // Card Back
            CardBackView(card: card)
                .opacity(isFlipped ? 1 : 0)
                .rotation3DEffect(
                    .degrees(isFlipped ? 0 : -180),
                    axis: (x: 0, y: 1, z: 0)
                )
        }
        .frame(width: 340, height: 214)
        .rotation3DEffect(
            .degrees(rotation),
            axis: (x: dragOffset.height, y: dragOffset.width, z: 0)
        )
        .gesture(
            DragGesture()
                .onChanged { value in
                    dragOffset = value.translation
                    rotation = sqrt(pow(dragOffset.width, 2) + pow(dragOffset.height, 2)) / 10
                }
                .onEnded { _ in
                    withAnimation(.spring(response: 0.6, dampingFraction: 0.7)) {
                        dragOffset = .zero
                        rotation = 0
                    }
                }
        )
        .onTapGesture {
            withAnimation(.spring(response: 0.6, dampingFraction: 0.8)) {
                isFlipped.toggle()
            }
        }
        .onLongPressGesture {
            withAnimation {
                showDetails.toggle()
            }
        }
    }
}

// MARK: - Card Front View

struct CardFrontView: View {
    let card: CryptoCard
    let showDetails: Bool

    var body: some View {
        ZStack {
            // Gradient background
            LinearGradient(
                gradient: Gradient(colors: cardGradient),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .cornerRadius(20)

            // Holographic effect
            HolographicOverlay()
                .cornerRadius(20)
                .opacity(0.3)

            VStack(alignment: .leading, spacing: 0) {
                // Header
                HStack {
                    Text("ETRID")
                        .font(.system(size: 20, weight: .bold, design: .rounded))
                        .foregroundColor(.white)

                    Spacer()

                    // Card type badge
                    HStack(spacing: 4) {
                        Image(systemName: "wave.3.right")
                            .font(.system(size: 12))
                        Text("CONTACTLESS")
                            .font(.system(size: 8, weight: .semibold))
                    }
                    .foregroundColor(.white.opacity(0.9))
                }
                .padding(.top, 20)
                .padding(.horizontal, 24)

                Spacer()

                // NFC chip
                Image(systemName: "wave.3.forward.circle.fill")
                    .font(.system(size: 40))
                    .foregroundColor(.white.opacity(0.8))
                    .padding(.leading, 24)

                Spacer()

                // Card number
                if showDetails {
                    Text(card.cardNumber)
                        .font(.system(size: 20, weight: .medium, design: .monospaced))
                        .foregroundColor(.white)
                        .tracking(2)
                        .padding(.horizontal, 24)
                } else {
                    Text(card.maskedCardNumber)
                        .font(.system(size: 20, weight: .medium, design: .monospaced))
                        .foregroundColor(.white)
                        .tracking(2)
                        .padding(.horizontal, 24)
                }

                Spacer()

                // Cardholder info
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("CARDHOLDER")
                            .font(.system(size: 8, weight: .medium))
                            .foregroundColor(.white.opacity(0.7))

                        Text(card.cardholderName.uppercased())
                            .font(.system(size: 14, weight: .semibold))
                            .foregroundColor(.white)
                    }

                    Spacer()

                    VStack(alignment: .trailing, spacing: 4) {
                        Text("EXPIRES")
                            .font(.system(size: 8, weight: .medium))
                            .foregroundColor(.white.opacity(0.7))

                        Text(card.expiryString)
                            .font(.system(size: 14, weight: .semibold, design: .monospaced))
                            .foregroundColor(.white)
                    }

                    // Card network logo
                    Image(systemName: "creditcard.fill")
                        .font(.system(size: 30))
                        .foregroundColor(.white.opacity(0.9))
                        .padding(.leading, 12)
                }
                .padding(.horizontal, 24)
                .padding(.bottom, 20)
            }

            // Status indicator
            if !card.isActive {
                VStack {
                    HStack {
                        Spacer()
                        CardStatusBadge(status: card.status)
                            .padding()
                    }
                    Spacer()
                }
            }
        }
        .shadow(color: .black.opacity(0.3), radius: 20, x: 0, y: 10)
    }

    private var cardGradient: [Color] {
        switch card.collateral.cryptoAsset {
        case "ETH":
            return [Color(hex: "627EEA"), Color(hex: "3C3C3D")]
        case "BTC":
            return [Color(hex: "F7931A"), Color(hex: "4D4D4D")]
        case "USDC":
            return [Color(hex: "2775CA"), Color(hex: "1E5A94")]
        default:
            return [Color(hex: "6B46C1"), Color(hex: "4C1D95")]
        }
    }
}

// MARK: - Card Back View

struct CardBackView: View {
    let card: CryptoCard

    var body: some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color(hex: "1F2937"), Color(hex: "111827")]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .cornerRadius(20)

            VStack(spacing: 0) {
                // Magnetic stripe
                Rectangle()
                    .fill(Color.black)
                    .frame(height: 50)
                    .padding(.top, 30)

                Spacer()

                // CVV section
                HStack {
                    Spacer()

                    VStack(alignment: .trailing, spacing: 8) {
                        Text("CVV")
                            .font(.system(size: 10, weight: .medium))
                            .foregroundColor(.white.opacity(0.7))

                        Text("•••")
                            .font(.system(size: 20, weight: .bold, design: .monospaced))
                            .foregroundColor(.black)
                            .padding(.horizontal, 16)
                            .padding(.vertical, 8)
                            .background(Color.white)
                            .cornerRadius(4)
                    }
                    .padding(.trailing, 30)
                }

                Spacer()

                // Back footer
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("CRYPTO-BACKED")
                            .font(.system(size: 8, weight: .semibold))
                            .foregroundColor(.white.opacity(0.6))

                        Text("Collateral: \(card.collateral.cryptoAsset)")
                            .font(.system(size: 10, weight: .medium))
                            .foregroundColor(.white.opacity(0.8))
                    }

                    Spacer()

                    Image(systemName: "lock.shield.fill")
                        .font(.system(size: 20))
                        .foregroundColor(.white.opacity(0.6))
                }
                .padding(.horizontal, 24)
                .padding(.bottom, 20)
            }
        }
        .shadow(color: .black.opacity(0.3), radius: 20, x: 0, y: 10)
    }
}

// MARK: - Status Badge

struct CardStatusBadge: View {
    let status: CardStatus

    var body: some View {
        HStack(spacing: 4) {
            Image(systemName: statusIcon)
                .font(.system(size: 10))
            Text(statusText)
                .font(.system(size: 10, weight: .semibold))
        }
        .foregroundColor(.white)
        .padding(.horizontal, 12)
        .padding(.vertical, 6)
        .background(statusColor)
        .cornerRadius(12)
    }

    private var statusIcon: String {
        switch status {
        case .frozen: return "snowflake"
        case .blocked: return "exclamationmark.triangle.fill"
        case .expired: return "clock.fill"
        case .pending: return "hourglass"
        default: return "checkmark.circle.fill"
        }
    }

    private var statusText: String {
        status.rawValue.uppercased()
    }

    private var statusColor: Color {
        switch status {
        case .active: return .green.opacity(0.9)
        case .frozen: return .blue.opacity(0.9)
        case .blocked: return .red.opacity(0.9)
        case .expired: return .gray.opacity(0.9)
        case .pending: return .orange.opacity(0.9)
        }
    }
}

// MARK: - Holographic Overlay

struct HolographicOverlay: View {
    @State private var gradientRotation: Double = 0

    var body: some View {
        LinearGradient(
            gradient: Gradient(colors: [
                .clear,
                Color.white.opacity(0.3),
                Color.cyan.opacity(0.3),
                Color.purple.opacity(0.3),
                .clear
            ]),
            startPoint: .topLeading,
            endPoint: .bottomTrailing
        )
        .rotationEffect(.degrees(gradientRotation))
        .onAppear {
            withAnimation(.linear(duration: 3).repeatForever(autoreverses: false)) {
                gradientRotation = 360
            }
        }
    }
}

// MARK: - Color Extension

extension Color {
    init(hex: String) {
        let hex = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
        var int: UInt64 = 0
        Scanner(string: hex).scanHexInt64(&int)
        let a, r, g, b: UInt64
        switch hex.count {
        case 3: // RGB (12-bit)
            (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
        case 6: // RGB (24-bit)
            (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
        case 8: // ARGB (32-bit)
            (a, r, g, b) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
        default:
            (a, r, g, b) = (1, 1, 1, 0)
        }

        self.init(
            .sRGB,
            red: Double(r) / 255,
            green: Double(g) / 255,
            blue:  Double(b) / 255,
            opacity: Double(a) / 255
        )
    }
}
