import type {
  CollateralPosition,
  CollateralAssetInput,
  HealthFactorSimulation,
  CryptoAsset,
  ApiResponse,
} from "@/lib/types/features"

/**
 * CollateralService - Collateral management for AU Bloccard
 * Handles adding/withdrawing collateral and health factor calculations
 */
export class CollateralService {
  private baseUrl: string
  private readonly LTV_RATIO = 0.6 // 60% loan-to-value
  private readonly SAFE_HEALTH_FACTOR = 150
  private readonly WARNING_HEALTH_FACTOR = 120
  private readonly DANGER_HEALTH_FACTOR = 100

  constructor(baseUrl: string = "/api/bloccard/collateral") {
    this.baseUrl = baseUrl
  }

  /**
   * Get current collateral positions
   */
  async getCollateral(): Promise<CollateralPosition[]> {
    try {
      const response = await fetch(`${this.baseUrl}`)
      const data: ApiResponse<CollateralPosition[]> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get collateral")
      }

      return data.data
    } catch (error) {
      console.error("Error getting collateral:", error)
      throw error
    }
  }

  /**
   * Add collateral to the account
   */
  async addCollateral(asset: CryptoAsset, amount: number): Promise<CollateralPosition> {
    try {
      const response = await fetch(`${this.baseUrl}/add`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ asset, amount }),
      })

      const data: ApiResponse<CollateralPosition> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to add collateral")
      }

      return data.data
    } catch (error) {
      console.error("Error adding collateral:", error)
      throw error
    }
  }

  /**
   * Withdraw collateral from the account
   */
  async withdrawCollateral(asset: CryptoAsset, amount: number): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/withdraw`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ asset, amount }),
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to withdraw collateral")
      }
    } catch (error) {
      console.error("Error withdrawing collateral:", error)
      throw error
    }
  }

  /**
   * Calculate current health factor
   */
  async calculateHealthFactor(): Promise<number> {
    try {
      const response = await fetch(`${this.baseUrl}/health-factor`)
      const data: ApiResponse<{ healthFactor: number }> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to calculate health factor")
      }

      return data.data.healthFactor
    } catch (error) {
      console.error("Error calculating health factor:", error)
      throw error
    }
  }

  /**
   * Calculate health factor locally
   */
  calculateHealthFactorLocal(collateralValueUsd: number, totalSpent: number): number {
    if (totalSpent === 0) return Infinity
    return (collateralValueUsd / totalSpent) * 100
  }

  /**
   * Get available spending limit based on collateral
   */
  async getSpendingLimit(): Promise<number> {
    try {
      const response = await fetch(`${this.baseUrl}/spending-limit`)
      const data: ApiResponse<{ spendingLimit: number; availableLimit: number }> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get spending limit")
      }

      return data.data.availableLimit
    } catch (error) {
      console.error("Error getting spending limit:", error)
      throw error
    }
  }

  /**
   * Calculate spending limit locally
   */
  calculateSpendingLimit(collateralValueUsd: number, totalSpent: number): number {
    const maxSpending = collateralValueUsd * this.LTV_RATIO
    return Math.max(0, maxSpending - totalSpent)
  }

  /**
   * Simulate withdrawal to see impact on health factor
   */
  async simulateWithdrawal(asset: CryptoAsset, amount: number): Promise<HealthFactorSimulation> {
    try {
      const response = await fetch(`${this.baseUrl}/simulate-withdrawal`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ asset, amount }),
      })

      const data: ApiResponse<HealthFactorSimulation> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to simulate withdrawal")
      }

      return data.data
    } catch (error) {
      console.error("Error simulating withdrawal:", error)
      throw error
    }
  }

  /**
   * Check if withdrawal is safe (won't trigger liquidation)
   */
  isWithdrawalSafe(simulation: HealthFactorSimulation): boolean {
    return simulation.newHealthFactor >= this.SAFE_HEALTH_FACTOR
  }

  /**
   * Get health factor status
   */
  getHealthFactorStatus(healthFactor: number): {
    status: "safe" | "warning" | "danger" | "critical"
    color: string
    message: string
  } {
    if (healthFactor >= this.SAFE_HEALTH_FACTOR) {
      return {
        status: "safe",
        color: "text-green-500",
        message: "Your collateral is healthy",
      }
    } else if (healthFactor >= this.WARNING_HEALTH_FACTOR) {
      return {
        status: "warning",
        color: "text-yellow-500",
        message: "Consider adding more collateral",
      }
    } else if (healthFactor >= this.DANGER_HEALTH_FACTOR) {
      return {
        status: "danger",
        color: "text-orange-500",
        message: "Add collateral immediately to avoid card freeze",
      }
    } else {
      return {
        status: "critical",
        color: "text-red-500",
        message: "CRITICAL: Card frozen. Add collateral now",
      }
    }
  }

  /**
   * Get total collateral value in USD
   */
  async getTotalCollateralValue(): Promise<number> {
    try {
      const collateral = await this.getCollateral()
      return collateral.reduce((total, position) => total + position.valueUsd, 0)
    } catch (error) {
      console.error("Error getting total collateral value:", error)
      throw error
    }
  }

  /**
   * Get recommended minimum collateral based on spending
   */
  getRecommendedMinCollateral(totalSpent: number): number {
    // Recommended: maintain 150% health factor
    return (totalSpent * this.SAFE_HEALTH_FACTOR) / 100
  }

  /**
   * Get collateral breakdown by asset
   */
  async getCollateralBreakdown(): Promise<
    Array<{
      asset: CryptoAsset
      amount: number
      valueUsd: number
      percentage: number
    }>
  > {
    try {
      const collateral = await this.getCollateral()
      const totalValue = collateral.reduce((sum, pos) => sum + pos.valueUsd, 0)

      return collateral.map((pos) => ({
        asset: pos.asset,
        amount: pos.amount,
        valueUsd: pos.valueUsd,
        percentage: totalValue > 0 ? (pos.valueUsd / totalValue) * 100 : 0,
      }))
    } catch (error) {
      console.error("Error getting collateral breakdown:", error)
      throw error
    }
  }

  /**
   * Check if user can withdraw specific amount
   */
  async canWithdraw(asset: CryptoAsset, amount: number): Promise<{ canWithdraw: boolean; reason?: string }> {
    try {
      const simulation = await this.simulateWithdrawal(asset, amount)

      if (simulation.newHealthFactor < this.DANGER_HEALTH_FACTOR) {
        return {
          canWithdraw: false,
          reason: "Withdrawal would bring health factor below safe threshold",
        }
      }

      return { canWithdraw: true }
    } catch (error) {
      console.error("Error checking withdrawal:", error)
      return { canWithdraw: false, reason: "Unable to verify withdrawal safety" }
    }
  }

  /**
   * Get asset prices (for local calculations)
   */
  async getAssetPrices(): Promise<Record<CryptoAsset, number>> {
    try {
      const response = await fetch(`${this.baseUrl}/prices`)
      const data: ApiResponse<Record<CryptoAsset, number>> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get asset prices")
      }

      return data.data
    } catch (error) {
      console.error("Error getting asset prices:", error)
      throw error
    }
  }

  /**
   * Constants
   */
  get ltvRatio(): number {
    return this.LTV_RATIO
  }

  get safeHealthFactor(): number {
    return this.SAFE_HEALTH_FACTOR
  }

  get warningHealthFactor(): number {
    return this.WARNING_HEALTH_FACTOR
  }

  get dangerHealthFactor(): number {
    return this.DANGER_HEALTH_FACTOR
  }
}

// Export singleton instance
export const collateralService = new CollateralService()
