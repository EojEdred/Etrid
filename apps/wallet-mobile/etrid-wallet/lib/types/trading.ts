// Trading Type Definitions

export type OrderType = 'market' | 'limit' | 'stop_loss' | 'stop_limit'
export type OrderSide = 'buy' | 'sell'
export type OrderStatus = 'pending' | 'open' | 'filled' | 'partially_filled' | 'cancelled' | 'expired'
export type TimeInForce = 'GTC' | 'IOC' | 'FOK' // Good Till Cancel, Immediate or Cancel, Fill or Kill

export interface Order {
  id: string
  user_id: string
  pair: string
  type: OrderType
  side: OrderSide
  amount: number
  price?: number // undefined for market orders
  stop_price?: number // for stop orders
  filled_amount: number
  status: OrderStatus
  time_in_force: TimeInForce
  created_at: string
  updated_at: string
  filled_at?: string
  cancelled_at?: string
}

export interface OrderInput {
  pair: string
  type: OrderType
  side: OrderSide
  amount: number
  price?: number
  stop_price?: number
  time_in_force?: TimeInForce
  slippage_tolerance?: number // percentage
}

export interface Trade {
  id: string
  pair: string
  price: number
  amount: number
  side: OrderSide
  timestamp: string
  tx_hash?: string
}

export interface OrderBook {
  pair: string
  bids: OrderBookLevel[]
  asks: OrderBookLevel[]
  spread: number
  spread_percentage: number
  last_updated: string
}

export interface OrderBookLevel {
  price: number
  amount: number
  total: number // cumulative
  orders_count: number
}

export interface Candle {
  timestamp: number
  open: number
  high: number
  low: number
  close: number
  volume: number
}

export type CandleInterval = '1m' | '5m' | '15m' | '30m' | '1h' | '4h' | '1d' | '1w' | '1M'

export interface ChartData {
  pair: string
  interval: CandleInterval
  candles: Candle[]
}

export type ChartType = 'candlestick' | 'line' | 'area' | 'bars'

export type IndicatorType =
  | 'SMA'
  | 'EMA'
  | 'RSI'
  | 'MACD'
  | 'BOLLINGER_BANDS'
  | 'VOLUME'
  | 'STOCHASTIC'
  | 'FIBONACCI'

export interface IndicatorConfig {
  type: IndicatorType
  params: Record<string, number>
  visible: boolean
  color?: string
}

export interface SMAConfig {
  period: number
}

export interface EMAConfig {
  period: number
}

export interface RSIConfig {
  period: number
  overbought: number
  oversold: number
}

export interface MACDConfig {
  fast_period: number
  slow_period: number
  signal_period: number
}

export interface BollingerBandsConfig {
  period: number
  std_dev: number
}

export interface Position {
  id: string
  pair: string
  side: OrderSide
  entry_price: number
  current_price: number
  amount: number
  unrealized_pnl: number
  unrealized_pnl_percentage: number
  take_profit?: number
  stop_loss?: number
  leverage?: number
  opened_at: string
}

export interface Alert {
  id: string
  user_id: string
  pair: string
  condition: AlertCondition
  target_price?: number
  indicator_condition?: IndicatorCondition
  status: 'active' | 'triggered' | 'cancelled'
  triggered_at?: string
  created_at: string
}

export type AlertCondition = 'above' | 'below' | 'crosses_above' | 'crosses_below'

export interface IndicatorCondition {
  indicator: IndicatorType
  condition: AlertCondition
  value: number
}

export interface AlertInput {
  pair: string
  condition: AlertCondition
  target_price?: number
  indicator_condition?: IndicatorCondition
}

export interface DCABot {
  id: string
  user_id: string
  pair: string
  amount_per_order: number
  frequency: 'hourly' | 'daily' | 'weekly'
  time_restriction?: TimeRestriction
  price_deviation_limit?: number // percentage
  active: boolean
  next_run: string
  total_invested: number
  total_tokens: number
  average_price: number
  created_at: string
  last_run?: string
}

export interface TimeRestriction {
  start_hour: number // 0-23
  end_hour: number // 0-23
  days?: number[] // 0-6 (Sunday-Saturday)
}

export interface DCABotInput {
  pair: string
  amount_per_order: number
  frequency: 'hourly' | 'daily' | 'weekly'
  time_restriction?: TimeRestriction
  price_deviation_limit?: number
}

export interface OrderFilters {
  pair?: string
  status?: OrderStatus
  side?: OrderSide
  from_date?: string
  to_date?: string
  limit?: number
  offset?: number
}

export interface TradingPair {
  symbol: string
  base_currency: string
  quote_currency: string
  price: number
  change_24h: number
  change_24h_percentage: number
  high_24h: number
  low_24h: number
  volume_24h: number
  last_updated: string
}

export interface TickerData {
  pair: string
  price: number
  change_24h: number
  change_24h_percentage: number
  high_24h: number
  low_24h: number
  volume_24h: number
  timestamp: string
}
