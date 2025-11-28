//
//  Extensions.swift
//  EtridWallet
//
//  Essential Swift extensions
//

import Foundation
import SwiftUI
import CryptoKit
import CryptoSwift

// MARK: - String Extensions
extension String {
    /// Checks if string is a valid Ethereum address
    var isValidEthereumAddress: Bool {
        let pattern = "^0x[a-fA-F0-9]{40}$"
        let regex = try? NSRegularExpression(pattern: pattern)
        let range = NSRange(location: 0, length: self.utf16.count)
        return regex?.firstMatch(in: self, range: range) != nil
    }

    /// Checks if string is a valid transaction hash
    var isValidTransactionHash: Bool {
        let pattern = "^0x[a-fA-F0-9]{64}$"
        let regex = try? NSRegularExpression(pattern: pattern)
        let range = NSRange(location: 0, length: self.utf16.count)
        return regex?.firstMatch(in: self, range: range) != nil
    }

    /// Converts hex string to Data
    var hexToData: Data? {
        var hex = self
        if hex.hasPrefix("0x") {
            hex = String(hex.dropFirst(2))
        }

        guard hex.count % 2 == 0 else { return nil }

        var data = Data()
        var index = hex.startIndex

        while index < hex.endIndex {
            let nextIndex = hex.index(index, offsetBy: 2)
            let byteString = hex[index..<nextIndex]
            guard let byte = UInt8(byteString, radix: 16) else { return nil }
            data.append(byte)
            index = nextIndex
        }

        return data
    }

    /// Formats address with ellipsis in middle
    func formatAddress(prefixLength: Int = 6, suffixLength: Int = 4) -> String {
        guard self.count > prefixLength + suffixLength else { return self }
        let prefix = self.prefix(prefixLength)
        let suffix = self.suffix(suffixLength)
        return "\(prefix)...\(suffix)"
    }

    /// Adds 0x prefix if not present
    var addHexPrefix: String {
        hasPrefix("0x") ? self : "0x" + self
    }

    /// Removes 0x prefix if present
    var removeHexPrefix: String {
        hasPrefix("0x") ? String(dropFirst(2)) : self
    }

    /// Converts to checksum address (EIP-55)
    var toChecksumAddress: String {
        let address = self.removeHexPrefix.lowercased()
        guard address.count == 40 else { return self }

        // Use Keccak256 hash for EIP-55 checksum
        guard let addressData = address.data(using: .utf8) else { return self }
        let hash = addressData.sha3(.keccak256).toHexString()

        var checksumAddress = "0x"

        for (index, char) in address.enumerated() {
            let hashChar = hash[hash.index(hash.startIndex, offsetBy: index)]
            if let hashValue = Int(String(hashChar), radix: 16), hashValue >= 8 {
                checksumAddress += char.uppercased()
            } else {
                checksumAddress += String(char)
            }
        }

        return checksumAddress
    }

    /// Unicode NFKD normalization for BIP39
    func normalizeNFKD() -> String {
        return self.decomposedStringWithCanonicalMapping
    }
}

// MARK: - Data Extensions
extension Data {
    /// Converts Data to hex string
    var hexString: String {
        map { String(format: "%02x", $0) }.joined()
    }

    /// Converts Data to hex string with 0x prefix
    var hexStringWithPrefix: String {
        "0x" + hexString
    }

    /// Keccak256 hash using CryptoSwift
    var keccak256: Data {
        return self.sha3(.keccak256)
    }
}

// MARK: - Double Extensions
extension Double {
    /// Formats as currency
    func formatAsCurrency(currencyCode: String = "USD") -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = currencyCode
        formatter.maximumFractionDigits = 2
        return formatter.string(from: NSNumber(value: self)) ?? "$0.00"
    }

    /// Formats as crypto amount
    func formatAsCrypto(symbol: String = "ETH", decimals: Int = 6) -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = decimals
        formatter.minimumFractionDigits = 2
        let formatted = formatter.string(from: NSNumber(value: self)) ?? "0"
        return "\(formatted) \(symbol)"
    }

    /// Formats as percentage
    func formatAsPercentage(decimals: Int = 2) -> String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .percent
        formatter.maximumFractionDigits = decimals
        formatter.minimumFractionDigits = decimals
        return formatter.string(from: NSNumber(value: self / 100)) ?? "0%"
    }
}

// MARK: - Int Extensions
extension Int {
    /// Converts Wei to Ether string
    func weiToEther() -> String {
        let value = Double(self) / 1e18
        return String(format: "%.6f", value)
    }
}

// MARK: - Date Extensions
extension Date {
    /// Formats as relative time (e.g., "2 hours ago")
    var relativeTime: String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .full
        return formatter.localizedString(for: self, relativeTo: Date())
    }

    /// Formats as short date/time
    var shortDateTime: String {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .short
        return formatter.string(from: self)
    }

    /// Formats as full date/time
    var fullDateTime: String {
        let formatter = DateFormatter()
        formatter.dateStyle = .medium
        formatter.timeStyle = .medium
        return formatter.string(from: self)
    }
}

// MARK: - Color Extensions
extension Color {
    /// Initialize from hex string
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

    static let etridBlue = Color(hex: "1E88E5")
    static let etridGreen = Color(hex: "43A047")
    static let etridRed = Color(hex: "E53935")
    static let etridOrange = Color(hex: "FB8C00")
    static let etridPurple = Color(hex: "8E24AA")
}

// MARK: - View Extensions
extension View {
    /// Applies conditional modifier
    @ViewBuilder
    func `if`<Transform: View>(_ condition: Bool, transform: (Self) -> Transform) -> some View {
        if condition {
            transform(self)
        } else {
            self
        }
    }

    /// Adds border with rounded corners
    func roundedBorder(color: Color = .gray, width: CGFloat = 1, cornerRadius: CGFloat = 8) -> some View {
        self
            .overlay(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .stroke(color, lineWidth: width)
            )
    }

    /// Adds card-like appearance
    func cardStyle(padding: CGFloat = 16, cornerRadius: CGFloat = 12) -> some View {
        self
            .padding(padding)
            .background(Color(.systemBackground))
            .cornerRadius(cornerRadius)
            .shadow(color: Color.black.opacity(0.1), radius: 5, x: 0, y: 2)
    }
}

// MARK: - Array Extensions
extension Array where Element == String {
    /// Joins mnemonic words
    var mnemonicPhrase: String {
        self.joined(separator: " ")
    }
}

// MARK: - BigInt Helper
struct BigInt {
    static func fromHex(_ hex: String) -> UInt64? {
        let clean = hex.removeHexPrefix
        return UInt64(clean, radix: 16)
    }

    static func toHex(_ value: UInt64) -> String {
        String(format: "0x%x", value)
    }

    static func fromDecimal(_ decimal: String) -> UInt64? {
        UInt64(decimal)
    }
}

// MARK: - UserDefaults Extensions
extension UserDefaults {
    subscript<T: Codable>(key: String) -> T? {
        get {
            guard let data = data(forKey: key) else { return nil }
            return try? JSONDecoder().decode(T.self, from: data)
        }
        set {
            guard let newValue = newValue else {
                removeObject(forKey: key)
                return
            }
            let data = try? JSONEncoder().encode(newValue)
            set(data, forKey: key)
        }
    }
}

// MARK: - Notification Name Extensions
extension Notification.Name {
    static let walletCreated = Notification.Name("walletCreated")
    static let walletImported = Notification.Name("walletImported")
    static let networkChanged = Notification.Name("networkChanged")
    static let accountChanged = Notification.Name("accountChanged")
    static let transactionSent = Notification.Name("transactionSent")
    static let transactionConfirmed = Notification.Name("transactionConfirmed")
}
