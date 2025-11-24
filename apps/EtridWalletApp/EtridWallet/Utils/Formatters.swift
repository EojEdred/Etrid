//
//  Formatters.swift
//  EtridWallet
//
//  Number and address formatting utilities
//

import Foundation

// MARK: - Number Formatter
struct NumberFormatters {
    static let currency: NumberFormatter = {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = "USD"
        formatter.maximumFractionDigits = 2
        formatter.minimumFractionDigits = 2
        return formatter
    }()

    static let crypto: NumberFormatter = {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 8
        formatter.minimumFractionDigits = 2
        formatter.groupingSeparator = ","
        return formatter
    }()

    static let percentage: NumberFormatter = {
        let formatter = NumberFormatter()
        formatter.numberStyle = .percent
        formatter.maximumFractionDigits = 2
        formatter.minimumFractionDigits = 2
        return formatter
    }()

    static let compact: NumberFormatter = {
        let formatter = NumberFormatter()
        formatter.numberStyle = .decimal
        formatter.maximumFractionDigits = 2
        formatter.minimumFractionDigits = 0
        return formatter
    }()
}

// MARK: - Address Formatter
struct AddressFormatter {
    /// Formats Ethereum address with checksum
    static func format(_ address: String, style: AddressStyle = .short) -> String {
        guard address.isValidEthereumAddress else { return address }

        switch style {
        case .short:
            return address.formatAddress(prefixLength: 6, suffixLength: 4)
        case .medium:
            return address.formatAddress(prefixLength: 10, suffixLength: 8)
        case .full:
            return address.toChecksumAddress
        }
    }

    enum AddressStyle {
        case short
        case medium
        case full
    }
}

// MARK: - Wei Converter
struct WeiConverter {
    /// Converts Wei to Ether
    static func weiToEther(_ wei: String) -> Double {
        guard let value = Double(wei) else { return 0 }
        return value / 1e18
    }

    /// Converts Ether to Wei
    static func etherToWei(_ ether: Double) -> String {
        let wei = ether * 1e18
        return String(format: "%.0f", wei)
    }

    /// Converts Wei to Gwei
    static func weiToGwei(_ wei: String) -> Double {
        guard let value = Double(wei) else { return 0 }
        return value / 1e9
    }

    /// Converts Gwei to Wei
    static func gweiToWei(_ gwei: Double) -> String {
        let wei = gwei * 1e9
        return String(format: "%.0f", wei)
    }

    /// Formats Wei as Ether string
    static func formatWeiAsEther(_ wei: String, decimals: Int = 6) -> String {
        let ether = weiToEther(wei)
        return String(format: "%.\(decimals)f", ether)
    }

    /// Formats Wei as Gwei string
    static func formatWeiAsGwei(_ wei: String, decimals: Int = 2) -> String {
        let gwei = weiToGwei(wei)
        return String(format: "%.\(decimals)f", gwei)
    }
}

// MARK: - Token Amount Formatter
struct TokenAmountFormatter {
    /// Formats token amount based on decimals
    static func format(_ amount: String, decimals: Int, symbol: String = "") -> String {
        guard let value = Double(amount) else { return "0 \(symbol)" }
        let divisor = pow(10.0, Double(decimals))
        let formatted = value / divisor

        let numberString: String
        if formatted < 0.000001 {
            numberString = "0"
        } else if formatted < 1 {
            numberString = String(format: "%.6f", formatted)
        } else if formatted < 1000 {
            numberString = String(format: "%.4f", formatted)
        } else {
            numberString = String(format: "%.2f", formatted)
        }

        return symbol.isEmpty ? numberString : "\(numberString) \(symbol)"
    }

    /// Parses user input to token amount with decimals
    static func parseInput(_ input: String, decimals: Int) -> String? {
        guard let value = Double(input) else { return nil }
        let multiplier = pow(10.0, Double(decimals))
        let amount = value * multiplier
        return String(format: "%.0f", amount)
    }
}

// MARK: - Gas Formatter
struct GasFormatter {
    /// Formats gas price in Gwei
    static func formatGasPrice(_ wei: String) -> String {
        let gwei = WeiConverter.weiToGwei(wei)
        return String(format: "%.2f Gwei", gwei)
    }

    /// Formats gas fee in ETH
    static func formatGasFee(gasLimit: String, gasPrice: String) -> String {
        guard let limit = UInt64(gasLimit),
              let price = UInt64(gasPrice) else {
            return "0 ETH"
        }

        let totalWei = limit * price
        let eth = Double(totalWei) / 1e18
        return String(format: "%.6f ETH", eth)
    }

    /// Formats gas fee in USD
    static func formatGasFeeUSD(gasLimit: String, gasPrice: String, ethPriceUSD: Double) -> String {
        guard let limit = UInt64(gasLimit),
              let price = UInt64(gasPrice) else {
            return "$0.00"
        }

        let totalWei = limit * price
        let eth = Double(totalWei) / 1e18
        let usd = eth * ethPriceUSD
        return String(format: "$%.2f", usd)
    }
}

// MARK: - Transaction Hash Formatter
struct TransactionHashFormatter {
    /// Formats transaction hash for display
    static func format(_ hash: String, style: HashStyle = .short) -> String {
        switch style {
        case .short:
            return hash.formatAddress(prefixLength: 6, suffixLength: 4)
        case .medium:
            return hash.formatAddress(prefixLength: 10, suffixLength: 8)
        case .full:
            return hash
        }
    }

    enum HashStyle {
        case short
        case medium
        case full
    }
}

// MARK: - Date Formatter
struct DateFormatters {
    static let short: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateStyle = .short
        formatter.timeStyle = .none
        return formatter
    }()

    static let medium: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateStyle = .medium
        formatter.timeStyle = .short
        return formatter
    }()

    static let long: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateStyle = .long
        formatter.timeStyle = .medium
        return formatter
    }()

    static let relative: RelativeDateTimeFormatter = {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .abbreviated
        return formatter
    }()

    static let iso8601: ISO8601DateFormatter = {
        let formatter = ISO8601DateFormatter()
        return formatter
    }()
}

// MARK: - Price Change Formatter
struct PriceChangeFormatter {
    /// Formats price change with color indicator
    static func format(_ change: Double) -> (text: String, isPositive: Bool) {
        let sign = change >= 0 ? "+" : ""
        let text = String(format: "%@%.2f%%", sign, change)
        return (text, change >= 0)
    }

    /// Formats price change as attributed string
    static func formatAttributed(_ change: Double) -> String {
        let (text, _) = format(change)
        return text
    }
}

// MARK: - Balance Formatter
struct BalanceFormatter {
    /// Formats balance with appropriate decimals based on value
    static func format(_ balance: Double, symbol: String = "") -> String {
        let formatted: String

        if balance < 0.000001 {
            formatted = "0"
        } else if balance < 0.01 {
            formatted = String(format: "%.6f", balance)
        } else if balance < 1 {
            formatted = String(format: "%.4f", balance)
        } else if balance < 1000 {
            formatted = String(format: "%.3f", balance)
        } else {
            formatted = String(format: "%.2f", balance)
        }

        return symbol.isEmpty ? formatted : "\(formatted) \(symbol)"
    }

    /// Formats large numbers with K, M, B suffixes
    static func formatCompact(_ value: Double) -> String {
        let absValue = abs(value)
        let sign = value < 0 ? "-" : ""

        if absValue < 1000 {
            return "\(sign)\(String(format: "%.2f", absValue))"
        } else if absValue < 1_000_000 {
            return "\(sign)\(String(format: "%.2fK", absValue / 1000))"
        } else if absValue < 1_000_000_000 {
            return "\(sign)\(String(format: "%.2fM", absValue / 1_000_000))"
        } else {
            return "\(sign)\(String(format: "%.2fB", absValue / 1_000_000_000))"
        }
    }
}

// MARK: - Network Fee Formatter
struct NetworkFeeFormatter {
    /// Formats network fee for display
    static func format(gasLimit: String, gasPrice: String, symbol: String = "ETH") -> String {
        let fee = GasFormatter.formatGasFee(gasLimit: gasLimit, gasPrice: gasPrice)
        return fee
    }

    /// Formats network fee with USD equivalent
    static func formatWithUSD(gasLimit: String, gasPrice: String, ethPrice: Double) -> String {
        let ethFee = GasFormatter.formatGasFee(gasLimit: gasLimit, gasPrice: gasPrice)
        let usdFee = GasFormatter.formatGasFeeUSD(gasLimit: gasLimit, gasPrice: gasPrice, ethPriceUSD: ethPrice)
        return "\(ethFee) (\(usdFee))"
    }
}
