"use client"

import { useState } from "react"
import { ChevronDown, ChevronUp, Info } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { QuoteResult, Network, SLIPPAGE_PRESETS, formatTokenAmount, getPriceImpactLevel } from "@/lib/dex"

interface SwapDetailsProps {
  fromAmount: string
  toAmount: string
  quote: QuoteResult | null
  slippage: number
  onSlippageChange: (value: number) => void
  onCustomSlippage: (value: number) => void
  network: Network
}

export function SwapDetails({
  fromAmount,
  toAmount,
  quote,
  slippage,
  onSlippageChange,
  onCustomSlippage,
  network,
}: SwapDetailsProps) {
  const [expanded, setExpanded] = useState(false)
  const [customSlippageInput, setCustomSlippageInput] = useState("")
  const [isCustom, setIsCustom] = useState(false)

  const priceImpactInfo = quote ? getPriceImpactLevel(quote.priceImpact) : null

  const handleSlippagePreset = (value: number) => {
    setIsCustom(false)
    setCustomSlippageInput("")
    onSlippageChange(value)
  }

  const handleCustomSlippage = (value: string) => {
    setCustomSlippageInput(value)
    const numValue = parseFloat(value)
    if (!isNaN(numValue) && numValue > 0 && numValue <= 50) {
      setIsCustom(true)
      onCustomSlippage(numValue)
    }
  }

  return (
    <div className="mt-4 p-4 rounded-xl bg-background/30 border border-border/30">
      <Button
        variant="ghost"
        onClick={() => setExpanded(!expanded)}
        className="w-full justify-between p-0 h-auto hover:bg-transparent"
      >
        <span className="text-sm font-medium">Transaction Settings</span>
        {expanded ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
      </Button>

      {expanded && (
        <div className="mt-4 space-y-4">
          {/* Slippage Tolerance */}
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <div className="flex items-center gap-1">
                <span className="text-muted-foreground">Slippage tolerance</span>
                <Info className="w-3 h-3 text-muted-foreground" />
              </div>
            </div>
            <div className="flex items-center gap-2">
              {SLIPPAGE_PRESETS.map((preset) => (
                <Button
                  key={preset}
                  variant={slippage === preset && !isCustom ? "default" : "outline"}
                  size="sm"
                  onClick={() => handleSlippagePreset(preset)}
                  className="flex-1"
                >
                  {preset}%
                </Button>
              ))}
              <div className="flex items-center gap-1">
                <Input
                  type="number"
                  placeholder="Custom"
                  value={customSlippageInput}
                  onChange={(e) => handleCustomSlippage(e.target.value)}
                  className="w-20 h-8 text-sm"
                  step="0.1"
                  min="0.01"
                  max="50"
                />
                <span className="text-sm">%</span>
              </div>
            </div>
            {slippage > 5 && (
              <p className="text-xs text-orange-500">
                Warning: High slippage tolerance may result in unfavorable rates
              </p>
            )}
          </div>

          {quote && (
            <>
              {/* Route */}
              {quote.route.length > 2 && (
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">Route</span>
                  <span className="font-medium text-xs">{quote.route.join(' → ')}</span>
                </div>
              )}

              {/* Minimum Received */}
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Minimum received</span>
                <span className="font-medium">{formatTokenAmount(quote.minimumOutputAmount)}</span>
              </div>

              {/* Price Impact */}
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Price impact</span>
                <span className={`font-medium ${priceImpactInfo?.color || ''}`}>
                  {quote.priceImpact.toFixed(2)}%
                </span>
              </div>

              {/* Liquidity Provider Fee */}
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Liquidity provider fee</span>
                <span className="font-medium">{formatTokenAmount(quote.fee)}</span>
              </div>

              {/* Network Fee */}
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Network fee (est.)</span>
                <span className="font-medium">
                  {quote.gas} {network === Network.SOLANA ? 'SOL' : 'ETH'}
                </span>
              </div>
            </>
          )}

          {!quote && fromAmount && (
            <div className="text-sm text-muted-foreground text-center py-2">
              Enter an amount to see transaction details
            </div>
          )}
        </div>
      )}
    </div>
  )
}
