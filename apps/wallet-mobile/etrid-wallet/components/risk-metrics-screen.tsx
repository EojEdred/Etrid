'use client'

import { ArrowLeft, TrendingDown, TrendingUp, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { useRiskMetrics } from '@/hooks/useRiskMetrics'
import { RiskScoreCard } from '@/components/analytics/RiskScoreCard'

interface RiskMetricsScreenProps {
  onBack: () => void
}

export function RiskMetricsScreen({ onBack }: RiskMetricsScreenProps) {
  const { riskMetrics, loading } = useRiskMetrics()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading risk metrics...</p>
        </div>
      </div>
    )
  }

  if (!riskMetrics) {
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
            <h1 className="text-2xl font-bold">Risk Analysis</h1>
            <p className="text-sm text-muted-foreground">
              Comprehensive risk metrics
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Overall Risk Score */}
        <RiskScoreCard
          score={riskMetrics.overallRiskScore}
          volatility={riskMetrics.volatility}
        />

        {/* Risk Metrics Grid */}
        <div className="grid gap-4">
          <Card className="p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <TrendingDown className="w-4 h-4 text-muted-foreground" />
                <span className="text-sm font-semibold">Volatility</span>
              </div>
              <span className="text-2xl font-bold">
                {(riskMetrics.volatility * 100).toFixed(1)}%
              </span>
            </div>
            <p className="text-xs text-muted-foreground">
              Annualized standard deviation of returns
            </p>
          </Card>

          <Card className="p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <TrendingUp className="w-4 h-4 text-muted-foreground" />
                <span className="text-sm font-semibold">Sharpe Ratio</span>
              </div>
              <span className="text-2xl font-bold">
                {riskMetrics.sharpeRatio.toFixed(2)}
              </span>
            </div>
            <p className="text-xs text-muted-foreground">
              Risk-adjusted return (higher is better)
            </p>
            <div className="mt-2 text-xs">
              {riskMetrics.sharpeRatio > 2 && (
                <span className="text-green-500">Excellent</span>
              )}
              {riskMetrics.sharpeRatio > 1 && riskMetrics.sharpeRatio <= 2 && (
                <span className="text-blue-500">Good</span>
              )}
              {riskMetrics.sharpeRatio <= 1 && (
                <span className="text-yellow-500">Fair</span>
              )}
            </div>
          </Card>

          <Card className="p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <TrendingUp className="w-4 h-4 text-muted-foreground" />
                <span className="text-sm font-semibold">Beta (vs BTC)</span>
              </div>
              <span className="text-2xl font-bold">
                {riskMetrics.beta.toFixed(2)}
              </span>
            </div>
            <p className="text-xs text-muted-foreground">
              Correlation with Bitcoin market movements
            </p>
            <div className="mt-2 text-xs">
              {riskMetrics.beta > 1 ? (
                <span className="text-orange-500">More volatile than BTC</span>
              ) : (
                <span className="text-green-500">Less volatile than BTC</span>
              )}
            </div>
          </Card>

          <Card className="p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <TrendingDown className="w-4 h-4 text-muted-foreground" />
                <span className="text-sm font-semibold">Max Drawdown</span>
              </div>
              <span className="text-2xl font-bold text-red-500">
                {riskMetrics.maxDrawdown.toFixed(1)}%
              </span>
            </div>
            <p className="text-xs text-muted-foreground">
              Largest peak-to-trough decline
            </p>
          </Card>

          <Card className="p-4">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-2">
                <AlertCircle className="w-4 h-4 text-muted-foreground" />
                <span className="text-sm font-semibold">Value at Risk (95%)</span>
              </div>
              <span className="text-2xl font-bold text-red-500">
                ${Math.abs(riskMetrics.valueAtRisk).toLocaleString()}
              </span>
            </div>
            <p className="text-xs text-muted-foreground">
              Potential loss in worst 5% of days
            </p>
          </Card>
        </div>

        {/* Recommendations */}
        <Card className="p-4">
          <h3 className="text-lg font-semibold mb-3 flex items-center gap-2">
            <AlertCircle className="w-5 h-5 text-accent" />
            Risk Recommendations
          </h3>
          <div className="space-y-2">
            {riskMetrics.recommendations.map((rec, index) => (
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
      </main>
    </div>
  )
}
