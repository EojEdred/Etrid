"use client"

import { useState } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useWallet } from "@/lib/polkadot/useWallet"
import Link from "next/link"
import { ArrowRight } from "lucide-react"

// Import staking components from app directory
import ValidatorBrowser from "./validator-browser"
import NominatorDashboard from "./nominator-dashboard"
import RewardsTracker from "./rewards-tracker"
import NominationManager from "./nomination-manager"
import ApyCalculator from "./apy-calculator"

export default function StakingPage() {
  const wallet = useWallet()
  const [activeTab, setActiveTab] = useState("validator-browser")

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-8">
        <div className="mb-8">
          <h1 className="text-4xl font-bold mb-4">Staking & Rewards</h1>
          <p className="text-xl text-muted-foreground max-w-2xl">
            Stake your tokens to earn rewards and secure the network
          </p>
        </div>

        {wallet.error && (
          <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg text-destructive">
            {wallet.error}
          </div>
        )}

        {/* Quick Links */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
          <Link href="/wallet/staking/eth-pbc">
            <Card className="hover:border-primary transition-colors cursor-pointer">
              <CardHeader>
                <CardTitle className="flex items-center justify-between">
                  ETH PBC Staking
                  <ArrowRight className="h-5 w-5" />
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  Stake LP tokens on Ethereum Partition Burst Chain
                </p>
              </CardContent>
            </Card>
          </Link>

          {/* Add more PBC chains as they become available */}
        </div>

        {/* Main Staking Interface */}
        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="grid w-full grid-cols-5">
            <TabsTrigger value="validator-browser">Validators</TabsTrigger>
            <TabsTrigger value="nominator-dashboard">Dashboard</TabsTrigger>
            <TabsTrigger value="nomination-manager">Manage</TabsTrigger>
            <TabsTrigger value="rewards-tracker">Rewards</TabsTrigger>
            <TabsTrigger value="apy-calculator">APY Calc</TabsTrigger>
          </TabsList>

          <TabsContent value="validator-browser" className="mt-6">
            <ValidatorBrowser />
          </TabsContent>

          <TabsContent value="nominator-dashboard" className="mt-6">
            <NominatorDashboard />
          </TabsContent>

          <TabsContent value="nomination-manager" className="mt-6">
            <NominationManager />
          </TabsContent>

          <TabsContent value="rewards-tracker" className="mt-6">
            <RewardsTracker />
          </TabsContent>

          <TabsContent value="apy-calculator" className="mt-6">
            <ApyCalculator />
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
