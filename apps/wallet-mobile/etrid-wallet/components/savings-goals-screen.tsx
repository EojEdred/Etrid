/**
 * SavingsGoalsScreen - All goals overview
 */

'use client'

import { useState } from 'react'
import { Target, Plus, TrendingUp } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { BottomNav } from '@/components/bottom-nav'
import { GoalCard } from '@/components/savings/goal-card'
import { GoalCelebration } from '@/components/savings/goal-celebration'
import { useSavingsGoals } from '@/hooks/use-savings-goals'
import { CreateGoalScreen } from '@/components/create-goal-screen'
import { GoalDetailScreen } from '@/components/goal-detail-screen'

interface SavingsGoalsScreenProps {
  onBack: () => void
  activeTab: string
  onTabChange: (tab: string) => void
}

export function SavingsGoalsScreen({ onBack, activeTab, onTabChange }: SavingsGoalsScreenProps) {
  const { goals, stats, loading } = useSavingsGoals()
  const [showCreateGoal, setShowCreateGoal] = useState(false)
  const [selectedGoalId, setSelectedGoalId] = useState<string | null>(null)
  const [celebratingGoalId, setCelebratingGoalId] = useState<string | null>(null)

  if (showCreateGoal) {
    return <CreateGoalScreen onBack={() => setShowCreateGoal(false)} />
  }

  if (selectedGoalId) {
    const goal = goals.find(g => g.id === selectedGoalId)
    if (goal) {
      return <GoalDetailScreen goal={goal} onBack={() => setSelectedGoalId(null)} />
    }
  }

  const celebratingGoal = goals.find(g => g.id === celebratingGoalId)

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-12 h-12 rounded-full bg-primary/20 flex items-center justify-center">
            <Target className="w-6 h-6 text-primary" />
          </div>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Savings Goals</h1>
            <p className="text-sm text-muted-foreground">Track your financial goals</p>
          </div>
          <Button
            size="icon"
            onClick={() => setShowCreateGoal(true)}
            style={{ background: '#00d9ff', color: '#000' }}
          >
            <Plus className="w-5 h-5" />
          </Button>
        </div>

        {/* Stats */}
        {stats && (
          <div className="grid grid-cols-3 gap-3 mb-6">
            <div className="glass rounded-xl p-4">
              <p className="text-xs text-muted-foreground mb-1">Total Saved</p>
              <p className="text-lg font-bold">{stats.totalSaved.toLocaleString()} ÉTR</p>
            </div>
            <div className="glass rounded-xl p-4">
              <p className="text-xs text-muted-foreground mb-1">Active Goals</p>
              <p className="text-lg font-bold">{stats.activeGoals}</p>
            </div>
            <div className="glass rounded-xl p-4">
              <p className="text-xs text-muted-foreground mb-1">Completed</p>
              <p className="text-lg font-bold text-success">{stats.completedGoals}</p>
            </div>
          </div>
        )}

        {stats && stats.activeGoals > 0 && (
          <div className="glass-strong rounded-xl p-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm text-muted-foreground">Average Progress</span>
              <span className="font-semibold">{stats.averageProgress.toFixed(0)}%</span>
            </div>
            <div className="w-full h-2 bg-muted rounded-full overflow-hidden">
              <div
                className="h-full bg-accent transition-all duration-500"
                style={{ width: `${Math.min(100, stats.averageProgress)}%` }}
              />
            </div>
          </div>
        )}
      </header>

      <main className="px-6 space-y-4">
        {loading ? (
          <div className="glass rounded-2xl p-8 text-center">
            <p className="text-muted-foreground">Loading goals...</p>
          </div>
        ) : goals.length === 0 ? (
          <div className="glass rounded-2xl p-8 text-center">
            <div className="w-16 h-16 rounded-full bg-accent/20 flex items-center justify-center mx-auto mb-4">
              <Target className="w-8 h-8 text-accent" />
            </div>
            <p className="font-semibold mb-2">No Goals Yet</p>
            <p className="text-sm text-muted-foreground mb-4">
              Create your first savings goal to get started
            </p>
            <Button
              onClick={() => setShowCreateGoal(true)}
              style={{ background: '#00d9ff', color: '#000' }}
            >
              Create Goal
            </Button>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-4">
            {goals.map((goal) => (
              <GoalCard
                key={goal.id}
                goal={goal}
                onClick={() => setSelectedGoalId(goal.id)}
              />
            ))}
          </div>
        )}
      </main>

      {celebratingGoal && (
        <GoalCelebration
          goal={celebratingGoal}
          open={!!celebratingGoalId}
          onClose={() => setCelebratingGoalId(null)}
          onCreateNew={() => {
            setCelebratingGoalId(null)
            setShowCreateGoal(true)
          }}
        />
      )}

      <BottomNav activeTab={activeTab} onTabChange={onTabChange} />
    </div>
  )
}
