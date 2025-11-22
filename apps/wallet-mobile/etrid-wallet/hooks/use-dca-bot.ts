// useDCABot Hook - Manage DCA (Dollar Cost Averaging) bots

import { useState, useEffect, useCallback } from 'react'
import { alertService } from '@/lib/services/alert.service'
import type { DCABot, DCABotInput } from '@/lib/types/trading'

export function useDCABot() {
  const [bots, setBots] = useState<DCABot[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchBots = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const data = await alertService.getDCABots()
      setBots(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchBots()
  }, [fetchBots])

  const createBot = useCallback(
    async (bot: DCABotInput) => {
      try {
        const newBot = await alertService.createDCABot(bot)
        await fetchBots()
        return newBot
      } catch (err) {
        throw err
      }
    },
    [fetchBots]
  )

  const startBot = useCallback(
    async (id: string) => {
      try {
        await alertService.toggleDCABot(id, true)
        await fetchBots()
      } catch (err) {
        throw err
      }
    },
    [fetchBots]
  )

  const stopBot = useCallback(
    async (id: string) => {
      try {
        await alertService.toggleDCABot(id, false)
        await fetchBots()
      } catch (err) {
        throw err
      }
    },
    [fetchBots]
  )

  const deleteBot = useCallback(
    async (id: string) => {
      try {
        await alertService.deleteDCABot(id)
        await fetchBots()
      } catch (err) {
        throw err
      }
    },
    [fetchBots]
  )

  const getBotMetrics = useCallback(async (id: string) => {
    try {
      const metrics = await alertService.getDCABotMetrics(id)
      return metrics
    } catch (err) {
      throw err
    }
  }, [])

  const getActiveBots = useCallback(() => {
    return bots.filter((bot) => bot.active)
  }, [bots])

  const getInactiveBots = useCallback(() => {
    return bots.filter((bot) => !bot.active)
  }, [bots])

  return {
    bots,
    loading,
    error,
    createBot,
    startBot,
    stopBot,
    deleteBot,
    getBotMetrics,
    getActiveBots,
    getInactiveBots,
    refresh: fetchBots,
  }
}
