//
//  LoadingView.swift
//  EtridWallet
//
//  Loading state views
//

import SwiftUI

struct LoadingView: View {
    let message: String

    init(message: String = "Loading...") {
        self.message = message
    }

    var body: some View {
        VStack(spacing: 16) {
            ProgressView()
                .scaleEffect(1.5)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

struct InlineLoadingView: View {
    let message: String

    init(message: String = "Loading...") {
        self.message = message
    }

    var body: some View {
        HStack(spacing: 12) {
            ProgressView()

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
        }
        .padding()
    }
}

struct PullToRefreshHeader: View {
    let isRefreshing: Bool

    var body: some View {
        HStack(spacing: 12) {
            if isRefreshing {
                ProgressView()
            } else {
                Image(systemName: "arrow.clockwise")
                    .foregroundColor(.secondary)
            }

            Text(isRefreshing ? "Refreshing..." : "Pull to refresh")
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .padding()
    }
}

struct SkeletonView: View {
    @State private var isAnimating = false

    var body: some View {
        RoundedRectangle(cornerRadius: 8)
            .fill(
                LinearGradient(
                    colors: [
                        Color(.systemGray5),
                        Color(.systemGray6),
                        Color(.systemGray5)
                    ],
                    startPoint: isAnimating ? .leading : .trailing,
                    endPoint: isAnimating ? .trailing : .leading
                )
            )
            .onAppear {
                withAnimation(.easeInOut(duration: 1.5).repeatForever(autoreverses: true)) {
                    isAnimating.toggle()
                }
            }
    }
}

struct TransactionLoadingRow: View {
    var body: some View {
        HStack(spacing: 12) {
            SkeletonView()
                .frame(width: 40, height: 40)
                .clipShape(Circle())

            VStack(alignment: .leading, spacing: 8) {
                SkeletonView()
                    .frame(width: 120, height: 12)

                SkeletonView()
                    .frame(width: 80, height: 10)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 8) {
                SkeletonView()
                    .frame(width: 60, height: 12)

                SkeletonView()
                    .frame(width: 40, height: 10)
            }
        }
        .padding(.vertical, 8)
    }
}

#Preview {
    VStack(spacing: 20) {
        LoadingView()

        Divider()

        InlineLoadingView()

        Divider()

        TransactionLoadingRow()
            .padding()
    }
}
