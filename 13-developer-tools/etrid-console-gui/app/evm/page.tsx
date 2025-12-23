'use client'

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import dynamic from 'next/dynamic'
import { PageHeader } from '@/components/layout/PageHeader'

const Web3Provider = dynamic(
  () => import('@/components/providers/Web3Provider').then((m) => m.Web3Provider),
  { ssr: false, loading: () => <div className="animate-pulse bg-muted h-48 rounded-lg" /> }
)

const EvmWallet = dynamic(
  () => import('@/components/evm/EvmWallet').then((m) => m.EvmWallet),
  { ssr: false, loading: () => <div className="animate-pulse bg-muted h-96 rounded-lg" /> }
)

export default function EvmPage() {
  return (
    <div className="min-h-screen gradient-bg-animated">
      <PageHeader title="EVM Wallet" subtitle="MetaMask-style connect, send, sign" />

      <div className="container mx-auto px-4 py-10 max-w-5xl space-y-6">
        <div className="text-center">
          <h1 className="text-3xl font-bold gradient-text mb-2">EVM Wallet</h1>
          <p className="text-white/60">
            Connect an EVM wallet (MetaMask/WalletConnect) and send transactions.
          </p>
        </div>

        <Web3Provider>
          <EvmWallet />
        </Web3Provider>

        <Card className="glass-card">
          <CardHeader>
            <CardTitle>Notes</CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-white/70 space-y-2">
            <p>
              This page is meant to mirror a MetaMask-style flow for ËTRID’s EVM surface. Configure the target
              EVM RPC via <span className="font-mono">NEXT_PUBLIC_EVM_RPC_HTTP_URL</span> /
              <span className="font-mono">NEXT_PUBLIC_EVM_CHAIN_ID</span>.
            </p>
            <p>
              WalletConnect requires a real{' '}
              <span className="font-mono">NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID</span>.
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
