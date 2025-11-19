'use client'

import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { WearableNFT, Rarity } from '@/lib/types/metaverse'
import { Shirt, Check } from 'lucide-react'

interface WearableItemProps {
  wearable: WearableNFT
  onEquip?: (id: string) => void
  onUnequip?: (id: string) => void
}

export function WearableItem({ wearable, onEquip, onUnequip }: WearableItemProps) {
  const getRarityColor = (rarity: Rarity): string => {
    switch (rarity) {
      case 'common':
        return 'text-gray-500 border-gray-500'
      case 'uncommon':
        return 'text-green-500 border-green-500'
      case 'rare':
        return 'text-blue-500 border-blue-500'
      case 'epic':
        return 'text-purple-500 border-purple-500'
      case 'legendary':
        return 'text-orange-500 border-orange-500'
    }
  }

  return (
    <Card className={`p-4 ${wearable.equipped ? 'border-accent' : ''}`}>
      <div className="aspect-square rounded-lg overflow-hidden mb-3 bg-gradient-to-br from-accent/10 to-primary/10 flex items-center justify-center relative">
        <Shirt className="w-16 h-16 text-accent" />
        {wearable.equipped && (
          <div className="absolute top-2 right-2 w-6 h-6 rounded-full bg-accent flex items-center justify-center">
            <Check className="w-4 h-4 text-accent-foreground" />
          </div>
        )}
      </div>

      <div className="mb-3">
        <h4 className="font-semibold mb-1">{wearable.name}</h4>
        <p className="text-xs text-muted-foreground mb-2">
          {wearable.description}
        </p>
        <div className="flex items-center gap-2">
          <span
            className={`text-xs font-semibold px-2 py-1 border rounded capitalize ${getRarityColor(
              wearable.rarity
            )}`}
          >
            {wearable.rarity}
          </span>
          <span className="text-xs text-muted-foreground">
            ${wearable.value}
          </span>
        </div>
      </div>

      <Button
        variant={wearable.equipped ? 'outline' : 'default'}
        size="sm"
        className="w-full"
        onClick={() =>
          wearable.equipped
            ? onUnequip?.(wearable.id)
            : onEquip?.(wearable.id)
        }
      >
        {wearable.equipped ? 'Unequip' : 'Equip'}
      </Button>
    </Card>
  )
}
