/**
 * ETRID Wallet Type Definitions
 */

export type KeypairType = 'sr25519' | 'ed25519';

export interface WalletAccount {
  address: string;
  publicKey: Uint8Array;
  name: string;
  derivationPath: string;
  keypairType: KeypairType;
  balance?: string;
}

export interface StoredWallet {
  mnemonic: string; // Encrypted
  accounts: WalletAccount[];
  createdAt: number;
  version: string;
}

export interface WalletConfig {
  ss58Prefix: number;
  derivationPath?: string;
  keypairType?: KeypairType;
}

export interface CreateWalletOptions {
  mnemonicWordCount?: 12 | 24;
  password: string;
  accountName?: string;
  keypairType?: KeypairType;
}

export interface ImportWalletOptions {
  mnemonic: string;
  password: string;
  accountName?: string;
  keypairType?: KeypairType;
}

export interface DeriveAccountOptions {
  derivationPath: string;
  accountName: string;
  keypairType?: KeypairType;
}

export interface WalletBalance {
  free: string;
  reserved: string;
  frozen: string;
  total: string;
}

export interface ChainInfo {
  name: string;
  genesisHash: string;
  ss58Prefix: number;
  tokenSymbol: string;
  tokenDecimals: number;
}
