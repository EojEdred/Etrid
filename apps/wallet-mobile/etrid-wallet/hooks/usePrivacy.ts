"use client"

import { useState, useEffect, useCallback } from "react"
import { PrivacySettings, PrivacyScore } from "@/lib/types/privacy"
import { privacyService } from "@/lib/services/PrivacyService"
import { toast } from "sonner"

export function usePrivacy() {
  const [settings, setSettings] = useState<PrivacySettings | null>(null)
  const [score, setScore] = useState<PrivacyScore | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  // Fetch privacy settings
  const fetchSettings = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const data = await privacyService.getPrivacySettings()
      setSettings(data)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to fetch settings"
      setError(message)
      console.error(message)
    } finally {
      setLoading(false)
    }
  }, [])

  // Fetch privacy score
  const fetchScore = useCallback(async () => {
    try {
      const data = await privacyService.getPrivacyScore()
      setScore(data)
    } catch (err) {
      console.error("Failed to fetch privacy score:", err)
    }
  }, [])

  // Update settings
  const updateSettings = useCallback(async (newSettings: Partial<PrivacySettings>) => {
    setLoading(true)
    setError(null)
    try {
      await privacyService.updateSettings(newSettings)
      await fetchSettings() // Refresh settings
      await fetchScore() // Refresh score
      toast.success("Privacy settings updated!")
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to update settings"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSettings, fetchScore])

  // Set privacy level
  const setPrivacyLevel = useCallback(async (level: "low" | "medium" | "high") => {
    setLoading(true)
    setError(null)
    try {
      await privacyService.setPrivacyLevel(level)
      await fetchSettings()
      await fetchScore()
      toast.success(`Privacy level set to ${level}`)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to set privacy level"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSettings, fetchScore])

  // Enable/disable Tor
  const setTor = useCallback(async (enabled: boolean) => {
    setLoading(true)
    setError(null)
    try {
      if (enabled) {
        await privacyService.enableTor()
      } else {
        await privacyService.disableTor()
      }
      await fetchSettings()
      toast.success(`Tor routing ${enabled ? "enabled" : "disabled"}`)
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed to update Tor setting"
      setError(message)
      toast.error(message)
      throw err
    } finally {
      setLoading(false)
    }
  }, [fetchSettings])

  // Initial fetch
  useEffect(() => {
    fetchSettings()
    fetchScore()
  }, [fetchSettings, fetchScore])

  return {
    settings,
    score,
    loading,
    error,
    updateSettings,
    setPrivacyLevel,
    setTor,
    fetchSettings,
    fetchScore,
  }
}
