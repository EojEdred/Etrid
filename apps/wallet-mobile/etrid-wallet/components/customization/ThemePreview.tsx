'use client'

import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Theme } from '@/lib/types/customization'
import { Check } from 'lucide-react'

interface ThemePreviewProps {
  theme: Theme
  isActive: boolean
  onSelect: () => void
}

export function ThemePreview({ theme, isActive, onSelect }: ThemePreviewProps) {
  return (
    <Card className="p-4 cursor-pointer hover:border-accent transition-colors" onClick={onSelect}>
      <div className="space-y-3">
        {/* Preview */}
        <div
          className="w-full h-32 rounded-lg p-3 flex flex-col justify-between"
          style={{ backgroundColor: theme.colors.background }}
        >
          <div className="space-y-1">
            <div
              className="h-3 w-20 rounded"
              style={{ backgroundColor: theme.colors.primary }}
            />
            <div
              className="h-2 w-16 rounded"
              style={{ backgroundColor: theme.colors.text, opacity: 0.5 }}
            />
          </div>
          <div className="flex gap-1">
            <div
              className="h-8 flex-1 rounded"
              style={{ backgroundColor: theme.colors.card }}
            />
            <div
              className="h-8 flex-1 rounded"
              style={{ backgroundColor: theme.colors.accent, opacity: 0.3 }}
            />
          </div>
        </div>

        {/* Theme Info */}
        <div className="flex items-center justify-between">
          <div>
            <h4 className="font-semibold">{theme.name}</h4>
            {/* Color Palette */}
            <div className="flex gap-1 mt-2">
              <div
                className="w-4 h-4 rounded-full"
                style={{ backgroundColor: theme.colors.primary }}
              />
              <div
                className="w-4 h-4 rounded-full"
                style={{ backgroundColor: theme.colors.accent }}
              />
              <div
                className="w-4 h-4 rounded-full"
                style={{ backgroundColor: theme.colors.card }}
              />
            </div>
          </div>
          {isActive && (
            <div className="w-8 h-8 rounded-full bg-accent flex items-center justify-center">
              <Check className="w-4 h-4 text-accent-foreground" />
            </div>
          )}
        </div>
      </div>
    </Card>
  )
}
