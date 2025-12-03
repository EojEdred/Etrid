import axios from 'axios';
import BigNumber from 'bignumber.js';
import {Network, NETWORK_CONFIG, WRAPPED_ETR} from './ContractAddresses';

// Swap quote from DEX aggregator
export interface SwapQuote {
  id: string;
  chain: Network;
  fromToken: string;
  toToken: string;
  fromAmount: string;
  toAmount: string;
  rate: string;
  priceImpact: number;
  route: string;
  estimatedGas: number;
  timestamp: Date;
}

// Swap transaction to be signed
export interface SwapTransaction {
  id: string;
  quote: SwapQuote;
  fromAddress: string;
  to: string;
  data: string;
  value: string;
  gasLimit: number;
  gasPrice: string | null;
  nonce: number | null;
}

// DEX route info
export interface DEXRoute {
  dex: string;
  name: string;
  logoURL: string | null;
}

// DEX Error types
export enum DEXErrorType {
  INVALID_URL = 'INVALID_URL',
  NETWORK_ERROR = 'NETWORK_ERROR',
  PARSE_ERROR = 'PARSE_ERROR',
  NO_LIQUIDITY = 'NO_LIQUIDITY',
  RATE_LIMITED = 'RATE_LIMITED',
  API_ERROR = 'API_ERROR',
  INSUFFICIENT_BALANCE = 'INSUFFICIENT_BALANCE',
  SLIPPAGE_TOO_HIGH = 'SLIPPAGE_TOO_HIGH',
}

export class DEXError extends Error {
  type: DEXErrorType;
  code?: number;

  constructor(type: DEXErrorType, message: string, code?: number) {
    super(message);
    this.type = type;
    this.code = code;
    this.name = 'DEXError';
  }
}

// DEX Service singleton
class DEXServiceClass {
  private currentQuote: SwapQuote | null = null;
  private isLoadingQuote = false;
  private quoteError: string | null = null;
  private listeners: Set<() => void> = new Set();

  // API endpoints
  private readonly oneInchBaseURL = 'https://api.1inch.dev/swap/v6.0';
  private readonly jupiterBaseURL = 'https://quote-api.jup.ag/v6';

  // API keys
  private readonly zeroXAPIKey = 'bbd6ae09-7c0b-47b9-b6b5-e2682fd7d5c4';
  private readonly oneInchAPIKey = 'bbd6ae09-7c0b-47b9-b6b5-e2682fd7d5c4';

  // Subscribe to state changes
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
      currentQuote: this.currentQuote,
      isLoadingQuote: this.isLoadingQuote,
      quoteError: this.quoteError,
    };
  }

  // Get swap quote for token exchange
  async getQuote(
    fromToken: string,
    toToken: string,
    amount: string,
    chain: Network,
    slippage: number = 0.5,
  ): Promise<SwapQuote> {
    this.isLoadingQuote = true;
    this.quoteError = null;
    this.notifyListeners();

    try {
      let quote: SwapQuote;

      if (chain === Network.SOLANA) {
        quote = await this.getJupiterQuote(fromToken, toToken, amount, slippage);
      } else {
        quote = await this.get1inchQuote(fromToken, toToken, amount, chain, slippage);
      }

      this.currentQuote = quote;
      this.isLoadingQuote = false;
      this.notifyListeners();
      return quote;
    } catch (error: any) {
      this.quoteError = error.message || 'Failed to get quote';
      this.isLoadingQuote = false;
      this.notifyListeners();
      throw error;
    }
  }

  // Build swap transaction for execution
  async buildSwapTransaction(
    quote: SwapQuote,
    fromAddress: string,
    slippage: number = 0.5,
  ): Promise<SwapTransaction> {
    if (quote.chain === Network.SOLANA) {
      return this.buildJupiterSwapTx(quote, fromAddress, slippage);
    } else {
      return this.build1inchSwapTx(quote, fromAddress, slippage);
    }
  }

  // Get available DEX routes for a token pair
  async getAvailableRoutes(
    fromToken: string,
    toToken: string,
    chain: Network,
  ): Promise<DEXRoute[]> {
    switch (chain) {
      case Network.ETHEREUM:
        return [
          {dex: 'Uniswap V3', name: 'Uniswap', logoURL: null},
          {dex: 'SushiSwap', name: 'SushiSwap', logoURL: null},
          {dex: 'Curve', name: 'Curve', logoURL: null},
        ];
      case Network.BNB_CHAIN:
        return [
          {dex: 'PancakeSwap V3', name: 'PancakeSwap', logoURL: null},
          {dex: 'BiSwap', name: 'BiSwap', logoURL: null},
        ];
      case Network.POLYGON:
        return [
          {dex: 'QuickSwap', name: 'QuickSwap', logoURL: null},
          {dex: 'Uniswap V3', name: 'Uniswap', logoURL: null},
        ];
      case Network.ARBITRUM:
        return [
          {dex: 'Uniswap V3', name: 'Uniswap', logoURL: null},
          {dex: 'Camelot', name: 'Camelot', logoURL: null},
          {dex: 'SushiSwap', name: 'SushiSwap', logoURL: null},
        ];
      case Network.SOLANA:
        return [
          {dex: 'Raydium', name: 'Raydium', logoURL: null},
          {dex: 'Orca', name: 'Orca', logoURL: null},
          {dex: 'Jupiter', name: 'Jupiter Aggregator', logoURL: null},
        ];
      default:
        return [];
    }
  }

  // Clear current quote
  clearQuote(): void {
    this.currentQuote = null;
    this.quoteError = null;
    this.notifyListeners();
  }

  // 1inch Integration (EVM Chains)
  private async get1inchQuote(
    fromToken: string,
    toToken: string,
    amount: string,
    chain: Network,
    slippage: number,
  ): Promise<SwapQuote> {
    const chainId = NETWORK_CONFIG[chain].chainId;
    const amountWei = new BigNumber(amount)
      .multipliedBy(new BigNumber(10).pow(18))
      .toFixed(0);

    // Use native token address for non-contract tokens
    const nativeAddress = '0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE';
    const fromTokenAddress = fromToken.startsWith('0x') ? fromToken : nativeAddress;
    const toTokenAddress = toToken.startsWith('0x') ? toToken : nativeAddress;

    // If no API key, return simulated quote
    if (!this.oneInchAPIKey) {
      return this.simulatedEVMQuote(fromToken, toToken, amount, chain);
    }

    try {
      const response = await axios.get(
        `${this.oneInchBaseURL}/${chainId}/quote`,
        {
          params: {
            src: fromTokenAddress,
            dst: toTokenAddress,
            amount: amountWei,
          },
          headers: {
            Accept: 'application/json',
            Authorization: `Bearer ${this.oneInchAPIKey}`,
          },
          timeout: 15000,
        },
      );

      if (response.status === 429) {
        throw new DEXError(DEXErrorType.RATE_LIMITED, 'Rate limited, try again later');
      }

      const data = response.data;
      const toAmountStr = data.toAmount;
      const outputAmount = new BigNumber(toAmountStr)
        .dividedBy(new BigNumber(10).pow(18))
        .toString();
      const rate = new BigNumber(outputAmount).dividedBy(amount).toString();

      return {
        id: this.generateId(),
        chain,
        fromToken,
        toToken,
        fromAmount: amount,
        toAmount: outputAmount,
        rate,
        priceImpact: data.estimatedGas ? 0.1 : 0.0,
        route: '1inch Aggregator',
        estimatedGas: data.estimatedGas || 150000,
        timestamp: new Date(),
      };
    } catch (error: any) {
      // Fallback to simulated quote on error
      if (error instanceof DEXError) throw error;
      return this.simulatedEVMQuote(fromToken, toToken, amount, chain);
    }
  }

  private async build1inchSwapTx(
    quote: SwapQuote,
    fromAddress: string,
    slippage: number,
  ): Promise<SwapTransaction> {
    // For demo, return a simulated transaction
    return {
      id: this.generateId(),
      quote,
      fromAddress,
      to: '0x1111111254EEB25477B68fb85Ed929f73A960582', // 1inch router
      data: '0x...', // Swap calldata
      value: quote.fromToken.startsWith('0x') ? '0' : quote.fromAmount,
      gasLimit: quote.estimatedGas,
      gasPrice: null,
      nonce: null,
    };
  }

  // Jupiter Integration (Solana)
  private async getJupiterQuote(
    fromToken: string,
    toToken: string,
    amount: string,
    slippage: number,
  ): Promise<SwapQuote> {
    const solMint = 'So11111111111111111111111111111111111111112';
    const wETRMint = WRAPPED_ETR[Network.SOLANA] || '';

    const inputMint = fromToken === 'SOL' ? solMint : fromToken;
    const outputMint = toToken === 'SOL' ? solMint : toToken;

    // Convert to lamports (9 decimals)
    const amountLamports = new BigNumber(amount)
      .multipliedBy(new BigNumber(10).pow(9))
      .toFixed(0);

    try {
      const response = await axios.get(`${this.jupiterBaseURL}/quote`, {
        params: {
          inputMint,
          outputMint,
          amount: amountLamports,
          slippageBps: Math.round(slippage * 100),
        },
        headers: {
          Accept: 'application/json',
        },
        timeout: 15000,
      });

      if (response.status !== 200) {
        return this.simulatedSolanaQuote(fromToken, toToken, amount);
      }

      const data = response.data;
      const outAmount = new BigNumber(data.outAmount)
        .dividedBy(new BigNumber(10).pow(9))
        .toString();
      const rate = new BigNumber(outAmount).dividedBy(amount).toString();
      const priceImpact = parseFloat(data.priceImpactPct || '0');

      return {
        id: this.generateId(),
        chain: Network.SOLANA,
        fromToken,
        toToken,
        fromAmount: amount,
        toAmount: outAmount,
        rate,
        priceImpact,
        route: data.routePlan?.[0]?.swapInfo || 'Jupiter',
        estimatedGas: 5000,
        timestamp: new Date(),
      };
    } catch (error) {
      return this.simulatedSolanaQuote(fromToken, toToken, amount);
    }
  }

  private async buildJupiterSwapTx(
    quote: SwapQuote,
    fromAddress: string,
    slippage: number,
  ): Promise<SwapTransaction> {
    return {
      id: this.generateId(),
      quote,
      fromAddress,
      to: 'JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4', // Jupiter program
      data: '', // Base64 encoded transaction
      value: '0',
      gasLimit: 200000,
      gasPrice: null,
      nonce: null,
    };
  }

  // Simulated quotes for demo/fallback
  private simulatedEVMQuote(
    fromToken: string,
    toToken: string,
    amount: string,
    chain: Network,
  ): SwapQuote {
    const wETRAddress = WRAPPED_ETR[chain]?.toLowerCase() || '';
    let rate: string;

    if (toToken.toLowerCase().includes(wETRAddress)) {
      rate = '6.67'; // Buying wETR
    } else {
      rate = '0.15'; // Selling wETR
    }

    const toAmount = new BigNumber(amount).multipliedBy(rate).toString();

    return {
      id: this.generateId(),
      chain,
      fromToken,
      toToken,
      fromAmount: amount,
      toAmount,
      rate,
      priceImpact: 0.15,
      route: 'Simulated DEX Route',
      estimatedGas: 150000,
      timestamp: new Date(),
    };
  }

  private simulatedSolanaQuote(
    fromToken: string,
    toToken: string,
    amount: string,
  ): SwapQuote {
    const rate = '0.0015'; // wETR/SOL rate
    let toAmount: string;

    if (toToken === WRAPPED_ETR[Network.SOLANA]) {
      toAmount = new BigNumber(amount).dividedBy(rate).toString();
    } else {
      toAmount = new BigNumber(amount).multipliedBy(rate).toString();
    }

    const calculatedRate = new BigNumber(toAmount).dividedBy(amount).toString();

    return {
      id: this.generateId(),
      chain: Network.SOLANA,
      fromToken,
      toToken,
      fromAmount: amount,
      toAmount,
      rate: calculatedRate,
      priceImpact: 0.1,
      route: 'Raydium (Simulated)',
      estimatedGas: 5000,
      timestamp: new Date(),
    };
  }

  private generateId(): string {
    return 'xxxx-xxxx-xxxx'.replace(/x/g, () =>
      Math.floor(Math.random() * 16).toString(16),
    );
  }

  // Calculate minimum received with slippage
  calculateMinimumReceived(quote: SwapQuote, slippage: number): string {
    return new BigNumber(quote.toAmount)
      .multipliedBy(1 - slippage / 100)
      .toString();
  }

  // Format swap preview
  formatSwapPreview(quote: SwapQuote): string {
    const from = new BigNumber(quote.fromAmount).toFormat(6);
    const to = new BigNumber(quote.toAmount).toFormat(6);
    return `${from} -> ${to}`;
  }

  // Check if price impact is high
  isHighPriceImpact(quote: SwapQuote): boolean {
    return quote.priceImpact > 1.0;
  }

  // Format rate for display
  formatRate(quote: SwapQuote): string {
    return new BigNumber(quote.rate).toFormat(6);
  }

  // Format price impact
  formatPriceImpact(quote: SwapQuote): string {
    return `${quote.priceImpact.toFixed(2)}%`;
  }
}

// Export singleton
export const DEXService = new DEXServiceClass();
