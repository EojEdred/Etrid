import dotenv from "dotenv";

dotenv.config();

export const config = {
  // Telegram Bot Token (from @BotFather)
  botToken: process.env.TELEGRAM_BOT_TOKEN || "",

  // Admin Telegram User IDs (comma-separated)
  adminIds: process.env.TELEGRAM_ADMIN_IDS?.split(",").map((id) => parseInt(id.trim())) || [],

  // Default network for commands
  defaultNetwork: (process.env.DEFAULT_NETWORK as "mainnet" | "testnet") || "mainnet",

  // Contract addresses
  etrTokenMainnet: process.env.ETR_TOKEN_ADDRESS_MAINNET || "",
  masterChefMainnet: process.env.MASTERCHEF_ADDRESS_MAINNET || "",
  etrTokenTestnet: process.env.ETR_TOKEN_ADDRESS_TESTNET || "",
  masterChefTestnet: process.env.MASTERCHEF_ADDRESS_TESTNET || "",

  // RPC URLs
  bscMainnetRpc: process.env.BSC_MAINNET_RPC || "https://bsc-dataseed1.binance.org",
  bscTestnetRpc: process.env.BSC_TESTNET_RPC || "https://data-seed-prebsc-1-s1.binance.org:8545",

  // Alert thresholds
  lowBalanceThreshold: parseFloat(process.env.LOW_BALANCE_THRESHOLD || "1000000"), // 1M ÉTR
  criticalBalanceThreshold: parseFloat(process.env.CRITICAL_BALANCE_THRESHOLD || "500000"), // 500K ÉTR
  tvlDropThreshold: parseFloat(process.env.TVL_DROP_THRESHOLD || "10"), // 10% drop
  aprDropThreshold: parseFloat(process.env.APR_DROP_THRESHOLD || "20"), // 20% drop

  // Polling interval for alerts (ms)
  alertCheckInterval: parseInt(process.env.ALERT_CHECK_INTERVAL || "300000"), // 5 minutes

  // Database path
  databasePath: process.env.DATABASE_PATH || "./database/masterchef.db",
};

export function validateConfig(): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  if (!config.botToken) {
    errors.push("TELEGRAM_BOT_TOKEN not set in .env");
  }

  if (config.adminIds.length === 0) {
    errors.push("TELEGRAM_ADMIN_IDS not set in .env");
  }

  if (!config.etrTokenMainnet) {
    errors.push("ETR_TOKEN_ADDRESS_MAINNET not set in .env");
  }

  if (!config.masterChefMainnet) {
    errors.push("MASTERCHEF_ADDRESS_MAINNET not set in .env");
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
