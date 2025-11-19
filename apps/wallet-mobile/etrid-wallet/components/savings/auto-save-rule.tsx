/**
 * AutoSaveRule - Rule display and toggle
 */

'use client'

import { Repeat, TrendingUp, Percent, Coins } from 'lucide-react'
import { Switch } from '@/components/ui/switch'
import { Button } from '@/components/ui/button'
import type { AutoSaveRule as AutoSaveRuleType } from '@/lib/services/auto-save-service'

interface AutoSaveRuleProps {
  rule: AutoSaveRuleType
  totalSaved?: number
  onToggle?: (active: boolean) => void
  onEdit?: () => void
}

export function AutoSaveRule({ rule, totalSaved = 0, onToggle, onEdit }: AutoSaveRuleProps) {
  const getIcon = () => {
    switch (rule.type) {
      case 'recurring':
        return <Repeat className="w-5 h-5" />
      case 'roundup':
        return <TrendingUp className="w-5 h-5" />
      case 'percentage':
        return <Percent className="w-5 h-5" />
      case 'spare-change':
        return <Coins className="w-5 h-5" />
    }
  }

  const getDescription = () => {
    switch (rule.type) {
      case 'recurring':
        return `Save ${rule.amount} ÉTR ${rule.frequency}`
      case 'roundup':
        return `Round up purchases ${rule.multiplier}x`
      case 'percentage':
        return `Save ${rule.percentage}% of deposits`
      case 'spare-change':
        return `Save amounts < ${rule.threshold} ÉTR`
    }
  }

  const getColor = () => {
    switch (rule.type) {
      case 'recurring':
        return 'text-accent'
      case 'roundup':
        return 'text-success'
      case 'percentage':
        return 'text-warning'
      case 'spare-change':
        return 'text-purple-500'
    }
  }

  return (
    <div className="glass rounded-2xl p-5">
      <div className="flex items-start gap-4">
        {/* Icon */}
        <div className={`w-12 h-12 rounded-full bg-accent/20 flex items-center justify-center ${getColor()}`}>
          {getIcon()}
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center justify-between mb-2">
            <h4 className="font-semibold capitalize">{rule.type.replace('-', ' ')} Rule</h4>
            {onToggle && (
              <Switch
                checked={rule.active}
                onCheckedChange={onToggle}
              />
            )}
          </div>

          <p className="text-sm text-muted-foreground mb-3">{getDescription()}</p>

          {/* Stats */}
          <div className="glass-strong rounded-xl p-3 space-y-2">
            <div className="flex items-center justify-between">
              <span className="text-xs text-muted-foreground">Total Saved</span>
              <span className="font-semibold text-success">{totalSaved.toFixed(2)} ÉTR</span>
            </div>

            {rule.lastExecuted && (
              <div className="flex items-center justify-between">
                <span className="text-xs text-muted-foreground">Last Executed</span>
                <span className="text-xs">
                  {new Date(rule.lastExecuted).toLocaleDateString()}
                </span>
              </div>
            )}

            {rule.nextExecution && (
              <div className="flex items-center justify-between">
                <span className="text-xs text-muted-foreground">Next Execution</span>
                <span className="text-xs">
                  {new Date(rule.nextExecution).toLocaleDateString()}
                </span>
              </div>
            )}
          </div>

          {/* Edit button */}
          {onEdit && (
            <Button
              variant="outline"
              size="sm"
              onClick={onEdit}
              className="w-full mt-3"
            >
              Edit Rule
            </Button>
          )}
        </div>
      </div>
    </div>
  )
}
