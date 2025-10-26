/**
 * Hyperliquid API Adapter
 * 
 * Integrates ÉTR and EDSC with Hyperliquid's hybrid DEX/CEX platform.
 * Provides REST API wrapper for market creation, liquidity, and trading.
 */

import axios, { AxiosInstance } from 'axios';
import { ethers } from 'ethers';

interface HyperliquidConfig {
  apiKey: string;
  apiSecret: string;
  baseUrl: string;
  ethBridgeAddress: string;
  privateKey: string;
}

interface MarketParams {
  symbol: string;          // e.g., "ETR/USDC"
  type: 'spot' | 'perp';
  minOrderSize: string;
  tickSize: string;
  bridge: string;          // Ethereum bridge address
}

interface OrderbookSnapshot {
  bids: [string, string][]; // [price, size]
  asks: [string, string][];
  timestamp: number;
}

class HyperliquidAdapter {
  private client: AxiosInstance;
  private wallet: ethers.Wallet;

  constructor(private config: HyperliquidConfig) {
    this.client = axios.create({
      baseURL: config.baseUrl,
      headers: {
        'X-API-KEY': config.apiKey,
        'Content-Type': 'application/json'
      }
    });

    this.wallet = new ethers.Wallet(config.privateKey);
  }

  /**
   * Create ÉTR spot market on Hyperliquid
   */
  async createETRMarket(): Promise<string> {
    console.log('📊 Creating ETR/USDC market on Hyperliquid...');

    const params: MarketParams = {
      symbol: 'ETR/USDC',
      type: 'spot',
      minOrderSize: '10',        // 10 ÉTR minimum
      tickSize: '0.0001',        // $0.0001 price increment
      bridge: this.config.ethBridgeAddress
    };

    try {
      const response = await this.client.post('/v1/markets', params);
      const marketId = response.data.marketId;

      console.log(`✅ Market created: ${marketId}`);
      return marketId;

    } catch (error: any) {
      console.error('❌ Market creation failed:', error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Create EDSC stablecoin market
   */
  async createEDSCMarket(): Promise<string> {
    console.log('📊 Creating EDSC/USDC market on Hyperliquid...');

    const params: MarketParams = {
      symbol: 'EDSC/USDC',
      type: 'spot',
      minOrderSize: '100',       // 100 EDSC minimum
      tickSize: '0.0001',        // Tight spread for stablecoin
      bridge: this.config.ethBridgeAddress
    };

    try {
      const response = await this.client.post('/v1/markets', params);
      const marketId = response.data.marketId;

      console.log(`✅ Market created: ${marketId}`);
      return marketId;

    } catch (error: any) {
      console.error('❌ Market creation failed:', error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Get current orderbook
   */
  async getOrderbook(symbol: string): Promise<OrderbookSnapshot> {
    try {
      const response = await this.client.get(`/v1/orderbook/${symbol}`);
      return response.data as OrderbookSnapshot;

    } catch (error: any) {
      console.error('❌ Orderbook fetch failed:', error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Place limit order
   */
  async placeLimitOrder(params: {
    symbol: string;
    side: 'buy' | 'sell';
    price: string;
    size: string;
  }): Promise<string> {
    console.log(`📝 Placing ${params.side} order: ${params.size} ${params.symbol} @ ${params.price}`);

    // Sign order with wallet
    const timestamp = Date.now();
    const message = JSON.stringify({
      symbol: params.symbol,
      side: params.side,
      price: params.price,
      size: params.size,
      timestamp
    });

    const signature = await this.wallet.signMessage(message);

    try {
      const response = await this.client.post('/v1/orders', {
        ...params,
        timestamp,
        signature
      });

      const orderId = response.data.orderId;
      console.log(`✅ Order placed: ${orderId}`);
      return orderId;

    } catch (error: any) {
      console.error('❌ Order placement failed:', error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Get market statistics
   */
  async getMarketStats(symbol: string): Promise<{
    volume24h: string;
    high24h: string;
    low24h: string;
    lastPrice: string;
  }> {
    try {
      const response = await this.client.get(`/v1/markets/${symbol}/stats`);
      return response.data;

    } catch (error: any) {
      console.error('❌ Stats fetch failed:', error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Deposit collateral to Hyperliquid
   */
  async depositCollateral(amount: string): Promise<string> {
    console.log(`💰 Depositing ${amount} USDC as collateral...`);

    // This would interact with Hyperliquid's bridge contract
    // Simplified for demo
    const timestamp = Date.now();
    const message = JSON.stringify({
      action: 'deposit',
      amount,
      timestamp
    });

    const signature = await this.wallet.signMessage(message);

    try {
      const response = await this.client.post('/v1/deposits', {
        amount,
        timestamp,
        signature
      });

      const depositId = response.data.depositId;
      console.log(`✅ Deposit confirmed: ${depositId}`);
      return depositId;

    } catch (error: any) {
      console.error('❌ Deposit failed:', error.response?.data || error.message);
      throw error;
    }
  }
}

export default HyperliquidAdapter;

// CLI entry point
if (require.main === module) {
  const config: HyperliquidConfig = {
    apiKey: process.env.HYPERLIQUID_API_KEY || '',
    apiSecret: process.env.HYPERLIQUID_API_SECRET || '',
    baseUrl: process.env.HYPERLIQUID_BASE_URL || 'https://api.hyperliquid.xyz',
    ethBridgeAddress: process.env.ETH_BRIDGE_ADDRESS || '',
    privateKey: process.env.BRIDGE_PRIVATE_KEY || ''
  };

  const adapter = new HyperliquidAdapter(config);

  (async () => {
    try {
      // Create markets
      await adapter.createETRMarket();
      await adapter.createEDSCMarket();

      // Deposit initial collateral
      await adapter.depositCollateral('500000'); // $500k USDC

      // Get initial stats
      const etrStats = await adapter.getMarketStats('ETR/USDC');
      console.log('ETR Stats:', etrStats);

      const edscStats = await adapter.getMarketStats('EDSC/USDC');
      console.log('EDSC Stats:', edscStats);

    } catch (error) {
      console.error('❌ Hyperliquid adapter error:', error);
      process.exit(1);
    }
  })();
}
