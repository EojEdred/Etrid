"use client"

import { useState } from "react"
import {
  ArrowLeft,
  Plus,
  Settings,
  Eye,
  Freeze,
  Lock,
  Unlock,
  TrendingUp,
  History,
  AlertCircle,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { useBloccard } from "@/hooks/useBloccard"
import { useCollateral } from "@/hooks/useCollateral"
import { useBloccardTransactions } from "@/hooks/useBloccardTransactions"
import { BloccardVisual } from "@/components/bloccard/BloccardVisual"
import { CollateralMeter } from "@/components/bloccard/CollateralMeter"
import { SpendingLimitBar } from "@/components/bloccard/SpendingLimitBar"
import { CollateralAssetItem } from "@/components/bloccard/CollateralAssetItem"
import { BloccardTransactionItem } from "@/components/bloccard/BloccardTransactionItem"
import { Skeleton } from "@/components/ui/skeleton"

interface AUBloccardScreenProps {
  onBack: () => void
  onAddCollateral: () => void
  onManageCollateral: () => void
  onSettings: () => void
  onTransactions: () => void
  onSetup: () => void
}

export function AUBloccardScreen({
  onBack,
  onAddCollateral,
  onManageCollateral,
  onSettings,
  onTransactions,
  onSetup,
}: AUBloccardScreenProps) {
  const { account, status, isLoading: cardLoading, freezeCard, unfreezeCard, hasCard } = useBloccard()
  const {
    positions,
    healthFactor,
    totalValue,
    spendingLimit,
    isLoading: collateralLoading,
    getHealthFactorStatus,
  } = useCollateral()
  const { transactions, isLoading: txLoading } = useBloccardTransactions()

  const [isProcessing, setIsProcessing] = useState(false)

  const handleFreeze = async () => {
    setIsProcessing(true)
    try {
      if (account?.status === "frozen") {
        await unfreezeCard()
      } else {
        await freezeCard()
      }
    } finally {
      setIsProcessing(false)
    }
  }

  // Show setup screen if no card
  if (!hasCard() && !cardLoading) {
    return (
      <div className="min-h-screen pb-24 flex items-center justify-center p-6">
        <div className="max-w-md w-full glass p-8 rounded-3xl text-center">
          <div className="w-20 h-20 rounded-full bg-accent/20 flex items-center justify-center mx-auto mb-6">
            <Lock className="w-10 h-10 text-accent" />
          </div>
          <h2 className="text-2xl font-bold mb-3">Get Your AU Bloccard</h2>
          <p className="text-muted-foreground mb-6">
            A secured crypto debit card backed by your collateral. Spend up to 60% of your collateral value.
          </p>
          <div className="space-y-3 mb-6 text-left">
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 rounded-full bg-green-500/20 flex items-center justify-center shrink-0 mt-0.5">
                <span className="text-green-500 text-sm">✓</span>
              </div>
              <div>
                <p className="font-semibold">No Credit Check</p>
                <p className="text-sm text-muted-foreground">Secured by your crypto</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 rounded-full bg-green-500/20 flex items-center justify-center shrink-0 mt-0.5">
                <span className="text-green-500 text-sm">✓</span>
              </div>
              <div>
                <p className="font-semibold">Instant Approval</p>
                <p className="text-sm text-muted-foreground">Start using immediately</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 rounded-full bg-green-500/20 flex items-center justify-center shrink-0 mt-0.5">
                <span className="text-green-500 text-sm">✓</span>
              </div>
              <div>
                <p className="font-semibold">Earn While You Spend</p>
                <p className="text-sm text-muted-foreground">Collateral keeps earning</p>
              </div>
            </div>
          </div>
          <Button onClick={onSetup} size="lg" className="w-full">
            Apply Now
          </Button>
        </div>
      </div>
    )
  }

  if (cardLoading || !account || !status) {
    return (
      <div className="min-h-screen pb-24 p-6 space-y-6">
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-48 w-full rounded-3xl" />
        <Skeleton className="h-32 w-full rounded-2xl" />
        <Skeleton className="h-32 w-full rounded-2xl" />
      </div>
    )
  }

  const healthStatus = getHealthFactorStatus()

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="flex items-center justify-between p-6 sticky top-0 glass-strong z-10">
        <Button variant="ghost" size="icon" onClick={onBack}>
          <ArrowLeft className="w-5 h-5" />
        </Button>
        <h1 className="text-xl font-bold">AU Bloccard</h1>
        <Button variant="ghost" size="icon" onClick={onSettings}>
          <Settings className="w-5 h-5" />
        </Button>
      </header>

      <div className="px-6 space-y-6">
        {/* Health Factor Alert */}
        {healthFactor < 150 && (
          <Alert variant={healthFactor < 120 ? "destructive" : "default"}>
            <AlertCircle className="h-4 w-4" />
            <AlertDescription>{healthStatus.message}</AlertDescription>
          </Alert>
        )}

        {/* Virtual Card */}
        <BloccardVisual account={account} />

        {/* Card Status */}
        <div className="flex items-center justify-between glass p-4 rounded-2xl">
          <div className="flex items-center gap-3">
            {account.status === "frozen" ? (
              <>
                <Freeze className="w-5 h-5 text-blue-500" />
                <div>
                  <p className="font-semibold">Card Frozen</p>
                  <p className="text-sm text-muted-foreground">Transactions disabled</p>
                </div>
              </>
            ) : (
              <>
                <Unlock className="w-5 h-5 text-green-500" />
                <div>
                  <p className="font-semibold">Card Active</p>
                  <p className="text-sm text-muted-foreground">Ready to use</p>
                </div>
              </>
            )}
          </div>
          <Button
            variant={account.status === "frozen" ? "default" : "outline"}
            size="sm"
            onClick={handleFreeze}
            disabled={isProcessing}
          >
            {account.status === "frozen" ? "Unfreeze" : "Freeze"}
          </Button>
        </div>

        {/* Quick Actions */}
        <div className="grid grid-cols-3 gap-3">
          <Button variant="outline" onClick={onAddCollateral} className="flex-col h-20 gap-2">
            <Plus className="w-5 h-5" />
            <span className="text-xs">Add Collateral</span>
          </Button>
          <Button variant="outline" onClick={onManageCollateral} className="flex-col h-20 gap-2">
            <TrendingUp className="w-5 h-5" />
            <span className="text-xs">Manage</span>
          </Button>
          <Button variant="outline" onClick={onTransactions} className="flex-col h-20 gap-2">
            <History className="w-5 h-5" />
            <span className="text-xs">History</span>
          </Button>
        </div>

        {/* Main Content Tabs */}
        <Tabs defaultValue="overview" className="w-full">
          <TabsList className="w-full grid grid-cols-3 glass">
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="collateral">Collateral</TabsTrigger>
            <TabsTrigger value="activity">Activity</TabsTrigger>
          </TabsList>

          {/* Overview Tab */}
          <TabsContent value="overview" className="space-y-6 mt-6">
            {/* Health Factor */}
            <CollateralMeter
              healthFactor={healthFactor}
              collateralValue={totalValue}
              totalSpent={status.totalSpent}
            />

            {/* Spending Limits */}
            <SpendingLimitBar
              spent={status.monthlySpent}
              limit={account.monthlyLimit}
              period="monthly"
            />

            {/* Available Spending */}
            <div className="glass p-6 rounded-2xl">
              <p className="text-sm text-muted-foreground mb-2">Available to Spend</p>
              <p className="text-4xl font-bold mb-4">${status.availableSpending.toLocaleString()}</p>
              <div className="grid grid-cols-2 gap-4 pt-4 border-t border-border">
                <div>
                  <p className="text-xs text-muted-foreground mb-1">Collateral Value</p>
                  <p className="text-lg font-semibold">${totalValue.toLocaleString()}</p>
                </div>
                <div className="text-right">
                  <p className="text-xs text-muted-foreground mb-1">LTV Ratio</p>
                  <p className="text-lg font-semibold">60%</p>
                </div>
              </div>
            </div>
          </TabsContent>

          {/* Collateral Tab */}
          <TabsContent value="collateral" className="space-y-4 mt-6">
            {collateralLoading ? (
              <div className="space-y-3">
                {[1, 2, 3].map((i) => (
                  <Skeleton key={i} className="h-20 rounded-2xl" />
                ))}
              </div>
            ) : positions.length === 0 ? (
              <div className="glass p-8 rounded-2xl text-center">
                <Lock className="w-12 h-12 mx-auto mb-3 text-muted-foreground" />
                <p className="text-muted-foreground mb-2">No collateral</p>
                <Button onClick={onAddCollateral} className="mt-4">
                  Add Collateral
                </Button>
              </div>
            ) : (
              positions.map((position) => (
                <CollateralAssetItem
                  key={position.id}
                  asset={position.asset}
                  amount={position.amount}
                  valueUsd={position.valueUsd}
                  percentage={(position.valueUsd / totalValue) * 100}
                  onClick={onManageCollateral}
                />
              ))
            )}
          </TabsContent>

          {/* Activity Tab */}
          <TabsContent value="activity" className="space-y-4 mt-6">
            {txLoading ? (
              <div className="space-y-3">
                {[1, 2, 3].map((i) => (
                  <Skeleton key={i} className="h-24 rounded-2xl" />
                ))}
              </div>
            ) : transactions.length === 0 ? (
              <div className="glass p-8 rounded-2xl text-center">
                <History className="w-12 h-12 mx-auto mb-3 text-muted-foreground" />
                <p className="text-muted-foreground">No transactions yet</p>
              </div>
            ) : (
              transactions.slice(0, 10).map((tx) => <BloccardTransactionItem key={tx.id} transaction={tx} />)
            )}
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
