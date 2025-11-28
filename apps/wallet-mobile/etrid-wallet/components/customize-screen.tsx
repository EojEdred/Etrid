'use client'

import { ArrowLeft, RotateCcw, Plus } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { useCustomization } from '@/hooks/useCustomization'
import { WidgetArranger } from '@/components/customization/WidgetArranger'
import { customizationService } from '@/lib/services/CustomizationService'
import { WidgetType } from '@/lib/types/customization'
import { useState } from 'react'

interface CustomizeScreenProps {
  onBack: () => void
}

export function CustomizeScreen({ onBack }: CustomizeScreenProps) {
  const { widgetLayout, updateWidgetLayout, resetLayout, loading } =
    useCustomization()
  const [showAddWidget, setShowAddWidget] = useState(false)

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading customization...</p>
        </div>
      </div>
    )
  }

  if (!widgetLayout) {
    return null
  }

  const availableWidgets = customizationService.getAvailableWidgets()

  const handleAddWidget = (type: WidgetType) => {
    const newWidget = {
      id: `${type}-${Date.now()}`,
      type,
      position: {
        row: widgetLayout.widgets.length,
        col: 0,
        width: 2,
        height: 1,
      },
      enabled: true,
    }

    updateWidgetLayout({
      ...widgetLayout,
      widgets: [...widgetLayout.widgets, newWidget],
    })
    setShowAddWidget(false)
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
              <h1 className="text-2xl font-bold">Customize Layout</h1>
              <p className="text-sm text-muted-foreground">
                Arrange your widgets
              </p>
            </div>
          </div>

          <Button variant="outline" size="sm" onClick={resetLayout}>
            <RotateCcw className="w-4 h-4 mr-2" />
            Reset
          </Button>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Current Widgets */}
        <div>
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-lg font-semibold">Your Widgets</h3>
            <Button
              size="sm"
              onClick={() => setShowAddWidget(!showAddWidget)}
              className="gap-2"
            >
              <Plus className="w-4 h-4" />
              Add Widget
            </Button>
          </div>

          <WidgetArranger
            widgets={widgetLayout.widgets}
            onUpdate={(widgets) =>
              updateWidgetLayout({ ...widgetLayout, widgets })
            }
          />
        </div>

        {/* Add Widget Panel */}
        {showAddWidget && (
          <Card className="p-6">
            <h3 className="text-lg font-semibold mb-4">Available Widgets</h3>
            <div className="grid gap-3">
              {availableWidgets.map((widget) => (
                <div
                  key={widget.type}
                  className="flex items-center justify-between p-3 border rounded-lg hover:bg-muted/50 cursor-pointer"
                  onClick={() => handleAddWidget(widget.type)}
                >
                  <div>
                    <h4 className="font-semibold text-sm">{widget.name}</h4>
                    <p className="text-xs text-muted-foreground">
                      {widget.description}
                    </p>
                  </div>
                  <Button size="sm">Add</Button>
                </div>
              ))}
            </div>
          </Card>
        )}

        {/* Tips */}
        <Card className="p-4 bg-accent/5 border-accent/20">
          <p className="text-sm text-muted-foreground">
            <strong>Tip:</strong> Drag widgets to reorder them, or use the arrow
            buttons. Toggle visibility or remove widgets you don't use.
          </p>
        </Card>
      </main>
    </div>
  )
}
