/**
 * HealthFactorGauge - Loan health indicator with radial gauge
 */

'use client'

import { AlertCircle, CheckCircle, AlertTriangle } from 'lucide-react'

interface HealthFactorGaugeProps {
  healthFactor: number
  showValue?: boolean
  size?: 'sm' | 'md' | 'lg'
}

export function HealthFactorGauge({
  healthFactor,
  showValue = true,
  size = 'md',
}: HealthFactorGaugeProps) {
  // Determine status and color
  let status: 'safe' | 'warning' | 'danger' | 'liquidation'
  let color: string
  let strokeColor: string
  let icon: React.ReactNode

  if (healthFactor >= 200) {
    status = 'safe'
    color = 'text-success'
    strokeColor = '#10b981'
    icon = <CheckCircle className="w-6 h-6 text-success" />
  } else if (healthFactor >= 150) {
    status = 'warning'
    color = 'text-warning'
    strokeColor = '#f59e0b'
    icon = <AlertTriangle className="w-6 h-6 text-warning" />
  } else if (healthFactor >= 120) {
    status = 'danger'
    color = 'text-destructive'
    strokeColor = '#ef4444'
    icon = <AlertCircle className="w-6 h-6 text-destructive" />
  } else {
    status = 'liquidation'
    color = 'text-destructive'
    strokeColor = '#dc2626'
    icon = <AlertCircle className="w-6 h-6 text-destructive animate-pulse" />
  }

  // Calculate gauge fill (0-100%)
  // Map health factor: 100% -> 0%, 300% -> 100%
  const maxHF = 300
  const minHF = 100
  const fillPercentage = Math.min(100, Math.max(0, ((healthFactor - minHF) / (maxHF - minHF)) * 100))

  // SVG dimensions based on size
  const dimensions = {
    sm: { size: 80, strokeWidth: 6, fontSize: '18px' },
    md: { size: 120, strokeWidth: 8, fontSize: '24px' },
    lg: { size: 160, strokeWidth: 10, fontSize: '32px' },
  }

  const { size: svgSize, strokeWidth, fontSize } = dimensions[size]
  const radius = (svgSize - strokeWidth) / 2
  const circumference = 2 * Math.PI * radius
  const offset = circumference - (fillPercentage / 100) * circumference

  return (
    <div className="flex flex-col items-center gap-3">
      <div className="relative" style={{ width: svgSize, height: svgSize }}>
        <svg width={svgSize} height={svgSize} className="transform -rotate-90">
          {/* Background circle */}
          <circle
            cx={svgSize / 2}
            cy={svgSize / 2}
            r={radius}
            fill="none"
            stroke="currentColor"
            strokeWidth={strokeWidth}
            className="text-muted opacity-20"
          />
          {/* Progress circle */}
          <circle
            cx={svgSize / 2}
            cy={svgSize / 2}
            r={radius}
            fill="none"
            stroke={strokeColor}
            strokeWidth={strokeWidth}
            strokeLinecap="round"
            strokeDasharray={circumference}
            strokeDashoffset={offset}
            className="transition-all duration-500"
          />
        </svg>

        {/* Center text */}
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          {showValue && (
            <>
              <span className={`font-bold ${color}`} style={{ fontSize }}>
                {healthFactor.toFixed(0)}%
              </span>
              <span className="text-xs text-muted-foreground">Health</span>
            </>
          )}
        </div>
      </div>

      {/* Status indicator */}
      <div className="flex items-center gap-2">
        {icon}
        <span className={`text-sm font-medium ${color}`}>
          {status === 'safe' && 'Safe'}
          {status === 'warning' && 'Warning'}
          {status === 'danger' && 'Danger'}
          {status === 'liquidation' && 'Liquidation Risk!'}
        </span>
      </div>

      {/* Status description */}
      <div className="text-center max-w-xs">
        <p className="text-xs text-muted-foreground">
          {status === 'safe' && 'Your collateral is well above liquidation threshold'}
          {status === 'warning' && 'Consider adding more collateral to improve health'}
          {status === 'danger' && 'Add collateral soon to avoid liquidation'}
          {status === 'liquidation' && 'URGENT: Add collateral immediately or risk liquidation'}
        </p>
      </div>
    </div>
  )
}
