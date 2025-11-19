"use client"

import { useState } from "react"
import { Settings, User } from "lucide-react"
import { Button } from "@/components/ui/button"
import { BalanceCard } from "@/components/balance-card"
import { QuickActions } from "@/components/quick-actions"
import { RecentTransactions } from "@/components/recent-transactions"
import { BottomNav } from "@/components/bottom-nav"
import { SendScreen } from "@/components/send-screen"
import { ReceiveScreen } from "@/components/receive-screen"
import { GovernanceScreen } from "@/components/governance-screen"
import { StakingScreen } from "@/components/staking-screen"
import { MultiChainPortfolio } from "@/components/multi-chain-portfolio"
import { FiatRampScreen } from "@/components/FiatRampScreen"
import { AUBloccardScreen } from "@/components/AUBloccardScreen"
import { NFTGalleryScreen } from "@/components/nft-gallery-screen"
import { NFTMarketplaceScreen } from "@/components/nft-marketplace-screen"
import { TradingScreen } from "@/components/trading-screen"
import { LendingScreen } from "@/components/lending-screen"
import { SavingsGoalsScreen } from "@/components/savings-goals-screen"
import { SocialHub } from "@/components/social-hub"

export function WalletHome() {
  const [activeTab, setActiveTab] = useState("home")
  const [showSend, setShowSend] = useState(false)
  const [showReceive, setShowReceive] = useState(false)
  const [showFiatRamp, setShowFiatRamp] = useState(false)
  const [nftView, setNftView] = useState<"gallery" | "marketplace">("gallery")

  // Mock user address - in production, get from wallet context
  const userAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  if (showSend) {
    return <SendScreen onClose={() => setShowSend(false)} />
  }

  if (showReceive) {
    return <ReceiveScreen onClose={() => setShowReceive(false)} />
  }

  if (showFiatRamp) {
    return (
      <FiatRampScreen
        onClose={() => setShowFiatRamp(false)}
        onBuyCrypto={() => {/* TODO: Navigate to Buy Crypto Screen */}}
        onSellCrypto={() => {/* TODO: Navigate to Sell Crypto Screen */}}
        onDCASetup={() => {/* TODO: Navigate to DCA Setup Screen */}}
        onPaymentMethods={() => {/* TODO: Navigate to Payment Methods Screen */}}
      />
    )
  }

  if (activeTab === "card") {
    return (
      <AUBloccardScreen
        onBack={() => setActiveTab("home")}
        onAddCollateral={() => {/* TODO: Navigate to Add Collateral Screen */}}
        onManageCollateral={() => {/* TODO: Navigate to Manage Collateral Screen */}}
        onSettings={() => {/* TODO: Navigate to Bloccard Settings Screen */}}
        onTransactions={() => {/* TODO: Navigate to Transactions Screen */}}
        onSetup={() => {/* TODO: Navigate to Bloccard Setup Screen */}}
      />
    )
  }

  if (activeTab === "governance") {
    return <GovernanceScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
  }

  if (activeTab === "stake") {
    return <StakingScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
  }

  if (activeTab === "nfts") {
    if (nftView === "marketplace") {
      return <NFTMarketplaceScreen onBack={() => setNftView("gallery")} />
    }
    return (
      <NFTGalleryScreen
        userAddress={userAddress}
        onBack={() => setActiveTab("home")}
      />
    )
  }

  if (activeTab === "trade") {
    return <TradingScreen onBack={() => setActiveTab("home")} />
  }

  if (activeTab === "lending") {
    return <LendingScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
  }

  if (activeTab === "savings") {
    return <SavingsGoalsScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
  }

  if (activeTab === "social") {
    return <SocialHub activeTab={activeTab} onTabChange={setActiveTab} />
  }

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="flex items-center justify-between p-6">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center">
            <User className="w-5 h-5 text-accent" />
          </div>
        </div>

        <div className="flex items-center gap-2">
          <div className="text-2xl font-bold tracking-tight">
            <span className="text-foreground">ËTRID</span>
          </div>
        </div>

        <Button variant="ghost" size="icon" className="text-muted-foreground hover:text-foreground">
          <Settings className="w-5 h-5" />
        </Button>
      </header>

      {/* Main Content */}
      <main className="px-6 space-y-6">
        <BalanceCard />
        <QuickActions
          onSend={() => setShowSend(true)}
          onReceive={() => setShowReceive(true)}
          onSavings={() => setActiveTab("savings")}
          onLending={() => setActiveTab("lending")}
        />
        <MultiChainPortfolio />
        <RecentTransactions />
      </main>

      {/* Bottom Navigation */}
      <BottomNav activeTab={activeTab} onTabChange={setActiveTab} />
    </div>
  )
}
