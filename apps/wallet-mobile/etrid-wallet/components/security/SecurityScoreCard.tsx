"use client"

import { Shield, CheckCircle2, AlertTriangle, XCircle } from "lucide-react"
import { SecurityScore } from "@/lib/types/security"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"

interface SecurityScoreCardProps {
  score: SecurityScore
}

export function SecurityScoreCard({ score }: SecurityScoreCardProps) {
  const getScoreColor = (value: number) => {
    if (value >= 80) return "text-green-500"
    if (value >= 50) return "text-yellow-500"
    return "text-red-500"
  }

  const getScoreBg = (value: number) => {
    if (value >= 80) return "bg-green-500"
    if (value >= 50) return "bg-yellow-500"
    return "bg-red-500"
  }

  const getScoreLabel = (value: number) => {
    if (value >= 80) return "Excellent"
    if (value >= 50) return "Good"
    return "Needs Improvement"
  }

  const getScoreIcon = (value: number) => {
    if (value >= 80) return CheckCircle2
    if (value >= 50) return AlertTriangle
    return XCircle
  }

  const ScoreIcon = getScoreIcon(score.overall)

  return (
    <Card className="overflow-hidden">
      <CardHeader className="bg-gradient-to-br from-accent/10 to-accent/5">
        <CardTitle className="flex items-center gap-2">
          <Shield className="w-5 h-5" />
          Security Score
        </CardTitle>
      </CardHeader>
      <CardContent className="p-6">
        {/* Overall Score */}
        <div className="flex items-center justify-center mb-6">
          <div className="relative">
            {/* Circular Progress */}
            <svg className="w-32 h-32 transform -rotate-90">
              <circle
                cx="64"
                cy="64"
                r="56"
                stroke="currentColor"
                strokeWidth="8"
                fill="none"
                className="text-muted/20"
              />
              <circle
                cx="64"
                cy="64"
                r="56"
                stroke="currentColor"
                strokeWidth="8"
                fill="none"
                strokeDasharray={`${(score.overall / 100) * 351.86} 351.86`}
                className={getScoreColor(score.overall)}
                strokeLinecap="round"
              />
            </svg>
            <div className="absolute inset-0 flex flex-col items-center justify-center">
              <ScoreIcon className={`w-8 h-8 mb-1 ${getScoreColor(score.overall)}`} />
              <p className={`text-3xl font-bold ${getScoreColor(score.overall)}`}>
                {score.overall}
              </p>
              <p className="text-xs text-muted-foreground">/ 100</p>
            </div>
          </div>
        </div>

        <div className="text-center mb-6">
          <Badge
            className={`${getScoreBg(score.overall)} text-white border-0 px-4 py-1`}
          >
            {getScoreLabel(score.overall)}
          </Badge>
        </div>

        {/* Breakdown */}
        <div className="space-y-3 mb-6">
          <h4 className="text-sm font-semibold mb-3">Security Features</h4>

          {Object.entries(score.breakdown).map(([key, value]) => {
            const label = key
              .replace(/([A-Z])/g, " $1")
              .replace(/^./, (str) => str.toUpperCase())

            return (
              <div key={key} className="space-y-1">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-muted-foreground">{label}</span>
                  <span className={`font-semibold ${getScoreColor(value)}`}>
                    {value}/100
                  </span>
                </div>
                <Progress
                  value={value}
                  className={`h-2 ${value >= 80 ? "bg-green-500/20" : value >= 50 ? "bg-yellow-500/20" : "bg-red-500/20"}`}
                />
              </div>
            )
          })}
        </div>

        {/* Recommendations */}
        {score.recommendations.length > 0 && (
          <div>
            <h4 className="text-sm font-semibold mb-3 flex items-center gap-2">
              <AlertTriangle className="w-4 h-4 text-yellow-500" />
              Recommendations
            </h4>
            <div className="space-y-2">
              {score.recommendations.slice(0, 3).map((rec) => (
                <div
                  key={rec.id}
                  className="p-3 rounded-lg bg-muted/50 hover:bg-muted transition-colors"
                >
                  <div className="flex items-start justify-between mb-1">
                    <p className="text-sm font-medium">{rec.title}</p>
                    <Badge variant="outline" className="text-xs">
                      +{rec.impact} pts
                    </Badge>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    {rec.description}
                  </p>
                </div>
              ))}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
