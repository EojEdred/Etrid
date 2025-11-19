/**
 * SavingsGoalService - Manage savings goals
 */

import type { Transaction } from './lending-service'

export interface GoalInput {
  name: string
  category: string
  targetAmount: number
  targetDate?: number
  initialContribution?: number
  icon?: string
}

export interface SavingsGoal {
  id: string
  userId: string
  name: string
  category: string
  targetAmount: number
  currentAmount: number
  targetDate?: number
  icon: string
  status: 'active' | 'completed' | 'archived'
  createdAt: number
  completedAt?: number
  milestones: GoalMilestone[]
}

export interface GoalMilestone {
  percentage: number
  achieved: boolean
  achievedAt?: number
}

export interface GoalContribution {
  id: string
  goalId: string
  amount: number
  source: 'manual' | 'auto-recurring' | 'auto-roundup' | 'auto-percentage' | 'auto-spare'
  timestamp: number
}

export interface GoalStats {
  totalSaved: number
  totalGoals: number
  completedGoals: number
  activeGoals: number
  averageProgress: number
  onTrackGoals: number
}

export class SavingsGoalService {
  private static instance: SavingsGoalService

  private goals: SavingsGoal[] = []
  private contributions: GoalContribution[] = []
  private currentUserId = 'user_123'

  static getInstance(): SavingsGoalService {
    if (!SavingsGoalService.instance) {
      SavingsGoalService.instance = new SavingsGoalService()
      // Initialize with mock data
      SavingsGoalService.instance.initializeMockData()
    }
    return SavingsGoalService.instance
  }

  /**
   * Create a new savings goal
   */
  async createGoal(input: GoalInput): Promise<SavingsGoal> {
    try {
      const goal: SavingsGoal = {
        id: `goal_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        userId: this.currentUserId,
        name: input.name,
        category: input.category,
        targetAmount: input.targetAmount,
        currentAmount: input.initialContribution || 0,
        targetDate: input.targetDate,
        icon: input.icon || this.getDefaultIcon(input.category),
        status: 'active',
        createdAt: Date.now(),
        milestones: [
          { percentage: 25, achieved: false },
          { percentage: 50, achieved: false },
          { percentage: 75, achieved: false },
          { percentage: 100, achieved: false },
        ],
      }

      this.goals.push(goal)

      // Add initial contribution if provided
      if (input.initialContribution && input.initialContribution > 0) {
        await this.contributeToGoal(goal.id, input.initialContribution, 'manual')
      }

      return goal
    } catch (error) {
      throw new Error(`Failed to create goal: ${error}`)
    }
  }

  /**
   * Get all goals with optional filter
   */
  async getGoals(filter?: 'active' | 'completed' | 'archived'): Promise<SavingsGoal[]> {
    let filteredGoals = [...this.goals]

    if (filter) {
      filteredGoals = filteredGoals.filter(g => g.status === filter)
    }

    // Update milestone achievements
    filteredGoals.forEach(goal => this.updateMilestones(goal))

    return filteredGoals
  }

  /**
   * Get a single goal by ID
   */
  async getGoal(id: string): Promise<SavingsGoal | null> {
    const goal = this.goals.find(g => g.id === id)
    if (goal) {
      this.updateMilestones(goal)
    }
    return goal || null
  }

  /**
   * Update a goal
   */
  async updateGoal(id: string, updates: Partial<SavingsGoal>): Promise<SavingsGoal> {
    const goal = this.goals.find(g => g.id === id)
    if (!goal) {
      throw new Error('Goal not found')
    }

    // Update allowed fields
    if (updates.name !== undefined) goal.name = updates.name
    if (updates.targetAmount !== undefined) goal.targetAmount = updates.targetAmount
    if (updates.targetDate !== undefined) goal.targetDate = updates.targetDate
    if (updates.icon !== undefined) goal.icon = updates.icon
    if (updates.status !== undefined) goal.status = updates.status

    return goal
  }

  /**
   * Delete a goal
   */
  async deleteGoal(id: string): Promise<void> {
    const index = this.goals.findIndex(g => g.id === id)
    if (index === -1) {
      throw new Error('Goal not found')
    }

    this.goals.splice(index, 1)
    // Also remove contributions
    this.contributions = this.contributions.filter(c => c.goalId !== id)
  }

  /**
   * Contribute to a goal
   */
  async contributeToGoal(
    id: string,
    amount: number,
    source: GoalContribution['source'] = 'manual'
  ): Promise<Transaction> {
    try {
      const goal = this.goals.find(g => g.id === id)
      if (!goal) {
        throw new Error('Goal not found')
      }

      if (goal.status !== 'active') {
        throw new Error('Cannot contribute to non-active goal')
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // Add contribution
      const contribution: GoalContribution = {
        id: `contrib_${Date.now()}`,
        goalId: id,
        amount,
        source,
        timestamp: Date.now(),
      }

      this.contributions.push(contribution)

      // Update goal amount
      goal.currentAmount += amount

      // Check if goal is completed
      if (goal.currentAmount >= goal.targetAmount) {
        goal.status = 'completed'
        goal.completedAt = Date.now()
      }

      // Update milestones
      this.updateMilestones(goal)

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to contribute to goal: ${error}`)
    }
  }

  /**
   * Withdraw from a goal
   */
  async withdrawFromGoal(id: string, amount: number): Promise<Transaction> {
    try {
      const goal = this.goals.find(g => g.id === id)
      if (!goal) {
        throw new Error('Goal not found')
      }

      if (amount > goal.currentAmount) {
        throw new Error('Insufficient funds in goal')
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      goal.currentAmount -= amount

      // Update milestones
      this.updateMilestones(goal)

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to withdraw from goal: ${error}`)
    }
  }

  /**
   * Get goal contributions
   */
  async getGoalContributions(goalId: string): Promise<GoalContribution[]> {
    return this.contributions
      .filter(c => c.goalId === goalId)
      .sort((a, b) => b.timestamp - a.timestamp)
  }

  /**
   * Get goal statistics
   */
  async getGoalStats(): Promise<GoalStats> {
    const activeGoals = this.goals.filter(g => g.status === 'active')
    const completedGoals = this.goals.filter(g => g.status === 'completed')

    const totalSaved = this.goals.reduce((sum, g) => sum + g.currentAmount, 0)

    const averageProgress = activeGoals.length > 0
      ? activeGoals.reduce((sum, g) => sum + (g.currentAmount / g.targetAmount * 100), 0) / activeGoals.length
      : 0

    // Count goals that are on track
    const onTrackGoals = activeGoals.filter(g => {
      if (!g.targetDate) return true // No deadline = always on track

      const progress = g.currentAmount / g.targetAmount
      const timeElapsed = Date.now() - g.createdAt
      const totalTime = g.targetDate - g.createdAt
      const expectedProgress = timeElapsed / totalTime

      return progress >= expectedProgress * 0.9 // Within 90% of expected progress
    }).length

    return {
      totalSaved,
      totalGoals: this.goals.length,
      completedGoals: completedGoals.length,
      activeGoals: activeGoals.length,
      averageProgress,
      onTrackGoals,
    }
  }

  /**
   * Calculate projected completion date
   */
  async getProjectedCompletionDate(goalId: string): Promise<number | null> {
    const goal = this.goals.find(g => g.id === goalId)
    if (!goal || goal.status !== 'active') return null

    const contributions = this.contributions.filter(c => c.goalId === goalId)
    if (contributions.length < 2) return null

    // Calculate average contribution rate
    const sortedContribs = contributions.sort((a, b) => a.timestamp - b.timestamp)
    const firstContrib = sortedContribs[0]
    const timeElapsed = Date.now() - firstContrib.timestamp

    if (timeElapsed === 0) return null

    const averageRate = goal.currentAmount / timeElapsed // amount per millisecond
    const remaining = goal.targetAmount - goal.currentAmount

    if (remaining <= 0) return Date.now()

    const timeToComplete = remaining / averageRate
    return Date.now() + timeToComplete
  }

  /**
   * Get recommended contribution amount
   */
  async getRecommendedContribution(goalId: string): Promise<number> {
    const goal = this.goals.find(g => g.id === goalId)
    if (!goal || goal.status !== 'active' || !goal.targetDate) return 0

    const remaining = goal.targetAmount - goal.currentAmount
    const timeRemaining = goal.targetDate - Date.now()

    if (timeRemaining <= 0) return remaining // Overdue

    // Calculate monthly contribution needed
    const monthsRemaining = timeRemaining / (1000 * 60 * 60 * 24 * 30)
    return remaining / monthsRemaining
  }

  // Helper methods

  private updateMilestones(goal: SavingsGoal): void {
    const progress = (goal.currentAmount / goal.targetAmount) * 100

    goal.milestones.forEach(milestone => {
      if (progress >= milestone.percentage && !milestone.achieved) {
        milestone.achieved = true
        milestone.achievedAt = Date.now()
      } else if (progress < milestone.percentage && milestone.achieved) {
        // Reset if withdrawn below milestone
        milestone.achieved = false
        milestone.achievedAt = undefined
      }
    })
  }

  private getDefaultIcon(category: string): string {
    const icons: Record<string, string> = {
      'Emergency Fund': '🛡️',
      'Vacation': '✈️',
      'House': '🏠',
      'Car': '🚗',
      'Education': '🎓',
      'Wedding': '💒',
      'Retirement': '🏖️',
      'Electronics': '💻',
      'Other': '🎯',
    }
    return icons[category] || '🎯'
  }

  private initializeMockData(): void {
    // Add some mock goals
    const now = Date.now()
    const oneMonth = 30 * 24 * 60 * 60 * 1000
    const sixMonths = 6 * oneMonth
    const oneYear = 12 * oneMonth

    this.goals = [
      {
        id: 'goal_1',
        userId: this.currentUserId,
        name: 'Emergency Fund',
        category: 'Emergency Fund',
        targetAmount: 10000,
        currentAmount: 6500,
        targetDate: now + sixMonths,
        icon: '🛡️',
        status: 'active',
        createdAt: now - oneMonth * 3,
        milestones: [
          { percentage: 25, achieved: true, achievedAt: now - oneMonth * 2.5 },
          { percentage: 50, achieved: true, achievedAt: now - oneMonth * 1.5 },
          { percentage: 75, achieved: false },
          { percentage: 100, achieved: false },
        ],
      },
      {
        id: 'goal_2',
        userId: this.currentUserId,
        name: 'Dream Vacation',
        category: 'Vacation',
        targetAmount: 5000,
        currentAmount: 1250,
        targetDate: now + oneYear,
        icon: '✈️',
        status: 'active',
        createdAt: now - oneMonth * 2,
        milestones: [
          { percentage: 25, achieved: true, achievedAt: now - oneMonth },
          { percentage: 50, achieved: false },
          { percentage: 75, achieved: false },
          { percentage: 100, achieved: false },
        ],
      },
      {
        id: 'goal_3',
        userId: this.currentUserId,
        name: 'New Laptop',
        category: 'Electronics',
        targetAmount: 2000,
        currentAmount: 2000,
        icon: '💻',
        status: 'completed',
        createdAt: now - oneMonth * 4,
        completedAt: now - oneMonth,
        milestones: [
          { percentage: 25, achieved: true, achievedAt: now - oneMonth * 3.5 },
          { percentage: 50, achieved: true, achievedAt: now - oneMonth * 3 },
          { percentage: 75, achieved: true, achievedAt: now - oneMonth * 2 },
          { percentage: 100, achieved: true, achievedAt: now - oneMonth },
        ],
      },
    ]

    // Update milestones
    this.goals.forEach(goal => this.updateMilestones(goal))
  }
}

export const savingsGoalService = SavingsGoalService.getInstance()
