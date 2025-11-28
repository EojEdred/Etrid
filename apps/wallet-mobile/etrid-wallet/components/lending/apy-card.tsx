/**
 * APYCard - Display supply/borrow rates for assets
 */

'use client'

import { TrendingUp, TrendingDown } from 'lucide-react'
import { Button } from '@/components/ui/button'

interface APYCardProps {
  asset: string
  icon?: string
  apy: number
  type: 'supply' | 'borrow'
  totalAmount: number
  yourPosition?: number
  onAction?: () => void
}

export function APYCard({
  asset,
  icon,
  apy,
  type,
  totalAmount,
  yourPosition = 0,
  onAction,
}: APYCardProps) {
  const isSupply = type === 'supply'

  return (
    <div className="glass rounded-2xl p-5 space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-12 h-12 rounded-full bg-accent/20 flex items-center justify-center">
            <span className="text-2xl">{icon || '💎'}</span>
          </div>
          <div>
            <h3 className="font-semibold text-lg">{asset}</h3>
            <p className="text-xs text-muted-foreground">
              {isSupply ? 'Supply to earn' : 'Borrow at'}
            </p>
          </div>
        </div>

        <div className="text-right">
          <div className={`flex items-center gap-1 ${isSupply ? 'text-success' : 'text-destructive'}`}>
            {isSupply ? (
              <TrendingUp className="w-4 h-4" />
            ) : (
              <TrendingDown className="w-4 h-4" />
            )}
            <span className="text-2xl font-bold">{apy.toFixed(2)}%</span>
          </div>
          <p className="text-xs text-muted-foreground">APY</p>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4 pt-3 border-t border-border">
        <div>
          <p className="text-xs text-muted-foreground mb-1">
            Total {isSupply ? 'Supplied' : 'Borrowed'}
          </p>
          <p className="font-semibold">{totalAmount.toLocaleString()} {asset}</p>
        </div>

        {yourPosition > 0 && (
          <div>
            <p className="text-xs text-muted-foreground mb-1">Your Position</p>
            <p className="font-semibold">{yourPosition.toLocaleString()} {asset}</p>
          </div>
        )}
      </div>

      {onAction && (
        <Button
          onClick={onAction}
          className="w-full"
          variant={isSupply ? 'default' : 'outline'}
          style={
            isSupply
              ? {
                  background: '#00d9ff',
                  color: '#000',
                }
              : undefined
          }
        >
          {isSupply ? 'Supply' : 'Borrow'}
        </Button>
      )}
    </div>
  )
}
