"use client"

import { useState } from "react"
import { ArrowLeft, DollarSign, TrendingDown, Repeat, CreditCard } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useFiatRamp } from "@/hooks/useFiatRamp"
import { usePaymentMethods } from "@/hooks/usePaymentMethods"
import { useDCA } from "@/hooks/useDCA"
import { PaymentMethodCard } from "@/components/fiat/PaymentMethodCard"
import { DCAScheduleItem } from "@/components/fiat/DCAScheduleItem"
import { Skeleton } from "@/components/ui/skeleton"

interface FiatRampScreenProps {
  onClose: () => void
  onBuyCrypto: () => void
  onSellCrypto: () => void
  onDCASetup: () => void
  onPaymentMethods: () => void
}

export function FiatRampScreen({
  onClose,
  onBuyCrypto,
  onSellCrypto,
  onDCASetup,
  onPaymentMethods,
}: FiatRampScreenProps) {
  const { transactions, isLoading: rampLoading } = useFiatRamp()
  const { paymentMethods, isLoading: methodsLoading } = usePaymentMethods()
  const { schedules, stats, isLoading: dcaLoading } = useDCA()

  const activeSchedules = schedules.filter((s) => s.isActive)

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="flex items-center justify-between p-6 sticky top-0 glass-strong z-10">
        <Button variant="ghost" size="icon" onClick={onClose}>
          <ArrowLeft className="w-5 h-5" />
        </Button>
        <h1 className="text-xl font-bold">Fiat Ramp</h1>
        <div className="w-10" />
      </header>

      {/* Quick Actions */}
      <div className="px-6 mb-6">
        <div className="grid grid-cols-2 gap-4">
          <Button onClick={onBuyCrypto} className="h-24 flex-col gap-2" size="lg">
            <DollarSign className="w-6 h-6" />
            <span className="font-semibold">Buy Crypto</span>
          </Button>
          <Button onClick={onSellCrypto} variant="outline" className="h-24 flex-col gap-2" size="lg">
            <TrendingDown className="w-6 h-6" />
            <span className="font-semibold">Sell Crypto</span>
          </Button>
        </div>
      </div>

      {/* Main Content */}
      <div className="px-6">
        <Tabs defaultValue="overview" className="w-full">
          <TabsList className="w-full grid grid-cols-3 glass">
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="dca">DCA</TabsTrigger>
            <TabsTrigger value="methods">Methods</TabsTrigger>
          </TabsList>

          {/* Overview Tab */}
          <TabsContent value="overview" className="space-y-6 mt-6">
            {/* Quick Buy Options */}
            <div className="space-y-4">
              <h2 className="font-semibold text-lg">Quick Buy</h2>
              <div className="grid grid-cols-3 gap-3">
                {[100, 500, 1000].map((amount) => (
                  <Button
                    key={amount}
                    variant="outline"
                    onClick={onBuyCrypto}
                    className="h-20 flex-col gap-2 glass hover:glass-strong"
                  >
                    <span className="text-2xl font-bold">${amount}</span>
                    <span className="text-xs text-muted-foreground">ÉTR</span>
                  </Button>
                ))}
              </div>
            </div>

            {/* Recent Transactions */}
            <div className="space-y-4">
              <h2 className="font-semibold text-lg">Recent Transactions</h2>
              {rampLoading ? (
                <div className="space-y-3">
                  {[1, 2, 3].map((i) => (
                    <Skeleton key={i} className="h-20 rounded-2xl" />
                  ))}
                </div>
              ) : transactions.length === 0 ? (
                <div className="glass p-8 rounded-2xl text-center">
                  <DollarSign className="w-12 h-12 mx-auto mb-3 text-muted-foreground" />
                  <p className="text-muted-foreground">No transactions yet</p>
                  <Button onClick={onBuyCrypto} className="mt-4">
                    Buy Your First Crypto
                  </Button>
                </div>
              ) : (
                <div className="space-y-3">
                  {transactions.slice(0, 5).map((tx) => (
                    <div key={tx.id} className="glass p-4 rounded-2xl">
                      <div className="flex items-center justify-between">
                        <div>
                          <p className="font-semibold">
                            {tx.type === "buy" ? "Bought" : "Sold"} {tx.asset}
                          </p>
                          <p className="text-sm text-muted-foreground">
                            {new Date(tx.createdAt).toLocaleDateString()}
                          </p>
                        </div>
                        <div className="text-right">
                          <p className="font-semibold">
                            {tx.type === "buy" ? "+" : "-"}
                            {tx.cryptoAmount} {tx.asset}
                          </p>
                          <p className="text-sm text-muted-foreground">${tx.fiatAmount}</p>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </TabsContent>

          {/* DCA Tab */}
          <TabsContent value="dca" className="space-y-6 mt-6">
            {/* DCA Stats */}
            {stats && (
              <div className="glass p-6 rounded-2xl">
                <h3 className="font-semibold mb-4">DCA Statistics</h3>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-xs text-muted-foreground mb-1">Total Invested</p>
                    <p className="text-2xl font-bold">${stats.totalInvested.toLocaleString()}</p>
                  </div>
                  <div>
                    <p className="text-xs text-muted-foreground mb-1">Total Purchases</p>
                    <p className="text-2xl font-bold">{stats.totalPurchases}</p>
                  </div>
                </div>
              </div>
            )}

            {/* Active Schedules */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h2 className="font-semibold text-lg">Active Schedules</h2>
                <Button onClick={onDCASetup} size="sm">
                  New Schedule
                </Button>
              </div>

              {dcaLoading ? (
                <div className="space-y-3">
                  {[1, 2].map((i) => (
                    <Skeleton key={i} className="h-32 rounded-2xl" />
                  ))}
                </div>
              ) : activeSchedules.length === 0 ? (
                <div className="glass p-8 rounded-2xl text-center">
                  <Repeat className="w-12 h-12 mx-auto mb-3 text-muted-foreground" />
                  <p className="text-muted-foreground mb-2">No DCA schedules</p>
                  <p className="text-sm text-muted-foreground mb-4">
                    Set up recurring purchases to dollar-cost average
                  </p>
                  <Button onClick={onDCASetup}>Create DCA Schedule</Button>
                </div>
              ) : (
                <div className="space-y-3">
                  {activeSchedules.map((schedule) => (
                    <DCAScheduleItem key={schedule.id} schedule={schedule} />
                  ))}
                </div>
              )}
            </div>
          </TabsContent>

          {/* Payment Methods Tab */}
          <TabsContent value="methods" className="space-y-6 mt-6">
            <div className="flex items-center justify-between">
              <h2 className="font-semibold text-lg">Payment Methods</h2>
              <Button onClick={onPaymentMethods} size="sm">
                Add Method
              </Button>
            </div>

            {methodsLoading ? (
              <div className="space-y-3">
                {[1, 2].map((i) => (
                  <Skeleton key={i} className="h-24 rounded-2xl" />
                ))}
              </div>
            ) : paymentMethods.length === 0 ? (
              <div className="glass p-8 rounded-2xl text-center">
                <CreditCard className="w-12 h-12 mx-auto mb-3 text-muted-foreground" />
                <p className="text-muted-foreground mb-2">No payment methods</p>
                <p className="text-sm text-muted-foreground mb-4">
                  Add a card or bank account to buy crypto
                </p>
                <Button onClick={onPaymentMethods}>Add Payment Method</Button>
              </div>
            ) : (
              <div className="space-y-3">
                {paymentMethods.map((method) => (
                  <PaymentMethodCard key={method.id} method={method} />
                ))}
              </div>
            )}
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
