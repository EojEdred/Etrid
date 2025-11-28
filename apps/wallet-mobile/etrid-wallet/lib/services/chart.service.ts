// Chart Service - Handle price data and indicators

import type {
  Candle,
  CandleInterval,
  ChartData,
  Trade,
  IndicatorType,
  TradingPair,
  TickerData,
} from '@/lib/types/trading'

export interface Subscription {
  unsubscribe: () => void
}

export class ChartService {
  private apiUrl: string
  private wsUrl: string

  constructor(
    apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000/api',
    wsUrl: string = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3000/ws'
  ) {
    this.apiUrl = apiUrl
    this.wsUrl = wsUrl
  }

  /**
   * Get candlestick data
   */
  async getCandles(
    pair: string,
    interval: CandleInterval,
    limit: number = 500
  ): Promise<Candle[]> {
    try {
      const params = new URLSearchParams({
        pair,
        interval,
        limit: limit.toString(),
      })

      const response = await fetch(`${this.apiUrl}/chart/candles?${params}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch candles: ${response.statusText}`)
      }

      const data = await response.json()
      return data.candles || []
    } catch (error) {
      console.error('Error fetching candles:', error)
      throw error
    }
  }

  /**
   * Subscribe to real-time trades
   */
  subscribeToTrades(pair: string, callback: (trade: Trade) => void): Subscription {
    const ws = new WebSocket(`${this.wsUrl}/trades/${pair}`)

    ws.onmessage = (event) => {
      try {
        const trade = JSON.parse(event.data)
        callback(trade)
      } catch (error) {
        console.error('Error parsing trade:', error)
      }
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    return {
      unsubscribe: () => ws.close(),
    }
  }

  /**
   * Subscribe to real-time candles
   */
  subscribeToCandles(
    pair: string,
    interval: CandleInterval,
    callback: (candle: Candle) => void
  ): Subscription {
    const ws = new WebSocket(`${this.wsUrl}/candles/${pair}/${interval}`)

    ws.onmessage = (event) => {
      try {
        const candle = JSON.parse(event.data)
        callback(candle)
      } catch (error) {
        console.error('Error parsing candle:', error)
      }
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
    }

    return {
      unsubscribe: () => ws.close(),
    }
  }

  /**
   * Get ticker data for a pair
   */
  async getTicker(pair: string): Promise<TickerData> {
    try {
      const response = await fetch(`${this.apiUrl}/chart/ticker/${pair}`)

      if (!response.ok) {
        throw new Error(`Failed to fetch ticker: ${response.statusText}`)
      }

      const data = await response.json()
      return data
    } catch (error) {
      console.error('Error fetching ticker:', error)
      throw error
    }
  }

  /**
   * Get all trading pairs
   */
  async getTradingPairs(): Promise<TradingPair[]> {
    try {
      const response = await fetch(`${this.apiUrl}/chart/pairs`)

      if (!response.ok) {
        throw new Error(`Failed to fetch trading pairs: ${response.statusText}`)
      }

      const data = await response.json()
      return data.pairs || []
    } catch (error) {
      console.error('Error fetching trading pairs:', error)
      throw error
    }
  }

  /**
   * Calculate Simple Moving Average (SMA)
   */
  calculateSMA(candles: Candle[], period: number): number[] {
    const closes = candles.map((c) => c.close)
    const sma: number[] = []

    for (let i = 0; i < closes.length; i++) {
      if (i < period - 1) {
        sma.push(NaN)
        continue
      }

      const sum = closes.slice(i - period + 1, i + 1).reduce((a, b) => a + b, 0)
      sma.push(sum / period)
    }

    return sma
  }

  /**
   * Calculate Exponential Moving Average (EMA)
   */
  calculateEMA(candles: Candle[], period: number): number[] {
    const closes = candles.map((c) => c.close)
    const ema: number[] = []
    const multiplier = 2 / (period + 1)

    // First EMA is SMA
    let sum = 0
    for (let i = 0; i < period; i++) {
      if (i < closes.length) {
        sum += closes[i]
      }
    }
    ema[period - 1] = sum / period

    // Calculate EMA for remaining values
    for (let i = period; i < closes.length; i++) {
      ema[i] = (closes[i] - ema[i - 1]) * multiplier + ema[i - 1]
    }

    // Fill initial values with NaN
    for (let i = 0; i < period - 1; i++) {
      ema[i] = NaN
    }

    return ema
  }

  /**
   * Calculate Relative Strength Index (RSI)
   */
  calculateRSI(candles: Candle[], period: number = 14): number[] {
    const closes = candles.map((c) => c.close)
    const rsi: number[] = []

    let gains = 0
    let losses = 0

    // Calculate initial average gain/loss
    for (let i = 1; i <= period; i++) {
      const change = closes[i] - closes[i - 1]
      if (change > 0) {
        gains += change
      } else {
        losses -= change
      }
    }

    let avgGain = gains / period
    let avgLoss = losses / period

    // Fill initial values
    for (let i = 0; i < period; i++) {
      rsi.push(NaN)
    }

    // Calculate RSI
    rsi.push(100 - 100 / (1 + avgGain / avgLoss))

    for (let i = period + 1; i < closes.length; i++) {
      const change = closes[i] - closes[i - 1]
      const gain = change > 0 ? change : 0
      const loss = change < 0 ? -change : 0

      avgGain = (avgGain * (period - 1) + gain) / period
      avgLoss = (avgLoss * (period - 1) + loss) / period

      const rs = avgGain / avgLoss
      rsi.push(100 - 100 / (1 + rs))
    }

    return rsi
  }

  /**
   * Calculate MACD (Moving Average Convergence Divergence)
   */
  calculateMACD(
    candles: Candle[],
    fastPeriod: number = 12,
    slowPeriod: number = 26,
    signalPeriod: number = 9
  ): { macd: number[]; signal: number[]; histogram: number[] } {
    const fastEMA = this.calculateEMA(candles, fastPeriod)
    const slowEMA = this.calculateEMA(candles, slowPeriod)

    // Calculate MACD line
    const macd = fastEMA.map((fast, i) => fast - slowEMA[i])

    // Calculate signal line (EMA of MACD)
    const macdCandles = macd.map((value, i) => ({
      timestamp: candles[i].timestamp,
      open: value,
      high: value,
      low: value,
      close: value,
      volume: 0,
    }))
    const signal = this.calculateEMA(macdCandles, signalPeriod)

    // Calculate histogram
    const histogram = macd.map((m, i) => m - signal[i])

    return { macd, signal, histogram }
  }

  /**
   * Calculate Bollinger Bands
   */
  calculateBollingerBands(
    candles: Candle[],
    period: number = 20,
    stdDev: number = 2
  ): { upper: number[]; middle: number[]; lower: number[] } {
    const closes = candles.map((c) => c.close)
    const middle = this.calculateSMA(candles, period)
    const upper: number[] = []
    const lower: number[] = []

    for (let i = 0; i < closes.length; i++) {
      if (i < period - 1) {
        upper.push(NaN)
        lower.push(NaN)
        continue
      }

      const slice = closes.slice(i - period + 1, i + 1)
      const mean = middle[i]
      const variance =
        slice.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / period
      const standardDeviation = Math.sqrt(variance)

      upper.push(mean + stdDev * standardDeviation)
      lower.push(mean - stdDev * standardDeviation)
    }

    return { upper, middle, lower }
  }

  /**
   * Get indicator data
   */
  async getIndicatorData(
    pair: string,
    indicator: IndicatorType,
    params: Record<string, number>
  ): Promise<number[]> {
    try {
      const candles = await this.getCandles(pair, '1h', 500)

      switch (indicator) {
        case 'SMA':
          return this.calculateSMA(candles, params.period || 20)
        case 'EMA':
          return this.calculateEMA(candles, params.period || 20)
        case 'RSI':
          return this.calculateRSI(candles, params.period || 14)
        case 'MACD': {
          const { macd } = this.calculateMACD(candles, params.fast || 12, params.slow || 26, params.signal || 9)
          return macd
        }
        case 'BOLLINGER_BANDS': {
          const { middle } = this.calculateBollingerBands(candles, params.period || 20, params.stdDev || 2)
          return middle
        }
        default:
          throw new Error(`Unsupported indicator: ${indicator}`)
      }
    } catch (error) {
      console.error('Error calculating indicator:', error)
      throw error
    }
  }
}

// Export singleton instance
export const chartService = new ChartService()
