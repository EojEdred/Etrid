// useNFTDetail Hook - Get detailed NFT information

import { useState, useEffect, useCallback } from 'react'
import { nftService, type NFTDetail } from '@/lib/services/nft.service'
import { nftMarketplaceService } from '@/lib/services/nft-marketplace.service'
import type { Offer } from '@/lib/types/nft'

export function useNFTDetail(contractAddress: string, tokenId: string) {
  const [nft, setNft] = useState<NFTDetail | null>(null)
  const [offers, setOffers] = useState<Offer[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const [listingId, setListingId] = useState<string | null>(null)

  const fetchNFTDetail = useCallback(async () => {
    try {
      setLoading(true)
      setError(null)

      const data = await nftService.getNFTDetails(contractAddress, tokenId)
      setNft(data)

      // Check if NFT is listed
      const listings = await nftMarketplaceService.getListings({
        status: 'active',
      })
      const listing = listings.find(
        (l) =>
          l.nft.contract_address === contractAddress &&
          l.nft.token_id === tokenId
      )

      if (listing) {
        setListingId(listing.id)
        // Fetch offers if listed
        const offersData = await nftMarketplaceService.getOffers(listing.id)
        setOffers(offersData)
      }
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [contractAddress, tokenId])

  useEffect(() => {
    if (contractAddress && tokenId) {
      fetchNFTDetail()
    }
  }, [contractAddress, tokenId, fetchNFTDetail])

  const transferNFT = useCallback(
    async (toAddress: string) => {
      try {
        const transaction = await nftService.transferNFT(
          toAddress,
          contractAddress,
          tokenId
        )
        return transaction
      } catch (err) {
        throw err
      }
    },
    [contractAddress, tokenId]
  )

  const listNFT = useCallback(
    async (price: number, currency: string = 'ETRID', duration?: number) => {
      try {
        const listing = await nftMarketplaceService.listNFT({
          contract_address: contractAddress,
          token_id: tokenId,
          price,
          currency,
          duration,
        })
        setListingId(listing.id)
        await fetchNFTDetail()
        return listing
      } catch (err) {
        throw err
      }
    },
    [contractAddress, tokenId, fetchNFTDetail]
  )

  const cancelListing = useCallback(async () => {
    if (!listingId) return

    try {
      await nftMarketplaceService.cancelListing(listingId)
      setListingId(null)
      setOffers([])
      await fetchNFTDetail()
    } catch (err) {
      throw err
    }
  }, [listingId, fetchNFTDetail])

  const makeOffer = useCallback(
    async (amount: number) => {
      if (!listingId) throw new Error('NFT is not listed')

      try {
        const offer = await nftMarketplaceService.makeOffer(listingId, amount)
        setOffers((prev) => [...prev, offer])
        return offer
      } catch (err) {
        throw err
      }
    },
    [listingId]
  )

  const acceptOffer = useCallback(
    async (offerId: string) => {
      try {
        const transaction = await nftMarketplaceService.acceptOffer(offerId)
        await fetchNFTDetail()
        return transaction
      } catch (err) {
        throw err
      }
    },
    [fetchNFTDetail]
  )

  return {
    nft,
    offers,
    loading,
    error,
    isListed: listingId !== null,
    listingId,
    transferNFT,
    listNFT,
    cancelListing,
    makeOffer,
    acceptOffer,
    refresh: fetchNFTDetail,
  }
}
