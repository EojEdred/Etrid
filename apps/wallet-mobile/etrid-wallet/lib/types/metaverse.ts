// Metaverse Types

export interface VirtualAsset {
  id: string
  type: VirtualAssetType
  name: string
  description: string
  platform: MetaversePlatform
  imageUrl: string
  value?: number
  metadata: Record<string, any>
}

export type VirtualAssetType = 'land' | 'wearable' | 'art' | 'building' | 'avatar'

export type MetaversePlatform =
  | 'decentraland'
  | 'sandbox'
  | 'cryptovoxels'
  | 'somnium'
  | 'etrid_metaverse'
  | 'other'

export interface LandNFT extends VirtualAsset {
  type: 'land'
  coordinates: Coordinates
  size: {
    width: number
    height: number
  }
  parcelId: string
  rented: boolean
  rentalIncome?: number
}

export interface Coordinates {
  x: number
  y: number
  z?: number
}

export interface WearableNFT extends VirtualAsset {
  type: 'wearable'
  category: WearableCategory
  rarity: Rarity
  equipped: boolean
  compatiblePlatforms: MetaversePlatform[]
}

export type WearableCategory =
  | 'head'
  | 'upper_body'
  | 'lower_body'
  | 'feet'
  | 'accessory'
  | 'skin'

export type Rarity = 'common' | 'uncommon' | 'rare' | 'epic' | 'legendary'

export interface Gallery {
  id: string
  userId: string
  name: string
  description?: string
  nftIds: string[]
  layout: GalleryLayout
  url: string
  views?: number
  createdAt: Date
  updatedAt: Date
}

export interface GalleryLayout {
  walls: Wall[]
  lighting: LightingConfig
  floor: string
  wallColor: string
}

export interface Wall {
  id: string
  position: Coordinates
  rotation: number
  nftId?: string
  nftPosition?: {
    x: number
    y: number
    scale: number
  }
}

export interface LightingConfig {
  ambient: number
  directional: number
  color: string
}

export interface MetaverseEvent {
  id: string
  platform: MetaversePlatform
  name: string
  description: string
  startDate: Date
  endDate: Date
  location?: Coordinates
  url: string
  bannerUrl?: string
  category: EventCategory
  rewards?: EventReward[]
  rsvpCount?: number
  userRSVP?: boolean
}

export type EventCategory =
  | 'concert'
  | 'exhibition'
  | 'conference'
  | 'game'
  | 'social'
  | 'education'
  | 'other'

export interface EventReward {
  type: 'poap' | 'nft' | 'token'
  name: string
  imageUrl?: string
  description?: string
}

export interface AvatarConfig {
  wearables: {
    head?: string
    upperBody?: string
    lowerBody?: string
    feet?: string
    accessories?: string[]
  }
  skin?: string
  customization?: Record<string, any>
}
