import type {
  BuyCryptoParams,
  SellCryptoParams,
  Quote,
  PaymentMethod,
  PaymentMethodInput,
  FiatTransaction,
  CryptoAsset,
  FiatCurrency,
  ApiResponse,
  PaginatedResponse,
} from "@/lib/types/features"

/**
 * FiatRampService - Integration with third-party fiat on/off ramp providers
 * Supports MoonPay, Wyre, and Ramp Network
 */
export class FiatRampService {
  private baseUrl: string

  constructor(baseUrl: string = "/api/fiat") {
    this.baseUrl = baseUrl
  }

  /**
   * Get a quote for buying crypto with fiat
   */
  async getQuote(
    amount: number,
    asset: CryptoAsset,
    fiatCurrency: FiatCurrency = "USD",
    isFiatAmount: boolean = true,
  ): Promise<Quote> {
    try {
      const params = new URLSearchParams({
        amount: amount.toString(),
        asset,
        fiatCurrency,
        isFiatAmount: isFiatAmount.toString(),
      })

      const response = await fetch(`${this.baseUrl}/quote?${params}`)
      const data: ApiResponse<Quote> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get quote")
      }

      return data.data
    } catch (error) {
      console.error("Error getting quote:", error)
      throw error
    }
  }

  /**
   * Buy crypto with fiat currency
   */
  async buyCrypto(params: BuyCryptoParams): Promise<FiatTransaction> {
    try {
      const response = await fetch(`${this.baseUrl}/buy`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(params),
      })

      const data: ApiResponse<FiatTransaction> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to buy crypto")
      }

      return data.data
    } catch (error) {
      console.error("Error buying crypto:", error)
      throw error
    }
  }

  /**
   * Sell crypto for fiat currency
   */
  async sellCrypto(params: SellCryptoParams): Promise<FiatTransaction> {
    try {
      const response = await fetch(`${this.baseUrl}/sell`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(params),
      })

      const data: ApiResponse<FiatTransaction> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to sell crypto")
      }

      return data.data
    } catch (error) {
      console.error("Error selling crypto:", error)
      throw error
    }
  }

  /**
   * Get all payment methods for the current user
   */
  async getPaymentMethods(): Promise<PaymentMethod[]> {
    try {
      const response = await fetch(`${this.baseUrl}/payment-methods`)
      const data: ApiResponse<PaymentMethod[]> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get payment methods")
      }

      return data.data
    } catch (error) {
      console.error("Error getting payment methods:", error)
      throw error
    }
  }

  /**
   * Add a new payment method
   */
  async addPaymentMethod(method: PaymentMethodInput): Promise<PaymentMethod> {
    try {
      const response = await fetch(`${this.baseUrl}/payment-methods`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(method),
      })

      const data: ApiResponse<PaymentMethod> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to add payment method")
      }

      return data.data
    } catch (error) {
      console.error("Error adding payment method:", error)
      throw error
    }
  }

  /**
   * Delete a payment method
   */
  async deletePaymentMethod(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/payment-methods/${id}`, {
        method: "DELETE",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to delete payment method")
      }
    } catch (error) {
      console.error("Error deleting payment method:", error)
      throw error
    }
  }

  /**
   * Set a payment method as default
   */
  async setDefaultPaymentMethod(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/payment-methods/${id}/default`, {
        method: "PUT",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to set default payment method")
      }
    } catch (error) {
      console.error("Error setting default payment method:", error)
      throw error
    }
  }

  /**
   * Get transaction history
   */
  async getTransactions(page: number = 1, pageSize: number = 20): Promise<PaginatedResponse<FiatTransaction>> {
    try {
      const params = new URLSearchParams({
        page: page.toString(),
        pageSize: pageSize.toString(),
      })

      const response = await fetch(`${this.baseUrl}/transactions?${params}`)
      const data: ApiResponse<PaginatedResponse<FiatTransaction>> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get transactions")
      }

      return data.data
    } catch (error) {
      console.error("Error getting transactions:", error)
      throw error
    }
  }

  /**
   * Get a specific transaction by ID
   */
  async getTransaction(id: string): Promise<FiatTransaction> {
    try {
      const response = await fetch(`${this.baseUrl}/transactions/${id}`)
      const data: ApiResponse<FiatTransaction> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get transaction")
      }

      return data.data
    } catch (error) {
      console.error("Error getting transaction:", error)
      throw error
    }
  }

  /**
   * Get supported assets
   */
  async getSupportedAssets(): Promise<CryptoAsset[]> {
    try {
      const response = await fetch(`${this.baseUrl}/assets`)
      const data: ApiResponse<CryptoAsset[]> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get supported assets")
      }

      return data.data
    } catch (error) {
      console.error("Error getting supported assets:", error)
      // Return defaults if API fails
      return ["ETR", "BTC", "ETH", "USDT", "USDC"]
    }
  }

  /**
   * Get minimum and maximum limits for buying/selling
   */
  async getLimits(asset: CryptoAsset): Promise<{ minBuy: number; maxBuy: number; minSell: number; maxSell: number }> {
    try {
      const response = await fetch(`${this.baseUrl}/limits/${asset}`)
      const data: ApiResponse<{ minBuy: number; maxBuy: number; minSell: number; maxSell: number }> =
        await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get limits")
      }

      return data.data
    } catch (error) {
      console.error("Error getting limits:", error)
      // Return defaults if API fails
      return { minBuy: 50, maxBuy: 10000, minSell: 10, maxSell: 50000 }
    }
  }
}

// Export singleton instance
export const fiatRampService = new FiatRampService()
