/**
 * BorrowScreen - Borrow against collateral
 */

'use client'

import { useState } from 'react'
import { ArrowLeft, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { HealthFactorGauge } from '@/components/lending/health-factor-gauge'
import { useLending } from '@/hooks/use-lending'

interface BorrowScreenProps {
  asset?: string
  onBack: () => void
}

export function BorrowScreen({ asset: initialAsset, onBack }: BorrowScreenProps) {
  const [asset, setAsset] = useState(initialAsset || 'ÉTR')
  const [amount, setAmount] = useState('')
  const [collateralAsset, setCollateralAsset] = useState('ÉTR')
  const [collateralAmount, setCollateralAmount] = useState('')
  const { borrow, loading } = useLending()

  const borrowAPY = asset === 'ÉTR' ? 12 : asset === 'BTC' ? 8 : 10
  const assetPrice = asset === 'ÉTR' ? 8 : asset === 'BTC' ? 45000 : 2500
  const collateralPrice = collateralAsset === 'ÉTR' ? 8 : collateralAsset === 'BTC' ? 45000 : 2500

  const borrowValue = parseFloat(amount || '0') * assetPrice
  const collateralValue = parseFloat(collateralAmount || '0') * collateralPrice
  const healthFactor = collateralValue > 0 ? (collateralValue / borrowValue) * 100 : 0

  const liquidationPrice = borrowValue > 0 ? (borrowValue * 1.5) / parseFloat(collateralAmount || '1') : 0

  const handleBorrow = async () => {
    try {
      await borrow(asset, parseFloat(amount), [
        {
          asset: collateralAsset,
          amount: parseFloat(collateralAmount),
          valueUSD: collateralValue,
        },
      ])
      setAmount('')
      setCollateralAmount('')
      onBack()
    } catch (err) {
      console.error(err)
    }
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Borrow {asset}</h1>
            <p className="text-sm text-muted-foreground">Borrow against collateral</p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6">
        {/* Asset to Borrow */}
        <div className="space-y-3">
          <Label>Asset to Borrow</Label>
          <Select value={asset} onValueChange={setAsset}>
            <SelectTrigger className="glass border-border h-14">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ÉTR">ÉTR - 12% APY</SelectItem>
              <SelectItem value="BTC">BTC - 8% APY</SelectItem>
              <SelectItem value="ETH">ETH - 10% APY</SelectItem>
            </SelectContent>
          </Select>
        </div>

        {/* Borrow Amount */}
        <div className="space-y-3">
          <Label>Borrow Amount</Label>
          <div className="glass rounded-2xl p-4">
            <div className="flex items-center gap-2">
              <Input
                type="number"
                placeholder="0.00"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="text-2xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0"
              />
              <span className="text-xl font-semibold text-muted-foreground">{asset}</span>
            </div>
            <div className="text-sm text-muted-foreground mt-2">
              ≈ ${borrowValue.toFixed(2)}
            </div>
          </div>
        </div>

        {/* Collateral */}
        <div className="space-y-3">
          <Label>Collateral Asset</Label>
          <Select value={collateralAsset} onValueChange={setCollateralAsset}>
            <SelectTrigger className="glass border-border h-14">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ÉTR">ÉTR</SelectItem>
              <SelectItem value="BTC">BTC</SelectItem>
              <SelectItem value="ETH">ETH</SelectItem>
            </SelectContent>
          </Select>

          <div className="glass rounded-2xl p-4">
            <div className="flex items-center gap-2">
              <Input
                type="number"
                placeholder="0.00"
                value={collateralAmount}
                onChange={(e) => setCollateralAmount(e.target.value)}
                className="text-2xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0"
              />
              <span className="text-xl font-semibold text-muted-foreground">{collateralAsset}</span>
            </div>
            <div className="text-sm text-muted-foreground mt-2">
              ≈ ${collateralValue.toFixed(2)}
            </div>
          </div>
        </div>

        {/* Health Factor */}
        {healthFactor > 0 && (
          <div className="glass-strong rounded-2xl p-5">
            <HealthFactorGauge healthFactor={healthFactor} size="md" />
          </div>
        )}

        {/* Warnings */}
        {healthFactor > 0 && healthFactor < 150 && (
          <div className="glass rounded-xl p-4 bg-destructive/10 border-destructive/20">
            <div className="flex gap-2">
              <AlertCircle className="w-5 h-5 text-destructive shrink-0" />
              <div>
                <p className="font-semibold text-destructive">Insufficient Collateral</p>
                <p className="text-sm text-muted-foreground mt-1">
                  Minimum 150% collateral required. Please add more collateral.
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Loan Details */}
        {healthFactor >= 150 && (
          <div className="glass rounded-xl p-4 space-y-2 text-sm">
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Borrow APY</span>
              <span className="font-semibold text-destructive">{borrowAPY}%</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Health Factor</span>
              <span className="font-semibold">{healthFactor.toFixed(0)}%</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-muted-foreground">Liquidation Price</span>
              <span className="font-semibold">${liquidationPrice.toFixed(2)}</span>
            </div>
          </div>
        )}

        {/* Borrow Button */}
        <Button
          onClick={handleBorrow}
          disabled={!amount || !collateralAmount || healthFactor < 150 || loading}
          className="w-full h-14 text-lg font-semibold"
          style={{
            background: healthFactor >= 150 ? '#00d9ff' : undefined,
            color: healthFactor >= 150 ? '#000' : undefined,
          }}
        >
          {loading ? 'Borrowing...' : 'Borrow'}
        </Button>
      </main>
    </div>
  )
}
