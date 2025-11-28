"use client"

import { useState } from 'react'
import { Plus, Settings, Eye, EyeOff, Trash2 } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import type { IndicatorType, IndicatorConfig } from '@/lib/types/trading'

interface IndicatorSelectorProps {
  indicators: IndicatorConfig[]
  onAdd: (indicator: IndicatorConfig) => void
  onRemove: (type: IndicatorType) => void
  onToggle: (type: IndicatorType) => void
  onUpdateParams: (type: IndicatorType, params: Record<string, number>) => void
  className?: string
}

const AVAILABLE_INDICATORS: Array<{
  type: IndicatorType
  name: string
  description: string
  defaultParams: Record<string, number>
}> = [
  {
    type: 'SMA',
    name: 'Simple Moving Average',
    description: 'Average price over a period',
    defaultParams: { period: 20 },
  },
  {
    type: 'EMA',
    name: 'Exponential Moving Average',
    description: 'Weighted average emphasizing recent prices',
    defaultParams: { period: 20 },
  },
  {
    type: 'RSI',
    name: 'Relative Strength Index',
    description: 'Momentum oscillator (0-100)',
    defaultParams: { period: 14 },
  },
  {
    type: 'MACD',
    name: 'MACD',
    description: 'Trend-following momentum indicator',
    defaultParams: { fast: 12, slow: 26, signal: 9 },
  },
  {
    type: 'BOLLINGER_BANDS',
    name: 'Bollinger Bands',
    description: 'Volatility bands around moving average',
    defaultParams: { period: 20, stdDev: 2 },
  },
]

export function IndicatorSelector({
  indicators,
  onAdd,
  onRemove,
  onToggle,
  onUpdateParams,
  className,
}: IndicatorSelectorProps) {
  const [selectedIndicator, setSelectedIndicator] = useState<typeof AVAILABLE_INDICATORS[0] | null>(null)
  const [params, setParams] = useState<Record<string, number>>({})

  const handleAdd = () => {
    if (!selectedIndicator) return

    onAdd({
      type: selectedIndicator.type,
      params,
      visible: true,
      color: '#3b82f6',
    })

    setSelectedIndicator(null)
    setParams({})
  }

  return (
    <div className={cn('space-y-4', className)}>
      {/* Active Indicators */}
      {indicators.length > 0 && (
        <Card className="p-4">
          <div className="space-y-2">
            <h3 className="font-semibold text-sm">Active Indicators</h3>
            {indicators.map((indicator) => (
              <div
                key={indicator.type}
                className="flex items-center justify-between p-2 rounded-lg bg-muted/50"
              >
                <div className="flex items-center gap-2">
                  <Badge variant="outline" className="text-xs">
                    {indicator.type}
                  </Badge>
                  <span className="text-sm">
                    {Object.entries(indicator.params)
                      .map(([key, value]) => `${key}:${value}`)
                      .join(', ')}
                  </span>
                </div>

                <div className="flex items-center gap-1">
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    onClick={() => onToggle(indicator.type)}
                  >
                    {indicator.visible ? (
                      <Eye className="w-4 h-4" />
                    ) : (
                      <EyeOff className="w-4 h-4 text-muted-foreground" />
                    )}
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    onClick={() => onRemove(indicator.type)}
                  >
                    <Trash2 className="w-4 h-4 text-destructive" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}

      {/* Add Indicator Dialog */}
      <Dialog>
        <DialogTrigger asChild>
          <Button variant="outline" className="w-full">
            <Plus className="w-4 h-4 mr-2" />
            Add Indicator
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add Technical Indicator</DialogTitle>
          </DialogHeader>

          <div className="space-y-4">
            {/* Indicator Selection */}
            <div className="space-y-2">
              {AVAILABLE_INDICATORS.map((indicator) => {
                const isAdded = indicators.some((i) => i.type === indicator.type)

                return (
                  <button
                    key={indicator.type}
                    onClick={() => {
                      setSelectedIndicator(indicator)
                      setParams(indicator.defaultParams)
                    }}
                    disabled={isAdded}
                    className={cn(
                      'w-full p-3 text-left rounded-lg border transition-all',
                      selectedIndicator?.type === indicator.type
                        ? 'border-primary bg-primary/5'
                        : 'border-border hover:bg-accent/50',
                      isAdded && 'opacity-50 cursor-not-allowed'
                    )}
                  >
                    <div className="font-semibold text-sm">{indicator.name}</div>
                    <div className="text-xs text-muted-foreground mt-1">
                      {indicator.description}
                    </div>
                    {isAdded && (
                      <Badge variant="secondary" className="mt-2 text-xs">
                        Already Added
                      </Badge>
                    )}
                  </button>
                )
              })}
            </div>

            {/* Parameter Configuration */}
            {selectedIndicator && (
              <div className="space-y-3 p-4 rounded-lg bg-muted/50">
                <h4 className="font-semibold text-sm">Configure Parameters</h4>
                {Object.entries(selectedIndicator.defaultParams).map(([key, value]) => (
                  <div key={key} className="space-y-1">
                    <Label className="text-xs capitalize">{key}</Label>
                    <Input
                      type="number"
                      value={params[key] ?? value}
                      onChange={(e) =>
                        setParams((prev) => ({
                          ...prev,
                          [key]: parseFloat(e.target.value),
                        }))
                      }
                    />
                  </div>
                ))}
              </div>
            )}

            {/* Add Button */}
            <Button
              onClick={handleAdd}
              disabled={!selectedIndicator}
              className="w-full"
            >
              Add Indicator
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}
