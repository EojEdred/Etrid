import {
  VirtualAsset,
  LandNFT,
  WearableNFT,
  Gallery,
  MetaverseEvent,
  MetaversePlatform,
  GalleryLayout,
} from '../types/metaverse'

export class MetaverseService {
  private static instance: MetaverseService

  static getInstance(): MetaverseService {
    if (!MetaverseService.instance) {
      MetaverseService.instance = new MetaverseService()
    }
    return MetaverseService.instance
  }

  async getVirtualAssets(): Promise<VirtualAsset[]> {
    try {
      const response = await fetch('/api/metaverse/assets')
      if (!response.ok) throw new Error('Failed to fetch virtual assets')
      return await response.json()
    } catch (error) {
      console.error('Error fetching virtual assets:', error)
      return this.getMockVirtualAssets()
    }
  }

  async getLandNFTs(): Promise<LandNFT[]> {
    try {
      const response = await fetch('/api/metaverse/land')
      if (!response.ok) throw new Error('Failed to fetch land NFTs')
      return await response.json()
    } catch (error) {
      console.error('Error fetching land NFTs:', error)
      return this.getMockLandNFTs()
    }
  }

  async getWearables(): Promise<WearableNFT[]> {
    try {
      const response = await fetch('/api/metaverse/wearables')
      if (!response.ok) throw new Error('Failed to fetch wearables')
      return await response.json()
    } catch (error) {
      console.error('Error fetching wearables:', error)
      return this.getMockWearables()
    }
  }

  async equipWearable(id: string): Promise<void> {
    try {
      const response = await fetch(`/api/metaverse/wearables/${id}/equip`, {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to equip wearable')
    } catch (error) {
      console.error('Error equipping wearable:', error)
      throw error
    }
  }

  async unequipWearable(id: string): Promise<void> {
    try {
      const response = await fetch(`/api/metaverse/wearables/${id}/unequip`, {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to unequip wearable')
    } catch (error) {
      console.error('Error unequipping wearable:', error)
      throw error
    }
  }

  async createGallery(nftIds: string[], name: string): Promise<Gallery> {
    try {
      const response = await fetch('/api/metaverse/gallery', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ nftIds, name }),
      })
      if (!response.ok) throw new Error('Failed to create gallery')
      return await response.json()
    } catch (error) {
      console.error('Error creating gallery:', error)
      throw error
    }
  }

  async getGalleries(): Promise<Gallery[]> {
    try {
      const response = await fetch('/api/metaverse/galleries')
      if (!response.ok) throw new Error('Failed to fetch galleries')
      return await response.json()
    } catch (error) {
      console.error('Error fetching galleries:', error)
      return []
    }
  }

  async updateGalleryLayout(galleryId: string, layout: GalleryLayout): Promise<void> {
    try {
      const response = await fetch(`/api/metaverse/gallery/${galleryId}/layout`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(layout),
      })
      if (!response.ok) throw new Error('Failed to update gallery layout')
    } catch (error) {
      console.error('Error updating gallery layout:', error)
      throw error
    }
  }

  async getEvents(): Promise<MetaverseEvent[]> {
    try {
      const response = await fetch('/api/metaverse/events')
      if (!response.ok) throw new Error('Failed to fetch events')
      return await response.json()
    } catch (error) {
      console.error('Error fetching events:', error)
      return this.getMockEvents()
    }
  }

  async rsvpEvent(eventId: string): Promise<void> {
    try {
      const response = await fetch(`/api/metaverse/events/${eventId}/rsvp`, {
        method: 'POST',
      })
      if (!response.ok) throw new Error('Failed to RSVP to event')
    } catch (error) {
      console.error('Error RSVPing to event:', error)
      throw error
    }
  }

  // Mock data methods
  private getMockVirtualAssets(): VirtualAsset[] {
    return [
      {
        id: '1',
        type: 'land',
        name: 'Ëtrid Plaza',
        description: 'Premium location in downtown district',
        platform: 'etrid_metaverse',
        imageUrl: '/assets/land-1.jpg',
        value: 15000,
        metadata: {},
      },
      {
        id: '2',
        type: 'wearable',
        name: 'Cyberpunk Jacket',
        description: 'Legendary rarity jacket',
        platform: 'etrid_metaverse',
        imageUrl: '/assets/jacket-1.jpg',
        value: 500,
        metadata: {},
      },
    ]
  }

  private getMockLandNFTs(): LandNFT[] {
    return [
      {
        id: '1',
        type: 'land',
        name: 'Ëtrid Plaza',
        description: 'Premium location in downtown district',
        platform: 'etrid_metaverse',
        imageUrl: '/assets/land-1.jpg',
        value: 15000,
        metadata: {},
        coordinates: { x: 100, y: 50 },
        size: { width: 16, height: 16 },
        parcelId: 'ETR-100-50',
        rented: false,
      },
      {
        id: '2',
        type: 'land',
        name: 'Beachfront Property',
        description: 'Scenic ocean view',
        platform: 'sandbox',
        imageUrl: '/assets/land-2.jpg',
        value: 8500,
        metadata: {},
        coordinates: { x: -42, y: 28 },
        size: { width: 3, height: 3 },
        parcelId: 'SB--42-28',
        rented: true,
        rentalIncome: 250,
      },
    ]
  }

  private getMockWearables(): WearableNFT[] {
    return [
      {
        id: '1',
        type: 'wearable',
        name: 'Cyberpunk Jacket',
        description: 'Legendary rarity jacket',
        platform: 'etrid_metaverse',
        imageUrl: '/assets/jacket-1.jpg',
        value: 500,
        metadata: {},
        category: 'upper_body',
        rarity: 'legendary',
        equipped: true,
        compatiblePlatforms: ['etrid_metaverse', 'decentraland'],
      },
      {
        id: '2',
        type: 'wearable',
        name: 'Neon Sneakers',
        description: 'Epic rarity footwear',
        platform: 'etrid_metaverse',
        imageUrl: '/assets/shoes-1.jpg',
        value: 200,
        metadata: {},
        category: 'feet',
        rarity: 'epic',
        equipped: false,
        compatiblePlatforms: ['etrid_metaverse', 'sandbox'],
      },
    ]
  }

  private getMockEvents(): MetaverseEvent[] {
    const now = new Date()
    return [
      {
        id: '1',
        platform: 'etrid_metaverse',
        name: 'Ëtrid Launch Party',
        description: 'Celebrate the launch of Ëtrid Metaverse',
        startDate: new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000),
        endDate: new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000 + 4 * 60 * 60 * 1000),
        url: 'https://metaverse.etrid.io/event/launch',
        bannerUrl: '/assets/event-1.jpg',
        category: 'concert',
        rewards: [
          {
            type: 'poap',
            name: 'Launch Attendee Badge',
            imageUrl: '/assets/badge-1.png',
          },
        ],
        rsvpCount: 1250,
        userRSVP: false,
      },
      {
        id: '2',
        platform: 'decentraland',
        name: 'NFT Art Exhibition',
        description: 'Curated collection of digital art',
        startDate: new Date(now.getTime() + 3 * 24 * 60 * 60 * 1000),
        endDate: new Date(now.getTime() + 10 * 24 * 60 * 60 * 1000),
        url: 'https://dcl.gg/art-expo',
        category: 'exhibition',
        rewards: [],
        rsvpCount: 842,
        userRSVP: true,
      },
    ]
  }
}

export const metaverseService = MetaverseService.getInstance()
