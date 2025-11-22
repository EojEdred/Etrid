'use client'

import { ArrowLeft, Map, Shirt, Image, Calendar } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { useMetaverse } from '@/hooks/useMetaverse'

interface MetaverseHubScreenProps {
  onBack: () => void
  onNavigate: (screen: string) => void
}

export function MetaverseHubScreen({ onBack, onNavigate }: MetaverseHubScreenProps) {
  const { virtualAssets, loading } = useMetaverse()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading metaverse...</p>
        </div>
      </div>
    )
  }

  const landCount = virtualAssets.filter((a) => a.type === 'land').length
  const wearableCount = virtualAssets.filter((a) => a.type === 'wearable').length
  const totalValue = virtualAssets.reduce((sum, asset) => sum + (asset.value || 0), 0)

  return (
    <div className="min-h-screen pb-6">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-2xl font-bold">Metaverse</h1>
            <p className="text-sm text-muted-foreground">
              Your virtual assets
            </p>
          </div>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Total Value */}
        <Card className="p-6 bg-gradient-to-br from-accent/10 to-primary/10">
          <p className="text-sm text-muted-foreground mb-2">
            Total Metaverse Value
          </p>
          <p className="text-4xl font-bold mb-4">
            ${totalValue.toLocaleString()}
          </p>
          <div className="flex gap-4 text-sm">
            <div>
              <span className="text-muted-foreground">{landCount}</span>
              <span className="ml-1">Land Plots</span>
            </div>
            <div>
              <span className="text-muted-foreground">{wearableCount}</span>
              <span className="ml-1">Wearables</span>
            </div>
          </div>
        </Card>

        {/* Featured Platforms */}
        <div>
          <h3 className="text-lg font-semibold mb-3">Featured Platforms</h3>
          <div className="grid grid-cols-2 gap-4">
            {[
              { name: 'Ëtrid Metaverse', color: 'bg-purple-500' },
              { name: 'Decentraland', color: 'bg-red-500' },
              { name: 'The Sandbox', color: 'bg-blue-500' },
              { name: 'Somnium Space', color: 'bg-green-500' },
            ].map((platform) => (
              <Card key={platform.name} className="p-4 cursor-pointer hover:border-accent">
                <div
                  className={`w-12 h-12 rounded-lg ${platform.color} mb-3 flex items-center justify-center text-white font-bold`}
                >
                  {platform.name[0]}
                </div>
                <p className="text-sm font-semibold">{platform.name}</p>
              </Card>
            ))}
          </div>
        </div>

        {/* Quick Links */}
        <div>
          <h3 className="text-lg font-semibold mb-3">Quick Access</h3>
          <div className="grid grid-cols-2 gap-4">
            <Button
              variant="outline"
              className="h-auto py-6 flex flex-col items-center"
              onClick={() => onNavigate('virtual-gallery')}
            >
              <Image className="w-8 h-8 mb-2 text-purple-500" />
              <span className="font-semibold">Virtual Gallery</span>
              <span className="text-xs text-muted-foreground">
                Display your NFTs
              </span>
            </Button>

            <Button
              variant="outline"
              className="h-auto py-6 flex flex-col items-center"
              onClick={() => onNavigate('land-nfts')}
            >
              <Map className="w-8 h-8 mb-2 text-green-500" />
              <span className="font-semibold">Land Holdings</span>
              <span className="text-xs text-muted-foreground">
                {landCount} plots owned
              </span>
            </Button>

            <Button
              variant="outline"
              className="h-auto py-6 flex flex-col items-center"
              onClick={() => onNavigate('wearables')}
            >
              <Shirt className="w-8 h-8 mb-2 text-blue-500" />
              <span className="font-semibold">Wearables</span>
              <span className="text-xs text-muted-foreground">
                {wearableCount} items
              </span>
            </Button>

            <Button
              variant="outline"
              className="h-auto py-6 flex flex-col items-center"
              onClick={() => onNavigate('metaverse-events')}
            >
              <Calendar className="w-8 h-8 mb-2 text-orange-500" />
              <span className="font-semibold">Events</span>
              <span className="text-xs text-muted-foreground">
                Upcoming activities
              </span>
            </Button>
          </div>
        </div>
      </main>
    </div>
  )
}
