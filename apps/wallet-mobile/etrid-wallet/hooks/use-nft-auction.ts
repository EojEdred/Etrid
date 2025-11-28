// useNFTAuction Hook - Manage NFT auctions

import { useState, useEffect, useCallback } from 'react'
import { nftAuctionService } from '@/lib/services/nft-auction.service'
import type { Auction, Bid, AuctionParams } from '@/lib/types/nft'

export function useNFTAuction(auctionId?: string) {
  const [auctions, setAuctions] = useState<Auction[]>([])
  const [currentAuction, setCurrentAuction] = useState<Auction | null>(null)
  const [bids, setBids] = useState<Bid[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  const [timeRemaining, setTimeRemaining] = useState<{
    days: number
    hours: number
    minutes: number
    seconds: number
  } | null>(null)

  const fetchAuctions = useCallback(async (filter?: 'active' | 'ended') => {
    try {
      setLoading(true)
      setError(null)

      const data = await nftAuctionService.getAuctions(filter)
      setAuctions(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [])

  const fetchAuctionDetail = useCallback(async (id: string) => {
    try {
      setLoading(true)
      setError(null)

      const auction = await nftAuctionService.getAuction(id)
      setCurrentAuction(auction)

      const bidsData = await nftAuctionService.getBidHistory(id)
      setBids(bidsData)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    if (auctionId) {
      fetchAuctionDetail(auctionId)
    } else {
      fetchAuctions('active')
    }
  }, [auctionId])

  // Update time remaining every second
  useEffect(() => {
    if (!currentAuction) return

    const updateTimer = () => {
      const remaining = nftAuctionService.getTimeRemaining(currentAuction.end_time)
      setTimeRemaining(remaining)
    }

    updateTimer()
    const interval = setInterval(updateTimer, 1000)

    return () => clearInterval(interval)
  }, [currentAuction])

  const createAuction = useCallback(
    async (params: AuctionParams) => {
      try {
        const auction = await nftAuctionService.createAuction(params)
        await fetchAuctions('active')
        return auction
      } catch (err) {
        throw err
      }
    },
    [fetchAuctions]
  )

  const placeBid = useCallback(
    async (auctionIdToBid: string, amount: number) => {
      try {
        const bid = await nftAuctionService.placeBid(auctionIdToBid, amount)

        // Refresh auction and bids
        if (auctionId === auctionIdToBid) {
          await fetchAuctionDetail(auctionIdToBid)
        }

        return bid
      } catch (err) {
        throw err
      }
    },
    [auctionId, fetchAuctionDetail]
  )

  const claimAuction = useCallback(
    async (auctionIdToClaim: string) => {
      try {
        const transaction = await nftAuctionService.claimAuction(auctionIdToClaim)

        if (auctionId === auctionIdToClaim) {
          await fetchAuctionDetail(auctionIdToClaim)
        }

        return transaction
      } catch (err) {
        throw err
      }
    },
    [auctionId, fetchAuctionDetail]
  )

  const cancelAuction = useCallback(
    async (auctionIdToCancel: string) => {
      try {
        await nftAuctionService.cancelAuction(auctionIdToCancel)
        await fetchAuctions('active')
      } catch (err) {
        throw err
      }
    },
    [fetchAuctions]
  )

  const setupAutoBid = useCallback(
    async (auctionIdForAutoBid: string, maxBid: number, increment: number) => {
      try {
        const autoBid = await nftAuctionService.setupAutoBid(
          auctionIdForAutoBid,
          maxBid,
          increment
        )
        return autoBid
      } catch (err) {
        throw err
      }
    },
    []
  )

  const getUserAuctions = useCallback(async (address: string) => {
    try {
      const data = await nftAuctionService.getUserAuctions(address)
      return data
    } catch (err) {
      throw err
    }
  }, [])

  const isEndingSoon = currentAuction
    ? nftAuctionService.isEndingSoon(currentAuction.end_time)
    : false

  return {
    auctions,
    currentAuction,
    bids,
    loading,
    error,
    timeRemaining,
    isEndingSoon,
    createAuction,
    placeBid,
    claimAuction,
    cancelAuction,
    setupAutoBid,
    getUserAuctions,
    refresh: auctionId ? () => fetchAuctionDetail(auctionId) : () => fetchAuctions('active'),
  }
}
