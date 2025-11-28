// useNFTMarketplace Hook - Browse and interact with marketplace

import { useState, useEffect, useCallback } from 'react'
import { nftMarketplaceService } from '@/lib/services/nft-marketplace.service'
import type { Listing, MarketplaceFilters, NFT } from '@/lib/types/nft'

export function useNFTMarketplace(initialFilters?: MarketplaceFilters) {
  const [listings, setListings] = useState<Listing[]>([])
  const [featuredNFTs, setFeaturedNFTs] = useState<NFT[]>([])
  const [trendingCollections, setTrendingCollections] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const [filters, setFilters] = useState<MarketplaceFilters>(initialFilters || {})

  const fetchListings = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const data = await nftMarketplaceService.getListings(filters)
      setListings(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [filters])

  const fetchFeatured = useCallback(async () => {
    try {
      const data = await nftMarketplaceService.getFeatured()
      setFeaturedNFTs(data)
    } catch (err) {
      console.error('Error fetching featured NFTs:', err)
    }
  }, [])

  const fetchTrending = useCallback(async () => {
    try {
      const data = await nftMarketplaceService.getTrendingCollections()
      setTrendingCollections(data)
    } catch (err) {
      console.error('Error fetching trending collections:', err)
    }
  }, [])

  useEffect(() => {
    fetchListings()
  }, [fetchListings])

  useEffect(() => {
    fetchFeatured()
    fetchTrending()
  }, [])

  const updateFilters = useCallback((newFilters: Partial<MarketplaceFilters>) => {
    setFilters((prev) => ({ ...prev, ...newFilters }))
  }, [])

  const clearFilters = useCallback(() => {
    setFilters({})
  }, [])

  const buyNFT = useCallback(
    async (listingId: string) => {
      try {
        const transaction = await nftMarketplaceService.buyNFT(listingId)
        await fetchListings() // Refresh listings
        return transaction
      } catch (err) {
        throw err
      }
    },
    [fetchListings]
  )

  const makeOffer = useCallback(
    async (listingId: string, amount: number) => {
      try {
        const offer = await nftMarketplaceService.makeOffer(listingId, amount)
        return offer
      } catch (err) {
        throw err
      }
    },
    []
  )

  return {
    listings,
    featuredNFTs,
    trendingCollections,
    loading,
    error,
    filters,
    updateFilters,
    clearFilters,
    buyNFT,
    makeOffer,
    refresh: fetchListings,
  }
}
