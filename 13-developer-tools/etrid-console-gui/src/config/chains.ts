import { defineChain } from 'viem'

function envNumber(name: string, fallback: number): number {
  const value = process.env[name]
  if (!value) return fallback
  const parsed = Number(value)
  return Number.isFinite(parsed) ? parsed : fallback
}

function envBool(name: string, fallback = false): boolean {
  const value = process.env[name]
  if (!value) return fallback
  return value === 'true' || value === '1'
}

function envString(name: string): string | undefined {
  const value = process.env[name]
  return value && value.trim() ? value.trim() : undefined
}

const evmHttpRpc =
  envString('NEXT_PUBLIC_EVM_RPC_HTTP_URL') ??
  envString('NEXT_PUBLIC_ETH_PBC_RPC_HTTP') ??
  'http://127.0.0.1:8545'

const evmWsRpc = envString('NEXT_PUBLIC_EVM_RPC_WS_URL') ?? envString('NEXT_PUBLIC_ETH_PBC_RPC_WS')

const evmChainId = envNumber('NEXT_PUBLIC_EVM_CHAIN_ID', envNumber('NEXT_PUBLIC_ETH_PBC_CHAIN_ID', 8888))

const evmChainName =
  envString('NEXT_PUBLIC_EVM_CHAIN_NAME') ?? envString('NEXT_PUBLIC_ETH_PBC_CHAIN_NAME') ?? 'Primearc Core (EVM)'

const evmNetwork =
  envString('NEXT_PUBLIC_EVM_CHAIN_NETWORK') ?? envString('NEXT_PUBLIC_ETH_PBC_CHAIN_NETWORK') ?? 'primearc-core-evm'

const evmExplorerUrl =
  envString('NEXT_PUBLIC_EVM_EXPLORER_URL') ?? envString('NEXT_PUBLIC_ETH_PBC_EXPLORER_URL') ?? undefined

const evmExplorerName =
  envString('NEXT_PUBLIC_EVM_EXPLORER_NAME') ?? envString('NEXT_PUBLIC_ETH_PBC_EXPLORER_NAME') ?? 'Explorer'

const evmNativeSymbol =
  envString('NEXT_PUBLIC_EVM_NATIVE_SYMBOL') ?? envString('NEXT_PUBLIC_ETH_PBC_NATIVE_SYMBOL') ?? 'ETR'

const evmNativeName =
  envString('NEXT_PUBLIC_EVM_NATIVE_NAME') ?? envString('NEXT_PUBLIC_ETH_PBC_NATIVE_NAME') ?? 'ETR'

const evmNativeDecimals = envNumber(
  'NEXT_PUBLIC_EVM_NATIVE_DECIMALS',
  envNumber('NEXT_PUBLIC_ETH_PBC_NATIVE_DECIMALS', 18)
)

export const primearcCoreEvm = defineChain({
  id: evmChainId,
  name: evmChainName,
  network: evmNetwork,
  nativeCurrency: {
    decimals: evmNativeDecimals,
    name: evmNativeName,
    symbol: evmNativeSymbol,
  },
  rpcUrls: {
    default: {
      http: [evmHttpRpc],
      ...(evmWsRpc ? { webSocket: [evmWsRpc] } : {}),
    },
    public: {
      http: [evmHttpRpc],
      ...(evmWsRpc ? { webSocket: [evmWsRpc] } : {}),
    },
  },
  ...(evmExplorerUrl
    ? {
        blockExplorers: {
          default: {
            name: evmExplorerName,
            url: evmExplorerUrl,
          },
        },
      }
    : {}),
  testnet: envBool('NEXT_PUBLIC_EVM_TESTNET', true),
})
