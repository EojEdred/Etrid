'use client'

import { useState, useEffect } from 'react'
import { metaverseService } from '@/lib/services/MetaverseService'
import {
  VirtualAsset,
  LandNFT,
  WearableNFT,
  Gallery,
  MetaverseEvent,
} from '@/lib/types/metaverse'

export function useMetaverse() {
  const [virtualAssets, setVirtualAssets] = useState<VirtualAsset[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadVirtualAssets()
  }, [])

  const loadVirtualAssets = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await metaverseService.getVirtualAssets()
      setVirtualAssets(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load virtual assets')
    } finally {
      setLoading(false)
    }
  }

  return {
    virtualAssets,
    loading,
    error,
    refresh: loadVirtualAssets,
  }
}

export function useLandNFTs() {
  const [landNFTs, setLandNFTs] = useState<LandNFT[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadLandNFTs()
  }, [])

  const loadLandNFTs = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await metaverseService.getLandNFTs()
      setLandNFTs(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load land NFTs')
    } finally {
      setLoading(false)
    }
  }

  return {
    landNFTs,
    loading,
    error,
    refresh: loadLandNFTs,
  }
}

export function useWearables() {
  const [wearables, setWearables] = useState<WearableNFT[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadWearables()
  }, [])

  const loadWearables = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await metaverseService.getWearables()
      setWearables(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load wearables')
    } finally {
      setLoading(false)
    }
  }

  const equipWearable = async (id: string) => {
    try {
      await metaverseService.equipWearable(id)
      setWearables((prev) =>
        prev.map((w) => (w.id === id ? { ...w, equipped: true } : w))
      )
    } catch (err) {
      throw err
    }
  }

  const unequipWearable = async (id: string) => {
    try {
      await metaverseService.unequipWearable(id)
      setWearables((prev) =>
        prev.map((w) => (w.id === id ? { ...w, equipped: false } : w))
      )
    } catch (err) {
      throw err
    }
  }

  return {
    wearables,
    loading,
    error,
    equipWearable,
    unequipWearable,
    refresh: loadWearables,
  }
}

export function useVirtualGallery() {
  const [galleries, setGalleries] = useState<Gallery[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadGalleries()
  }, [])

  const loadGalleries = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await metaverseService.getGalleries()
      setGalleries(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load galleries')
    } finally {
      setLoading(false)
    }
  }

  const createGallery = async (nftIds: string[], name: string) => {
    try {
      const newGallery = await metaverseService.createGallery(nftIds, name)
      setGalleries((prev) => [...prev, newGallery])
      return newGallery
    } catch (err) {
      throw err
    }
  }

  return {
    galleries,
    loading,
    error,
    createGallery,
    refresh: loadGalleries,
  }
}

export function useMetaverseEvents() {
  const [events, setEvents] = useState<MetaverseEvent[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadEvents()
  }, [])

  const loadEvents = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await metaverseService.getEvents()
      setEvents(data)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load events')
    } finally {
      setLoading(false)
    }
  }

  const rsvpEvent = async (eventId: string) => {
    try {
      await metaverseService.rsvpEvent(eventId)
      setEvents((prev) =>
        prev.map((e) =>
          e.id === eventId
            ? { ...e, userRSVP: true, rsvpCount: (e.rsvpCount || 0) + 1 }
            : e
        )
      )
    } catch (err) {
      throw err
    }
  }

  return {
    events,
    loading,
    error,
    rsvpEvent,
    refresh: loadEvents,
  }
}
