"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { TrendingUp, Zap, Network, Activity } from "lucide-react"
import type { NetworkStats } from "@/lib/lightning/types"

interface NetworkStatsProps {
  lightning: any
}

export function NetworkStats({ lightning }: NetworkStatsProps) {
  const stats = lightning.networkStats

  if (!stats) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Network Statistics</CardTitle>
          <CardDescription>Loading...</CardDescription>
        </CardHeader>
      </Card>
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Network Statistics</CardTitle>
        <CardDescription>Lightning Network overview</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-muted-foreground">
            <Network className="h-4 w-4" />
            <span className="text-sm">Total Channels</span>
          </div>
          <div className="text-2xl font-bold">{stats.totalChannels.toLocaleString()}</div>
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-muted-foreground">
            <Zap className="h-4 w-4" />
            <span className="text-sm">Total Capacity</span>
          </div>
          <div className="text-2xl font-bold">{stats.totalCapacity}</div>
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-muted-foreground">
            <Activity className="h-4 w-4" />
            <span className="text-sm">Active Chains</span>
          </div>
          <div className="text-2xl font-bold">{stats.activeChains}/14</div>
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-muted-foreground">
            <TrendingUp className="h-4 w-4" />
            <span className="text-sm">Success Rate</span>
          </div>
          <div className="text-2xl font-bold">{stats.successRate}%</div>
        </div>

        <div className="pt-4 border-t">
          <div className="text-sm text-muted-foreground mb-2">Recent Activity</div>
          <div className="text-3xl font-bold text-primary">
            {stats.recentPayments}
          </div>
          <div className="text-xs text-muted-foreground">payments in last 24h</div>
        </div>
      </CardContent>
    </Card>
  )
}
