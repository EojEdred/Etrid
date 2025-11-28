/**
 * GoalDetailScreen - Single goal progress and management
 */

'use client'

import { useState } from 'react'
import { ArrowLeft, Plus, TrendingUp, Calendar, Clock } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { GoalProgressBar } from '@/components/savings/goal-progress-bar'
import { MilestoneMarker } from '@/components/savings/milestone-marker'
import { useSavingsGoals } from '@/hooks/use-savings-goals'
import { useGoalProgress } from '@/hooks/use-goal-progress'
import type { SavingsGoal } from '@/lib/services/savings-goal-service'

interface GoalDetailScreenProps {
  goal: SavingsGoal
  onBack: () => void
}

export function GoalDetailScreen({ goal: initialGoal, onBack }: GoalDetailScreenProps) {
  const [contributeAmount, setContributeAmount] = useState('')
  const { contribute, loading } = useSavingsGoals()
  const { goal, progress } = useGoalProgress(initialGoal.id)

  const currentGoal = goal || initialGoal

  const handleContribute = async () => {
    try {
      await contribute(currentGoal.id, parseFloat(contributeAmount))
      setContributeAmount('')
    } catch (err) {
      console.error(err)
    }
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">{currentGoal.name}</h1>
            <p className="text-sm text-muted-foreground">{currentGoal.category}</p>
          </div>
        </div>

        {/* Goal Icon and Progress */}
        <div className="glass-strong rounded-2xl p-6 text-center mb-6">
          <div className="text-6xl mb-4">{currentGoal.icon}</div>
          <div className="text-3xl font-bold mb-2">
            {currentGoal.currentAmount.toLocaleString()} ÉTR
          </div>
          <div className="text-muted-foreground mb-4">
            of {currentGoal.targetAmount.toLocaleString()} ÉTR
          </div>
          {progress && (
            <div className="text-5xl font-bold text-accent mb-2">
              {progress.percentage.toFixed(0)}%
            </div>
          )}
          <div className="text-sm text-muted-foreground">Complete</div>
        </div>
      </header>

      <main className="px-6 space-y-6">
        {/* Progress Bar */}
        <GoalProgressBar
          currentAmount={currentGoal.currentAmount}
          targetAmount={currentGoal.targetAmount}
          milestones={currentGoal.milestones}
        />

        {/* Quick Contribute */}
        <div className="glass rounded-2xl p-5 space-y-4">
          <h3 className="font-semibold">Quick Contribute</h3>
          <div className="flex gap-2">
            <Input
              type="number"
              placeholder="Amount"
              value={contributeAmount}
              onChange={(e) => setContributeAmount(e.target.value)}
              className="glass border-border"
            />
            <Button
              onClick={handleContribute}
              disabled={!contributeAmount || parseFloat(contributeAmount) <= 0 || loading}
              style={{ background: '#00d9ff', color: '#000' }}
            >
              <Plus className="w-4 h-4 mr-2" />
              Add
            </Button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-2 gap-3">
          {currentGoal.targetDate && (
            <div className="glass rounded-xl p-4">
              <div className="flex items-center gap-2 mb-2">
                <Calendar className="w-4 h-4 text-muted-foreground" />
                <p className="text-xs text-muted-foreground">Target Date</p>
              </div>
              <p className="font-semibold">
                {new Date(currentGoal.targetDate).toLocaleDateString()}
              </p>
            </div>
          )}

          {progress && progress.daysRemaining !== null && (
            <div className="glass rounded-xl p-4">
              <div className="flex items-center gap-2 mb-2">
                <Clock className="w-4 h-4 text-muted-foreground" />
                <p className="text-xs text-muted-foreground">Days Remaining</p>
              </div>
              <p className="font-semibold">
                {progress.daysRemaining > 0 ? progress.daysRemaining : 'Overdue'}
              </p>
            </div>
          )}

          {progress && progress.amountRemaining > 0 && (
            <div className="glass rounded-xl p-4">
              <p className="text-xs text-muted-foreground mb-2">Amount Remaining</p>
              <p className="font-semibold">
                {progress.amountRemaining.toLocaleString()} ÉTR
              </p>
            </div>
          )}

          {progress && progress.recommendedMonthlyContribution > 0 && (
            <div className="glass rounded-xl p-4">
              <div className="flex items-center gap-2 mb-2">
                <TrendingUp className="w-4 h-4 text-success" />
                <p className="text-xs text-muted-foreground">Recommended/Month</p>
              </div>
              <p className="font-semibold text-success">
                {progress.recommendedMonthlyContribution.toFixed(2)} ÉTR
              </p>
            </div>
          )}
        </div>

        {/* On Track Status */}
        {progress && (
          <div className={`glass rounded-xl p-4 ${progress.isOnTrack ? 'bg-success/10 border-success/20' : 'bg-warning/10 border-warning/20'}`}>
            <p className={`font-semibold ${progress.isOnTrack ? 'text-success' : 'text-warning'}`}>
              {progress.isOnTrack ? '✓ On Track' : '⚠️ Behind Schedule'}
            </p>
            <p className="text-sm text-muted-foreground mt-1">
              {progress.isOnTrack
                ? 'You\'re making great progress!'
                : 'Consider increasing your contributions'}
            </p>
          </div>
        )}

        {/* Milestones */}
        <div className="space-y-3">
          <h3 className="font-semibold">Milestones</h3>
          <div className="space-y-2">
            {currentGoal.milestones.map((milestone, index) => (
              <MilestoneMarker key={index} milestone={milestone} />
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}
