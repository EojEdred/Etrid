//
//  MarketDataExample.swift
//  EtridWalletSwift
//
//  Created by Ëtrid Team
//  Copyright © 2025 Ëtrid. All rights reserved.
//
//  Example usage of Phase 4 Market Data features
//

import Foundation

/// Example demonstrating Phase 4 Market Data features
@MainActor
class MarketDataExample {

    // MARK: - Services

    private let priceService: PriceService
    private let metadataService: TokenMetadataService
    private let discoveryService: TokenDiscoveryService
    private let marketManager: MarketDataManager

    // MARK: - Initialization

    init() {
        // Initialize services
        self.priceService = PriceService() // Use free tier
        self.metadataService = TokenMetadataService()
        self.discoveryService = TokenDiscoveryService(metadataService: metadataService)

        // Initialize market manager
        self.marketManager = MarketDataManager(
            priceService: priceService,
            metadataService: metadataService,
            discoveryService: discoveryService
        )
    }

    // MARK: - Example 1: Get Real-Time Prices

    func example1_GetPrices() async {
        print("=== Example 1: Get Real-Time Prices ===\n")

        do {
            // Fetch prices for popular tokens
            let prices = try await priceService.getTokenPrices(
                ids: ["bitcoin", "ethereum", "binancecoin"],
                currency: "usd"
            )

            // Display results
            for (id, price) in prices {
                print("\(id.uppercased()):")
                print("  Price: \(price.formattedPrice())")
                print("  24h Change: \(price.formattedChange24h)")
                print("  Market Cap: $\(price.marketCap ?? 0)")
                print()
            }

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 2: Discover Tokens in Wallet

    func example2_DiscoverTokens() async {
        print("=== Example 2: Discover Tokens ===\n")

        do {
            // Example Ethereum address (replace with real address)
            let walletAddress = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"

            let tokens = try await discoveryService.discoverTokens(
                for: walletAddress,
                network: .ethereum,
                includeZeroBalance: false
            )

            print("Found \(tokens.count) tokens:\n")

            for token in tokens.prefix(10) { // Show first 10
                let metadata = token.metadata
                print("\(metadata.name) (\(metadata.symbol))")
                print("  Balance: \(token.balance)")
                print("  Verified: \(metadata.verified ? "✅" : "❌")")
                print()
            }

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 3: Calculate Portfolio Value

    func example3_CalculatePortfolio() async {
        print("=== Example 3: Calculate Portfolio ===\n")

        do {
            // Sample accounts
            let accounts = [
                Account(
                    address: "0x123...",
                    name: "Main Wallet",
                    chainId: 1
                )
            ]

            // Sample tokens with balances
            let tokens = [
                Token(
                    contractAddress: "0xdac17f958d2ee523a2206206994597c13d831ec7",
                    chainId: 1,
                    symbol: "USDT",
                    name: "Tether USD",
                    decimals: 6,
                    balance: "1000000000", // 1,000 USDT
                    coingeckoId: "tether"
                ),
                Token(
                    contractAddress: "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984",
                    chainId: 1,
                    symbol: "UNI",
                    name: "Uniswap",
                    decimals: 18,
                    balance: "10000000000000000000", // 10 UNI
                    coingeckoId: "uniswap"
                )
            ]

            // Calculate portfolio value
            let portfolio = try await marketManager.calculatePortfolioValue(
                accounts: accounts,
                tokens: tokens
            )

            print("Portfolio Summary:")
            print("  Total Value: \(portfolio.formattedTotalValue)")
            print("  24h Change: \(portfolio.formattedChange24h)")
            print("  Number of Tokens: \(portfolio.tokens.count)")
            print()

            // Show top gainer
            if let topGainer = portfolio.topGainers.first {
                print("Top Gainer:")
                print("  \(topGainer.token.symbol): \(topGainer.priceData?.formattedChange24h ?? "N/A")")
            }

            // Show top loser
            if let topLoser = portfolio.topLosers.first {
                print("Top Loser:")
                print("  \(topLoser.token.symbol): \(topLoser.priceData?.formattedChange24h ?? "N/A")")
            }

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 4: Watchlist Management

    func example4_Watchlist() async {
        print("=== Example 4: Watchlist Management ===\n")

        do {
            // Add tokens to watchlist
            try await marketManager.addToWatchlist(
                tokenId: "bitcoin",
                symbol: "BTC",
                name: "Bitcoin"
            )

            try await marketManager.addToWatchlist(
                tokenId: "ethereum",
                symbol: "ETH",
                name: "Ethereum"
            )

            try await marketManager.addToWatchlist(
                tokenId: "cardano",
                symbol: "ADA",
                name: "Cardano"
            )

            print("Added 3 tokens to watchlist\n")

            // Get watchlist with current prices
            let watchlist = try await marketManager.getWatchlist()

            print("Watchlist (\(watchlist.count) items):\n")

            for (item, price) in watchlist {
                print("\(item.symbol) - \(item.name)")
                print("  Price: \(price.formattedPrice())")
                print("  24h Change: \(price.formattedChange24h)")
                print()
            }

            // Save to persistence
            await marketManager.saveWatchlist()
            print("Watchlist saved to persistence")

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 5: Price Alerts

    func example5_PriceAlerts() async {
        print("=== Example 5: Price Alerts ===\n")

        do {
            // Create price alerts
            await marketManager.createPriceAlert(
                tokenId: "ethereum",
                tokenSymbol: "ETH",
                targetPrice: 2000.0,
                condition: .below
            )

            await marketManager.createPriceAlert(
                tokenId: "bitcoin",
                tokenSymbol: "BTC",
                targetPrice: 50000.0,
                condition: .above
            )

            print("Created 2 price alerts\n")

            // Check if any alerts triggered
            let triggered = try await marketManager.checkPriceAlerts()

            if triggered.isEmpty {
                print("No alerts triggered")
            } else {
                print("Triggered Alerts:")
                for alert in triggered {
                    print("  \(alert.tokenSymbol) is \(alert.condition.symbol) $\(alert.targetPrice)")
                }
            }

            // Save to persistence
            await marketManager.savePriceAlerts()
            print("\nAlerts saved to persistence")

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 6: Chart Data

    func example6_ChartData() async {
        print("=== Example 6: Chart Data ===\n")

        do {
            // Get 7-day chart data for Ethereum
            let chartData = try await marketManager.getChartData(
                tokenId: "ethereum",
                timeframe: .sevenDays
            )

            print("Chart Data for ETH (7 days):")
            print("  Data Points: \(chartData.dataPoints.count)")
            print("  Currency: \(chartData.currency.uppercased())")
            print()

            // Show first and last prices
            if let first = chartData.dataPoints.first,
               let last = chartData.dataPoints.last {

                let change = ((last.price - first.price) / first.price) * 100

                print("  Starting Price: $\(String(format: "%.2f", first.price))")
                print("  Ending Price: $\(String(format: "%.2f", last.price))")
                print("  7d Change: \(String(format: "%.2f%%", change))")
            }

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 7: Token Search

    func example7_SearchTokens() async {
        print("=== Example 7: Token Search ===\n")

        do {
            // Search for "uniswap"
            let results = try await marketManager.searchTokens(query: "uniswap")

            print("Search Results for 'uniswap' (\(results.count) found):\n")

            for result in results.prefix(5) { // Show top 5
                print("\(result.name) (\(result.symbol))")
                if let rank = result.marketCapRank {
                    print("  Market Cap Rank: #\(rank)")
                }
                print()
            }

        } catch {
            print("Error: \(error.localizedDescription)")
        }
    }

    // MARK: - Example 8: Auto-Refresh

    func example8_AutoRefresh() async {
        print("=== Example 8: Auto-Refresh ===\n")

        // Enable auto-refresh with 30-second interval
        await marketManager.setAutoRefresh(enabled: true)
        await marketManager.setRefreshInterval(30)

        print("Auto-refresh enabled (30s interval)")
        print("Price alerts will be checked automatically")
        print()

        // Simulate waiting
        print("Waiting 5 seconds...")
        try? await Task.sleep(nanoseconds: 5_000_000_000)

        print("Auto-refresh is running in background")
        print()

        // Disable auto-refresh
        await marketManager.setAutoRefresh(enabled: false)
        print("Auto-refresh disabled")
    }

    // MARK: - Run All Examples

    func runAllExamples() async {
        print("\n🚀 Running Phase 4 Market Data Examples\n")
        print("==========================================\n")

        await example1_GetPrices()
        print("\n==========================================\n")

        await example3_CalculatePortfolio()
        print("\n==========================================\n")

        await example4_Watchlist()
        print("\n==========================================\n")

        await example5_PriceAlerts()
        print("\n==========================================\n")

        await example6_ChartData()
        print("\n==========================================\n")

        await example7_SearchTokens()
        print("\n==========================================\n")

        await example8_AutoRefresh()
        print("\n==========================================\n")

        print("✅ All examples completed!\n")
    }
}

// MARK: - Usage

/*
 To run these examples:

 Task {
     let examples = MarketDataExample()
     await examples.runAllExamples()
 }

 Or run individual examples:

 Task {
     let examples = MarketDataExample()
     await examples.example1_GetPrices()
 }
 */
