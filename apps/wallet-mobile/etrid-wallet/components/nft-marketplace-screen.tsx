"use client"

import { useState } from 'react'
import { ArrowLeft, Filter, TrendingUp } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Card } from '@/components/ui/card'
import { Carousel, CarouselContent, CarouselItem } from '@/components/ui/carousel'
import { NFTGrid } from '@/components/nft/nft-grid'
import { NFTCard } from '@/components/nft/nft-card'
import { useNFTMarketplace } from '@/hooks/use-nft-marketplace'
import type { NFT, MarketplaceFilters } from '@/lib/types/nft'

interface NFTMarketplaceScreenProps {
  onBack: () => void
  onNFTClick?: (nft: NFT) => void
}

export function NFTMarketplaceScreen({ onBack, onNFTClick }: NFTMarketplaceScreenProps) {
  const [category, setCategory] = useState<string>('all')
  const {
    listings,
    featuredNFTs,
    trendingCollections,
    loading,
    filters,
    updateFilters,
  } = useNFTMarketplace()

  const categories = ['all', 'art', 'music', 'gaming', 'collectibles', 'photography']

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <h1 className="text-xl font-bold">NFT Marketplace</h1>
          <Button variant="ghost" size="icon">
            <Filter className="w-5 h-5" />
          </Button>
        </div>
      </header>

      {/* Content */}
      <main className="space-y-6">
        {/* Featured NFTs Carousel */}
        {featuredNFTs.length > 0 && (
          <section className="px-4 pt-4">
            <h2 className="text-lg font-semibold mb-3">Featured</h2>
            <Carousel>
              <CarouselContent>
                {featuredNFTs.map((nft) => (
                  <CarouselItem key={nft.id} className="basis-4/5">
                    <NFTCard
                      nft={nft}
                      onClick={() => onNFTClick?.(nft)}
                      className="h-full"
                    />
                  </CarouselItem>
                ))}
              </CarouselContent>
            </Carousel>
          </section>
        )}

        {/* Trending Collections */}
        {trendingCollections.length > 0 && (
          <section className="px-4">
            <h2 className="text-lg font-semibold mb-3 flex items-center gap-2">
              <TrendingUp className="w-5 h-5" />
              Trending Collections
            </h2>
            <div className="space-y-2">
              {trendingCollections.slice(0, 5).map((collection, index) => (
                <Card key={collection.id} className="p-3">
                  <div className="flex items-center gap-3">
                    <span className="text-muted-foreground font-semibold">#{index + 1}</span>
                    <div className="flex-1 min-w-0">
                      <h3 className="font-semibold truncate">{collection.name}</h3>
                      <p className="text-sm text-muted-foreground">
                        Floor: {collection.floor_price} ETRID
                      </p>
                    </div>
                    <div className="text-right">
                      <p className="text-sm font-semibold text-green-500">
                        +{collection.change_percentage}%
                      </p>
                      <p className="text-xs text-muted-foreground">{collection.volume_24h} Vol</p>
                    </div>
                  </div>
                </Card>
              ))}
            </div>
          </section>
        )}

        {/* Categories */}
        <section className="px-4">
          <Tabs value={category} onValueChange={setCategory}>
            <TabsList className="w-full overflow-x-auto">
              {categories.map((cat) => (
                <TabsTrigger key={cat} value={cat} className="capitalize">
                  {cat}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
        </section>

        {/* Filters */}
        <section className="px-4">
          <div className="grid grid-cols-3 gap-2">
            <Select
              value={filters.chain || ''}
              onValueChange={(value) => updateFilters({ chain: value || undefined })}
            >
              <SelectTrigger>
                <SelectValue placeholder="Chain" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All</SelectItem>
                <SelectItem value="etrid">Etrid</SelectItem>
                <SelectItem value="ethereum">Ethereum</SelectItem>
              </SelectContent>
            </Select>

            <Input
              type="number"
              placeholder="Min Price"
              onChange={(e) =>
                updateFilters({ min_price: parseFloat(e.target.value) || undefined })
              }
            />

            <Select
              value={filters.sort_by || ''}
              onValueChange={(value) => updateFilters({ sort_by: value as any })}
            >
              <SelectTrigger>
                <SelectValue placeholder="Sort" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="price_asc">Price: Low to High</SelectItem>
                <SelectItem value="price_desc">Price: High to Low</SelectItem>
                <SelectItem value="recent">Recently Listed</SelectItem>
                <SelectItem value="ending_soon">Ending Soon</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </section>

        {/* Listings Grid */}
        <section className="px-4">
          <NFTGrid
            nfts={listings.map((l) => l.nft)}
            loading={loading}
            onNFTClick={onNFTClick}
            emptyMessage="No listings found"
          />
        </section>
      </main>
    </div>
  )
}
