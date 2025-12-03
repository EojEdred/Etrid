//
//  SharedComponents.swift
//  EtridWallet
//
//  Shared UI components used across multiple views
//

import SwiftUI

// MARK: - Shared Transaction Status
enum SharedTransactionStatus: String, Codable {
    case pending
    case confirmed
    case failed
    case cancelled
}

// MARK: - Empty Response for API calls
struct SharedEmptyResponse: Codable {}

// MARK: - Shared Loading View
struct SharedLoadingView: View {
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
        .padding(32)
        .background(Color(.systemBackground).opacity(0.9))
        .cornerRadius(16)
        .shadow(radius: 20)
    }
}

// MARK: - Shared Empty State View
struct SharedEmptyStateView: View {
    let icon: String
    let title: String
    let message: String

    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: icon)
                .font(.system(size: 48))
                .foregroundColor(.secondary)

            Text(title)
                .font(.headline)
                .foregroundColor(.primary)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 40)
    }
}

// MARK: - Shared Error Banner
struct SharedErrorBanner: View {
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

// MARK: - Shared Transaction Row
struct SharedTransactionRow: View {
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
                .fontWeight(.medium)
                .foregroundColor(.primary)
                .lineLimit(1)
        }
    }
}

// MARK: - Shared Stat Card
struct SharedStatCard: View {
    let title: String
    let value: String
    let icon: String
    let color: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Image(systemName: icon)
                    .foregroundColor(color)
                Text(title)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Text(value)
                .font(.title2)
                .fontWeight(.bold)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
}

// MARK: - Tax Report Model
struct TaxReport: Codable {
    let year: Int
    let totalGains: Double
    let totalLosses: Double
    let shortTermGains: Double
    let longTermGains: Double
    let transactions: [TaxTransaction]
}

struct TaxTransaction: Codable {
    let date: Date
    let type: String
    let asset: String
    let amount: Double
    let costBasis: Double
    let proceeds: Double
    let gainLoss: Double
}

// MARK: - Insight Model
struct Insight: Identifiable, Codable {
    let id: String
    let type: String
    let title: String
    let description: String
    let priority: String
    let action: String?
}
