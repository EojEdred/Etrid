//
//  ReceiveView.swift
//  EtridWallet
//
//  Receive screen with QR code
//

import SwiftUI
import CoreImage.CIFilterBuiltins

struct ReceiveView: View {
    @Environment(\.dismiss) private var dismiss
    let address: String
    let network: Network

    @State private var qrImage: Image?
    @State private var showCopiedAlert = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 30) {
                    // QR Code
                    VStack(spacing: 16) {
                        if let qrImage = qrImage {
                            qrImage
                                .interpolation(.none)
                                .resizable()
                                .frame(width: 250, height: 250)
                                .padding()
                                .background(Color.white)
                                .cornerRadius(20)
                                .shadow(radius: 10)
                        } else {
                            ProgressView()
                                .frame(width: 250, height: 250)
                        }

                        VStack(spacing: 4) {
                            Text("Scan to send to this address")
                                .font(.subheadline)
                                .foregroundColor(.secondary)

                            Text(network.name)
                                .font(.caption)
                                .fontWeight(.semibold)
                                .foregroundColor(.etridBlue)
                                .padding(.horizontal, 12)
                                .padding(.vertical, 4)
                                .background(Color.etridBlue.opacity(0.1))
                                .cornerRadius(8)
                        }
                    }

                    // Address
                    VStack(spacing: 12) {
                        Text("Your Address")
                            .font(.headline)

                        Text(address)
                            .font(.system(.body, design: .monospaced))
                            .multilineTextAlignment(.center)
                            .padding()
                            .background(Color(.systemGray6))
                            .cornerRadius(12)

                        Button(action: copyAddress) {
                            Label("Copy Address", systemImage: "doc.on.doc")
                                .font(.headline)
                                .foregroundColor(.white)
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.etridBlue)
                                .cornerRadius(16)
                        }
                    }

                    // Warning
                    HStack(spacing: 12) {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundColor(.orange)

                        Text(warningMessage)
                            .font(.caption)
                            .foregroundColor(.secondary)
                    }
                    .padding()
                    .background(Color.orange.opacity(0.1))
                    .cornerRadius(12)
                }
                .padding()
            }
            .navigationTitle("Receive")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
            .alert("Copied!", isPresented: $showCopiedAlert) {
                Button("OK") {}
            } message: {
                Text("Address copied to clipboard")
            }
        }
        .onAppear {
            generateQRCode()
        }
    }

    var warningMessage: String {
        switch network.name {
        case "Ethereum":
            return "Only send Ethereum and ERC-20 tokens to this address"
        case "Polygon":
            return "Only send Polygon (MATIC) and tokens on Polygon network to this address"
        case "BSC":
            return "Only send BNB and BEP-20 tokens on Binance Smart Chain to this address"
        case "Arbitrum":
            return "Only send tokens on Arbitrum network to this address"
        case "Optimism":
            return "Only send tokens on Optimism network to this address"
        default:
            return "Only send tokens on \(network.name) network to this address"
        }
    }

    func generateQRCode() {
        let context = CIContext()
        let filter = CIFilter.qrCodeGenerator()

        // Use EIP-681 format: ethereum:address@chainId
        let qrContent = "ethereum:\(address)@\(network.chainId)"
        filter.message = Data(qrContent.utf8)
        filter.correctionLevel = "M"

        if let outputImage = filter.outputImage,
           let cgImage = context.createCGImage(outputImage, from: outputImage.extent) {
            qrImage = Image(decorative: cgImage, scale: 1.0)
        }
    }

    func copyAddress() {
        UIPasteboard.general.string = address
        showCopiedAlert = true
    }
}

#Preview {
    ReceiveView(address: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", network: .ethereum)
}
