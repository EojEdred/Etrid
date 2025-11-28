/**
 * CollateralSlider - Adjust collateral with health factor preview
 */

'use client'

import { useState } from 'react'
import { Plus, Minus } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Slider } from '@/components/ui/slider'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'

interface CollateralSliderProps {
  currentCollateral: number
  borrowedValue: number
  asset: string
  onAdd?: (amount: number) => void
  onRemove?: (amount: number) => void
}

export function CollateralSlider({
  currentCollateral,
  borrowedValue,
  asset,
  onAdd,
  onRemove,
}: CollateralSliderProps) {
  const [adjustAmount, setAdjustAmount] = useState(0)
  const [mode, setMode] = useState<'add' | 'remove'>('add')

  const newCollateral = mode === 'add'
    ? currentCollateral + adjustAmount
    : currentCollateral - adjustAmount

  const currentHF = (currentCollateral / borrowedValue) * 100
  const newHF = (newCollateral / borrowedValue) * 100

  const getHFColor = (hf: number) => {
    if (hf >= 200) return 'text-success'
    if (hf >= 150) return 'text-warning'
    return 'text-destructive'
  }

  return (
    <div className="glass rounded-2xl p-5 space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="font-semibold">Adjust Collateral</h3>
        <div className="flex gap-2">
          <Button
            size="sm"
            variant={mode === 'add' ? 'default' : 'outline'}
            onClick={() => setMode('add')}
            style={mode === 'add' ? { background: '#00d9ff', color: '#000' } : undefined}
          >
            <Plus className="w-4 h-4" />
          </Button>
          <Button
            size="sm"
            variant={mode === 'remove' ? 'default' : 'outline'}
            onClick={() => setMode('remove')}
          >
            <Minus className="w-4 h-4" />
          </Button>
        </div>
      </div>

      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Amount</span>
          <span className="font-semibold">{adjustAmount.toFixed(2)} {asset}</span>
        </div>

        <Slider
          value={[adjustAmount]}
          onValueChange={(value) => setAdjustAmount(value[0])}
          max={mode === 'add' ? 1000 : currentCollateral}
          step={1}
        />
      </div>

      <div className="glass-strong rounded-xl p-4 space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Current Health Factor</span>
          <span className={`font-bold ${getHFColor(currentHF)}`}>
            {currentHF.toFixed(0)}%
          </span>
        </div>

        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">New Health Factor</span>
          <span className={`font-bold ${getHFColor(newHF)}`}>
            {newHF.toFixed(0)}%
          </span>
        </div>

        {mode === 'remove' && newHF < 150 && (
          <div className="pt-2 border-t border-border">
            <p className="text-xs text-destructive">
              ⚠️ Warning: Health factor below safe threshold
            </p>
          </div>
        )}
      </div>

      <div className="flex gap-2">
        {onAdd && mode === 'add' && (
          <Button
            onClick={() => onAdd(adjustAmount)}
            disabled={adjustAmount === 0}
            className="flex-1"
            style={{ background: '#00d9ff', color: '#000' }}
          >
            Add Collateral
          </Button>
        )}
        {onRemove && mode === 'remove' && (
          <Button
            onClick={() => onRemove(adjustAmount)}
            disabled={adjustAmount === 0 || newHF < 150}
            variant="outline"
            className="flex-1"
          >
            Remove Collateral
          </Button>
        )}
      </div>
    </div>
  )
}
