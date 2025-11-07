"use client"

import { LightningHeader } from "@/components/lightning/lightning-header"
import { PaymentCard } from "@/components/lightning/payment-card"
import { ChannelsList } from "@/components/lightning/channels-list"
import { PaymentHistory } from "@/components/lightning/payment-history"
import { NetworkStats } from "@/components/lightning/network-stats"
import { CrossChainInfo } from "@/components/lightning/cross-chain-info"
import { useWallet } from "@/lib/polkadot/useWallet"
import { useLightning } from "@/lib/lightning/useLightning"

export default function LightningPage() {
  const wallet = useWallet()
  const lightning = useLightning()

  return (
    <div className="min-h-screen bg-background">
      <LightningHeader wallet={wallet} lightning={lightning} />

      <main className="container mx-auto px-4 py-8">
        {wallet.error && (
          <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive">
            {wallet.error}
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Main payment area */}
          <div className="lg:col-span-2 space-y-6">
            <PaymentCard wallet={wallet} lightning={lightning} />
            <CrossChainInfo />
            <PaymentHistory payments={lightning.payments} />
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            <NetworkStats lightning={lightning} />
            <ChannelsList channels={lightning.channels} />
          </div>
        </div>
      </main>
    </div>
  )
}
