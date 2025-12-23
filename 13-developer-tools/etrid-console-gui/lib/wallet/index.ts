/**
 * ETRID Wallet Library
 * Complete wallet solution for ETRID blockchain
 */

// Export main service
export { WalletService, WalletError, getWalletService, resetWalletService } from './service';

// Export React hooks
export {
  useWalletService,
  useWalletAccounts,
  useBalance,
  useChainInfo,
  useSendTransaction,
  useWalletBackup,
} from './hooks';

// Export crypto utilities
export {
  initCrypto,
  generateMnemonic,
  validateMnemonic,
  deriveKeypair,
  generateSS58Address,
  decodeSS58Address,
  encrypt,
  decrypt,
  hash,
  generateRandomSeed,
  mnemonicToSeed,
  keypairFromSeed,
  isValidSS58Address,
} from './crypto';

// Export storage utilities
export {
  hasStoredWallet,
  saveWallet,
  loadWallet,
  updateWalletAccounts,
  deleteWallet,
  exportWallet,
  importWallet,
  clearAllWalletData,
  getStorageSize,
  isStorageAvailable,
  StorageError,
} from './storage';

// Export types
export type {
  KeypairType,
  WalletAccount,
  StoredWallet,
  WalletConfig,
  CreateWalletOptions,
  ImportWalletOptions,
  DeriveAccountOptions,
  WalletBalance,
  ChainInfo,
} from './types';
