import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction } from '@solana/spl-token';

// Raydium Program IDs
const RAYDIUM_LIQUIDITY_POOL_PROGRAM_ID_V4 = new PublicKey(
  '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8'
);
const RAYDIUM_AMM_AUTHORITY = new PublicKey(
  '5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1'
);

export interface SolanaDexConfig {
  wETRAddress: string;
  dexName: string;
  ammProgramId: string;
}

export const SOLANA_DEX_CONFIG: SolanaDexConfig = {
  wETRAddress: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
  dexName: 'Raydium',
  ammProgramId: '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8',
};

export interface SolanaSwapQuote {
  amountIn: string;
  amountOut: string;
  priceImpact: number;
  minimumReceived: string;
  slippage: number;
  route: string[];
}

export interface SolanaLiquidityPoolInfo {
  poolId: string;
  tokenA: string;
  tokenB: string;
  reserveA: string;
  reserveB: string;
  lpSupply: string;
  userLpBalance: string;
}

// Raydium AMM Instructions
const RAYDIUM_SWAP_INSTRUCTION = 9;
const RAYDIUM_ADD_LIQUIDITY_INSTRUCTION = 3;
const RAYDIUM_REMOVE_LIQUIDITY_INSTRUCTION = 4;

interface RaydiumPoolKeys {
  id: PublicKey;
  baseMint: PublicKey;
  quoteMint: PublicKey;
  lpMint: PublicKey;
  baseVault: PublicKey;
  quoteVault: PublicKey;
  authority: PublicKey;
  openOrders: PublicKey;
  targetOrders: PublicKey;
  withdrawQueue: PublicKey;
  lpVault: PublicKey;
  marketId: PublicKey;
  marketProgramId: PublicKey;
}

export class SolanaDexService {
  private connection: Connection;
  private config: SolanaDexConfig;

  constructor(rpcUrl: string) {
    this.connection = new Connection(rpcUrl, 'confirmed');
    this.config = SOLANA_DEX_CONFIG;
  }

  /**
   * Get token price from Raydium pool
   */
  async getTokenPrice(
    tokenIn: string,
    tokenOut: string,
    amountIn: string
  ): Promise<string> {
    const poolInfo = await this.getLiquidityPoolInfo(tokenIn, tokenOut);

    const amountInBN = BigInt(amountIn);
    const reserveInBN = BigInt(poolInfo.reserveA);
    const reserveOutBN = BigInt(poolInfo.reserveB);

    // Calculate output using constant product formula with 0.25% fee
    const amountInWithFee = amountInBN * BigInt(9975); // 0.25% fee
    const numerator = amountInWithFee * reserveOutBN;
    const denominator = reserveInBN * BigInt(10000) + amountInWithFee;
    const amountOut = numerator / denominator;

    return amountOut.toString();
  }

  /**
   * Get swap quote with price impact
   */
  async getSwapQuote(
    tokenIn: string,
    tokenOut: string,
    amountIn: string,
    slippage: number = 0.5
  ): Promise<SolanaSwapQuote> {
    const amountOut = await this.getTokenPrice(tokenIn, tokenOut, amountIn);

    // Get pool info for price impact calculation
    const poolInfo = await this.getLiquidityPoolInfo(tokenIn, tokenOut);
    const priceImpact = this.calculatePriceImpact(
      amountIn,
      amountOut,
      poolInfo.reserveA,
      poolInfo.reserveB
    );

    // Calculate minimum received with slippage
    const slippageMultiplier = (100 - slippage) / 100;
    const minimumReceived = (
      BigInt(amountOut) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();

    return {
      amountIn,
      amountOut,
      priceImpact,
      minimumReceived,
      slippage,
      route: [tokenIn, tokenOut],
    };
  }

  /**
   * Execute a token swap on Raydium
   */
  async executeSwap(
    walletPublicKey: PublicKey,
    tokenIn: string,
    tokenOut: string,
    amountIn: string,
    minimumAmountOut: string,
    poolKeys: RaydiumPoolKeys
  ): Promise<Transaction> {
    const transaction = new Transaction();

    // Get token accounts
    const tokenInMint = new PublicKey(tokenIn);
    const tokenOutMint = new PublicKey(tokenOut);

    const userTokenInAccount = await getAssociatedTokenAddress(
      tokenInMint,
      walletPublicKey
    );

    const userTokenOutAccount = await getAssociatedTokenAddress(
      tokenOutMint,
      walletPublicKey
    );

    // Check if token out account exists, if not create it
    const tokenOutAccountInfo = await this.connection.getAccountInfo(userTokenOutAccount);
    if (!tokenOutAccountInfo) {
      transaction.add(
        createAssociatedTokenAccountInstruction(
          walletPublicKey,
          userTokenOutAccount,
          walletPublicKey,
          tokenOutMint
        )
      );
    }

    // Build swap instruction
    const swapInstruction = await this.createSwapInstruction(
      poolKeys,
      walletPublicKey,
      userTokenInAccount,
      userTokenOutAccount,
      amountIn,
      minimumAmountOut
    );

    transaction.add(swapInstruction);

    // Get recent blockhash
    const { blockhash } = await this.connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = walletPublicKey;

    return transaction;
  }

  /**
   * Get liquidity pool information
   */
  async getLiquidityPoolInfo(
    tokenA: string,
    tokenB: string,
    userAddress?: string
  ): Promise<SolanaLiquidityPoolInfo> {
    // This is a simplified implementation
    // In production, you would query the Raydium API or on-chain data

    // For demo purposes, we'll use placeholder data
    // Real implementation would fetch from Raydium SDK or API

    const poolId = this.derivePoolAddress(tokenA, tokenB);

    try {
      const poolAccountInfo = await this.connection.getAccountInfo(new PublicKey(poolId));

      if (!poolAccountInfo) {
        throw new Error('Pool does not exist');
      }

      // Parse pool data (simplified - real implementation needs proper deserialization)
      // This is a placeholder structure
      const poolData = {
        poolId,
        tokenA,
        tokenB,
        reserveA: '1000000000000', // Placeholder
        reserveB: '1000000000000', // Placeholder
        lpSupply: '1000000000', // Placeholder
        userLpBalance: '0',
      };

      if (userAddress) {
        // Fetch user LP balance
        poolData.userLpBalance = await this.getUserLpBalance(poolId, userAddress);
      }

      return poolData;
    } catch (error) {
      throw new Error(`Failed to fetch pool info: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Add liquidity to Raydium pool
   */
  async addLiquidity(
    walletPublicKey: PublicKey,
    tokenA: string,
    tokenB: string,
    amountA: string,
    amountB: string,
    slippage: number = 0.5,
    poolKeys: RaydiumPoolKeys
  ): Promise<Transaction> {
    const transaction = new Transaction();

    const tokenAMint = new PublicKey(tokenA);
    const tokenBMint = new PublicKey(tokenB);

    const userTokenAAccount = await getAssociatedTokenAddress(
      tokenAMint,
      walletPublicKey
    );

    const userTokenBAccount = await getAssociatedTokenAddress(
      tokenBMint,
      walletPublicKey
    );

    // Get or create LP token account
    const userLpAccount = await getAssociatedTokenAddress(
      poolKeys.lpMint,
      walletPublicKey
    );

    const lpAccountInfo = await this.connection.getAccountInfo(userLpAccount);
    if (!lpAccountInfo) {
      transaction.add(
        createAssociatedTokenAccountInstruction(
          walletPublicKey,
          userLpAccount,
          walletPublicKey,
          poolKeys.lpMint
        )
      );
    }

    // Calculate minimum amounts with slippage
    const slippageMultiplier = (100 - slippage) / 100;
    const minAmountA = (
      BigInt(amountA) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();
    const minAmountB = (
      BigInt(amountB) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();

    // Build add liquidity instruction
    const addLiquidityInstruction = await this.createAddLiquidityInstruction(
      poolKeys,
      walletPublicKey,
      userTokenAAccount,
      userTokenBAccount,
      userLpAccount,
      amountA,
      amountB,
      minAmountA,
      minAmountB
    );

    transaction.add(addLiquidityInstruction);

    const { blockhash } = await this.connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = walletPublicKey;

    return transaction;
  }

  /**
   * Remove liquidity from Raydium pool
   */
  async removeLiquidity(
    walletPublicKey: PublicKey,
    tokenA: string,
    tokenB: string,
    lpAmount: string,
    slippage: number = 0.5,
    poolKeys: RaydiumPoolKeys
  ): Promise<Transaction> {
    const transaction = new Transaction();

    const tokenAMint = new PublicKey(tokenA);
    const tokenBMint = new PublicKey(tokenB);

    const userTokenAAccount = await getAssociatedTokenAddress(
      tokenAMint,
      walletPublicKey
    );

    const userTokenBAccount = await getAssociatedTokenAddress(
      tokenBMint,
      walletPublicKey
    );

    const userLpAccount = await getAssociatedTokenAddress(
      poolKeys.lpMint,
      walletPublicKey
    );

    // Get pool info to calculate minimum amounts
    const poolInfo = await this.getLiquidityPoolInfo(tokenA, tokenB);

    const lpAmountBN = BigInt(lpAmount);
    const lpSupplyBN = BigInt(poolInfo.lpSupply);
    const reserveABN = BigInt(poolInfo.reserveA);
    const reserveBBN = BigInt(poolInfo.reserveB);

    const amountA = (lpAmountBN * reserveABN / lpSupplyBN).toString();
    const amountB = (lpAmountBN * reserveBBN / lpSupplyBN).toString();

    const slippageMultiplier = (100 - slippage) / 100;
    const minAmountA = (
      BigInt(amountA) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();
    const minAmountB = (
      BigInt(amountB) * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
    ).toString();

    // Build remove liquidity instruction
    const removeLiquidityInstruction = await this.createRemoveLiquidityInstruction(
      poolKeys,
      walletPublicKey,
      userTokenAAccount,
      userTokenBAccount,
      userLpAccount,
      lpAmount,
      minAmountA,
      minAmountB
    );

    transaction.add(removeLiquidityInstruction);

    const { blockhash } = await this.connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = walletPublicKey;

    return transaction;
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
   * Derive pool address (simplified)
   */
  private derivePoolAddress(tokenA: string, tokenB: string): string {
    // In production, this would derive the actual PDA or fetch from Raydium API
    // For now, return a placeholder
    return 'PoolAddressPlaceholder';
  }

  /**
   * Get user LP token balance
   */
  private async getUserLpBalance(poolId: string, userAddress: string): Promise<string> {
    try {
      // In production, fetch actual LP token balance
      // For now, return placeholder
      return '0';
    } catch {
      return '0';
    }
  }

  /**
   * Create swap instruction for Raydium
   */
  private async createSwapInstruction(
    poolKeys: RaydiumPoolKeys,
    userPublicKey: PublicKey,
    userSourceToken: PublicKey,
    userDestToken: PublicKey,
    amountIn: string,
    minimumAmountOut: string
  ): Promise<TransactionInstruction> {
    // This is a simplified version
    // Production would use Raydium SDK or proper instruction encoding

    const keys = [
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: poolKeys.id, isSigner: false, isWritable: true },
      { pubkey: poolKeys.authority, isSigner: false, isWritable: false },
      { pubkey: poolKeys.openOrders, isSigner: false, isWritable: true },
      { pubkey: poolKeys.targetOrders, isSigner: false, isWritable: true },
      { pubkey: poolKeys.baseVault, isSigner: false, isWritable: true },
      { pubkey: poolKeys.quoteVault, isSigner: false, isWritable: true },
      { pubkey: poolKeys.marketProgramId, isSigner: false, isWritable: false },
      { pubkey: poolKeys.marketId, isSigner: false, isWritable: true },
      { pubkey: userSourceToken, isSigner: false, isWritable: true },
      { pubkey: userDestToken, isSigner: false, isWritable: true },
      { pubkey: userPublicKey, isSigner: true, isWritable: false },
    ];

    // Simplified data encoding (production needs proper borsh serialization)
    const data = Buffer.alloc(32);
    data.writeUInt8(RAYDIUM_SWAP_INSTRUCTION, 0);
    data.writeBigUInt64LE(BigInt(amountIn), 1);
    data.writeBigUInt64LE(BigInt(minimumAmountOut), 9);

    return new TransactionInstruction({
      keys,
      programId: new PublicKey(this.config.ammProgramId),
      data,
    });
  }

  /**
   * Create add liquidity instruction
   */
  private async createAddLiquidityInstruction(
    poolKeys: RaydiumPoolKeys,
    userPublicKey: PublicKey,
    userTokenA: PublicKey,
    userTokenB: PublicKey,
    userLpAccount: PublicKey,
    amountA: string,
    amountB: string,
    minAmountA: string,
    minAmountB: string
  ): Promise<TransactionInstruction> {
    const keys = [
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: poolKeys.id, isSigner: false, isWritable: true },
      { pubkey: poolKeys.authority, isSigner: false, isWritable: false },
      { pubkey: poolKeys.lpMint, isSigner: false, isWritable: true },
      { pubkey: poolKeys.baseVault, isSigner: false, isWritable: true },
      { pubkey: poolKeys.quoteVault, isSigner: false, isWritable: true },
      { pubkey: userTokenA, isSigner: false, isWritable: true },
      { pubkey: userTokenB, isSigner: false, isWritable: true },
      { pubkey: userLpAccount, isSigner: false, isWritable: true },
      { pubkey: userPublicKey, isSigner: true, isWritable: false },
    ];

    const data = Buffer.alloc(48);
    data.writeUInt8(RAYDIUM_ADD_LIQUIDITY_INSTRUCTION, 0);
    data.writeBigUInt64LE(BigInt(amountA), 1);
    data.writeBigUInt64LE(BigInt(amountB), 9);
    data.writeBigUInt64LE(BigInt(minAmountA), 17);
    data.writeBigUInt64LE(BigInt(minAmountB), 25);

    return new TransactionInstruction({
      keys,
      programId: new PublicKey(this.config.ammProgramId),
      data,
    });
  }

  /**
   * Create remove liquidity instruction
   */
  private async createRemoveLiquidityInstruction(
    poolKeys: RaydiumPoolKeys,
    userPublicKey: PublicKey,
    userTokenA: PublicKey,
    userTokenB: PublicKey,
    userLpAccount: PublicKey,
    lpAmount: string,
    minAmountA: string,
    minAmountB: string
  ): Promise<TransactionInstruction> {
    const keys = [
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: poolKeys.id, isSigner: false, isWritable: true },
      { pubkey: poolKeys.authority, isSigner: false, isWritable: false },
      { pubkey: poolKeys.lpMint, isSigner: false, isWritable: true },
      { pubkey: poolKeys.baseVault, isSigner: false, isWritable: true },
      { pubkey: poolKeys.quoteVault, isSigner: false, isWritable: true },
      { pubkey: userTokenA, isSigner: false, isWritable: true },
      { pubkey: userTokenB, isSigner: false, isWritable: true },
      { pubkey: userLpAccount, isSigner: false, isWritable: true },
      { pubkey: userPublicKey, isSigner: true, isWritable: false },
    ];

    const data = Buffer.alloc(32);
    data.writeUInt8(RAYDIUM_REMOVE_LIQUIDITY_INSTRUCTION, 0);
    data.writeBigUInt64LE(BigInt(lpAmount), 1);
    data.writeBigUInt64LE(BigInt(minAmountA), 9);
    data.writeBigUInt64LE(BigInt(minAmountB), 17);

    return new TransactionInstruction({
      keys,
      programId: new PublicKey(this.config.ammProgramId),
      data,
    });
  }

  /**
   * Get token balance on Solana
   */
  async getTokenBalance(tokenMint: string, userAddress: string): Promise<string> {
    try {
      const userPubkey = new PublicKey(userAddress);
      const mintPubkey = new PublicKey(tokenMint);
      const tokenAccount = await getAssociatedTokenAddress(mintPubkey, userPubkey);

      const accountInfo = await this.connection.getTokenAccountBalance(tokenAccount);
      return accountInfo.value.amount;
    } catch {
      return '0';
    }
  }
}
