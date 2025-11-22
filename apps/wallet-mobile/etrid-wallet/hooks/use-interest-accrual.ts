/**
 * useInterestAccrual - Real-time interest calculation
 */

'use client'

import { useState, useEffect, useCallback } from 'react'

export type TimePeriod = 'daily' | 'monthly' | 'yearly'

export function useInterestAccrual(
  principal: number,
  apy: number,
  startDate: number = Date.now()
) {
  const [interestEarned, setInterestEarned] = useState(0)
  const [timePeriod, setTimePeriod] = useState<TimePeriod>('monthly')

  const calculateInterest = useCallback(() => {
    const now = Date.now()
    const timeElapsed = (now - startDate) / (1000 * 60 * 60 * 24 * 365) // years
    const totalInterest = principal * (apy / 100) * timeElapsed

    setInterestEarned(totalInterest)
  }, [principal, apy, startDate])

  const getProjectedInterest = useCallback((period: TimePeriod): number => {
    let multiplier: number

    switch (period) {
      case 'daily':
        multiplier = 1 / 365
        break
      case 'monthly':
        multiplier = 1 / 12
        break
      case 'yearly':
        multiplier = 1
        break
    }

    return principal * (apy / 100) * multiplier
  }, [principal, apy])

  const getNextPayoutDate = useCallback((): number => {
    // Assume monthly payouts
    const now = new Date()
    const nextMonth = new Date(now.getFullYear(), now.getMonth() + 1, 1)
    return nextMonth.getTime()
  }, [])

  useEffect(() => {
    // Calculate immediately
    calculateInterest()

    // Update every second for real-time feel
    const interval = setInterval(calculateInterest, 1000)

    return () => clearInterval(interval)
  }, [calculateInterest])

  return {
    interestEarned,
    timePeriod,
    setTimePeriod,
    getProjectedInterest,
    getNextPayoutDate,
  }
}
