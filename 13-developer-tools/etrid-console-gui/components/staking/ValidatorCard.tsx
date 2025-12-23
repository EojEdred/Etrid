'use client'

import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Checkbox } from '@/components/ui/checkbox'
import { CheckCircle2, AlertCircle, Globe, ExternalLink } from 'lucide-react'
import type { Validator } from '@/lib/polkadot/staking'
import { formatETR, formatCommission, truncateAddress } from '@/hooks/useStaking'

interface ValidatorCardProps {
  validator: Validator
  isSelected?: boolean
  onToggleSelect?: (address: string) => void
  selectable?: boolean
}

export default function ValidatorCard({
  validator,
  isSelected = false,
  onToggleSelect,
  selectable = false,
}: ValidatorCardProps) {
  const handleClick = () => {
    if (selectable && onToggleSelect) {
      onToggleSelect(validator.address)
    }
  }

  const commissionPercentage = formatCommission(validator.commission)

  return (
    <Card
      className={`p-4 hover:border-accent/50 transition-all glass-card ${
        selectable ? 'cursor-pointer' : ''
      } ${isSelected ? 'border-accent' : ''}`}
      onClick={handleClick}
    >
      <div className="space-y-3">
        {/* Header with Selection */}
        <div className="flex items-start justify-between gap-3">
          <div className="flex items-start gap-3 flex-1 min-w-0">
            {selectable && (
              <Checkbox
                checked={isSelected}
                onCheckedChange={() => onToggleSelect?.(validator.address)}
                onClick={(e) => e.stopPropagation()}
                className="mt-1"
              />
            )}

            <div className="flex-1 min-w-0">
              {/* Identity or Address */}
              <div className="flex items-center gap-2 mb-1">
                {validator.identity?.display ? (
                  <>
                    <h3 className="text-base font-bold truncate">
                      {validator.identity.display}
                    </h3>
                    {validator.identity.web && (
                      <a
                        href={
                          validator.identity.web.startsWith('http')
                            ? validator.identity.web
                            : `https://${validator.identity.web}`
                        }
                        target="_blank"
                        rel="noopener noreferrer"
                        onClick={(e) => e.stopPropagation()}
                        className="text-accent hover:text-accent/80"
                      >
                        <Globe className="w-3 h-3" />
                      </a>
                    )}
                  </>
                ) : (
                  <h3 className="text-sm font-mono text-muted-foreground truncate">
                    {truncateAddress(validator.address)}
                  </h3>
                )}
              </div>

              {/* Address (if identity exists) */}
              {validator.identity?.display && (
                <p className="text-xs font-mono text-muted-foreground truncate">
                  {truncateAddress(validator.address, 8, 6)}
                </p>
              )}
            </div>
          </div>

          {/* Status Badge */}
          <Badge
            className={
              validator.isActive
                ? 'bg-green-500/10 text-green-500 border-green-500/20'
                : 'bg-gray-500/10 text-gray-500 border-gray-500/20'
            }
          >
            {validator.isActive ? 'Active' : 'Waiting'}
          </Badge>
        </div>

        {/* Stats Grid */}
        <div className="grid grid-cols-3 gap-2 text-xs">
          {/* Commission */}
          <div className="space-y-1">
            <p className="text-muted-foreground">Commission</p>
            <p className="font-bold text-sm">{commissionPercentage}%</p>
          </div>

          {/* Total Stake */}
          <div className="space-y-1">
            <p className="text-muted-foreground">Total Stake</p>
            <p className="font-bold text-sm">{formatETR(validator.totalStake)} ETR</p>
          </div>

          {/* Nominators */}
          <div className="space-y-1">
            <p className="text-muted-foreground">Nominators</p>
            <p className="font-bold text-sm">{validator.nominatorCount}</p>
          </div>
        </div>

        {/* Own Stake */}
        {validator.isActive && (
          <div className="flex items-center justify-between text-xs pt-2 border-t border-white/5">
            <span className="text-muted-foreground">Own Stake</span>
            <span className="font-semibold">{formatETR(validator.ownStake)} ETR</span>
          </div>
        )}

        {/* Warnings */}
        {validator.blocked && (
          <div className="flex items-center gap-2 text-xs text-red-500 pt-2 border-t border-red-500/20">
            <AlertCircle className="w-3 h-3" />
            <span>Not accepting nominations</span>
          </div>
        )}

        {/* Selection Indicator */}
        {isSelected && (
          <div className="flex items-center gap-2 text-xs text-accent pt-2 border-t border-accent/20">
            <CheckCircle2 className="w-3 h-3" />
            <span>Selected for nomination</span>
          </div>
        )}
      </div>
    </Card>
  )
}
