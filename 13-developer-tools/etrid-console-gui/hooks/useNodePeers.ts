'use client'

import { useCallback, useEffect, useRef, useState } from 'react'
import { getApi } from '@/lib/polkadot/api'

export type NodePeer = {
  peerId: string
  roles: string
  bestNumber: number | null
  bestHash: string | null
  protocolVersion: number | null
}

function toNumber(value: unknown): number | null {
  if (typeof value === 'number' && Number.isFinite(value)) return value
  if (typeof value === 'string') {
    const n = Number(value)
    return Number.isFinite(n) ? n : null
  }
  if (typeof (value as any)?.toNumber === 'function') {
    try {
      const n = (value as any).toNumber()
      return typeof n === 'number' && Number.isFinite(n) ? n : null
    } catch {
      return null
    }
  }
  return null
}

function pickString(...values: unknown[]): string {
  for (const value of values) {
    if (typeof value === 'string' && value.trim()) return value
  }
  return ''
}

function normalizePeer(peer: any): NodePeer {
  const json = typeof peer?.toJSON === 'function' ? peer.toJSON() : peer
  const human = typeof peer?.toHuman === 'function' ? peer.toHuman() : null

  const peerId = pickString(json?.peerId, json?.peer_id, human?.peerId, human?.peer_id)
  const roles = pickString(
    json?.roles,
    human?.roles,
    typeof json?.roles === 'number' ? String(json.roles) : '',
  )

  const bestHash = pickString(json?.bestHash, json?.best_hash, human?.bestHash, human?.best_hash) || null
  const bestNumber =
    toNumber(json?.bestNumber) ??
    toNumber(json?.best_number) ??
    toNumber(human?.bestNumber) ??
    toNumber(human?.best_number)

  const protocolVersion =
    toNumber(json?.protocolVersion) ??
    toNumber(json?.protocol_version) ??
    toNumber(human?.protocolVersion) ??
    toNumber(human?.protocol_version)

  return {
    peerId,
    roles: roles || '—',
    bestNumber,
    bestHash,
    protocolVersion,
  }
}

export async function fetchNodePeers(): Promise<NodePeer[]> {
  const api = getApi()
  const result = await (api.rpc.system as any).peers?.()
  if (!Array.isArray(result)) {
    throw new Error('Peers RPC is not available on this node.')
  }
  const normalized = result.map(normalizePeer).filter((p: NodePeer) => p.peerId)
  normalized.sort((a: NodePeer, b: NodePeer) => (b.bestNumber ?? 0) - (a.bestNumber ?? 0))
  return normalized
}

export function useNodePeers(options?: { pollIntervalMs?: number; enabled?: boolean }) {
  const enabled = options?.enabled ?? false
  const pollIntervalMs = options?.pollIntervalMs ?? 10_000

  const [peers, setPeers] = useState<NodePeer[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const lastUpdatedRef = useRef<number | null>(null)

  const refresh = useCallback(async () => {
    if (!enabled) return
    setIsLoading(true)
    setError(null)

    try {
      const normalized = await fetchNodePeers()
      setPeers(normalized)
      lastUpdatedRef.current = Date.now()
    } catch (e) {
      setPeers([])
      setError(e instanceof Error ? e.message : 'Failed to load peers')
    } finally {
      setIsLoading(false)
    }
  }, [enabled])

  useEffect(() => {
    if (!enabled) {
      setPeers([])
      setIsLoading(false)
      setError(null)
      return
    }

    void refresh()
    const id = setInterval(() => void refresh(), pollIntervalMs)
    return () => clearInterval(id)
  }, [enabled, pollIntervalMs, refresh])

  return {
    peers,
    isLoading,
    error,
    lastUpdated: lastUpdatedRef.current,
    refresh,
  }
}
