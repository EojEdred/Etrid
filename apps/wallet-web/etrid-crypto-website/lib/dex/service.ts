import { ethers, Signer, ContractTransactionResponse, JsonRpcProvider, isAddress } from 'ethers';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { EVMDexService, EVM_DEX_CONFIGS, SwapQuote as EVMSwapQuote, LiquidityPoolInfo as EVMLiquidityPoolInfo } from './evm';
import { SolanaDexService, SolanaSwapQuote, SolanaLiquidityPoolInfo } from './solana';

export type ChainType = 'evm' | 'solana';

export interface ChainConfig {
  chainId?: number; // For EVM chains
  rpcUrl: string;
  chainType: ChainType;
  chainName: string;
  wETRAddress: string;
  nativeCurrency: {
    name: string;
    symbol: string;
    decimals: number;
  };
}

export const SUPPORTED_CHAINS: Record<string, ChainConfig> = {
  ethereum: {
    chainId: 1,
    rpcUrl: 'https://eth.llamarpc.com',
    chainType: 'evm',
    chainName: 'Ethereum',
    wETRAddress: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    nativeCurrency: {
      name: 'Ethereum',
      symbol: 'ETH',
      decimals: 18,
    },
  },
  bsc: {
    chainId: 56,
    rpcUrl: 'https://bsc-dataseed1.binance.org',
    chainType: 'evm',
    chainName: 'BSC',
    wETRAddress: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    nativeCurrency: {
      name: 'BNB',
      symbol: 'BNB',
      decimals: 18,
    },
  },
  polygon: {
    chainId: 137,
    rpcUrl: 'https://polygon-rpc.com',
    chainType: 'evm',
    chainName: 'Polygon',
    wETRAddress: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    nativeCurrency: {
      name: 'Matic',
      symbol: 'MATIC',
      decimals: 18,
    },
  },
  arbitrum: {
    chainId: 42161,
    rpcUrl: 'https://arb1.arbitrum.io/rpc',
    chainType: 'evm',
    chainName: 'Arbitrum',
    wETRAddress: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
    nativeCurrency: {
      name: 'Ethereum',
      symbol: 'ETH',
      decimals: 18,
    },
  },
  solana: {
    rpcUrl: 'https://api.mainnet-beta.solana.com',
    chainType: 'solana',
    chainName: 'Solana',
    wETRAddress: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
    nativeCurrency: {
      name: 'Solana',
      symbol: 'SOL',
      decimals: 9,
    },
  },
};

export interface UnifiedSwapQuote {
  amountIn: string;
  amountOut: string;
  priceImpact: number;
  minimumReceived: string;
  slippage: number;
  route: string[];
  estimatedGas?: string;
  chainType: ChainType;
}

export interface UnifiedLiquidityPoolInfo {
  poolId: string;
  tokenA: string;
  tokenB: string;
  reserveA: string;
  reserveB: string;
  totalSupply: string;
  userBalance: string;
  apr?: number;
  volume24h?: string;
  chainType: ChainType;
}

export interface SwapParams {
  fromToken: string;
  toToken: string;
  amount: string;
  slippage?: number;
  recipient?: string;
  deadline?: number;
}

export interface LiquidityParams {
  tokenA: string;
  tokenB: string;
  amountA: string;
  amountB: string;
  slippage?: number;
  recipient?: string;
  deadline?: number;
}

export class DexService {
  private chainConfig: ChainConfig;
  private evmService?: EVMDexService;
  private solanaService?: SolanaDexService;

  constructor(chainKey: string) {
    const config = SUPPORTED_CHAINS[chainKey];
    if (!config) {
      throw new Error(`Unsupported chain: ${chainKey}`);
    }
    this.chainConfig = config;
    this.initializeService();
  }

  private initializeService() {
    if (this.chainConfig.chainType === 'evm') {
      const provider = new JsonRpcProvider(this.chainConfig.rpcUrl);
      this.evmService = new EVMDexService(provider, this.chainConfig.chainId!);
    } else if (this.chainConfig.chainType === 'solana') {
      this.solanaService = new SolanaDexService(this.chainConfig.rpcUrl);
    }
  }

  /**
   * Get token price across all supported chains
   */
  async getTokenPrice(
    tokenIn: string,
    tokenOut: string,
    amountIn: string
  ): Promise<string> {
    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      return await this.evmService.getTokenPrice(tokenIn, tokenOut, amountIn);
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      return await this.solanaService.getTokenPrice(tokenIn, tokenOut, amountIn);
    }
    throw new Error('Service not initialized');
  }

  /**
   * Get unified swap quote
   */
  async getSwapQuote(params: SwapParams): Promise<UnifiedSwapQuote> {
    const { fromToken, toToken, amount, slippage = 0.5 } = params;

    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      const quote = await this.evmService.getSwapQuote(
        fromToken,
        toToken,
        amount,
        slippage
      );

      return {
        amountIn: quote.amountIn,
        amountOut: quote.amountOut,
        priceImpact: quote.priceImpact,
        minimumReceived: quote.minimumReceived,
        slippage: quote.slippage,
        route: quote.path,
        chainType: 'evm',
      };
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      const quote = await this.solanaService.getSwapQuote(
        fromToken,
        toToken,
        amount,
        slippage
      );

      return {
        amountIn: quote.amountIn,
        amountOut: quote.amountOut,
        priceImpact: quote.priceImpact,
        minimumReceived: quote.minimumReceived,
        slippage: quote.slippage,
        route: quote.route,
        chainType: 'solana',
      };
    }

    throw new Error('Service not initialized');
  }

  /**
   * Execute swap (returns transaction for signing)
   */
  async executeSwap(
    signer: Signer | PublicKey,
    params: SwapParams
  ): Promise<ContractTransactionResponse | Transaction> {
    const { fromToken, toToken, amount, slippage = 0.5, recipient, deadline } = params;

    const quote = await this.getSwapQuote(params);

    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      // Check if it's an EVM signer by checking for getAddress method
      if (typeof (signer as Signer).getAddress !== 'function') {
        throw new Error('Invalid signer for EVM chain');
      }

      const recipientAddress = recipient || await signer.getAddress();

      return await this.evmService.executeSwap(
        signer,
        fromToken,
        toToken,
        amount,
        quote.minimumReceived,
        recipientAddress,
        deadline
      );
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      if (!(signer instanceof PublicKey)) {
        throw new Error('Invalid signer for Solana chain');
      }

      // Note: For Solana, you would need to pass poolKeys
      // This is a simplified version
      throw new Error('Solana swap requires poolKeys parameter');
    }

    throw new Error('Service not initialized');
  }

  /**
   * Get unified liquidity pool info
   */
  async getLiquidityPoolInfo(
    tokenA: string,
    tokenB: string,
    userAddress?: string
  ): Promise<UnifiedLiquidityPoolInfo> {
    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      const poolInfo = await this.evmService.getLiquidityPoolInfo(
        tokenA,
        tokenB,
        userAddress
      );

      return {
        poolId: poolInfo.pairAddress,
        tokenA: poolInfo.token0,
        tokenB: poolInfo.token1,
        reserveA: poolInfo.reserve0,
        reserveB: poolInfo.reserve1,
        totalSupply: poolInfo.totalSupply,
        userBalance: poolInfo.userBalance,
        chainType: 'evm',
      };
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      const poolInfo = await this.solanaService.getLiquidityPoolInfo(
        tokenA,
        tokenB,
        userAddress
      );

      return {
        poolId: poolInfo.poolId,
        tokenA: poolInfo.tokenA,
        tokenB: poolInfo.tokenB,
        reserveA: poolInfo.reserveA,
        reserveB: poolInfo.reserveB,
        totalSupply: poolInfo.lpSupply,
        userBalance: poolInfo.userLpBalance,
        chainType: 'solana',
      };
    }

    throw new Error('Service not initialized');
  }

  /**
   * Add liquidity to pool
   */
  async addLiquidity(
    signer: Signer | PublicKey,
    params: LiquidityParams
  ): Promise<ContractTransactionResponse | Transaction> {
    const { tokenA, tokenB, amountA, amountB, slippage = 0.5, recipient, deadline } = params;

    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      // Check if it's an EVM signer by checking for getAddress method
      if (typeof (signer as Signer).getAddress !== 'function') {
        throw new Error('Invalid signer for EVM chain');
      }

      return await this.evmService.addLiquidity(
        signer,
        tokenA,
        tokenB,
        amountA,
        amountB,
        slippage,
        recipient,
        deadline
      );
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      if (!(signer instanceof PublicKey)) {
        throw new Error('Invalid signer for Solana chain');
      }

      throw new Error('Solana add liquidity requires poolKeys parameter');
    }

    throw new Error('Service not initialized');
  }

  /**
   * Remove liquidity from pool
   */
  async removeLiquidity(
    signer: Signer | PublicKey,
    tokenA: string,
    tokenB: string,
    liquidity: string,
    slippage: number = 0.5,
    recipient?: string,
    deadline?: number
  ): Promise<ContractTransactionResponse | Transaction> {
    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      // Check if it's an EVM signer by checking for getAddress method
      if (typeof (signer as Signer).getAddress !== 'function') {
        throw new Error('Invalid signer for EVM chain');
      }

      return await this.evmService.removeLiquidity(
        signer,
        tokenA,
        tokenB,
        liquidity,
        slippage,
        recipient,
        deadline
      );
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      if (!(signer instanceof PublicKey)) {
        throw new Error('Invalid signer for Solana chain');
      }

      throw new Error('Solana remove liquidity requires poolKeys parameter');
    }

    throw new Error('Service not initialized');
  }

  /**
   * Get token balance
   */
  async getTokenBalance(tokenAddress: string, userAddress: string): Promise<string> {
    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      return await this.evmService.getTokenBalance(tokenAddress, userAddress);
    } else if (this.chainConfig.chainType === 'solana' && this.solanaService) {
      return await this.solanaService.getTokenBalance(tokenAddress, userAddress);
    }

    throw new Error('Service not initialized');
  }

  /**
   * Get token info (for EVM chains)
   */
  async getTokenInfo(tokenAddress: string): Promise<{ symbol: string; decimals: number }> {
    if (this.chainConfig.chainType === 'evm' && this.evmService) {
      return await this.evmService.getTokenInfo(tokenAddress);
    }

    throw new Error('Token info only available for EVM chains');
  }

  /**
   * Get wETR address for current chain
   */
  getWETRAddress(): string {
    return this.chainConfig.wETRAddress;
  }

  /**
   * Get chain config
   */
  getChainConfig(): ChainConfig {
    return this.chainConfig;
  }

  /**
   * Get DEX name for current chain
   */
  getDexName(): string {
    if (this.chainConfig.chainType === 'evm' && this.chainConfig.chainId) {
      return EVM_DEX_CONFIGS[this.chainConfig.chainId]?.dexName || 'Unknown DEX';
    }
    return 'Raydium';
  }

  /**
   * Calculate optimal swap route (basic implementation)
   */
  async calculateOptimalRoute(
    fromToken: string,
    toToken: string,
    amount: string
  ): Promise<string[]> {
    // Basic implementation - direct route
    // Production would implement multi-hop routing
    return [fromToken, toToken];
  }

  /**
   * Get all wETR addresses across supported chains
   */
  static getAllWETRAddresses(): Record<string, string> {
    return Object.entries(SUPPORTED_CHAINS).reduce((acc, [key, config]) => {
      acc[key] = config.wETRAddress;
      return acc;
    }, {} as Record<string, string>);
  }

  /**
   * Get supported chains
   */
  static getSupportedChains(): Record<string, ChainConfig> {
    return SUPPORTED_CHAINS;
  }
}

// Helper functions

/**
 * Format token amount with decimals
 */
export function formatTokenAmount(amount: string, decimals: number): string {
  const divisor = BigInt(10 ** decimals);
  const wholePart = BigInt(amount) / divisor;
  const fractionalPart = BigInt(amount) % divisor;

  if (fractionalPart === BigInt(0)) {
    return wholePart.toString();
  }

  const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
  const trimmedFractional = fractionalStr.replace(/0+$/, '');

  return `${wholePart}.${trimmedFractional}`;
}

/**
 * Parse token amount to smallest unit
 */
export function parseTokenAmount(amount: string, decimals: number): string {
  const [whole, fraction = ''] = amount.split('.');
  const paddedFraction = fraction.padEnd(decimals, '0').slice(0, decimals);
  const value = BigInt(whole + paddedFraction);
  return value.toString();
}

/**
 * Calculate price ratio between two tokens
 */
export function calculatePriceRatio(
  reserveA: string,
  reserveB: string,
  decimalsA: number,
  decimalsB: number
): number {
  const reserveABN = BigInt(reserveA);
  const reserveBBN = BigInt(reserveB);

  const adjustedReserveA = Number(reserveABN) / (10 ** decimalsA);
  const adjustedReserveB = Number(reserveBBN) / (10 ** decimalsB);

  return adjustedReserveB / adjustedReserveA;
}

/**
 * Validate Ethereum address
 */
export function isValidEthereumAddress(address: string): boolean {
  return isAddress(address);
}

/**
 * Validate Solana address
 */
export function isValidSolanaAddress(address: string): boolean {
  try {
    new PublicKey(address);
    return true;
  } catch {
    return false;
  }
}

/**
 * Get chain type from address
 */
export function getChainTypeFromAddress(address: string): ChainType {
  if (isValidEthereumAddress(address)) {
    return 'evm';
  } else if (isValidSolanaAddress(address)) {
    return 'solana';
  }
  throw new Error('Invalid address format');
}
