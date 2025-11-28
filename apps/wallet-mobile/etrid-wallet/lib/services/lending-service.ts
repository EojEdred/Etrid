/**
 * LendingService - ReserveVault integration for supply/borrow operations
 */

export interface Transaction {
  id: string
  hash: string
  status: 'pending' | 'confirmed' | 'failed'
  timestamp: number
}

export interface CollateralInput {
  asset: string
  amount: number
  valueUSD: number
}

export interface LendingPosition {
  id: string
  type: 'supply' | 'borrow'
  asset: string
  amount: number
  apy: number
  valueUSD: number
  openedAt: number
  interestAccrued: number
}

export interface LoanDetail {
  id: string
  asset: string
  borrowedAmount: number
  collateral: CollateralInput[]
  healthFactor: number
  apy: number
  interestAccrued: number
  liquidationPrice: number
  createdAt: number
}

export class LendingService {
  private static instance: LendingService

  // APY rates by asset
  private supplyAPYRates: Record<string, number> = {
    'ÉTR': 8.0,
    'BTC': 4.0,
    'ETH': 5.0,
    'USDT': 6.5,
    'USDC': 6.0,
  }

  private borrowAPYRates: Record<string, number> = {
    'ÉTR': 12.0,
    'BTC': 8.0,
    'ETH': 10.0,
    'USDT': 9.5,
    'USDC': 9.0,
  }

  // Mock positions storage
  private positions: LendingPosition[] = []
  private loans: LoanDetail[] = []

  static getInstance(): LendingService {
    if (!LendingService.instance) {
      LendingService.instance = new LendingService()
    }
    return LendingService.instance
  }

  /**
   * Supply assets to earn interest
   */
  async supply(asset: string, amount: number): Promise<Transaction> {
    try {
      // Simulate blockchain transaction
      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // Create position
      const position: LendingPosition = {
        id: `pos_${Date.now()}`,
        type: 'supply',
        asset,
        amount,
        apy: this.supplyAPYRates[asset] || 5.0,
        valueUSD: amount * this.getAssetPrice(asset),
        openedAt: Date.now(),
        interestAccrued: 0,
      }

      this.positions.push(position)

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to supply ${asset}: ${error}`)
    }
  }

  /**
   * Withdraw supplied assets
   */
  async withdraw(asset: string, amount: number): Promise<Transaction> {
    try {
      // Find position
      const position = this.positions.find(
        p => p.type === 'supply' && p.asset === asset && p.amount >= amount
      )

      if (!position) {
        throw new Error('Insufficient supplied balance')
      }

      // Check liquidity (simplified - in real implementation, check protocol liquidity)
      const availableLiquidity = position.amount * 0.8 // 80% liquidity available
      if (amount > availableLiquidity) {
        throw new Error('Insufficient liquidity in pool')
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // Update position
      position.amount -= amount
      if (position.amount === 0) {
        this.positions = this.positions.filter(p => p.id !== position.id)
      }

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to withdraw ${asset}: ${error}`)
    }
  }

  /**
   * Borrow assets against collateral
   */
  async borrow(asset: string, amount: number, collateral: CollateralInput[]): Promise<Transaction> {
    try {
      // Calculate total collateral value
      const totalCollateralValue = collateral.reduce((sum, c) => sum + c.valueUSD, 0)
      const borrowValue = amount * this.getAssetPrice(asset)

      // Calculate health factor
      const healthFactor = (totalCollateralValue / borrowValue) * 100

      // Require minimum 150% collateral
      if (healthFactor < 150) {
        throw new Error('Insufficient collateral. Minimum 150% required.')
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // Create loan
      const loan: LoanDetail = {
        id: `loan_${Date.now()}`,
        asset,
        borrowedAmount: amount,
        collateral,
        healthFactor,
        apy: this.borrowAPYRates[asset] || 10.0,
        interestAccrued: 0,
        liquidationPrice: this.calculateLiquidationPrice(borrowValue, totalCollateralValue),
        createdAt: Date.now(),
      }

      this.loans.push(loan)

      // Also create borrow position
      const position: LendingPosition = {
        id: `pos_${Date.now()}`,
        type: 'borrow',
        asset,
        amount,
        apy: this.borrowAPYRates[asset] || 10.0,
        valueUSD: borrowValue,
        openedAt: Date.now(),
        interestAccrued: 0,
      }

      this.positions.push(position)

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to borrow ${asset}: ${error}`)
    }
  }

  /**
   * Repay borrowed assets
   */
  async repay(loanId: string, amount: number): Promise<Transaction> {
    try {
      const loan = this.loans.find(l => l.id === loanId)
      if (!loan) {
        throw new Error('Loan not found')
      }

      // Calculate total owed (principal + interest)
      const totalOwed = loan.borrowedAmount + loan.interestAccrued

      if (amount > totalOwed) {
        throw new Error('Repayment amount exceeds total owed')
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // Update loan
      const remainingDebt = totalOwed - amount
      loan.borrowedAmount = Math.max(0, remainingDebt)

      if (loan.borrowedAmount === 0) {
        // Loan fully repaid, remove it
        this.loans = this.loans.filter(l => l.id !== loanId)
      }

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to repay loan: ${error}`)
    }
  }

  /**
   * Get supply APY for an asset
   */
  async getSupplyAPY(asset: string): Promise<number> {
    return this.supplyAPYRates[asset] || 5.0
  }

  /**
   * Get borrow APY for an asset
   */
  async getBorrowAPY(asset: string): Promise<number> {
    return this.borrowAPYRates[asset] || 10.0
  }

  /**
   * Get all lending positions
   */
  async getPositions(): Promise<LendingPosition[]> {
    // Update interest accrued
    const now = Date.now()
    this.positions.forEach(position => {
      const timeElapsed = (now - position.openedAt) / (1000 * 60 * 60 * 24 * 365) // years
      position.interestAccrued = position.amount * (position.apy / 100) * timeElapsed
    })

    return [...this.positions]
  }

  /**
   * Get health factor for a loan
   */
  async getHealthFactor(loanId: string): Promise<number> {
    const loan = this.loans.find(l => l.id === loanId)
    if (!loan) {
      throw new Error('Loan not found')
    }

    const totalCollateralValue = loan.collateral.reduce((sum, c) => sum + c.valueUSD, 0)
    const borrowValue = loan.borrowedAmount * this.getAssetPrice(loan.asset)

    return (totalCollateralValue / borrowValue) * 100
  }

  /**
   * Get loan details
   */
  async getLoan(loanId: string): Promise<LoanDetail | null> {
    const loan = this.loans.find(l => l.id === loanId)
    if (!loan) return null

    // Update interest
    const now = Date.now()
    const timeElapsed = (now - loan.createdAt) / (1000 * 60 * 60 * 24 * 365)
    loan.interestAccrued = loan.borrowedAmount * (loan.apy / 100) * timeElapsed

    return loan
  }

  /**
   * Get all loans
   */
  async getLoans(): Promise<LoanDetail[]> {
    return [...this.loans]
  }

  /**
   * Calculate total supplied across all assets
   */
  async getTotalSupplied(): Promise<number> {
    const supplyPositions = this.positions.filter(p => p.type === 'supply')
    return supplyPositions.reduce((sum, p) => sum + p.valueUSD, 0)
  }

  /**
   * Calculate total borrowed across all assets
   */
  async getTotalBorrowed(): Promise<number> {
    const borrowPositions = this.positions.filter(p => p.type === 'borrow')
    return borrowPositions.reduce((sum, p) => sum + p.valueUSD, 0)
  }

  /**
   * Calculate net APY (supply APY - borrow APY weighted by amounts)
   */
  async getNetAPY(): Promise<number> {
    const supplyPositions = this.positions.filter(p => p.type === 'supply')
    const borrowPositions = this.positions.filter(p => p.type === 'borrow')

    const totalSupplied = supplyPositions.reduce((sum, p) => sum + p.valueUSD, 0)
    const totalBorrowed = borrowPositions.reduce((sum, p) => sum + p.valueUSD, 0)

    if (totalSupplied === 0 && totalBorrowed === 0) return 0

    const supplyEarnings = supplyPositions.reduce((sum, p) => sum + (p.valueUSD * p.apy / 100), 0)
    const borrowCosts = borrowPositions.reduce((sum, p) => sum + (p.valueUSD * p.apy / 100), 0)

    const netEarnings = supplyEarnings - borrowCosts
    const totalValue = totalSupplied + totalBorrowed

    return (netEarnings / totalValue) * 100
  }

  // Helper methods

  private getAssetPrice(asset: string): number {
    // Mock prices (in production, fetch from price oracle)
    const prices: Record<string, number> = {
      'ÉTR': 8.0,
      'BTC': 45000,
      'ETH': 2500,
      'USDT': 1.0,
      'USDC': 1.0,
    }
    return prices[asset] || 1.0
  }

  private calculateLiquidationPrice(borrowValue: number, collateralValue: number): number {
    // Liquidation happens at 150% health factor
    // liquidationPrice = borrowValue * 1.5 / collateral amount
    return (borrowValue * 1.5) / collateralValue
  }
}

export const lendingService = LendingService.getInstance()
