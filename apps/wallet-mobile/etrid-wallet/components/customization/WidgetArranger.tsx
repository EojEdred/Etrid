'use client'

import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Widget, WidgetType } from '@/lib/types/customization'
import { GripVertical, X } from 'lucide-react'

interface WidgetArrangerProps {
  widgets: Widget[]
  onUpdate: (widgets: Widget[]) => void
}

export function WidgetArranger({ widgets, onUpdate }: WidgetArrangerProps) {
  const getWidgetLabel = (type: WidgetType): string => {
    const labels: Record<WidgetType, string> = {
      balance: 'Balance',
      price_ticker: 'Price Ticker',
      portfolio_chart: 'Portfolio Chart',
      quick_actions: 'Quick Actions',
      recent_transactions: 'Recent Transactions',
      market_overview: 'Market Overview',
      staking_rewards: 'Staking Rewards',
      nft_gallery: 'NFT Gallery',
    }
    return labels[type]
  }

  const handleRemove = (widgetId: string) => {
    onUpdate(widgets.filter((w) => w.id !== widgetId))
  }

  const handleToggle = (widgetId: string) => {
    onUpdate(
      widgets.map((w) =>
        w.id === widgetId ? { ...w, enabled: !w.enabled } : w
      )
    )
  }

  const handleMoveUp = (index: number) => {
    if (index === 0) return
    const newWidgets = [...widgets]
    ;[newWidgets[index - 1], newWidgets[index]] = [
      newWidgets[index],
      newWidgets[index - 1],
    ]
    onUpdate(newWidgets)
  }

  const handleMoveDown = (index: number) => {
    if (index === widgets.length - 1) return
    const newWidgets = [...widgets]
    ;[newWidgets[index], newWidgets[index + 1]] = [
      newWidgets[index + 1],
      newWidgets[index],
    ]
    onUpdate(newWidgets)
  }

  return (
    <div className="space-y-3">
      {widgets.map((widget, index) => (
        <Card key={widget.id} className={`p-4 ${!widget.enabled ? 'opacity-50' : ''}`}>
          <div className="flex items-center gap-3">
            <div className="cursor-move">
              <GripVertical className="w-5 h-5 text-muted-foreground" />
            </div>

            <div className="flex-1">
              <h4 className="font-semibold text-sm">
                {getWidgetLabel(widget.type)}
              </h4>
              <p className="text-xs text-muted-foreground">
                Position: Row {widget.position.row}, Col {widget.position.col}
              </p>
            </div>

            <div className="flex gap-1">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => handleMoveUp(index)}
                disabled={index === 0}
                className="h-8 w-8 p-0"
              >
                ↑
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => handleMoveDown(index)}
                disabled={index === widgets.length - 1}
                className="h-8 w-8 p-0"
              >
                ↓
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => handleToggle(widget.id)}
                className="h-8 px-2 text-xs"
              >
                {widget.enabled ? 'Hide' : 'Show'}
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => handleRemove(widget.id)}
                className="h-8 w-8 p-0"
              >
                <X className="w-4 h-4 text-red-500" />
              </Button>
            </div>
          </div>
        </Card>
      ))}
    </div>
  )
}
