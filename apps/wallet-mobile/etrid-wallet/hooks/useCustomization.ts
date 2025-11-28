'use client'

import { useState, useEffect } from 'react'
import { customizationService } from '@/lib/services/CustomizationService'
import { themeService } from '@/lib/services/ThemeService'
import { WidgetLayout, Theme } from '@/lib/types/customization'

export function useCustomization() {
  const [widgetLayout, setWidgetLayout] = useState<WidgetLayout | null>(null)
  const [themes, setThemes] = useState<Theme[]>([])
  const [currentThemeId, setCurrentThemeId] = useState<string>('light')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadCustomization()
  }, [])

  const loadCustomization = async () => {
    try {
      setLoading(true)
      setError(null)
      const [layout, themesList] = await Promise.all([
        customizationService.getWidgetLayout(),
        themeService.getThemes(),
      ])
      setWidgetLayout(layout)
      setThemes(themesList)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load customization')
    } finally {
      setLoading(false)
    }
  }

  const updateWidgetLayout = async (layout: WidgetLayout) => {
    try {
      await customizationService.updateWidgetLayout(layout)
      setWidgetLayout(layout)
    } catch (err) {
      throw err
    }
  }

  const resetLayout = async () => {
    try {
      const defaultLayout = await customizationService.resetToDefault()
      setWidgetLayout(defaultLayout)
    } catch (err) {
      throw err
    }
  }

  const setTheme = async (themeId: string) => {
    try {
      await themeService.setTheme(themeId)
      setCurrentThemeId(themeId)
    } catch (err) {
      throw err
    }
  }

  const createTheme = async (theme: any) => {
    try {
      const newTheme = await themeService.createCustomTheme(theme)
      setThemes((prev) => [...prev, newTheme])
      return newTheme
    } catch (err) {
      throw err
    }
  }

  return {
    widgetLayout,
    themes,
    currentThemeId,
    loading,
    error,
    updateWidgetLayout,
    resetLayout,
    setTheme,
    createTheme,
    refresh: loadCustomization,
  }
}
