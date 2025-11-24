import { defineChain } from 'viem'

export const ethPBC = defineChain({
  id: 8888, // TODO: Update with actual chain ID from ETH PBC node
  name: 'ETH Partition Burst Chain',
  network: 'eth-pbc',
  nativeCurrency: {
    decimals: 18,
    name: 'ETR',
    symbol: 'ETR',
  },
  rpcUrls: {
    default: {
      http: ['http://127.0.0.1:9944'],
      webSocket: ['ws://127.0.0.1:9944'],
    },
    public: {
      http: ['http://127.0.0.1:9944'],
      webSocket: ['ws://127.0.0.1:9944'],
    },
  },
  blockExplorers: {
    default: {
      name: 'ETH PBC Explorer',
      url: 'http://localhost:3001' // TODO: Update when explorer is available
    },
  },
  testnet: true,
})
