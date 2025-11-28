'use client'

import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { TrendingDown, DollarSign, Info } from 'lucide-react'
import { TaxLossOpportunity } from '@/lib/types/analytics'
import { useState } from 'react'

interface TaxLossCardProps {
  opportunity: TaxLossOpportunity
  onHarvest?: (asset: string, amount: number) => Promise<void>
}

export function TaxLossCard({ opportunity, onHarvest }: TaxLossCardProps) {
  const [harvesting, setHarvesting] = useState(false)
  const [showExplanation, setShowExplanation] = useState(false)

  const handleHarvest = async () => {
    if (!onHarvest) return

    try {
      setHarvesting(true)
      await onHarvest(opportunity.asset, opportunity.amount)
    } catch (error) {
      console.error('Failed to harvest loss:', error)
    } finally {
      setHarvesting(false)
    }
  }

  return (
    <Card className="p-4">
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center gap-2">
          <div className="w-10 h-10 rounded-full bg-red-500/10 flex items-center justify-center">
            <TrendingDown className="w-5 h-5 text-red-500" />
          </div>
          <div>
            <h4 className="font-semibold">{opportunity.symbol}</h4>
            <p className="text-xs text-muted-foreground">{opportunity.asset}</p>
          </div>
        </div>
        <button
          onClick={() => setShowExplanation(!showExplanation)}
          className="text-muted-foreground hover:text-foreground"
        >
          <Info className="w-4 h-4" />
        </button>
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div>
          <p className="text-xs text-muted-foreground mb-1">Unrealized Loss</p>
          <p className="text-lg font-bold text-red-500">
            ${Math.abs(opportunity.unrealizedLoss).toLocaleString()}
          </p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground mb-1">Est. Tax Savings</p>
          <p className="text-lg font-bold text-green-500">
            ${opportunity.estimatedTaxSavings.toLocaleString()}
          </p>
        </div>
      </div>

      <div className="flex items-center justify-between text-xs text-muted-foreground mb-4">
        <span>Amount: {opportunity.amount} {opportunity.symbol}</span>
        <span>
          Cost: ${opportunity.costBasis.toFixed(2)} → ${opportunity.currentPrice.toFixed(2)}
        </span>
      </div>

      {showExplanation && (
        <div className="mb-4 p-3 bg-muted/50 rounded-lg text-xs space-y-2">
          <p>
            <strong>Tax Loss Harvesting:</strong> Sell this asset at a loss to offset
            capital gains and reduce your tax liability.
          </p>
          <p>
            <strong>Warning:</strong> Avoid wash sale rule - don't repurchase within 30
            days.
          </p>
        </div>
      )}

      <Button
        onClick={handleHarvest}
        disabled={harvesting}
        className="w-full"
        variant="outline"
      >
        {harvesting ? 'Harvesting...' : 'Harvest Loss'}
      </Button>
    </Card>
  )
}
