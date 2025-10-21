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

export function WalletHome() {
  const [activeTab, setActiveTab] = useState("home")
  const [showSend, setShowSend] = useState(false)
  const [showReceive, setShowReceive] = useState(false)

  if (showSend) {
    return <SendScreen onClose={() => setShowSend(false)} />
  }

  if (showReceive) {
    return <ReceiveScreen onClose={() => setShowReceive(false)} />
  }

  if (activeTab === "governance") {
    return <GovernanceScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
  }

  if (activeTab === "stake") {
    return <StakingScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
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
        <QuickActions onSend={() => setShowSend(true)} onReceive={() => setShowReceive(true)} />
        <MultiChainPortfolio />
        <RecentTransactions />
      </main>

      {/* Bottom Navigation */}
      <BottomNav activeTab={activeTab} onTabChange={setActiveTab} />
    </div>
  )
}
