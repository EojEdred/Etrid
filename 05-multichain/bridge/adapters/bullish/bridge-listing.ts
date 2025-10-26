/**
 * BullEx Bridge Listing Adapter
 *
 * Automates token listing on BullEx multi-chain DEX.
 * Supports Ethereum, BSC, and Solana listings with bridge integration.
 */

import axios, { AxiosInstance } from 'axios';
import { ethers } from 'ethers';

interface BullExConfig {
  apiKey: string;
  apiUrl: string;
  bridgeAddress: {
    ethereum: string;
    bsc: string;
    solana: string;
  };
  tokens: {
    etr: {
      ethereum: string;
      bsc: string;
      solana: string;
    };
    edsc: {
      ethereum: string;
      bsc: string;
      solana: string;
    };
  };
}

interface ListingRequest {
  token: string;
  chain: 'ethereum' | 'bsc' | 'solana';
  bridge: string;
  metadata: {
    name: string;
    symbol: string;
    decimals: number;
    logoUrl: string;
    website: string;
  };
}

interface PoolConfig {
  tokenA: string;
  tokenB: string;
  chain: string;
  feeLevel: string;  // "low" | "medium" | "high"
  initialLiquidity: {
    tokenA: string;
    tokenB: string;
  };
}

class BullExAdapter {
  private client: AxiosInstance;

  constructor(private config: BullExConfig) {
    this.client = axios.create({
      baseURL: config.apiUrl,
      headers: {
        'X-API-KEY': config.apiKey,
        'Content-Type': 'application/json'
      }
    });
  }

  /**
   * List ÉTR on all supported chains
   */
  async listETR(): Promise<{
    ethereum: string;
    bsc: string;
    solana: string;
  }> {
    console.log('📋 Listing ÉTR on BullEx (all chains)...');

    const listings = await Promise.all([
      this.listToken({
        token: this.config.tokens.etr.ethereum,
        chain: 'ethereum',
        bridge: this.config.bridgeAddress.ethereum,
        metadata: {
          name: 'Etrid Coin (Ethereum)',
          symbol: 'ÉTR',
          decimals: 18,
          logoUrl: 'https://etrid.com/assets/etr-logo.png',
          website: 'https://etrid.com'
        }
      }),

      this.listToken({
        token: this.config.tokens.etr.bsc,
        chain: 'bsc',
        bridge: this.config.bridgeAddress.bsc,
        metadata: {
          name: 'Etrid Coin (BSC)',
          symbol: 'ÉTR',
          decimals: 18,
          logoUrl: 'https://etrid.com/assets/etr-logo.png',
          website: 'https://etrid.com'
        }
      }),

      this.listToken({
        token: this.config.tokens.etr.solana,
        chain: 'solana',
        bridge: this.config.bridgeAddress.solana,
        metadata: {
          name: 'Etrid Coin (Solana)',
          symbol: 'ÉTR',
          decimals: 9,  // SPL tokens use 9 decimals
          logoUrl: 'https://etrid.com/assets/etr-logo.png',
          website: 'https://etrid.com'
        }
      })
    ]);

    const [ethListingId, bscListingId, solListingId] = listings;

    console.log('✅ ÉTR listed on all chains:');
    console.log(`   Ethereum: ${ethListingId}`);
    console.log(`   BSC: ${bscListingId}`);
    console.log(`   Solana: ${solListingId}`);

    return {
      ethereum: ethListingId,
      bsc: bscListingId,
      solana: solListingId
    };
  }

  /**
   * List EDSC on all supported chains
   */
  async listEDSC(): Promise<{
    ethereum: string;
    bsc: string;
    solana: string;
  }> {
    console.log('📋 Listing EDSC on BullEx (all chains)...');

    const listings = await Promise.all([
      this.listToken({
        token: this.config.tokens.edsc.ethereum,
        chain: 'ethereum',
        bridge: this.config.bridgeAddress.ethereum,
        metadata: {
          name: 'Etrid Dollar Stablecoin (Ethereum)',
          symbol: 'EDSC',
          decimals: 18,
          logoUrl: 'https://etrid.com/assets/edsc-logo.png',
          website: 'https://etrid.com'
        }
      }),

      this.listToken({
        token: this.config.tokens.edsc.bsc,
        chain: 'bsc',
        bridge: this.config.bridgeAddress.bsc,
        metadata: {
          name: 'Etrid Dollar Stablecoin (BSC)',
          symbol: 'EDSC',
          decimals: 18,
          logoUrl: 'https://etrid.com/assets/edsc-logo.png',
          website: 'https://etrid.com'
        }
      }),

      this.listToken({
        token: this.config.tokens.edsc.solana,
        chain: 'solana',
        bridge: this.config.bridgeAddress.solana,
        metadata: {
          name: 'Etrid Dollar Stablecoin (Solana)',
          symbol: 'EDSC',
          decimals: 9,
          logoUrl: 'https://etrid.com/assets/edsc-logo.png',
          website: 'https://etrid.com'
        }
      })
    ]);

    const [ethListingId, bscListingId, solListingId] = listings;

    console.log('✅ EDSC listed on all chains:');
    console.log(`   Ethereum: ${ethListingId}`);
    console.log(`   BSC: ${bscListingId}`);
    console.log(`   Solana: ${solListingId}`);

    return {
      ethereum: ethListingId,
      bsc: bscListingId,
      solana: solListingId
    };
  }

  /**
   * List a single token on BullEx
   */
  private async listToken(request: ListingRequest): Promise<string> {
    try {
      const response = await this.client.post('/v1/listing', request);

      if (response.data.status === 'success') {
        return response.data.listingId;
      } else {
        throw new Error(`Listing failed: ${response.data.error}`);
      }

    } catch (error: any) {
      console.error(`❌ Listing failed on ${request.chain}:`, error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Create liquidity pools on BullEx
   */
  async createPools(): Promise<void> {
    console.log('💧 Creating liquidity pools on BullEx...');

    const pools: PoolConfig[] = [
      // Ethereum pools
      {
        tokenA: this.config.tokens.etr.ethereum,
        tokenB: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',  // WETH
        chain: 'ethereum',
        feeLevel: 'medium',  // 0.3%
        initialLiquidity: {
          tokenA: '1000000',  // 1M ÉTR
          tokenB: '100'       // 100 ETH
        }
      },
      {
        tokenA: this.config.tokens.edsc.ethereum,
        tokenB: '0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48',  // USDC
        chain: 'ethereum',
        feeLevel: 'low',  // 0.05% (stablecoin)
        initialLiquidity: {
          tokenA: '500000',   // 500k EDSC
          tokenB: '500000'    // 500k USDC
        }
      },

      // BSC pools
      {
        tokenA: this.config.tokens.etr.bsc,
        tokenB: '0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c',  // WBNB
        chain: 'bsc',
        feeLevel: 'medium',
        initialLiquidity: {
          tokenA: '500000',   // 500k ÉTR
          tokenB: '50'        // 50 BNB
        }
      },
      {
        tokenA: this.config.tokens.edsc.bsc,
        tokenB: '0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56',  // BUSD
        chain: 'bsc',
        feeLevel: 'low',
        initialLiquidity: {
          tokenA: '500000',   // 500k EDSC
          tokenB: '500000'    // 500k BUSD
        }
      },

      // Solana pools
      {
        tokenA: this.config.tokens.etr.solana,
        tokenB: 'So11111111111111111111111111111111111111112',  // WSOL
        chain: 'solana',
        feeLevel: 'medium',
        initialLiquidity: {
          tokenA: '1000000',  // 1M ÉTR
          tokenB: '500'       // 500 SOL
        }
      },
      {
        tokenA: this.config.tokens.edsc.solana,
        tokenB: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',  // USDC (SPL)
        chain: 'solana',
        feeLevel: 'low',
        initialLiquidity: {
          tokenA: '1000000',  // 1M EDSC
          tokenB: '1000000'   // 1M USDC
        }
      }
    ];

    for (const pool of pools) {
      await this.createPool(pool);
    }

    console.log('✅ All liquidity pools created!');
  }

  /**
   * Create a single liquidity pool
   */
  private async createPool(config: PoolConfig): Promise<string> {
    console.log(`💧 Creating pool on ${config.chain}...`);

    try {
      const response = await this.client.post('/v1/pools', config);

      if (response.data.status === 'success') {
        const poolId = response.data.poolId;
        console.log(`✅ Pool created: ${poolId}`);
        return poolId;
      } else {
        throw new Error(`Pool creation failed: ${response.data.error}`);
      }

    } catch (error: any) {
      console.error(`❌ Pool creation failed on ${config.chain}:`, error.response?.data || error.message);
      throw error;
    }
  }

  /**
   * Get pool statistics
   */
  async getPoolStats(poolId: string): Promise<{
    tvl: string;
    volume24h: string;
    apy: string;
  }> {
    try {
      const response = await this.client.get(`/v1/pools/${poolId}/stats`);
      return response.data;

    } catch (error: any) {
      console.error('❌ Stats fetch failed:', error.response?.data || error.message);
      throw error;
    }
  }
}

export default BullExAdapter;

// CLI entry point
if (require.main === module) {
  const config: BullExConfig = {
    apiKey: process.env.BULLEX_API_KEY || '',
    apiUrl: process.env.BULLEX_API_URL || 'https://api.bullex.io',
    bridgeAddress: {
      ethereum: process.env.ETH_BRIDGE_ADDRESS || '',
      bsc: process.env.BSC_BRIDGE_ADDRESS || '',
      solana: process.env.SOL_BRIDGE_ADDRESS || ''
    },
    tokens: {
      etr: {
        ethereum: process.env.ETR_ETH_ADDRESS || '',
        bsc: process.env.ETR_BSC_ADDRESS || '',
        solana: process.env.ETR_SOL_ADDRESS || ''
      },
      edsc: {
        ethereum: process.env.EDSC_ETH_ADDRESS || '',
        bsc: process.env.EDSC_BSC_ADDRESS || '',
        solana: process.env.EDSC_SOL_ADDRESS || ''
      }
    }
  };

  const adapter = new BullExAdapter(config);

  (async () => {
    try {
      // List tokens on all chains
      await adapter.listETR();
      await adapter.listEDSC();

      // Create liquidity pools
      await adapter.createPools();

      console.log('✅ BullEx integration complete!');

    } catch (error) {
      console.error('❌ BullEx adapter error:', error);
      process.exit(1);
    }
  })();
}
