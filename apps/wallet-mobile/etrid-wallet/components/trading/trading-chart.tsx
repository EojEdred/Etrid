"use client"

import { useEffect, useRef, useState } from 'react'
import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { Candle, CandleInterval, ChartType } from '@/lib/types/trading'

interface TradingChartProps {
  candles: Candle[]
  currentPrice: number
  pair: string
  interval: CandleInterval
  onIntervalChange?: (interval: CandleInterval) => void
  chartType?: ChartType
  className?: string
}

const INTERVALS: CandleInterval[] = ['1m', '5m', '15m', '1h', '4h', '1d', '1w']

export function TradingChart({
  candles,
  currentPrice,
  pair,
  interval,
  onIntervalChange,
  chartType = 'candlestick',
  className,
}: TradingChartProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [hoveredCandle, setHoveredCandle] = useState<Candle | null>(null)

  useEffect(() => {
    if (!canvasRef.current || candles.length === 0) return

    const canvas = canvasRef.current
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Set canvas size
    canvas.width = canvas.offsetWidth * window.devicePixelRatio
    canvas.height = canvas.offsetHeight * window.devicePixelRatio
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio)

    // Clear canvas
    ctx.clearRect(0, 0, canvas.offsetWidth, canvas.offsetHeight)

    // Calculate dimensions
    const padding = 40
    const chartWidth = canvas.offsetWidth - padding * 2
    const chartHeight = canvas.offsetHeight - padding * 2

    // Find min/max prices
    const prices = candles.flatMap((c) => [c.high, c.low])
    const minPrice = Math.min(...prices)
    const maxPrice = Math.max(...prices)
    const priceRange = maxPrice - minPrice

    // Draw grid
    ctx.strokeStyle = '#333'
    ctx.lineWidth = 1
    for (let i = 0; i <= 5; i++) {
      const y = padding + (chartHeight / 5) * i
      ctx.beginPath()
      ctx.moveTo(padding, y)
      ctx.lineTo(padding + chartWidth, y)
      ctx.stroke()

      // Price labels
      const price = maxPrice - (priceRange / 5) * i
      ctx.fillStyle = '#888'
      ctx.font = '12px monospace'
      ctx.fillText(price.toFixed(2), padding + chartWidth + 5, y + 4)
    }

    // Draw candles
    const candleWidth = chartWidth / candles.length
    const candleSpacing = candleWidth * 0.2

    candles.forEach((candle, index) => {
      const x = padding + index * candleWidth + candleSpacing / 2
      const width = candleWidth - candleSpacing

      const openY =
        padding + chartHeight - ((candle.open - minPrice) / priceRange) * chartHeight
      const closeY =
        padding + chartHeight - ((candle.close - minPrice) / priceRange) * chartHeight
      const highY =
        padding + chartHeight - ((candle.high - minPrice) / priceRange) * chartHeight
      const lowY =
        padding + chartHeight - ((candle.low - minPrice) / priceRange) * chartHeight

      const isBullish = candle.close >= candle.open

      // Draw wick
      ctx.strokeStyle = isBullish ? '#10b981' : '#ef4444'
      ctx.lineWidth = 1
      ctx.beginPath()
      ctx.moveTo(x + width / 2, highY)
      ctx.lineTo(x + width / 2, lowY)
      ctx.stroke()

      // Draw body
      ctx.fillStyle = isBullish ? '#10b981' : '#ef4444'
      const bodyHeight = Math.abs(closeY - openY) || 1
      ctx.fillRect(x, Math.min(openY, closeY), width, bodyHeight)
    })

    // Draw current price line
    const currentPriceY =
      padding + chartHeight - ((currentPrice - minPrice) / priceRange) * chartHeight
    ctx.strokeStyle = '#3b82f6'
    ctx.lineWidth = 2
    ctx.setLineDash([5, 5])
    ctx.beginPath()
    ctx.moveTo(padding, currentPriceY)
    ctx.lineTo(padding + chartWidth, currentPriceY)
    ctx.stroke()
    ctx.setLineDash([])

    // Current price label
    ctx.fillStyle = '#3b82f6'
    ctx.fillRect(padding + chartWidth + 2, currentPriceY - 10, 60, 20)
    ctx.fillStyle = '#fff'
    ctx.font = 'bold 12px monospace'
    ctx.fillText(
      currentPrice.toFixed(2),
      padding + chartWidth + 5,
      currentPriceY + 4
    )
  }, [candles, currentPrice])

  const latestCandle = candles[candles.length - 1]
  const priceChange = latestCandle
    ? ((latestCandle.close - candles[0].open) / candles[0].open) * 100
    : 0

  return (
    <Card className={cn('overflow-hidden', className)}>
      {/* Header */}
      <div className="p-4 border-b flex items-center justify-between">
        <div>
          <h3 className="font-semibold">{pair}</h3>
          <div className="flex items-center gap-2 mt-1">
            <span className="text-2xl font-bold">${currentPrice.toFixed(2)}</span>
            <span
              className={cn(
                'text-sm font-semibold',
                priceChange >= 0 ? 'text-green-500' : 'text-red-500'
              )}
            >
              {priceChange >= 0 ? '+' : ''}
              {priceChange.toFixed(2)}%
            </span>
          </div>
        </div>

        {/* Interval Selector */}
        <Tabs value={interval} onValueChange={(v) => onIntervalChange?.(v as CandleInterval)}>
          <TabsList className="h-8">
            {INTERVALS.map((int) => (
              <TabsTrigger key={int} value={int} className="text-xs px-2 h-7">
                {int}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>
      </div>

      {/* Chart */}
      <div className="relative h-[400px] p-4">
        <canvas
          ref={canvasRef}
          className="w-full h-full"
        />
      </div>

      {/* Hovered Candle Info */}
      {hoveredCandle && (
        <div className="p-4 border-t bg-muted/50">
          <div className="grid grid-cols-5 gap-4 text-sm">
            <div>
              <span className="text-muted-foreground">O:</span>{' '}
              <span className="font-mono">{hoveredCandle.open.toFixed(2)}</span>
            </div>
            <div>
              <span className="text-muted-foreground">H:</span>{' '}
              <span className="font-mono">{hoveredCandle.high.toFixed(2)}</span>
            </div>
            <div>
              <span className="text-muted-foreground">L:</span>{' '}
              <span className="font-mono">{hoveredCandle.low.toFixed(2)}</span>
            </div>
            <div>
              <span className="text-muted-foreground">C:</span>{' '}
              <span className="font-mono">{hoveredCandle.close.toFixed(2)}</span>
            </div>
            <div>
              <span className="text-muted-foreground">V:</span>{' '}
              <span className="font-mono">{hoveredCandle.volume.toFixed(2)}</span>
            </div>
          </div>
        </div>
      )}
    </Card>
  )
}
