import { createClient, RedisClientType } from 'redis';
import config from '../config';
import logger from '../utils/logger';
import { CacheOptions } from '../types';

class CacheService {
  private client: RedisClientType;
  private isConnected: boolean = false;

  constructor() {
    this.client = createClient({
      url: config.redis.url,
      password: config.redis.password,
      database: config.redis.db,
    });

    this.client.on('error', (err) => {
      logger.error('Redis Client Error', { error: err.message });
    });

    this.client.on('connect', () => {
      logger.info('Redis connected');
      this.isConnected = true;
    });

    this.client.on('disconnect', () => {
      logger.warn('Redis disconnected');
      this.isConnected = false;
    });
  }

  /**
   * Initialize connection
   */
  async connect(): Promise<void> {
    if (!this.isConnected) {
      await this.client.connect();
    }
  }

  /**
   * Get value from cache
   */
  async get<T = any>(key: string): Promise<T | null> {
    try {
      const value = await this.client.get(key);
      if (!value) return null;

      return JSON.parse(value) as T;
    } catch (error: any) {
      logger.error('Cache get error', { key, error: error.message });
      return null;
    }
  }

  /**
   * Set value in cache with optional TTL
   */
  async set(key: string, value: any, options?: CacheOptions): Promise<void> {
    try {
      const ttl = options?.ttl || 3600; // Default 1 hour
      const prefixedKey = options?.prefix ? `${options.prefix}:${key}` : key;

      await this.client.setEx(
        prefixedKey,
        ttl,
        JSON.stringify(value)
      );
    } catch (error: any) {
      logger.error('Cache set error', { key, error: error.message });
    }
  }

  /**
   * Delete key from cache
   */
  async del(key: string): Promise<void> {
    try {
      await this.client.del(key);
    } catch (error: any) {
      logger.error('Cache delete error', { key, error: error.message });
    }
  }

  /**
   * Delete multiple keys matching pattern
   */
  async delPattern(pattern: string): Promise<void> {
    try {
      const keys = await this.client.keys(pattern);
      if (keys.length > 0) {
        await this.client.del(keys);
      }
    } catch (error: any) {
      logger.error('Cache delete pattern error', { pattern, error: error.message });
    }
  }

  /**
   * Check if key exists
   */
  async exists(key: string): Promise<boolean> {
    try {
      return (await this.client.exists(key)) === 1;
    } catch (error: any) {
      logger.error('Cache exists error', { key, error: error.message });
      return false;
    }
  }

  /**
   * Set expiration time for key
   */
  async expire(key: string, seconds: number): Promise<void> {
    try {
      await this.client.expire(key, seconds);
    } catch (error: any) {
      logger.error('Cache expire error', { key, error: error.message });
    }
  }

  // ============================================================================
  // SPECIFIC CACHE METHODS
  // ============================================================================

  /**
   * Cache user balance
   */
  async cacheBalance(address: string, balance: any): Promise<void> {
    const key = `balance:${address}`;
    await this.set(key, balance, { ttl: config.cache.balanceTTL });
  }

  /**
   * Get cached balance
   */
  async getCachedBalance(address: string): Promise<any> {
    const key = `balance:${address}`;
    return await this.get(key);
  }

  /**
   * Invalidate balance cache
   */
  async invalidateBalance(address: string): Promise<void> {
    const key = `balance:${address}`;
    await this.del(key);
  }

  /**
   * Cache price data
   */
  async cachePrice(asset: string, priceData: any): Promise<void> {
    const key = `price:${asset}`;
    await this.set(key, priceData, { ttl: config.cache.priceTTL });
  }

  /**
   * Get cached price
   */
  async getCachedPrice(asset: string): Promise<any> {
    const key = `price:${asset}`;
    return await this.get(key);
  }

  /**
   * Cache validator list
   */
  async cacheValidators(validators: any[]): Promise<void> {
    const key = 'validators:list';
    await this.set(key, validators, { ttl: config.cache.validatorTTL });
  }

  /**
   * Get cached validators
   */
  async getCachedValidators(): Promise<any[]> {
    const key = 'validators:list';
    return await this.get(key);
  }

  /**
   * Cache proposal
   */
  async cacheProposal(proposalId: number, proposal: any): Promise<void> {
    const key = `proposal:${proposalId}`;
    await this.set(key, proposal, { ttl: config.cache.proposalTTL });
  }

  /**
   * Get cached proposal
   */
  async getCachedProposal(proposalId: number): Promise<any> {
    const key = `proposal:${proposalId}`;
    return await this.get(key);
  }

  /**
   * Cache transaction
   */
  async cacheTransaction(txHash: string, tx: any): Promise<void> {
    const key = `tx:${txHash}`;
    await this.set(key, tx, { ttl: 3600 }); // 1 hour
  }

  /**
   * Get cached transaction
   */
  async getCachedTransaction(txHash: string): Promise<any> {
    const key = `tx:${txHash}`;
    return await this.get(key);
  }

  /**
   * Store rate limit counter
   */
  async incrementRateLimit(identifier: string, windowSeconds: number): Promise<number> {
    try {
      const key = `ratelimit:${identifier}`;
      const current = await this.client.incr(key);

      if (current === 1) {
        await this.client.expire(key, windowSeconds);
      }

      return current;
    } catch (error: any) {
      logger.error('Rate limit increment error', { identifier, error: error.message });
      return 0;
    }
  }

  /**
   * Store session data
   */
  async setSession(sessionId: string, data: any, ttl: number = 86400): Promise<void> {
    const key = `session:${sessionId}`;
    await this.set(key, data, { ttl });
  }

  /**
   * Get session data
   */
  async getSession(sessionId: string): Promise<any> {
    const key = `session:${sessionId}`;
    return await this.get(key);
  }

  /**
   * Delete session
   */
  async deleteSession(sessionId: string): Promise<void> {
    const key = `session:${sessionId}`;
    await this.del(key);
  }

  /**
   * Get cache statistics
   */
  async getStats(): Promise<any> {
    try {
      const info = await this.client.info('stats');
      return {
        connected: this.isConnected,
        info,
      };
    } catch (error: any) {
      logger.error('Failed to get cache stats', { error: error.message });
      return { connected: false };
    }
  }

  /**
   * Close connection
   */
  async disconnect(): Promise<void> {
    await this.client.quit();
    this.isConnected = false;
    logger.info('Redis disconnected');
  }
}

export default new CacheService();
