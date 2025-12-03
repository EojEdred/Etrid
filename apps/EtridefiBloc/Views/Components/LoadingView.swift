//
//  LoadingView.swift
//  EtridWallet
//
//  Created on 2025-01-22.
//  Copyright © 2025 Ëtrid. All rights reserved.
//

import SwiftUI

// MARK: - Loading View

/// Beautiful loading state with optional message
struct LoadingView: View {
    let message: String?
    var style: LoadingStyle = .spinner

    enum LoadingStyle {
        case spinner
        case dots
        case pulse
    }

    init(message: String? = nil, style: LoadingStyle = .spinner) {
        self.message = message
        self.style = style
    }

    var body: some View {
        VStack(spacing: 20) {
            switch style {
            case .spinner:
                SpinnerView()
            case .dots:
                DotsLoadingView()
            case .pulse:
                PulseLoadingView()
            }

            if let message = message {
                Text(message)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(.systemBackground))
    }
}

// MARK: - Spinner View

struct SpinnerView: View {
    @State private var isAnimating = false

    var body: some View {
        Circle()
            .trim(from: 0, to: 0.7)
            .stroke(
                LinearGradient(
                    colors: [.blue, .purple],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                ),
                style: StrokeStyle(lineWidth: 4, lineCap: .round)
            )
            .frame(width: 50, height: 50)
            .rotationEffect(Angle(degrees: isAnimating ? 360 : 0))
            .animation(
                .linear(duration: 1)
                .repeatForever(autoreverses: false),
                value: isAnimating
            )
            .onAppear {
                isAnimating = true
            }
    }
}

// MARK: - Dots Loading View

struct DotsLoadingView: View {
    @State private var activeIndex = 0

    var body: some View {
        HStack(spacing: 8) {
            ForEach(0..<3) { index in
                Circle()
                    .fill(Color.blue)
                    .frame(width: 12, height: 12)
                    .scaleEffect(activeIndex == index ? 1.0 : 0.5)
                    .opacity(activeIndex == index ? 1.0 : 0.3)
            }
        }
        .onAppear {
            withAnimation(.easeInOut(duration: 0.6).repeatForever()) {
                activeIndex = (activeIndex + 1) % 3
            }
        }
    }
}

// MARK: - Pulse Loading View

struct PulseLoadingView: View {
    @State private var isPulsing = false

    var body: some View {
        ZStack {
            ForEach(0..<3) { index in
                Circle()
                    .stroke(Color.blue.opacity(0.3), lineWidth: 2)
                    .frame(width: 50, height: 50)
                    .scaleEffect(isPulsing ? 1.5 : 0.5)
                    .opacity(isPulsing ? 0 : 1)
                    .animation(
                        .easeInOut(duration: 1.5)
                        .repeatForever(autoreverses: false)
                        .delay(Double(index) * 0.2),
                        value: isPulsing
                    )
            }
        }
        .onAppear {
            isPulsing = true
        }
    }
}

// MARK: - Skeleton View

/// Skeleton placeholder for loading content
struct SkeletonView: View {
    @State private var isAnimating = false
    var cornerRadius: CGFloat = 8

    var body: some View {
        RoundedRectangle(cornerRadius: cornerRadius)
            .fill(Color.gray.opacity(0.2))
            .overlay(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .fill(
                        LinearGradient(
                            colors: [.clear, .white.opacity(0.4), .clear],
                            startPoint: .leading,
                            endPoint: .trailing
                        )
                    )
                    .offset(x: isAnimating ? 400 : -400)
            )
            .clipShape(RoundedRectangle(cornerRadius: cornerRadius))
            .onAppear {
                withAnimation(.linear(duration: 1.5).repeatForever(autoreverses: false)) {
                    isAnimating = true
                }
            }
    }
}

// MARK: - Skeleton List Item

struct SkeletonListItem: View {
    var body: some View {
        HStack(spacing: 12) {
            // Avatar
            SkeletonView()
                .frame(width: 50, height: 50)
                .cornerRadius(25)

            VStack(alignment: .leading, spacing: 8) {
                // Title
                SkeletonView()
                    .frame(height: 16)
                    .frame(maxWidth: .infinity)

                // Subtitle
                SkeletonView()
                    .frame(height: 12)
                    .frame(maxWidth: 200)
            }
        }
        .padding()
    }
}

// MARK: - Skeleton Card

struct SkeletonCard: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Header
            HStack {
                SkeletonView()
                    .frame(width: 100, height: 20)

                Spacer()

                SkeletonView()
                    .frame(width: 60, height: 20)
            }

            // Main content
            SkeletonView()
                .frame(height: 80)

            // Footer
            HStack {
                SkeletonView()
                    .frame(width: 80, height: 16)

                Spacer()

                SkeletonView()
                    .frame(width: 80, height: 16)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .cornerRadius(12)
        .shadow(radius: 2)
    }
}

// MARK: - Pull to Refresh

struct PullToRefreshView<Content: View>: View {
    let content: Content
    let onRefresh: () async -> Void

    @State private var isRefreshing = false

    init(onRefresh: @escaping () async -> Void, @ViewBuilder content: () -> Content) {
        self.onRefresh = onRefresh
        self.content = content()
    }

    var body: some View {
        ScrollView {
            VStack {
                if isRefreshing {
                    ProgressView()
                        .padding()
                }

                content
            }
        }
        .refreshable {
            isRefreshing = true
            await onRefresh()
            isRefreshing = false
        }
    }
}

// MARK: - Empty State

struct EmptyStateView: View {
    let icon: String
    let title: String
    let message: String
    var actionTitle: String?
    var action: (() -> Void)?

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: icon)
                .font(.system(size: 60))
                .foregroundColor(.gray)

            VStack(spacing: 8) {
                Text(title)
                    .font(.title2)
                    .fontWeight(.semibold)

                Text(message)
                    .font(.body)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal)
            }

            if let actionTitle = actionTitle, let action = action {
                Button(action: action) {
                    Text(actionTitle)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                        .padding(.horizontal, 24)
                        .padding(.vertical, 12)
                        .background(Color.blue)
                        .cornerRadius(10)
                }
                .padding(.top, 8)
            }
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(Color(.systemBackground))
    }
}

// MARK: - Loading Overlay

struct LoadingOverlay: View {
    let message: String?
    @Binding var isShowing: Bool

    var body: some View {
        if isShowing {
            ZStack {
                Color.black.opacity(0.4)
                    .ignoresSafeArea()

                VStack(spacing: 20) {
                    ProgressView()
                        .scaleEffect(1.5)
                        .progressViewStyle(CircularProgressViewStyle(tint: .white))

                    if let message = message {
                        Text(message)
                            .font(.subheadline)
                            .foregroundColor(.white)
                    }
                }
                .padding(30)
                .background(
                    RoundedRectangle(cornerRadius: 12)
                        .fill(Color(.systemGray6))
                )
            }
            .transition(.opacity)
        }
    }
}

// MARK: - Progress Bar

struct ProgressBarView: View {
    @Binding var progress: Double
    var color: Color = .blue
    var height: CGFloat = 8

    var body: some View {
        GeometryReader { geometry in
            ZStack(alignment: .leading) {
                RoundedRectangle(cornerRadius: height / 2)
                    .fill(Color.gray.opacity(0.2))
                    .frame(height: height)

                RoundedRectangle(cornerRadius: height / 2)
                    .fill(color)
                    .frame(width: geometry.size.width * CGFloat(min(progress, 1.0)), height: height)
                    .animation(.linear, value: progress)
            }
        }
        .frame(height: height)
    }
}

// MARK: - Inline Loading

struct InlineLoadingView: View {
    let message: String?

    var body: some View {
        HStack(spacing: 12) {
            ProgressView()
                .scaleEffect(0.8)

            if let message = message {
                Text(message)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }
        }
        .padding()
    }
}

// MARK: - Shimmer Effect

struct ShimmerEffect: ViewModifier {
    @State private var phase: CGFloat = 0

    func body(content: Content) -> some View {
        content
            .overlay(
                LinearGradient(
                    gradient: Gradient(colors: [
                        .clear,
                        .white.opacity(0.3),
                        .clear
                    ]),
                    startPoint: .leading,
                    endPoint: .trailing
                )
                .offset(x: phase)
                .mask(content)
            )
            .onAppear {
                withAnimation(.linear(duration: 1.5).repeatForever(autoreverses: false)) {
                    phase = 400
                }
            }
    }
}

extension View {
    func shimmer() -> some View {
        modifier(ShimmerEffect())
    }
}

// MARK: - Preview

#if DEBUG
struct LoadingView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            LoadingView(message: "Loading your wallet...")
                .previewDisplayName("Spinner")

            LoadingView(message: "Processing...", style: .dots)
                .previewDisplayName("Dots")

            LoadingView(message: "Please wait...", style: .pulse)
                .previewDisplayName("Pulse")

            SkeletonListItem()
                .previewDisplayName("Skeleton List")

            SkeletonCard()
                .padding()
                .previewDisplayName("Skeleton Card")

            EmptyStateView(
                icon: "tray",
                title: "No Transactions",
                message: "Your transaction history will appear here",
                actionTitle: "Get Started",
                action: {}
            )
            .previewDisplayName("Empty State")
        }
    }
}
#endif
