// Polygon Mainnet Configuration for Wrapped ETR Token
// Chain ID: 137

module.exports = {
  network: {
    name: 'polygon',
    chainId: 137,
    rpcUrl: process.env.POLYGON_RPC_URL || 'https://polygon-rpc.com',
    fallbackRpcUrls: [
      'https://polygon-mainnet.g.alchemy.com/v2/YOUR_API_KEY',
      'https://rpc-mainnet.maticvigil.com',
      'https://rpc.ankr.com/polygon',
      'https://polygon-bor.publicnode.com'
    ]
  },

  gas: {
    // Polygon has much lower gas costs than Ethereum
    gasPrice: process.env.GAS_PRICE || 'auto',
    gasLimit: 3000000,
    maxFeePerGas: process.env.MAX_FEE || null, // For EIP-1559
    maxPriorityFeePerGas: process.env.PRIORITY_FEE || '30000000000', // 30 gwei

    // Estimated costs (in MATIC, based on 50 gwei)
    estimatedDeploymentCost: '0.5 MATIC (~$0.40 @ $0.80/MATIC)',
    estimatedVerificationCost: '0.05 MATIC (~$0.04 @ $0.80/MATIC)',

    // Gas optimization tips
    notes: 'Polygon gas is ~1000x cheaper than Ethereum. Use higher gas limits for safety.'
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
    name: 'Polygonscan',
    url: 'https://polygonscan.com',
    apiUrl: 'https://api.polygonscan.com/api',
    apiKey: process.env.POLYGONSCAN_API_KEY || '',

    // Verification settings
    verifyContract: true,
    verificationAttempts: 3,
    verificationDelay: 15000 // 15 seconds (faster blocks)
  },

  deployment: {
    confirmations: 5, // Wait for 5 block confirmations (blocks are faster)
    timeout: 180000, // 3 minutes

    // Constructor arguments for the ERC-20 contract
    constructorArgs: [],

    // Post-deployment actions
    postDeploy: {
      verify: true,
      transferOwnership: process.env.OWNER_ADDRESS || '',
      pauseContract: false,
      setupBridge: true,
      addToQuickSwap: true // Auto-list on QuickSwap
    }
  },

  bridge: {
    // Polygon PoS Bridge integration
    enabled: true,
    bridgeContract: process.env.POLYGON_BRIDGE_ADDRESS || '',

    // Native ETR chain connection
    nativeChainRpc: process.env.NATIVE_CHAIN_RPC || '',
    lockContract: process.env.LOCK_CONTRACT || '',

    // Polygon-specific bridge settings
    rootChainManager: '0xA0c68C638235ee32657e8f720a23ceC1bFc77C77', // Official Polygon bridge
    predicateProxy: '', // Will be set during registration
    childChainManager: '0xA6FA4fB5f76172d178d61B04b0ecd319C5d1C0aa'
  },

  security: {
    // Ownership and access control
    owner: process.env.OWNER_ADDRESS || '',
    multiSigWallet: process.env.MULTISIG_ADDRESS || '',

    // Safety features
    pausable: true,
    upgradeable: false,

    // Rate limiting (optional)
    maxTransferAmount: '10000000000000000000000000', // 10M tokens
    dailyLimit: '50000000000000000000000000' // 50M tokens
  },

  dex: {
    // QuickSwap (Polygon's main DEX)
    quickswap: {
      router: '0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff',
      factory: '0x5757371414417b8C6CAad45bAeF941aBc7d3Ab32',
      wmatic: '0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270'
    },

    // Uniswap V3 on Polygon
    uniswapV3: {
      router: '0xE592427A0AEce92De3Edee1F18E0157C05861564',
      factory: '0x1F98431c8aD98523631AE4a59f267346ea31F984',
      positionManager: '0xC36442b4a4522E871399CD717aBDD847Ab11FE88'
    }
  },

  // Frontend integration
  metadata: {
    website: 'https://etrid.io',
    logo: 'https://etrid.io/assets/logo.png',
    description: 'Wrapped ETR token for Polygon network - fast and low-cost transactions',
    social: {
      twitter: 'https://twitter.com/etrid',
      telegram: 'https://t.me/etrid',
      discord: 'https://discord.gg/etrid'
    }
  },

  monitoring: {
    // Polygon-specific monitoring
    gasTracker: 'https://polygonscan.com/gastracker',
    networkStatus: 'https://polygon.technology/system',

    // Alert thresholds
    lowGasPrice: '30000000000', // 30 gwei
    highGasPrice: '500000000000' // 500 gwei
  }
};
