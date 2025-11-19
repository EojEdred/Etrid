// NFT Auction Service - Handle auction operations

import type { Auction, AuctionParams, Bid } from '@/lib/types/nft'
import type { Transaction } from './nft.service'

export class NFTAuctionService {
  private apiUrl: string

  constructor(apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api') {
    this.apiUrl = apiUrl
  }

  /**
   * Create a new auction
   */
  async createAuction(params: AuctionParams): Promise<Auction> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/create`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(params),
      })

      if (!response.ok) {
        throw new Error(`Failed to create auction: ${response.statusText}`)
      }

      const data = await response.json()
      return data.auction
    } catch (error) {
      console.error('Error creating auction:', error)
      throw error
    }
  }

  /**
   * Place a bid on an auction
   */
  async placeBid(auctionId: string, amount: number): Promise<Bid> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}/bid`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ amount }),
      })

      if (!response.ok) {
        const errorData = await response.json()
        throw new Error(errorData.message || `Failed to place bid: ${response.statusText}`)
      }

      const data = await response.json()
      return data.bid
    } catch (error) {
      console.error('Error placing bid:', error)
      throw error
    }
  }

  /**
   * Get auctions with optional filter
   */
  async getAuctions(filter?: 'active' | 'ended' | 'pending'): Promise<Auction[]> {
    try {
      const params = new URLSearchParams()
      if (filter) {
        params.append('status', filter)
      }

      const response = await fetch(`${this.apiUrl}/auctions?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch auctions: ${response.statusText}`)
      }

      const data = await response.json()
      return data.auctions || []
    } catch (error) {
      console.error('Error fetching auctions:', error)
      throw error
    }
  }

  /**
   * Get a specific auction by ID
   */
  async getAuction(auctionId: string): Promise<Auction> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch auction: ${response.statusText}`)
      }

      const data = await response.json()
      return data.auction
    } catch (error) {
      console.error('Error fetching auction:', error)
      throw error
    }
  }

  /**
   * Get bid history for an auction
   */
  async getBidHistory(auctionId: string): Promise<Bid[]> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}/bids`)

      if (!response.ok) {
        throw new Error(`Failed to fetch bid history: ${response.statusText}`)
      }

      const data = await response.json()
      return data.bids || []
    } catch (error) {
      console.error('Error fetching bid history:', error)
      throw error
    }
  }

  /**
   * Claim an auction (for winner or seller after end)
   */
  async claimAuction(auctionId: string): Promise<Transaction> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}/claim`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error(`Failed to claim auction: ${response.statusText}`)
      }

      const data = await response.json()
      return data.transaction
    } catch (error) {
      console.error('Error claiming auction:', error)
      throw error
    }
  }

  /**
   * Cancel an auction (only if no bids)
   */
  async cancelAuction(auctionId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        const errorData = await response.json()
        throw new Error(errorData.message || `Failed to cancel auction: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error cancelling auction:', error)
      throw error
    }
  }

  /**
   * Setup auto-bid for an auction
   */
  async setupAutoBid(
    auctionId: string,
    maxBid: number,
    increment: number
  ): Promise<{ id: string; auction_id: string; max_bid: number; increment: number }> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}/auto-bid`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          max_bid: maxBid,
          increment,
        }),
      })

      if (!response.ok) {
        throw new Error(`Failed to setup auto-bid: ${response.statusText}`)
      }

      const data = await response.json()
      return data.auto_bid
    } catch (error) {
      console.error('Error setting up auto-bid:', error)
      throw error
    }
  }

  /**
   * Cancel auto-bid
   */
  async cancelAutoBid(auctionId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/auctions/${auctionId}/auto-bid`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error(`Failed to cancel auto-bid: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error cancelling auto-bid:', error)
      throw error
    }
  }

  /**
   * Get user's auction participation (as seller or bidder)
   */
  async getUserAuctions(address: string): Promise<{
    created: Auction[]
    bidding: Auction[]
    won: Auction[]
  }> {
    try {
      const params = new URLSearchParams({ user: address })
      const response = await fetch(`${this.apiUrl}/auctions/user?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch user auctions: ${response.statusText}`)
      }

      const data = await response.json()
      return {
        created: data.created || [],
        bidding: data.bidding || [],
        won: data.won || [],
      }
    } catch (error) {
      console.error('Error fetching user auctions:', error)
      throw error
    }
  }

  /**
   * Calculate time remaining for an auction
   */
  getTimeRemaining(endTime: string): {
    days: number
    hours: number
    minutes: number
    seconds: number
    total: number
  } {
    const total = new Date(endTime).getTime() - Date.now()

    if (total <= 0) {
      return { days: 0, hours: 0, minutes: 0, seconds: 0, total: 0 }
    }

    const seconds = Math.floor((total / 1000) % 60)
    const minutes = Math.floor((total / 1000 / 60) % 60)
    const hours = Math.floor((total / (1000 * 60 * 60)) % 24)
    const days = Math.floor(total / (1000 * 60 * 60 * 24))

    return { days, hours, minutes, seconds, total }
  }

  /**
   * Check if auction is ending soon (within 1 hour)
   */
  isEndingSoon(endTime: string): boolean {
    const remaining = this.getTimeRemaining(endTime)
    return remaining.total > 0 && remaining.total <= 3600000 // 1 hour in milliseconds
  }
}

// Export singleton instance
export const nftAuctionService = new NFTAuctionService()
