/**
 * GoalCard - Goal with circular progress ring
 */

'use client'

import { Calendar } from 'lucide-react'
import type { SavingsGoal } from '@/lib/services/savings-goal-service'

interface GoalCardProps {
  goal: SavingsGoal
  onClick?: () => void
}

export function GoalCard({ goal, onClick }: GoalCardProps) {
  const progress = (goal.currentAmount / goal.targetAmount) * 100
  const progressClamped = Math.min(100, Math.max(0, progress))

  // SVG circle calculations
  const size = 120
  const strokeWidth = 8
  const radius = (size - strokeWidth) / 2
  const circumference = 2 * Math.PI * radius
  const offset = circumference - (progressClamped / 100) * circumference

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })
  }

  const getStatusColor = () => {
    if (goal.status === 'completed') return '#10b981'
    if (progressClamped >= 75) return '#00d9ff'
    if (progressClamped >= 50) return '#f59e0b'
    return '#ef4444'
  }

  return (
    <button
      onClick={onClick}
      className="glass rounded-2xl p-5 space-y-4 hover:bg-accent/5 transition-colors w-full text-left"
    >
      {/* Progress ring */}
      <div className="flex items-center justify-center">
        <div className="relative" style={{ width: size, height: size }}>
          <svg width={size} height={size} className="transform -rotate-90">
            {/* Background circle */}
            <circle
              cx={size / 2}
              cy={size / 2}
              r={radius}
              fill="none"
              stroke="currentColor"
              strokeWidth={strokeWidth}
              className="text-muted opacity-20"
            />
            {/* Progress circle */}
            <circle
              cx={size / 2}
              cy={size / 2}
              r={radius}
              fill="none"
              stroke={getStatusColor()}
              strokeWidth={strokeWidth}
              strokeLinecap="round"
              strokeDasharray={circumference}
              strokeDashoffset={offset}
              className="transition-all duration-500"
            />
          </svg>

          {/* Center icon */}
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <span className="text-4xl mb-1">{goal.icon}</span>
            <span className="text-sm font-bold">{progressClamped.toFixed(0)}%</span>
          </div>
        </div>
      </div>

      {/* Goal info */}
      <div>
        <h3 className="font-semibold text-lg mb-1">{goal.name}</h3>
        <p className="text-sm text-muted-foreground">{goal.category}</p>
      </div>

      {/* Progress amounts */}
      <div className="glass-strong rounded-xl p-3">
        <div className="flex items-center justify-between mb-2">
          <span className="text-xs text-muted-foreground">Progress</span>
          <span className="text-xs font-medium">
            {goal.currentAmount.toLocaleString()} / {goal.targetAmount.toLocaleString()} ÉTR
          </span>
        </div>

        {/* Progress bar */}
        <div className="w-full h-2 bg-muted rounded-full overflow-hidden">
          <div
            className="h-full transition-all duration-500"
            style={{
              width: `${progressClamped}%`,
              background: getStatusColor(),
            }}
          />
        </div>
      </div>

      {/* Target date */}
      {goal.targetDate && (
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <Calendar className="w-3 h-3" />
          <span>Target: {formatDate(goal.targetDate)}</span>
        </div>
      )}

      {/* Completed badge */}
      {goal.status === 'completed' && (
        <div className="glass-strong rounded-lg p-2 bg-success/10 border border-success/20 text-center">
          <span className="text-success font-semibold text-sm">🎉 Completed!</span>
        </div>
      )}
    </button>
  )
}
