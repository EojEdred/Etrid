// NFT Type Definitions

export interface NFTAttribute {
  trait_type: string
  value: string | number
  rarity?: number // Percentage (0-100)
  floor_price?: number
}

export interface NFTMetadata {
  name: string
  description: string
  image: string
  image_url?: string
  animation_url?: string
  external_url?: string
  attributes: NFTAttribute[]
  properties?: Record<string, any>
}

export interface NFT {
  id: string
  contract_address: string
  token_id: string
  chain: string
  owner: string
  name: string
  description: string
  image_url: string
  animation_url?: string
  metadata: NFTMetadata
  collection?: Collection
  token_uri?: string
  created_at: string
  updated_at: string
}

export interface Collection {
  id: string
  name: string
  description: string
  image_url: string
  banner_url?: string
  creator: string
  contract_address: string
  chain: string
  floor_price?: number
  volume_24h?: number
  total_supply?: number
  owners?: number
  verified?: boolean
  created_at: string
}

export interface Listing {
  id: string
  nft: NFT
  seller: string
  price: number
  currency: string
  status: 'active' | 'sold' | 'cancelled' | 'expired'
  created_at: string
  expires_at?: string
  sold_at?: string
}

export interface Offer {
  id: string
  listing_id: string
  nft_id: string
  bidder: string
  amount: number
  currency: string
  status: 'pending' | 'accepted' | 'rejected' | 'expired'
  created_at: string
  expires_at?: string
}

export interface Auction {
  id: string
  nft: NFT
  seller: string
  start_price: number
  reserve_price?: number
  current_bid?: number
  highest_bidder?: string
  currency: string
  status: 'pending' | 'active' | 'ended' | 'cancelled'
  start_time: string
  end_time: string
  created_at: string
}

export interface Bid {
  id: string
  auction_id: string
  bidder: string
  amount: number
  currency: string
  timestamp: string
  tx_hash?: string
}

export interface NFTSale {
  id: string
  nft: NFT
  seller: string
  buyer: string
  price: number
  currency: string
  timestamp: string
  tx_hash: string
}

export interface ListingParams {
  contract_address: string
  token_id: string
  price: number
  currency: string
  duration?: number // in days
}

export interface AuctionParams {
  contract_address: string
  token_id: string
  start_price: number
  reserve_price?: number
  currency: string
  duration: number // in hours
}

export interface MarketplaceFilters {
  chain?: string
  collection?: string
  category?: string
  min_price?: number
  max_price?: number
  sort_by?: 'price_asc' | 'price_desc' | 'recent' | 'ending_soon'
  status?: 'active' | 'sold'
}

export interface CollectionInput {
  name: string
  description: string
  symbol: string
  image_file?: File
  banner_file?: File
  royalty_percentage: number
  chain: string
}

export interface MintParams {
  name: string
  description: string
  file: File
  attributes: NFTAttribute[]
  collection_id?: string
  royalty_percentage: number
  chain: string
}

export type NFTCategory = 'art' | 'music' | 'gaming' | 'collectibles' | 'photography' | 'sports' | 'virtual_worlds'
