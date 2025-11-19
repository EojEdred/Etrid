/**
 * CollateralService - Manage collateral for loans
 */

import type { Transaction } from './lending-service'

export interface CollateralAsset {
  asset: string
  amount: number
  valueUSD: number
  liquidationThreshold: number // percentage (e.g., 80 = 80%)
}

export interface HealthFactorResult {
  healthFactor: number
  status: 'safe' | 'warning' | 'danger' | 'liquidation'
  liquidationRisk: number // 0-100
}

export class CollateralService {
  private static instance: CollateralService

  // Liquidation thresholds by asset (what % of collateral value can be borrowed)
  private liquidationThresholds: Record<string, number> = {
    'ÉTR': 80, // Can borrow up to 80% of ETR collateral value
    'BTC': 85, // Can borrow up to 85% of BTC collateral value
    'ETH': 82,
    'USDT': 90,
    'USDC': 90,
  }

  static getInstance(): CollateralService {
    if (!CollateralService.instance) {
      CollateralService.instance = new CollateralService()
    }
    return CollateralService.instance
  }

  /**
   * Add collateral to an existing loan
   */
  async addCollateral(
    loanId: string,
    asset: string,
    amount: number
  ): Promise<Transaction> {
    try {
      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // In production, this would interact with smart contract
      // to add collateral to the loan

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to add collateral: ${error}`)
    }
  }

  /**
   * Remove collateral from a loan (if health factor allows)
   */
  async removeCollateral(
    loanId: string,
    asset: string,
    amount: number
  ): Promise<Transaction> {
    try {
      // In production, calculate new health factor first
      // and ensure it stays above liquidation threshold

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to remove collateral: ${error}`)
    }
  }

  /**
   * Calculate health factor
   * Formula: (Total Collateral Value × Liquidation Threshold) / Total Borrowed Value × 100
   *
   * Example:
   * - Collateral: $10,000 worth of ETH (threshold 82%)
   * - Borrowed: $5,000
   * - Health Factor = (10,000 × 0.82) / 5,000 × 100 = 164%
   *
   * Safe: >200%
   * Warning: 150-200%
   * Danger: 120-150%
   * Liquidation: <120%
   */
  async calculateHealthFactor(
    collateralValue: number,
    borrowedValue: number,
    collateralAssets?: CollateralAsset[]
  ): Promise<HealthFactorResult> {
    if (borrowedValue === 0) {
      return {
        healthFactor: Infinity,
        status: 'safe',
        liquidationRisk: 0,
      }
    }

    let effectiveCollateralValue = collateralValue

    // If collateral assets are provided, apply liquidation thresholds
    if (collateralAssets && collateralAssets.length > 0) {
      effectiveCollateralValue = collateralAssets.reduce((sum, c) => {
        const threshold = c.liquidationThreshold / 100
        return sum + (c.valueUSD * threshold)
      }, 0)
    } else {
      // Default 80% threshold if no specific assets provided
      effectiveCollateralValue = collateralValue * 0.8
    }

    const healthFactor = (effectiveCollateralValue / borrowedValue) * 100

    let status: 'safe' | 'warning' | 'danger' | 'liquidation'
    let liquidationRisk: number

    if (healthFactor >= 200) {
      status = 'safe'
      liquidationRisk = 0
    } else if (healthFactor >= 150) {
      status = 'warning'
      liquidationRisk = 25
    } else if (healthFactor >= 120) {
      status = 'danger'
      liquidationRisk = 60
    } else {
      status = 'liquidation'
      liquidationRisk = 100
    }

    return {
      healthFactor,
      status,
      liquidationRisk,
    }
  }

  /**
   * Calculate liquidation price for a specific collateral asset
   * This is the price at which the collateral would trigger liquidation
   */
  async getLiquidationPrice(
    loanId: string,
    collateralAsset: string,
    collateralAmount: number,
    borrowedValue: number
  ): Promise<number> {
    try {
      const threshold = this.liquidationThresholds[collateralAsset] || 80

      // Liquidation happens when:
      // (collateralAmount × liquidationPrice × threshold) / borrowedValue = 1.2 (120%)
      //
      // Solving for liquidationPrice:
      // liquidationPrice = (borrowedValue × 1.2) / (collateralAmount × threshold)

      const liquidationPrice = (borrowedValue * 1.2) / (collateralAmount * (threshold / 100))

      return liquidationPrice
    } catch (error) {
      throw new Error(`Failed to calculate liquidation price: ${error}`)
    }
  }

  /**
   * Get max borrowable amount given collateral
   */
  async getMaxBorrowable(
    collateralAssets: CollateralAsset[]
  ): Promise<number> {
    const effectiveCollateralValue = collateralAssets.reduce((sum, c) => {
      const threshold = c.liquidationThreshold / 100
      return sum + (c.valueUSD * threshold)
    }, 0)

    // Max borrow at 150% health factor
    // healthFactor = (effectiveCollateralValue / borrowedValue) × 100 = 150
    // borrowedValue = effectiveCollateralValue / 1.5

    return effectiveCollateralValue / 1.5
  }

  /**
   * Get required collateral for a borrow amount
   */
  async getRequiredCollateral(
    borrowAmount: number,
    borrowAssetPrice: number,
    collateralAsset: string,
    targetHealthFactor: number = 200
  ): Promise<number> {
    const borrowValue = borrowAmount * borrowAssetPrice
    const threshold = this.liquidationThresholds[collateralAsset] || 80

    // healthFactor = (collateralValue × threshold) / borrowValue × 100
    // collateralValue = (borrowValue × healthFactor) / (threshold × 100)

    const requiredCollateralValue = (borrowValue * targetHealthFactor) / threshold

    return requiredCollateralValue
  }

  /**
   * Check if a loan should be liquidated
   */
  async shouldLiquidate(
    collateralValue: number,
    borrowedValue: number
  ): Promise<boolean> {
    const result = await this.calculateHealthFactor(collateralValue, borrowedValue)
    return result.healthFactor < 120
  }

  /**
   * Get collateral asset info
   */
  getCollateralInfo(asset: string): {
    liquidationThreshold: number
    riskLevel: 'low' | 'medium' | 'high'
  } {
    const threshold = this.liquidationThresholds[asset] || 80

    let riskLevel: 'low' | 'medium' | 'high'
    if (threshold >= 85) {
      riskLevel = 'low'
    } else if (threshold >= 75) {
      riskLevel = 'medium'
    } else {
      riskLevel = 'high'
    }

    return {
      liquidationThreshold: threshold,
      riskLevel,
    }
  }

  /**
   * Simulate adding collateral
   */
  async simulateAddCollateral(
    currentCollateralValue: number,
    additionalCollateralValue: number,
    borrowedValue: number
  ): Promise<HealthFactorResult> {
    const newCollateralValue = currentCollateralValue + additionalCollateralValue
    return this.calculateHealthFactor(newCollateralValue, borrowedValue)
  }

  /**
   * Simulate removing collateral
   */
  async simulateRemoveCollateral(
    currentCollateralValue: number,
    removeCollateralValue: number,
    borrowedValue: number
  ): Promise<HealthFactorResult> {
    const newCollateralValue = Math.max(0, currentCollateralValue - removeCollateralValue)
    return this.calculateHealthFactor(newCollateralValue, borrowedValue)
  }

  /**
   * Get asset price (mock - in production, use price oracle)
   */
  private getAssetPrice(asset: string): number {
    const prices: Record<string, number> = {
      'ÉTR': 8.0,
      'BTC': 45000,
      'ETH': 2500,
      'USDT': 1.0,
      'USDC': 1.0,
    }
    return prices[asset] || 1.0
  }
}

export const collateralService = CollateralService.getInstance()
