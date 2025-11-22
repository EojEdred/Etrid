/**
 * SupplyScreen - Deposit to earn interest
 */

'use client'

import { useState } from 'react'
import { ArrowLeft, TrendingUp } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { InterestAccrual } from '@/components/lending/interest-accrual'
import { useLending } from '@/hooks/use-lending'

interface SupplyScreenProps {
  asset?: string
  onBack: () => void
}

export function SupplyScreen({ asset: initialAsset, onBack }: SupplyScreenProps) {
  const [asset, setAsset] = useState(initialAsset || 'ÉTR')
  const [amount, setAmount] = useState('')
  const { supply, loading, getAPY } = useLending()
  const [apy, setAPY] = useState(8.0)

  const availableBalance = 734.56
  const assetPrice = asset === 'ÉTR' ? 8 : asset === 'BTC' ? 45000 : 2500

  const handleSupply = async () => {
    try {
      await supply(asset, parseFloat(amount))
      setAmount('')
      onBack()
    } catch (err) {
      console.error(err)
    }
  }

  const projectedEarnings = {
    daily: (parseFloat(amount || '0') * (apy / 100)) / 365,
    monthly: (parseFloat(amount || '0') * (apy / 100)) / 12,
    yearly: parseFloat(amount || '0') * (apy / 100),
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Supply {asset}</h1>
            <p className="text-sm text-muted-foreground">Deposit to earn interest</p>
          </div>
        </div>

        <div className="glass-strong rounded-2xl p-5">
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Current APY</span>
            <div className="flex items-center gap-1">
              <TrendingUp className="w-4 h-4 text-success" />
              <span className="text-2xl font-bold text-success">{apy}%</span>
            </div>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6">
        {/* Asset Selector */}
        <div className="space-y-3">
          <Label>Select Asset</Label>
          <Select value={asset} onValueChange={setAsset}>
            <SelectTrigger className="glass border-border h-14">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ÉTR">ÉTR - 8% APY</SelectItem>
              <SelectItem value="BTC">BTC - 4% APY</SelectItem>
              <SelectItem value="ETH">ETH - 5% APY</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {/* Amount Input */}
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <Label>Amount</Label>
            <span className="text-sm text-muted-foreground">
              Available: {availableBalance.toFixed(2)} {asset}
            </span>
          </div>

          <div className="glass rounded-2xl p-4">
            <div className="flex items-center gap-2 mb-4">
              <Input
                type="number"
                placeholder="0.00"
                value={amount}
                onChange={(e) => {
                  const value = parseFloat(e.target.value)
                  if (value <= availableBalance) {
                    setAmount(e.target.value)
                  }
                }}
                className="text-2xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0"
              />
              <span className="text-xl font-semibold text-muted-foreground">{asset}</span>
              <Button
                variant="ghost"
                size="sm"
                className="ml-auto text-accent"
                onClick={() => setAmount(availableBalance.toString())}
              >
                Max
              </Button>
            </div>

            <div className="text-sm text-muted-foreground">
              ≈ ${(parseFloat(amount || '0') * assetPrice).toFixed(2)}
            </div>
          </div>
        </div>

        {/* Projected Earnings */}
        {parseFloat(amount || '0') > 0 && (
          <div className="glass-strong rounded-2xl p-5 space-y-3">
            <h3 className="font-semibold flex items-center gap-2">
              <TrendingUp className="w-5 h-5 text-success" />
              Projected Earnings
            </h3>
            <div className="grid grid-cols-3 gap-4">
              <div>
                <p className="text-xs text-muted-foreground mb-1">Daily</p>
                <p className="text-lg font-bold text-success">
                  +{projectedEarnings.daily.toFixed(4)}
                </p>
              </div>
              <div>
                <p className="text-xs text-muted-foreground mb-1">Monthly</p>
                <p className="text-lg font-bold text-success">
                  +{projectedEarnings.monthly.toFixed(2)}
                </p>
              </div>
              <div>
                <p className="text-xs text-muted-foreground mb-1">Yearly</p>
                <p className="text-lg font-bold text-success">
                  +{projectedEarnings.yearly.toFixed(2)}
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Supply Button */}
        <Button
          onClick={handleSupply}
          disabled={!amount || parseFloat(amount) <= 0 || loading}
          className="w-full h-14 text-lg font-semibold"
          style={{
            background: amount && parseFloat(amount) > 0 ? '#00d9ff' : undefined,
            color: amount && parseFloat(amount) > 0 ? '#000' : undefined,
          }}
        >
          {loading ? 'Supplying...' : 'Supply'}
        </Button>

        {/* Info */}
        <div className="glass rounded-xl p-4 space-y-2 text-sm">
          <div className="flex items-center justify-between">
            <span className="text-muted-foreground">Withdrawal</span>
            <span>Anytime (if liquidity available)</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-muted-foreground">Lock Period</span>
            <span>None</span>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-muted-foreground">Interest</span>
            <span>Accrues automatically</span>
          </div>
        </div>
      </main>
    </div>
  )
}
