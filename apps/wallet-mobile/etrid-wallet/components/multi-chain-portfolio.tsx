"use client"

import { useState, useEffect } from "react"
import { TrendingUp, ChevronRight, Loader2 } from "lucide-react"
import { useWallet } from "@/lib/polkadot/useWallet"
import { getBalance, formatBalance } from "@/lib/polkadot/api"
import { CHAINS, getAllChains, type ChainId } from "@/lib/polkadot/chains"

interface ChainBalance {
  chainId: ChainId
  symbol: string
  name: string
  balance: string
  balanceRaw: bigint
  color: string
  usdValue: number
}

export function MultiChainPortfolio() {
  const { selectedAccount, isConnected } = useWallet()
  const [balances, setBalances] = useState<ChainBalance[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [totalUSD, setTotalUSD] = useState(0)

  // Mock USD prices for each token (in production, fetch from oracle/API)
  const USD_PRICES: Record<string, number> = {
    'ÉTR': 8.00,
    'BTC': 45000.00,
    'ETH': 2800.00,
    'DOGE': 0.08,
    'SOL': 105.00,
    'XLM': 0.12,
    'XRP': 0.55,
    'BNB': 320.00,
    'TRX': 0.10,
    'ADA': 0.45,
    'LINK': 14.50,
    'MATIC': 0.85,
    'SC-USDT': 1.00,
    'EDSC': 1.00,
  }

  // Fetch balances for all chains in parallel
  useEffect(() => {
    if (!isConnected || !selectedAccount) return

    const fetchAllBalances = async () => {
      setIsLoading(true)
      try {
        const chains = getAllChains()

        // Fetch all balances in parallel
        const balancePromises = chains.map(async (chain) => {
          try {
            const balanceRaw = await getBalance(chain.id, selectedAccount.address)
            const balance = formatBalance(balanceRaw, chain.decimals)
            const numericBalance = parseFloat(balance)
            const usdPrice = USD_PRICES[chain.symbol] || 0
            const usdValue = numericBalance * usdPrice

            return {
              chainId: chain.id,
              symbol: chain.symbol,
              name: chain.name,
              balance,
              balanceRaw,
              color: chain.color,
              usdValue,
            }
          } catch (error) {
            console.error(`Failed to fetch balance for ${chain.symbol}:`, error)
            return {
              chainId: chain.id,
              symbol: chain.symbol,
              name: chain.name,
              balance: '0',
              balanceRaw: BigInt(0),
              color: chain.color,
              usdValue: 0,
            }
          }
        })

        const fetchedBalances = await Promise.all(balancePromises)

        // Sort by USD value (highest first)
        fetchedBalances.sort((a, b) => b.usdValue - a.usdValue)

        setBalances(fetchedBalances)

        // Calculate total portfolio value
        const total = fetchedBalances.reduce((sum, bal) => sum + bal.usdValue, 0)
        setTotalUSD(total)
      } catch (error) {
        console.error('[MultiChainPortfolio] Failed to fetch balances:', error)
      } finally {
        setIsLoading(false)
      }
    }

    fetchAllBalances()

    // Refresh balances every 30 seconds
    const interval = setInterval(fetchAllBalances, 30000)
    return () => clearInterval(interval)
  }, [isConnected, selectedAccount])

  if (!isConnected) {
    return null
  }

  return (
    <div className="space-y-4">
      {/* Portfolio Summary */}
      <div className="glass-strong rounded-3xl p-6 shadow-2xl">
        <div className="text-center mb-4">
          <p className="text-muted-foreground text-sm mb-2">Total Portfolio Value</p>
          <h2 className="text-4xl font-bold mb-2">
            {isLoading ? (
              <Loader2 className="w-8 h-8 animate-spin mx-auto" />
            ) : (
              `$${totalUSD.toLocaleString("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
            )}
          </h2>
          <div className="flex items-center justify-center gap-1 text-success text-sm">
            <TrendingUp className="w-4 h-4" />
            <span>Across {balances.length} chains</span>
          </div>
        </div>
      </div>

      {/* Chain Balances List */}
      <div className="space-y-3">
        <h3 className="text-lg font-semibold px-2">Multi-Chain Balances</h3>

        {isLoading && balances.length === 0 ? (
          <div className="glass-strong rounded-2xl p-8 text-center">
            <Loader2 className="w-8 h-8 animate-spin mx-auto mb-2" />
            <p className="text-muted-foreground">Loading balances from all chains...</p>
          </div>
        ) : (
          balances.map((chainBalance) => (
            <ChainBalanceCard key={chainBalance.chainId} {...chainBalance} />
          ))
        )}
      </div>
    </div>
  )
}

// Individual chain balance card
function ChainBalanceCard({ symbol, name, balance, color, usdValue }: ChainBalance) {
  const numericBalance = parseFloat(balance)

  // Only show chains with balance > 0 or top chains
  const shouldShow = numericBalance > 0 || ['ÉTR', 'EDSC', 'BTC', 'ETH'].includes(symbol)

  if (!shouldShow) return null

  return (
    <div className="glass rounded-2xl p-4 hover:glass-strong transition-all cursor-pointer">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div
            className="w-12 h-12 rounded-full flex items-center justify-center font-bold text-sm"
            style={{
              backgroundColor: `${color}20`,
              color: color
            }}
          >
            {symbol.slice(0, 3)}
          </div>
          <div>
            <p className="font-semibold">{symbol}</p>
            <p className="text-xs text-muted-foreground">{name}</p>
          </div>
        </div>

        <div className="text-right flex items-center gap-2">
          <div>
            <p className="font-semibold">
              {numericBalance.toLocaleString(undefined, {
                minimumFractionDigits: 2,
                maximumFractionDigits: 6
              })} {symbol}
            </p>
            <p className="text-xs text-muted-foreground">
              ≈ ${usdValue.toLocaleString("en-US", {
                minimumFractionDigits: 2,
                maximumFractionDigits: 2
              })}
            </p>
          </div>
          <ChevronRight className="w-5 h-5 text-muted-foreground" />
        </div>
      </div>
    </div>
  )
}
