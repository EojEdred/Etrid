"use client"

import { Card } from "@/components/ui/card"
import { Network, useTransactionHistory, NETWORK_CONFIG, formatTokenAmount, formatUsdValue } from "@/lib/dex"
import { ExternalLink, Loader2, CheckCircle2, XCircle, Clock } from "lucide-react"
import { Button } from "@/components/ui/button"

interface TransactionHistoryProps {
  network: Network
  walletAddress: string | null
}

export function TransactionHistory({ network, walletAddress }: TransactionHistoryProps) {
  const { transactions, loading, error, refetch } = useTransactionHistory(network, walletAddress)

  const getStatusIcon = (status: 'pending' | 'success' | 'failed') => {
    switch (status) {
      case 'success':
        return <CheckCircle2 className="w-4 h-4 text-green-500" />
      case 'failed':
        return <XCircle className="w-4 h-4 text-red-500" />
      case 'pending':
        return <Clock className="w-4 h-4 text-yellow-500 animate-pulse" />
    }
  }

  const getExplorerUrl = (hash: string) => {
    const config = NETWORK_CONFIG[network]
    return `${config.explorer}/tx/${hash}`
  }

  const formatTimestamp = (timestamp: number) => {
    const date = new Date(timestamp)
    const now = Date.now()
    const diff = now - timestamp

    if (diff < 60000) return 'Just now'
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`
    return date.toLocaleDateString()
  }

  if (!walletAddress) {
    return (
      <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
        <h3 className="text-xl font-bold mb-4">Transaction History</h3>
        <div className="text-center py-8 text-muted-foreground">
          <p>Connect your wallet to view transaction history</p>
        </div>
      </Card>
    )
  }

  return (
    <Card className="p-6 bg-card/30 backdrop-blur-xl border-border/50">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-xl font-bold">Transaction History</h3>
        <Button variant="ghost" size="sm" onClick={refetch} disabled={loading}>
          Refresh
        </Button>
      </div>

      {loading && (
        <div className="py-8 flex items-center justify-center">
          <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
        </div>
      )}

      {error && (
        <div className="py-8 text-center text-sm text-destructive">
          Failed to load transaction history
        </div>
      )}

      {!loading && !error && transactions.length === 0 && (
        <div className="py-8 text-center text-muted-foreground">
          <p>No transactions yet</p>
          <p className="text-sm mt-2">Your swap history will appear here</p>
        </div>
      )}

      {!loading && !error && transactions.length > 0 && (
        <div className="space-y-3">
          {transactions.map((tx) => (
            <div
              key={tx.hash}
              className="p-4 rounded-lg bg-background/50 border border-border/50 hover:border-accent/50 transition-colors"
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-2">
                    {getStatusIcon(tx.status)}
                    <span className="font-medium">
                      {formatTokenAmount(tx.amountIn)} {tx.tokenIn.symbol} → {formatTokenAmount(tx.amountOut)}{' '}
                      {tx.tokenOut.symbol}
                    </span>
                  </div>
                  <div className="text-sm text-muted-foreground">
                    {formatTimestamp(tx.timestamp)} · {tx.dex.charAt(0).toUpperCase() + tx.dex.slice(1)}
                  </div>
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-8 w-8"
                  asChild
                >
                  <a
                    href={getExplorerUrl(tx.hash)}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <ExternalLink className="w-4 h-4" />
                  </a>
                </Button>
              </div>
            </div>
          ))}
        </div>
      )}
    </Card>
  )
}
