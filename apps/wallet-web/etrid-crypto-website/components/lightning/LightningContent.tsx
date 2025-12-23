'use client'

import { PaymentCard } from "./payment-card"
import { ChannelsList } from "./channels-list"
import { PaymentHistory } from "./payment-history"
import { NetworkStats } from "./network-stats"
import { CrossChainInfo } from "./cross-chain-info"
import { useWallet } from "@/lib/polkadot/useWallet"
import { useLightning } from "@/lib/lightning/useLightning"

export default function LightningContent() {
  const wallet = useWallet()
  const lightning = useLightning()

  return (
    <main className="container mx-auto px-4 py-8">
      {(wallet as any).error && (
        <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive">
          {(wallet as any).error}
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
  )
}
