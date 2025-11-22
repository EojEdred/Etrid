import {
  PortfolioMetrics,
  PerformanceData,
  AllocationData,
  TimePeriod,
  TimeSeriesData,
  AssetPerformance,
  BenchmarkComparison,
} from '../types/analytics'

export class AnalyticsService {
  private static instance: AnalyticsService

  static getInstance(): AnalyticsService {
    if (!AnalyticsService.instance) {
      AnalyticsService.instance = new AnalyticsService()
    }
    return AnalyticsService.instance
  }

  async getPortfolioMetrics(): Promise<PortfolioMetrics> {
    try {
      // TODO: Replace with actual API call
      const response = await fetch('/api/analytics/metrics')
      if (!response.ok) throw new Error('Failed to fetch portfolio metrics')
      return await response.json()
    } catch (error) {
      console.error('Error fetching portfolio metrics:', error)
      // Return mock data for development
      return this.getMockPortfolioMetrics()
    }
  }

  async getRiskScore(): Promise<number> {
    try {
      const response = await fetch('/api/analytics/risk-score')
      if (!response.ok) throw new Error('Failed to fetch risk score')
      const data = await response.json()
      return data.riskScore
    } catch (error) {
      console.error('Error fetching risk score:', error)
      return 45 // Mock risk score
    }
  }

  async getPerformance(period: TimePeriod): Promise<PerformanceData> {
    try {
      const response = await fetch(`/api/analytics/performance?period=${period}`)
      if (!response.ok) throw new Error('Failed to fetch performance data')
      return await response.json()
    } catch (error) {
      console.error('Error fetching performance:', error)
      return this.getMockPerformanceData(period)
    }
  }

  async getAssetAllocation(): Promise<AllocationData[]> {
    try {
      const response = await fetch('/api/analytics/allocation')
      if (!response.ok) throw new Error('Failed to fetch asset allocation')
      return await response.json()
    } catch (error) {
      console.error('Error fetching allocation:', error)
      return this.getMockAllocationData()
    }
  }

  async getDiversificationScore(): Promise<number> {
    try {
      const response = await fetch('/api/analytics/diversification')
      if (!response.ok) throw new Error('Failed to fetch diversification score')
      const data = await response.json()
      return data.score
    } catch (error) {
      console.error('Error fetching diversification:', error)
      return 72 // Mock diversification score
    }
  }

  async getBenchmarkComparison(
    period: TimePeriod,
    benchmark: string = 'BTC'
  ): Promise<BenchmarkComparison> {
    try {
      const response = await fetch(
        `/api/analytics/benchmark?period=${period}&benchmark=${benchmark}`
      )
      if (!response.ok) throw new Error('Failed to fetch benchmark comparison')
      return await response.json()
    } catch (error) {
      console.error('Error fetching benchmark:', error)
      return {
        benchmark,
        portfolioReturn: 15.5,
        benchmarkReturn: 12.3,
        outperformance: 3.2,
      }
    }
  }

  // Mock data methods for development
  private getMockPortfolioMetrics(): PortfolioMetrics {
    return {
      totalValue: 125430.5,
      dayChange: 2345.67,
      dayChangePercent: 1.91,
      roi: 23.5,
      riskScore: 45,
      diversificationScore: 72,
    }
  }

  private getMockPerformanceData(period: TimePeriod): PerformanceData {
    const dailyReturns = this.generateMockTimeSeriesData(30)
    const weeklyReturns = this.generateMockTimeSeriesData(12)
    const monthlyReturns = this.generateMockTimeSeriesData(6)

    return {
      totalReturn: 12500.0,
      totalReturnPercent: 23.5,
      cagr: 18.2,
      bestAsset: {
        asset: 'Ëtrid',
        symbol: 'ÉTR',
        return: 5420.0,
        returnPercent: 45.2,
        value: 17420.0,
      },
      worstAsset: {
        asset: 'Polkadot',
        symbol: 'DOT',
        return: -320.0,
        returnPercent: -5.1,
        value: 5980.0,
      },
      dailyReturns,
      weeklyReturns,
      monthlyReturns,
    }
  }

  private getMockAllocationData(): AllocationData[] {
    return [
      {
        asset: 'Ëtrid',
        symbol: 'ÉTR',
        value: 45230.5,
        percentage: 36.1,
        color: '#8b5cf6',
      },
      {
        asset: 'Bitcoin',
        symbol: 'BTC',
        value: 35420.0,
        percentage: 28.2,
        color: '#f59e0b',
      },
      {
        asset: 'Ethereum',
        symbol: 'ETH',
        value: 25340.0,
        percentage: 20.2,
        color: '#3b82f6',
      },
      {
        asset: 'Polkadot',
        symbol: 'DOT',
        value: 12220.0,
        percentage: 9.7,
        color: '#ec4899',
      },
      {
        asset: 'Others',
        symbol: 'MISC',
        value: 7220.0,
        percentage: 5.8,
        color: '#6b7280',
      },
    ]
  }

  private generateMockTimeSeriesData(points: number): TimeSeriesData[] {
    const now = Date.now()
    const data: TimeSeriesData[] = []
    let value = 100000

    for (let i = points - 1; i >= 0; i--) {
      const change = (Math.random() - 0.5) * 2000
      value += change
      data.push({
        timestamp: now - i * 24 * 60 * 60 * 1000,
        value: Math.max(0, value),
      })
    }

    return data
  }
}

export const analyticsService = AnalyticsService.getInstance()
