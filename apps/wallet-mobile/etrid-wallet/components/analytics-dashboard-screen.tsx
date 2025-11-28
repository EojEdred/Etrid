'use client'

import { ArrowLeft, TrendingUp, DollarSign, PieChart, AlertTriangle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { useAnalytics, useAssetAllocation } from '@/hooks/useAnalytics'
import { useRiskMetrics } from '@/hooks/useRiskMetrics'
import { PerformanceChart } from '@/components/analytics/PerformanceChart'
import { RiskScoreCard } from '@/components/analytics/RiskScoreCard'
import { DiversificationChart } from '@/components/analytics/DiversificationChart'
import { useState } from 'react'

interface AnalyticsDashboardScreenProps {
  onBack: () => void
  onNavigate: (screen: string) => void
}

export function AnalyticsDashboardScreen({
  onBack,
  onNavigate,
}: AnalyticsDashboardScreenProps) {
  const { metrics, loading: metricsLoading } = useAnalytics()
  const { allocation, loading: allocationLoading } = useAssetAllocation()
  const { riskMetrics, loading: riskLoading } = useRiskMetrics()

  if (metricsLoading || allocationLoading || riskLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading analytics...</p>
        </div>
      </div>
    )
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
            <h1 className="text-2xl font-bold">Portfolio Analytics</h1>
            <p className="text-sm text-muted-foreground">
              Advanced insights and metrics
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Key Metrics */}
        <div className="grid grid-cols-2 gap-4">
          <Card className="p-4">
            <div className="flex items-center gap-2 mb-2">
              <DollarSign className="w-4 h-4 text-muted-foreground" />
              <span className="text-xs text-muted-foreground">Total Value</span>
            </div>
            <p className="text-2xl font-bold">
              ${metrics?.totalValue.toLocaleString()}
            </p>
            <p
              className={`text-sm ${
                (metrics?.dayChangePercent || 0) >= 0
                  ? 'text-green-500'
                  : 'text-red-500'
              }`}
            >
              {(metrics?.dayChangePercent || 0) >= 0 ? '+' : ''}
              {metrics?.dayChangePercent.toFixed(2)}% (24h)
            </p>
          </Card>

          <Card className="p-4">
            <div className="flex items-center gap-2 mb-2">
              <TrendingUp className="w-4 h-4 text-muted-foreground" />
              <span className="text-xs text-muted-foreground">ROI</span>
            </div>
            <p className="text-2xl font-bold">{metrics?.roi.toFixed(1)}%</p>
            <p className="text-sm text-muted-foreground">All-time return</p>
          </Card>
        </div>

        {/* Performance Chart */}
        {metrics && (
          <PerformanceChart
            data={[
              { timestamp: Date.now() - 30 * 24 * 60 * 60 * 1000, value: 100000 },
              { timestamp: Date.now() - 20 * 24 * 60 * 60 * 1000, value: 105000 },
              { timestamp: Date.now() - 10 * 24 * 60 * 60 * 1000, value: 110000 },
              { timestamp: Date.now(), value: metrics.totalValue },
            ]}
            title="Portfolio Value (30d)"
          />
        )}

        {/* Risk Score */}
        {riskMetrics && (
          <RiskScoreCard
            score={riskMetrics.overallRiskScore}
            volatility={riskMetrics.volatility}
            concentration={0.45}
            liquidity={0.85}
          />
        )}

        {/* Asset Allocation */}
        {allocation.length > 0 && <DiversificationChart data={allocation} />}

        {/* Quick Links */}
        <div className="grid grid-cols-2 gap-4">
          <Button
            variant="outline"
            className="h-auto py-4 flex flex-col items-start"
            onClick={() => onNavigate('risk-metrics')}
          >
            <AlertTriangle className="w-5 h-5 mb-2 text-yellow-500" />
            <span className="font-semibold">Risk Analysis</span>
            <span className="text-xs text-muted-foreground">
              Detailed risk metrics
            </span>
          </Button>

          <Button
            variant="outline"
            className="h-auto py-4 flex flex-col items-start"
            onClick={() => onNavigate('tax-report')}
          >
            <DollarSign className="w-5 h-5 mb-2 text-green-500" />
            <span className="font-semibold">Tax Report</span>
            <span className="text-xs text-muted-foreground">
              Optimize your taxes
            </span>
          </Button>

          <Button
            variant="outline"
            className="h-auto py-4 flex flex-col items-start"
            onClick={() => onNavigate('performance')}
          >
            <TrendingUp className="w-5 h-5 mb-2 text-blue-500" />
            <span className="font-semibold">Performance</span>
            <span className="text-xs text-muted-foreground">
              Detailed attribution
            </span>
          </Button>

          <Button
            variant="outline"
            className="h-auto py-4 flex flex-col items-start"
            onClick={() => onNavigate('correlation')}
          >
            <PieChart className="w-5 h-5 mb-2 text-purple-500" />
            <span className="font-semibold">Correlations</span>
            <span className="text-xs text-muted-foreground">
              Asset relationships
            </span>
          </Button>
        </div>
      </main>
    </div>
  )
}
