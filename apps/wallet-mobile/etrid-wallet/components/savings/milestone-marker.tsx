/**
 * MilestoneMarker - Goal milestone achievement display
 */

'use client'

import { Trophy, Star, Target } from 'lucide-react'
import type { GoalMilestone } from '@/lib/services/savings-goal-service'

interface MilestoneMarkerProps {
  milestone: GoalMilestone
}

export function MilestoneMarker({ milestone }: MilestoneMarkerProps) {
  const getIcon = () => {
    switch (milestone.percentage) {
      case 25:
        return <Star className="w-5 h-5" />
      case 50:
        return <Target className="w-5 h-5" />
      case 75:
        return <Trophy className="w-5 h-5" />
      case 100:
        return <Trophy className="w-6 h-6" />
      default:
        return <Star className="w-5 h-5" />
    }
  }

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })
  }

  return (
    <div
      className={`glass rounded-xl p-4 transition-all ${
        milestone.achieved
          ? 'bg-success/10 border border-success/20'
          : 'opacity-50'
      }`}
    >
      <div className="flex items-center gap-3">
        {/* Icon */}
        <div
          className={`w-12 h-12 rounded-full flex items-center justify-center ${
            milestone.achieved
              ? 'bg-success/20 text-success'
              : 'bg-muted text-muted-foreground'
          }`}
        >
          {getIcon()}
        </div>

        {/* Info */}
        <div className="flex-1">
          <h4 className="font-semibold">
            {milestone.percentage}% Milestone
          </h4>
          {milestone.achieved && milestone.achievedAt ? (
            <p className="text-xs text-success">
              Achieved on {formatDate(milestone.achievedAt)}
            </p>
          ) : (
            <p className="text-xs text-muted-foreground">
              Not yet achieved
            </p>
          )}
        </div>

        {/* Checkmark */}
        {milestone.achieved && (
          <div className="w-8 h-8 rounded-full bg-success flex items-center justify-center">
            <span className="text-white text-xl">✓</span>
          </div>
        )}
      </div>
    </div>
  )
}
