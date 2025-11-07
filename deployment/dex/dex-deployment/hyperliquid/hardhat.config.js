require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  networks: {
    mainnet: {
      url: process.env.HYPERLIQUID_RPC_URL || "https://rpc.hyperliquid.xyz/evm",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      chainId: 999,
      // HyperEVM specific settings
      gasPrice: "auto",
      timeout: 120000
    },
    testnet: {
      url: process.env.HYPERLIQUID_TESTNET_RPC || "https://rpc-testnet.hyperliquid.xyz/evm",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      chainId: 998
    }
  },
  etherscan: {
    // HyperEVM uses a custom explorer
    // Verification may not be supported yet
    apiKey: {
      hyperliquid: process.env.HYPERLIQUID_API_KEY || "not-needed"
    },
    customChains: [
      {
        network: "hyperliquid",
        chainId: 999,
        urls: {
          apiURL: "https://api.hyperliquid.xyz", // May not exist yet
          browserURL: "https://explorer.hyperliquid.xyz" // Check official docs
        }
      }
    ]
  }
};
