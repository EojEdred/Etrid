'use client'

import { ArrowLeft, TrendingUp, Award, TrendingDown } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { usePerformance } from '@/hooks/useAnalytics'
import { PerformanceChart } from '@/components/analytics/PerformanceChart'
import { TimePeriod } from '@/lib/types/analytics'
import { useState } from 'react'

interface PerformanceScreenProps {
  onBack: () => void
}

export function PerformanceScreen({ onBack }: PerformanceScreenProps) {
  const [period, setPeriod] = useState<TimePeriod>('30d')
  const { performance, loading } = usePerformance(period)

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading performance data...</p>
        </div>
      </div>
    )
  }

  if (!performance) {
    return null
  }

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Performance</h1>
            <p className="text-sm text-muted-foreground">
              Detailed attribution analysis
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Total Return */}
        <Card className="p-6">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <p className="text-xs text-muted-foreground mb-1">Total Return</p>
              <p className="text-3xl font-bold text-green-500">
                ${performance.totalReturn.toLocaleString()}
              </p>
              <p className="text-sm text-muted-foreground">
                +{performance.totalReturnPercent.toFixed(2)}%
              </p>
            </div>
            <div>
              <p className="text-xs text-muted-foreground mb-1">CAGR</p>
              <p className="text-3xl font-bold">{performance.cagr.toFixed(1)}%</p>
              <p className="text-sm text-muted-foreground">
                Compound annual growth rate
              </p>
            </div>
          </div>
        </Card>

        {/* Performance Chart */}
        <PerformanceChart data={performance.dailyReturns} title="Returns Over Time" />

        {/* Best/Worst Performers */}
        <div className="grid gap-4">
          <Card className="p-4">
            <div className="flex items-center gap-3 mb-3">
              <div className="w-10 h-10 rounded-full bg-green-500/10 flex items-center justify-center">
                <Award className="w-5 h-5 text-green-500" />
              </div>
              <div>
                <h3 className="font-semibold">Best Performer</h3>
                <p className="text-xs text-muted-foreground">{performance.bestAsset.asset}</p>
              </div>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">{performance.bestAsset.symbol}</span>
              <div className="text-right">
                <p className="font-bold text-green-500">
                  +${performance.bestAsset.return.toLocaleString()}
                </p>
                <p className="text-xs text-muted-foreground">
                  +{performance.bestAsset.returnPercent.toFixed(1)}%
                </p>
              </div>
            </div>
          </Card>

          <Card className="p-4">
            <div className="flex items-center gap-3 mb-3">
              <div className="w-10 h-10 rounded-full bg-red-500/10 flex items-center justify-center">
                <TrendingDown className="w-5 h-5 text-red-500" />
              </div>
              <div>
                <h3 className="font-semibold">Worst Performer</h3>
                <p className="text-xs text-muted-foreground">{performance.worstAsset.asset}</p>
              </div>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">{performance.worstAsset.symbol}</span>
              <div className="text-right">
                <p className="font-bold text-red-500">
                  ${performance.worstAsset.return.toLocaleString()}
                </p>
                <p className="text-xs text-muted-foreground">
                  {performance.worstAsset.returnPercent.toFixed(1)}%
                </p>
              </div>
            </div>
          </Card>
        </div>

        {/* Benchmark Comparison */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">Benchmark Comparison</h3>
          <div className="space-y-3">
            {[
              { name: 'Bitcoin', symbol: 'BTC', return: 12.3 },
              { name: 'Ethereum', symbol: 'ETH', return: 18.7 },
              { name: 'S&P 500', symbol: 'SPX', return: 8.5 },
            ].map((benchmark) => {
              const outperformance = performance.totalReturnPercent - benchmark.return
              return (
                <div key={benchmark.symbol} className="flex items-center justify-between py-2 border-b last:border-0">
                  <div>
                    <p className="font-medium">{benchmark.name}</p>
                    <p className="text-xs text-muted-foreground">
                      {benchmark.return.toFixed(1)}%
                    </p>
                  </div>
                  <div className="text-right">
                    <p
                      className={`font-semibold ${
                        outperformance >= 0 ? 'text-green-500' : 'text-red-500'
                      }`}
                    >
                      {outperformance >= 0 ? '+' : ''}
                      {outperformance.toFixed(1)}%
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {outperformance >= 0 ? 'Outperforming' : 'Underperforming'}
                    </p>
                  </div>
                </div>
              )
            })}
          </div>
        </Card>
      </main>
    </div>
  )
}
