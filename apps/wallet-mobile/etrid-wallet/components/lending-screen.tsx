/**
 * LendingScreen - Main lending hub
 */

'use client'

import { useState } from 'react'
import { ArrowLeft, TrendingUp, TrendingDown, Wallet } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { BottomNav } from '@/components/bottom-nav'
import { APYCard } from '@/components/lending/apy-card'
import { useLending } from '@/hooks/use-lending'
import { SupplyScreen } from '@/components/supply-screen'
import { BorrowScreen } from '@/components/borrow-screen'

interface LendingScreenProps {
  onBack: () => void
  activeTab: string
  onTabChange: (tab: string) => void
}

export function LendingScreen({ onBack, activeTab, onTabChange }: LendingScreenProps) {
  const { positions, totalSupplied, totalBorrowed, netAPY, loading } = useLending()
  const [currentView, setCurrentView] = useState<'main' | 'supply' | 'borrow'>('main')
  const [selectedAsset, setSelectedAsset] = useState('')

  // Mock asset data
  const assets = [
    { name: 'ÉTR', icon: '💎', supplyAPY: 8.0, borrowAPY: 12.0, totalSupplied: 125000, totalBorrowed: 45000 },
    { name: 'BTC', icon: '₿', supplyAPY: 4.0, borrowAPY: 8.0, totalSupplied: 250, totalBorrowed: 100 },
    { name: 'ETH', icon: 'Ξ', supplyAPY: 5.0, borrowAPY: 10.0, totalSupplied: 5000, totalBorrowed: 2000 },
  ]

  if (currentView === 'supply') {
    return <SupplyScreen asset={selectedAsset} onBack={() => setCurrentView('main')} />
  }

  if (currentView === 'borrow') {
    return <BorrowScreen asset={selectedAsset} onBack={() => setCurrentView('main')} />
  }

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-12 h-12 rounded-full bg-primary/20 flex items-center justify-center">
            <Wallet className="w-6 h-6 text-primary" />
          </div>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Lending & Borrowing</h1>
            <p className="text-sm text-muted-foreground">Supply to earn, borrow against collateral</p>
          </div>
        </div>

        {/* Summary Cards */}
        <div className="grid grid-cols-3 gap-3">
          <div className="glass rounded-xl p-4">
            <div className="flex items-center gap-1 mb-1">
              <TrendingUp className="w-3 h-3 text-success" />
              <p className="text-xs text-muted-foreground">Supplied</p>
            </div>
            <p className="text-lg font-bold">${totalSupplied.toLocaleString()}</p>
          </div>

          <div className="glass rounded-xl p-4">
            <div className="flex items-center gap-1 mb-1">
              <TrendingDown className="w-3 h-3 text-destructive" />
              <p className="text-xs text-muted-foreground">Borrowed</p>
            </div>
            <p className="text-lg font-bold">${totalBorrowed.toLocaleString()}</p>
          </div>

          <div className="glass rounded-xl p-4">
            <p className="text-xs text-muted-foreground mb-1">Net APY</p>
            <p className={`text-lg font-bold ${netAPY >= 0 ? 'text-success' : 'text-destructive'}`}>
              {netAPY >= 0 ? '+' : ''}{netAPY.toFixed(2)}%
            </p>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="px-6 space-y-6">
        <Tabs defaultValue="supply" className="w-full">
          <TabsList className="grid w-full grid-cols-3 glass">
            <TabsTrigger value="supply">Supply</TabsTrigger>
            <TabsTrigger value="borrow">Borrow</TabsTrigger>
            <TabsTrigger value="positions">My Positions</TabsTrigger>
          </TabsList>

          <TabsContent value="supply" className="space-y-4 mt-6">
            {assets.map((asset) => (
              <APYCard
                key={asset.name}
                asset={asset.name}
                icon={asset.icon}
                apy={asset.supplyAPY}
                type="supply"
                totalAmount={asset.totalSupplied}
                yourPosition={positions.find(p => p.asset === asset.name && p.type === 'supply')?.amount}
                onAction={() => {
                  setSelectedAsset(asset.name)
                  setCurrentView('supply')
                }}
              />
            ))}
          </TabsContent>

          <TabsContent value="borrow" className="space-y-4 mt-6">
            {assets.map((asset) => (
              <APYCard
                key={asset.name}
                asset={asset.name}
                icon={asset.icon}
                apy={asset.borrowAPY}
                type="borrow"
                totalAmount={asset.totalBorrowed}
                yourPosition={positions.find(p => p.asset === asset.name && p.type === 'borrow')?.amount}
                onAction={() => {
                  setSelectedAsset(asset.name)
                  setCurrentView('borrow')
                }}
              />
            ))}
          </TabsContent>

          <TabsContent value="positions" className="space-y-4 mt-6">
            {positions.length === 0 ? (
              <div className="glass rounded-2xl p-8 text-center">
                <p className="text-muted-foreground">No active positions</p>
                <p className="text-sm text-muted-foreground mt-2">
                  Supply or borrow to get started
                </p>
              </div>
            ) : (
              positions.map((position) => (
                <div key={position.id} className="glass rounded-2xl p-5 space-y-3">
                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="font-semibold">{position.asset}</h3>
                      <p className="text-sm text-muted-foreground capitalize">{position.type}</p>
                    </div>
                    <div className="text-right">
                      <p className="text-lg font-bold">{position.amount.toFixed(2)}</p>
                      <p className={`text-sm ${position.type === 'supply' ? 'text-success' : 'text-destructive'}`}>
                        {position.apy}% APY
                      </p>
                    </div>
                  </div>

                  <div className="glass-strong rounded-xl p-3">
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-muted-foreground">Interest Accrued</span>
                      <span className="font-semibold">{position.interestAccrued.toFixed(4)} {position.asset}</span>
                    </div>
                  </div>
                </div>
              ))
            )}
          </TabsContent>
        </Tabs>
      </main>

      {/* Bottom Navigation */}
      <BottomNav activeTab={activeTab} onTabChange={onTabChange} />
    </div>
  )
}
