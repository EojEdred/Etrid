'use client'

import { ArrowLeft, User } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { useWearables } from '@/hooks/useMetaverse'
import { WearableItem } from '@/components/metaverse/WearableItem'

interface WearablesScreenProps {
  onBack: () => void
}

export function WearablesScreen({ onBack }: WearablesScreenProps) {
  const { wearables, loading, equipWearable, unequipWearable } = useWearables()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading wearables...</p>
        </div>
      </div>
    )
  }

  const equippedCount = wearables.filter((w) => w.equipped).length
  const totalValue = wearables.reduce((sum, w) => sum + (w.value || 0), 0)

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Wearables</h1>
            <p className="text-sm text-muted-foreground">
              {wearables.length} items in collection
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Avatar Preview */}
        <Card className="p-6">
          <h3 className="text-lg font-semibold mb-4">Your Avatar</h3>
          <div className="aspect-square rounded-lg bg-gradient-to-br from-accent/20 to-primary/20 flex items-center justify-center mb-4">
            <User className="w-24 h-24 text-accent" />
          </div>
          <div className="text-center">
            <p className="text-sm text-muted-foreground">
              {equippedCount} items equipped
            </p>
            <p className="text-xs text-muted-foreground">
              Total value: ${totalValue.toLocaleString()}
            </p>
          </div>
        </Card>

        {/* Wearables Grid */}
        <div>
          <h3 className="text-lg font-semibold mb-3">Your Wearables</h3>
          <div className="grid grid-cols-2 gap-4">
            {wearables.map((wearable) => (
              <WearableItem
                key={wearable.id}
                wearable={wearable}
                onEquip={equipWearable}
                onUnequip={unequipWearable}
              />
            ))}
          </div>
        </div>
      </main>
    </div>
  )
}
