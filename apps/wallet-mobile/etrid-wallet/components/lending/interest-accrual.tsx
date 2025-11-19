/**
 * InterestAccrual - Real-time interest counter
 */

'use client'

import { TrendingUp } from 'lucide-react'
import { useInterestAccrual, type TimePeriod } from '@/hooks/use-interest-accrual'
import { Button } from '@/components/ui/button'

interface InterestAccrualProps {
  principal: number
  apy: number
  asset: string
  type: 'earning' | 'owing'
  startDate?: number
}

export function InterestAccrual({
  principal,
  apy,
  asset,
  type,
  startDate,
}: InterestAccrualProps) {
  const {
    interestEarned,
    timePeriod,
    setTimePeriod,
    getProjectedInterest,
    getNextPayoutDate,
  } = useInterestAccrual(principal, apy, startDate)

  const isEarning = type === 'earning'
  const periods: TimePeriod[] = ['daily', 'monthly', 'yearly']

  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
    })
  }

  return (
    <div className="glass rounded-2xl p-5 space-y-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <TrendingUp className={`w-5 h-5 ${isEarning ? 'text-success' : 'text-destructive'}`} />
          <h3 className="font-semibold">
            {isEarning ? 'Interest Earning' : 'Interest Owing'}
          </h3>
        </div>
      </div>

      {/* Real-time counter */}
      <div className="glass-strong rounded-xl p-4 text-center">
        <p className="text-xs text-muted-foreground mb-2">Accrued So Far</p>
        <p className={`text-3xl font-bold ${isEarning ? 'text-success' : 'text-destructive'}`}>
          {isEarning ? '+' : '-'}{interestEarned.toFixed(6)} {asset}
        </p>
        <p className="text-xs text-muted-foreground mt-2">
          At {apy}% APY
        </p>
      </div>

      {/* Period selector */}
      <div className="flex gap-2">
        {periods.map((period) => (
          <Button
            key={period}
            size="sm"
            variant={timePeriod === period ? 'default' : 'outline'}
            onClick={() => setTimePeriod(period)}
            className="flex-1 capitalize"
            style={
              timePeriod === period
                ? { background: '#00d9ff', color: '#000' }
                : undefined
            }
          >
            {period}
          </Button>
        ))}
      </div>

      {/* Projected interest */}
      <div className="grid grid-cols-3 gap-3 pt-3 border-t border-border">
        {periods.map((period) => {
          const projected = getProjectedInterest(period)
          return (
            <div key={period}>
              <p className="text-xs text-muted-foreground mb-1 capitalize">{period}</p>
              <p className={`font-semibold ${isEarning ? 'text-success' : 'text-destructive'}`}>
                {isEarning ? '+' : '-'}{projected.toFixed(4)}
              </p>
            </div>
          )
        })}
      </div>

      {/* Next payout */}
      {isEarning && (
        <div className="glass-strong rounded-xl p-3">
          <div className="flex items-center justify-between">
            <span className="text-xs text-muted-foreground">Next Payout</span>
            <span className="font-semibold">{formatDate(getNextPayoutDate())}</span>
          </div>
        </div>
      )}
    </div>
  )
}
