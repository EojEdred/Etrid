'use client'

import { Card } from '@/components/ui/card'
import { TimeSeriesData } from '@/lib/types/analytics'
import { useState } from 'react'

interface PerformanceChartProps {
  data: TimeSeriesData[]
  title?: string
  showBenchmark?: boolean
  benchmarkData?: TimeSeriesData[]
}

export function PerformanceChart({
  data,
  title = 'Portfolio Performance',
  showBenchmark = false,
  benchmarkData = [],
}: PerformanceChartProps) {
  const [timeframe, setTimeframe] = useState<'24h' | '7d' | '30d' | '1y'>('30d')

  if (!data || data.length === 0) {
    return null
  }

  const minValue = Math.min(...data.map((d) => d.value))
  const maxValue = Math.max(...data.map((d) => d.value))
  const range = maxValue - minValue
  const padding = range * 0.1

  const chartHeight = 200
  const chartWidth = 800

  const points = data.map((point, index) => {
    const x = (index / (data.length - 1)) * chartWidth
    const y = chartHeight - ((point.value - minValue + padding) / (range + padding * 2)) * chartHeight
    return `${x},${y}`
  })

  const pathData = `M ${points.join(' L ')}`

  const areaData = `M 0,${chartHeight} L ${points.join(' L ')} L ${chartWidth},${chartHeight} Z`

  return (
    <Card className="p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold">{title}</h3>
        <div className="flex gap-1">
          {(['24h', '7d', '30d', '1y'] as const).map((tf) => (
            <button
              key={tf}
              onClick={() => setTimeframe(tf)}
              className={`px-3 py-1 text-xs rounded ${
                timeframe === tf
                  ? 'bg-accent text-accent-foreground'
                  : 'text-muted-foreground hover:bg-muted'
              }`}
            >
              {tf}
            </button>
          ))}
        </div>
      </div>

      <div className="overflow-x-auto">
        <svg
          viewBox={`0 0 ${chartWidth} ${chartHeight}`}
          className="w-full h-48"
          preserveAspectRatio="none"
        >
          {/* Grid lines */}
          {[0, 0.25, 0.5, 0.75, 1].map((fraction) => (
            <line
              key={fraction}
              x1="0"
              y1={chartHeight * fraction}
              x2={chartWidth}
              y2={chartHeight * fraction}
              stroke="currentColor"
              strokeWidth="1"
              className="text-gray-200 dark:text-gray-700"
              opacity="0.5"
            />
          ))}

          {/* Area fill */}
          <path
            d={areaData}
            fill="url(#gradient)"
            opacity="0.2"
          />

          {/* Line */}
          <path
            d={pathData}
            fill="none"
            stroke="currentColor"
            strokeWidth="3"
            className="text-accent"
            strokeLinecap="round"
            strokeLinejoin="round"
          />

          {/* Gradient definition */}
          <defs>
            <linearGradient id="gradient" x1="0" x2="0" y1="0" y2="1">
              <stop offset="0%" stopColor="currentColor" className="text-accent" />
              <stop offset="100%" stopColor="currentColor" className="text-accent" stopOpacity="0" />
            </linearGradient>
          </defs>
        </svg>
      </div>

      {/* Value labels */}
      <div className="flex items-center justify-between mt-4 text-sm">
        <div>
          <span className="text-muted-foreground">Min: </span>
          <span className="font-semibold">${minValue.toLocaleString()}</span>
        </div>
        <div>
          <span className="text-muted-foreground">Current: </span>
          <span className="font-semibold">${data[data.length - 1].value.toLocaleString()}</span>
        </div>
        <div>
          <span className="text-muted-foreground">Max: </span>
          <span className="font-semibold">${maxValue.toLocaleString()}</span>
        </div>
      </div>
    </Card>
  )
}
