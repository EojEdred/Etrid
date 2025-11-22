'use client'

import { Card } from '@/components/ui/card'
import { Switch } from '@/components/ui/switch'
import { Button } from '@/components/ui/button'
import { Settings } from 'lucide-react'

interface AlertToggleProps {
  name: string
  description: string
  enabled: boolean
  onToggle: () => void
  onConfigure?: () => void
}

export function AlertToggle({
  name,
  description,
  enabled,
  onToggle,
  onConfigure,
}: AlertToggleProps) {
  return (
    <Card className="p-4">
      <div className="flex items-center justify-between">
        <div className="flex-1 mr-4">
          <h4 className="font-semibold text-sm mb-1">{name}</h4>
          <p className="text-xs text-muted-foreground">{description}</p>
        </div>

        <div className="flex items-center gap-2">
          {onConfigure && (
            <Button
              variant="ghost"
              size="sm"
              onClick={onConfigure}
              className="h-8 w-8 p-0"
            >
              <Settings className="w-4 h-4" />
            </Button>
          )}
          <Switch checked={enabled} onCheckedChange={onToggle} />
        </div>
      </div>
    </Card>
  )
}
