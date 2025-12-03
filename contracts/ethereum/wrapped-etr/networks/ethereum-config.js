// Ethereum Mainnet Configuration for Wrapped ETR Token
// Chain ID: 1

module.exports = {
  network: {
    name: 'ethereum',
    chainId: 1,
    rpcUrl: process.env.ETHEREUM_RPC_URL || 'https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY',
    fallbackRpcUrls: [
      'https://mainnet.infura.io/v3/YOUR_INFURA_KEY',
      'https://rpc.ankr.com/eth',
      'https://cloudflare-eth.com'
    ]
  },

  gas: {
    // Ethereum mainnet typically has higher gas costs
    gasPrice: process.env.GAS_PRICE || 'auto', // Will use network gas price
    gasLimit: 3000000,
    maxFeePerGas: process.env.MAX_FEE || null, // For EIP-1559
    maxPriorityFeePerGas: process.env.PRIORITY_FEE || null,

    // Estimated costs (in ETH, based on 50 gwei)
    estimatedDeploymentCost: '0.15 ETH (~$375 @ $2,500/ETH)',
    estimatedVerificationCost: '0.01 ETH (~$25 @ $2,500/ETH)'
  },

  token: {
    name: 'Wrapped ETR',
    symbol: 'wETR',
    decimals: 18,
    totalSupply: '1000000000000000000000000000', // 1 billion tokens

    // Optional: Initial distribution addresses
    initialHolders: {
      treasury: process.env.TREASURY_ADDRESS || '',
      liquidityPool: process.env.LIQUIDITY_ADDRESS || '',
      bridge: process.env.BRIDGE_ADDRESS || ''
    }
  },

  explorer: {
    name: 'Etherscan',
    url: 'https://etherscan.io',
    apiUrl: 'https://api.etherscan.io/api',
    apiKey: process.env.ETHERSCAN_API_KEY || '',

    // Verification settings
    verifyContract: true,
    verificationAttempts: 3,
    verificationDelay: 30000 // 30 seconds
  },

  deployment: {
    confirmations: 3, // Wait for 3 block confirmations
    timeout: 300000, // 5 minutes

    // Constructor arguments for the ERC-20 contract
    constructorArgs: [],

    // Post-deployment actions
    postDeploy: {
      verify: true,
      transferOwnership: process.env.OWNER_ADDRESS || '',
      pauseContract: false,
      setupBridge: true
    }
  },

  bridge: {
    // Bridge contract addresses for cross-chain transfers
    enabled: true,
    bridgeContract: process.env.ETH_BRIDGE_ADDRESS || '',

    // Native ETR chain connection
    nativeChainRpc: process.env.NATIVE_CHAIN_RPC || '',
    lockContract: process.env.LOCK_CONTRACT || ''
  },

  security: {
    // Ownership and access control
    owner: process.env.OWNER_ADDRESS || '',
    multiSigWallet: process.env.MULTISIG_ADDRESS || '',

    // Safety features
    pausable: true,
    upgradeable: false, // Set to true if using proxy pattern

    // Rate limiting (optional)
    maxTransferAmount: '10000000000000000000000000', // 10M tokens
    dailyLimit: '50000000000000000000000000' // 50M tokens
  },

  // Frontend integration
  metadata: {
    website: 'https://etrid.io',
    logo: 'https://etrid.io/assets/logo.png',
    description: 'Wrapped ETR token for Ethereum mainnet',
    social: {
      twitter: 'https://twitter.com/etrid',
      telegram: 'https://t.me/etrid',
      discord: 'https://discord.gg/etrid'
    }
  }
};
