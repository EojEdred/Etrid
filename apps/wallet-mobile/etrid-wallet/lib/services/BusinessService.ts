import type {
  BusinessInfo,
  BusinessAccount,
  DashboardStats,
} from '@/lib/types/business'

export class BusinessService {
  private baseUrl = '/api/business'

  async createBusinessAccount(info: BusinessInfo): Promise<BusinessAccount> {
    try {
      const response = await fetch(`${this.baseUrl}/create`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(info),
      })

      if (!response.ok) {
        throw new Error('Failed to create business account')
      }

      return await response.json()
    } catch (error) {
      console.error('Error creating business account:', error)
      throw error
    }
  }

  async getBusinessAccount(): Promise<BusinessAccount> {
    try {
      const response = await fetch(`${this.baseUrl}`)

      if (!response.ok) {
        throw new Error('Failed to fetch business account')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching business account:', error)
      throw error
    }
  }

  async updateBusinessAccount(
    updates: Partial<BusinessAccount>
  ): Promise<BusinessAccount> {
    try {
      const response = await fetch(`${this.baseUrl}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (!response.ok) {
        throw new Error('Failed to update business account')
      }

      return await response.json()
    } catch (error) {
      console.error('Error updating business account:', error)
      throw error
    }
  }

  async getDashboardStats(): Promise<DashboardStats> {
    try {
      const response = await fetch(`${this.baseUrl}/dashboard`)

      if (!response.ok) {
        throw new Error('Failed to fetch dashboard stats')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching dashboard stats:', error)
      throw error
    }
  }

  async deleteBusinessAccount(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to delete business account')
      }
    } catch (error) {
      console.error('Error deleting business account:', error)
      throw error
    }
  }
}

export const businessService = new BusinessService()
