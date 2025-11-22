import type {
  PaymentLinkInput,
  PaymentLink,
  PaymentLinkStatus,
  PaymentStatus,
} from '@/lib/types/merchant'

export class PaymentLinkService {
  private baseUrl = '/api/payment-links'

  async createPaymentLink(link: PaymentLinkInput): Promise<PaymentLink> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(link),
      })

      if (!response.ok) {
        throw new Error('Failed to create payment link')
      }

      return await response.json()
    } catch (error) {
      console.error('Error creating payment link:', error)
      throw error
    }
  }

  async getPaymentLinks(
    filter?: PaymentLinkStatus
  ): Promise<PaymentLink[]> {
    try {
      const url = filter
        ? `${this.baseUrl}?status=${filter}`
        : this.baseUrl
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to fetch payment links')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching payment links:', error)
      throw error
    }
  }

  async getPaymentLink(id: string): Promise<PaymentLink> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`)

      if (!response.ok) {
        throw new Error('Failed to fetch payment link')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching payment link:', error)
      throw error
    }
  }

  async deactivateLink(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/deactivate`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error('Failed to deactivate payment link')
      }
    } catch (error) {
      console.error('Error deactivating payment link:', error)
      throw error
    }
  }

  async getPaymentStatus(id: string): Promise<PaymentStatus> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/status`)

      if (!response.ok) {
        throw new Error('Failed to fetch payment status')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching payment status:', error)
      throw error
    }
  }

  async deletePaymentLink(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to delete payment link')
      }
    } catch (error) {
      console.error('Error deleting payment link:', error)
      throw error
    }
  }

  getPaymentLinkUrl(linkCode: string): string {
    const baseUrl = typeof window !== 'undefined' ? window.location.origin : ''
    return `${baseUrl}/pay/${linkCode}`
  }
}

export const paymentLinkService = new PaymentLinkService()
