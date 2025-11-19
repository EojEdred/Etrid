'use client'

import { useState, useEffect } from 'react'
import { taxService } from '@/lib/services/TaxService'
import { CapitalGains, TaxLossOpportunity, CostBasisMethod } from '@/lib/types/analytics'

export function useTaxReport(year: number = new Date().getFullYear()) {
  const [capitalGains, setCapitalGains] = useState<CapitalGains | null>(null)
  const [opportunities, setOpportunities] = useState<TaxLossOpportunity[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [costBasisMethod, setCostBasisMethodState] = useState<CostBasisMethod>('FIFO')

  useEffect(() => {
    loadTaxData()
  }, [year, costBasisMethod])

  const loadTaxData = async () => {
    try {
      setLoading(true)
      setError(null)
      const [gains, opps] = await Promise.all([
        taxService.calculateCapitalGains(year),
        taxService.findTaxLossOpportunities(),
      ])
      setCapitalGains(gains)
      setOpportunities(opps)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load tax data')
    } finally {
      setLoading(false)
    }
  }

  const setCostBasisMethod = (method: CostBasisMethod) => {
    taxService.setCostBasisMethod(method)
    setCostBasisMethodState(method)
  }

  const harvestLoss = async (asset: string, amount: number) => {
    try {
      await taxService.harvestLoss(asset, amount)
      await loadTaxData() // Refresh data
    } catch (err) {
      throw err
    }
  }

  const exportToTurboTax = async () => {
    try {
      const blob = await taxService.exportToTurboTax()
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `turbotax-export-${year}.csv`
      a.click()
      URL.revokeObjectURL(url)
    } catch (err) {
      throw err
    }
  }

  const generateForm8949 = async () => {
    try {
      const blob = await taxService.generateForm8949()
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `form-8949-${year}.pdf`
      a.click()
      URL.revokeObjectURL(url)
    } catch (err) {
      throw err
    }
  }

  return {
    capitalGains,
    opportunities,
    loading,
    error,
    costBasisMethod,
    setCostBasisMethod,
    harvestLoss,
    exportToTurboTax,
    generateForm8949,
    refresh: loadTaxData,
  }
}
