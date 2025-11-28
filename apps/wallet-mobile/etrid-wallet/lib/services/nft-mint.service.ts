// NFT Mint Service - Handle NFT creation and minting

import type {
  NFTMetadata,
  Collection,
  CollectionInput,
  MintParams,
} from '@/lib/types/nft'
import type { Transaction } from './nft.service'

export class NFTMintService {
  private apiUrl: string
  private ipfsGateway: string

  constructor(
    apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api',
    ipfsGateway: string = process.env.NEXT_PUBLIC_IPFS_GATEWAY || 'https://ipfs.io/ipfs/'
  ) {
    this.apiUrl = apiUrl
    this.ipfsGateway = ipfsGateway
  }

  /**
   * Upload file to IPFS
   */
  async uploadToIPFS(file: File): Promise<string> {
    try {
      const formData = new FormData()
      formData.append('file', file)

      const response = await fetch(`${this.apiUrl}/ipfs/upload`, {
        method: 'POST',
        body: formData,
      })

      if (!response.ok) {
        throw new Error(`Failed to upload to IPFS: ${response.statusText}`)
      }

      const data = await response.json()
      return data.ipfs_hash // Returns the IPFS hash (CID)
    } catch (error) {
      console.error('Error uploading to IPFS:', error)
      throw error
    }
  }

  /**
   * Upload JSON metadata to IPFS
   */
  async uploadMetadataToIPFS(metadata: NFTMetadata): Promise<string> {
    try {
      const response = await fetch(`${this.apiUrl}/ipfs/upload-json`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(metadata),
      })

      if (!response.ok) {
        throw new Error(`Failed to upload metadata to IPFS: ${response.statusText}`)
      }

      const data = await response.json()
      return data.ipfs_hash
    } catch (error) {
      console.error('Error uploading metadata to IPFS:', error)
      throw error
    }
  }

  /**
   * Mint a new NFT
   */
  async mintNFT(params: MintParams): Promise<Transaction> {
    try {
      // Step 1: Upload the file to IPFS
      const imageHash = await this.uploadToIPFS(params.file)
      const imageUrl = `ipfs://${imageHash}`

      // Step 2: Prepare metadata
      const metadata: NFTMetadata = {
        name: params.name,
        description: params.description,
        image: imageUrl,
        attributes: params.attributes,
        properties: {
          collection_id: params.collection_id,
          royalty_percentage: params.royalty_percentage,
        },
      }

      // Step 3: Upload metadata to IPFS
      const metadataHash = await this.uploadMetadataToIPFS(metadata)
      const tokenURI = `ipfs://${metadataHash}`

      // Step 4: Mint the NFT on-chain
      const response = await fetch(`${this.apiUrl}/nfts/mint`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          token_uri: tokenURI,
          chain: params.chain,
          collection_id: params.collection_id,
        }),
      })

      if (!response.ok) {
        throw new Error(`Failed to mint NFT: ${response.statusText}`)
      }

      const data = await response.json()
      return data.transaction
    } catch (error) {
      console.error('Error minting NFT:', error)
      throw error
    }
  }

  /**
   * Create a new NFT collection
   */
  async createCollection(collection: CollectionInput): Promise<Collection> {
    try {
      let imageUrl: string | undefined
      let bannerUrl: string | undefined

      // Upload collection images to IPFS if provided
      if (collection.image_file) {
        const imageHash = await this.uploadToIPFS(collection.image_file)
        imageUrl = `ipfs://${imageHash}`
      }

      if (collection.banner_file) {
        const bannerHash = await this.uploadToIPFS(collection.banner_file)
        bannerUrl = `ipfs://${bannerHash}`
      }

      const response = await fetch(`${this.apiUrl}/nfts/collection`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: collection.name,
          description: collection.description,
          symbol: collection.symbol,
          image_url: imageUrl,
          banner_url: bannerUrl,
          royalty_percentage: collection.royalty_percentage,
          chain: collection.chain,
        }),
      })

      if (!response.ok) {
        throw new Error(`Failed to create collection: ${response.statusText}`)
      }

      const data = await response.json()
      return data.collection
    } catch (error) {
      console.error('Error creating collection:', error)
      throw error
    }
  }

  /**
   * Get user's collections
   */
  async getUserCollections(creatorAddress: string): Promise<Collection[]> {
    try {
      const params = new URLSearchParams({ creator: creatorAddress })
      const response = await fetch(`${this.apiUrl}/nfts/collections?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch collections: ${response.statusText}`)
      }

      const data = await response.json()
      return data.collections || []
    } catch (error) {
      console.error('Error fetching collections:', error)
      throw error
    }
  }

  /**
   * Validate file for NFT minting
   */
  validateFile(file: File): { valid: boolean; error?: string } {
    const maxSize = 100 * 1024 * 1024 // 100MB
    const allowedTypes = [
      'image/jpeg',
      'image/png',
      'image/gif',
      'image/webp',
      'image/svg+xml',
      'video/mp4',
      'video/webm',
      'audio/mpeg',
      'audio/wav',
    ]

    if (file.size > maxSize) {
      return {
        valid: false,
        error: 'File size exceeds 100MB limit',
      }
    }

    if (!allowedTypes.includes(file.type)) {
      return {
        valid: false,
        error: 'File type not supported',
      }
    }

    return { valid: true }
  }

  /**
   * Mint and list NFT in one transaction
   */
  async mintAndList(
    mintParams: MintParams,
    listingPrice: number,
    listingCurrency: string = 'ETRID'
  ): Promise<{ mintTransaction: Transaction; listingId: string }> {
    try {
      // First mint the NFT
      const mintTransaction = await this.mintNFT(mintParams)

      // Wait for mint to complete (this would need proper blockchain confirmation)
      // For now, we'll proceed optimistically

      // Then list it for sale
      const listingResponse = await fetch(`${this.apiUrl}/marketplace/list`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          contract_address: mintTransaction.to, // Assuming the contract address is in the transaction
          token_id: 'pending', // Would need to extract from transaction
          price: listingPrice,
          currency: listingCurrency,
        }),
      })

      if (!listingResponse.ok) {
        throw new Error(`Failed to list NFT: ${listingResponse.statusText}`)
      }

      const listingData = await listingResponse.json()

      return {
        mintTransaction,
        listingId: listingData.listing.id,
      }
    } catch (error) {
      console.error('Error minting and listing NFT:', error)
      throw error
    }
  }
}

// Export singleton instance
export const nftMintService = new NFTMintService()
