import axios from 'axios';
import BigNumber from 'bignumber.js';

// Price data for a token
export interface TokenPrice {
  symbol: string;
  usdPrice: string;
  change24h: number;
  lastUpdated: Date;
}

// Price service singleton
class PriceServiceClass {
  private prices: Map<string, TokenPrice> = new Map();
  private isLoading = false;
  private lastUpdated: Date | null = null;
  private updateInterval: NodeJS.Timeout | null = null;
  private listeners: Set<() => void> = new Set();

  // CoinGecko token IDs
  private readonly tokenIds: Record<string, string> = {
    ETH: 'ethereum',
    BNB: 'binancecoin',
    MATIC: 'matic-network',
    SOL: 'solana',
    USDT: 'tether',
    USDC: 'usd-coin',
    wETR: 'etrid', // Custom token - will use estimated price
    ETR: 'etrid',
  };

  // Fallback prices (used when API fails)
  private readonly fallbackPrices: Record<string, string> = {
    ETH: '2500',
    BNB: '300',
    MATIC: '0.80',
    SOL: '100',
    USDT: '1.00',
    USDC: '1.00',
    wETR: '0.15',
    ETR: '0.15',
  };

  // Subscribe to price updates
  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notifyListeners(): void {
    this.listeners.forEach(listener => listener());
  }

  // Get current state
  getState() {
    return {
      prices: new Map(this.prices),
      isLoading: this.isLoading,
      lastUpdated: this.lastUpdated,
    };
  }

  // Start automatic price updates
  startPriceUpdates(intervalMs: number = 60000): void {
    // Initial fetch
    this.refreshPrices();

    // Clear existing interval
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
    }

    // Schedule periodic updates
    this.updateInterval = setInterval(() => {
      this.refreshPrices();
    }, intervalMs);
  }

  // Stop automatic updates
  stopPriceUpdates(): void {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }

  // Refresh all prices
  async refreshPrices(): Promise<void> {
    this.isLoading = true;
    this.notifyListeners();

    try {
      const ids = Object.values(this.tokenIds).filter(
        (id, index, self) => self.indexOf(id) === index,
      );

      const response = await axios.get(
        'https://api.coingecko.com/api/v3/simple/price',
        {
          params: {
            ids: ids.join(','),
            vs_currencies: 'usd',
            include_24hr_change: 'true',
          },
          timeout: 10000,
        },
      );

      const data = response.data;

      // Update prices for each token
      for (const [symbol, coinGeckoId] of Object.entries(this.tokenIds)) {
        const priceData = data[coinGeckoId];
        if (priceData) {
          this.prices.set(symbol, {
            symbol,
            usdPrice: priceData.usd?.toString() || this.fallbackPrices[symbol] || '0',
            change24h: priceData.usd_24h_change || 0,
            lastUpdated: new Date(),
          });
        } else {
          // Use fallback price
          this.prices.set(symbol, {
            symbol,
            usdPrice: this.fallbackPrices[symbol] || '0',
            change24h: 0,
            lastUpdated: new Date(),
          });
        }
      }
    } catch (error) {
      // Use fallback prices on error
      for (const [symbol, price] of Object.entries(this.fallbackPrices)) {
        if (!this.prices.has(symbol)) {
          this.prices.set(symbol, {
            symbol,
            usdPrice: price,
            change24h: 0,
            lastUpdated: new Date(),
          });
        }
      }
    }

    this.isLoading = false;
    this.lastUpdated = new Date();
    this.notifyListeners();
  }

  // Get price for a specific token
  getPrice(symbol: string): string | null {
    const price = this.prices.get(symbol.toUpperCase());
    if (price) {
      return price.usdPrice;
    }
    // Return fallback if available
    return this.fallbackPrices[symbol.toUpperCase()] || null;
  }

  // Get 24h change for a token
  get24hChange(symbol: string): number {
    return this.prices.get(symbol.toUpperCase())?.change24h || 0;
  }

  // Calculate USD value for an amount
  calculateUSDValue(amount: string, symbol: string): string {
    const price = this.getPrice(symbol);
    if (!price) return '0';

    return new BigNumber(amount).multipliedBy(price).toString();
  }

  // Format USD value
  formatUSDValue(amount: string, symbol: string): string {
    const usdValue = this.calculateUSDValue(amount, symbol);
    const bn = new BigNumber(usdValue);

    if (bn.isNaN() || bn.isZero()) {
      return '$0.00';
    }

    return `$${bn.toFormat(2, BigNumber.ROUND_DOWN)}`;
  }

  // Format price with appropriate precision
  formatPrice(price: string | number, decimals: number = 2): string {
    const bn = new BigNumber(price);

    if (bn.isNaN()) {
      return '$0.00';
    }

    if (bn.isLessThan(0.01)) {
      return `$${bn.toFormat(6)}`;
    } else if (bn.isLessThan(1)) {
      return `$${bn.toFormat(4)}`;
    } else {
      return `$${bn.toFormat(decimals)}`;
    }
  }

  // Format 24h change
  format24hChange(symbol: string): string {
    const change = this.get24hChange(symbol);
    const sign = change >= 0 ? '+' : '';
    return `${sign}${change.toFixed(2)}%`;
  }

  // Check if price is up
  isPriceUp(symbol: string): boolean {
    return this.get24hChange(symbol) >= 0;
  }
}

// Export singleton
export const PriceService = new PriceServiceClass();
