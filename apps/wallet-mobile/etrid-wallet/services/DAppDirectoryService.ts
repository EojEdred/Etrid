/**
 * DApp Directory Service
 * Handles dApp discovery, search, and directory management
 */

import { DApp, DAppCategory, Bookmark } from '@/types/dapp';

export class DAppDirectoryService {
  private bookmarks: Map<string, Bookmark> = new Map();
  private recentlyVisited: DApp[] = [];
  private cachedDApps: DApp[] = [];

  /**
   * Get featured dApps
   */
  async getFeaturedDApps(): Promise<DApp[]> {
    // TODO: Fetch from API
    return this.getMockFeaturedDApps();
  }

  /**
   * Get dApps by category
   */
  async getDAppsByCategory(category: DAppCategory): Promise<DApp[]> {
    // TODO: Fetch from API
    const allDApps = this.getMockDApps();
    return allDApps.filter((dApp) => dApp.category === category);
  }

  /**
   * Search dApps
   */
  async searchDApps(query: string): Promise<DApp[]> {
    // TODO: Implement actual search API
    const allDApps = this.getMockDApps();
    const lowerQuery = query.toLowerCase();

    return allDApps.filter(
      (dApp) =>
        dApp.name.toLowerCase().includes(lowerQuery) ||
        dApp.description.toLowerCase().includes(lowerQuery) ||
        dApp.category.toLowerCase().includes(lowerQuery)
    );
  }

  /**
   * Get trending dApps
   */
  async getTrendingDApps(): Promise<DApp[]> {
    // TODO: Fetch from API based on user activity
    return this.getMockDApps()
      .filter((dApp) => dApp.isTrending)
      .slice(0, 10);
  }

  /**
   * Get recently visited dApps
   */
  async getRecentlyVisited(): Promise<DApp[]> {
    // Load from storage if not in memory
    if (this.recentlyVisited.length === 0) {
      await this.loadRecentlyVisited();
    }
    return this.recentlyVisited;
  }

  /**
   * Add to recently visited
   */
  async addToRecentlyVisited(dApp: DApp): Promise<void> {
    // Remove if already exists
    this.recentlyVisited = this.recentlyVisited.filter((d) => d.id !== dApp.id);

    // Add to beginning
    this.recentlyVisited.unshift(dApp);

    // Keep only last 20
    if (this.recentlyVisited.length > 20) {
      this.recentlyVisited = this.recentlyVisited.slice(0, 20);
    }

    // Persist to storage
    await this.saveRecentlyVisited();
  }

  /**
   * Get all bookmarks
   */
  async getBookmarks(): Promise<Bookmark[]> {
    if (this.bookmarks.size === 0) {
      await this.loadBookmarks();
    }
    return Array.from(this.bookmarks.values());
  }

  /**
   * Get bookmarks by folder
   */
  async getBookmarksByFolder(folder?: string): Promise<Bookmark[]> {
    const allBookmarks = await this.getBookmarks();
    if (!folder) {
      return allBookmarks.filter((b) => !b.folder);
    }
    return allBookmarks.filter((b) => b.folder === folder);
  }

  /**
   * Add bookmark
   */
  async addBookmark(bookmark: Omit<Bookmark, 'id' | 'createdAt'>): Promise<Bookmark> {
    const newBookmark: Bookmark = {
      ...bookmark,
      id: Date.now().toString(),
      createdAt: new Date(),
    };

    this.bookmarks.set(newBookmark.id, newBookmark);
    await this.saveBookmarks();

    return newBookmark;
  }

  /**
   * Update bookmark
   */
  async updateBookmark(
    id: string,
    updates: Partial<Omit<Bookmark, 'id' | 'userId' | 'createdAt'>>
  ): Promise<Bookmark> {
    const bookmark = this.bookmarks.get(id);
    if (!bookmark) {
      throw new Error('Bookmark not found');
    }

    const updated = { ...bookmark, ...updates };
    this.bookmarks.set(id, updated);
    await this.saveBookmarks();

    return updated;
  }

  /**
   * Delete bookmark
   */
  async deleteBookmark(id: string): Promise<void> {
    this.bookmarks.delete(id);
    await this.saveBookmarks();
  }

  /**
   * Check if URL is bookmarked
   */
  async isBookmarked(url: string): Promise<boolean> {
    const bookmarks = await this.getBookmarks();
    return bookmarks.some((b) => b.url === url);
  }

  /**
   * Get bookmark folders
   */
  async getFolders(): Promise<string[]> {
    const bookmarks = await this.getBookmarks();
    const folders = new Set(
      bookmarks.filter((b) => b.folder).map((b) => b.folder!)
    );
    return Array.from(folders);
  }

  /**
   * Add custom dApp
   */
  async addCustomDApp(dApp: Omit<DApp, 'id'>): Promise<DApp> {
    const newDApp: DApp = {
      ...dApp,
      id: Date.now().toString(),
    };

    // Add to cache
    this.cachedDApps.push(newDApp);

    // Also bookmark it
    await this.addBookmark({
      userId: 'current-user', // TODO: Get from auth service
      url: newDApp.url,
      title: newDApp.name,
      faviconUrl: newDApp.iconUrl,
      folder: 'Custom',
    });

    return newDApp;
  }

  // Storage methods
  private async saveBookmarks(): Promise<void> {
    const bookmarksArray = Array.from(this.bookmarks.values());
    // TODO: Persist to AsyncStorage or API
    // await AsyncStorage.setItem('dapp_bookmarks', JSON.stringify(bookmarksArray));
  }

  private async loadBookmarks(): Promise<void> {
    // TODO: Load from AsyncStorage or API
    // const bookmarksJson = await AsyncStorage.getItem('dapp_bookmarks');
    // if (bookmarksJson) {
    //   const bookmarks = JSON.parse(bookmarksJson);
    //   bookmarks.forEach((bookmark: Bookmark) => {
    //     this.bookmarks.set(bookmark.id, bookmark);
    //   });
    // }
  }

  private async saveRecentlyVisited(): Promise<void> {
    // TODO: Persist to AsyncStorage
    // await AsyncStorage.setItem('dapp_recently_visited', JSON.stringify(this.recentlyVisited));
  }

  private async loadRecentlyVisited(): Promise<void> {
    // TODO: Load from AsyncStorage
    // const recentJson = await AsyncStorage.getItem('dapp_recently_visited');
    // if (recentJson) {
    //   this.recentlyVisited = JSON.parse(recentJson);
    // }
  }

  // Mock data methods
  private getMockFeaturedDApps(): DApp[] {
    return [
      {
        id: '1',
        name: 'Uniswap',
        url: 'https://app.uniswap.org',
        category: 'DeFi',
        description: 'Decentralized exchange protocol',
        iconUrl: 'https://app.uniswap.org/favicon.ico',
        rating: 4.8,
        userCount: 1000000,
        isFeatured: true,
      },
      {
        id: '2',
        name: 'OpenSea',
        url: 'https://opensea.io',
        category: 'NFT',
        description: 'NFT marketplace',
        iconUrl: 'https://opensea.io/favicon.ico',
        rating: 4.5,
        userCount: 500000,
        isFeatured: true,
      },
      {
        id: '3',
        name: 'Aave',
        url: 'https://app.aave.com',
        category: 'DeFi',
        description: 'Decentralized lending protocol',
        iconUrl: 'https://app.aave.com/favicon.ico',
        rating: 4.7,
        userCount: 300000,
        isFeatured: true,
      },
    ];
  }

  private getMockDApps(): DApp[] {
    return [
      ...this.getMockFeaturedDApps(),
      {
        id: '4',
        name: 'Compound',
        url: 'https://app.compound.finance',
        category: 'DeFi',
        description: 'Algorithmic money market protocol',
        iconUrl: 'https://app.compound.finance/favicon.ico',
        rating: 4.6,
        userCount: 200000,
      },
      {
        id: '5',
        name: 'Rarible',
        url: 'https://rarible.com',
        category: 'NFT',
        description: 'NFT marketplace and minting platform',
        iconUrl: 'https://rarible.com/favicon.ico',
        rating: 4.3,
        userCount: 150000,
        isTrending: true,
      },
      {
        id: '6',
        name: 'Axie Infinity',
        url: 'https://app.axieinfinity.com',
        category: 'Gaming',
        description: 'Play-to-earn NFT game',
        iconUrl: 'https://app.axieinfinity.com/favicon.ico',
        rating: 4.4,
        userCount: 800000,
        isTrending: true,
      },
      {
        id: '7',
        name: 'Lens Protocol',
        url: 'https://lenster.xyz',
        category: 'Social',
        description: 'Decentralized social network',
        iconUrl: 'https://lenster.xyz/favicon.ico',
        rating: 4.2,
        userCount: 100000,
        isTrending: true,
      },
      {
        id: '8',
        name: 'ENS',
        url: 'https://app.ens.domains',
        category: 'Tools',
        description: 'Ethereum Name Service',
        iconUrl: 'https://app.ens.domains/favicon.ico',
        rating: 4.9,
        userCount: 400000,
      },
      {
        id: '9',
        name: 'Curve',
        url: 'https://curve.fi',
        category: 'DeFi',
        description: 'Stablecoin exchange',
        iconUrl: 'https://curve.fi/favicon.ico',
        rating: 4.5,
        userCount: 250000,
      },
      {
        id: '10',
        name: 'Mirror',
        url: 'https://mirror.xyz',
        category: 'Social',
        description: 'Decentralized publishing platform',
        iconUrl: 'https://mirror.xyz/favicon.ico',
        rating: 4.1,
        userCount: 50000,
      },
    ];
  }
}

export const dAppDirectoryService = new DAppDirectoryService();
