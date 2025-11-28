import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * Price pair identifier
 * Format: "BASE/QUOTE" (e.g., "ETR/USD", "BTC/USD", "ETH/ETR")
 */
export type PricePair = string;

/**
 * Oracle data source identifier
 */
export type OracleSource = string;

/**
 * Timestamp in milliseconds
 */
export type Timestamp = number;

/**
 * Price in smallest unit (1e18 precision)
 * Example: 1.5 USD = 1_500_000_000_000_000_000n
 */
export type Price = bigint;

/**
 * Price precision constant (18 decimals)
 */
export const PRICE_PRECISION = 1_000_000_000_000_000_000n;

/**
 * Helper to convert human-readable price to on-chain format
 * @param price - Human-readable price (e.g., 1.5)
 * @returns Price in 1e18 precision
 *
 * @example
 * ```typescript
 * const price = toPrice(42.50); // 42_500_000_000_000_000_000n
 * ```
 */
export function toPrice(price: number): Price {
  return BigInt(Math.floor(price * Number(PRICE_PRECISION)));
}

/**
 * Helper to convert on-chain price to human-readable format
 * @param price - Price in 1e18 precision
 * @returns Human-readable price
 *
 * @example
 * ```typescript
 * const readable = fromPrice(42_500_000_000_000_000_000n); // 42.5
 * ```
 */
export function fromPrice(price: Price): number {
  return Number(price) / Number(PRICE_PRECISION);
}

/**
 * Current price data from oracle
 */
export interface PriceData {
  /** Price pair (e.g., "ETR/USD") */
  pair: PricePair;
  /** Current price in 1e18 precision */
  price: Price;
  /** Price in human-readable format */
  priceFormatted: number;
  /** Timestamp of last update */
  timestamp: Timestamp;
  /** Number of oracle sources reporting */
  sources: number;
  /** Confidence level (0-10000, where 10000 = 100%) */
  confidence: number;
  /** Price deviation percentage (0-10000) */
  deviation: number;
}

/**
 * Time-weighted average price data
 */
export interface TWAPData {
  /** Price pair */
  pair: PricePair;
  /** TWAP value in 1e18 precision */
  twap: Price;
  /** TWAP in human-readable format */
  twapFormatted: number;
  /** Start timestamp of TWAP period */
  startTime: Timestamp;
  /** End timestamp of TWAP period */
  endTime: Timestamp;
  /** Number of data points used */
  dataPoints: number;
  /** Minimum price in period */
  minPrice: Price;
  /** Maximum price in period */
  maxPrice: Price;
  /** Standard deviation */
  stdDeviation: number;
}

/**
 * Oracle data source information
 */
export interface OracleSourceInfo {
  /** Source identifier */
  id: OracleSource;
  /** Source name (e.g., "Chainlink", "Band Protocol") */
  name: string;
  /** Source endpoint/address */
  endpoint: string;
  /** Current status */
  status: OracleSourceStatus;
  /** Last successful update timestamp */
  lastUpdate: Timestamp;
  /** Number of successful updates */
  successfulUpdates: number;
  /** Number of failed updates */
  failedUpdates: number;
  /** Uptime percentage (0-10000) */
  uptime: number;
  /** Weight in aggregation (0-10000) */
  weight: number;
  /** Supported price pairs */
  supportedPairs: PricePair[];
}

/**
 * Oracle source status
 */
export enum OracleSourceStatus {
  Active = 'Active',
  Inactive = 'Inactive',
  Degraded = 'Degraded',
  Offline = 'Offline',
}

/**
 * Price update event data
 */
export interface PriceUpdateEvent {
  /** Price pair */
  pair: PricePair;
  /** New price */
  price: Price;
  /** Previous price */
  previousPrice: Price;
  /** Price change percentage */
  changePercent: number;
  /** Timestamp */
  timestamp: Timestamp;
  /** Oracle sources that reported this update */
  sources: OracleSource[];
}

/**
 * Historical price data point
 */
export interface HistoricalPrice {
  /** Price pair */
  pair: PricePair;
  /** Price at this timestamp */
  price: Price;
  /** Timestamp */
  timestamp: Timestamp;
  /** Volume (if available) */
  volume?: bigint;
}

/**
 * Price feed subscription callback
 */
export type PriceUpdateCallback = (event: PriceUpdateEvent) => void;

/**
 * Subscription handle for unsubscribing
 */
export interface PriceSubscription {
  /** Unsubscribe from price updates */
  unsubscribe: () => void;
  /** Check if subscription is active */
  isActive: () => boolean;
}

/**
 * TWAP calculation parameters
 */
export interface TWAPParams {
  /** Price pair */
  pair: PricePair;
  /** Start timestamp (default: 24 hours ago) */
  startTime?: Timestamp;
  /** End timestamp (default: now) */
  endTime?: Timestamp;
  /** Minimum number of data points required */
  minDataPoints?: number;
}

/**
 * Price aggregation method
 */
export enum AggregationMethod {
  /** Simple average of all sources */
  Mean = 'Mean',
  /** Weighted average based on source weights */
  WeightedMean = 'WeightedMean',
  /** Median of all sources */
  Median = 'Median',
  /** Volume-weighted average */
  VWAP = 'VWAP',
}

/**
 * Custom errors for Oracle operations
 */
export class OracleError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'OracleError';
  }
}

export class PriceNotFoundError extends OracleError {
  constructor(pair: PricePair) {
    super(`Price not found for pair: ${pair}`);
    this.name = 'PriceNotFoundError';
  }
}

export class InsufficientDataError extends OracleError {
  constructor(message: string) {
    super(message);
    this.name = 'InsufficientDataError';
  }
}

export class OracleSourceError extends OracleError {
  constructor(source: OracleSource, message: string) {
    super(`Oracle source ${source}: ${message}`);
    this.name = 'OracleSourceError';
  }
}

/**
 * OracleWrapper
 *
 * Wrapper for the pallet-reserve-oracle module.
 *
 * Provides decentralized price feeds from multiple oracle sources,
 * aggregating data to provide reliable, manipulation-resistant prices
 * for DeFi applications on Ëtrid.
 *
 * **Features:**
 * - Multi-source price aggregation
 * - Time-weighted average price (TWAP) calculations
 * - Real-time price update subscriptions
 * - Oracle source health monitoring
 * - Historical price data queries
 * - Confidence and deviation metrics
 *
 * **Supported Price Pairs:**
 * - Crypto: ETR/USD, BTC/USD, ETH/USD, SOL/USD, etc.
 * - Cross-rates: ETH/ETR, BTC/ETR, etc.
 * - Stablecoins: USDT/USD, USDC/USD, DAI/USD
 * - Traditional assets (via oracle feeds)
 *
 * @example
 * ```typescript
 * import { ApiPromise } from '@polkadot/api';
 * import { OracleWrapper, fromPrice } from '@etrid/sdk';
 *
 * const api = await ApiPromise.create({ provider });
 * const oracle = new OracleWrapper(api);
 *
 * // Get current price
 * const priceData = await oracle.getPrice('ETR/USD');
 * console.log(`ETR price: $${priceData.priceFormatted}`);
 * console.log(`Confidence: ${priceData.confidence / 100}%`);
 *
 * // Get 24-hour TWAP
 * const twap = await oracle.getTWAP({ pair: 'ETR/USD' });
 * console.log(`24h TWAP: $${twap.twapFormatted}`);
 *
 * // Subscribe to price updates
 * const subscription = await oracle.subscribePriceUpdates(
 *   'ETR/USD',
 *   (event) => {
 *     console.log(`Price updated: $${fromPrice(event.price)}`);
 *     console.log(`Change: ${event.changePercent}%`);
 *   }
 * );
 *
 * // Check oracle sources
 * const sources = await oracle.getPriceSources('ETR/USD');
 * sources.forEach(source => {
 *   console.log(`${source.name}: ${source.status}, uptime ${source.uptime / 100}%`);
 * });
 *
 * // Later: unsubscribe
 * subscription.unsubscribe();
 * ```
 */
export class OracleWrapper {
  /**
   * Creates an OracleWrapper instance
   * @param api - Connected Polkadot.js API instance
   */
  constructor(private api: ApiPromise) {}

  /**
   * Get current price for a price pair
   *
   * Retrieves the latest aggregated price from all active oracle sources.
   *
   * @param pair - Price pair (e.g., "ETR/USD", "BTC/USD")
   * @returns Current price data with confidence metrics
   * @throws {PriceNotFoundError} If price pair is not supported
   * @throws {InsufficientDataError} If not enough oracle sources are available
   *
   * @example
   * ```typescript
   * const priceData = await oracle.getPrice('ETR/USD');
   * console.log(`Price: $${priceData.priceFormatted}`);
   * console.log(`Confidence: ${priceData.confidence / 100}%`);
   * console.log(`Last update: ${new Date(priceData.timestamp)}`);
   * ```
   */
  async getPrice(pair: PricePair): Promise<PriceData> {
    try {
      const priceInfo = await this.api.query.reserveOracle.prices(pair);

      if (priceInfo.isNone) {
        throw new PriceNotFoundError(pair);
      }

      const data = priceInfo.unwrap();
      const price = BigInt(data.price.toString());
      const timestamp = Number(data.timestamp.toString());
      const sources = Number(data.sources.toString());
      const confidence = Number(data.confidence.toString());
      const deviation = Number(data.deviation.toString());

      if (sources === 0) {
        throw new InsufficientDataError(`No oracle sources available for ${pair}`);
      }

      return {
        pair,
        price,
        priceFormatted: fromPrice(price),
        timestamp,
        sources,
        confidence,
        deviation,
      };
    } catch (error) {
      if (error instanceof OracleError) {
        throw error;
      }
      throw new OracleError(`Failed to get price for ${pair}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get time-weighted average price (TWAP)
   *
   * Calculates TWAP over a specified time period, providing a more
   * manipulation-resistant price metric than spot prices.
   *
   * @param params - TWAP calculation parameters
   * @returns TWAP data with statistics
   * @throws {PriceNotFoundError} If price pair is not supported
   * @throws {InsufficientDataError} If not enough historical data
   *
   * @example
   * ```typescript
   * // Default: 24-hour TWAP
   * const twap24h = await oracle.getTWAP({ pair: 'ETR/USD' });
   *
   * // Custom time range: last hour
   * const now = Date.now();
   * const oneHourAgo = now - 3600000;
   * const twap1h = await oracle.getTWAP({
   *   pair: 'ETR/USD',
   *   startTime: oneHourAgo,
   *   endTime: now,
   *   minDataPoints: 10
   * });
   *
   * console.log(`1h TWAP: $${twap1h.twapFormatted}`);
   * console.log(`Min: $${fromPrice(twap1h.minPrice)}`);
   * console.log(`Max: $${fromPrice(twap1h.maxPrice)}`);
   * ```
   */
  async getTWAP(params: TWAPParams): Promise<TWAPData> {
    const now = Date.now();
    const endTime = params.endTime || now;
    const startTime = params.startTime || (endTime - 24 * 60 * 60 * 1000); // Default: 24 hours
    const minDataPoints = params.minDataPoints || 10;

    try {
      const twapInfo = await this.api.query.reserveOracle.twap(
        params.pair,
        startTime,
        endTime
      );

      if (twapInfo.isNone) {
        throw new PriceNotFoundError(params.pair);
      }

      const data = twapInfo.unwrap();
      const twap = BigInt(data.twap.toString());
      const dataPoints = Number(data.dataPoints.toString());
      const minPrice = BigInt(data.minPrice.toString());
      const maxPrice = BigInt(data.maxPrice.toString());
      const stdDeviation = Number(data.stdDeviation.toString()) / 10000; // Convert from basis points

      if (dataPoints < minDataPoints) {
        throw new InsufficientDataError(
          `Insufficient data points: ${dataPoints} < ${minDataPoints}`
        );
      }

      return {
        pair: params.pair,
        twap,
        twapFormatted: fromPrice(twap),
        startTime,
        endTime,
        dataPoints,
        minPrice,
        maxPrice,
        stdDeviation,
      };
    } catch (error) {
      if (error instanceof OracleError) {
        throw error;
      }
      throw new OracleError(`Failed to calculate TWAP for ${params.pair}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get oracle sources for a price pair
   *
   * Retrieves information about all oracle sources providing data
   * for the specified price pair.
   *
   * @param pair - Price pair
   * @returns Array of oracle source information
   * @throws {PriceNotFoundError} If price pair is not supported
   *
   * @example
   * ```typescript
   * const sources = await oracle.getPriceSources('ETR/USD');
   *
   * sources.forEach(source => {
   *   console.log(`Source: ${source.name}`);
   *   console.log(`  Status: ${source.status}`);
   *   console.log(`  Uptime: ${source.uptime / 100}%`);
   *   console.log(`  Weight: ${source.weight / 100}%`);
   *   console.log(`  Success rate: ${
   *     (source.successfulUpdates / (source.successfulUpdates + source.failedUpdates) * 100).toFixed(2)
   *   }%`);
   * });
   * ```
   */
  async getPriceSources(pair: PricePair): Promise<OracleSourceInfo[]> {
    try {
      const sourcesData = await this.api.query.reserveOracle.pairSources(pair);

      if (sourcesData.isNone) {
        throw new PriceNotFoundError(pair);
      }

      const sources = sourcesData.unwrap();
      const sourceInfos: OracleSourceInfo[] = [];

      for (const sourceId of sources) {
        const sourceInfo = await this.api.query.reserveOracle.sources(sourceId.toString());

        if (sourceInfo.isSome) {
          const info = sourceInfo.unwrap();
          sourceInfos.push({
            id: sourceId.toString(),
            name: info.name.toString(),
            endpoint: info.endpoint.toString(),
            status: this.parseSourceStatus(info.status),
            lastUpdate: Number(info.lastUpdate.toString()),
            successfulUpdates: Number(info.successfulUpdates.toString()),
            failedUpdates: Number(info.failedUpdates.toString()),
            uptime: Number(info.uptime.toString()),
            weight: Number(info.weight.toString()),
            supportedPairs: info.supportedPairs.map((p: any) => p.toString()),
          });
        }
      }

      return sourceInfos;
    } catch (error) {
      if (error instanceof OracleError) {
        throw error;
      }
      throw new OracleError(`Failed to get sources for ${pair}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Subscribe to real-time price updates
   *
   * Creates a subscription that calls the provided callback whenever
   * the price for the specified pair is updated.
   *
   * @param pair - Price pair to monitor
   * @param callback - Function to call on price updates
   * @returns Subscription handle for unsubscribing
   *
   * @example
   * ```typescript
   * const subscription = await oracle.subscribePriceUpdates(
   *   'ETR/USD',
   *   (event) => {
   *     console.log(`New price: $${fromPrice(event.price)}`);
   *     console.log(`Previous: $${fromPrice(event.previousPrice)}`);
   *     console.log(`Change: ${event.changePercent.toFixed(2)}%`);
   *     console.log(`Sources: ${event.sources.join(', ')}`);
   *   }
   * );
   *
   * // Later, when done:
   * subscription.unsubscribe();
   * ```
   */
  async subscribePriceUpdates(
    pair: PricePair,
    callback: PriceUpdateCallback
  ): Promise<PriceSubscription> {
    let isActive = true;
    let previousPrice: Price | null = null;

    // Get initial price
    try {
      const initialData = await this.getPrice(pair);
      previousPrice = initialData.price;
    } catch (error) {
      // Ignore if price doesn't exist yet
    }

    const unsubscribe = await this.api.query.reserveOracle.prices(
      pair,
      (priceInfo: any) => {
        if (!isActive) return;

        if (priceInfo.isSome) {
          const data = priceInfo.unwrap();
          const price = BigInt(data.price.toString());
          const timestamp = Number(data.timestamp.toString());
          const sources = data.sources.map((s: any) => s.toString());

          let changePercent = 0;
          if (previousPrice !== null && previousPrice > 0n) {
            const change = Number(price - previousPrice);
            const prev = Number(previousPrice);
            changePercent = (change / prev) * 100;
          }

          const event: PriceUpdateEvent = {
            pair,
            price,
            previousPrice: previousPrice || 0n,
            changePercent,
            timestamp,
            sources,
          };

          callback(event);
          previousPrice = price;
        }
      }
    );

    return {
      unsubscribe: () => {
        isActive = false;
        unsubscribe();
      },
      isActive: () => isActive,
    };
  }

  /**
   * Get historical price data
   *
   * Retrieves historical price points for analysis and charting.
   *
   * @param pair - Price pair
   * @param startTime - Start timestamp
   * @param endTime - End timestamp (default: now)
   * @param interval - Time interval between data points in ms (default: 1 hour)
   * @returns Array of historical price points
   *
   * @example
   * ```typescript
   * const now = Date.now();
   * const weekAgo = now - 7 * 24 * 60 * 60 * 1000;
   *
   * const history = await oracle.getHistoricalPrices(
   *   'ETR/USD',
   *   weekAgo,
   *   now,
   *   3600000 // 1 hour intervals
   * );
   *
   * history.forEach(point => {
   *   console.log(`${new Date(point.timestamp)}: $${fromPrice(point.price)}`);
   * });
   * ```
   */
  async getHistoricalPrices(
    pair: PricePair,
    startTime: Timestamp,
    endTime: Timestamp = Date.now(),
    interval: number = 3600000 // 1 hour default
  ): Promise<HistoricalPrice[]> {
    try {
      const historyData = await this.api.query.reserveOracle.priceHistory(
        pair,
        startTime,
        endTime,
        interval
      );

      if (historyData.isNone) {
        return [];
      }

      const history = historyData.unwrap();
      return history.map((point: any) => ({
        pair,
        price: BigInt(point.price.toString()),
        timestamp: Number(point.timestamp.toString()),
        volume: point.volume ? BigInt(point.volume.toString()) : undefined,
      }));
    } catch (error) {
      throw new OracleError(`Failed to get historical prices for ${pair}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get all supported price pairs
   *
   * @returns Array of supported price pair identifiers
   *
   * @example
   * ```typescript
   * const pairs = await oracle.getSupportedPairs();
   * console.log(`Supported pairs: ${pairs.join(', ')}`);
   * ```
   */
  async getSupportedPairs(): Promise<PricePair[]> {
    try {
      const pairs = await this.api.query.reserveOracle.supportedPairs();
      return pairs.map((p: any) => p.toString());
    } catch (error) {
      throw new OracleError(`Failed to get supported pairs: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Check if a price pair is supported
   *
   * @param pair - Price pair to check
   * @returns True if pair is supported
   *
   * @example
   * ```typescript
   * if (await oracle.isPairSupported('ETR/USD')) {
   *   console.log('ETR/USD is supported');
   * }
   * ```
   */
  async isPairSupported(pair: PricePair): Promise<boolean> {
    try {
      const priceInfo = await this.api.query.reserveOracle.prices(pair);
      return priceInfo.isSome;
    } catch (error) {
      return false;
    }
  }

  /**
   * Parse oracle source status from chain data
   * @private
   */
  private parseSourceStatus(status: any): OracleSourceStatus {
    const statusStr = status.toString();
    if (statusStr === 'Active') return OracleSourceStatus.Active;
    if (statusStr === 'Inactive') return OracleSourceStatus.Inactive;
    if (statusStr === 'Degraded') return OracleSourceStatus.Degraded;
    if (statusStr === 'Offline') return OracleSourceStatus.Offline;
    return OracleSourceStatus.Offline;
  }
}
