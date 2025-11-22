"use client"

import { useState } from 'react'
import { ArrowLeft, Settings } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { TradingChart } from '@/components/trading/trading-chart'
import { OrderBook } from '@/components/trading/order-book'
import { OrderForm } from '@/components/trading/order-form'
import { PositionCard } from '@/components/trading/position-card'
import { IndicatorSelector } from '@/components/trading/indicator-selector'
import { useChart } from '@/hooks/use-chart'
import { useTrading } from '@/hooks/use-trading'
import { useOrderBook } from '@/hooks/use-orderbook'
import type { CandleInterval } from '@/lib/types/trading'

interface TradingScreenProps {
  pair?: string
  onBack: () => void
}

export function TradingScreen({ pair = 'ETRID/USDT', onBack }: TradingScreenProps) {
  const [interval, setInterval] = useState<CandleInterval>('1h')
  const [showIndicators, setShowIndicators] = useState(false)

  const {
    candles,
    currentPrice,
    loading: chartLoading,
    indicators,
    addIndicator,
    removeIndicator,
    toggleIndicator,
  } = useChart(pair, interval)

  const {
    openOrders,
    positions,
    placeOrder,
    cancelOrder,
    closePosition,
    setTPSL,
  } = useTrading(pair)

  const { orderBook, recentTrades, fillOrderFormFromBook } = useOrderBook(pair)

  const handleOrderSubmit = async (order: any) => {
    await placeOrder(order)
  }

  return (
    <div className="min-h-screen pb-24 bg-background">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <h1 className="text-lg font-bold">{pair}</h1>
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setShowIndicators(!showIndicators)}
          >
            <Settings className="w-5 h-5" />
          </Button>
        </div>
      </header>

      {/* Main Content */}
      <main className="space-y-4 p-4">
        {/* Chart */}
        <TradingChart
          candles={candles}
          currentPrice={currentPrice}
          pair={pair}
          interval={interval}
          onIntervalChange={setInterval}
        />

        {/* Indicators Panel */}
        {showIndicators && (
          <IndicatorSelector
            indicators={indicators}
            onAdd={addIndicator}
            onRemove={removeIndicator}
            onToggle={toggleIndicator}
            onUpdateParams={() => {}}
          />
        )}

        {/* Positions */}
        {positions.length > 0 && (
          <section className="space-y-2">
            <h2 className="text-sm font-semibold text-muted-foreground">Open Positions</h2>
            {positions.map((position) => (
              <PositionCard
                key={position.id}
                position={position}
                onClose={() => closePosition(position.id)}
                onSetTPSL={(tp, sl) => setTPSL(position.id, tp, sl)}
              />
            ))}
          </section>
        )}

        {/* Trading Interface */}
        <Tabs defaultValue="trade" className="w-full">
          <TabsList className="w-full grid grid-cols-3">
            <TabsTrigger value="trade">Trade</TabsTrigger>
            <TabsTrigger value="orderbook">Order Book</TabsTrigger>
            <TabsTrigger value="orders">Orders ({openOrders.length})</TabsTrigger>
          </TabsList>

          <TabsContent value="trade" className="mt-4">
            <OrderForm
              pair={pair}
              currentPrice={currentPrice}
              availableBalance={10000} // Mock balance
              currency="USDT"
              onSubmit={handleOrderSubmit}
            />
          </TabsContent>

          <TabsContent value="orderbook" className="mt-4">
            <OrderBook
              orderBook={orderBook}
              onPriceClick={(price, side) => {
                // Auto-fill order form
                console.log('Fill order:', price, side)
              }}
            />

            {/* Recent Trades */}
            <div className="mt-4">
              <h3 className="text-sm font-semibold mb-2">Recent Trades</h3>
              <div className="space-y-1 max-h-[200px] overflow-y-auto">
                {recentTrades.map((trade, index) => (
                  <div key={index} className="flex items-center justify-between text-xs p-2 hover:bg-accent/50 rounded">
                    <span className={trade.side === 'buy' ? 'text-green-500' : 'text-red-500'}>
                      {trade.price.toFixed(2)}
                    </span>
                    <span className="font-mono">{trade.amount.toFixed(4)}</span>
                    <span className="text-muted-foreground">
                      {new Date(trade.timestamp).toLocaleTimeString()}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </TabsContent>

          <TabsContent value="orders" className="mt-4 space-y-2">
            {openOrders.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">
                No open orders
              </div>
            ) : (
              openOrders.map((order) => (
                <div
                  key={order.id}
                  className="p-3 rounded-lg border flex items-center justify-between"
                >
                  <div>
                    <div className="flex items-center gap-2">
                      <span className={order.side === 'buy' ? 'text-green-500' : 'text-red-500'}>
                        {order.side.toUpperCase()}
                      </span>
                      <span className="text-xs text-muted-foreground">{order.type}</span>
                    </div>
                    <p className="text-sm font-semibold mt-1">
                      {order.amount} @ {order.price}
                    </p>
                  </div>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => cancelOrder(order.id)}
                  >
                    Cancel
                  </Button>
                </div>
              ))
            )}
          </TabsContent>
        </Tabs>
      </main>
    </div>
  )
}
