import AsyncStorage from '@react-native-async-storage/async-storage';

interface CacheEntry<T> {
  data: T;
  timestamp: number;
  ttl: number; // Time to live in milliseconds
}

/**
 * Cache manager for storing and retrieving data with TTL
 */
class CacheManager {
  private memoryCache: Map<string, CacheEntry<any>> = new Map();

  /**
   * Set cache entry
   */
  public async set<T>(
    key: string,
    data: T,
    ttl: number = 300000 // 5 minutes default
  ): Promise<void> {
    const entry: CacheEntry<T> = {
      data,
      timestamp: Date.now(),
      ttl,
    };

    // Store in memory cache
    this.memoryCache.set(key, entry);

    // Store in AsyncStorage for persistence
    try {
      await AsyncStorage.setItem(`cache_${key}`, JSON.stringify(entry));
    } catch (error) {
      console.error('Failed to save to AsyncStorage:', error);
    }
  }

  /**
   * Get cache entry
   */
  public async get<T>(key: string): Promise<T | null> {
    // Check memory cache first
    const memEntry = this.memoryCache.get(key);
    if (memEntry && this.isValid(memEntry)) {
      return memEntry.data as T;
    }

    // Check AsyncStorage
    try {
      const stored = await AsyncStorage.getItem(`cache_${key}`);
      if (stored) {
        const entry: CacheEntry<T> = JSON.parse(stored);
        if (this.isValid(entry)) {
          // Restore to memory cache
          this.memoryCache.set(key, entry);
          return entry.data;
        } else {
          // Expired, remove it
          await this.remove(key);
        }
      }
    } catch (error) {
      console.error('Failed to read from AsyncStorage:', error);
    }

    return null;
  }

  /**
   * Check if cache entry exists and is valid
   */
  public async has(key: string): Promise<boolean> {
    const data = await this.get(key);
    return data !== null;
  }

  /**
   * Remove cache entry
   */
  public async remove(key: string): Promise<void> {
    this.memoryCache.delete(key);
    try {
      await AsyncStorage.removeItem(`cache_${key}`);
    } catch (error) {
      console.error('Failed to remove from AsyncStorage:', error);
    }
  }

  /**
   * Clear all cache
   */
  public async clear(): Promise<void> {
    this.memoryCache.clear();
    try {
      const keys = await AsyncStorage.getAllKeys();
      const cacheKeys = keys.filter((key) => key.startsWith('cache_'));
      await AsyncStorage.multiRemove(cacheKeys);
    } catch (error) {
      console.error('Failed to clear AsyncStorage cache:', error);
    }
  }

  /**
   * Invalidate cache by pattern
   */
  public async invalidatePattern(pattern: string): Promise<void> {
    // Clear from memory cache
    const keysToDelete: string[] = [];
    this.memoryCache.forEach((_, key) => {
      if (key.includes(pattern)) {
        keysToDelete.push(key);
      }
    });
    keysToDelete.forEach((key) => this.memoryCache.delete(key));

    // Clear from AsyncStorage
    try {
      const keys = await AsyncStorage.getAllKeys();
      const matchingKeys = keys.filter(
        (key) => key.startsWith('cache_') && key.includes(pattern)
      );
      await AsyncStorage.multiRemove(matchingKeys);
    } catch (error) {
      console.error('Failed to invalidate pattern in AsyncStorage:', error);
    }
  }

  /**
   * Get or fetch data with caching
   */
  public async getOrFetch<T>(
    key: string,
    fetchFn: () => Promise<T>,
    ttl: number = 300000
  ): Promise<T> {
    // Try to get from cache
    const cached = await this.get<T>(key);
    if (cached !== null) {
      return cached;
    }

    // Fetch fresh data
    const data = await fetchFn();

    // Cache it
    await this.set(key, data, ttl);

    return data;
  }

  /**
   * Clean up expired entries
   */
  public async cleanup(): Promise<void> {
    // Clean memory cache
    const expiredKeys: string[] = [];
    this.memoryCache.forEach((entry, key) => {
      if (!this.isValid(entry)) {
        expiredKeys.push(key);
      }
    });
    expiredKeys.forEach((key) => this.memoryCache.delete(key));

    // Clean AsyncStorage
    try {
      const keys = await AsyncStorage.getAllKeys();
      const cacheKeys = keys.filter((key) => key.startsWith('cache_'));

      for (const key of cacheKeys) {
        const stored = await AsyncStorage.getItem(key);
        if (stored) {
          const entry: CacheEntry<any> = JSON.parse(stored);
          if (!this.isValid(entry)) {
            await AsyncStorage.removeItem(key);
          }
        }
      }
    } catch (error) {
      console.error('Failed to cleanup AsyncStorage:', error);
    }
  }

  /**
   * Check if cache entry is still valid
   */
  private isValid(entry: CacheEntry<any>): boolean {
    return Date.now() - entry.timestamp < entry.ttl;
  }

  /**
   * Get cache statistics
   */
  public getStats(): {
    memoryEntries: number;
    memorySize: number;
  } {
    let memorySize = 0;
    this.memoryCache.forEach((entry) => {
      memorySize += JSON.stringify(entry.data).length;
    });

    return {
      memoryEntries: this.memoryCache.size,
      memorySize,
    };
  }
}

// Predefined cache keys and TTLs
export const CacheKeys = {
  BALANCE: 'balance',
  PRICE_FEED: 'price_feed',
  TRANSACTION_HISTORY: 'tx_history',
  VALIDATOR_LIST: 'validator_list',
  GPU_LIST: 'gpu_list',
  GOVERNANCE_PROPOSALS: 'governance_proposals',
  STAKING_INFO: 'staking_info',
  BRIDGE_HISTORY: 'bridge_history',
  ETH_PBC_BALANCE: 'eth_pbc_balance',
};

export const CacheTTLs = {
  BALANCE: 300000, // 5 minutes
  PRICE_FEED: 60000, // 1 minute
  TRANSACTION_HISTORY: 600000, // 10 minutes
  VALIDATOR_LIST: 3600000, // 1 hour
  GPU_LIST: 300000, // 5 minutes
  GOVERNANCE_PROPOSALS: 600000, // 10 minutes
  STAKING_INFO: 300000, // 5 minutes
  BRIDGE_HISTORY: 600000, // 10 minutes
  ETH_PBC_BALANCE: 300000, // 5 minutes
};

export default new CacheManager();
