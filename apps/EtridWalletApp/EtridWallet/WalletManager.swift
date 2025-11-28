import Foundation
import Combine
import LocalAuthentication

class WalletManager: ObservableObject {
    @Published var isUnlocked = false
    @Published var accounts: [WalletAccount] = []
    @Published var selectedNetwork: Network = .ethereum
    
    private let keychain = KeychainManager()
    
    init() {
        loadAccounts()
    }
    
    // MARK: - Biometric Authentication
    func authenticateWithBiometrics() async throws {
        let context = LAContext()
        var error: NSError?
        
        guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
            throw WalletError.biometricsNotAvailable
        }
        
        let reason = "Unlock your Ëtrid Wallet"
        let success = try await context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, localizedReason: reason)
        
        await MainActor.run {
            self.isUnlocked = success
        }
    }
    
    // MARK: - Account Management
    func createNewAccount(name: String) throws -> WalletAccount {
        let privateKey = try generatePrivateKey()
        let address = try deriveAddress(from: privateKey)
        
        let account = WalletAccount(
            id: UUID(),
            name: name,
            address: address,
            balance: "0",
            tokens: [],
            createdAt: Date(),
            isHD: false,
            derivationPath: nil
        )
        
        // Store private key securely in Keychain
        try keychain.store(privateKey, for: address)
        
        accounts.append(account)
        return account
    }
    
    private func loadAccounts() {
        // Load saved accounts from persistent storage
    }
    
    private func generatePrivateKey() throws -> Data {
        var bytes = [UInt8](repeating: 0, count: 32)
        let status = SecRandomCopyBytes(kSecRandomDefault, bytes.count, &bytes)
        guard status == errSecSuccess else {
            throw WalletError.keyGenerationFailed
        }
        return Data(bytes)
    }
    
    private func deriveAddress(from privateKey: Data) throws -> String {
        // Implement address derivation logic
        // This would use cryptographic libraries for real implementation
        return "0x" + privateKey.prefix(20).map { String(format: "%02x", $0) }.joined()
    }
}
