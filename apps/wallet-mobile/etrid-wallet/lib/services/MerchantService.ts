import type {
  MerchantInfo,
  MerchantAccount,
  SalesStats,
  Sale,
} from '@/lib/types/merchant'

export class MerchantService {
  private baseUrl = '/api/merchant'

  async createMerchantAccount(info: MerchantInfo): Promise<MerchantAccount> {
    try {
      const response = await fetch(`${this.baseUrl}/create`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(info),
      })

      if (!response.ok) {
        throw new Error('Failed to create merchant account')
      }

      return await response.json()
    } catch (error) {
      console.error('Error creating merchant account:', error)
      throw error
    }
  }

  async getMerchantAccount(): Promise<MerchantAccount> {
    try {
      const response = await fetch(this.baseUrl)

      if (!response.ok) {
        throw new Error('Failed to fetch merchant account')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching merchant account:', error)
      throw error
    }
  }

  async getSalesStats(
    period: 'day' | 'week' | 'month' | 'year' = 'month'
  ): Promise<SalesStats> {
    try {
      const response = await fetch(`${this.baseUrl}/stats?period=${period}`)

      if (!response.ok) {
        throw new Error('Failed to fetch sales stats')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching sales stats:', error)
      throw error
    }
  }

  async getRecentSales(limit: number = 10): Promise<Sale[]> {
    try {
      const response = await fetch(`${this.baseUrl}/sales/recent?limit=${limit}`)

      if (!response.ok) {
        throw new Error('Failed to fetch recent sales')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching recent sales:', error)
      throw error
    }
  }

  async updateMerchantAccount(
    updates: Partial<MerchantAccount>
  ): Promise<MerchantAccount> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (!response.ok) {
        throw new Error('Failed to update merchant account')
      }

      return await response.json()
    } catch (error) {
      console.error('Error updating merchant account:', error)
      throw error
    }
  }
}

export const merchantService = new MerchantService()
