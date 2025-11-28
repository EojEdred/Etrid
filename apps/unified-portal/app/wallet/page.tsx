"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Wallet, ArrowRightLeft, Coins, Zap, TrendingUp } from "lucide-react"
import Link from "next/link"
import { useWallet } from "@/lib/polkadot/useWallet"
import { Button } from "@/components/ui/button"

export default function WalletPage() {
  const wallet = useWallet()

  const walletFeatures = [
    {
      title: "Swap",
      description: "Exchange tokens instantly with best rates",
      icon: ArrowRightLeft,
      href: "/wallet/swap",
      color: "text-blue-500",
      bgColor: "bg-blue-500/10"
    },
    {
      title: "Staking",
      description: "Stake tokens and earn rewards",
      icon: Coins,
      href: "/wallet/staking",
      color: "text-green-500",
      bgColor: "bg-green-500/10"
    },
    {
      title: "Lightning",
      description: "Instant cross-chain payments",
      icon: Zap,
      href: "/wallet/lightning",
      color: "text-yellow-500",
      bgColor: "bg-yellow-500/10"
    },
    {
      title: "ETH PBC Staking",
      description: "Stake LP tokens on ETH Partition Burst Chain",
      icon: TrendingUp,
      href: "/wallet/staking/eth-pbc",
      color: "text-purple-500",
      bgColor: "bg-purple-500/10"
    }
  ]

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <Wallet className="h-8 w-8 text-green-500" />
            <h1 className="text-4xl font-bold">Wallet & DeFi</h1>
          </div>
          {!wallet.isConnected && (
            <Button onClick={wallet.connect} disabled={wallet.isLoading}>
              {wallet.isLoading ? "Connecting..." : "Connect Wallet"}
            </Button>
          )}
        </div>
        <p className="text-zinc-600 dark:text-zinc-400">
          Swap, stake, and manage your digital assets
        </p>
      </div>

      {/* Wallet Status */}
      {wallet.isConnected && wallet.selectedAccount && (
        <Card className="mb-8">
          <CardHeader>
            <CardTitle>Connected Wallet</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Account:</span>
                <span className="font-mono">{wallet.selectedAccount.meta.name || "Unnamed"}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Address:</span>
                <span className="font-mono text-sm">
                  {wallet.selectedAccount.address.slice(0, 8)}...{wallet.selectedAccount.address.slice(-8)}
                </span>
              </div>
              {wallet.selectedAccount.balance && (
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Balance:</span>
                  <span className="font-bold">{wallet.selectedAccount.balance}</span>
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Error Display */}
      {wallet.error && (
        <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive">
          {wallet.error}
        </div>
      )}

      {/* Feature Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-6">
        {walletFeatures.map((feature) => {
          const Icon = feature.icon
          return (
            <Link key={feature.href} href={feature.href}>
              <Card className="hover:border-primary transition-all hover:shadow-lg cursor-pointer h-full">
                <CardHeader>
                  <div className={`w-12 h-12 rounded-lg ${feature.bgColor} flex items-center justify-center mb-4`}>
                    <Icon className={`h-6 w-6 ${feature.color}`} />
                  </div>
                  <CardTitle>{feature.title}</CardTitle>
                  <CardDescription>{feature.description}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center text-sm text-primary">
                    Launch <ArrowRightLeft className="h-4 w-4 ml-2" />
                  </div>
                </CardContent>
              </Card>
            </Link>
          )
        })}
      </div>

      {/* Info Section */}
      <div className="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Multi-Chain Support</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Connect to Primearc Core Chain and all Partition Burst Chains (ETH, BSC, Polygon, etc.)
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Lightning Fast</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Instant cross-chain payments with Lightning-Bloc technology
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-lg">Secure & Private</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Non-custodial wallet with Polkadot.js extension integration
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
