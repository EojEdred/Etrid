/**
 * useAutoSave - Manage auto-save rules
 */

'use client'

import { useState, useEffect, useCallback } from 'react'
import {
  autoSaveService,
  type AutoSaveRule,
  type AutoSaveRuleInput,
  type RuleStats,
} from '@/lib/services/auto-save-service'
import { useToast } from '@/hooks/use-toast'

export function useAutoSave(goalId?: string) {
  const [rules, setRules] = useState<AutoSaveRule[]>([])
  const [ruleStats, setRuleStats] = useState<Map<string, RuleStats>>(new Map())
  const [loading, setLoading] = useState(true)
  const { toast } = useToast()

  const loadRules = useCallback(async () => {
    try {
      setLoading(true)
      const rulesData = await autoSaveService.getRules(goalId)
      setRules(rulesData)

      // Load stats for each rule
      const statsMap = new Map<string, RuleStats>()
      await Promise.all(
        rulesData.map(async (rule) => {
          const stats = await autoSaveService.getRuleStats(rule.id)
          statsMap.set(rule.id, stats)
        })
      )
      setRuleStats(statsMap)
    } catch (err) {
      console.error('Failed to load rules:', err)
    } finally {
      setLoading(false)
    }
  }, [goalId])

  const createRule = useCallback(async (input: AutoSaveRuleInput) => {
    try {
      setLoading(true)
      const rule = await autoSaveService.createRule(input)

      toast({
        title: 'Rule Created',
        description: 'Auto-save rule created successfully',
      })

      await loadRules()
      return rule
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create rule'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadRules])

  const updateRule = useCallback(async (id: string, updates: Partial<AutoSaveRule>) => {
    try {
      setLoading(true)
      const rule = await autoSaveService.updateRule(id, updates)

      toast({
        title: 'Rule Updated',
        description: 'Auto-save rule updated successfully',
      })

      await loadRules()
      return rule
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to update rule'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadRules])

  const deleteRule = useCallback(async (id: string) => {
    try {
      setLoading(true)
      await autoSaveService.deleteRule(id)

      toast({
        title: 'Rule Deleted',
        description: 'Auto-save rule deleted successfully',
      })

      await loadRules()
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to delete rule'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadRules])

  const toggleRule = useCallback(async (id: string, active: boolean) => {
    try {
      await autoSaveService.updateRule(id, { active })

      toast({
        title: active ? 'Rule Enabled' : 'Rule Disabled',
        description: `Auto-save rule ${active ? 'enabled' : 'disabled'}`,
      })

      await loadRules()
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to toggle rule'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    }
  }, [toast, loadRules])

  const executeRule = useCallback(async (id: string) => {
    try {
      setLoading(true)
      const tx = await autoSaveService.executeRule(id)

      toast({
        title: 'Rule Executed',
        description: 'Successfully executed auto-save rule',
      })

      await loadRules()
      return tx
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to execute rule'
      toast({
        title: 'Error',
        description: errorMessage,
        variant: 'destructive',
      })
      throw err
    } finally {
      setLoading(false)
    }
  }, [toast, loadRules])

  const getTotalSaved = useCallback(async () => {
    return await autoSaveService.getTotalSaved()
  }, [])

  useEffect(() => {
    loadRules()
  }, [loadRules])

  return {
    rules,
    ruleStats,
    loading,
    createRule,
    updateRule,
    deleteRule,
    toggleRule,
    executeRule,
    getTotalSaved,
    refresh: loadRules,
  }
}
