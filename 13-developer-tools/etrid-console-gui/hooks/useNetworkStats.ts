'use client'

import { useEffect, useState } from 'react'
import { usePolkadotApi } from './usePolkadotApi'
import { getApi, getRpcEndpoint } from '@/lib/polkadot/api'
import { PRIMEARC_CORE } from '@/lib/chains/config'

export interface NetworkStats {
  blockHeight: number
  finalizedHeight: number
  validatorCount: number
  nodeCount: number
  peerCount: number
  tps: number
  blockTime: number
  finalityTime: number
  chainName: string
  version: string
  genesisHash: string
}

export interface NetworkNode {
  name: string
  type: 'bootstrap' | 'validator' | 'full-node'
  location: string
  status: 'online' | 'offline' | 'syncing'
  version: string
  block: number
  peers: number
  uptime: string
  lat: number
  lon: number
}

const DEFAULT_NODE_LOCATION = { lat: 0, lon: 0 }

export function useNetworkStats() {
  const { isConnected } = usePolkadotApi()
  const [stats, setStats] = useState<NetworkStats>({
    blockHeight: 0,
    finalizedHeight: 0,
    validatorCount: 0,
    nodeCount: 0,
    peerCount: 0,
    tps: 0,
    blockTime: 0,
    finalityTime: 0,
    chainName: PRIMEARC_CORE.name,
    version: '',
    genesisHash: ''
  })
  const [nodes, setNodes] = useState<NetworkNode[]>([
    {
      name: 'Primearc Core RPC',
      type: 'bootstrap',
      location: PRIMEARC_CORE.wsEndpoint,
      status: 'offline',
      version: '',
      block: 0,
      peers: 0,
      uptime: 'N/A',
      ...DEFAULT_NODE_LOCATION,
    },
  ])
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    if (!isConnected) {
      setNodes((prev) =>
        prev.map((n) => ({
          ...n,
          status: 'offline',
        }))
      )
      setIsLoading(false)
      return
    }

    let isMounted = true; // To prevent state updates after unmount

    async function fetchStats() {
      try {
        const api = getApi()
        if (!api || !isMounted) return

        const [header, finalizedHash, chain, version, health] = await Promise.all([
          api.rpc.chain.getHeader(),
          api.rpc.chain.getFinalizedHead(),
          api.rpc.system.chain(),
          api.rpc.system.version(),
          api.rpc.system.health(),
        ])

        if (!isMounted) return; // Check again after async operations

        const finalizedHeader = await api.rpc.chain.getHeader(finalizedHash)

        let validatorCount = 0
        try {
          const validators = await api.query.session.validators()
          const validatorsJson = validators.toJSON()
          validatorCount = Array.isArray(validatorsJson) ? validatorsJson.length : 0
        } catch (e) {
          // Use default
        }

        if (isMounted) {
          const blockHeight = header.number.toNumber()
          const finalizedHeight = finalizedHeader.number.toNumber()
          const peers = health.peers.toNumber()
          const isSyncing =
            typeof (health as any).isSyncing === 'boolean'
              ? (health as any).isSyncing
              : Boolean((health as any).isSyncing?.isTrue)
          const nodeCount = peers > 0 ? peers + 1 : 1

          let blockTimeSeconds = 0
          let tps = 0

          try {
            if (blockHeight > 1) {
              const [currentHash, prevHash] = await Promise.all([
                api.rpc.chain.getBlockHash(blockHeight),
                api.rpc.chain.getBlockHash(blockHeight - 1),
              ])

              const [currentTimestamp, prevTimestamp, block] = await Promise.all([
                api.query.timestamp.now.at(currentHash),
                api.query.timestamp.now.at(prevHash),
                api.rpc.chain.getBlock(currentHash),
              ])

              const deltaMs = Number(currentTimestamp.toString()) - Number(prevTimestamp.toString())
              if (Number.isFinite(deltaMs) && deltaMs > 0) {
                blockTimeSeconds = deltaMs / 1000
                const extrinsics = block.block.extrinsics.length
                tps = extrinsics / blockTimeSeconds
              }
            }
          } catch {
            // Optional metrics; keep defaults.
          }

          const finalityLag = Math.max(0, blockHeight - finalizedHeight)
          const finalityTime = blockTimeSeconds > 0 ? finalityLag * blockTimeSeconds : 0

          const connectedEndpoint = getRpcEndpoint() || PRIMEARC_CORE.wsEndpoint

          setStats({
            blockHeight,
            finalizedHeight,
            validatorCount,
            nodeCount,
            peerCount: peers,
            tps,
            blockTime: blockTimeSeconds,
            finalityTime,
            chainName: chain.toString(),
            version: version.toString(),
            genesisHash: api.genesisHash.toHex()
          })

          setNodes([
            {
              name: 'Primearc Core RPC',
              type: 'bootstrap',
              location: connectedEndpoint,
              status: isSyncing ? 'syncing' : 'online',
              version: version.toString(),
              block: blockHeight,
              peers,
              uptime: 'N/A',
              ...DEFAULT_NODE_LOCATION,
            },
          ])

          setIsLoading(false)
        }
      } catch (error) {
        console.error('Error fetching network stats:', error)
        if (isMounted) {
          setIsLoading(false)
        }
      }
    }

    fetchStats()
    const interval = setInterval(fetchStats, 10000) // Update every 10 seconds

    return () => {
      clearInterval(interval)
      isMounted = false
    }
  }, [isConnected])

  return { stats, nodes, isLoading }
}
