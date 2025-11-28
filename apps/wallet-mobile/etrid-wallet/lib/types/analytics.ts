// Analytics Types

export interface PortfolioMetrics {
  totalValue: number
  dayChange: number
  dayChangePercent: number
  roi: number
  riskScore: number
  diversificationScore: number
}

export interface RiskMetrics {
  overallRiskScore: number
  volatility: number
  sharpeRatio: number
  beta: number
  maxDrawdown: number
  valueAtRisk: number
  diversificationScore: number
  recommendations: string[]
}

export interface PerformanceData {
  totalReturn: number
  totalReturnPercent: number
  cagr: number
  bestAsset: AssetPerformance
  worstAsset: AssetPerformance
  dailyReturns: TimeSeriesData[]
  weeklyReturns: TimeSeriesData[]
  monthlyReturns: TimeSeriesData[]
}

export interface AssetPerformance {
  asset: string
  symbol: string
  return: number
  returnPercent: number
  value: number
}

export interface TimeSeriesData {
  timestamp: number
  value: number
  label?: string
}

export interface AllocationData {
  asset: string
  symbol: string
  value: number
  percentage: number
  color: string
}

export interface TaxLot {
  id: string
  asset: string
  amount: number
  costBasis: number
  purchaseDate: Date
  currentValue: number
}

export interface CapitalGains {
  shortTerm: number
  longTerm: number
  totalGains: number
  totalLosses: number
  netGains: number
  estimatedTax: number
}

export interface TaxLossOpportunity {
  asset: string
  symbol: string
  unrealizedLoss: number
  estimatedTaxSavings: number
  amount: number
  currentPrice: number
  costBasis: number
}

export interface CorrelationData {
  asset1: string
  asset2: string
  correlation: number
}

export interface CorrelationMatrix {
  assets: string[]
  matrix: number[][]
}

export interface BenchmarkComparison {
  benchmark: string
  portfolioReturn: number
  benchmarkReturn: number
  outperformance: number
}

export type TimePeriod = '24h' | '7d' | '30d' | '90d' | '1y' | 'all'
export type CostBasisMethod = 'FIFO' | 'LIFO' | 'HIFO' | 'SpecificID'
export type RiskLevel = 'low' | 'medium' | 'high'
