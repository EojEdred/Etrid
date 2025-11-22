/**
 * useGoalProgress - Calculate and track goal progress
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import { savingsGoalService, type SavingsGoal } from '@/lib/services/savings-goal-service'

export interface GoalProgress {
  percentage: number
  amountRemaining: number
  isOnTrack: boolean
  projectedCompletionDate: number | null
  daysRemaining: number | null
  recommendedMonthlyContribution: number
}

export function useGoalProgress(goalId: string) {
  const [goal, setGoal] = useState<SavingsGoal | null>(null)
  const [progress, setProgress] = useState<GoalProgress | null>(null)
  const [loading, setLoading] = useState(true)

  const calculateProgress = useCallback(async (goalData: SavingsGoal): Promise<GoalProgress> => {
    const percentage = (goalData.currentAmount / goalData.targetAmount) * 100
    const amountRemaining = Math.max(0, goalData.targetAmount - goalData.currentAmount)

    let daysRemaining: number | null = null
    let isOnTrack = true

    if (goalData.targetDate) {
      daysRemaining = Math.ceil((goalData.targetDate - Date.now()) / (1000 * 60 * 60 * 24))

      if (daysRemaining > 0) {
        // Calculate if on track
        const timeElapsed = Date.now() - goalData.createdAt
        const totalTime = goalData.targetDate - goalData.createdAt
        const expectedProgress = timeElapsed / totalTime
        const actualProgress = goalData.currentAmount / goalData.targetAmount

        // Consider on track if within 90% of expected progress
        isOnTrack = actualProgress >= expectedProgress * 0.9
      } else {
        // Past target date
        isOnTrack = percentage >= 100
      }
    }

    const projectedCompletionDate = await savingsGoalService.getProjectedCompletionDate(goalData.id)
    const recommendedMonthlyContribution = await savingsGoalService.getRecommendedContribution(goalData.id)

    return {
      percentage: Math.min(100, percentage),
      amountRemaining,
      isOnTrack,
      projectedCompletionDate,
      daysRemaining,
      recommendedMonthlyContribution,
    }
  }, [])

  const loadGoalProgress = useCallback(async () => {
    try {
      setLoading(true)
      const goalData = await savingsGoalService.getGoal(goalId)

      if (goalData) {
        setGoal(goalData)
        const progressData = await calculateProgress(goalData)
        setProgress(progressData)
      }
    } catch (err) {
      console.error('Failed to load goal progress:', err)
    } finally {
      setLoading(false)
    }
  }, [goalId, calculateProgress])

  useEffect(() => {
    loadGoalProgress()

    // Update progress every minute
    const interval = setInterval(loadGoalProgress, 60000)

    return () => clearInterval(interval)
  }, [loadGoalProgress])

  return {
    goal,
    progress,
    loading,
    refresh: loadGoalProgress,
  }
}
