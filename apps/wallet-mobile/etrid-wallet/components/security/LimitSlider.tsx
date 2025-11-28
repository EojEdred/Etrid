"use client"

import { TrendingUp, AlertCircle } from "lucide-react"
import { Slider } from "@/components/ui/slider"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"

interface LimitSliderProps {
  label: string
  period: "daily" | "weekly" | "monthly" | "per-transaction"
  limit: number
  spent?: number
  onChange: (value: number) => void
  min?: number
  max?: number
  disabled?: boolean
}

export function LimitSlider({
  label,
  period,
  limit,
  spent = 0,
  onChange,
  min = 0,
  max = 100000,
  disabled = false,
}: LimitSliderProps) {
  const formatCurrency = (value: number) => {
    return `$${value.toLocaleString()}`
  }

  const remaining = limit - spent
  const usagePercent = limit > 0 ? (spent / limit) * 100 : 0
  const isNearLimit = usagePercent >= 80

  const recommendedLimits = {
    daily: 10000,
    weekly: 50000,
    monthly: 200000,
    "per-transaction": 10000,
  }

  const recommended = recommendedLimits[period]
  const isUsingRecommended = limit === recommended

  return (
    <Card>
      <CardContent className="p-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-4">
          <div>
            <h4 className="font-semibold">{label}</h4>
            <p className="text-xs text-muted-foreground capitalize">
              {period.replace("-", " ")} spending limit
            </p>
          </div>
          {isUsingRecommended && (
            <Badge variant="outline" className="text-xs">
              <TrendingUp className="w-3 h-3 mr-1" />
              Recommended
            </Badge>
          )}
        </div>

        {/* Current Limit Display */}
        <div className="text-center p-4 rounded-lg bg-gradient-to-br from-accent/10 to-accent/5 mb-4">
          <p className="text-2xl font-bold">{formatCurrency(limit)}</p>
        </div>

        {/* Slider */}
        <div className="mb-6">
          <Slider
            value={[limit]}
            onValueChange={(values) => onChange(values[0])}
            min={min}
            max={max}
            step={100}
            disabled={disabled}
            className="cursor-pointer"
          />
          <div className="flex justify-between text-xs text-muted-foreground mt-2">
            <span>{formatCurrency(min)}</span>
            <span>{formatCurrency(max)}</span>
          </div>
        </div>

        {/* Usage Stats (only if spent > 0) */}
        {spent > 0 && (
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span className="text-muted-foreground">Usage this period</span>
              <span className={isNearLimit ? "text-red-500 font-semibold" : "font-semibold"}>
                {formatCurrency(spent)} / {formatCurrency(limit)}
              </span>
            </div>
            <Progress
              value={usagePercent}
              className={`h-2 ${isNearLimit ? "bg-red-500/20" : ""}`}
            />
            <div className="flex items-center justify-between text-xs">
              <span className="text-muted-foreground">
                Remaining: {formatCurrency(remaining)}
              </span>
              <span className={isNearLimit ? "text-red-500" : "text-muted-foreground"}>
                {usagePercent.toFixed(0)}% used
              </span>
            </div>
          </div>
        )}

        {/* Warning for near limit */}
        {isNearLimit && (
          <div className="mt-4 flex items-start gap-2 p-3 rounded-lg bg-red-500/10 border border-red-500/50">
            <AlertCircle className="w-4 h-4 text-red-500 mt-0.5" />
            <div className="text-xs">
              <p className="font-semibold text-red-500">Approaching limit</p>
              <p className="text-muted-foreground">
                You've used {usagePercent.toFixed(0)}% of your {period} limit.
                Consider adjusting your limit or tracking your spending.
              </p>
            </div>
          </div>
        )}

        {/* Recommended Limit Button */}
        {!isUsingRecommended && (
          <button
            onClick={() => !disabled && onChange(recommended)}
            disabled={disabled}
            className="mt-4 w-full text-xs text-muted-foreground hover:text-foreground transition-colors"
          >
            Use recommended limit ({formatCurrency(recommended)})
          </button>
        )}
      </CardContent>
    </Card>
  )
}
