//
//  NetworkManager.swift
//  EtridWallet
//
//  Network switching and management
//

import Foundation
import Combine

@MainActor
class NetworkManager: ObservableObject {
    // MARK: - Published Properties
    @Published var selectedNetwork: Network
    @Published var availableNetworks: [Network]
    @Published var customNetworks: [Network] = []
    @Published var isTestnetEnabled = false

    // MARK: - Private Properties
    private var cancellables = Set<AnyCancellable>()

    // MARK: - Singleton
    static let shared = NetworkManager()

    init(selectedNetwork: Network = .ethereum) {
        self.selectedNetwork = selectedNetwork
        self.availableNetworks = Networks.allMainnets
        loadCustomNetworks()
        loadSettings()
    }

    // MARK: - Network Selection
    /// Switches to a different network
    /// - Parameter network: Network to switch to
    func switchNetwork(_ network: Network) {
        selectedNetwork = network
        saveSelectedNetwork()
        NotificationCenter.default.post(name: .networkDidChange, object: network)
    }

    /// Switches network by chain ID
    /// - Parameter chainId: Chain ID to switch to
    func switchNetwork(chainId: Int) {
        guard let network = Networks.network(for: chainId) else {
            ErrorLogger.logWarning("Network with chainId \(chainId) not found")
            return
        }
        switchNetwork(network)
    }

    /// Switches network by name
    /// - Parameter name: Network name
    func switchNetwork(name: String) {
        guard let network = Networks.network(for: name) else {
            ErrorLogger.logWarning("Network '\(name)' not found")
            return
        }
        switchNetwork(network)
    }

    // MARK: - Custom Networks
    /// Adds a custom network
    /// - Parameter network: Custom network to add
    func addCustomNetwork(_ network: Network) {
        customNetworks.append(network)
        updateAvailableNetworks()
        saveCustomNetworks()
    }

    /// Removes a custom network
    /// - Parameter network: Network to remove
    func removeCustomNetwork(_ network: Network) {
        customNetworks.removeAll { $0.chainId == network.chainId }
        updateAvailableNetworks()
        saveCustomNetworks()
    }

    /// Updates a custom network
    /// - Parameters:
    ///   - old: Old network
    ///   - new: New network configuration
    func updateCustomNetwork(_ old: Network, with new: Network) {
        if let index = customNetworks.firstIndex(where: { $0.chainId == old.chainId }) {
            customNetworks[index] = new
            updateAvailableNetworks()
            saveCustomNetworks()
        }
    }

    // MARK: - Testnet Support
    /// Toggles testnet visibility
    /// - Parameter enabled: Whether testnets should be shown
    func toggleTestnets(_ enabled: Bool) {
        isTestnetEnabled = enabled
        updateAvailableNetworks()
        saveSettings()
    }

    private func updateAvailableNetworks() {
        var networks = Networks.allMainnets + customNetworks.filter { !$0.isTestnet }

        if isTestnetEnabled {
            networks += Networks.allTestnets + customNetworks.filter { $0.isTestnet }
        }

        availableNetworks = networks
    }

    // MARK: - Network Info
    /// Gets current network info
    var currentNetworkInfo: NetworkInfo {
        NetworkInfo(
            network: selectedNetwork,
            blockTime: selectedNetwork.blockTime,
            supportsEIP1559: selectedNetwork.supportsEIP1559,
            isTestnet: selectedNetwork.isTestnet
        )
    }

    /// Checks if network is connected
    /// - Parameter network: Network to check
    /// - Returns: Connection status
    func isNetworkConnected(_ network: Network) async -> Bool {
        // Try to make a simple RPC call
        do {
            let rpc = RPCProvider(url: network.rpcURL)
            _ = try await rpc.request(method: "net_version", params: []) as String
            return true
        } catch {
            return false
        }
    }

    /// Gets network latency
    /// - Parameter network: Network to check
    /// - Returns: Latency in milliseconds
    func getNetworkLatency(_ network: Network) async -> TimeInterval? {
        let start = Date()
        let isConnected = await isNetworkConnected(network)
        let latency = Date().timeIntervalSince(start)

        return isConnected ? latency * 1000 : nil // Convert to ms
    }

    // MARK: - Popular Networks
    var popularNetworks: [Network] {
        [.ethereum, .polygon, .bsc, .arbitrum, .optimism]
    }

    var erc20Networks: [Network] {
        availableNetworks.filter { $0.isEthereumBased }
    }

    // MARK: - Persistence
    private func saveSelectedNetwork() {
        UserDefaults.standard.set(selectedNetwork.chainId, forKey: "selectedNetworkChainId")
    }

    private func loadSelectedNetwork() {
        let chainId = UserDefaults.standard.integer(forKey: "selectedNetworkChainId")
        if chainId != 0, let network = Networks.network(for: chainId) {
            selectedNetwork = network
        }
    }

    private func saveCustomNetworks() {
        if let encoded = try? JSONEncoder().encode(customNetworks) {
            UserDefaults.standard.set(encoded, forKey: "customNetworks")
        }
    }

    private func loadCustomNetworks() {
        if let data = UserDefaults.standard.data(forKey: "customNetworks"),
           let networks = try? JSONDecoder().decode([Network].self, from: data) {
            customNetworks = networks
        }
    }

    private func saveSettings() {
        UserDefaults.standard.set(isTestnetEnabled, forKey: "isTestnetEnabled")
    }

    private func loadSettings() {
        isTestnetEnabled = UserDefaults.standard.bool(forKey: "isTestnetEnabled")
        loadSelectedNetwork()
        updateAvailableNetworks()
    }

    // MARK: - Network Validation
    /// Validates network configuration
    /// - Parameter network: Network to validate
    /// - Returns: Validation result
    func validateNetwork(_ network: Network) -> NetworkValidationResult {
        var issues: [String] = []

        // Check RPC URL
        if let url = URL(string: network.rpcURL) {
            if url.scheme != "http" && url.scheme != "https" {
                issues.append("Invalid RPC URL")
            }
        } else {
            issues.append("Invalid RPC URL")
        }

        // Check explorer URL
        if let explorerURL = URL(string: network.explorerURL),
           explorerURL.scheme != "http" && explorerURL.scheme != "https" {
            issues.append("Invalid explorer URL")
        }

        // Check chain ID
        if network.chainId <= 0 {
            issues.append("Invalid chain ID")
        }

        return NetworkValidationResult(
            isValid: issues.isEmpty,
            issues: issues
        )
    }
}

// MARK: - Supporting Types
struct NetworkInfo {
    let network: Network
    let blockTime: TimeInterval
    let supportsEIP1559: Bool
    let isTestnet: Bool

    var confirmationTime: TimeInterval {
        blockTime * 12 // ~12 confirmations
    }
}

struct NetworkValidationResult {
    let isValid: Bool
    let issues: [String]

    var errorMessage: String? {
        issues.isEmpty ? nil : issues.joined(separator: "\n")
    }
}

// MARK: - Notifications
extension Notification.Name {
    static let networkDidChange = Notification.Name("networkDidChange")
}
