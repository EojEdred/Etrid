import { ethers } from 'ethers';

// Uniswap V2 Router ABI (standard interface used by PancakeSwap, QuickSwap, Camelot)
const UNISWAP_V2_ROUTER_ABI = [
  'function getAmountsOut(uint amountIn, address[] memory path) public view returns (uint[] memory amounts)',
  'function swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)',
  'function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline) external payable returns (uint[] memory amounts)',
  'function swapExactTokensForETH(uint amountIn, uint amountOutMin, address[] calldata path, address to, uint deadline) external returns (uint[] memory amounts)',
  'function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB, uint liquidity)',
  'function removeLiquidity(address tokenA, address tokenB, uint liquidity, uint amountAMin, uint amountBMin, address to, uint deadline) external returns (uint amountA, uint amountB)',
  'function factory() external view returns (address)',
];

// Uniswap V2 Factory ABI
const UNISWAP_V2_FACTORY_ABI = [
  'function getPair(address tokenA, address tokenB) external view returns (address pair)',
];

// Uniswap V2 Pair ABI
const UNISWAP_V2_PAIR_ABI = [
  'function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)',
  'function token0() external view returns (address)',
  'function token1() external view returns (address)',
  'function totalSupply() external view returns (uint)',
  'function balanceOf(address owner) external view returns (uint)',
];

// ERC20 ABI
const ERC20_ABI = [
  'function approve(address spender, uint256 amount) external returns (bool)',
  'function allowance(address owner, address spender) external view returns (uint256)',
  'function balanceOf(address account) external view returns (uint256)',
  'function decimals() external view returns (uint8)',
  'function symbol() external view returns (string)',
];

export interface EVMDexConfig {
  chainId: number;
  chainName: string;
  wETRAddress: string;
  routerAddress: string;
  factoryAddress: string;
  wrappedNativeAddress: string; // WETH, WBNB, WMATIC, etc.
  dexName: string;
}

export const EVM_DEX_CONFIGS: Record<number, EVMDexConfig> = {
  // Ethereum Mainnet (Uniswap V2)
  1: {
    chainId: 1,
    chainName: 'Ethereum',
    wETRAddress: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    routerAddress: '0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D',
    factoryAddress: '0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f',
    wrappedNativeAddress: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2', // WETH
    dexName: 'Uniswap V2',
  },
  // BSC (PancakeSwap V2)
  56: {
    chainId: 56,
    chainName: 'BSC',
    wETRAddress: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    routerAddress: '0x10ED43C718714eb63d5aA57B78B54704E256024E',
    factoryAddress: '0xcA143Ce32Fe78f1f7019d7d551a6402fC5350c73',
    wrappedNativeAddress: '0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c', // WBNB
    dexName: 'PancakeSwap V2',
  },
  // Polygon (QuickSwap)
  137: {
    chainId: 137,
    chainName: 'Polygon',
    wETRAddress: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    routerAddress: '0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff',
    factoryAddress: '0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32',
    wrappedNativeAddress: '0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270', // WMATIC
    dexName: 'QuickSwap',
  },
  // Arbitrum (Camelot)
  42161: {
    chainId: 42161,
    chainName: 'Arbitrum',
    wETRAddress: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
    routerAddress: '0xc873fEcbd354f5A56E00E710B90EF4201db2448d',
    factoryAddress: '0x6EcCab422D763aC031210895C81787E87B43A652',
    wrappedNativeAddress: '0x82aF49447D8a07e3bd95BD0d56f35241523fBab1', // WETH
    dexName: 'Camelot',
  },
};

export interface SwapQuote {
  amountIn: string;
  amountOut: string;
  path: string[];
  priceImpact: number;
  minimumReceived: string;
  slippage: number;
}

export interface LiquidityPoolInfo {
  pairAddress: string;
  token0: string;
  token1: string;
  reserve0: string;
  reserve1: string;
  totalSupply: string;
  userBalance: string;
}

export class EVMDexService {
  private provider: ethers.Provider;
  private config: EVMDexConfig;

  constructor(provider: ethers.Provider, chainId: number) {
    this.provider = provider;
    const config = EVM_DEX_CONFIGS[chainId];
    if (!config) {
      throw new Error(`Unsupported chain ID: ${chainId}`);
    }
    this.config = config;
  }

  /**
   * Get token price in terms of another token
   */
  async getTokenPrice(
    tokenIn: string,
    tokenOut: string,
    amountIn: string
  ): Promise<string> {
    const router = new ethers.Contract(
      this.config.routerAddress,
      UNISWAP_V2_ROUTER_ABI,
      this.provider
    );

    const path = [tokenIn, tokenOut];
    const amounts = await router.getAmountsOut(amountIn, path);

    return amounts[1].toString();
  }

  /**
   * Get swap quote with price impact calculation
   */
  async getSwapQuote(
    tokenIn: string,
    tokenOut: string,
    amountIn: string,
    slippage: number = 0.5 // 0.5% default slippage
  ): Promise<SwapQuote> {
    const router = new ethers.Contract(
      this.config.routerAddress,
      UNISWAP_V2_ROUTER_ABI,
      this.provider
    );

    const path = [tokenIn, tokenOut];
    const amounts = await router.getAmountsOut(amountIn, path);
    const amountOut = amounts[1].toString();

    // Calculate price impact
    const poolInfo = await this.getLiquidityPoolInfo(tokenIn, tokenOut);
    const priceImpact = this.calculatePriceImpact(
      amountIn,
      amountOut,
      poolInfo.reserve0,
      poolInfo.reserve1
    );

    // Calculate minimum received with slippage
    const slippageMultiplier = (100 - slippage) / 100;
    const minimumReceived = (
      BigInt(amountOut) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();

    return {
      amountIn,
      amountOut,
      path,
      priceImpact,
      minimumReceived,
      slippage,
    };
  }

  /**
   * Execute a token swap
   */
  async executeSwap(
    signer: ethers.Signer,
    tokenIn: string,
    tokenOut: string,
    amountIn: string,
    minimumAmountOut: string,
    recipient: string,
    deadline?: number
  ): Promise<ethers.ContractTransactionResponse> {
    const router = new ethers.Contract(
      this.config.routerAddress,
      UNISWAP_V2_ROUTER_ABI,
      signer
    );

    // Check and approve token if needed
    await this.ensureTokenApproval(signer, tokenIn, amountIn);

    const path = [tokenIn, tokenOut];
    const deadlineTimestamp = deadline || Math.floor(Date.now() / 1000) + 60 * 20; // 20 minutes

    // Check if we're swapping from native token
    const isFromNative = tokenIn.toLowerCase() === this.config.wrappedNativeAddress.toLowerCase();

    if (isFromNative) {
      return await router.swapExactETHForTokens(
        minimumAmountOut,
        path,
        recipient,
        deadlineTimestamp,
        { value: amountIn }
      );
    } else {
      // Check if we're swapping to native token
      const isToNative = tokenOut.toLowerCase() === this.config.wrappedNativeAddress.toLowerCase();

      if (isToNative) {
        return await router.swapExactTokensForETH(
          amountIn,
          minimumAmountOut,
          path,
          recipient,
          deadlineTimestamp
        );
      } else {
        return await router.swapExactTokensForTokens(
          amountIn,
          minimumAmountOut,
          path,
          recipient,
          deadlineTimestamp
        );
      }
    }
  }

  /**
   * Get liquidity pool information
   */
  async getLiquidityPoolInfo(
    tokenA: string,
    tokenB: string,
    userAddress?: string
  ): Promise<LiquidityPoolInfo> {
    const router = new ethers.Contract(
      this.config.routerAddress,
      UNISWAP_V2_ROUTER_ABI,
      this.provider
    );

    const factoryAddress = await router.factory();
    const factory = new ethers.Contract(
      factoryAddress,
      UNISWAP_V2_FACTORY_ABI,
      this.provider
    );

    const pairAddress = await factory.getPair(tokenA, tokenB);

    if (pairAddress === ethers.ZeroAddress) {
      throw new Error('Liquidity pool does not exist');
    }

    const pair = new ethers.Contract(
      pairAddress,
      UNISWAP_V2_PAIR_ABI,
      this.provider
    );

    const [reserves, token0, token1, totalSupply] = await Promise.all([
      pair.getReserves(),
      pair.token0(),
      pair.token1(),
      pair.totalSupply(),
    ]);

    let userBalance = '0';
    if (userAddress) {
      userBalance = (await pair.balanceOf(userAddress)).toString();
    }

    return {
      pairAddress,
      token0,
      token1,
      reserve0: reserves[0].toString(),
      reserve1: reserves[1].toString(),
      totalSupply: totalSupply.toString(),
      userBalance,
    };
  }

  /**
   * Add liquidity to a pool
   */
  async addLiquidity(
    signer: ethers.Signer,
    tokenA: string,
    tokenB: string,
    amountADesired: string,
    amountBDesired: string,
    slippage: number = 0.5,
    recipient?: string,
    deadline?: number
  ): Promise<ethers.ContractTransactionResponse> {
    const router = new ethers.Contract(
      this.config.routerAddress,
      UNISWAP_V2_ROUTER_ABI,
      signer
    );

    // Approve both tokens
    await Promise.all([
      this.ensureTokenApproval(signer, tokenA, amountADesired),
      this.ensureTokenApproval(signer, tokenB, amountBDesired),
    ]);

    const slippageMultiplier = (100 - slippage) / 100;
    const amountAMin = (
      BigInt(amountADesired) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();
    const amountBMin = (
      BigInt(amountBDesired) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();

    const recipientAddress = recipient || await signer.getAddress();
    const deadlineTimestamp = deadline || Math.floor(Date.now() / 1000) + 60 * 20;

    return await router.addLiquidity(
      tokenA,
      tokenB,
      amountADesired,
      amountBDesired,
      amountAMin,
      amountBMin,
      recipientAddress,
      deadlineTimestamp
    );
  }

  /**
   * Remove liquidity from a pool
   */
  async removeLiquidity(
    signer: ethers.Signer,
    tokenA: string,
    tokenB: string,
    liquidity: string,
    slippage: number = 0.5,
    recipient?: string,
    deadline?: number
  ): Promise<ethers.ContractTransactionResponse> {
    const router = new ethers.Contract(
      this.config.routerAddress,
      UNISWAP_V2_ROUTER_ABI,
      signer
    );

    // Get pool info to calculate minimum amounts
    const poolInfo = await this.getLiquidityPoolInfo(tokenA, tokenB);

    const liquidityBN = BigInt(liquidity);
    const totalSupplyBN = BigInt(poolInfo.totalSupply);
    const reserve0BN = BigInt(poolInfo.reserve0);
    const reserve1BN = BigInt(poolInfo.reserve1);

    const amountA = (liquidityBN * reserve0BN / totalSupplyBN).toString();
    const amountB = (liquidityBN * reserve1BN / totalSupplyBN).toString();

    const slippageMultiplier = (100 - slippage) / 100;
    const amountAMin = (
      BigInt(amountA) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();
    const amountBMin = (
      BigInt(amountB) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();

    // Approve LP token
    await this.ensureTokenApproval(signer, poolInfo.pairAddress, liquidity);

    const recipientAddress = recipient || await signer.getAddress();
    const deadlineTimestamp = deadline || Math.floor(Date.now() / 1000) + 60 * 20;

    return await router.removeLiquidity(
      tokenA,
      tokenB,
      liquidity,
      amountAMin,
      amountBMin,
      recipientAddress,
      deadlineTimestamp
    );
  }

  /**
   * Calculate price impact percentage
   */
  private calculatePriceImpact(
    amountIn: string,
    amountOut: string,
    reserveIn: string,
    reserveOut: string
  ): number {
    const amountInBN = BigInt(amountIn);
    const amountOutBN = BigInt(amountOut);
    const reserveInBN = BigInt(reserveIn);
    const reserveOutBN = BigInt(reserveOut);

    // Spot price before trade
    const spotPriceBefore = Number(reserveOutBN) / Number(reserveInBN);

    // Effective price of the trade
    const effectivePrice = Number(amountOutBN) / Number(amountInBN);

    // Price impact = (spotPriceBefore - effectivePrice) / spotPriceBefore * 100
    const priceImpact = ((spotPriceBefore - effectivePrice) / spotPriceBefore) * 100;

    return Math.abs(priceImpact);
  }

  /**
   * Ensure token approval for router
   */
  private async ensureTokenApproval(
    signer: ethers.Signer,
    tokenAddress: string,
    amount: string
  ): Promise<void> {
    const token = new ethers.Contract(tokenAddress, ERC20_ABI, signer);
    const signerAddress = await signer.getAddress();
    const allowance = await token.allowance(signerAddress, this.config.routerAddress);

    if (BigInt(allowance.toString()) < BigInt(amount)) {
      const approveTx = await token.approve(this.config.routerAddress, ethers.MaxUint256);
      await approveTx.wait();
    }
  }

  /**
   * Get token info (symbol, decimals)
   */
  async getTokenInfo(tokenAddress: string): Promise<{ symbol: string; decimals: number }> {
    const token = new ethers.Contract(tokenAddress, ERC20_ABI, this.provider);
    const [symbol, decimals] = await Promise.all([
      token.symbol(),
      token.decimals(),
    ]);
    return { symbol, decimals: Number(decimals) };
  }

  /**
   * Get token balance
   */
  async getTokenBalance(tokenAddress: string, userAddress: string): Promise<string> {
    const token = new ethers.Contract(tokenAddress, ERC20_ABI, this.provider);
    const balance = await token.balanceOf(userAddress);
    return balance.toString();
  }
}
