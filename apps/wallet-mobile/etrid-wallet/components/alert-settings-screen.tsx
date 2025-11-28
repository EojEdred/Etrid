'use client'

import { ArrowLeft, Plus, Moon, Sun } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { useAlertSettings } from '@/hooks/useAlertSettings'
import { AlertToggle } from '@/components/notifications/AlertToggle'
import { ThresholdInput } from '@/components/notifications/ThresholdInput'
import { useState } from 'react'

interface AlertSettingsScreenProps {
  onBack: () => void
}

export function AlertSettingsScreen({ onBack }: AlertSettingsScreenProps) {
  const { alerts, settings, loading, toggleAlert, updateSettings, createAlert } =
    useAlertSettings()
  const [showCreateAlert, setShowCreateAlert] = useState(false)

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading alert settings...</p>
        </div>
      </div>
    )
  }

  const alertTypes = [
    {
      type: 'price_above' as const,
      name: 'Price Alerts (Above)',
      description: 'Get notified when price goes above threshold',
    },
    {
      type: 'price_below' as const,
      name: 'Price Alerts (Below)',
      description: 'Get notified when price goes below threshold',
    },
    {
      type: 'whale_movement' as const,
      name: 'Whale Movements',
      description: 'Large transfers and unusual activity',
    },
    {
      type: 'governance_new' as const,
      name: 'New Governance Proposals',
      description: 'When new proposals are created',
    },
    {
      type: 'staking_reward' as const,
      name: 'Staking Rewards',
      description: 'When you receive staking rewards',
    },
    {
      type: 'security_login' as const,
      name: 'Security Alerts',
      description: 'New logins and suspicious activity',
    },
  ]

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
              <h1 className="text-2xl font-bold">Alert Settings</h1>
              <p className="text-sm text-muted-foreground">
                Configure your notifications
              </p>
            </div>
          </div>

          <Button
            onClick={() => setShowCreateAlert(!showCreateAlert)}
            size="sm"
            className="gap-2"
          >
            <Plus className="w-4 h-4" />
            New Alert
          </Button>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Create Alert */}
        {showCreateAlert && (
          <ThresholdInput
            onSave={async (config) => {
              await createAlert({
                alertType: 'price_above',
                conditions: config,
                channels: ['push'],
              })
              setShowCreateAlert(false)
            }}
          />
        )}

        {/* Active Alerts */}
        {alerts.length > 0 && (
          <div>
            <h3 className="text-lg font-semibold mb-3">Your Alerts</h3>
            <div className="space-y-3">
              {alerts.map((alert) => (
                <AlertToggle
                  key={alert.id}
                  name={`${alert.conditions.asset || 'Asset'} ${alert.alertType}`}
                  description={`Threshold: ${alert.conditions.threshold}`}
                  enabled={alert.enabled}
                  onToggle={() => toggleAlert(alert.id)}
                />
              ))}
            </div>
          </div>
        )}

        {/* Alert Types */}
        <div>
          <h3 className="text-lg font-semibold mb-3">Alert Types</h3>
          <div className="space-y-3">
            {alertTypes.map((alertType) => {
              const isEnabled = settings?.enabledTypes.includes(alertType.type) ?? false
              return (
                <AlertToggle
                  key={alertType.type}
                  name={alertType.name}
                  description={alertType.description}
                  enabled={isEnabled}
                  onToggle={async () => {
                    const newEnabledTypes = isEnabled
                      ? settings!.enabledTypes.filter((t) => t !== alertType.type)
                      : [...settings!.enabledTypes, alertType.type]
                    await updateSettings({ enabledTypes: newEnabledTypes })
                  }}
                />
              )
            })}
          </div>
        </div>

        {/* Quiet Hours */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
            <Moon className="w-5 h-5" />
            Quiet Hours
          </h3>
          <p className="text-sm text-muted-foreground mb-4">
            Don't send notifications during these hours
          </p>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="text-sm font-medium mb-2 block">Start</label>
              <Input
                type="time"
                value={settings?.quietHoursStart || '22:00'}
                onChange={(e) =>
                  updateSettings({ quietHoursStart: e.target.value })
                }
              />
            </div>
            <div>
              <label className="text-sm font-medium mb-2 block">End</label>
              <Input
                type="time"
                value={settings?.quietHoursEnd || '08:00'}
                onChange={(e) => updateSettings({ quietHoursEnd: e.target.value })}
              />
            </div>
          </div>
        </Card>

        {/* Frequency Limit */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">Alert Frequency</h3>
          <div>
            <label className="text-sm font-medium mb-2 block">
              Max alerts per hour
            </label>
            <Input
              type="number"
              value={settings?.maxAlertsPerHour || 10}
              onChange={(e) =>
                updateSettings({ maxAlertsPerHour: parseInt(e.target.value) })
              }
              min="1"
              max="100"
            />
          </div>
        </Card>
      </main>
    </div>
  )
}
