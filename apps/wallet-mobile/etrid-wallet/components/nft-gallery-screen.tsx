"use client"

import { useState } from 'react'
import { ArrowLeft, Grid3x3, List, Filter, Search } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { NFTGrid } from '@/components/nft/nft-grid'
import { useNFTGallery, type NFTGalleryFilters } from '@/hooks/use-nft-gallery'
import type { NFT } from '@/lib/types/nft'

interface NFTGalleryScreenProps {
  userAddress: string
  onBack: () => void
  onNFTClick?: (nft: NFT) => void
}

export function NFTGalleryScreen({ userAddress, onBack, onNFTClick }: NFTGalleryScreenProps) {
  const [search, setSearch] = useState('')
  const [filters, setFilters] = useState<NFTGalleryFilters>({})

  const {
    nfts,
    loading,
    viewMode,
    setViewMode,
    selectedNFTs,
    toggleNFTSelection,
    selectAll,
    clearSelection,
    refresh,
  } = useNFTGallery(userAddress, { ...filters, search })

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between mb-4">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <h1 className="text-xl font-bold">My NFT Gallery</h1>
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setViewMode(viewMode === 'grid' ? 'list' : 'grid')}
          >
            {viewMode === 'grid' ? <List className="w-5 h-5" /> : <Grid3x3 className="w-5 h-5" />}
          </Button>
        </div>

        {/* Search */}
        <div className="relative mb-3">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <Input
            placeholder="Search NFTs..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="pl-10"
          />
        </div>

        {/* Filters */}
        <div className="grid grid-cols-2 gap-2">
          <Select
            value={filters.chain || ''}
            onValueChange={(value) => setFilters({ ...filters, chain: value || undefined })}
          >
            <SelectTrigger>
              <SelectValue placeholder="All Chains" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="">All Chains</SelectItem>
              <SelectItem value="etrid">Etrid</SelectItem>
              <SelectItem value="ethereum">Ethereum</SelectItem>
              <SelectItem value="polygon">Polygon</SelectItem>
            </SelectContent>
          </Select>

          <Select
            value={filters.sort || ''}
            onValueChange={(value) =>
              setFilters({ ...filters, sort: (value as any) || undefined })
            }
          >
            <SelectTrigger>
              <SelectValue placeholder="Sort By" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="recent">Recent</SelectItem>
              <SelectItem value="name">Name</SelectItem>
              <SelectItem value="value">Value</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </header>

      {/* Content */}
      <main className="p-4">
        {/* Selection Controls */}
        {selectedNFTs.length > 0 && (
          <div className="mb-4 p-3 rounded-lg bg-accent flex items-center justify-between">
            <span className="text-sm font-medium">{selectedNFTs.length} selected</span>
            <div className="flex gap-2">
              <Button variant="outline" size="sm" onClick={clearSelection}>
                Clear
              </Button>
              <Button size="sm">Transfer</Button>
            </div>
          </div>
        )}

        {/* NFT Grid */}
        <NFTGrid
          nfts={nfts}
          loading={loading}
          onNFTClick={onNFTClick}
          emptyMessage="No NFTs in your collection"
          columns={viewMode === 'grid' ? 2 : 3}
        />
      </main>
    </div>
  )
}
