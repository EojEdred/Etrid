'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Activity } from "lucide-react"
import { HealthCard } from "@/components/monitoring/health-card"
import { ServiceLinkCard } from "@/components/monitoring/service-link-card"
import { useNetworkStats } from "@/lib/hooks/useNetworkStats"

export default function MonitoringPage() {
  const { stats, nodes, isLoading } = useNetworkStats()

  const activeNodes = nodes.filter(n => n.status === 'online').length
  const validators = nodes.filter(n => n.type === 'validator' && n.status === 'online').length

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <div className="flex items-center gap-3 mb-2">
          <Activity className="h-8 w-8 text-purple-500" />
          <h1 className="text-4xl font-bold">Monitoring Overview</h1>
        </div>
        <p className="text-zinc-600 dark:text-zinc-400">
          Real-time network telemetry, metrics, and health dashboards
        </p>
      </div>

      {/* System Health Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <HealthCard
          system="PrimeArc Core Chain"
          status={stats.blockHeight > 0 ? "healthy" : "offline"}
          metrics={{
            block: stats.blockHeight.toLocaleString(),
            validators: stats.validatorCount,
            peers: activeNodes
          }}
          lastUpdate={new Date().toLocaleTimeString()}
        />
        <HealthCard
          system={`${validators} Active Validators`}
          status={validators > 0 ? "healthy" : "warning"}
          metrics={{
            active: validators,
            offline: stats.validatorCount - validators,
            syncing: 0
          }}
          lastUpdate={new Date().toLocaleTimeString()}
        />
        <HealthCard
          system="Network Status"
          status={activeNodes > 0 ? "healthy" : "offline"}
          metrics={{
            tps: stats.tps,
            latency: `${stats.finalityTime}s`,
            uptime: "99.9%"
          }}
          lastUpdate={new Date().toLocaleTimeString()}
        />
      </div>

      {/* Quick Links to Monitoring Tools */}
      <div className="mb-8">
        <h2 className="text-2xl font-semibold mb-4">Monitoring Tools</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <ServiceLinkCard
            title="Network Telemetry"
            description="Live network map and node explorer with real-time metrics"
            href="/monitoring/telemetry"
            icon="🌍"
          />
          <ServiceLinkCard
            title="Grafana Dashboards"
            description="Visual metrics and analytics for validators and chains"
            href="/monitoring/grafana"
            icon="📈"
          />
          <ServiceLinkCard
            title="Prometheus Metrics"
            description="Raw metrics queries and system performance data"
            href="/monitoring/prometheus"
            icon="🔬"
          />
        </div>
      </div>

      {/* Network Statistics */}
      <Card>
        <CardHeader>
          <CardTitle>Network Statistics</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading ? (
            <div className="text-center py-8 text-muted-foreground">
              Loading network data...
            </div>
          ) : (
            <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
              <div>
                <div className="text-sm text-muted-foreground mb-1">Chain Name</div>
                <div className="font-semibold">{stats.chainName}</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Runtime Version</div>
                <div className="font-semibold font-mono text-sm">{stats.version}</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Block Time</div>
                <div className="font-semibold">{stats.blockTime}s</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Finality Time</div>
                <div className="font-semibold">{stats.finalityTime}s</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Best Block</div>
                <div className="font-semibold">{stats.blockHeight.toLocaleString()}</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Finalized Block</div>
                <div className="font-semibold">{stats.finalizedHeight.toLocaleString()}</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Total Nodes</div>
                <div className="font-semibold">{stats.nodeCount}</div>
              </div>
              <div>
                <div className="text-sm text-muted-foreground mb-1">Network TPS</div>
                <div className="font-semibold">{stats.tps}</div>
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
