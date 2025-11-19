'use client'

import { ArrowLeft, Info } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { CorrelationMatrix } from '@/components/analytics/CorrelationMatrix'

interface CorrelationScreenProps {
  onBack: () => void
}

export function CorrelationScreen({ onBack }: CorrelationScreenProps) {
  // Mock correlation data
  const correlationData = {
    assets: ['ÉTR', 'BTC', 'ETH', 'DOT', 'ADA'],
    matrix: [
      [1.0, 0.65, 0.72, 0.58, 0.52],
      [0.65, 1.0, 0.85, 0.68, 0.61],
      [0.72, 0.85, 1.0, 0.73, 0.67],
      [0.58, 0.68, 0.73, 1.0, 0.78],
      [0.52, 0.61, 0.67, 0.78, 1.0],
    ],
  }

  const highlyCorrelated = [
    { asset1: 'BTC', asset2: 'ETH', correlation: 0.85 },
    { asset1: 'DOT', asset2: 'ADA', correlation: 0.78 },
  ]

  const recommendations = [
    'BTC and ETH are highly correlated (0.85) - consider diversifying',
    'DOT and ADA move together - may not provide true diversification',
    'ÉTR has moderate correlation with major assets - good for balance',
  ]

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Asset Correlations</h1>
            <p className="text-sm text-muted-foreground">
              Analyze asset relationships
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Correlation Matrix */}
        <CorrelationMatrix data={correlationData} />

        {/* Highly Correlated Pairs */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">Highly Correlated Pairs</h3>
          <div className="space-y-3">
            {highlyCorrelated.map((pair) => (
              <div
                key={`${pair.asset1}-${pair.asset2}`}
                className="flex items-center justify-between p-3 bg-muted/50 rounded-lg"
              >
                <div className="flex items-center gap-2">
                  <span className="font-medium">{pair.asset1}</span>
                  <span className="text-muted-foreground">↔</span>
                  <span className="font-medium">{pair.asset2}</span>
                </div>
                <span className="font-bold text-red-500">
                  {pair.correlation.toFixed(2)}
                </span>
              </div>
            ))}
          </div>
        </Card>

        {/* Diversification Recommendations */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
            <Info className="w-5 h-5 text-accent" />
            Recommendations
          </h3>
          <div className="space-y-2">
            {recommendations.map((rec, index) => (
              <div
                key={index}
                className="flex items-start gap-2 p-3 bg-muted/50 rounded-lg"
              >
                <div className="w-1.5 h-1.5 rounded-full bg-accent mt-2" />
                <p className="text-sm">{rec}</p>
              </div>
            ))}
          </div>
        </Card>

        {/* Rebalancing Suggestions */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">
            Portfolio Rebalancing Suggestions
          </h3>
          <div className="space-y-4">
            <div className="p-4 bg-green-500/10 border border-green-500/20 rounded-lg">
              <p className="font-semibold text-green-500 mb-2">Increase Allocation</p>
              <p className="text-sm">
                Consider adding more non-correlated assets like stablecoins or
                commodities to reduce portfolio volatility.
              </p>
            </div>
            <div className="p-4 bg-yellow-500/10 border border-yellow-500/20 rounded-lg">
              <p className="font-semibold text-yellow-500 mb-2">
                Reduce Concentration
              </p>
              <p className="text-sm">
                BTC and ETH together represent high correlation risk. Consider
                reducing one position.
              </p>
            </div>
          </div>
        </Card>
      </main>
    </div>
  )
}
