'use client'

import { Card } from '@/components/ui/card'
import { AllocationData } from '@/lib/types/analytics'

interface DiversificationChartProps {
  data: AllocationData[]
  recommendation?: string
}

export function DiversificationChart({
  data,
  recommendation = 'Consider adding more BTC for better diversification',
}: DiversificationChartProps) {
  const total = data.reduce((sum, item) => sum + item.value, 0)

  return (
    <Card className="p-6">
      <h3 className="text-lg font-semibold mb-4">Asset Allocation</h3>

      <div className="flex items-center justify-center mb-6">
        <svg viewBox="0 0 200 200" className="w-48 h-48">
          {data.map((item, index) => {
            const startAngle = data
              .slice(0, index)
              .reduce((sum, d) => sum + (d.percentage / 100) * 360, 0)
            const endAngle = startAngle + (item.percentage / 100) * 360

            const startRad = (startAngle * Math.PI) / 180
            const endRad = (endAngle * Math.PI) / 180

            const x1 = 100 + 80 * Math.cos(startRad)
            const y1 = 100 + 80 * Math.sin(startRad)
            const x2 = 100 + 80 * Math.cos(endRad)
            const y2 = 100 + 80 * Math.sin(endRad)

            const largeArc = item.percentage > 50 ? 1 : 0

            const path = [
              `M 100 100`,
              `L ${x1} ${y1}`,
              `A 80 80 0 ${largeArc} 1 ${x2} ${y2}`,
              'Z',
            ].join(' ')

            return (
              <path
                key={item.symbol}
                d={path}
                fill={item.color}
                stroke="white"
                strokeWidth="2"
              />
            )
          })}
        </svg>
      </div>

      {/* Legend */}
      <div className="space-y-2">
        {data.map((item) => (
          <div key={item.symbol} className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: item.color }}
              />
              <span className="text-sm font-medium">{item.symbol}</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-sm text-muted-foreground">
                ${item.value.toLocaleString()}
              </span>
              <span className="text-sm font-semibold">{item.percentage.toFixed(1)}%</span>
            </div>
          </div>
        ))}
      </div>

      {recommendation && (
        <div className="mt-4 p-3 bg-accent/10 rounded-lg">
          <p className="text-sm text-accent">{recommendation}</p>
        </div>
      )}
    </Card>
  )
}
