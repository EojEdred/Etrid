import { Theme, CustomTheme, BUILT_IN_THEMES } from '../types/customization'

export class ThemeService {
  private static instance: ThemeService

  static getInstance(): ThemeService {
    if (!ThemeService.instance) {
      ThemeService.instance = new ThemeService()
    }
    return ThemeService.instance
  }

  async getThemes(): Promise<Theme[]> {
    try {
      const response = await fetch('/api/themes')
      if (!response.ok) throw new Error('Failed to fetch themes')
      const customThemes = await response.json()
      return [...BUILT_IN_THEMES, ...customThemes]
    } catch (error) {
      console.error('Error fetching themes:', error)
      return BUILT_IN_THEMES
    }
  }

  async setTheme(themeId: string): Promise<void> {
    try {
      const response = await fetch('/api/themes/active', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ themeId }),
      })
      if (!response.ok) throw new Error('Failed to set theme')

      // Apply theme to document
      const themes = await this.getThemes()
      const theme = themes.find((t) => t.id === themeId)
      if (theme) {
        this.applyTheme(theme)
      }
    } catch (error) {
      console.error('Error setting theme:', error)
      throw error
    }
  }

  async createCustomTheme(customTheme: CustomTheme): Promise<Theme> {
    try {
      const response = await fetch('/api/themes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(customTheme),
      })
      if (!response.ok) throw new Error('Failed to create theme')
      return await response.json()
    } catch (error) {
      console.error('Error creating theme:', error)
      throw error
    }
  }

  async deleteTheme(themeId: string): Promise<void> {
    try {
      const response = await fetch(`/api/themes/${themeId}`, {
        method: 'DELETE',
      })
      if (!response.ok) throw new Error('Failed to delete theme')
    } catch (error) {
      console.error('Error deleting theme:', error)
      throw error
    }
  }

  async exportTheme(themeId: string): Promise<string> {
    try {
      const themes = await this.getThemes()
      const theme = themes.find((t) => t.id === themeId)
      if (!theme) throw new Error('Theme not found')
      return JSON.stringify(theme, null, 2)
    } catch (error) {
      console.error('Error exporting theme:', error)
      throw error
    }
  }

  async importTheme(themeJSON: string): Promise<Theme> {
    try {
      const themeData = JSON.parse(themeJSON)
      return await this.createCustomTheme({
        name: themeData.name,
        colors: themeData.colors,
      })
    } catch (error) {
      console.error('Error importing theme:', error)
      throw error
    }
  }

  applyTheme(theme: Theme): void {
    const root = document.documentElement

    Object.entries(theme.colors).forEach(([key, value]) => {
      // Convert camelCase to kebab-case for CSS variables
      const cssVar = key.replace(/([A-Z])/g, '-$1').toLowerCase()
      root.style.setProperty(`--${cssVar}`, value)
    })
  }

  getThemeById(themeId: string): Theme | null {
    return BUILT_IN_THEMES.find((t) => t.id === themeId) || null
  }
}

export const themeService = ThemeService.getInstance()
