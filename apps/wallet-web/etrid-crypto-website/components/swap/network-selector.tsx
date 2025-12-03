"use client"

import { Network, DEX, NETWORK_CONFIG, WETR_POOLS } from "@/lib/dex"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Check } from "lucide-react"

interface NetworkSelectorProps {
  selectedNetwork: Network
  onNetworkChange: (network: Network) => void
}

const NETWORK_ICONS: Record<Network, string> = {
  [Network.ETHEREUM]: '⟠',
  [Network.BSC]: '🟡',
  [Network.POLYGON]: '🟣',
  [Network.ARBITRUM]: '🔵',
  [Network.SOLANA]: '🌅',
}

const DEX_NAMES: Record<DEX, string> = {
  [DEX.UNISWAP]: 'Uniswap',
  [DEX.PANCAKESWAP]: 'PancakeSwap',
  [DEX.QUICKSWAP]: 'QuickSwap',
  [DEX.CAMELOT]: 'Camelot',
  [DEX.RAYDIUM]: 'Raydium',
}

export function NetworkSelector({ selectedNetwork, onNetworkChange }: NetworkSelectorProps) {
  const networks = Object.values(Network)

  return (
    <Card className="p-4 bg-card/30 backdrop-blur-xl border-border/50">
      <h3 className="text-sm font-semibold mb-3">Select Network</h3>
      <div className="grid grid-cols-2 md:grid-cols-5 gap-2">
        {networks.map((network) => {
          const config = NETWORK_CONFIG[network]
          const poolConfig = WETR_POOLS[network]
          const isSelected = network === selectedNetwork

          return (
            <Button
              key={network}
              variant={isSelected ? "default" : "outline"}
              className="h-auto flex-col gap-2 p-3 relative"
              onClick={() => onNetworkChange(network)}
            >
              {isSelected && (
                <Check className="w-4 h-4 absolute top-2 right-2" />
              )}
              <div className="text-2xl">{NETWORK_ICONS[network]}</div>
              <div className="text-sm font-medium">{config.name}</div>
              <div className="text-xs text-muted-foreground">
                {DEX_NAMES[poolConfig.dex]}
              </div>
            </Button>
          )
        })}
      </div>
    </Card>
  )
}
