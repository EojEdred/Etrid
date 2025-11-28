import type {
  BloccardAccount,
  BloccardTransaction,
  BloccardStatus,
  SpendingLimit,
  TransactionFilters,
  ApiResponse,
  PaginatedResponse,
  CardType,
} from "@/lib/types/features"

/**
 * BloccardService - AU Bloccard management
 * Handles card operations, transactions, and settings
 */
export class BloccardService {
  private baseUrl: string

  constructor(baseUrl: string = "/api/bloccard") {
    this.baseUrl = baseUrl
  }

  /**
   * Apply for a new AU Bloccard
   */
  async applyForCard(cardType: CardType = "virtual"): Promise<BloccardAccount> {
    try {
      const response = await fetch(`${this.baseUrl}/apply`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ cardType }),
      })

      const data: ApiResponse<BloccardAccount> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to apply for card")
      }

      return data.data
    } catch (error) {
      console.error("Error applying for card:", error)
      throw error
    }
  }

  /**
   * Get current card status and details
   */
  async getCardStatus(): Promise<BloccardStatus> {
    try {
      const response = await fetch(`${this.baseUrl}/status`)
      const data: ApiResponse<BloccardStatus> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get card status")
      }

      return data.data
    } catch (error) {
      console.error("Error getting card status:", error)
      throw error
    }
  }

  /**
   * Get card account details
   */
  async getCardAccount(): Promise<BloccardAccount> {
    try {
      const response = await fetch(`${this.baseUrl}/account`)
      const data: ApiResponse<BloccardAccount> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get card account")
      }

      return data.data
    } catch (error) {
      console.error("Error getting card account:", error)
      throw error
    }
  }

  /**
   * Freeze the card (prevent all transactions)
   */
  async freezeCard(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/freeze`, {
        method: "POST",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to freeze card")
      }
    } catch (error) {
      console.error("Error freezing card:", error)
      throw error
    }
  }

  /**
   * Unfreeze the card (allow transactions)
   */
  async unfreezeCard(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/unfreeze`, {
        method: "POST",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to unfreeze card")
      }
    } catch (error) {
      console.error("Error unfreezing card:", error)
      throw error
    }
  }

  /**
   * Update spending limits
   */
  async setSpendingLimit(limit: SpendingLimit): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/limits`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(limit),
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to set spending limit")
      }
    } catch (error) {
      console.error("Error setting spending limit:", error)
      throw error
    }
  }

  /**
   * Get current spending limits
   */
  async getSpendingLimits(): Promise<SpendingLimit> {
    try {
      const response = await fetch(`${this.baseUrl}/limits`)
      const data: ApiResponse<SpendingLimit> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get spending limits")
      }

      return data.data
    } catch (error) {
      console.error("Error getting spending limits:", error)
      throw error
    }
  }

  /**
   * Get card transactions with optional filters
   */
  async getTransactions(
    filters?: TransactionFilters,
    page: number = 1,
    pageSize: number = 20,
  ): Promise<PaginatedResponse<BloccardTransaction>> {
    try {
      const params = new URLSearchParams({
        page: page.toString(),
        pageSize: pageSize.toString(),
      })

      if (filters) {
        if (filters.startDate) params.append("startDate", filters.startDate.toISOString())
        if (filters.endDate) params.append("endDate", filters.endDate.toISOString())
        if (filters.category) params.append("category", filters.category)
        if (filters.minAmount) params.append("minAmount", filters.minAmount.toString())
        if (filters.maxAmount) params.append("maxAmount", filters.maxAmount.toString())
        if (filters.merchant) params.append("merchant", filters.merchant)
      }

      const response = await fetch(`${this.baseUrl}/transactions?${params}`)
      const data: ApiResponse<PaginatedResponse<BloccardTransaction>> = await response.json()

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
  async getTransaction(id: string): Promise<BloccardTransaction> {
    try {
      const response = await fetch(`${this.baseUrl}/transactions/${id}`)
      const data: ApiResponse<BloccardTransaction> = await response.json()

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
   * Export transactions to CSV
   */
  async exportTransactions(filters?: TransactionFilters): Promise<Blob> {
    try {
      const params = new URLSearchParams()

      if (filters) {
        if (filters.startDate) params.append("startDate", filters.startDate.toISOString())
        if (filters.endDate) params.append("endDate", filters.endDate.toISOString())
        if (filters.category) params.append("category", filters.category)
        if (filters.minAmount) params.append("minAmount", filters.minAmount.toString())
        if (filters.maxAmount) params.append("maxAmount", filters.maxAmount.toString())
        if (filters.merchant) params.append("merchant", filters.merchant)
      }

      const response = await fetch(`${this.baseUrl}/transactions/export?${params}`)

      if (!response.ok) {
        throw new Error("Failed to export transactions")
      }

      return await response.blob()
    } catch (error) {
      console.error("Error exporting transactions:", error)
      throw error
    }
  }

  /**
   * Get spending analytics
   */
  async getSpendingAnalytics(startDate: Date, endDate: Date): Promise<{
    totalSpent: number
    transactionCount: number
    averageTransaction: number
    categoryBreakdown: Record<string, number>
    merchantBreakdown: Record<string, number>
    dailySpending: { date: string; amount: number }[]
  }> {
    try {
      const params = new URLSearchParams({
        startDate: startDate.toISOString(),
        endDate: endDate.toISOString(),
      })

      const response = await fetch(`${this.baseUrl}/analytics?${params}`)
      const data: ApiResponse<{
        totalSpent: number
        transactionCount: number
        averageTransaction: number
        categoryBreakdown: Record<string, number>
        merchantBreakdown: Record<string, number>
        dailySpending: { date: string; amount: number }[]
      }> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get spending analytics")
      }

      return data.data
    } catch (error) {
      console.error("Error getting spending analytics:", error)
      throw error
    }
  }

  /**
   * Request a physical card (upgrade from virtual)
   */
  async requestPhysicalCard(shippingAddress: {
    line1: string
    line2?: string
    city: string
    state: string
    postalCode: string
    country: string
  }): Promise<{ trackingNumber: string; estimatedDelivery: Date }> {
    try {
      const response = await fetch(`${this.baseUrl}/request-physical`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(shippingAddress),
      })

      const data: ApiResponse<{ trackingNumber: string; estimatedDelivery: Date }> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to request physical card")
      }

      return data.data
    } catch (error) {
      console.error("Error requesting physical card:", error)
      throw error
    }
  }

  /**
   * Update alert settings
   */
  async updateAlertSettings(settings: {
    transactionAlerts: boolean
    largeTransactionThreshold?: number
    lowBalanceAlert: boolean
    lowBalanceThreshold?: number
    healthFactorAlert: boolean
    healthFactorThreshold?: number
  }): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/alerts`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(settings),
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to update alert settings")
      }
    } catch (error) {
      console.error("Error updating alert settings:", error)
      throw error
    }
  }
}

// Export singleton instance
export const bloccardService = new BloccardService()
