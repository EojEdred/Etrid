"use client"

import { useEffect, useState } from 'react'
import { Clock } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'

interface AuctionTimerProps {
  endTime: string
  startTime?: string
  onEnd?: () => void
  showProgress?: boolean
  compact?: boolean
  className?: string
}

export function AuctionTimer({
  endTime,
  startTime,
  onEnd,
  showProgress = false,
  compact = false,
  className,
}: AuctionTimerProps) {
  const [timeRemaining, setTimeRemaining] = useState({
    days: 0,
    hours: 0,
    minutes: 0,
    seconds: 0,
    total: 0,
  })

  useEffect(() => {
    const calculateTimeRemaining = () => {
      const total = new Date(endTime).getTime() - Date.now()

      if (total <= 0) {
        setTimeRemaining({ days: 0, hours: 0, minutes: 0, seconds: 0, total: 0 })
        onEnd?.()
        return
      }

      const seconds = Math.floor((total / 1000) % 60)
      const minutes = Math.floor((total / 1000 / 60) % 60)
      const hours = Math.floor((total / (1000 * 60 * 60)) % 24)
      const days = Math.floor(total / (1000 * 60 * 60 * 24))

      setTimeRemaining({ days, hours, minutes, seconds, total })
    }

    calculateTimeRemaining()
    const interval = setInterval(calculateTimeRemaining, 1000)

    return () => clearInterval(interval)
  }, [endTime, onEnd])

  const isEndingSoon = timeRemaining.total > 0 && timeRemaining.total <= 3600000 // 1 hour

  const progressPercentage = startTime
    ? ((Date.now() - new Date(startTime).getTime()) /
        (new Date(endTime).getTime() - new Date(startTime).getTime())) *
      100
    : 0

  if (compact) {
    return (
      <div className={cn('inline-flex items-center gap-2', className)}>
        <Clock className="w-4 h-4 text-muted-foreground" />
        <span
          className={cn(
            'text-sm font-medium',
            isEndingSoon && 'text-destructive'
          )}
        >
          {timeRemaining.total === 0
            ? 'Ended'
            : timeRemaining.days > 0
            ? `${timeRemaining.days}d ${timeRemaining.hours}h`
            : timeRemaining.hours > 0
            ? `${timeRemaining.hours}h ${timeRemaining.minutes}m`
            : `${timeRemaining.minutes}m ${timeRemaining.seconds}s`}
        </span>
      </div>
    )
  }

  return (
    <Card
      className={cn(
        'p-4 space-y-3',
        isEndingSoon && 'border-destructive/50 bg-destructive/5',
        className
      )}
    >
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Clock className={cn('w-4 h-4', isEndingSoon && 'text-destructive')} />
          <span className="text-sm font-medium">
            {timeRemaining.total === 0 ? 'Auction Ended' : 'Time Remaining'}
          </span>
        </div>
        {isEndingSoon && timeRemaining.total > 0 && (
          <span className="text-xs font-semibold text-destructive">
            Ending Soon!
          </span>
        )}
      </div>

      {/* Timer Display */}
      {timeRemaining.total > 0 ? (
        <div className="grid grid-cols-4 gap-2">
          {[
            { label: 'Days', value: timeRemaining.days },
            { label: 'Hours', value: timeRemaining.hours },
            { label: 'Minutes', value: timeRemaining.minutes },
            { label: 'Seconds', value: timeRemaining.seconds },
          ].map((unit) => (
            <div
              key={unit.label}
              className="flex flex-col items-center p-2 rounded-lg bg-muted"
            >
              <span className="text-2xl font-bold tabular-nums">
                {unit.value.toString().padStart(2, '0')}
              </span>
              <span className="text-xs text-muted-foreground">{unit.label}</span>
            </div>
          ))}
        </div>
      ) : (
        <div className="text-center py-4 text-muted-foreground">
          This auction has ended
        </div>
      )}

      {/* Progress Bar */}
      {showProgress && startTime && timeRemaining.total > 0 && (
        <div className="space-y-1">
          <Progress value={progressPercentage} className="h-2" />
          <p className="text-xs text-muted-foreground text-center">
            {progressPercentage.toFixed(1)}% elapsed
          </p>
        </div>
      )}
    </Card>
  )
}
