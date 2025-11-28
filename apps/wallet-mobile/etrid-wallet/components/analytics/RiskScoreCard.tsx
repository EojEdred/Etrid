'use client'

import { Card } from '@/components/ui/card'
import { AlertTriangle, TrendingUp, ShieldCheck } from 'lucide-react'

interface RiskScoreCardProps {
  score: number
  volatility?: number
  concentration?: number
  liquidity?: number
}

export function RiskScoreCard({
  score,
  volatility = 0.35,
  concentration = 0.45,
  liquidity = 0.85,
}: RiskScoreCardProps) {
  const getRiskLevel = (score: number): 'low' | 'medium' | 'high' => {
    if (score < 33) return 'low'
    if (score < 67) return 'medium'
    return 'high'
  }

  const getRiskColor = (level: string) => {
    switch (level) {
      case 'low':
        return 'text-green-500'
      case 'medium':
        return 'text-yellow-500'
      case 'high':
        return 'text-red-500'
      default:
        return 'text-gray-500'
    }
  }

  const riskLevel = getRiskLevel(score)
  const colorClass = getRiskColor(riskLevel)

  return (
    <Card className="p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold">Portfolio Risk Score</h3>
        {riskLevel === 'low' && <ShieldCheck className="w-5 h-5 text-green-500" />}
        {riskLevel === 'medium' && <TrendingUp className="w-5 h-5 text-yellow-500" />}
        {riskLevel === 'high' && <AlertTriangle className="w-5 h-5 text-red-500" />}
      </div>

      {/* Circular Gauge */}
      <div className="flex items-center justify-center mb-6">
        <div className="relative w-40 h-40">
          <svg className="transform -rotate-90 w-40 h-40">
            <circle
              cx="80"
              cy="80"
              r="70"
              stroke="currentColor"
              strokeWidth="12"
              fill="transparent"
              className="text-gray-200 dark:text-gray-700"
            />
            <circle
              cx="80"
              cy="80"
              r="70"
              stroke="currentColor"
              strokeWidth="12"
              fill="transparent"
              strokeDasharray={`${2 * Math.PI * 70}`}
              strokeDashoffset={`${2 * Math.PI * 70 * (1 - score / 100)}`}
              className={colorClass}
              strokeLinecap="round"
            />
          </svg>
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <span className={`text-4xl font-bold ${colorClass}`}>{score}</span>
            <span className="text-sm text-muted-foreground uppercase">
              {riskLevel} Risk
            </span>
          </div>
        </div>
      </div>

      {/* Breakdown */}
      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Volatility</span>
          <div className="flex items-center gap-2">
            <div className="w-24 h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
              <div
                className="h-full bg-accent"
                style={{ width: `${volatility * 100}%` }}
              />
            </div>
            <span className="text-sm font-medium">{(volatility * 100).toFixed(0)}%</span>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Concentration</span>
          <div className="flex items-center gap-2">
            <div className="w-24 h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
              <div
                className="h-full bg-accent"
                style={{ width: `${concentration * 100}%` }}
              />
            </div>
            <span className="text-sm font-medium">
              {(concentration * 100).toFixed(0)}%
            </span>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <span className="text-sm text-muted-foreground">Liquidity</span>
          <div className="flex items-center gap-2">
            <div className="w-24 h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
              <div
                className="h-full bg-green-500"
                style={{ width: `${liquidity * 100}%` }}
              />
            </div>
            <span className="text-sm font-medium">{(liquidity * 100).toFixed(0)}%</span>
          </div>
        </div>
      </div>
    </Card>
  )
}
