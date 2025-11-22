import type {
  RefundRequest,
  Refund,
  RefundStatus,
} from '@/lib/types/merchant'

export class RefundService {
  private baseUrl = '/api/refunds'

  async requestRefund(request: RefundRequest): Promise<Refund> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(request),
      })

      if (!response.ok) {
        throw new Error('Failed to request refund')
      }

      return await response.json()
    } catch (error) {
      console.error('Error requesting refund:', error)
      throw error
    }
  }

  async processRefund(
    refundId: string,
    amount: number
  ): Promise<{ transaction_id: string }> {
    try {
      const response = await fetch(`${this.baseUrl}/${refundId}/process`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ amount }),
      })

      if (!response.ok) {
        throw new Error('Failed to process refund')
      }

      return await response.json()
    } catch (error) {
      console.error('Error processing refund:', error)
      throw error
    }
  }

  async getRefunds(filter?: RefundStatus): Promise<Refund[]> {
    try {
      const url = filter
        ? `${this.baseUrl}?status=${filter}`
        : this.baseUrl
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to fetch refunds')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching refunds:', error)
      throw error
    }
  }

  async getRefund(id: string): Promise<Refund> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`)

      if (!response.ok) {
        throw new Error('Failed to fetch refund')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching refund:', error)
      throw error
    }
  }

  async rejectRefund(refundId: string, reason: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${refundId}/reject`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ reason }),
      })

      if (!response.ok) {
        throw new Error('Failed to reject refund')
      }
    } catch (error) {
      console.error('Error rejecting refund:', error)
      throw error
    }
  }

  async approveRefund(refundId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${refundId}/approve`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error('Failed to approve refund')
      }
    } catch (error) {
      console.error('Error approving refund:', error)
      throw error
    }
  }
}

export const refundService = new RefundService()
