import * as FileSystem from 'expo-file-system';
import { Image } from 'react-native';

interface ImageCache {
  uri: string;
  localPath: string;
  timestamp: number;
  size: number;
}

/**
 * Image optimization and caching utility
 */
class ImageOptimizer {
  private cache: Map<string, ImageCache> = new Map();
  private cacheDir = `${FileSystem.cacheDirectory}images/`;
  private maxCacheSize = 100 * 1024 * 1024; // 100MB
  private maxCacheAge = 7 * 24 * 3600 * 1000; // 7 days

  constructor() {
    this.initializeCache();
  }

  /**
   * Initialize cache directory
   */
  private async initializeCache(): Promise<void> {
    try {
      const dirInfo = await FileSystem.getInfoAsync(this.cacheDir);
      if (!dirInfo.exists) {
        await FileSystem.makeDirectoryAsync(this.cacheDir, { intermediates: true });
      }
      await this.loadCacheMetadata();
    } catch (error) {
      console.error('Failed to initialize image cache:', error);
    }
  }

  /**
   * Load cache metadata
   */
  private async loadCacheMetadata(): Promise<void> {
    try {
      const metadataPath = `${this.cacheDir}metadata.json`;
      const metadataInfo = await FileSystem.getInfoAsync(metadataPath);

      if (metadataInfo.exists) {
        const metadata = await FileSystem.readAsStringAsync(metadataPath);
        const cacheData: ImageCache[] = JSON.parse(metadata);

        cacheData.forEach((entry) => {
          this.cache.set(entry.uri, entry);
        });
      }
    } catch (error) {
      console.error('Failed to load cache metadata:', error);
    }
  }

  /**
   * Save cache metadata
   */
  private async saveCacheMetadata(): Promise<void> {
    try {
      const metadataPath = `${this.cacheDir}metadata.json`;
      const cacheData = Array.from(this.cache.values());
      await FileSystem.writeAsStringAsync(metadataPath, JSON.stringify(cacheData));
    } catch (error) {
      console.error('Failed to save cache metadata:', error);
    }
  }

  /**
   * Get cached image or download
   */
  public async getCachedImage(uri: string): Promise<string> {
    // Check if already cached
    const cached = this.cache.get(uri);
    if (cached) {
      const fileInfo = await FileSystem.getInfoAsync(cached.localPath);
      if (fileInfo.exists) {
        // Check if not expired
        if (Date.now() - cached.timestamp < this.maxCacheAge) {
          return cached.localPath;
        }
      }
    }

    // Download and cache
    return await this.downloadAndCache(uri);
  }

  /**
   * Download and cache image
   */
  private async downloadAndCache(uri: string): Promise<string> {
    try {
      const filename = this.generateFilename(uri);
      const localPath = `${this.cacheDir}${filename}`;

      // Download
      const downloadResult = await FileSystem.downloadAsync(uri, localPath);

      if (downloadResult.status === 200) {
        const fileInfo = await FileSystem.getInfoAsync(localPath);
        const size = fileInfo.size || 0;

        // Add to cache
        const cacheEntry: ImageCache = {
          uri,
          localPath,
          timestamp: Date.now(),
          size,
        };

        this.cache.set(uri, cacheEntry);
        await this.saveCacheMetadata();

        // Cleanup if cache is too large
        await this.cleanupIfNeeded();

        return localPath;
      }

      // Fallback to original URI
      return uri;
    } catch (error) {
      console.error('Failed to download and cache image:', error);
      return uri;
    }
  }

  /**
   * Preload image
   */
  public async preloadImage(uri: string): Promise<void> {
    try {
      await Image.prefetch(uri);
    } catch (error) {
      console.error('Failed to preload image:', error);
    }
  }

  /**
   * Preload multiple images
   */
  public async preloadImages(uris: string[]): Promise<void> {
    try {
      await Promise.all(uris.map((uri) => this.preloadImage(uri)));
    } catch (error) {
      console.error('Failed to preload images:', error);
    }
  }

  /**
   * Clear cache
   */
  public async clearCache(): Promise<void> {
    try {
      // Delete all cached files
      for (const entry of this.cache.values()) {
        await FileSystem.deleteAsync(entry.localPath, { idempotent: true });
      }

      // Clear cache map
      this.cache.clear();

      // Save empty metadata
      await this.saveCacheMetadata();

      console.log('Image cache cleared');
    } catch (error) {
      console.error('Failed to clear image cache:', error);
    }
  }

  /**
   * Cleanup if cache size exceeds limit
   */
  private async cleanupIfNeeded(): Promise<void> {
    const totalSize = this.getTotalCacheSize();

    if (totalSize > this.maxCacheSize) {
      // Sort by timestamp (oldest first)
      const entries = Array.from(this.cache.entries()).sort(
        ([, a], [, b]) => a.timestamp - b.timestamp
      );

      // Remove oldest entries until size is under limit
      let currentSize = totalSize;
      for (const [uri, entry] of entries) {
        if (currentSize <= this.maxCacheSize * 0.8) break;

        try {
          await FileSystem.deleteAsync(entry.localPath, { idempotent: true });
          this.cache.delete(uri);
          currentSize -= entry.size;
        } catch (error) {
          console.error('Failed to delete cached image:', error);
        }
      }

      await this.saveCacheMetadata();
      console.log(`Cache cleanup: removed ${entries.length} entries`);
    }
  }

  /**
   * Remove expired cache entries
   */
  public async removeExpired(): Promise<void> {
    const now = Date.now();
    const expiredEntries: string[] = [];

    for (const [uri, entry] of this.cache.entries()) {
      if (now - entry.timestamp > this.maxCacheAge) {
        expiredEntries.push(uri);
        try {
          await FileSystem.deleteAsync(entry.localPath, { idempotent: true });
        } catch (error) {
          console.error('Failed to delete expired image:', error);
        }
      }
    }

    expiredEntries.forEach((uri) => this.cache.delete(uri));

    if (expiredEntries.length > 0) {
      await this.saveCacheMetadata();
      console.log(`Removed ${expiredEntries.length} expired cache entries`);
    }
  }

  /**
   * Get cache statistics
   */
  public getCacheStats(): {
    entries: number;
    totalSize: number;
    totalSizeMB: number;
    oldestEntry: Date | null;
    newestEntry: Date | null;
  } {
    const entries = Array.from(this.cache.values());
    const totalSize = this.getTotalCacheSize();

    let oldestTimestamp = Infinity;
    let newestTimestamp = 0;

    entries.forEach((entry) => {
      if (entry.timestamp < oldestTimestamp) oldestTimestamp = entry.timestamp;
      if (entry.timestamp > newestTimestamp) newestTimestamp = entry.timestamp;
    });

    return {
      entries: entries.length,
      totalSize,
      totalSizeMB: totalSize / (1024 * 1024),
      oldestEntry: oldestTimestamp !== Infinity ? new Date(oldestTimestamp) : null,
      newestEntry: newestTimestamp !== 0 ? new Date(newestTimestamp) : null,
    };
  }

  /**
   * Get total cache size
   */
  private getTotalCacheSize(): number {
    let total = 0;
    this.cache.forEach((entry) => {
      total += entry.size;
    });
    return total;
  }

  /**
   * Generate filename from URI
   */
  private generateFilename(uri: string): string {
    // Create hash from URI
    let hash = 0;
    for (let i = 0; i < uri.length; i++) {
      const char = uri.charCodeAt(i);
      hash = (hash << 5) - hash + char;
      hash = hash & hash; // Convert to 32-bit integer
    }

    // Get file extension
    const extension = uri.split('.').pop()?.split('?')[0] || 'jpg';

    return `${Math.abs(hash)}.${extension}`;
  }

  /**
   * Get cached image path if exists
   */
  public getCachedPath(uri: string): string | null {
    const cached = this.cache.get(uri);
    return cached ? cached.localPath : null;
  }

  /**
   * Check if image is cached
   */
  public isCached(uri: string): boolean {
    return this.cache.has(uri);
  }
}

export default new ImageOptimizer();
