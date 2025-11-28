//
//  DefaultTokens.swift
//  EtridWallet
//
//  Default token lists for supported networks
//

import Foundation

struct DefaultTokens {
    // MARK: - Ethereum Tokens
    static let ethereumTokens: [Token] = [
        Token(
            symbol: "USDT",
            name: "Tether USD",
            contractAddress: "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            decimals: 6,
            logoURL: "https://assets.coingecko.com/coins/images/325/small/Tether.png"
        ),
        Token(
            symbol: "USDC",
            name: "USD Coin",
            contractAddress: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
            decimals: 6,
            logoURL: "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png"
        ),
        Token(
            symbol: "DAI",
            name: "Dai Stablecoin",
            contractAddress: "0x6B175474E89094C44Da98b954EedeAC495271d0F",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/9956/small/dai-multi-collateral-mcd.png"
        ),
        Token(
            symbol: "WETH",
            name: "Wrapped Ether",
            contractAddress: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/2518/small/weth.png"
        ),
        Token(
            symbol: "WBTC",
            name: "Wrapped Bitcoin",
            contractAddress: "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
            decimals: 8,
            logoURL: "https://assets.coingecko.com/coins/images/7598/small/wrapped_bitcoin_wbtc.png"
        ),
        Token(
            symbol: "UNI",
            name: "Uniswap",
            contractAddress: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/12504/small/uni.jpg"
        ),
        Token(
            symbol: "LINK",
            name: "Chainlink",
            contractAddress: "0x514910771AF9Ca656af840dff83E8264EcF986CA",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/877/small/chainlink-new-logo.png"
        )
    ]

    // MARK: - Polygon Tokens
    static let polygonTokens: [Token] = [
        Token(
            symbol: "USDT",
            name: "Tether USD",
            contractAddress: "0xc2132D05D31c914a87C6611C10748AEb04B58e8F",
            decimals: 6,
            logoURL: "https://assets.coingecko.com/coins/images/325/small/Tether.png"
        ),
        Token(
            symbol: "USDC",
            name: "USD Coin",
            contractAddress: "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
            decimals: 6,
            logoURL: "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png"
        ),
        Token(
            symbol: "DAI",
            name: "Dai Stablecoin",
            contractAddress: "0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/9956/small/dai-multi-collateral-mcd.png"
        ),
        Token(
            symbol: "WETH",
            name: "Wrapped Ether",
            contractAddress: "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/2518/small/weth.png"
        ),
        Token(
            symbol: "WMATIC",
            name: "Wrapped Matic",
            contractAddress: "0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/4713/small/matic-token-icon.png"
        )
    ]

    // MARK: - BSC Tokens
    static let bscTokens: [Token] = [
        Token(
            symbol: "USDT",
            name: "Tether USD",
            contractAddress: "0x55d398326f99059fF775485246999027B3197955",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/325/small/Tether.png"
        ),
        Token(
            symbol: "USDC",
            name: "USD Coin",
            contractAddress: "0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png"
        ),
        Token(
            symbol: "BUSD",
            name: "Binance USD",
            contractAddress: "0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/9576/small/BUSD.png"
        ),
        Token(
            symbol: "WBNB",
            name: "Wrapped BNB",
            contractAddress: "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/12591/small/binance-coin-logo.png"
        ),
        Token(
            symbol: "CAKE",
            name: "PancakeSwap",
            contractAddress: "0x0E09FaBB73Bd3Ade0a17ECC321fD13a19e81cE82",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/12632/small/pancakeswap-cake-logo.png"
        )
    ]

    // MARK: - Arbitrum Tokens
    static let arbitrumTokens: [Token] = [
        Token(
            symbol: "USDT",
            name: "Tether USD",
            contractAddress: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9",
            decimals: 6,
            logoURL: "https://assets.coingecko.com/coins/images/325/small/Tether.png"
        ),
        Token(
            symbol: "USDC",
            name: "USD Coin",
            contractAddress: "0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8",
            decimals: 6,
            logoURL: "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png"
        ),
        Token(
            symbol: "DAI",
            name: "Dai Stablecoin",
            contractAddress: "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/9956/small/dai-multi-collateral-mcd.png"
        ),
        Token(
            symbol: "WETH",
            name: "Wrapped Ether",
            contractAddress: "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1",
            decimals: 18,
            logoURL: "https://assets.coingecko.com/coins/images/2518/small/weth.png"
        )
    ]

    // MARK: - Helper Functions
    static func tokens(for network: Network) -> [Token] {
        switch network.chainId {
        case 1, 5, 11155111: // Ethereum mainnet and testnets
            return ethereumTokens
        case 137, 80001: // Polygon mainnet and Mumbai
            return polygonTokens
        case 56, 97: // BSC mainnet and testnet
            return bscTokens
        case 42161: // Arbitrum
            return arbitrumTokens
        case 10: // Optimism
            return [] // Add Optimism tokens as needed
        case 43114: // Avalanche
            return [] // Add Avalanche tokens as needed
        default:
            return []
        }
    }

    static func nativeToken(for network: Network) -> Token {
        Token(
            symbol: network.symbol,
            name: network.name,
            contractAddress: nil,
            balance: "0",
            decimals: 18,
            logoURL: nil,
            isNative: true
        )
    }

    // MARK: - Popular Tokens by Category
    static let stablecoins = ["USDT", "USDC", "DAI", "BUSD"]
    static let wrappedTokens = ["WETH", "WBTC", "WBNB", "WMATIC"]
    static let defiTokens = ["UNI", "AAVE", "COMP", "SUSHI", "CAKE"]

    static func isStablecoin(_ symbol: String) -> Bool {
        stablecoins.contains(symbol.uppercased())
    }

    static func isWrappedToken(_ symbol: String) -> Bool {
        wrappedTokens.contains(symbol.uppercased())
    }

    static func isDeFiToken(_ symbol: String) -> Bool {
        defiTokens.contains(symbol.uppercased())
    }
}

// MARK: - ERC20 ABI Helpers
struct ERC20ABI {
    static let balanceOfSignature = "0x70a08231" // balanceOf(address)
    static let transferSignature = "0xa9059cbb" // transfer(address,uint256)
    static let approveSignature = "0x095ea7b3" // approve(address,uint256)
    static let allowanceSignature = "0xdd62ed3e" // allowance(address,address)
    static let totalSupplySignature = "0x18160ddd" // totalSupply()
    static let nameSignature = "0x06fdde03" // name()
    static let symbolSignature = "0x95d89b41" // symbol()
    static let decimalsSignature = "0x313ce567" // decimals()

    static func encodeBalanceOf(address: String) -> String {
        let cleanAddress = address.removeHexPrefix.lowercased()
        let paddedAddress = String(repeating: "0", count: 64 - cleanAddress.count) + cleanAddress
        return balanceOfSignature + paddedAddress
    }

    static func encodeTransfer(to: String, amount: String) -> String {
        let cleanTo = to.removeHexPrefix.lowercased()
        let paddedTo = String(repeating: "0", count: 64 - cleanTo.count) + cleanTo

        let cleanAmount = amount.removeHexPrefix.lowercased()
        let paddedAmount = String(repeating: "0", count: 64 - cleanAmount.count) + cleanAmount

        return transferSignature + paddedTo + paddedAmount
    }

    static func decodeBalance(data: String) -> String? {
        guard data.count >= 66 else { return nil }
        let hex = data.removeHexPrefix
        return "0x" + hex
    }
}
