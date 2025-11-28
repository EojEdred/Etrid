"use client"

import { AlertTriangle, CheckCircle2 } from "lucide-react"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"

interface CollateralMeterProps {
  healthFactor: number
  collateralValue: number
  totalSpent: number
  className?: string
}

export function CollateralMeter({ healthFactor, collateralValue, totalSpent, className = "" }: CollateralMeterProps) {
  const getStatus = () => {
    if (healthFactor >= 150) {
      return {
        status: "Safe",
        color: "text-green-500",
        bgColor: "bg-green-500",
        badgeVariant: "default" as const,
        icon: CheckCircle2,
        message: "Your collateral is healthy",
      }
    } else if (healthFactor >= 120) {
      return {
        status: "Warning",
        color: "text-yellow-500",
        bgColor: "bg-yellow-500",
        badgeVariant: "secondary" as const,
        icon: AlertTriangle,
        message: "Consider adding more collateral",
      }
    } else if (healthFactor >= 100) {
      return {
        status: "Danger",
        color: "text-orange-500",
        bgColor: "bg-orange-500",
        badgeVariant: "destructive" as const,
        icon: AlertTriangle,
        message: "Add collateral immediately",
      }
    } else {
      return {
        status: "Critical",
        color: "text-red-500",
        bgColor: "bg-red-500",
        badgeVariant: "destructive" as const,
        icon: AlertTriangle,
        message: "CRITICAL: Card may be frozen",
      }
    }
  }

  const status = getStatus()
  const Icon = status.icon

  // Calculate gauge percentage (0-200% health factor maps to 0-100% gauge)
  const gaugePercentage = Math.min(100, (healthFactor / 200) * 100)

  // Format health factor display
  const formatHealthFactor = (hf: number) => {
    if (hf === Infinity || isNaN(hf)) return "∞"
    return `${Math.round(hf)}%`
  }

  return (
    <div className={`glass p-6 rounded-2xl ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div>
          <p className="text-sm text-muted-foreground mb-1">Health Factor</p>
          <p className={`text-3xl font-bold ${status.color}`}>{formatHealthFactor(healthFactor)}</p>
        </div>
        <Badge variant={status.badgeVariant} className={`${status.color}`}>
          {status.status}
        </Badge>
      </div>

      {/* Radial Gauge Visualization */}
      <div className="relative w-40 h-40 mx-auto mb-6">
        <svg className="w-full h-full -rotate-90" viewBox="0 0 100 100">
          {/* Background Circle */}
          <circle
            cx="50"
            cy="50"
            r="40"
            fill="none"
            stroke="currentColor"
            strokeWidth="8"
            className="text-muted opacity-20"
          />

          {/* Progress Circle */}
          <circle
            cx="50"
            cy="50"
            r="40"
            fill="none"
            stroke="currentColor"
            strokeWidth="8"
            strokeLinecap="round"
            strokeDasharray={`${2 * Math.PI * 40}`}
            strokeDashoffset={`${2 * Math.PI * 40 * (1 - gaugePercentage / 100)}`}
            className={`${status.color} transition-all duration-500`}
          />

          {/* Markers */}
          {[0, 25, 50, 75, 100].map((percent) => {
            const angle = (percent / 100) * 360 - 90
            const x1 = 50 + 35 * Math.cos((angle * Math.PI) / 180)
            const y1 = 50 + 35 * Math.sin((angle * Math.PI) / 180)
            const x2 = 50 + 40 * Math.cos((angle * Math.PI) / 180)
            const y2 = 50 + 40 * Math.sin((angle * Math.PI) / 180)

            return (
              <line
                key={percent}
                x1={x1}
                y1={y1}
                x2={x2}
                y2={y2}
                stroke="currentColor"
                strokeWidth="2"
                className="text-muted-foreground"
              />
            )
          })}
        </svg>

        {/* Center Text */}
        <div className="absolute inset-0 flex items-center justify-center">
          <Icon className={`w-12 h-12 ${status.color}`} />
        </div>
      </div>

      {/* Status Message */}
      <div className={`flex items-start gap-2 p-3 rounded-lg bg-accent/5 border border-accent/20`}>
        <Icon className={`w-5 h-5 ${status.color} shrink-0 mt-0.5`} />
        <p className="text-sm">{status.message}</p>
      </div>

      {/* Collateral Details */}
      <div className="grid grid-cols-2 gap-4 mt-6 pt-6 border-t border-border">
        <div>
          <p className="text-xs text-muted-foreground mb-1">Total Collateral</p>
          <p className="text-lg font-semibold">${collateralValue.toLocaleString()}</p>
        </div>
        <div className="text-right">
          <p className="text-xs text-muted-foreground mb-1">Total Spent</p>
          <p className="text-lg font-semibold">${totalSpent.toLocaleString()}</p>
        </div>
      </div>

      {/* Health Factor Zones Reference */}
      <div className="mt-6 pt-6 border-t border-border space-y-2">
        <p className="text-xs text-muted-foreground mb-3">Health Factor Zones:</p>
        <div className="flex items-center gap-2 text-xs">
          <div className="w-3 h-3 rounded-full bg-green-500"></div>
          <span>Safe: &gt;150%</span>
        </div>
        <div className="flex items-center gap-2 text-xs">
          <div className="w-3 h-3 rounded-full bg-yellow-500"></div>
          <span>Warning: 120-150%</span>
        </div>
        <div className="flex items-center gap-2 text-xs">
          <div className="w-3 h-3 rounded-full bg-red-500"></div>
          <span>Danger: &lt;120%</span>
        </div>
      </div>
    </div>
  )
}
