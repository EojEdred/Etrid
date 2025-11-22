'use client'

import { ArrowLeft, Plus, Image } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { useVirtualGallery } from '@/hooks/useMetaverse'

interface VirtualGalleryScreenProps {
  onBack: () => void
}

export function VirtualGalleryScreen({ onBack }: VirtualGalleryScreenProps) {
  const { galleries, loading } = useVirtualGallery()

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent mx-auto mb-4" />
          <p className="text-muted-foreground">Loading galleries...</p>
        </div>
      </div>
    )
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
              <h1 className="text-2xl font-bold">Virtual Gallery</h1>
              <p className="text-sm text-muted-foreground">
                Display your NFT collection
              </p>
            </div>
          </div>

          <Button size="sm" className="gap-2">
            <Plus className="w-4 h-4" />
            Create
          </Button>
        </div>
      </header>

      <main className="px-6 space-y-6 mt-6">
        {/* Gallery Info */}
        <Card className="p-6 bg-gradient-to-br from-purple-500/10 to-blue-500/10">
          <h3 className="text-lg font-semibold mb-2">3D NFT Gallery</h3>
          <p className="text-sm text-muted-foreground mb-4">
            Create immersive 3D galleries to showcase your NFT collection. Place
            artworks on walls, customize lighting, and share with the world.
          </p>
          <div className="flex gap-2">
            <div className="flex-1 text-center p-3 bg-background/50 rounded">
              <p className="text-2xl font-bold">{galleries.length}</p>
              <p className="text-xs text-muted-foreground">Galleries</p>
            </div>
            <div className="flex-1 text-center p-3 bg-background/50 rounded">
              <p className="text-2xl font-bold">
                {galleries.reduce((sum, g) => sum + g.nftIds.length, 0)}
              </p>
              <p className="text-xs text-muted-foreground">NFTs Displayed</p>
            </div>
          </div>
        </Card>

        {/* Galleries List */}
        {galleries.length > 0 ? (
          <div>
            <h3 className="text-lg font-semibold mb-3">Your Galleries</h3>
            <div className="grid gap-4">
              {galleries.map((gallery) => (
                <Card key={gallery.id} className="p-4 cursor-pointer hover:border-accent">
                  <div className="flex items-start gap-4">
                    <div className="w-20 h-20 rounded-lg bg-gradient-to-br from-accent/20 to-primary/20 flex items-center justify-center flex-shrink-0">
                      <Image className="w-8 h-8 text-accent" />
                    </div>
                    <div className="flex-1 min-w-0">
                      <h4 className="font-semibold mb-1">{gallery.name}</h4>
                      <p className="text-xs text-muted-foreground mb-2">
                        {gallery.nftIds.length} NFTs displayed
                      </p>
                      {gallery.views !== undefined && (
                        <p className="text-xs text-muted-foreground">
                          {gallery.views} views
                        </p>
                      )}
                    </div>
                    <Button variant="outline" size="sm">
                      View
                    </Button>
                  </div>
                </Card>
              ))}
            </div>
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="w-20 h-20 rounded-full bg-muted flex items-center justify-center mx-auto mb-4">
              <Image className="w-10 h-10 text-muted-foreground" />
            </div>
            <p className="text-muted-foreground mb-4">
              No galleries created yet
            </p>
            <Button className="gap-2">
              <Plus className="w-4 h-4" />
              Create Your First Gallery
            </Button>
          </div>
        )}
      </main>
    </div>
  )
}
