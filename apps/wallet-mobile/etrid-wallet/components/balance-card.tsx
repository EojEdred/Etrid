"use client"

import { TrendingUp, Wallet } from "lucide-react"
import { useEffect, useState } from "react"
import { useWallet } from "@/lib/polkadot/useWallet"
import { getBalance, formatBalance } from "@/lib/polkadot/api"

export function BalanceCard() {
  const { selectedAccount, selectedChain, isConnected, connect } = useWallet()
  const [etrBalance, setEtrBalance] = useState<string>("0")
  const [edscBalance, setEdscBalance] = useState<string>("0")
  const [isLoading, setIsLoading] = useState(false)

  // Fetch balances when account/chain changes
  useEffect(() => {
    if (!isConnected || !selectedAccount) return

    const fetchBalances = async () => {
      setIsLoading(true)
      try {
        // Fetch ÉTR balance from FlareChain
        const etrRaw = await getBalance('flarechain', selectedAccount.address)
        setEtrBalance(formatBalance(etrRaw, 12))

        // Fetch EDSC balance from EDSC-PBC
        const edscRaw = await getBalance('edsc-pbc', selectedAccount.address)
        setEdscBalance(formatBalance(edscRaw, 12))
      } catch (error) {
        console.error('[BalanceCard] Failed to fetch balances:', error)
      } finally {
        setIsLoading(false)
      }
    }

    fetchBalances()
  }, [isConnected, selectedAccount, selectedChain])

  // Calculate total USD value (assuming ÉTR = $8, EDSC = $1)
  const totalUSD = (parseFloat(etrBalance) * 8 + parseFloat(edscBalance)).toFixed(2)

  // Show connect button if not connected
  if (!isConnected) {
    return (
      <div className="glass-strong rounded-3xl p-6 shadow-2xl text-center">
        <Wallet className="w-16 h-16 mx-auto mb-4 text-muted-foreground" />
        <p className="text-muted-foreground mb-4">Connect your wallet to view balances</p>
        <button
          onClick={connect}
          className="px-6 py-2 bg-accent text-white rounded-xl hover:bg-accent/90 transition-colors"
        >
          Connect Wallet
        </button>
      </div>
    )
  }

  return (
    <div className="glass-strong rounded-3xl p-6 shadow-2xl">
      {/* Total Balance */}
      <div className="text-center mb-6">
        <p className="text-muted-foreground text-sm mb-2">Total Balance</p>
        <h1 className="text-5xl font-bold mb-2">
          {isLoading ? (
            <span className="text-muted-foreground">Loading...</span>
          ) : (
            `$${parseFloat(totalUSD).toLocaleString("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
          )}
        </h1>
        <div className="flex items-center justify-center gap-1 text-success text-sm">
          <TrendingUp className="w-4 h-4" />
          <span>Live blockchain data</span>
        </div>
      </div>

      {/* Token Breakdown */}
      <div className="space-y-4">
        {/* ÉTR Token */}
        <div className="flex items-center justify-between p-4 rounded-2xl bg-surface/50">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center">
              <span className="text-accent font-bold text-sm">Ë</span>
            </div>
            <div>
              <p className="font-semibold">ÉTR</p>
              <p className="text-xs text-muted-foreground">Ëtrid Coin</p>
            </div>
          </div>
          <div className="text-right">
            <p className="font-semibold">{parseFloat(etrBalance).toLocaleString()} ÉTR</p>
            <p className="text-xs text-muted-foreground">≈ ${(parseFloat(etrBalance) * 8).toFixed(2)}</p>
          </div>
        </div>

        {/* EDSC Token */}
        <div className="flex items-center justify-between p-4 rounded-2xl bg-surface/50">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-full bg-success/20 flex items-center justify-center">
              <span className="text-success font-bold text-sm">$</span>
            </div>
            <div>
              <p className="font-semibold">EDSC</p>
              <p className="text-xs text-muted-foreground">Dollar Stable</p>
            </div>
          </div>
          <div className="text-right">
            <p className="font-semibold">{parseFloat(edscBalance).toLocaleString()} EDSC</p>
            <p className="text-xs text-muted-foreground">≈ ${parseFloat(edscBalance).toFixed(2)}</p>
          </div>
        </div>
      </div>

      {selectedAccount && (
        <div className="mt-4 text-center text-xs text-muted-foreground">
          {selectedAccount.address.slice(0, 6)}...{selectedAccount.address.slice(-4)}
        </div>
      )}
    </div>
  )
}
