"use client"

import { useState } from "react"
import { Settings, User, Briefcase, Store } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
} from "@/components/ui/dropdown-menu"
import { BalanceCard } from "@/components/balance-card"
import { QuickActions } from "@/components/quick-actions"
import { RecentTransactions } from "@/components/recent-transactions"
import { BusinessMerchantNav } from "@/components/business-merchant-nav"
import { SendScreen } from "@/components/send-screen"
import { ReceiveScreen } from "@/components/receive-screen"
import { GovernanceScreen } from "@/components/governance-screen"
import { StakingScreen } from "@/components/staking-screen"
import { MultiChainPortfolio } from "@/components/multi-chain-portfolio"

// Business Screens
import { BusinessDashboardScreen } from "@/components/business-dashboard-screen"
import { TeamManagementScreen } from "@/components/team-management-screen"
import { InvoicingScreen } from "@/components/invoicing-screen"
import { PayrollScreen } from "@/components/payroll-screen"
import { ExpenseTrackingScreen } from "@/components/expense-tracking-screen"

// Merchant Screens
import { MerchantDashboardScreen } from "@/components/merchant-dashboard-screen"
import { POSScreen } from "@/components/pos-screen"
import { PaymentLinksScreen } from "@/components/payment-links-screen"
import { ProductCatalogScreen } from "@/components/product-catalog-screen"
import { RefundsScreen } from "@/components/refunds-screen"

type Mode = "personal" | "business" | "merchant"

export function UnifiedWalletHome() {
  const [activeTab, setActiveTab] = useState("home")
  const [mode, setMode] = useState<Mode>("personal")
  const [showSend, setShowSend] = useState(false)
  const [showReceive, setShowReceive] = useState(false)

  // Personal mode navigation
  if (showSend) {
    return <SendScreen onClose={() => setShowSend(false)} />
  }

  if (showReceive) {
    return <ReceiveScreen onClose={() => setShowReceive(false)} />
  }

  if (mode === "personal") {
    if (activeTab === "governance") {
      return <GovernanceScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
    }

    if (activeTab === "stake") {
      return <StakingScreen onBack={() => setActiveTab("home")} activeTab={activeTab} onTabChange={setActiveTab} />
    }
  }

  // Business mode navigation
  if (mode === "business") {
    if (activeTab === "business") {
      return (
        <BusinessDashboardScreen
          onBack={() => setActiveTab("home")}
          onCreateInvoice={() => setActiveTab("invoices")}
          onRunPayroll={() => setActiveTab("payroll")}
          onAddTeam={() => setActiveTab("team")}
          onNavigate={setActiveTab}
        />
      )
    }

    if (activeTab === "team") {
      return <TeamManagementScreen onBack={() => setActiveTab("business")} />
    }

    if (activeTab === "invoices") {
      return (
        <InvoicingScreen
          onBack={() => setActiveTab("business")}
          onCreate={() => {}}
          onView={() => {}}
        />
      )
    }

    if (activeTab === "payroll") {
      return (
        <PayrollScreen
          onBack={() => setActiveTab("business")}
          onCreate={() => {}}
          onSettings={() => {}}
        />
      )
    }

    if (activeTab === "expenses") {
      return (
        <ExpenseTrackingScreen
          onBack={() => setActiveTab("business")}
          onAdd={() => {}}
        />
      )
    }
  }

  // Merchant mode navigation
  if (mode === "merchant") {
    if (activeTab === "merchant") {
      return (
        <MerchantDashboardScreen
          onBack={() => setActiveTab("home")}
          onNewSale={() => setActiveTab("pos")}
          onPaymentLink={() => setActiveTab("links")}
          onNavigate={setActiveTab}
        />
      )
    }

    if (activeTab === "pos") {
      return <POSScreen onBack={() => setActiveTab("merchant")} />
    }

    if (activeTab === "products") {
      return (
        <ProductCatalogScreen
          onBack={() => setActiveTab("merchant")}
          onCreate={() => {}}
          onEdit={() => {}}
        />
      )
    }

    if (activeTab === "links") {
      return (
        <PaymentLinksScreen
          onBack={() => setActiveTab("merchant")}
          onCreate={() => {}}
        />
      )
    }

    if (activeTab === "refunds") {
      return <RefundsScreen onBack={() => setActiveTab("merchant")} />
    }
  }

  // Home screen (shown for all modes when activeTab === "home")
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

        <div className="flex items-center gap-2">
          {/* Mode Switcher */}
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon" className="text-muted-foreground hover:text-foreground">
                {mode === "business" ? (
                  <Briefcase className="w-5 h-5" />
                ) : mode === "merchant" ? (
                  <Store className="w-5 h-5" />
                ) : (
                  <User className="w-5 h-5" />
                )}
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => setMode("personal")}>
                <User className="w-4 h-4 mr-2" />
                Personal Wallet
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={() => setMode("business")}>
                <Briefcase className="w-4 h-4 mr-2" />
                Business Account
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setMode("merchant")}>
                <Store className="w-4 h-4 mr-2" />
                Merchant Tools
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>

          <Button variant="ghost" size="icon" className="text-muted-foreground hover:text-foreground">
            <Settings className="w-5 h-5" />
          </Button>
        </div>
      </header>

      {/* Main Content */}
      <main className="px-6 space-y-6">
        <BalanceCard />
        <QuickActions onSend={() => setShowSend(true)} onReceive={() => setShowReceive(true)} />
        <MultiChainPortfolio />
        <RecentTransactions />
      </main>

      {/* Bottom Navigation */}
      <BusinessMerchantNav activeTab={activeTab} onTabChange={setActiveTab} mode={mode} />
    </div>
  )
}
