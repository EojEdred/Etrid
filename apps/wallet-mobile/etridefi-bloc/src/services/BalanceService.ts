import axios from 'axios';
import BigNumber from 'bignumber.js';
import {
  Network,
  NETWORK_CONFIG,
  WRAPPED_ETR,
  getNetworkConfig,
} from './ContractAddresses';

// Balance for a single chain
export interface ChainBalance {
  chain: Network;
  nativeBalance: string;
  wETRBalance: string;
  lastUpdated: Date;
}

// All balances state
export interface BalancesState {
  balances: Record<string, ChainBalance>;
  isLoading: boolean;
  lastUpdated: Date | null;
  errors: Record<string, string>;
}

// Singleton balance service
class BalanceServiceClass {
  private balances: Record<string, ChainBalance> = {};
  private errors: Record<string, string> = {};
  private isLoading = false;
  private lastUpdated: Date | null = null;
  private updateInterval: NodeJS.Timeout | null = null;
  private listeners: Set<(state: BalancesState) => void> = new Set();

  // Subscribe to balance updates
  subscribe(listener: (state: BalancesState) => void): () => void {
    this.listeners.add(listener);
    // Immediately send current state
    listener(this.getState());
    // Return unsubscribe function
    return () => this.listeners.delete(listener);
  }

  // Get current state
  getState(): BalancesState {
    return {
      balances: {...this.balances},
      isLoading: this.isLoading,
      lastUpdated: this.lastUpdated,
      errors: {...this.errors},
    };
  }

  // Notify all listeners
  private notifyListeners(): void {
    const state = this.getState();
    this.listeners.forEach(listener => listener(state));
  }

  // Start automatic balance updates
  startBalanceUpdates(
    address: string,
    chains: Network[] = [
      Network.BNB_CHAIN,
      Network.POLYGON,
      Network.ETHEREUM,
      Network.ARBITRUM,
    ],
    intervalMs: number = 30000,
  ): void {
    // Initial fetch
    this.refreshBalances(address, chains);

    // Clear existing interval
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
    }

    // Schedule periodic updates
    this.updateInterval = setInterval(() => {
      this.refreshBalances(address, chains);
    }, intervalMs);
  }

  // Stop automatic updates
  stopBalanceUpdates(): void {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }

  // Refresh balances for all chains
  async refreshBalances(address: string, chains: Network[]): Promise<void> {
    this.isLoading = true;
    this.errors = {};
    this.notifyListeners();

    const promises = chains.map(async chain => {
      try {
        const balance = await this.fetchBalance(address, chain);
        this.balances[chain] = balance;
      } catch (error: any) {
        this.errors[chain] = error.message || 'Failed to fetch balance';
      }
    });

    await Promise.allSettled(promises);

    // Also fetch Solana if address looks like a Solana pubkey
    if (address.length >= 32 && !address.startsWith('0x')) {
      try {
        const solBalance = await this.fetchSolanaBalance(address);
        this.balances[Network.SOLANA] = solBalance;
      } catch (error: any) {
        this.errors[Network.SOLANA] = error.message || 'Failed to fetch Solana balance';
      }
    }

    this.isLoading = false;
    this.lastUpdated = new Date();
    this.notifyListeners();
  }

  // Get wETR balance for a specific chain
  getWETRBalance(chain: Network): string {
    return this.balances[chain]?.wETRBalance || '0';
  }

  // Get native token balance for a specific chain
  getNativeBalance(chain: Network): string {
    return this.balances[chain]?.nativeBalance || '0';
  }

  // Get total wETR balance across all chains
  getTotalWETRBalance(): string {
    return Object.values(this.balances)
      .reduce((total, balance) => {
        return total.plus(new BigNumber(balance.wETRBalance));
      }, new BigNumber(0))
      .toString();
  }

  // Fetch balance for EVM chain
  private async fetchBalance(
    address: string,
    chain: Network,
  ): Promise<ChainBalance> {
    const wETRAddress = WRAPPED_ETR[chain];
    if (!wETRAddress) {
      throw new Error('Contract not deployed on this chain');
    }

    const [nativeBalance, wETRBalance] = await Promise.all([
      this.fetchNativeBalance(address, chain),
      this.fetchERC20Balance(address, wETRAddress, chain),
    ]);

    return {
      chain,
      nativeBalance,
      wETRBalance,
      lastUpdated: new Date(),
    };
  }

  // Fetch native token balance (ETH, BNB, etc.)
  private async fetchNativeBalance(
    address: string,
    chain: Network,
  ): Promise<string> {
    const config = getNetworkConfig(chain);

    const response = await axios.post(config.rpcURL, {
      jsonrpc: '2.0',
      method: 'eth_getBalance',
      params: [address, 'latest'],
      id: 1,
    });

    if (response.data.error) {
      throw new Error(response.data.error.message || 'RPC error');
    }

    const hexBalance = response.data.result;
    return this.hexToDecimal(hexBalance, config.decimals);
  }

  // Fetch ERC20 token balance
  private async fetchERC20Balance(
    address: string,
    tokenContract: string,
    chain: Network,
  ): Promise<string> {
    const config = getNetworkConfig(chain);

    // balanceOf(address) function selector: 0x70a08231
    const paddedAddress = address.toLowerCase().replace('0x', '').padStart(64, '0');
    const data = '0x70a08231' + paddedAddress;

    const response = await axios.post(config.rpcURL, {
      jsonrpc: '2.0',
      method: 'eth_call',
      params: [
        {
          to: tokenContract,
          data: data,
        },
        'latest',
      ],
      id: 1,
    });

    if (response.data.error) {
      throw new Error(response.data.error.message || 'RPC error');
    }

    const hexBalance = response.data.result;
    // wETR has 18 decimals on EVM chains
    return this.hexToDecimal(hexBalance, 18);
  }

  // Fetch Solana balance
  private async fetchSolanaBalance(address: string): Promise<ChainBalance> {
    const wETRMint = WRAPPED_ETR[Network.SOLANA];
    if (!wETRMint) {
      throw new Error('wETR not deployed on Solana');
    }

    const config = getNetworkConfig(Network.SOLANA);

    // Fetch SOL balance
    const solResponse = await axios.post(config.rpcURL, {
      jsonrpc: '2.0',
      method: 'getBalance',
      params: [address],
      id: 1,
    });

    const solLamports = solResponse.data.result?.value || 0;
    const solBalance = new BigNumber(solLamports)
      .dividedBy(new BigNumber(10).pow(9))
      .toString();

    // Fetch wETR SPL token balance
    const wETRBalance = await this.fetchSPLTokenBalance(address, wETRMint);

    return {
      chain: Network.SOLANA,
      nativeBalance: solBalance,
      wETRBalance,
      lastUpdated: new Date(),
    };
  }

  // Fetch SPL token balance
  private async fetchSPLTokenBalance(
    owner: string,
    mint: string,
  ): Promise<string> {
    const config = getNetworkConfig(Network.SOLANA);

    const response = await axios.post(config.rpcURL, {
      jsonrpc: '2.0',
      method: 'getTokenAccountsByOwner',
      params: [
        owner,
        {mint: mint},
        {encoding: 'jsonParsed'},
      ],
      id: 1,
    });

    const accounts = response.data.result?.value || [];
    let totalBalance = new BigNumber(0);

    for (const account of accounts) {
      const uiAmount =
        account?.account?.data?.parsed?.info?.tokenAmount?.uiAmount;
      if (uiAmount !== undefined) {
        totalBalance = totalBalance.plus(new BigNumber(uiAmount));
      }
    }

    return totalBalance.toString();
  }

  // Convert hex to decimal string
  private hexToDecimal(hex: string, decimals: number): string {
    let cleanHex = hex.toLowerCase();
    if (cleanHex.startsWith('0x')) {
      cleanHex = cleanHex.slice(2);
    }

    if (!cleanHex || cleanHex === '0') {
      return '0';
    }

    try {
      const bn = new BigNumber(cleanHex, 16);
      return bn.dividedBy(new BigNumber(10).pow(decimals)).toString();
    } catch {
      return '0';
    }
  }
}

// Export singleton instance
export const BalanceService = new BalanceServiceClass();

// Format balance for display
export function formatBalance(
  balance: string,
  decimals: number = 6,
  minDecimals: number = 2,
): string {
  const bn = new BigNumber(balance);
  if (bn.isNaN() || bn.isZero()) {
    return '0.00';
  }

  return bn.toFormat(decimals, BigNumber.ROUND_DOWN, {
    decimalSeparator: '.',
    groupSeparator: ',',
    groupSize: 3,
  });
}

// Get native token symbol for chain
export function getNativeSymbol(chain: Network): string {
  return getNetworkConfig(chain).symbol;
}
