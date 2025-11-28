'use client'

import { useEffect, useState } from 'react'

interface PrometheusResult {
  value: string | number
  loading: boolean
  error: string | null
}

export function usePrometheusQuery(query: string, refreshInterval = 5000): PrometheusResult {
  const [value, setValue] = useState<string | number>('--')
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function fetchMetric() {
      try {
        setLoading(true)
        setError(null)

        const response = await fetch(
          `http://localhost:9090/api/v1/query?query=${encodeURIComponent(query)}`,
          { cache: 'no-store' }
        )

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }

        const data = await response.json()

        if (data.status === 'success' && data.data.result.length > 0) {
          setValue(data.data.result[0].value[1])
        } else {
          setValue('N/A')
        }

        setLoading(false)
      } catch (err) {
        console.error('Prometheus query error:', err)
        setError(err instanceof Error ? err.message : 'Failed to fetch metric')
        setValue('Error')
        setLoading(false)
      }
    }

    fetchMetric()
    const interval = setInterval(fetchMetric, refreshInterval)

    return () => clearInterval(interval)
  }, [query, refreshInterval])

  return { value, loading, error }
}

export function formatBytes(bytes: string | number): string {
  if (typeof bytes === 'string') {
    bytes = parseFloat(bytes)
  }

  if (isNaN(bytes)) return 'N/A'

  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let i = 0

  while (bytes >= 1024 && i < units.length - 1) {
    bytes /= 1024
    i++
  }

  return `${bytes.toFixed(2)} ${units[i]}`
}
