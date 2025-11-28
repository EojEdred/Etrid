'use client'

import { ArrowLeft, Plus, Moon, Sun } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Card } from '@/components/ui/card'
import { useCustomization } from '@/hooks/useCustomization'
import { ThemePreview } from '@/components/customization/ThemePreview'
import { useState } from 'react'

interface ThemeSettingsScreenProps {
  onBack: () => void
  onNavigate?: (screen: string) => void
}

export function ThemeSettingsScreen({ onBack, onNavigate }: ThemeSettingsScreenProps) {
  const { themes, currentThemeId, setTheme, loading } = useCustomization()
  const [autoDarkMode, setAutoDarkMode] = useState(false)

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading themes...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center justify-between p-6">
          <div className="flex items-center gap-4">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <div>
              <h1 className="text-2xl font-bold">Theme Settings</h1>
              <p className="text-sm text-muted-foreground">
                Personalize your wallet
              </p>
            </div>
          </div>

          {onNavigate && (
            <Button
              onClick={() => onNavigate('custom-theme')}
              size="sm"
              className="gap-2"
            >
              <Plus className="w-4 h-4" />
              Create
            </Button>
          )}
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Auto Dark Mode */}
        <Card className="p-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-accent/10 flex items-center justify-center">
                <Moon className="w-5 h-5 text-accent" />
              </div>
              <div>
                <h3 className="font-semibold">Auto Dark Mode</h3>
                <p className="text-xs text-muted-foreground">
                  Automatically switch at sunset
                </p>
              </div>
            </div>
            <Switch
              checked={autoDarkMode}
              onCheckedChange={setAutoDarkMode}
            />
          </div>

          {autoDarkMode && (
            <div className="text-xs text-muted-foreground bg-muted/50 p-3 rounded-lg">
              Theme will automatically switch to dark mode from sunset to sunrise
              based on your location.
            </div>
          )}
        </Card>

        {/* Built-in Themes */}
        <div>
          <h3 className="text-lg font-semibold mb-3">Built-in Themes</h3>
          <div className="grid grid-cols-2 gap-4">
            {themes
              .filter((theme) => theme.builtIn)
              .map((theme) => (
                <ThemePreview
                  key={theme.id}
                  theme={theme}
                  isActive={currentThemeId === theme.id}
                  onSelect={() => setTheme(theme.id)}
                />
              ))}
          </div>
        </div>

        {/* Custom Themes */}
        {themes.some((theme) => !theme.builtIn) && (
          <div>
            <h3 className="text-lg font-semibold mb-3">Custom Themes</h3>
            <div className="grid grid-cols-2 gap-4">
              {themes
                .filter((theme) => !theme.builtIn)
                .map((theme) => (
                  <ThemePreview
                    key={theme.id}
                    theme={theme}
                    isActive={currentThemeId === theme.id}
                    onSelect={() => setTheme(theme.id)}
                  />
                ))}
            </div>
          </div>
        )}
      </main>
    </div>
  )
}
