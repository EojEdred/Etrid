"use client"

import { useState } from "react"
import { ChevronDown, ChevronUp } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

interface SwapDetailsProps {
  fromAmount: string
  toAmount: string
}

export function SwapDetails({ fromAmount, toAmount }: SwapDetailsProps) {
  const [expanded, setExpanded] = useState(false)
  const [slippage, setSlippage] = useState("0.5")

  const priceImpact = 0.1
  const lpFee = Number(fromAmount) * 0.003
  const minReceived = Number(toAmount) * (1 - Number(slippage) / 100)

  return (
    <div className="mt-4 p-4 rounded-xl bg-background/30 border border-border/30">
      <Button
        variant="ghost"
        onClick={() => setExpanded(!expanded)}
        className="w-full justify-between p-0 h-auto hover:bg-transparent"
      >
        <span className="text-sm font-medium">Transaction Details</span>
        {expanded ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
      </Button>

      {expanded && (
        <div className="mt-4 space-y-3">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Slippage tolerance</span>
            <div className="flex items-center gap-2">
              <Input
                type="number"
                value={slippage}
                onChange={(e) => setSlippage(e.target.value)}
                className="w-16 h-7 text-right"
              />
              <span>%</span>
            </div>
          </div>

          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Minimum received</span>
            <span className="font-medium">{minReceived.toFixed(2)} EDSC</span>
          </div>

          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Price impact</span>
            <span className={`font-medium ${priceImpact < 1 ? "text-success" : "text-destructive"}`}>
              {priceImpact}%
            </span>
          </div>

          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Liquidity provider fee</span>
            <span className="font-medium">0.3% ({lpFee.toFixed(4)} ÉTR)</span>
          </div>
        </div>
      )}
    </div>
  )
}
