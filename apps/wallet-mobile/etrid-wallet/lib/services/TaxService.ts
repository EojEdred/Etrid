import {
  TaxLot,
  CapitalGains,
  TaxLossOpportunity,
  CostBasisMethod,
} from '../types/analytics'

interface Transaction {
  id: string
  type: 'buy' | 'sell'
  asset: string
  amount: number
  price: number
  timestamp: Date
}

export class TaxService {
  private static instance: TaxService
  private costBasisMethod: CostBasisMethod = 'FIFO'

  static getInstance(): TaxService {
    if (!TaxService.instance) {
      TaxService.instance = new TaxService()
    }
    return TaxService.instance
  }

  setCostBasisMethod(method: CostBasisMethod): void {
    this.costBasisMethod = method
  }

  async getTaxLots(asset: string): Promise<TaxLot[]> {
    try {
      const response = await fetch(`/api/analytics/tax/lots?asset=${asset}`)
      if (!response.ok) throw new Error('Failed to fetch tax lots')
      return await response.json()
    } catch (error) {
      console.error('Error fetching tax lots:', error)
      return this.getMockTaxLots(asset)
    }
  }

  async calculateCapitalGains(year: number): Promise<CapitalGains> {
    try {
      const response = await fetch(
        `/api/analytics/tax/capital-gains?year=${year}&method=${this.costBasisMethod}`
      )
      if (!response.ok) throw new Error('Failed to calculate capital gains')
      return await response.json()
    } catch (error) {
      console.error('Error calculating capital gains:', error)
      return this.getMockCapitalGains()
    }
  }

  async findTaxLossOpportunities(): Promise<TaxLossOpportunity[]> {
    try {
      const response = await fetch('/api/analytics/tax/loss-harvest')
      if (!response.ok) throw new Error('Failed to find tax loss opportunities')
      return await response.json()
    } catch (error) {
      console.error('Error finding tax loss opportunities:', error)
      return this.getMockTaxLossOpportunities()
    }
  }

  async harvestLoss(asset: string, amount: number): Promise<Transaction> {
    try {
      const response = await fetch('/api/analytics/tax/harvest', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ asset, amount }),
      })
      if (!response.ok) throw new Error('Failed to harvest loss')
      return await response.json()
    } catch (error) {
      console.error('Error harvesting loss:', error)
      throw error
    }
  }

  async exportToTurboTax(): Promise<Blob> {
    try {
      const response = await fetch('/api/analytics/tax/export/turbotax')
      if (!response.ok) throw new Error('Failed to export to TurboTax')
      return await response.blob()
    } catch (error) {
      console.error('Error exporting to TurboTax:', error)
      throw error
    }
  }

  async generateForm8949(): Promise<Blob> {
    try {
      const response = await fetch('/api/analytics/tax/form8949')
      if (!response.ok) throw new Error('Failed to generate Form 8949')
      return await response.blob()
    } catch (error) {
      console.error('Error generating Form 8949:', error)
      throw error
    }
  }

  // Local calculation methods
  calculateTaxSavings(loss: number, taxRate: number = 0.24): number {
    return Math.abs(loss) * taxRate
  }

  isWashSaleViolation(
    sellDate: Date,
    purchaseDates: Date[]
  ): boolean {
    const thirtyDaysAgo = new Date(sellDate)
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30)

    const thirtyDaysLater = new Date(sellDate)
    thirtyDaysLater.setDate(thirtyDaysLater.getDate() + 30)

    return purchaseDates.some(
      (date) => date >= thirtyDaysAgo && date <= thirtyDaysLater
    )
  }

  // Mock data methods
  private getMockTaxLots(asset: string): TaxLot[] {
    return [
      {
        id: '1',
        asset,
        amount: 100,
        costBasis: 4500,
        purchaseDate: new Date('2023-01-15'),
        currentValue: 5200,
      },
      {
        id: '2',
        asset,
        amount: 50,
        costBasis: 2800,
        purchaseDate: new Date('2023-06-20'),
        currentValue: 2600,
      },
      {
        id: '3',
        asset,
        amount: 75,
        costBasis: 3200,
        purchaseDate: new Date('2023-09-10'),
        currentValue: 3900,
      },
    ]
  }

  private getMockCapitalGains(): CapitalGains {
    const shortTerm = 4200
    const longTerm = 8500
    const totalGains = 15800
    const totalLosses = 3100
    const netGains = totalGains - totalLosses
    const estimatedTax = shortTerm * 0.24 + longTerm * 0.15

    return {
      shortTerm,
      longTerm,
      totalGains,
      totalLosses,
      netGains,
      estimatedTax,
    }
  }

  private getMockTaxLossOpportunities(): TaxLossOpportunity[] {
    return [
      {
        asset: 'Polkadot',
        symbol: 'DOT',
        unrealizedLoss: -420.0,
        estimatedTaxSavings: 100.8, // 24% tax rate
        amount: 50,
        currentPrice: 52.0,
        costBasis: 60.4,
      },
      {
        asset: 'Cardano',
        symbol: 'ADA',
        unrealizedLoss: -180.0,
        estimatedTaxSavings: 43.2,
        amount: 1000,
        currentPrice: 0.42,
        costBasis: 0.6,
      },
    ]
  }
}

export const taxService = TaxService.getInstance()
