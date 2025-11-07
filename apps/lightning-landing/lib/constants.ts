// Constants for the ÉTRID Lightning Network

export const SUPPORTED_CHAINS = [
  'Bitcoin', 'Ethereum', 'Solana', 'Cardano', 'Polkadot',
  'Avalanche', 'Polygon', 'Algorand', 'Cosmos', 'Tezos',
  'Flare', 'Hedera', 'NEAR', 'Aptos'
] as const;

export const NETWORK_STATS = {
  TVL_USD: 1234567890,
  ACTIVE_CHANNELS: 45678,
  TOTAL_TRANSACTIONS: 9876543,
  AVERAGE_FEE: 0.08,
} as const;

export const SOCIAL_LINKS = {
  github: 'https://github.com/etrid/lightning-network',
  discord: 'https://discord.gg/etrid',
  twitter: 'https://twitter.com/etrid',
  telegram: 'https://t.me/etrid',
} as const;

export const API_ENDPOINTS = {
  mainnet: 'https://lightning-rpc.etrid.org',
  testnet: 'https://lightning-testnet.etrid.org',
} as const;
