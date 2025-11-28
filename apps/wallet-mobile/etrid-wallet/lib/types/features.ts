// ============================================================================
// FIAT RAMP TYPES
// ============================================================================

export type FiatCurrency = "USD" | "EUR" | "GBP" | "CAD" | "AUD"
export type CryptoAsset = "ETR" | "BTC" | "ETH" | "USDT" | "USDC"
export type TransactionStatus = "pending" | "processing" | "completed" | "failed" | "cancelled"
export type PaymentMethodType = "card" | "bank" | "ach" | "wire"
export type DCAFrequency = "daily" | "weekly" | "biweekly" | "monthly"

export interface Quote {
  id: string
  asset: CryptoAsset
  fiatCurrency: FiatCurrency
  cryptoAmount: number
  fiatAmount: number
  exchangeRate: number
  fee: number
  total: number
  expiresAt: Date
  provider: "moonpay" | "wyre" | "ramp"
}

export interface PaymentMethod {
  id: string
  userId: string
  type: PaymentMethodType
  provider: string // e.g., "visa", "mastercard", "chase", "bofa"
  last4: string
  expiryMonth?: number
  expiryYear?: number
  isDefault: boolean
  billingAddress?: BillingAddress
  createdAt: Date
  verificationStatus: "pending" | "verified" | "failed"
}

export interface BillingAddress {
  line1: string
  line2?: string
  city: string
  state: string
  postalCode: string
  country: string
}

export interface BuyCryptoParams {
  asset: CryptoAsset
  fiatAmount: number
  fiatCurrency: FiatCurrency
  paymentMethodId: string
  walletAddress: string
}

export interface SellCryptoParams {
  asset: CryptoAsset
  cryptoAmount: number
  fiatCurrency: FiatCurrency
  paymentMethodId: string // bank account to receive funds
}

export interface FiatTransaction {
  id: string
  userId: string
  type: "buy" | "sell"
  asset: CryptoAsset
  cryptoAmount: number
  fiatAmount: number
  fiatCurrency: FiatCurrency
  fee: number
  status: TransactionStatus
  paymentMethodId: string
  transactionHash?: string
  createdAt: Date
  completedAt?: Date
  failureReason?: string
}

export interface DCASchedule {
  id: string
  userId: string
  asset: CryptoAsset
  amountUsd: number
  frequency: DCAFrequency
  paymentMethodId: string
  isActive: boolean
  nextRunDate: Date
  lastRunDate?: Date
  startDate: Date
  endDate?: Date
  totalPurchased: number
  purchaseCount: number
  createdAt: Date
}

export interface PaymentMethodInput {
  type: PaymentMethodType
  cardNumber?: string
  expiryMonth?: number
  expiryYear?: number
  cvv?: string
  routingNumber?: string
  accountNumber?: string
  billingAddress: BillingAddress
}

// ============================================================================
// AU BLOCCARD TYPES
// ============================================================================

export type BloccardStatus = "active" | "frozen" | "pending_activation" | "suspended" | "closed"
export type CardType = "virtual" | "physical"
export type TransactionCategory =
  | "groceries"
  | "dining"
  | "shopping"
  | "travel"
  | "entertainment"
  | "gas"
  | "utilities"
  | "other"

export interface BloccardAccount {
  id: string
  userId: string
  cardNumber: string
  cardholderName: string
  expiryMonth: number
  expiryYear: number
  cvv: string
  cardType: CardType
  status: BloccardStatus
  dailyLimit: number
  weeklyLimit: number
  monthlyLimit: number
  createdAt: Date
  activatedAt?: Date
  lastUsedAt?: Date
}

export interface CollateralPosition {
  id: string
  accountId: string
  asset: CryptoAsset
  amount: number
  valueUsd: number
  lockedAt: Date
  canWithdraw: boolean
}

export interface BloccardTransaction {
  id: string
  accountId: string
  merchant: string
  merchantCategory: TransactionCategory
  amountUsd: number
  description?: string
  timestamp: Date
  location?: string
  status: "pending" | "completed" | "declined"
  declineReason?: string
}

export interface SpendingLimit {
  daily?: number
  weekly?: number
  monthly?: number
  perTransaction?: number
}

export interface HealthFactorSimulation {
  currentHealthFactor: number
  newHealthFactor: number
  currentCollateralValue: number
  newCollateralValue: number
  spendingLimit: number
  newSpendingLimit: number
  isWithdrawalSafe: boolean
  recommendedMinCollateral: number
}

export interface BloccardStatus {
  account: BloccardAccount
  collateral: CollateralPosition[]
  totalCollateralValue: number
  totalSpent: number
  healthFactor: number
  availableSpending: number
  monthlySpent: number
  weeklySpent: number
  dailySpent: number
}

export interface CollateralAssetInput {
  asset: CryptoAsset
  amount: number
}

export interface TransactionFilters {
  startDate?: Date
  endDate?: Date
  category?: TransactionCategory
  minAmount?: number
  maxAmount?: number
  merchant?: string
}

// ============================================================================
// SHARED TYPES
// ============================================================================

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
  message?: string
}

export interface PaginatedResponse<T> {
  items: T[]
  total: number
  page: number
  pageSize: number
  hasMore: boolean
}

export interface LoadingState {
  isLoading: boolean
  error: string | null
}
