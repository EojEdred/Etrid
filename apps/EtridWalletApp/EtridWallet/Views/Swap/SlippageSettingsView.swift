//
//  SlippageSettingsView.swift
//  EtridWallet
//
//  Slippage tolerance settings for swaps
//

import SwiftUI

struct SlippageSettingsView: View {
    @Environment(\.dismiss) private var dismiss
    @Binding var settings: SlippageSettings
    @State private var customValue: String = ""
    @State private var showCustomInput = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 24) {
                // Header
                VStack(spacing: 8) {
                    Text("Max Slippage")
                        .font(.headline)

                    Text("Your transaction will revert if the price changes unfavorably by more than this percentage.")
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .multilineTextAlignment(.center)
                }
                .padding()

                // Preset Options
                VStack(spacing: 12) {
                    ForEach(SlippageSettings.presets, id: \.self) { preset in
                        Button(action: {
                            settings.tolerance = preset
                            settings.isCustom = false
                            showCustomInput = false
                        }) {
                            HStack {
                                Text(String(format: "%.1f%%", preset))
                                    .font(.headline)
                                    .foregroundColor(.primary)

                                Spacer()

                                if !settings.isCustom && settings.tolerance == preset {
                                    Image(systemName: "checkmark.circle.fill")
                                        .foregroundColor(.etridBlue)
                                }
                            }
                            .padding()
                            .background(
                                RoundedRectangle(cornerRadius: 12)
                                    .strokeBorder(
                                        !settings.isCustom && settings.tolerance == preset ? Color.etridBlue : Color(.systemGray4),
                                        lineWidth: 2
                                    )
                            )
                        }
                    }

                    // Custom Option
                    VStack(spacing: 12) {
                        Button(action: {
                            showCustomInput = true
                            settings.isCustom = true
                            customValue = String(format: "%.1f", settings.tolerance)
                        }) {
                            HStack {
                                Text("Custom")
                                    .font(.headline)
                                    .foregroundColor(.primary)

                                Spacer()

                                if settings.isCustom {
                                    Text(settings.formattedTolerance)
                                        .font(.subheadline)
                                        .foregroundColor(.etridBlue)

                                    Image(systemName: "checkmark.circle.fill")
                                        .foregroundColor(.etridBlue)
                                }
                            }
                            .padding()
                            .background(
                                RoundedRectangle(cornerRadius: 12)
                                    .strokeBorder(
                                        settings.isCustom ? Color.etridBlue : Color(.systemGray4),
                                        lineWidth: 2
                                    )
                            )
                        }

                        if showCustomInput {
                            VStack(alignment: .leading, spacing: 8) {
                                HStack {
                                    TextField("Enter custom slippage", text: $customValue)
                                        .keyboardType(.decimalPad)
                                        .textFieldStyle(.plain)
                                        .padding()
                                        .background(Color(.systemGray6))
                                        .cornerRadius(8)

                                    Text("%")
                                        .foregroundColor(.secondary)
                                }

                                if let value = Double(customValue), (value < 0.1 || value > 50) {
                                    HStack {
                                        Image(systemName: "exclamationmark.triangle.fill")
                                            .foregroundColor(.etridOrange)
                                            .font(.caption)

                                        Text("Slippage should be between 0.1% and 50%")
                                            .font(.caption)
                                            .foregroundColor(.secondary)
                                    }
                                }

                                Button("Apply Custom Slippage") {
                                    if let value = Double(customValue), value >= 0.1 && value <= 50 {
                                        settings.tolerance = value
                                        settings.isCustom = true
                                    }
                                }
                                .font(.subheadline.bold())
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.etridBlue)
                                .foregroundColor(.white)
                                .cornerRadius(8)
                                .disabled(customValue.isEmpty || Double(customValue) == nil)
                            }
                            .padding()
                            .background(Color(.systemGray6).opacity(0.5))
                            .cornerRadius(12)
                        }
                    }
                }

                // Warning Messages
                VStack(spacing: 12) {
                    if settings.tolerance < 0.5 {
                        WarningBox(
                            icon: "exclamationmark.triangle.fill",
                            color: .etridOrange,
                            message: "Low slippage tolerance may cause transaction failure"
                        )
                    }

                    if settings.tolerance > 5 {
                        WarningBox(
                            icon: "exclamationmark.triangle.fill",
                            color: .etridRed,
                            message: "High slippage tolerance may result in unfavorable rates"
                        )
                    }
                }

                Spacer()

                // Done Button
                Button("Done") {
                    dismiss()
                }
                .font(.headline)
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.etridBlue)
                .foregroundColor(.white)
                .cornerRadius(12)
            }
            .padding()
            .navigationTitle("Slippage Settings")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

private struct WarningBox: View {
    let icon: String
    let color: Color
    let message: String

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: icon)
                .foregroundColor(color)

            Text(message)
                .font(.caption)
                .foregroundColor(.secondary)

            Spacer()
        }
        .padding()
        .background(color.opacity(0.1))
        .cornerRadius(8)
    }
}

#Preview {
    SlippageSettingsView(settings: .constant(.default))
}
