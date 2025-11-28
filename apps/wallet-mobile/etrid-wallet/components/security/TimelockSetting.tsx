"use client"

import { Clock, Info } from "lucide-react"
import { Slider } from "@/components/ui/slider"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { Alert, AlertDescription } from "@/components/ui/alert"

interface TimelockSettingProps {
  hours: number
  onChange: (hours: number) => void
  disabled?: boolean
}

export function TimelockSetting({ hours, onChange, disabled = false }: TimelockSettingProps) {
  const presets = [
    { hours: 0, label: "Disabled" },
    { hours: 24, label: "24 hours" },
    { hours: 48, label: "48 hours" },
    { hours: 72, label: "3 days" },
    { hours: 168, label: "7 days" },
  ]

  const formatDuration = (h: number) => {
    if (h === 0) return "Disabled"
    if (h < 24) return `${h} hours`
    const days = Math.floor(h / 24)
    const remainingHours = h % 24
    if (remainingHours === 0) return `${days} day${days > 1 ? "s" : ""}`
    return `${days}d ${remainingHours}h`
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center gap-2">
          <Clock className="w-5 h-5" />
          <CardTitle>Timelock Delay</CardTitle>
        </div>
        <CardDescription>
          Delay before withdrawals are processed
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Current Value Display */}
        <div className="text-center p-4 rounded-lg bg-gradient-to-br from-accent/10 to-accent/5">
          <p className="text-sm text-muted-foreground mb-1">Current Delay</p>
          <p className="text-3xl font-bold">{formatDuration(hours)}</p>
        </div>

        {/* Slider */}
        <div className="space-y-4">
          <Slider
            value={[hours]}
            onValueChange={(values) => onChange(values[0])}
            max={168}
            step={1}
            disabled={disabled}
            className="cursor-pointer"
          />

          {/* Preset Buttons */}
          <div className="flex flex-wrap gap-2 justify-center">
            {presets.map((preset) => (
              <Badge
                key={preset.hours}
                variant={hours === preset.hours ? "default" : "outline"}
                className="cursor-pointer hover:bg-accent/20 transition-colors"
                onClick={() => !disabled && onChange(preset.hours)}
              >
                {preset.label}
              </Badge>
            ))}
          </div>
        </div>

        {/* Preview */}
        <div className="p-3 rounded-lg bg-muted/50">
          <h4 className="text-sm font-semibold mb-2">How it works</h4>
          <p className="text-sm text-muted-foreground">
            {hours === 0
              ? "Withdrawals will be processed immediately without any delay."
              : `Withdrawals will be locked for ${formatDuration(hours)} after initiation. You can cancel during this period.`}
          </p>
        </div>

        {/* Security Notice */}
        {hours > 0 && (
          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription className="text-xs">
              <strong>Exceptions:</strong> Whitelisted addresses and small amounts
              (under $100) bypass the timelock for convenience.
            </AlertDescription>
          </Alert>
        )}
      </CardContent>
    </Card>
  )
}
