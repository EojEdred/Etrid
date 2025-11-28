"use client"

import { SwapHeader } from "@/components/swap/swap-header"
import { SwapCard } from "@/components/swap/swap-card"
import { ExchangeRate } from "@/components/swap/exchange-rate"
import { RecentSwaps } from "@/components/swap/recent-swaps"
import { PriceChart } from "@/components/swap/price-chart"
import { InfoCards } from "@/components/swap/info-cards"
import { useWallet } from "@/lib/polkadot/useWallet"

export default function SwapPage() {
  const wallet = useWallet()

  return (
    <div className="min-h-screen bg-background">
      <SwapHeader wallet={wallet} />

      <main className="container mx-auto px-4 py-8">
        {wallet.error && (
          <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive">
            {wallet.error}
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Main swap area */}
          <div className="lg:col-span-2 space-y-6">
            <SwapCard wallet={wallet} />
            <ExchangeRate />
            <RecentSwaps />
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            <PriceChart />
          </div>
        </div>

        {/* Info cards at bottom */}
        <InfoCards />
      </main>
    </div>
  )
}
