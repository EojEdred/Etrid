'use client'

import { Card } from '@/components/ui/card'
import { CorrelationMatrix as CorrelationMatrixType } from '@/lib/types/analytics'

interface CorrelationMatrixProps {
  data: CorrelationMatrixType
}

export function CorrelationMatrix({ data }: CorrelationMatrixProps) {
  const getColorForCorrelation = (value: number): string => {
    if (value > 0.7) return 'bg-red-500'
    if (value > 0.4) return 'bg-orange-500'
    if (value > 0) return 'bg-yellow-500'
    if (value > -0.4) return 'bg-blue-300'
    if (value > -0.7) return 'bg-blue-500'
    return 'bg-blue-700'
  }

  const getTextColor = (value: number): string => {
    return Math.abs(value) > 0.4 ? 'text-white' : 'text-gray-900'
  }

  return (
    <Card className="p-6">
      <h3 className="text-lg font-semibold mb-4">Asset Correlation Matrix</h3>

      <div className="overflow-x-auto">
        <div className="inline-block min-w-full">
          <div className="grid gap-1" style={{ gridTemplateColumns: `80px repeat(${data.assets.length}, 60px)` }}>
            {/* Header row */}
            <div></div>
            {data.assets.map((asset) => (
              <div
                key={asset}
                className="text-xs font-semibold text-center p-2"
              >
                {asset}
              </div>
            ))}

            {/* Data rows */}
            {data.assets.map((asset1, i) => (
              <div key={asset1} className="contents">
                <div className="text-xs font-semibold p-2 flex items-center">
                  {asset1}
                </div>
                {data.matrix[i].map((correlation, j) => {
                  const asset2 = data.assets[j]
                  return (
                    <div
                      key={`${asset1}-${asset2}`}
                      className={`
                        ${getColorForCorrelation(correlation)}
                        ${getTextColor(correlation)}
                        text-xs font-semibold text-center p-2 rounded
                        ${i === j ? 'opacity-50' : ''}
                      `}
                      title={`${asset1} vs ${asset2}: ${correlation.toFixed(2)}`}
                    >
                      {correlation.toFixed(2)}
                    </div>
                  )
                })}
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Legend */}
      <div className="mt-4 flex items-center justify-center gap-4 text-xs">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-blue-700 rounded" />
          <span>Strong Negative</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-yellow-500 rounded" />
          <span>Weak</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 bg-red-500 rounded" />
          <span>Strong Positive</span>
        </div>
      </div>

      <div className="mt-4 p-3 bg-muted/50 rounded-lg text-xs text-muted-foreground">
        <p>
          Correlation ranges from -1 (perfect negative) to +1 (perfect positive).
          Diversify with negatively correlated assets to reduce risk.
        </p>
      </div>
    </Card>
  )
}
