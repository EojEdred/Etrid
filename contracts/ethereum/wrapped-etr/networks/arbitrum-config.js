// Arbitrum One Configuration for Wrapped ETR Token
// Chain ID: 42161

module.exports = {
  network: {
    name: 'arbitrum',
    chainId: 42161,
    rpcUrl: process.env.ARBITRUM_RPC_URL || 'https://arb1.arbitrum.io/rpc',
    fallbackRpcUrls: [
      'https://arbitrum-mainnet.infura.io/v3/YOUR_INFURA_KEY',
      'https://arb-mainnet.g.alchemy.com/v2/YOUR_API_KEY',
      'https://rpc.ankr.com/arbitrum',
      'https://arbitrum.blockpi.network/v1/rpc/public'
    ]
  },

  gas: {
    // Arbitrum uses L2 gas pricing (much cheaper than L1)
    gasPrice: process.env.GAS_PRICE || 'auto',
    gasLimit: 5000000, // Higher limit for L2
    maxFeePerGas: process.env.MAX_FEE || null,
    maxPriorityFeePerGas: process.env.PRIORITY_FEE || null,

    // Estimated costs (in ETH on Arbitrum)
    estimatedDeploymentCost: '0.002 ETH (~$5 @ $2,500/ETH)',
    estimatedVerificationCost: '0.0002 ETH (~$0.50 @ $2,500/ETH)',

    // L1 data costs
    l1GasEstimate: '0.001 ETH', // L1 calldata cost
    notes: 'Arbitrum has two gas components: L2 execution (cheap) + L1 data (moderate)'
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
    name: 'Arbiscan',
    url: 'https://arbiscan.io',
    apiUrl: 'https://api.arbiscan.io/api',
    apiKey: process.env.ARBISCAN_API_KEY || '',

    // Verification settings
    verifyContract: true,
    verificationAttempts: 3,
    verificationDelay: 20000 // 20 seconds
  },

  deployment: {
    confirmations: 2, // Arbitrum has fast finality
    timeout: 240000, // 4 minutes

    // Constructor arguments for the ERC-20 contract
    constructorArgs: [],

    // Post-deployment actions
    postDeploy: {
      verify: true,
      transferOwnership: process.env.OWNER_ADDRESS || '',
      pauseContract: false,
      setupBridge: true,
      addToCamelot: true // Auto-list on Camelot DEX
    }
  },

  bridge: {
    // Arbitrum Bridge integration
    enabled: true,
    bridgeContract: process.env.ARBITRUM_BRIDGE_ADDRESS || '',

    // Native ETR chain connection
    nativeChainRpc: process.env.NATIVE_CHAIN_RPC || '',
    lockContract: process.env.LOCK_CONTRACT || '',

    // Arbitrum-specific bridge contracts
    l1Gateway: '0xa3A7B6F88361F48403514059F1F16C8E78d60EeC', // Standard ERC20 Gateway
    l2Gateway: '0x09e9222E96E7B4AE2a407B98d48e330053351EEe',
    l1Router: '0x72Ce9c846789fdB6fC1f34aC4AD25Dd9ef7031ef',
    l2Router: '0x5288c571Fd7aD117beA99bF60FE0846C4E84F933',

    // Retryable ticket parameters
    maxSubmissionCost: '0.0001 ETH',
    gasLimit: 100000,
    gasPriceBid: '100000000' // 0.1 gwei
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
    // Camelot (Arbitrum's leading DEX)
    camelot: {
      router: '0xc873fEcbd354f5A56E00E710B90EF4201db2448d',
      factory: '0x6EcCab422D763aC031210895C81787E87B43A652',
      weth: '0x82aF49447D8a07e3bd95BD0d56f35241523fBab1'
    },

    // Uniswap V3 on Arbitrum
    uniswapV3: {
      router: '0xE592427A0AEce92De3Edee1F18E0157C05861564',
      factory: '0x1F98431c8aD98523631AE4a59f267346ea31F984',
      positionManager: '0xC36442b4a4522E871399CD717aBDD847Ab11FE88',
      quoter: '0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6'
    },

    // SushiSwap on Arbitrum
    sushiswap: {
      router: '0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506',
      factory: '0xc35DADB65012eC5796536bD9864eD8773aBc74C4'
    }
  },

  // Frontend integration
  metadata: {
    website: 'https://etrid.io',
    logo: 'https://etrid.io/assets/logo.png',
    description: 'Wrapped ETR token for Arbitrum One - Ethereum security with L2 speed and cost',
    social: {
      twitter: 'https://twitter.com/etrid',
      telegram: 'https://t.me/etrid',
      discord: 'https://discord.gg/etrid'
    }
  },

  monitoring: {
    // Arbitrum-specific monitoring
    gasTracker: 'https://arbiscan.io/gastracker',
    networkStatus: 'https://portal.arbitrum.one/stats',
    l1GasTracker: 'https://etherscan.io/gastracker', // Monitor L1 costs too

    // Alert thresholds
    lowGasPrice: '100000000', // 0.1 gwei
    highGasPrice: '5000000000', // 5 gwei

    // Sequencer monitoring
    sequencerFeed: '0xFdB631F5EE196F0ed6FAa767959853A9F217697D', // Chainlink sequencer uptime feed
    checkSequencer: true
  },

  arbitrumSpecific: {
    // Nitro upgrade compatibility
    nitroCompatible: true,

    // ArbOS precompiles
    arbSys: '0x0000000000000000000000000000000000000064',
    arbRetryableTx: '0x000000000000000000000000000000000000006E',
    arbGasInfo: '0x000000000000000000000000000000000000006C',

    // Transaction types
    supportedTxTypes: ['0', '2'], // Legacy and EIP-1559

    // Block time
    averageBlockTime: '0.25 seconds',
    estimatedFinality: '13 minutes' // When L1 confirms
  }
};
