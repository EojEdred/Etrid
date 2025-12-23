/**
 * ETRID Wallet Service
 * Main wallet service for managing accounts, transactions, and chain connections
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import type { KeyringPair } from '@polkadot/keyring/types';
import { formatBalance } from '@polkadot/util';
import {
  initCrypto,
  generateMnemonic,
  validateMnemonic,
  deriveKeypair,
  isValidSS58Address,
} from './crypto';
import {
  hasStoredWallet,
  saveWallet,
  loadWallet,
  updateWalletAccounts,
  deleteWallet,
  exportWallet as exportWalletStorage,
  importWallet as importWalletStorage,
} from './storage';
import type {
  WalletAccount,
  WalletConfig,
  CreateWalletOptions,
  ImportWalletOptions,
  DeriveAccountOptions,
  WalletBalance,
  ChainInfo,
  KeypairType,
} from './types';
import { ETRID_CHAIN } from '../chain/config';

export class WalletError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'WalletError';
  }
}

/**
 * ETRID Wallet Service
 * Manages wallet creation, accounts, and blockchain interactions
 */
export class WalletService {
  private api: ApiPromise | null = null;
  private keyring: Keyring | null = null;
  private currentMnemonic: string | null = null;
  private accounts: WalletAccount[] = [];
  private config: WalletConfig;
  private isInitialized = false;
  private connectionPromise: Promise<void> | null = null;

  constructor(config?: Partial<WalletConfig>) {
    this.config = {
      ss58Prefix: ETRID_CHAIN.ss58Prefix,
      derivationPath: '//0',
      keypairType: 'sr25519',
      ...config,
    };
  }

  /**
   * Initialize the wallet service
   * Must be called before using any other methods
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) return;

    try {
      await initCrypto();
      this.keyring = new Keyring({
        type: this.config.keypairType || 'sr25519',
        ss58Format: this.config.ss58Prefix,
      });
      this.isInitialized = true;
    } catch (error) {
      throw new WalletError(`Failed to initialize wallet: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Connect to ETRID chain
   * Attempts primary RPC first, falls back to secondary
   */
  async connect(): Promise<void> {
    if (this.api?.isConnected) return;

    // If there's already a connection attempt in progress, wait for it
    if (this.connectionPromise) {
      return this.connectionPromise;
    }

    this.connectionPromise = this._connect();
    try {
      await this.connectionPromise;
    } finally {
      this.connectionPromise = null;
    }
  }

  private async _connect(): Promise<void> {
    const endpoints = [ETRID_CHAIN.rpc.primary, ETRID_CHAIN.rpc.fallback];
    let lastError: Error | null = null;

    for (const endpoint of endpoints) {
      try {
        console.log(`Attempting to connect to ${endpoint}...`);
        const provider = new WsProvider(endpoint, 2500); // 2.5s timeout
        this.api = await ApiPromise.create({ provider });
        await this.api.isReady;

        console.log(`Connected to ETRID chain via ${endpoint}`);
        return;
      } catch (error) {
        lastError = error as Error;
        console.warn(`Failed to connect to ${endpoint}:`, error);
        if (this.api) {
          await this.api.disconnect();
          this.api = null;
        }
      }
    }

    throw new WalletError(
      `Failed to connect to ETRID chain: ${lastError?.message || 'All endpoints unavailable'}`
    );
  }

  /**
   * Disconnect from chain
   */
  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
      this.api = null;
    }
  }

  /**
   * Get chain information
   */
  async getChainInfo(): Promise<ChainInfo> {
    await this.connect();
    if (!this.api) throw new WalletError('Not connected to chain');

    const [chain, properties, genesisHash] = await Promise.all([
      this.api.rpc.system.chain(),
      this.api.rpc.system.properties(),
      this.api.genesisHash,
    ]);

    const tokenSymbolArr = properties.tokenSymbol.unwrapOr(['ETR']);
    const tokenDecimalsArr = properties.tokenDecimals.unwrapOr([18]);

    const tokenSymbol = tokenSymbolArr[0]?.toString() || 'ETR';
    const tokenDecimals = typeof tokenDecimalsArr[0] === 'number'
      ? tokenDecimalsArr[0]
      : (tokenDecimalsArr[0] as any).toNumber?.() || 18;

    return {
      name: chain.toString(),
      genesisHash: genesisHash.toHex(),
      ss58Prefix: this.config.ss58Prefix,
      tokenSymbol,
      tokenDecimals,
    };
  }

  /**
   * Create a new wallet
   */
  async createWallet(options: CreateWalletOptions): Promise<WalletAccount> {
    await this.initialize();

    const { mnemonicWordCount = 12, password, accountName = 'Account 1', keypairType = 'sr25519' } = options;

    // Generate mnemonic
    const mnemonic = generateMnemonic(mnemonicWordCount);

    if (!validateMnemonic(mnemonic)) {
      throw new WalletError('Generated invalid mnemonic');
    }

    // Derive first account
    const derivationPath = '//0';
    const keypair = deriveKeypair(mnemonic, derivationPath, keypairType, this.config.ss58Prefix);

    const account: WalletAccount = {
      address: keypair.address,
      publicKey: keypair.publicKey,
      name: accountName,
      derivationPath,
      keypairType,
    };

    // Save to storage
    await saveWallet(mnemonic, [account], password);

    // Update internal state
    this.currentMnemonic = mnemonic;
    this.accounts = [account];

    return account;
  }

  /**
   * Import an existing wallet from mnemonic
   */
  async importWallet(options: ImportWalletOptions): Promise<WalletAccount> {
    await this.initialize();

    const { mnemonic, password, accountName = 'Account 1', keypairType = 'sr25519' } = options;

    // Validate mnemonic
    if (!validateMnemonic(mnemonic)) {
      throw new WalletError('Invalid mnemonic phrase');
    }

    // Derive first account
    const derivationPath = '//0';
    const keypair = deriveKeypair(mnemonic, derivationPath, keypairType, this.config.ss58Prefix);

    const account: WalletAccount = {
      address: keypair.address,
      publicKey: keypair.publicKey,
      name: accountName,
      derivationPath,
      keypairType,
    };

    // Save to storage
    await saveWallet(mnemonic, [account], password);

    // Update internal state
    this.currentMnemonic = mnemonic;
    this.accounts = [account];

    return account;
  }

  /**
   * Unlock wallet with password
   */
  async unlockWallet(password: string): Promise<WalletAccount[]> {
    await this.initialize();

    if (!hasStoredWallet()) {
      throw new WalletError('No wallet found');
    }

    const wallet = await loadWallet(password);

    this.currentMnemonic = wallet.mnemonic;
    this.accounts = wallet.accounts;

    return this.accounts;
  }

  /**
   * Lock wallet (clear sensitive data)
   */
  lockWallet(): void {
    this.currentMnemonic = null;
    this.accounts = [];
  }

  /**
   * Check if wallet is unlocked
   */
  isUnlocked(): boolean {
    return this.currentMnemonic !== null && this.accounts.length > 0;
  }

  /**
   * Check if wallet exists in storage
   */
  hasWallet(): boolean {
    return hasStoredWallet();
  }

  /**
   * Derive a new account from the current wallet
   */
  async deriveAccount(options: DeriveAccountOptions, password: string): Promise<WalletAccount> {
    if (!this.currentMnemonic) {
      throw new WalletError('Wallet is locked');
    }

    const { derivationPath, accountName, keypairType = 'sr25519' } = options;

    // Check if derivation path already exists
    if (this.accounts.some((acc) => acc.derivationPath === derivationPath)) {
      throw new WalletError('Account with this derivation path already exists');
    }

    // Derive new account
    const keypair = deriveKeypair(this.currentMnemonic, derivationPath, keypairType, this.config.ss58Prefix);

    const account: WalletAccount = {
      address: keypair.address,
      publicKey: keypair.publicKey,
      name: accountName,
      derivationPath,
      keypairType,
    };

    // Update accounts
    this.accounts.push(account);

    // Save to storage
    await updateWalletAccounts(this.accounts, password);

    return account;
  }

  /**
   * Get all accounts
   */
  getAccounts(): WalletAccount[] {
    return [...this.accounts];
  }

  /**
   * Get account by address
   */
  getAccount(address: string): WalletAccount | undefined {
    return this.accounts.find((acc) => acc.address === address);
  }

  /**
   * Get account balance
   */
  async getBalance(address: string): Promise<WalletBalance> {
    await this.connect();
    if (!this.api) throw new WalletError('Not connected to chain');

    if (!isValidSS58Address(address)) {
      throw new WalletError('Invalid address');
    }

    const accountInfo = await this.api.query.system.account(address);
    const data = (accountInfo as any).data;

    const free = data.free.toString();
    const reserved = data.reserved.toString();
    const frozen = data.frozen?.toString() || '0';

    // Calculate total (free + reserved)
    const total = (BigInt(free) + BigInt(reserved)).toString();

    return {
      free,
      reserved,
      frozen,
      total,
    };
  }

  /**
   * Format balance for display
   */
  formatBalance(balance: string, decimals: number = 18, symbol: string = 'ETR'): string {
    return formatBalance(balance, {
      decimals,
      withUnit: symbol,
      withSi: true,
    });
  }

  /**
   * Send transaction
   */
  async sendTransaction(
    from: string,
    to: string,
    amount: string,
    password: string
  ): Promise<{ hash: string; success: boolean }> {
    await this.connect();
    if (!this.api || !this.keyring) {
      throw new WalletError('Wallet not initialized or not connected');
    }

    if (!this.currentMnemonic) {
      throw new WalletError('Wallet is locked');
    }

    const account = this.getAccount(from);
    if (!account) {
      throw new WalletError('Account not found');
    }

    // Recreate keypair from mnemonic
    const uri = `${this.currentMnemonic}${account.derivationPath}`;
    const pair = this.keyring.addFromUri(uri, {}, account.keypairType);

    // Create transfer
    return new Promise((resolve, reject) => {
      this.api!.tx.balances
        .transferKeepAlive(to, amount)
        .signAndSend(pair, ({ status, txHash }) => {
          if (status.isInBlock) {
            resolve({
              hash: txHash.toHex(),
              success: true,
            });
          } else if (status.isFinalized) {
            console.log(`Transaction finalized in block ${status.asFinalized.toHex()}`);
          }
        })
        .catch((error) => {
          reject(new WalletError(`Transaction failed: ${error.message}`));
        });
    });
  }

  /**
   * Export wallet mnemonic (requires password)
   */
  async exportMnemonic(password: string): Promise<string> {
    const wallet = await loadWallet(password);
    return wallet.mnemonic;
  }

  /**
   * Export wallet as encrypted backup
   */
  async exportWallet(password: string): Promise<string> {
    return exportWalletStorage(password);
  }

  /**
   * Import wallet from backup
   */
  async importWalletBackup(backupJson: string, password: string): Promise<void> {
    await importWalletStorage(backupJson, password);
    await this.unlockWallet(password);
  }

  /**
   * Delete wallet from storage
   */
  async deleteWallet(): Promise<void> {
    deleteWallet();
    this.lockWallet();
  }

  /**
   * Rename account
   */
  async renameAccount(address: string, newName: string, password: string): Promise<void> {
    const account = this.getAccount(address);
    if (!account) {
      throw new WalletError('Account not found');
    }

    account.name = newName;
    await updateWalletAccounts(this.accounts, password);
  }

  /**
   * Subscribe to balance changes
   */
  async subscribeBalance(
    address: string,
    callback: (balance: WalletBalance) => void
  ): Promise<() => void> {
    await this.connect();
    if (!this.api) throw new WalletError('Not connected to chain');

    const unsubscribe = await this.api.query.system.account(address, (accountInfo: any) => {
      const data = accountInfo.data;
      const free = data.free.toString();
      const reserved = data.reserved.toString();
      const frozen = data.frozen?.toString() || '0';
      const total = (BigInt(free) + BigInt(reserved)).toString();

      callback({ free, reserved, frozen, total });
    });

    return unsubscribe as unknown as () => void;
  }

  /**
   * Get current block number
   */
  async getBlockNumber(): Promise<number> {
    await this.connect();
    if (!this.api) throw new WalletError('Not connected to chain');

    const header = await this.api.rpc.chain.getHeader();
    return header.number.toNumber();
  }

  /**
   * Get transaction fee estimate
   */
  async estimateFee(from: string, to: string, amount: string): Promise<string> {
    await this.connect();
    if (!this.api) throw new WalletError('Not connected to chain');

    const info = await this.api.tx.balances
      .transferKeepAlive(to, amount)
      .paymentInfo(from);

    return info.partialFee.toString();
  }
}

// Singleton instance
let walletServiceInstance: WalletService | null = null;

/**
 * Get wallet service singleton
 */
export function getWalletService(config?: Partial<WalletConfig>): WalletService {
  if (!walletServiceInstance) {
    walletServiceInstance = new WalletService(config);
  }
  return walletServiceInstance;
}

/**
 * Reset wallet service (mainly for testing)
 */
export function resetWalletService(): void {
  walletServiceInstance = null;
}
