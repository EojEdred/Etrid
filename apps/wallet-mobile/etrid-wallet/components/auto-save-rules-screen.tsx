/**
 * AutoSaveRulesScreen - Manage auto-save rules
 */

'use client'

import { useState } from 'react'
import { ArrowLeft, Plus } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { AutoSaveRule } from '@/components/savings/auto-save-rule'
import { useAutoSave } from '@/hooks/use-auto-save'

interface AutoSaveRulesScreenProps {
  goalId: string
  onBack: () => void
}

export function AutoSaveRulesScreen({ goalId, onBack }: AutoSaveRulesScreenProps) {
  const { rules, ruleStats, toggleRule, loading } = useAutoSave(goalId)

  return (
    <div className="min-h-screen pb-24">
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Auto-Save Rules</h1>
            <p className="text-sm text-muted-foreground">Automate your savings</p>
          </div>
          <Button size="icon" style={{ background: '#00d9ff', color: '#000' }}>
            <Plus className="w-5 h-5" />
          </Button>
        </div>

        <div className="glass-strong rounded-xl p-4 space-y-2">
          <h3 className="font-semibold">Rule Types</h3>
          <div className="grid grid-cols-2 gap-2 text-xs">
            <div>
              <span className="text-accent">●</span> Recurring - Save on schedule
            </div>
            <div>
              <span className="text-success">●</span> Round-up - Spare change
            </div>
            <div>
              <span className="text-warning">●</span> Percentage - % of income
            </div>
            <div>
              <span className="text-purple-500">●</span> Spare Change - Small amounts
            </div>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-4">
        {rules.length === 0 ? (
          <div className="glass rounded-2xl p-8 text-center">
            <p className="text-muted-foreground">No auto-save rules yet</p>
            <p className="text-sm text-muted-foreground mt-2">
              Create rules to automate your savings
            </p>
          </div>
        ) : (
          rules.map((rule) => (
            <AutoSaveRule
              key={rule.id}
              rule={rule}
              totalSaved={ruleStats.get(rule.id)?.totalSaved || 0}
              onToggle={(active) => toggleRule(rule.id, active)}
            />
          ))
        )}
      </main>
    </div>
  )
}
