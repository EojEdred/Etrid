/**
 * P2PLendingService - Peer-to-peer lending marketplace
 */

import type { Transaction } from './lending-service'

export interface LoanOfferInput {
  asset: string
  amount: number
  apy: number
  duration: number // days
  minCollateralRatio: number // percentage (e.g., 150 for 150%)
  borrowerAddress?: string // if targeting specific borrower
}

export interface LoanOffer {
  id: string
  lenderId: string
  lenderUsername: string
  lenderRating: number // 0-5 stars
  asset: string
  amount: number
  apy: number
  duration: number
  minCollateralRatio: number
  status: 'active' | 'accepted' | 'cancelled' | 'completed'
  borrowerAddress?: string
  createdAt: number
}

export interface P2PLoan {
  id: string
  offerId: string
  lenderId: string
  borrowerId: string
  asset: string
  amount: number
  apy: number
  duration: number
  collateralRatio: number
  collateral: {
    asset: string
    amount: number
    valueUSD: number
  }[]
  status: 'active' | 'repaid' | 'defaulted' | 'liquidated'
  startDate: number
  dueDate: number
  interestAccrued: number
  amountRepaid: number
}

export interface LoanFilters {
  asset?: string
  minAPY?: number
  maxAPY?: number
  minDuration?: number
  maxDuration?: number
  minAmount?: number
  maxAmount?: number
}

export class P2PLendingService {
  private static instance: P2PLendingService

  private offers: LoanOffer[] = []
  private loans: P2PLoan[] = []
  private currentUserId = 'user_123' // Mock user ID

  static getInstance(): P2PLendingService {
    if (!P2PLendingService.instance) {
      P2PLendingService.instance = new P2PLendingService()
      // Initialize with mock data
      P2PLendingService.instance.initializeMockData()
    }
    return P2PLendingService.instance
  }

  /**
   * Create a new loan offer
   */
  async createLoanOffer(input: LoanOfferInput): Promise<LoanOffer> {
    try {
      const offer: LoanOffer = {
        id: `offer_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        lenderId: this.currentUserId,
        lenderUsername: this.getUserName(this.currentUserId),
        lenderRating: this.getUserRating(this.currentUserId),
        asset: input.asset,
        amount: input.amount,
        apy: input.apy,
        duration: input.duration,
        minCollateralRatio: input.minCollateralRatio,
        status: 'active',
        borrowerAddress: input.borrowerAddress,
        createdAt: Date.now(),
      }

      this.offers.push(offer)
      return offer
    } catch (error) {
      throw new Error(`Failed to create loan offer: ${error}`)
    }
  }

  /**
   * Accept a loan offer
   */
  async acceptLoanOffer(offerId: string, collateral: { asset: string; amount: number }[]): Promise<Transaction> {
    try {
      const offer = this.offers.find(o => o.id === offerId && o.status === 'active')
      if (!offer) {
        throw new Error('Loan offer not found or no longer active')
      }

      // Check if offer is for specific borrower
      if (offer.borrowerAddress && offer.borrowerAddress !== this.currentUserId) {
        throw new Error('This loan offer is not available to you')
      }

      // Calculate collateral value
      const collateralValue = collateral.reduce((sum, c) => {
        return sum + (c.amount * this.getAssetPrice(c.asset))
      }, 0)

      const loanValue = offer.amount * this.getAssetPrice(offer.asset)
      const collateralRatio = (collateralValue / loanValue) * 100

      // Check collateral requirement
      if (collateralRatio < offer.minCollateralRatio) {
        throw new Error(
          `Insufficient collateral. Required: ${offer.minCollateralRatio}%, Provided: ${collateralRatio.toFixed(2)}%`
        )
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      // Create P2P loan
      const loan: P2PLoan = {
        id: `p2p_loan_${Date.now()}`,
        offerId: offer.id,
        lenderId: offer.lenderId,
        borrowerId: this.currentUserId,
        asset: offer.asset,
        amount: offer.amount,
        apy: offer.apy,
        duration: offer.duration,
        collateralRatio,
        collateral: collateral.map(c => ({
          asset: c.asset,
          amount: c.amount,
          valueUSD: c.amount * this.getAssetPrice(c.asset),
        })),
        status: 'active',
        startDate: Date.now(),
        dueDate: Date.now() + offer.duration * 24 * 60 * 60 * 1000,
        interestAccrued: 0,
        amountRepaid: 0,
      }

      this.loans.push(loan)

      // Update offer status
      offer.status = 'accepted'

      return {
        id: txId,
        hash: `0x${txId}`,
        status: 'confirmed',
        timestamp: Date.now(),
      }
    } catch (error) {
      throw new Error(`Failed to accept loan offer: ${error}`)
    }
  }

  /**
   * Get all loan offers with optional filters
   */
  async getLoanOffers(filters?: LoanFilters): Promise<LoanOffer[]> {
    let filteredOffers = this.offers.filter(o => o.status === 'active')

    if (filters) {
      if (filters.asset) {
        filteredOffers = filteredOffers.filter(o => o.asset === filters.asset)
      }
      if (filters.minAPY !== undefined) {
        filteredOffers = filteredOffers.filter(o => o.apy >= filters.minAPY!)
      }
      if (filters.maxAPY !== undefined) {
        filteredOffers = filteredOffers.filter(o => o.apy <= filters.maxAPY!)
      }
      if (filters.minDuration !== undefined) {
        filteredOffers = filteredOffers.filter(o => o.duration >= filters.minDuration!)
      }
      if (filters.maxDuration !== undefined) {
        filteredOffers = filteredOffers.filter(o => o.duration <= filters.maxDuration!)
      }
      if (filters.minAmount !== undefined) {
        filteredOffers = filteredOffers.filter(o => o.amount >= filters.minAmount!)
      }
      if (filters.maxAmount !== undefined) {
        filteredOffers = filteredOffers.filter(o => o.amount <= filters.maxAmount!)
      }
    }

    return filteredOffers
  }

  /**
   * Get my loans (as lender or borrower)
   */
  async getMyLoans(type: 'lender' | 'borrower'): Promise<P2PLoan[]> {
    if (type === 'lender') {
      return this.loans.filter(l => l.lenderId === this.currentUserId)
    } else {
      return this.loans.filter(l => l.borrowerId === this.currentUserId)
    }
  }

  /**
   * Get my loan offers
   */
  async getMyOffers(): Promise<LoanOffer[]> {
    return this.offers.filter(o => o.lenderId === this.currentUserId)
  }

  /**
   * Cancel a loan offer
   */
  async cancelOffer(offerId: string): Promise<void> {
    const offer = this.offers.find(o => o.id === offerId && o.lenderId === this.currentUserId)
    if (!offer) {
      throw new Error('Offer not found or you are not the lender')
    }
    if (offer.status !== 'active') {
      throw new Error('Only active offers can be cancelled')
    }
    offer.status = 'cancelled'
  }

  /**
   * Repay P2P loan
   */
  async repayLoan(loanId: string, amount: number): Promise<Transaction> {
    try {
      const loan = this.loans.find(l => l.id === loanId && l.borrowerId === this.currentUserId)
      if (!loan) {
        throw new Error('Loan not found or you are not the borrower')
      }

      if (loan.status !== 'active') {
        throw new Error('Loan is not active')
      }

      // Calculate interest
      const now = Date.now()
      const timeElapsed = (now - loan.startDate) / (1000 * 60 * 60 * 24 * 365) // years
      loan.interestAccrued = loan.amount * (loan.apy / 100) * timeElapsed

      const totalOwed = loan.amount + loan.interestAccrued
      const remainingDebt = totalOwed - loan.amountRepaid

      if (amount > remainingDebt) {
        throw new Error('Repayment amount exceeds remaining debt')
      }

      const txId = `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`

      loan.amountRepaid += amount

      // Check if fully repaid
      if (loan.amountRepaid >= totalOwed) {
        loan.status = 'repaid'
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
   * Get loan details
   */
  async getLoan(loanId: string): Promise<P2PLoan | null> {
    const loan = this.loans.find(l => l.id === loanId)
    if (!loan) return null

    // Update interest
    const now = Date.now()
    const timeElapsed = (now - loan.startDate) / (1000 * 60 * 60 * 24 * 365)
    loan.interestAccrued = loan.amount * (loan.apy / 100) * timeElapsed

    return loan
  }

  // Helper methods

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

  private getUserName(userId: string): string {
    // Mock username
    const usernames: Record<string, string> = {
      'user_123': 'CryptoLender',
      'user_456': 'DeFiExpert',
      'user_789': 'YieldSeeker',
    }
    return usernames[userId] || 'Anonymous'
  }

  private getUserRating(userId: string): number {
    // Mock rating (0-5)
    const ratings: Record<string, number> = {
      'user_123': 4.8,
      'user_456': 4.5,
      'user_789': 4.9,
    }
    return ratings[userId] || 4.0
  }

  private initializeMockData(): void {
    // Add some mock loan offers
    this.offers = [
      {
        id: 'offer_1',
        lenderId: 'user_456',
        lenderUsername: 'DeFiExpert',
        lenderRating: 4.5,
        asset: 'ÉTR',
        amount: 1000,
        apy: 15,
        duration: 30,
        minCollateralRatio: 150,
        status: 'active',
        createdAt: Date.now() - 86400000,
      },
      {
        id: 'offer_2',
        lenderId: 'user_789',
        lenderUsername: 'YieldSeeker',
        lenderRating: 4.9,
        asset: 'ETH',
        amount: 5,
        apy: 12,
        duration: 60,
        minCollateralRatio: 160,
        status: 'active',
        createdAt: Date.now() - 172800000,
      },
      {
        id: 'offer_3',
        lenderId: 'user_456',
        lenderUsername: 'DeFiExpert',
        lenderRating: 4.5,
        asset: 'BTC',
        amount: 0.5,
        apy: 10,
        duration: 90,
        minCollateralRatio: 140,
        status: 'active',
        createdAt: Date.now() - 259200000,
      },
      {
        id: 'offer_4',
        lenderId: 'user_789',
        lenderUsername: 'YieldSeeker',
        lenderRating: 4.9,
        asset: 'USDT',
        amount: 5000,
        apy: 18,
        duration: 14,
        minCollateralRatio: 170,
        status: 'active',
        createdAt: Date.now() - 43200000,
      },
    ]
  }
}

export const p2pLendingService = P2PLendingService.getInstance()
