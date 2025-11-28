/**
 * GoalProgressBar - Linear progress with milestones
 */

'use client'

import { Trophy } from 'lucide-react'
import type { GoalMilestone } from '@/lib/services/savings-goal-service'

interface GoalProgressBarProps {
  currentAmount: number
  targetAmount: number
  milestones: GoalMilestone[]
}

export function GoalProgressBar({
  currentAmount,
  targetAmount,
  milestones,
}: GoalProgressBarProps) {
  const progress = (currentAmount / targetAmount) * 100
  const progressClamped = Math.min(100, Math.max(0, progress))

  return (
    <div className="space-y-3">
      {/* Progress bar container */}
      <div className="relative">
        {/* Main progress bar */}
        <div className="w-full h-4 bg-muted rounded-full overflow-hidden">
          <div
            className="h-full bg-gradient-to-r from-accent/50 to-accent transition-all duration-500"
            style={{ width: `${progressClamped}%` }}
          />
        </div>

        {/* Milestone markers */}
        {milestones.map((milestone, index) => {
          const position = milestone.percentage
          const isPassed = progress >= milestone.percentage

          return (
            <div
              key={index}
              className="absolute top-1/2 -translate-y-1/2 -translate-x-1/2"
              style={{ left: `${position}%` }}
            >
              <div
                className={`w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all ${
                  milestone.achieved
                    ? 'bg-success border-success'
                    : isPassed
                    ? 'bg-accent border-accent'
                    : 'bg-background border-muted'
                }`}
              >
                {milestone.achieved && (
                  <Trophy className="w-3 h-3 text-white" />
                )}
              </div>

              {/* Milestone label */}
              <div className="absolute top-8 left-1/2 -translate-x-1/2 whitespace-nowrap">
                <span
                  className={`text-xs font-medium ${
                    milestone.achieved ? 'text-success' : 'text-muted-foreground'
                  }`}
                >
                  {milestone.percentage}%
                </span>
              </div>
            </div>
          )
        })}
      </div>

      {/* Amount labels */}
      <div className="flex items-center justify-between text-sm">
        <span className="font-semibold">{currentAmount.toLocaleString()} ÉTR</span>
        <span className="text-muted-foreground">{targetAmount.toLocaleString()} ÉTR</span>
      </div>
    </div>
  )
}
