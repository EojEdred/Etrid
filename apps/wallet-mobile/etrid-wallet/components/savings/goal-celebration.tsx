/**
 * GoalCelebration - Goal completion modal
 */

'use client'

import { useEffect, useState } from 'react'
import { Trophy, Share2 } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent } from '@/components/ui/dialog'
import type { SavingsGoal } from '@/lib/services/savings-goal-service'

interface GoalCelebrationProps {
  goal: SavingsGoal
  open: boolean
  onClose: () => void
  onCreateNew?: () => void
  onShare?: () => void
}

export function GoalCelebration({
  goal,
  open,
  onClose,
  onCreateNew,
  onShare,
}: GoalCelebrationProps) {
  const [showConfetti, setShowConfetti] = useState(false)

  useEffect(() => {
    if (open) {
      setShowConfetti(true)
      const timeout = setTimeout(() => setShowConfetti(false), 3000)
      return () => clearTimeout(timeout)
    }
  }, [open])

  const timeTaken = goal.completedAt
    ? Math.ceil((goal.completedAt - goal.createdAt) / (1000 * 60 * 60 * 24))
    : 0

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-md">
        <div className="relative">
          {/* Confetti effect */}
          {showConfetti && (
            <div className="absolute inset-0 pointer-events-none overflow-hidden">
              {Array.from({ length: 50 }).map((_, i) => (
                <div
                  key={i}
                  className="absolute animate-fall"
                  style={{
                    left: `${Math.random() * 100}%`,
                    top: `-${Math.random() * 20}px`,
                    animationDelay: `${Math.random() * 2}s`,
                    animationDuration: `${2 + Math.random() * 2}s`,
                  }}
                >
                  {['🎉', '🎊', '✨', '🌟', '⭐'][Math.floor(Math.random() * 5)]}
                </div>
              ))}
            </div>
          )}

          {/* Content */}
          <div className="text-center space-y-6 py-6">
            {/* Trophy icon */}
            <div className="flex justify-center">
              <div className="w-24 h-24 rounded-full bg-success/20 flex items-center justify-center animate-bounce">
                <Trophy className="w-12 h-12 text-success" />
              </div>
            </div>

            {/* Title */}
            <div>
              <h2 className="text-3xl font-bold mb-2">Goal Completed!</h2>
              <p className="text-lg text-muted-foreground">
                Congratulations on reaching your {goal.name} goal!
              </p>
            </div>

            {/* Goal icon */}
            <div className="text-6xl">{goal.icon}</div>

            {/* Stats */}
            <div className="glass-strong rounded-2xl p-6 space-y-4">
              <div>
                <p className="text-sm text-muted-foreground mb-1">Amount Saved</p>
                <p className="text-3xl font-bold text-success">
                  {goal.targetAmount.toLocaleString()} ÉTR
                </p>
              </div>

              <div className="grid grid-cols-2 gap-4 pt-4 border-t border-border">
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Time Taken</p>
                  <p className="font-semibold">{timeTaken} days</p>
                </div>
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Progress</p>
                  <p className="font-semibold text-success">100%</p>
                </div>
              </div>
            </div>

            {/* Actions */}
            <div className="space-y-3">
              {onShare && (
                <Button
                  onClick={onShare}
                  variant="outline"
                  className="w-full"
                >
                  <Share2 className="w-4 h-4 mr-2" />
                  Share Achievement
                </Button>
              )}

              {onCreateNew && (
                <Button
                  onClick={onCreateNew}
                  className="w-full"
                  style={{ background: '#00d9ff', color: '#000' }}
                >
                  Create New Goal
                </Button>
              )}

              <Button
                onClick={onClose}
                variant="ghost"
                className="w-full"
              >
                Close
              </Button>
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
