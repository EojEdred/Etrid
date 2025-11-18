/**
 * Application constants
 */

// Network Configuration
export const NETWORK_CONFIG = {
  RPC_ENDPOINT: 'wss://rpc.flarechain.etrid.network',
  FALLBACK_RPC: 'wss://rpc2.flarechain.etrid.network',
  CHAIN_NAME: 'FlareChain',
  NETWORK_NAME: 'Ëtrid Mainnet',
  EXPLORER_URL: 'https://explorer.etrid.network',
};

// Token Decimals
export const TOKEN_DECIMALS = {
  ETR: 12,
  BTC: 8,
  ETH: 18,
  SOL: 9,
  USDT: 6,
  BNB: 18,
  ADA: 6,
  DOT: 10,
  MATIC: 18,
  LINK: 18,
  XRP: 6,
  DOGE: 8,
  TRX: 6,
  XLM: 7,
};

// Supported Assets
export const SUPPORTED_ASSETS = [
  'ETR',
  'BTC',
  'ETH',
  'SOL',
  'USDT',
  'BNB',
  'ADA',
  'DOT',
  'MATIC',
  'LINK',
  'XRP',
  'DOGE',
  'TRX',
  'XLM',
] as const;

export type SupportedAsset = typeof SUPPORTED_ASSETS[number];

// Transaction Speed Levels
export const TRANSACTION_SPEEDS = {
  INSTANT: {
    label: 'Instant',
    description: 'L3 - Sub-second confirmation',
    layer: 3,
    estimatedTime: '< 1 second',
    multiplier: 2.0,
  },
  FAST: {
    label: 'Fast',
    description: 'L2 - Fast confirmation',
    layer: 2,
    estimatedTime: '3-5 seconds',
    multiplier: 1.5,
  },
  STANDARD: {
    label: 'Standard',
    description: 'L1 - Normal confirmation',
    layer: 1,
    estimatedTime: '10-15 seconds',
    multiplier: 1.0,
  },
};

// Account Types
export const ACCOUNT_TYPES = {
  CHECKING: 'checking',
  SAVINGS: 'savings',
  STAKING: 'staking',
  INVESTMENT: 'investment',
} as const;

// Transaction Types
export const TRANSACTION_TYPES = {
  SEND: 'sent',
  RECEIVE: 'received',
  STAKE: 'staked',
  UNSTAKE: 'unstaked',
  REWARD: 'reward',
  BRIDGE: 'bridge',
  SWAP: 'swap',
} as const;

// Staking Configuration
export const STAKING_CONFIG = {
  MIN_STAKE: '10', // 10 ETR minimum
  LOCK_PERIOD: 7 * 24 * 60 * 60 * 1000, // 7 days in milliseconds
  APY_RANGE: {
    MIN: 3.5,
    MAX: 12.5,
  },
};

// Bridge Configuration
export const BRIDGE_CONFIG = {
  MIN_BRIDGE_AMOUNT: {
    BTC: '0.001',
    ETH: '0.01',
    SOL: '0.1',
  },
  BRIDGE_FEE_PERCENT: 0.3, // 0.3% bridge fee
  ESTIMATED_TIME: {
    BTC: '30-60 minutes',
    ETH: '15-30 minutes',
    SOL: '5-10 minutes',
  },
};

// Chart Configuration
export const CHART_CONFIG = {
  TIME_RANGES: ['1D', '1W', '1M', '3M', '1Y', 'ALL'],
  DEFAULT_RANGE: '1W',
  CHART_POINTS: {
    '1D': 24,
    '1W': 7,
    '1M': 30,
    '3M': 90,
    '1Y': 365,
    'ALL': 730,
  },
};

// Feature Flags
export const FEATURE_FLAGS = {
  ENABLE_BIOMETRIC: true,
  ENABLE_NOTIFICATIONS: true,
  ENABLE_BRIDGE: true,
  ENABLE_STAKING: true,
  ENABLE_GOVERNANCE: true,
  ENABLE_SWAP: true,
  ENABLE_NFT: false, // Coming soon
  ENABLE_LENDING: false, // Coming soon
};

// UI Constants
export const UI_CONSTANTS = {
  MAX_RECENT_TRANSACTIONS: 5,
  ITEMS_PER_PAGE: 20,
  DEBOUNCE_DELAY: 300,
  REFRESH_INTERVAL: 30000, // 30 seconds
  ANIMATION_DURATION: 200,
  LONG_PRESS_DURATION: 500,
};

// Validation Rules
export const VALIDATION_RULES = {
  MIN_PASSWORD_LENGTH: 8,
  MNEMONIC_WORDS: 12,
  ADDRESS_LENGTH: 48, // Substrate address length
  MIN_TRANSACTION_AMOUNT: '0.000001',
  MAX_TRANSACTION_AMOUNT: '1000000',
};

// Error Messages
export const ERROR_MESSAGES = {
  NETWORK_ERROR: 'Unable to connect to Ëtrid network. Please check your internet connection.',
  INSUFFICIENT_BALANCE: 'Insufficient balance for this transaction.',
  INVALID_ADDRESS: 'Invalid recipient address.',
  INVALID_AMOUNT: 'Invalid transaction amount.',
  INVALID_MNEMONIC: 'Invalid recovery phrase. Please check and try again.',
  BIOMETRIC_FAILED: 'Biometric authentication failed.',
  TRANSACTION_FAILED: 'Transaction failed. Please try again.',
  WALLET_EXISTS: 'A wallet already exists. Please restore or create a new one.',
};

// Success Messages
export const SUCCESS_MESSAGES = {
  WALLET_CREATED: 'Wallet created successfully!',
  WALLET_IMPORTED: 'Wallet imported successfully!',
  TRANSACTION_SENT: 'Transaction sent successfully!',
  BIOMETRIC_ENABLED: 'Biometric authentication enabled.',
  SETTINGS_SAVED: 'Settings saved successfully.',
};

// Storage Keys
export const STORAGE_KEYS = {
  THEME: 'etrid_theme',
  CURRENCY: 'etrid_currency',
  LANGUAGE: 'etrid_language',
  BIOMETRIC_ENABLED: 'etrid_biometric_enabled',
  NOTIFICATIONS_ENABLED: 'etrid_notifications_enabled',
  PRICE_ALERTS: 'etrid_price_alerts',
};

// API Endpoints (for future use)
export const API_ENDPOINTS = {
  PRICE_FEED: 'https://api.etrid.network/v1/prices',
  NEWS_FEED: 'https://api.etrid.network/v1/news',
  GOVERNANCE_PROPOSALS: 'https://api.etrid.network/v1/governance',
  STAKING_POOLS: 'https://api.etrid.network/v1/staking',
};
