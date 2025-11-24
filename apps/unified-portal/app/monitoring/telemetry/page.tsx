'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Globe } from "lucide-react"
import { StatCard } from "@/components/monitoring/stat-card"
import { NetworkMap } from "@/components/monitoring/network-map"
import { NodeExplorer } from "@/components/monitoring/node-explorer"
import { useNetworkStats } from "@/lib/hooks/useNetworkStats"
import { useEffect, useState } from "react"

export default function TelemetryPage() {
  const { stats, nodes, isLoading } = useNetworkStats()
  const [lastUpdate, setLastUpdate] = useState<string>('')
  const [countdown, setCountdown] = useState(10)

  useEffect(() => {
    setLastUpdate(new Date().toLocaleTimeString())

    const interval = setInterval(() => {
      setCountdown(prev => {
        if (prev <= 1) {
          setLastUpdate(new Date().toLocaleTimeString())
          return 10
        }
        return prev - 1
      })
    }, 1000)

    return () => clearInterval(interval)
  }, [])

  const activeNodes = nodes.filter(n => n.status === 'online')
  const activeValidators = nodes.filter(n => n.type === 'validator' && n.status === 'online')

  return (
    <div className="container mx-auto p-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <Globe className="h-8 w-8 text-purple-500" />
              <h1 className="text-4xl font-bold">Network Telemetry</h1>
            </div>
            <p className="text-zinc-600 dark:text-zinc-400">
              Real-time network monitoring and node explorer
            </p>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <div className="w-3 h-3 rounded-full bg-green-500 animate-pulse" />
              <span className="text-sm text-muted-foreground">Network Online</span>
            </div>
            <div className="text-sm text-muted-foreground">
              Last Updated: {lastUpdate}
            </div>
            <div className="text-sm text-muted-foreground">
              Auto-refresh: {countdown}s
            </div>
          </div>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <StatCard
          title="Best Block"
          value={stats.blockHeight.toLocaleString()}
          subtitle={`Finalized: ${stats.finalizedHeight.toLocaleString()}`}
        />
        <StatCard
          title="Total Nodes"
          value={stats.nodeCount}
          subtitle={`${activeNodes.length} active`}
        />
        <StatCard
          title="Validators"
          value={stats.validatorCount}
          subtitle={`${activeValidators.length} producing blocks`}
        />
        <StatCard
          title="Network TPS"
          value={stats.tps}
          subtitle="Transactions/second"
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        <StatCard
          title="Block Time"
          value={`${stats.blockTime}s`}
          subtitle="Target: 5s"
        />
        <StatCard
          title="Finality Time"
          value={`${stats.finalityTime}s`}
          subtitle="ASF Consensus"
        />
        <StatCard
          title="Chain"
          value={stats.chainName}
          subtitle={`Runtime: ${stats.version}`}
        />
      </div>

      {/* Geographic Map */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <span>🌍</span>
            Node Distribution
          </CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="h-[400px] flex items-center justify-center text-muted-foreground">
              Loading network map...
            </div>
          ) : (
            <NetworkMap nodes={nodes} />
          )}
        </CardContent>
      </Card>

      {/* Node List */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <span>📡</span>
            Active Nodes
          </CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center py-8 text-muted-foreground">
              Connecting to network...
            </div>
          ) : (
            <NodeExplorer nodes={nodes} />
          )}
        </CardContent>
      </Card>

      {/* Network Statistics */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <span>📊</span>
            Network Statistics
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div>
              <h4 className="font-semibold mb-4 text-muted-foreground">Chain Info</h4>
              <div className="space-y-3">
                <div className="flex justify-between items-center py-2 border-b border-border">
                  <span className="text-sm text-muted-foreground">Chain Name</span>
                  <span className="font-medium">{stats.chainName}</span>
                </div>
                <div className="flex justify-between items-center py-2 border-b border-border">
                  <span className="text-sm text-muted-foreground">Runtime Version</span>
                  <span className="font-medium font-mono text-sm">{stats.version}</span>
                </div>
                <div className="flex justify-between items-center py-2 border-b border-border">
                  <span className="text-sm text-muted-foreground">Genesis Hash</span>
                  <span className="font-medium font-mono text-xs">{stats.genesisHash}</span>
                </div>
              </div>
            </div>

            <div>
              <h4 className="font-semibold mb-4 text-muted-foreground">Network Health</h4>
              <div className="space-y-3">
                <div className="flex justify-between items-center py-2 border-b border-border">
                  <span className="text-sm text-muted-foreground">Active Peers</span>
                  <span className="font-medium">{activeNodes.length}</span>
                </div>
                <div className="flex justify-between items-center py-2 border-b border-border">
                  <span className="text-sm text-muted-foreground">Sync Status</span>
                  <span className="font-medium">
                    {activeNodes.length > 0 ? 'Synchronized' : 'Syncing...'}
                  </span>
                </div>
                <div className="flex justify-between items-center py-2 border-b border-border">
                  <span className="text-sm text-muted-foreground">Network Uptime</span>
                  <span className="font-medium">99.9%</span>
                </div>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}
