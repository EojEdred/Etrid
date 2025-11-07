import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-verify";
import * as dotenv from "dotenv";

dotenv.config();

const DEPLOYER_PRIVATE_KEY = process.env.DEPLOYER_PRIVATE_KEY || "0x0000000000000000000000000000000000000000000000000000000000000001";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
      viaIR: true, // Enable IR-based code generation for better optimization
    },
  },
  networks: {
    // Local development
    hardhat: {
      chainId: 31337,
      accounts: {
        mnemonic: "test test test test test test test test test test test junk",
        accountsBalance: "10000000000000000000000", // 10k ETH
      },
    },
    localhost: {
      url: "http://127.0.0.1:8545",
      chainId: 31337,
    },

    // ========================================
    // ETH PBC (Ëtrid's EVM chain)
    // ========================================
    ethPBC: {
      url: process.env.ETH_PBC_RPC || "http://localhost:9944",
      chainId: 42069, // Custom chain ID for ETH PBC
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: 1000000000, // 1 gwei
      timeout: 60000,
    },

    // ========================================
    // Ethereum Networks
    // ========================================
    ethereum: {
      url: process.env.ETHEREUM_RPC || "https://eth-mainnet.g.alchemy.com/v2/demo",
      chainId: 1,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },
    sepolia: {
      url: process.env.SEPOLIA_RPC || "https://eth-sepolia.g.alchemy.com/v2/demo",
      chainId: 11155111,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },

    // ========================================
    // BNB Chain Networks
    // ========================================
    bsc: {
      url: process.env.BSC_RPC || "https://bsc-dataseed.bnbchain.org",
      chainId: 56,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: 3000000000, // 3 gwei
    },
    bscTestnet: {
      url: process.env.BSC_TESTNET_RPC || "https://data-seed-prebsc-1-s1.bnbchain.org:8545",
      chainId: 97,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: 10000000000, // 10 gwei
    },

    // ========================================
    // Polygon Networks
    // ========================================
    polygon: {
      url: process.env.POLYGON_RPC || "https://polygon-rpc.com",
      chainId: 137,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: 50000000000, // 50 gwei
    },
    polygonMumbai: {
      url: process.env.POLYGON_MUMBAI_RPC || "https://rpc-mumbai.maticvigil.com",
      chainId: 80001,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },

    // ========================================
    // Arbitrum Networks
    // ========================================
    arbitrum: {
      url: process.env.ARBITRUM_RPC || "https://arb1.arbitrum.io/rpc",
      chainId: 42161,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },
    arbitrumSepolia: {
      url: process.env.ARBITRUM_SEPOLIA_RPC || "https://sepolia-rollup.arbitrum.io/rpc",
      chainId: 421614,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },

    // ========================================
    // Base Networks
    // ========================================
    base: {
      url: process.env.BASE_RPC || "https://mainnet.base.org",
      chainId: 8453,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },
    baseSepolia: {
      url: process.env.BASE_SEPOLIA_RPC || "https://sepolia.base.org",
      chainId: 84532,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },

    // ========================================
    // Additional EVM Chains
    // ========================================
    // Avalanche
    avalanche: {
      url: process.env.AVALANCHE_RPC || "https://api.avax.network/ext/bc/C/rpc",
      chainId: 43114,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: 25000000000, // 25 gwei
    },
    // Optimism
    optimism: {
      url: process.env.OPTIMISM_RPC || "https://mainnet.optimism.io",
      chainId: 10,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },
    // Fantom
    fantom: {
      url: process.env.FANTOM_RPC || "https://rpc.ftm.tools",
      chainId: 250,
      accounts: [DEPLOYER_PRIVATE_KEY],
      gasPrice: "auto",
    },
  },

  etherscan: {
    apiKey: {
      mainnet: process.env.ETHERSCAN_API_KEY || "",
      sepolia: process.env.ETHERSCAN_API_KEY || "",
      bsc: process.env.BSCSCAN_API_KEY || "",
      bscTestnet: process.env.BSCSCAN_API_KEY || "",
      polygon: process.env.POLYGONSCAN_API_KEY || "",
      polygonMumbai: process.env.POLYGONSCAN_API_KEY || "",
      arbitrumOne: process.env.ARBISCAN_API_KEY || "",
      base: process.env.BASESCAN_API_KEY || "",
    },
  },

  gasReporter: {
    enabled: process.env.REPORT_GAS === "true",
    currency: "USD",
    coinmarketcap: process.env.COINMARKETCAP_API_KEY,
  },

  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts",
  },

  mocha: {
    timeout: 120000, // 2 minutes
  },
};

export default config;
