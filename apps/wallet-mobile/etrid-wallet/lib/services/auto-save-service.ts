/**
 * AutoSaveService - Automated savings rules
 */

import type { Transaction } from './lending-service'
import { savingsGoalService } from './savings-goal-service'

export type RuleType = 'recurring' | 'roundup' | 'percentage' | 'spare-change'

export interface AutoSaveRuleInput {
  goalId: string
  type: RuleType
  amount?: number // For recurring rules
  frequency?: 'daily' | 'weekly' | 'monthly' // For recurring rules
  multiplier?: number // For roundup rules (1x, 2x, 5x, 10x)
  percentage?: number // For percentage rules
  threshold?: number // For spare-change rules
  active?: boolean
}

export interface AutoSaveRule {
  id: string
  userId: string
  goalId: string
  type: RuleType
  amount?: number
  frequency?: 'daily' | 'weekly' | 'monthly'
  multiplier?: number
  percentage?: number
  threshold?: number
  active: boolean
  createdAt: number
  lastExecuted?: number
  nextExecution?: number
}

export interface RuleStats {
  totalSaved: number
  executionCount: number
  averageAmount: number
  lastExecution?: number
}

export interface RuleExecution {
  id: string
  ruleId: string
  amount: number
  timestamp: number
  success: boolean
  error?: string
}

export class AutoSaveService {
  private static instance: AutoSaveService

  private rules: AutoSaveRule[] = []
  private executions: RuleExecution[] = []
  private currentUserId = 'user_123'

  static getInstance(): AutoSaveService {
    if (!AutoSaveService.instance) {
      AutoSaveService.instance = new AutoSaveService()
      // Initialize with mock data
      AutoSaveService.instance.initializeMockData()
    }
    return AutoSaveService.instance
  }

  /**
   * Create a new auto-save rule
   */
  async createRule(input: AutoSaveRuleInput): Promise<AutoSaveRule> {
    try {
      // Validate goal exists
      const goal = await savingsGoalService.getGoal(input.goalId)
      if (!goal) {
        throw new Error('Goal not found')
      }

      if (goal.status !== 'active') {
        throw new Error('Cannot create rule for non-active goal')
      }

      // Validate rule parameters
      this.validateRuleInput(input)

      const rule: AutoSaveRule = {
        id: `rule_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        userId: this.currentUserId,
        goalId: input.goalId,
        type: input.type,
        amount: input.amount,
        frequency: input.frequency,
        multiplier: input.multiplier,
        percentage: input.percentage,
        threshold: input.threshold,
        active: input.active !== undefined ? input.active : true,
        createdAt: Date.now(),
      }

      // Set next execution for recurring rules
      if (rule.type === 'recurring' && rule.frequency) {
        rule.nextExecution = this.calculateNextExecution(Date.now(), rule.frequency)
      }

      this.rules.push(rule)
      return rule
    } catch (error) {
      throw new Error(`Failed to create auto-save rule: ${error}`)
    }
  }

  /**
   * Get all rules for current user
   */
  async getRules(goalId?: string): Promise<AutoSaveRule[]> {
    let filteredRules = this.rules.filter(r => r.userId === this.currentUserId)

    if (goalId) {
      filteredRules = filteredRules.filter(r => r.goalId === goalId)
    }

    return filteredRules
  }

  /**
   * Get a specific rule
   */
  async getRule(id: string): Promise<AutoSaveRule | null> {
    return this.rules.find(r => r.id === id) || null
  }

  /**
   * Update a rule
   */
  async updateRule(id: string, updates: Partial<AutoSaveRule>): Promise<AutoSaveRule> {
    const rule = this.rules.find(r => r.id === id)
    if (!rule) {
      throw new Error('Rule not found')
    }

    // Update allowed fields
    if (updates.amount !== undefined) rule.amount = updates.amount
    if (updates.frequency !== undefined) {
      rule.frequency = updates.frequency
      if (rule.type === 'recurring') {
        rule.nextExecution = this.calculateNextExecution(Date.now(), rule.frequency)
      }
    }
    if (updates.multiplier !== undefined) rule.multiplier = updates.multiplier
    if (updates.percentage !== undefined) rule.percentage = updates.percentage
    if (updates.threshold !== undefined) rule.threshold = updates.threshold
    if (updates.active !== undefined) rule.active = updates.active

    return rule
  }

  /**
   * Delete a rule
   */
  async deleteRule(id: string): Promise<void> {
    const index = this.rules.findIndex(r => r.id === id)
    if (index === -1) {
      throw new Error('Rule not found')
    }

    this.rules.splice(index, 1)
    // Also remove executions
    this.executions = this.executions.filter(e => e.ruleId !== id)
  }

  /**
   * Execute a rule manually
   */
  async executeRule(ruleId: string): Promise<Transaction> {
    try {
      const rule = this.rules.find(r => r.id === ruleId)
      if (!rule) {
        throw new Error('Rule not found')
      }

      if (!rule.active) {
        throw new Error('Rule is not active')
      }

      // Calculate amount based on rule type
      let amount: number

      switch (rule.type) {
        case 'recurring':
          if (!rule.amount) {
            throw new Error('Recurring rule requires amount')
          }
          amount = rule.amount
          break

        case 'roundup':
          // For manual execution, use a mock transaction amount
          amount = this.calculateRoundupAmount(47.35, rule.multiplier || 1)
          break

        case 'percentage':
          // For manual execution, use a mock deposit amount
          amount = this.calculatePercentageAmount(500, rule.percentage || 10)
          break

        case 'spare-change':
          // For manual execution, use a mock amount
          amount = this.calculateSpareChangeAmount(0.73, rule.threshold || 1)
          break

        default:
          throw new Error(`Unknown rule type: ${rule.type}`)
      }

      // Execute contribution
      const transaction = await savingsGoalService.contributeToGoal(
        rule.goalId,
        amount,
        this.getContributionSource(rule.type)
      )

      // Record execution
      const execution: RuleExecution = {
        id: `exec_${Date.now()}`,
        ruleId: rule.id,
        amount,
        timestamp: Date.now(),
        success: true,
      }

      this.executions.push(execution)

      // Update rule
      rule.lastExecuted = Date.now()
      if (rule.type === 'recurring' && rule.frequency) {
        rule.nextExecution = this.calculateNextExecution(Date.now(), rule.frequency)
      }

      return transaction
    } catch (error) {
      // Record failed execution
      const execution: RuleExecution = {
        id: `exec_${Date.now()}`,
        ruleId: ruleId,
        amount: 0,
        timestamp: Date.now(),
        success: false,
        error: String(error),
      }

      this.executions.push(execution)
      throw error
    }
  }

  /**
   * Get rule statistics
   */
  async getRuleStats(ruleId: string): Promise<RuleStats> {
    const executions = this.executions.filter(
      e => e.ruleId === ruleId && e.success
    )

    const totalSaved = executions.reduce((sum, e) => sum + e.amount, 0)
    const executionCount = executions.length
    const averageAmount = executionCount > 0 ? totalSaved / executionCount : 0
    const lastExecution = executions.length > 0
      ? Math.max(...executions.map(e => e.timestamp))
      : undefined

    return {
      totalSaved,
      executionCount,
      averageAmount,
      lastExecution,
    }
  }

  /**
   * Process a transaction through all applicable rules
   */
  async processTransaction(
    type: 'incoming' | 'outgoing',
    amount: number
  ): Promise<void> {
    const activeRules = this.rules.filter(r => r.active)

    for (const rule of activeRules) {
      try {
        let saveAmount: number | null = null

        switch (rule.type) {
          case 'roundup':
            if (type === 'outgoing') {
              saveAmount = this.calculateRoundupAmount(amount, rule.multiplier || 1)
            }
            break

          case 'percentage':
            if (type === 'incoming' && rule.percentage) {
              saveAmount = this.calculatePercentageAmount(amount, rule.percentage)
            }
            break

          case 'spare-change':
            saveAmount = this.calculateSpareChangeAmount(amount, rule.threshold || 1)
            break
        }

        if (saveAmount && saveAmount > 0) {
          await this.executeRuleWithAmount(rule.id, saveAmount)
        }
      } catch (error) {
        console.error(`Failed to execute rule ${rule.id}:`, error)
      }
    }
  }

  /**
   * Get total saved across all rules
   */
  async getTotalSaved(): Promise<number> {
    const successfulExecutions = this.executions.filter(e => e.success)
    return successfulExecutions.reduce((sum, e) => sum + e.amount, 0)
  }

  // Helper methods

  private validateRuleInput(input: AutoSaveRuleInput): void {
    switch (input.type) {
      case 'recurring':
        if (!input.amount || input.amount <= 0) {
          throw new Error('Recurring rules require a positive amount')
        }
        if (!input.frequency) {
          throw new Error('Recurring rules require a frequency')
        }
        break

      case 'roundup':
        if (input.multiplier && (input.multiplier < 1 || input.multiplier > 10)) {
          throw new Error('Round-up multiplier must be between 1 and 10')
        }
        break

      case 'percentage':
        if (!input.percentage || input.percentage <= 0 || input.percentage > 100) {
          throw new Error('Percentage must be between 0 and 100')
        }
        break

      case 'spare-change':
        if (input.threshold && input.threshold <= 0) {
          throw new Error('Spare-change threshold must be positive')
        }
        break
    }
  }

  private calculateNextExecution(
    fromDate: number,
    frequency: 'daily' | 'weekly' | 'monthly'
  ): number {
    const date = new Date(fromDate)

    switch (frequency) {
      case 'daily':
        date.setDate(date.getDate() + 1)
        break
      case 'weekly':
        date.setDate(date.getDate() + 7)
        break
      case 'monthly':
        date.setMonth(date.getMonth() + 1)
        break
    }

    return date.getTime()
  }

  private calculateRoundupAmount(transactionAmount: number, multiplier: number): number {
    const rounded = Math.ceil(transactionAmount)
    const difference = rounded - transactionAmount
    return difference * multiplier
  }

  private calculatePercentageAmount(depositAmount: number, percentage: number): number {
    return depositAmount * (percentage / 100)
  }

  private calculateSpareChangeAmount(amount: number, threshold: number): number {
    return amount < threshold ? amount : 0
  }

  private getContributionSource(
    ruleType: RuleType
  ): 'manual' | 'auto-recurring' | 'auto-roundup' | 'auto-percentage' | 'auto-spare' {
    switch (ruleType) {
      case 'recurring':
        return 'auto-recurring'
      case 'roundup':
        return 'auto-roundup'
      case 'percentage':
        return 'auto-percentage'
      case 'spare-change':
        return 'auto-spare'
    }
  }

  private async executeRuleWithAmount(ruleId: string, amount: number): Promise<void> {
    const rule = this.rules.find(r => r.id === ruleId)
    if (!rule || !rule.active) return

    try {
      await savingsGoalService.contributeToGoal(
        rule.goalId,
        amount,
        this.getContributionSource(rule.type)
      )

      const execution: RuleExecution = {
        id: `exec_${Date.now()}`,
        ruleId: rule.id,
        amount,
        timestamp: Date.now(),
        success: true,
      }

      this.executions.push(execution)
      rule.lastExecuted = Date.now()
    } catch (error) {
      const execution: RuleExecution = {
        id: `exec_${Date.now()}`,
        ruleId: rule.id,
        amount: 0,
        timestamp: Date.now(),
        success: false,
        error: String(error),
      }

      this.executions.push(execution)
    }
  }

  private initializeMockData(): void {
    // Add some mock rules
    const now = Date.now()

    this.rules = [
      {
        id: 'rule_1',
        userId: this.currentUserId,
        goalId: 'goal_1', // Emergency Fund
        type: 'recurring',
        amount: 500,
        frequency: 'monthly',
        active: true,
        createdAt: now - 90 * 24 * 60 * 60 * 1000,
        lastExecuted: now - 15 * 24 * 60 * 60 * 1000,
        nextExecution: now + 15 * 24 * 60 * 60 * 1000,
      },
      {
        id: 'rule_2',
        userId: this.currentUserId,
        goalId: 'goal_2', // Vacation
        type: 'roundup',
        multiplier: 2,
        active: true,
        createdAt: now - 60 * 24 * 60 * 60 * 1000,
        lastExecuted: now - 1 * 24 * 60 * 60 * 1000,
      },
      {
        id: 'rule_3',
        userId: this.currentUserId,
        goalId: 'goal_1', // Emergency Fund
        type: 'percentage',
        percentage: 10,
        active: true,
        createdAt: now - 90 * 24 * 60 * 60 * 1000,
        lastExecuted: now - 7 * 24 * 60 * 60 * 1000,
      },
    ]
  }
}

export const autoSaveService = AutoSaveService.getInstance()
