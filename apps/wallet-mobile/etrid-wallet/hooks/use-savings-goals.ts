/**
 * useSavingsGoals - Manage savings goals
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  savingsGoalService,
  type SavingsGoal,
  type GoalInput,
  type GoalStats,
  type GoalContribution,
} from '@/lib/services/savings-goal-service'
import { useToast } from '@/hooks/use-toast'

export function useSavingsGoals() {
  const [goals, setGoals] = useState<SavingsGoal[]>([])
  const [stats, setStats] = useState<GoalStats | null>(null)
  const [loading, setLoading] = useState(true)
  const { toast } = useToast()

  const loadGoals = useCallback(async (filter?: 'active' | 'completed' | 'archived') => {
    try {
      setLoading(true)
      const [goalsData, statsData] = await Promise.all([
        savingsGoalService.getGoals(filter),
        savingsGoalService.getGoalStats(),
      ])

      setGoals(goalsData)
      setStats(statsData)
    } catch (err) {
      console.error('Failed to load goals:', err)
    } finally {
      setLoading(false)
    }
  }, [])

  const createGoal = useCallback(async (input: GoalInput) => {
    try {
      setLoading(true)
      const goal = await savingsGoalService.createGoal(input)

      toast({
        title: 'Goal Created',
        description: `Created goal: ${input.name}`,
      })

      await loadGoals()
      return goal
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create goal'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadGoals])

  const updateGoal = useCallback(async (id: string, updates: Partial<SavingsGoal>) => {
    try {
      setLoading(true)
      const goal = await savingsGoalService.updateGoal(id, updates)

      toast({
        title: 'Goal Updated',
        description: 'Successfully updated goal',
      })

      await loadGoals()
      return goal
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to update goal'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadGoals])

  const deleteGoal = useCallback(async (id: string) => {
    try {
      setLoading(true)
      await savingsGoalService.deleteGoal(id)

      toast({
        title: 'Goal Deleted',
        description: 'Successfully deleted goal',
      })

      await loadGoals()
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to delete goal'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadGoals])

  const contribute = useCallback(async (id: string, amount: number) => {
    try {
      setLoading(true)
      const tx = await savingsGoalService.contributeToGoal(id, amount, 'manual')

      // Check if goal was completed
      const goal = await savingsGoalService.getGoal(id)
      if (goal?.status === 'completed') {
        toast({
          title: '🎉 Goal Completed!',
          description: `Congratulations! You've reached your ${goal.name} goal!`,
        })
      } else {
        toast({
          title: 'Contribution Added',
          description: `Added ${amount} ÉTR to goal`,
        })
      }

      await loadGoals()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to contribute'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadGoals])

  const withdraw = useCallback(async (id: string, amount: number) => {
    try {
      setLoading(true)
      const tx = await savingsGoalService.withdrawFromGoal(id, amount)

      toast({
        title: 'Withdrawal Successful',
        description: `Withdrew ${amount} ÉTR from goal`,
      })

      await loadGoals()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to withdraw'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadGoals])

  useEffect(() => {
    loadGoals()
  }, [loadGoals])

  return {
    goals,
    stats,
    loading,
    createGoal,
    updateGoal,
    deleteGoal,
    contribute,
    withdraw,
    refresh: loadGoals,
  }
}
