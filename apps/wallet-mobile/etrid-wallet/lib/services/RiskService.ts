import { RiskMetrics } from '../types/analytics'

interface Portfolio {
  assets: Array<{
    symbol: string
    value: number
    returns: number[]
  }>
}

export class RiskService {
  private static instance: RiskService

  static getInstance(): RiskService {
    if (!RiskService.instance) {
      RiskService.instance = new RiskService()
    }
    return RiskService.instance
  }

  async getRiskMetrics(): Promise<RiskMetrics> {
    try {
      const response = await fetch('/api/analytics/risk')
      if (!response.ok) throw new Error('Failed to fetch risk metrics')
      return await response.json()
    } catch (error) {
      console.error('Error fetching risk metrics:', error)
      return this.getMockRiskMetrics()
    }
  }

  async calculateVolatility(asset: string, period: number = 30): Promise<number> {
    try {
      const response = await fetch(
        `/api/analytics/volatility?asset=${asset}&period=${period}`
      )
      if (!response.ok) throw new Error('Failed to calculate volatility')
      const data = await response.json()
      return data.volatility
    } catch (error) {
      console.error('Error calculating volatility:', error)
      // Return mock volatility
      return 0.35
    }
  }

  async calculateSharpeRatio(portfolio: Portfolio): Promise<number> {
    try {
      const response = await fetch('/api/analytics/sharpe', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ portfolio }),
      })
      if (!response.ok) throw new Error('Failed to calculate Sharpe ratio')
      const data = await response.json()
      return data.sharpeRatio
    } catch (error) {
      console.error('Error calculating Sharpe ratio:', error)
      // Calculate locally as fallback
      return this.calculateSharpeRatioLocal(portfolio)
    }
  }

  async calculateBeta(asset: string, benchmark: string = 'BTC'): Promise<number> {
    try {
      const response = await fetch(
        `/api/analytics/beta?asset=${asset}&benchmark=${benchmark}`
      )
      if (!response.ok) throw new Error('Failed to calculate beta')
      const data = await response.json()
      return data.beta
    } catch (error) {
      console.error('Error calculating beta:', error)
      return 1.2 // Mock beta
    }
  }

  async calculateVaR(
    portfolio: Portfolio,
    confidence: number = 0.95
  ): Promise<number> {
    try {
      const response = await fetch('/api/analytics/var', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ portfolio, confidence }),
      })
      if (!response.ok) throw new Error('Failed to calculate VaR')
      const data = await response.json()
      return data.var
    } catch (error) {
      console.error('Error calculating VaR:', error)
      return this.calculateVaRLocal(portfolio, confidence)
    }
  }

  async getMaxDrawdown(portfolio: Portfolio): Promise<number> {
    try {
      const response = await fetch('/api/analytics/max-drawdown', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ portfolio }),
      })
      if (!response.ok) throw new Error('Failed to get max drawdown')
      const data = await response.json()
      return data.maxDrawdown
    } catch (error) {
      console.error('Error getting max drawdown:', error)
      return -15.2 // Mock max drawdown
    }
  }

  // Local calculation methods
  private calculateSharpeRatioLocal(portfolio: Portfolio): number {
    const riskFreeRate = 0.02 // 2% risk-free rate
    const totalValue = portfolio.assets.reduce((sum, asset) => sum + asset.value, 0)

    // Calculate weighted average return
    const portfolioReturn = portfolio.assets.reduce((sum, asset) => {
      const weight = asset.value / totalValue
      const avgReturn =
        asset.returns.reduce((a, b) => a + b, 0) / asset.returns.length
      return sum + weight * avgReturn
    }, 0)

    // Calculate portfolio volatility
    const volatility = this.calculatePortfolioVolatility(portfolio)

    return (portfolioReturn - riskFreeRate) / volatility
  }

  private calculatePortfolioVolatility(portfolio: Portfolio): number {
    const totalValue = portfolio.assets.reduce((sum, asset) => sum + asset.value, 0)

    // Simplified volatility calculation
    const weightedVolatilities = portfolio.assets.map((asset) => {
      const weight = asset.value / totalValue
      const stdDev = this.calculateStdDev(asset.returns)
      return weight * stdDev
    })

    return Math.sqrt(
      weightedVolatilities.reduce((sum, vol) => sum + vol * vol, 0)
    )
  }

  private calculateStdDev(returns: number[]): number {
    const avg = returns.reduce((a, b) => a + b, 0) / returns.length
    const squareDiffs = returns.map((value) => Math.pow(value - avg, 2))
    const avgSquareDiff = squareDiffs.reduce((a, b) => a + b, 0) / returns.length
    return Math.sqrt(avgSquareDiff)
  }

  private calculateVaRLocal(portfolio: Portfolio, confidence: number): number {
    const totalValue = portfolio.assets.reduce((sum, asset) => sum + asset.value, 0)

    // Collect all returns
    const allReturns: number[] = []
    portfolio.assets.forEach((asset) => {
      allReturns.push(...asset.returns)
    })

    // Sort returns
    allReturns.sort((a, b) => a - b)

    // Find percentile
    const index = Math.floor((1 - confidence) * allReturns.length)
    const percentileReturn = allReturns[index] || -0.05

    return totalValue * percentileReturn
  }

  private getMockRiskMetrics(): RiskMetrics {
    return {
      overallRiskScore: 45,
      volatility: 0.35,
      sharpeRatio: 1.8,
      beta: 1.2,
      maxDrawdown: -15.2,
      valueAtRisk: -8420.0,
      diversificationScore: 72,
      recommendations: [
        'Consider increasing Bitcoin allocation for lower volatility',
        'Your portfolio has high correlation with market movements (Beta: 1.2)',
        'Diversification score is good, but could be improved',
        'Risk-adjusted returns (Sharpe) are above average',
      ],
    }
  }
}

export const riskService = RiskService.getInstance()
