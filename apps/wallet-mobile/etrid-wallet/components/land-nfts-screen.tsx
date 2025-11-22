'use client'

import { ArrowLeft, Map } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { useLandNFTs } from '@/hooks/useMetaverse'
import { LandPlot } from '@/components/metaverse/LandPlot'

interface LandNFTsScreenProps {
  onBack: () => void
}

export function LandNFTsScreen({ onBack }: LandNFTsScreenProps) {
  const { landNFTs, loading } = useLandNFTs()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading land NFTs...</p>
        </div>
      </div>
    )
  }

  const totalValue = landNFTs.reduce((sum, land) => sum + (land.value || 0), 0)
  const rentedCount = landNFTs.filter((land) => land.rented).length
  const totalIncome = landNFTs
    .filter((land) => land.rented && land.rentalIncome)
    .reduce((sum, land) => sum + (land.rentalIncome || 0), 0)

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Land Holdings</h1>
            <p className="text-sm text-muted-foreground">
              {landNFTs.length} virtual land plots
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Summary Stats */}
        <div className="grid grid-cols-3 gap-4">
          <div className="text-center">
            <p className="text-2xl font-bold">{landNFTs.length}</p>
            <p className="text-xs text-muted-foreground">Total Plots</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold text-green-500">
              ${totalValue.toLocaleString()}
            </p>
            <p className="text-xs text-muted-foreground">Total Value</p>
          </div>
          <div className="text-center">
            <p className="text-2xl font-bold">${totalIncome}</p>
            <p className="text-xs text-muted-foreground">Monthly Income</p>
          </div>
        </div>

        {/* Land NFTs Grid */}
        <div>
          <h3 className="text-lg font-semibold mb-3">Your Land</h3>
          <div className="grid grid-cols-2 gap-4">
            {landNFTs.map((land) => (
              <LandPlot key={land.id} land={land} />
            ))}
          </div>
        </div>

        {/* Map View Button */}
        <Button className="w-full gap-2">
          <Map className="w-4 h-4" />
          View All on Map
        </Button>
      </main>
    </div>
  )
}
