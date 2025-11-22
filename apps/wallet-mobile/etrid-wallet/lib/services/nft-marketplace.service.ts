// NFT Marketplace Service - Handle marketplace operations

import type {
  Listing,
  ListingParams,
  Offer,
  MarketplaceFilters,
  NFT,
} from '@/lib/types/nft'
import type { Transaction } from './nft.service'

export class NFTMarketplaceService {
  private apiUrl: string

  constructor(apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api') {
    this.apiUrl = apiUrl
  }

  /**
   * List an NFT for sale
   */
  async listNFT(params: ListingParams): Promise<Listing> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/list`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(params),
      })

      if (!response.ok) {
        throw new Error(`Failed to list NFT: ${response.statusText}`)
      }

      const data = await response.json()
      return data.listing
    } catch (error) {
      console.error('Error listing NFT:', error)
      throw error
    }
  }

  /**
   * Buy an NFT from a listing
   */
  async buyNFT(listingId: string): Promise<Transaction> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/buy`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ listing_id: listingId }),
      })

      if (!response.ok) {
        throw new Error(`Failed to buy NFT: ${response.statusText}`)
      }

      const data = await response.json()
      return data.transaction
    } catch (error) {
      console.error('Error buying NFT:', error)
      throw error
    }
  }

  /**
   * Make an offer on a listing
   */
  async makeOffer(listingId: string, amount: number): Promise<Offer> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/offer`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          listing_id: listingId,
          amount,
        }),
      })

      if (!response.ok) {
        throw new Error(`Failed to make offer: ${response.statusText}`)
      }

      const data = await response.json()
      return data.offer
    } catch (error) {
      console.error('Error making offer:', error)
      throw error
    }
  }

  /**
   * Cancel a listing
   */
  async cancelListing(listingId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/listing/${listingId}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error(`Failed to cancel listing: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error cancelling listing:', error)
      throw error
    }
  }

  /**
   * Get marketplace listings with filters
   */
  async getListings(filters?: MarketplaceFilters): Promise<Listing[]> {
    try {
      const params = new URLSearchParams()

      if (filters) {
        Object.entries(filters).forEach(([key, value]) => {
          if (value !== undefined) {
            params.append(key, value.toString())
          }
        })
      }

      const response = await fetch(`${this.apiUrl}/marketplace/listings?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch listings: ${response.statusText}`)
      }

      const data = await response.json()
      return data.listings || []
    } catch (error) {
      console.error('Error fetching listings:', error)
      throw error
    }
  }

  /**
   * Get featured NFTs for homepage
   */
  async getFeatured(): Promise<NFT[]> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/featured`)

      if (!response.ok) {
        throw new Error(`Failed to fetch featured NFTs: ${response.statusText}`)
      }

      const data = await response.json()
      return data.nfts || []
    } catch (error) {
      console.error('Error fetching featured NFTs:', error)
      throw error
    }
  }

  /**
   * Get trending collections
   */
  async getTrendingCollections(limit: number = 10): Promise<any[]> {
    try {
      const params = new URLSearchParams({ limit: limit.toString() })
      const response = await fetch(`${this.apiUrl}/marketplace/trending?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch trending collections: ${response.statusText}`)
      }

      const data = await response.json()
      return data.collections || []
    } catch (error) {
      console.error('Error fetching trending collections:', error)
      throw error
    }
  }

  /**
   * Get offers for a listing
   */
  async getOffers(listingId: string): Promise<Offer[]> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/listing/${listingId}/offers`)

      if (!response.ok) {
        throw new Error(`Failed to fetch offers: ${response.statusText}`)
      }

      const data = await response.json()
      return data.offers || []
    } catch (error) {
      console.error('Error fetching offers:', error)
      throw error
    }
  }

  /**
   * Accept an offer
   */
  async acceptOffer(offerId: string): Promise<Transaction> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/offer/${offerId}/accept`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error(`Failed to accept offer: ${response.statusText}`)
      }

      const data = await response.json()
      return data.transaction
    } catch (error) {
      console.error('Error accepting offer:', error)
      throw error
    }
  }

  /**
   * Reject an offer
   */
  async rejectOffer(offerId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/marketplace/offer/${offerId}/reject`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error(`Failed to reject offer: ${response.statusText}`)
      }
    } catch (error) {
      console.error('Error rejecting offer:', error)
      throw error
    }
  }

  /**
   * Get user's active listings
   */
  async getUserListings(address: string): Promise<Listing[]> {
    try {
      const params = new URLSearchParams({ seller: address, status: 'active' })
      const response = await fetch(`${this.apiUrl}/marketplace/listings?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch user listings: ${response.statusText}`)
      }

      const data = await response.json()
      return data.listings || []
    } catch (error) {
      console.error('Error fetching user listings:', error)
      throw error
    }
  }
}

// Export singleton instance
export const nftMarketplaceService = new NFTMarketplaceService()
