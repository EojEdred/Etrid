"use client"

import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { OrderBook as OrderBookType } from '@/lib/types/trading'

interface OrderBookProps {
  orderBook: OrderBookType | null
  onPriceClick?: (price: number, side: 'buy' | 'sell') => void
  className?: string
}

export function OrderBook({
  orderBook,
  onPriceClick,
  className,
}: OrderBookProps) {
  if (!orderBook) {
    return (
      <Card className={cn('p-6', className)}>
        <div className="text-center text-muted-foreground">
          Loading order book...
        </div>
      </Card>
    )
  }

  const renderOrderLevel = (
    price: number,
    amount: number,
    total: number,
    side: 'buy' | 'sell',
    maxTotal: number
  ) => {
    const fillPercentage = (total / maxTotal) * 100

    return (
      <button
        onClick={() => onPriceClick?.(price, side)}
        className={cn(
          'relative w-full p-2 hover:bg-accent/50 transition-colors text-left',
          'grid grid-cols-3 gap-2 text-xs'
        )}
      >
        {/* Background depth indicator */}
        <div
          className={cn(
            'absolute inset-0 opacity-10',
            side === 'buy' ? 'bg-green-500' : 'bg-red-500'
          )}
          style={{
            width: `${fillPercentage}%`,
            right: side === 'buy' ? 0 : 'auto',
            left: side === 'sell' ? 0 : 'auto',
          }}
        />

        <span className={cn('font-mono', side === 'buy' ? 'text-green-500' : 'text-red-500')}>
          {price.toLocaleString()}
        </span>
        <span className="font-mono text-right">{amount.toFixed(4)}</span>
        <span className="font-mono text-right text-muted-foreground">
          {total.toFixed(2)}
        </span>
      </button>
    )
  }

  const maxTotal = Math.max(
    ...orderBook.bids.map((b) => b.total),
    ...orderBook.asks.map((a) => a.total)
  )

  return (
    <Card className={cn('overflow-hidden', className)}>
      <Tabs defaultValue="both" className="w-full">
        <TabsList className="w-full grid grid-cols-3">
          <TabsTrigger value="buy" className="text-xs">Buy</TabsTrigger>
          <TabsTrigger value="both" className="text-xs">Both</TabsTrigger>
          <TabsTrigger value="sell" className="text-xs">Sell</TabsTrigger>
        </TabsList>

        {/* Header */}
        <div className="grid grid-cols-3 gap-2 px-2 py-2 bg-muted/50 text-xs text-muted-foreground">
          <span>Price</span>
          <span className="text-right">Amount</span>
          <span className="text-right">Total</span>
        </div>

        <TabsContent value="both" className="mt-0 space-y-0">
          {/* Asks (Sell orders) - Reversed to show lowest first */}
          <div className="max-h-[200px] overflow-y-auto">
            {[...orderBook.asks].reverse().slice(0, 10).map((ask, index) => (
              <div key={`ask-${index}`}>
                {renderOrderLevel(
                  ask.price,
                  ask.amount,
                  ask.total,
                  'sell',
                  maxTotal
                )}
              </div>
            ))}
          </div>

          {/* Spread */}
          <div className="py-3 px-2 bg-accent/20 text-center">
            <div className="text-xs text-muted-foreground">Spread</div>
            <div className="font-semibold text-sm">
              {orderBook.spread.toFixed(2)} ({orderBook.spread_percentage.toFixed(2)}%)
            </div>
          </div>

          {/* Bids (Buy orders) */}
          <div className="max-h-[200px] overflow-y-auto">
            {orderBook.bids.slice(0, 10).map((bid, index) => (
              <div key={`bid-${index}`}>
                {renderOrderLevel(
                  bid.price,
                  bid.amount,
                  bid.total,
                  'buy',
                  maxTotal
                )}
              </div>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="buy" className="mt-0">
          <div className="max-h-[400px] overflow-y-auto">
            {orderBook.bids.map((bid, index) => (
              <div key={`bid-${index}`}>
                {renderOrderLevel(
                  bid.price,
                  bid.amount,
                  bid.total,
                  'buy',
                  maxTotal
                )}
              </div>
            ))}
          </div>
        </TabsContent>

        <TabsContent value="sell" className="mt-0">
          <div className="max-h-[400px] overflow-y-auto">
            {orderBook.asks.map((ask, index) => (
              <div key={`ask-${index}`}>
                {renderOrderLevel(
                  ask.price,
                  ask.amount,
                  ask.total,
                  'sell',
                  maxTotal
                )}
              </div>
            ))}
          </div>
        </TabsContent>
      </Tabs>
    </Card>
  )
}
