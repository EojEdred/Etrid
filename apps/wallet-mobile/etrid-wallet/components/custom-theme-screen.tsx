'use client'

import { ArrowLeft, Save, Share } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { ColorPicker } from '@/components/customization/ColorPicker'
import { useCustomization } from '@/hooks/useCustomization'
import { useState } from 'react'
import { ThemeColors } from '@/lib/types/customization'

interface CustomThemeScreenProps {
  onBack: () => void
}

export function CustomThemeScreen({ onBack }: CustomThemeScreenProps) {
  const { createTheme } = useCustomization()
  const [themeName, setThemeName] = useState('My Custom Theme')
  const [colors, setColors] = useState<ThemeColors>({
    primary: '#3b82f6',
    background: '#ffffff',
    card: '#f9fafb',
    text: '#111827',
    textSecondary: '#6b7280',
    accent: '#8b5cf6',
    border: '#e5e7eb',
    success: '#10b981',
    warning: '#f59e0b',
    error: '#ef4444',
  })

  const updateColor = (key: keyof ThemeColors, value: string) => {
    setColors((prev) => ({ ...prev, [key]: value }))
  }

  const handleSave = async () => {
    try {
      await createTheme({ name: themeName, colors })
      onBack()
    } catch (error) {
      console.error('Failed to create theme:', error)
    }
  }

  const handleExport = () => {
    const themeJSON = JSON.stringify({ name: themeName, colors }, null, 2)
    const blob = new Blob([themeJSON], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${themeName.toLowerCase().replace(/\s+/g, '-')}.json`
    a.click()
    URL.revokeObjectURL(url)
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
              <h1 className="text-2xl font-bold">Custom Theme</h1>
              <p className="text-sm text-muted-foreground">
                Create your own theme
              </p>
            </div>
          </div>

          <div className="flex gap-2">
            <Button variant="outline" size="sm" onClick={handleExport}>
              <Share className="w-4 h-4 mr-2" />
              Export
            </Button>
            <Button size="sm" onClick={handleSave}>
              <Save className="w-4 h-4 mr-2" />
              Save
            </Button>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Theme Name */}
        <Card className="p-6">
          <label className="text-sm font-medium mb-2 block">Theme Name</label>
          <Input
            value={themeName}
            onChange={(e) => setThemeName(e.target.value)}
            placeholder="My Custom Theme"
          />
        </Card>

        {/* Preview */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">Preview</h3>
          <div
            className="w-full h-48 rounded-lg p-4 flex flex-col justify-between"
            style={{ backgroundColor: colors.background }}
          >
            <div className="space-y-2">
              <div
                className="h-6 w-32 rounded flex items-center justify-center text-sm font-semibold"
                style={{
                  backgroundColor: colors.primary,
                  color: colors.background,
                }}
              >
                Primary Button
              </div>
              <div
                className="h-4 w-48 rounded"
                style={{ backgroundColor: colors.text }}
              />
              <div
                className="h-4 w-40 rounded"
                style={{ backgroundColor: colors.textSecondary }}
              />
            </div>
            <div className="grid grid-cols-2 gap-2">
              <div
                className="h-16 rounded"
                style={{ backgroundColor: colors.card }}
              />
              <div
                className="h-16 rounded"
                style={{ backgroundColor: colors.accent, opacity: 0.3 }}
              />
            </div>
          </div>
        </Card>

        {/* Color Pickers */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">Colors</h3>
          <div className="space-y-4">
            <ColorPicker
              label="Primary"
              value={colors.primary}
              onChange={(val) => updateColor('primary', val)}
            />
            <ColorPicker
              label="Background"
              value={colors.background}
              onChange={(val) => updateColor('background', val)}
            />
            <ColorPicker
              label="Card"
              value={colors.card}
              onChange={(val) => updateColor('card', val)}
            />
            <ColorPicker
              label="Text"
              value={colors.text}
              onChange={(val) => updateColor('text', val)}
            />
            <ColorPicker
              label="Text Secondary"
              value={colors.textSecondary}
              onChange={(val) => updateColor('textSecondary', val)}
            />
            <ColorPicker
              label="Accent"
              value={colors.accent}
              onChange={(val) => updateColor('accent', val)}
            />
            <ColorPicker
              label="Border"
              value={colors.border}
              onChange={(val) => updateColor('border', val)}
            />
            <ColorPicker
              label="Success"
              value={colors.success}
              onChange={(val) => updateColor('success', val)}
            />
            <ColorPicker
              label="Warning"
              value={colors.warning}
              onChange={(val) => updateColor('warning', val)}
            />
            <ColorPicker
              label="Error"
              value={colors.error}
              onChange={(val) => updateColor('error', val)}
            />
          </div>
        </Card>
      </main>
    </div>
  )
}
