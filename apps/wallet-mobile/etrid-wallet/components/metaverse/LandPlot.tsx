'use client'

import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { LandNFT } from '@/lib/types/metaverse'
import { MapPin, DollarSign, Maximize } from 'lucide-react'

interface LandPlotProps {
  land: LandNFT
  onViewMap?: () => void
}

export function LandPlot({ land, onViewMap }: LandPlotProps) {
  return (
    <Card className="p-4">
      <div className="aspect-square rounded-lg overflow-hidden mb-3 bg-gradient-to-br from-accent/20 to-primary/20 flex items-center justify-center">
        <div className="text-center p-4">
          <MapPin className="w-12 h-12 mx-auto mb-2 text-accent" />
          <p className="text-xs text-muted-foreground">
            {land.size.width}×{land.size.height}
          </p>
        </div>
      </div>

      <h4 className="font-semibold mb-1">{land.name}</h4>
      <p className="text-xs text-muted-foreground mb-3">{land.platform}</p>

      <div className="grid grid-cols-2 gap-3 mb-3">
        <div>
          <p className="text-xs text-muted-foreground">Coordinates</p>
          <p className="text-sm font-semibold">
            {land.coordinates.x}, {land.coordinates.y}
          </p>
        </div>
        <div>
          <p className="text-xs text-muted-foreground">Value</p>
          <p className="text-sm font-semibold text-green-500">
            ${land.value?.toLocaleString()}
          </p>
        </div>
      </div>

      {land.rented && land.rentalIncome && (
        <div className="mb-3 p-2 bg-green-500/10 rounded text-xs">
          <p className="text-green-500 font-semibold">
            Earning ${land.rentalIncome}/month
          </p>
        </div>
      )}

      <Button variant="outline" size="sm" className="w-full" onClick={onViewMap}>
        <Maximize className="w-4 h-4 mr-2" />
        View on Map
      </Button>
    </Card>
  )
}
