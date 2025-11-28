'use client'

import { Card } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { TestTube } from 'lucide-react'
import { useState } from 'react'

interface ThresholdInputProps {
  onSave: (config: ThresholdConfig) => void
  onTest?: () => void
}

interface ThresholdConfig {
  asset: string
  threshold: number
  condition: 'above' | 'below' | 'crosses'
}

export function ThresholdInput({ onSave, onTest }: ThresholdInputProps) {
  const [asset, setAsset] = useState('ÉTR')
  const [threshold, setThreshold] = useState('')
  const [condition, setCondition] = useState<'above' | 'below' | 'crosses'>('above')

  const handleSave = () => {
    if (!threshold) return

    onSave({
      asset,
      threshold: parseFloat(threshold),
      condition,
    })

    // Reset form
    setThreshold('')
  }

  return (
    <Card className="p-4">
      <h4 className="font-semibold mb-4">Configure Price Alert</h4>

      <div className="space-y-4">
        <div>
          <label className="text-sm font-medium mb-2 block">Asset</label>
          <Select value={asset} onValueChange={setAsset}>
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ÉTR">ÉTR</SelectItem>
              <SelectItem value="BTC">BTC</SelectItem>
              <SelectItem value="ETH">ETH</SelectItem>
              <SelectItem value="DOT">DOT</SelectItem>
              <SelectItem value="ADA">ADA</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div>
          <label className="text-sm font-medium mb-2 block">Condition</label>
          <Select
            value={condition}
            onValueChange={(val) => setCondition(val as any)}
          >
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="above">Goes Above</SelectItem>
              <SelectItem value="below">Goes Below</SelectItem>
              <SelectItem value="crosses">Crosses</SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div>
          <label className="text-sm font-medium mb-2 block">
            Threshold ($)
          </label>
          <Input
            type="number"
            placeholder="50.00"
            value={threshold}
            onChange={(e) => setThreshold(e.target.value)}
            step="0.01"
          />
        </div>

        <div className="flex gap-2">
          <Button onClick={handleSave} className="flex-1">
            Save Alert
          </Button>
          {onTest && (
            <Button variant="outline" size="icon" onClick={onTest}>
              <TestTube className="w-4 h-4" />
            </Button>
          )}
        </div>
      </div>
    </Card>
  )
}
