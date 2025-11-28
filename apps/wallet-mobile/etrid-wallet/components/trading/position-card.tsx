"use client"

import { TrendingUp, TrendingDown, X } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import type { Position } from '@/lib/types/trading'

interface PositionCardProps {
  position: Position
  onClose?: () => void
  onSetTPSL?: (takeProfit?: number, stopLoss?: number) => void
  className?: string
}

export function PositionCard({
  position,
  onClose,
  onSetTPSL,
  className,
}: PositionCardProps) {
  const isProfitable = position.unrealized_pnl >= 0

  return (
    <Card className={cn('p-4 space-y-3', className)}>
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <div className="flex items-center gap-2">
            <h3 className="font-semibold">{position.pair}</h3>
            <Badge
              variant={position.side === 'buy' ? 'default' : 'destructive'}
              className="text-xs"
            >
              {position.side === 'buy' ? 'Long' : 'Short'}
            </Badge>
            {position.leverage && (
              <Badge variant="outline" className="text-xs">
                {position.leverage}x
              </Badge>
            )}
          </div>
          <p className="text-xs text-muted-foreground mt-1">
            Opened {new Date(position.opened_at).toLocaleDateString()}
          </p>
        </div>

        {onClose && (
          <Button
            variant="ghost"
            size="icon-sm"
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground"
          >
            <X className="w-4 h-4" />
          </Button>
        )}
      </div>

      {/* Price Info */}
      <div className="grid grid-cols-3 gap-3">
        <div>
          <p className="text-xs text-muted-foreground">Entry Price</p>
          <p className="font-semibold text-sm">${position.entry_price.toLocaleString()}</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">Current Price</p>
          <p className="font-semibold text-sm">${position.current_price.toLocaleString()}</p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">Amount</p>
          <p className="font-semibold text-sm">{position.amount.toFixed(6)}</p>
        </div>
      </div>

      {/* P&L */}
      <div className="p-3 rounded-lg bg-muted/50">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            {isProfitable ? (
              <TrendingUp className="w-4 h-4 text-green-500" />
            ) : (
              <TrendingDown className="w-4 h-4 text-red-500" />
            )}
            <span className="text-sm text-muted-foreground">Unrealized P&L</span>
          </div>
          <div className="text-right">
            <p
              className={cn(
                'font-bold',
                isProfitable ? 'text-green-500' : 'text-red-500'
              )}
            >
              {isProfitable ? '+' : ''}${position.unrealized_pnl.toFixed(2)}
            </p>
            <p
              className={cn(
                'text-xs',
                isProfitable ? 'text-green-500' : 'text-red-500'
              )}
            >
              {isProfitable ? '+' : ''}{position.unrealized_pnl_percentage.toFixed(2)}%
            </p>
          </div>
        </div>
      </div>

      {/* TP/SL */}
      {(position.take_profit || position.stop_loss) && (
        <div className="grid grid-cols-2 gap-3 text-xs">
          {position.take_profit && (
            <div>
              <p className="text-muted-foreground">Take Profit</p>
              <p className="font-semibold text-green-500">
                ${position.take_profit.toLocaleString()}
              </p>
            </div>
          )}
          {position.stop_loss && (
            <div>
              <p className="text-muted-foreground">Stop Loss</p>
              <p className="font-semibold text-red-500">
                ${position.stop_loss.toLocaleString()}
              </p>
            </div>
          )}
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-2">
        {onSetTPSL && (
          <Button variant="outline" size="sm" className="flex-1" onClick={() => onSetTPSL()}>
            Set TP/SL
          </Button>
        )}
        {onClose && (
          <Button variant="destructive" size="sm" className="flex-1" onClick={onClose}>
            Close Position
          </Button>
        )}
      </div>
    </Card>
  )
}
