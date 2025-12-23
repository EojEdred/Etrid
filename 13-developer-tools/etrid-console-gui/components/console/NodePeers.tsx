'use client'

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { useNodePeers } from '@/hooks/useNodePeers'
import { explorerBlock, explorerHome } from '@/lib/explorer'
import { ExternalLink, RefreshCw, Users } from 'lucide-react'

function truncate(value: string, start = 10, end = 6) {
  if (!value) return '—'
  if (value.length <= start + end + 3) return value
  return `${value.slice(0, start)}...${value.slice(-end)}`
}

export function NodePeers({ enabled }: { enabled: boolean }) {
  const { peers, isLoading, error, refresh } = useNodePeers({ enabled })

  return (
    <Card className="glass-card">
      <CardHeader className="flex flex-row items-center justify-between gap-4">
        <CardTitle className="flex items-center gap-2">
          <Users className="w-5 h-5 text-cyan-400" />
          Node Peers
          <span className="text-white/50 text-sm font-normal">({peers.length})</span>
        </CardTitle>
        <div className="flex items-center gap-2">
          <a
            href={explorerHome()}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex"
          >
            <Button variant="outline" size="sm">
              <ExternalLink className="w-4 h-4 mr-2" />
              Explorer
            </Button>
          </a>
          <Button variant="outline" size="sm" onClick={() => void refresh()} disabled={isLoading}>
            <RefreshCw className={`w-4 h-4 mr-2 ${isLoading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
        </div>
      </CardHeader>

      <CardContent>
        {error ? (
          <div className="text-sm text-red-400">{error}</div>
        ) : !enabled ? (
          <div className="text-sm text-white/60">Connect to the node to view peers.</div>
        ) : peers.length === 0 ? (
          <div className="text-sm text-white/60">
            {isLoading ? 'Loading peers…' : 'No peers reported by the node.'}
          </div>
        ) : (
          <div className="space-y-2">
            {peers.slice(0, 25).map((peer) => (
              <div
                key={peer.peerId}
                className="flex flex-col md:flex-row md:items-center md:justify-between gap-2 p-3 rounded-lg bg-white/5 border border-white/10"
              >
                <div className="min-w-0">
                  <div className="font-mono text-sm break-all" title={peer.peerId}>
                    {truncate(peer.peerId, 14, 10)}
                  </div>
                  <div className="text-xs text-white/50">
                    roles: {peer.roles} • protocol: {peer.protocolVersion ?? '—'}
                  </div>
                </div>
                <div className="flex items-center gap-3 text-xs text-white/70">
                  <div className="font-mono">
                    best: {peer.bestNumber ?? '—'}
                  </div>
                  {peer.bestNumber != null ? (
                    <a
                      href={explorerBlock(peer.bestNumber)}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="inline-flex items-center gap-1 hover:underline text-cyan-300"
                      title={peer.bestHash ?? undefined}
                    >
                      View
                      <ExternalLink className="w-3 h-3" />
                    </a>
                  ) : null}
                </div>
              </div>
            ))}
            {peers.length > 25 ? (
              <div className="text-xs text-white/50 pt-2">
                Showing first 25 peers.
              </div>
            ) : null}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
