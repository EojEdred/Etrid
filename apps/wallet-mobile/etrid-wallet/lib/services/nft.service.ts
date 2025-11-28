// NFT Service - Handle NFT operations

import type { NFT, NFTDetail, NFTMetadata } from '@/lib/types/nft'

export interface NFTDetail extends NFT {
  price_history: Array<{ price: number; timestamp: string }>
  transfer_history: Array<{
    from: string
    to: string
    price?: number
    timestamp: string
    tx_hash: string
  }>
  rarity_rank?: number
  rarity_score?: number
}

export interface Transaction {
  hash: string
  status: 'pending' | 'success' | 'failed'
  timestamp: string
  from: string
  to: string
  value?: string
}

export class NFTService {
  private apiUrl: string

  constructor(apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api') {
    this.apiUrl = apiUrl
  }

  /**
   * Get all NFTs owned by an address
   */
  async getUserNFTs(address: string, chain?: string): Promise<NFT[]> {
    try {
      const params = new URLSearchParams({ address })
      if (chain) params.append('chain', chain)

      const response = await fetch(`${this.apiUrl}/nfts/gallery?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch NFTs: ${response.statusText}`)
      }

      const data = await response.json()
      return data.nfts || []
    } catch (error) {
      console.error('Error fetching user NFTs:', error)
      throw error
    }
  }

  /**
   * Get detailed information about a specific NFT
   */
  async getNFTDetails(contractAddress: string, tokenId: string): Promise<NFTDetail> {
    try {
      const response = await fetch(
        `${this.apiUrl}/nfts/${contractAddress}/${tokenId}`
      )

      if (!response.ok) {
        throw new Error(`Failed to fetch NFT details: ${response.statusText}`)
      }

      const data = await response.json()
      return data
    } catch (error) {
      console.error('Error fetching NFT details:', error)
      throw error
    }
  }

  /**
   * Transfer an NFT to another address
   */
  async transferNFT(
    to: string,
    contractAddress: string,
    tokenId: string
  ): Promise<Transaction> {
    try {
      const response = await fetch(`${this.apiUrl}/nfts/transfer`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          to,
          contract_address: contractAddress,
          token_id: tokenId,
        }),
      })

      if (!response.ok) {
        throw new Error(`Failed to transfer NFT: ${response.statusText}`)
      }

      const data = await response.json()
      return data.transaction
    } catch (error) {
      console.error('Error transferring NFT:', error)
      throw error
    }
  }

  /**
   * Fetch NFT metadata from token URI
   */
  async getNFTMetadata(tokenURI: string): Promise<NFTMetadata> {
    try {
      // Handle IPFS URIs
      let url = tokenURI
      if (tokenURI.startsWith('ipfs://')) {
        url = tokenURI.replace('ipfs://', 'https://ipfs.io/ipfs/')
      }

      const response = await fetch(url)

      if (!response.ok) {
        throw new Error(`Failed to fetch metadata: ${response.statusText}`)
      }

      const metadata = await response.json()
      return metadata
    } catch (error) {
      console.error('Error fetching NFT metadata:', error)
      throw error
    }
  }

  /**
   * Get NFTs by collection
   */
  async getCollectionNFTs(
    collectionAddress: string,
    limit: number = 50,
    offset: number = 0
  ): Promise<{ nfts: NFT[]; total: number }> {
    try {
      const params = new URLSearchParams({
        collection: collectionAddress,
        limit: limit.toString(),
        offset: offset.toString(),
      })

      const response = await fetch(`${this.apiUrl}/nfts/collection?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch collection NFTs: ${response.statusText}`)
      }

      const data = await response.json()
      return data
    } catch (error) {
      console.error('Error fetching collection NFTs:', error)
      throw error
    }
  }

  /**
   * Search NFTs
   */
  async searchNFTs(query: string, filters?: {
    chain?: string
    collection?: string
    min_price?: number
    max_price?: number
  }): Promise<NFT[]> {
    try {
      const params = new URLSearchParams({ query })

      if (filters) {
        Object.entries(filters).forEach(([key, value]) => {
          if (value !== undefined) {
            params.append(key, value.toString())
          }
        })
      }

      const response = await fetch(`${this.apiUrl}/nfts/search?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to search NFTs: ${response.statusText}`)
      }

      const data = await response.json()
      return data.nfts || []
    } catch (error) {
      console.error('Error searching NFTs:', error)
      throw error
    }
  }

  /**
   * Bulk transfer NFTs
   */
  async bulkTransferNFTs(
    transfers: Array<{
      to: string
      contract_address: string
      token_id: string
    }>
  ): Promise<Transaction[]> {
    try {
      const response = await fetch(`${this.apiUrl}/nfts/bulk-transfer`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ transfers }),
      })

      if (!response.ok) {
        throw new Error(`Failed to bulk transfer NFTs: ${response.statusText}`)
      }

      const data = await response.json()
      return data.transactions
    } catch (error) {
      console.error('Error bulk transferring NFTs:', error)
      throw error
    }
  }
}

// Export singleton instance
export const nftService = new NFTService()
