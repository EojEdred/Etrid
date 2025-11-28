// useNFTGallery Hook - Manage user's NFT collection

import { useState, useEffect, useCallback } from 'react'
import { nftService } from '@/lib/services/nft.service'
import type { NFT } from '@/lib/types/nft'

export interface NFTGalleryFilters {
  chain?: string
  collection?: string
  search?: string
  sort?: 'recent' | 'value' | 'name'
}

export function useNFTGallery(address: string, filters?: NFTGalleryFilters) {
  const [nfts, setNfts] = useState<NFT[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const [selectedNFTs, setSelectedNFTs] = useState<string[]>([])
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')

  const fetchNFTs = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      let fetchedNFTs = await nftService.getUserNFTs(address, filters?.chain)

      // Apply filters
      if (filters?.collection) {
        fetchedNFTs = fetchedNFTs.filter(
          (nft) => nft.collection?.id === filters.collection
        )
      }

      if (filters?.search) {
        const searchLower = filters.search.toLowerCase()
        fetchedNFTs = fetchedNFTs.filter(
          (nft) =>
            nft.name.toLowerCase().includes(searchLower) ||
            nft.description?.toLowerCase().includes(searchLower) ||
            nft.collection?.name.toLowerCase().includes(searchLower)
        )
      }

      // Apply sorting
      if (filters?.sort) {
        switch (filters.sort) {
          case 'recent':
            fetchedNFTs.sort(
              (a, b) =>
                new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
            )
            break
          case 'name':
            fetchedNFTs.sort((a, b) => a.name.localeCompare(b.name))
            break
          case 'value':
            // Would need to implement value calculation
            break
        }
      }

      setNfts(fetchedNFTs)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [address, filters])

  useEffect(() => {
    if (address) {
      fetchNFTs()
    }
  }, [address, fetchNFTs])

  const toggleNFTSelection = useCallback((nftId: string) => {
    setSelectedNFTs((prev) =>
      prev.includes(nftId) ? prev.filter((id) => id !== nftId) : [...prev, nftId]
    )
  }, [])

  const selectAll = useCallback(() => {
    setSelectedNFTs(nfts.map((nft) => nft.id))
  }, [nfts])

  const clearSelection = useCallback(() => {
    setSelectedNFTs([])
  }, [])

  const bulkTransfer = useCallback(
    async (toAddress: string) => {
      try {
        const transfers = selectedNFTs
          .map((id) => nfts.find((nft) => nft.id === id))
          .filter((nft): nft is NFT => nft !== undefined)
          .map((nft) => ({
            to: toAddress,
            contract_address: nft.contract_address,
            token_id: nft.token_id,
          }))

        await nftService.bulkTransferNFTs(transfers)
        clearSelection()
        await fetchNFTs() // Refresh the gallery
      } catch (err) {
        throw err
      }
    },
    [selectedNFTs, nfts, clearSelection, fetchNFTs]
  )

  return {
    nfts,
    loading,
    error,
    selectedNFTs,
    viewMode,
    setViewMode,
    toggleNFTSelection,
    selectAll,
    clearSelection,
    bulkTransfer,
    refresh: fetchNFTs,
  }
}
