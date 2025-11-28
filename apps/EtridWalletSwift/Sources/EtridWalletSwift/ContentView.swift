import SwiftUI

struct ContentView: View {
    @EnvironmentObject var walletManager: WalletManager
    @State private var showingScanner = false
    
    var body: some View {
        NavigationView {
            VStack(spacing: 20) {
                // Wallet Header
                Text("Ëtrid Wallet")
                    .font(.largeTitle)
                    .fontWeight(.bold)
                
                // Balance Display
                VStack {
                    Text("Total Balance")
                        .font(.headline)
                        .foregroundColor(.secondary)
                    
                    Text("$0.00")
                        .font(.system(size: 48, weight: .bold))
                }
                .padding()
                .background(Color.blue.opacity(0.1))
                .cornerRadius(16)
                
                Spacer()
                
                // Action Buttons
                HStack(spacing: 20) {
                    ActionButton(title: "Send", icon: "arrow.up.circle.fill") {
                        // Send action
                    }
                    
                    ActionButton(title: "Receive", icon: "arrow.down.circle.fill") {
                        // Receive action
                    }
                    
                    ActionButton(title: "Scan", icon: "qrcode.viewfinder") {
                        showingScanner = true
                    }
                }
                .padding()
            }
            .padding()
            .navigationBarTitleDisplayMode(.inline)
        }
        .sheet(isPresented: $showingScanner) {
            QRScannerView()
        }
    }
}

struct ActionButton: View {
    let title: String
    let icon: String
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            VStack {
                Image(systemName: icon)
                    .font(.system(size: 32))
                Text(title)
                    .font(.caption)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color.blue)
            .foregroundColor(.white)
            .cornerRadius(12)
        }
    }
}
