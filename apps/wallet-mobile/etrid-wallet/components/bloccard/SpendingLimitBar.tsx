"use client"

import { Progress } from "@/components/ui/progress"
import { TrendingUp } from "lucide-react"

interface SpendingLimitBarProps {
  spent: number
  limit: number
  period?: "daily" | "weekly" | "monthly"
  className?: string
}

export function SpendingLimitBar({ spent, limit, period = "monthly", className = "" }: SpendingLimitBarProps) {
  const percentage = limit > 0 ? (spent / limit) * 100 : 0
  const remaining = Math.max(0, limit - spent)

  const getColor = () => {
    if (percentage >= 90) return "text-red-500"
    if (percentage >= 75) return "text-yellow-500"
    return "text-green-500"
  }

  const getProgressColor = () => {
    if (percentage >= 90) return "bg-red-500"
    if (percentage >= 75) return "bg-yellow-500"
    return "bg-green-500"
  }

  const getPeriodLabel = () => {
    const labels = {
      daily: "Today",
      weekly: "This Week",
      monthly: "This Month",
    }
    return labels[period]
  }

  return (
    <div className={`glass p-6 rounded-2xl ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <TrendingUp className="w-5 h-5 text-accent" />
          <p className="font-semibold">Spending Limit</p>
        </div>
        <p className="text-sm text-muted-foreground">{getPeriodLabel()}</p>
      </div>

      {/* Amount Display */}
      <div className="mb-4">
        <div className="flex items-baseline gap-2 mb-1">
          <p className={`text-3xl font-bold ${getColor()}`}>
            ${spent.toLocaleString()}
          </p>
          <p className="text-sm text-muted-foreground">of ${limit.toLocaleString()}</p>
        </div>
        <p className="text-sm text-muted-foreground">
          ${remaining.toLocaleString()} remaining
        </p>
      </div>

      {/* Progress Bar */}
      <div className="space-y-2">
        <div className="relative h-3 bg-muted rounded-full overflow-hidden">
          <div
            className={`absolute inset-y-0 left-0 ${getProgressColor()} transition-all duration-300 rounded-full`}
            style={{ width: `${Math.min(100, percentage)}%` }}
          />
        </div>

        <div className="flex items-center justify-between text-xs text-muted-foreground">
          <span>0%</span>
          <span className={percentage >= 75 ? getColor() : ""}>
            {percentage.toFixed(1)}% used
          </span>
          <span>100%</span>
        </div>
      </div>

      {/* Warning Message */}
      {percentage >= 75 && (
        <div className={`mt-4 p-3 rounded-lg ${
          percentage >= 90 ? "bg-red-500/10 border border-red-500/20" : "bg-yellow-500/10 border border-yellow-500/20"
        }`}>
          <p className={`text-sm ${percentage >= 90 ? "text-red-500" : "text-yellow-500"}`}>
            {percentage >= 90
              ? "You've reached your spending limit"
              : "You're approaching your spending limit"}
          </p>
        </div>
      )}
    </div>
  )
}
