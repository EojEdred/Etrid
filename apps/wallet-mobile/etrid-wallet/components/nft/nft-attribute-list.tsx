"use client"

import { Card } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import type { NFTAttribute } from '@/lib/types/nft'

interface NFTAttributeListProps {
  attributes: NFTAttribute[]
  showRarity?: boolean
  showFloorPrice?: boolean
  className?: string
}

export function NFTAttributeList({
  attributes,
  showRarity = true,
  showFloorPrice = false,
  className,
}: NFTAttributeListProps) {
  if (attributes.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground">
        No attributes available
      </div>
    )
  }

  return (
    <div className={className}>
      <div className="grid grid-cols-2 gap-3">
        {attributes.map((attr, index) => (
          <Card key={index} className="p-3 space-y-2 border-border/50">
            {/* Trait Type */}
            <div className="text-xs text-muted-foreground uppercase tracking-wide">
              {attr.trait_type}
            </div>

            {/* Trait Value */}
            <div className="font-semibold text-sm truncate">
              {attr.value.toString()}
            </div>

            {/* Rarity */}
            {showRarity && attr.rarity !== undefined && (
              <div className="flex items-center gap-2">
                <Badge
                  variant="secondary"
                  className="text-xs"
                >
                  {attr.rarity.toFixed(1)}% rarity
                </Badge>
              </div>
            )}

            {/* Floor Price */}
            {showFloorPrice && attr.floor_price !== undefined && (
              <div className="text-xs text-muted-foreground">
                Floor: {attr.floor_price} ETRID
              </div>
            )}
          </Card>
        ))}
      </div>
    </div>
  )
}
