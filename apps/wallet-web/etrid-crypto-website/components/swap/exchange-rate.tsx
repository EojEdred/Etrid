"use client"

import { useState, useEffect } from "react"
import { RefreshCw, TrendingUp, TrendingDown } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Network, useRealtimePrice, useNetworkSelection, WETR_TOKEN, formatTokenAmount } from "@/lib/dex"

interface ExchangeRateProps {
  network?: Network
}

export function ExchangeRate({ network: propNetwork }: ExchangeRateProps = {}) {
  const { network: defaultNetwork } = useNetworkSelection()
  const network = propNetwork || defaultNetwork

  const wetrToken = WETR_TOKEN[network]
  const { price, lastUpdate, loading, refetch } = useRealtimePrice(network, wetrToken.address)

  const [previousPrice, setPreviousPrice] = useState<number | null>(null)
  const [priceChange, setPriceChange] = useState<number>(0)
  const [secondsSinceUpdate, setSecondsSinceUpdate] = useState(0)

  useEffect(() => {
    if (price !== null && previousPrice !== null) {
      const change = ((price - previousPrice) / previousPrice) * 100
      setPriceChange(change)
    }
    if (price !== null) {
      setPreviousPrice(price)
    }
  }, [price])

  useEffect(() => {
    const interval = setInterval(() => {
      const seconds = Math.floor((Date.now() - lastUpdate) / 1000)
      setSecondsSinceUpdate(seconds)
    }, 1000)
    return () => clearInterval(interval)
  }, [lastUpdate])

  const handleRefresh = () => {
    refetch()
  }

  const displayPrice = price || 8.0
  const inversePrice = 1 / displayPrice

  return (
    <Card className="p-4 bg-card/30 backdrop-blur-xl border-border/50">
      <div className="flex items-center justify-between">
        <div className="space-y-1">
          <div className="flex items-center gap-2">
            <span className="text-lg font-semibold">
              1 wETR = {formatTokenAmount(displayPrice.toString())} USD
            </span>
            <Button variant="ghost" size="icon" onClick={handleRefresh} className="h-8 w-8" disabled={loading}>
              <RefreshCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
            </Button>
            {priceChange !== 0 && (
              <span
                className={`text-xs flex items-center gap-1 ${
                  priceChange > 0 ? "text-green-500" : "text-red-500"
                }`}
              >
                {priceChange > 0 ? <TrendingUp className="w-3 h-3" /> : <TrendingDown className="w-3 h-3" />}
                {Math.abs(priceChange).toFixed(2)}%
              </span>
            )}
          </div>
          <div className="text-sm text-muted-foreground">
            1 USD = {formatTokenAmount(inversePrice.toString())} wETR
          </div>
        </div>

        <div className="text-right">
          <div className="text-sm text-muted-foreground">
            {secondsSinceUpdate < 60 ? `${secondsSinceUpdate}s ago` : `${Math.floor(secondsSinceUpdate / 60)}m ago`}
          </div>
          <div className="text-xs text-muted-foreground/70">
            {network.charAt(0).toUpperCase() + network.slice(1)}
          </div>
        </div>
      </div>
    </Card>
  )
}
