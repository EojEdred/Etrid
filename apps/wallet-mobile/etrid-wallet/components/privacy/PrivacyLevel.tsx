"use client"

import { Shield, Zap, DollarSign, CheckCircle2 } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { PRIVACY_LEVELS } from "@/lib/types/privacy"

interface PrivacyLevelProps {
  selected: "low" | "medium" | "high"
  onSelect: (level: "low" | "medium" | "high") => void
  disabled?: boolean
}

export function PrivacyLevel({ selected, onSelect, disabled = false }: PrivacyLevelProps) {
  const levels: Array<"low" | "medium" | "high"> = ["low", "medium", "high"]

  const levelColors = {
    low: {
      bg: "bg-yellow-500/10 border-yellow-500",
      text: "text-yellow-500",
      icon: "🔓",
    },
    medium: {
      bg: "bg-blue-500/10 border-blue-500",
      text: "text-blue-500",
      icon: "🔒",
    },
    high: {
      bg: "bg-green-500/10 border-green-500",
      text: "text-green-500",
      icon: "🛡️",
    },
  }

  const speedIcons = {
    fast: "⚡",
    medium: "🚶",
    slow: "🐢",
  }

  const costIcons = {
    low: "💰",
    medium: "💰💰",
    high: "💰💰💰",
  }

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-lg font-semibold mb-2">Privacy Level</h3>
        <p className="text-sm text-muted-foreground mb-4">
          Choose your privacy preference. Higher levels provide more anonymity but
          may cost more and take longer.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {levels.map((level) => {
          const config = PRIVACY_LEVELS[level]
          const colors = levelColors[level]
          const isSelected = selected === level

          return (
            <Card
              key={level}
              className={`cursor-pointer transition-all ${
                isSelected
                  ? `border-2 ${colors.bg} shadow-lg scale-105`
                  : "border-2 border-transparent hover:border-muted"
              } ${disabled ? "opacity-50 cursor-not-allowed" : ""}`}
              onClick={() => !disabled && onSelect(level)}
            >
              <CardContent className="p-6">
                {/* Header */}
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-2">
                    <span className="text-3xl">{colors.icon}</span>
                    <div>
                      <h4 className="font-semibold">{config.name}</h4>
                      <p className="text-xs text-muted-foreground capitalize">
                        {level} privacy
                      </p>
                    </div>
                  </div>
                  {isSelected && (
                    <CheckCircle2 className={`w-6 h-6 ${colors.text}`} />
                  )}
                </div>

                {/* Description */}
                <p className="text-sm text-muted-foreground mb-4">
                  {config.description}
                </p>

                {/* Privacy Rating */}
                <div className="mb-4">
                  <div className="flex items-center justify-between text-xs mb-1">
                    <span className="text-muted-foreground">Privacy Score</span>
                    <span className={`font-bold ${colors.text}`}>
                      {config.privacyRating}/100
                    </span>
                  </div>
                  <div className="h-2 bg-muted rounded-full overflow-hidden">
                    <div
                      className={`h-full ${colors.text.replace("text-", "bg-")}`}
                      style={{ width: `${config.privacyRating}%` }}
                    />
                  </div>
                </div>

                {/* Features */}
                <div className="space-y-2 mb-4">
                  <div className="flex items-center gap-2 text-xs">
                    <Shield className="w-3 h-3" />
                    <span className="text-muted-foreground">
                      {config.features.stealthAddresses ? "Stealth addresses" : "No stealth"}
                    </span>
                  </div>
                  {config.features.mixing && (
                    <div className="flex items-center gap-2 text-xs">
                      <Shield className="w-3 h-3" />
                      <span className="text-muted-foreground">
                        {config.features.mixingRounds} mixing rounds
                      </span>
                    </div>
                  )}
                  {config.features.torRouting && (
                    <div className="flex items-center gap-2 text-xs">
                      <Shield className="w-3 h-3" />
                      <span className="text-muted-foreground">Tor routing</span>
                    </div>
                  )}
                </div>

                {/* Speed & Cost */}
                <div className="flex items-center justify-between pt-4 border-t">
                  <div className="flex items-center gap-1">
                    <Zap className="w-4 h-4 text-muted-foreground" />
                    <span className="text-xs text-muted-foreground">
                      {speedIcons[config.speed]} {config.speed}
                    </span>
                  </div>
                  <div className="flex items-center gap-1">
                    <DollarSign className="w-4 h-4 text-muted-foreground" />
                    <span className="text-xs text-muted-foreground">
                      {costIcons[config.cost]}
                    </span>
                  </div>
                </div>

                {/* Selected Badge */}
                {isSelected && (
                  <Badge className={`w-full mt-4 ${colors.bg} border-0 justify-center`}>
                    Current Selection
                  </Badge>
                )}
              </CardContent>
            </Card>
          )
        })}
      </div>

      {/* Feature Comparison Table */}
      <div className="mt-6 p-4 rounded-lg bg-muted/50">
        <h4 className="text-sm font-semibold mb-3">Feature Comparison</h4>
        <div className="space-y-2 text-xs">
          <div className="grid grid-cols-4 gap-2 font-semibold">
            <span>Feature</span>
            <span className="text-center">Low</span>
            <span className="text-center">Medium</span>
            <span className="text-center">High</span>
          </div>
          <div className="grid grid-cols-4 gap-2 text-muted-foreground">
            <span>Stealth Addresses</span>
            <span className="text-center">❌</span>
            <span className="text-center">✅</span>
            <span className="text-center">✅</span>
          </div>
          <div className="grid grid-cols-4 gap-2 text-muted-foreground">
            <span>Coin Mixing</span>
            <span className="text-center">❌</span>
            <span className="text-center">3 rounds</span>
            <span className="text-center">10 rounds</span>
          </div>
          <div className="grid grid-cols-4 gap-2 text-muted-foreground">
            <span>Tor Routing</span>
            <span className="text-center">❌</span>
            <span className="text-center">❌</span>
            <span className="text-center">✅</span>
          </div>
          <div className="grid grid-cols-4 gap-2 text-muted-foreground">
            <span>Metadata Scrubbing</span>
            <span className="text-center">❌</span>
            <span className="text-center">✅</span>
            <span className="text-center">✅</span>
          </div>
        </div>
      </div>
    </div>
  )
}
