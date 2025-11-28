'use client'

import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Activity, ExternalLink, Search } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { MetricCard } from '@/components/monitoring/metric-card'
import { usePrometheusQuery, formatBytes } from '@/lib/hooks/usePrometheusQuery'

// Common Prometheus queries
const COMMON_QUERIES = [
  { name: 'Block Height', query: 'substrate_block_height{status="best"}', format: 'number' },
  { name: 'Finalized Block', query: 'substrate_block_height{status="finalized"}', format: 'number' },
  { name: 'Peer Count', query: 'substrate_sub_libp2p_peers_count', format: 'number' },
  { name: 'Memory Usage', query: 'process_resident_memory_bytes', format: 'bytes' },
  { name: 'CPU Usage', query: 'rate(process_cpu_seconds_total[5m])', format: 'percent' },
  { name: 'Network In', query: 'rate(substrate_sub_libp2p_network_bytes_total{direction="in"}[5m])', format: 'bytes' },
]

export default function PrometheusPage() {
  const [customQuery, setCustomQuery] = useState('')
  const [queryResults, setQueryResults] = useState<Array<{ query: string; timestamp: string }>>([])

  const handleCustomQuery = () => {
    if (customQuery.trim()) {
      setQueryResults([
        { query: customQuery, timestamp: new Date().toLocaleTimeString() },
        ...queryResults.slice(0, 4) // Keep last 5 queries
      ])
    }
  }

  return (
    <div className="container mx-auto p-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <Activity className="h-8 w-8 text-purple-500" />
              <h1 className="text-4xl font-bold">Prometheus Metrics</h1>
            </div>
            <p className="text-zinc-600 dark:text-zinc-400">
              Raw metrics queries and system performance data
            </p>
          </div>
          <Button
            variant="outline"
            onClick={() => window.open('http://localhost:9090', '_blank')}
          >
            <ExternalLink className="w-4 h-4 mr-2" />
            Open Prometheus UI
          </Button>
        </div>
      </div>

      {/* Quick Metrics */}
      <div className="mb-8">
        <h2 className="text-2xl font-semibold mb-4">Quick Metrics</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {COMMON_QUERIES.map((metric) => (
            <QuickMetricCard key={metric.query} {...metric} />
          ))}
        </div>
      </div>

      {/* Custom Query Interface */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Search className="w-5 h-5" />
            PromQL Query Builder
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex gap-2 mb-4">
            <Input
              placeholder="Enter PromQL query (e.g., substrate_block_height)"
              value={customQuery}
              onChange={(e) => setCustomQuery(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleCustomQuery()}
              className="flex-1 font-mono text-sm"
            />
            <Button onClick={handleCustomQuery}>
              <Search className="w-4 h-4 mr-2" />
              Query
            </Button>
          </div>

          {queryResults.length > 0 && (
            <div className="space-y-2">
              <h4 className="text-sm font-medium text-muted-foreground">Recent Queries</h4>
              {queryResults.map((result, index) => (
                <div
                  key={index}
                  className="flex items-center justify-between p-3 bg-muted/50 rounded-md"
                >
                  <code className="text-xs flex-1">{result.query}</code>
                  <span className="text-xs text-muted-foreground ml-4">{result.timestamp}</span>
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setCustomQuery(result.query)}
                    className="ml-2"
                  >
                    Reuse
                  </Button>
                </div>
              ))}
            </div>
          )}

          <div className="mt-6 p-4 bg-muted/30 rounded-md">
            <h4 className="text-sm font-semibold mb-2">Common PromQL Examples:</h4>
            <ul className="text-sm text-muted-foreground space-y-1">
              <li><code className="bg-muted px-2 py-0.5 rounded">substrate_block_height</code> - Current block height</li>
              <li><code className="bg-muted px-2 py-0.5 rounded">rate(process_cpu_seconds_total[5m])</code> - CPU usage rate</li>
              <li><code className="bg-muted px-2 py-0.5 rounded">substrate_sub_libp2p_peers_count</code> - Connected peers</li>
              <li><code className="bg-muted px-2 py-0.5 rounded">process_resident_memory_bytes</code> - Memory usage</li>
            </ul>
          </div>
        </CardContent>
      </Card>

      {/* Metric Documentation */}
      <Card>
        <CardHeader>
          <CardTitle>Available Metrics</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 className="font-semibold mb-3 text-muted-foreground">Blockchain Metrics</h4>
              <ul className="space-y-2 text-sm">
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_block_height</code>
                  <span className="text-muted-foreground">Block height</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_finality_grandpa_round</code>
                  <span className="text-muted-foreground">GRANDPA round</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_proposer_block_constructed</code>
                  <span className="text-muted-foreground">Blocks constructed</span>
                </li>
              </ul>
            </div>

            <div>
              <h4 className="font-semibold mb-3 text-muted-foreground">Network Metrics</h4>
              <ul className="space-y-2 text-sm">
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_sub_libp2p_peers_count</code>
                  <span className="text-muted-foreground">Peer count</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_sub_libp2p_network_bytes_total</code>
                  <span className="text-muted-foreground">Network traffic</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_sync_peers</code>
                  <span className="text-muted-foreground">Syncing peers</span>
                </li>
              </ul>
            </div>

            <div>
              <h4 className="font-semibold mb-3 text-muted-foreground">System Metrics</h4>
              <ul className="space-y-2 text-sm">
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">process_resident_memory_bytes</code>
                  <span className="text-muted-foreground">Memory usage</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">process_cpu_seconds_total</code>
                  <span className="text-muted-foreground">CPU time</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">process_open_fds</code>
                  <span className="text-muted-foreground">Open file descriptors</span>
                </li>
              </ul>
            </div>

            <div>
              <h4 className="font-semibold mb-3 text-muted-foreground">Database Metrics</h4>
              <ul className="space-y-2 text-sm">
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_state_cache_bytes</code>
                  <span className="text-muted-foreground">State cache size</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_state_db_cache_bytes</code>
                  <span className="text-muted-foreground">DB cache size</span>
                </li>
                <li className="flex justify-between py-1 border-b border-border">
                  <code className="text-xs">substrate_database_cache_bytes</code>
                  <span className="text-muted-foreground">Total cache</span>
                </li>
              </ul>
            </div>
          </div>

          <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-950 rounded-md border border-blue-200 dark:border-blue-800">
            <p className="text-sm text-blue-900 dark:text-blue-100">
              <strong>Note:</strong> Prometheus must be running on localhost:9090 and scraping your ËTRID nodes.
              Configure scrape targets in your prometheus.yml configuration file.
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}

function QuickMetricCard({ name, query, format }: { name: string; query: string; format: string }) {
  const { value, loading, error } = usePrometheusQuery(query)

  let displayValue = value
  if (!loading && !error) {
    if (format === 'bytes') {
      displayValue = formatBytes(value)
    } else if (format === 'percent') {
      const numValue = typeof value === 'string' ? parseFloat(value) : value
      displayValue = `${(numValue * 100).toFixed(2)}%`
    } else if (format === 'number') {
      const numValue = typeof value === 'string' ? parseFloat(value) : value
      displayValue = Math.round(numValue).toLocaleString()
    }
  }

  return (
    <MetricCard
      title={name}
      value={displayValue}
      query={query}
      loading={loading}
      error={error || undefined}
    />
  )
}
