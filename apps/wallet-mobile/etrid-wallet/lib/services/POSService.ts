import type { SaleInput, Sale, Discount } from '@/lib/types/merchant'

export class POSService {
  private baseUrl = '/api/merchant/pos'

  async createSale(sale: SaleInput): Promise<Sale> {
    try {
      const response = await fetch(`${this.baseUrl}/sale`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(sale),
      })

      if (!response.ok) {
        throw new Error('Failed to create sale')
      }

      return await response.json()
    } catch (error) {
      console.error('Error creating sale:', error)
      throw error
    }
  }

  async voidSale(saleId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/sale/${saleId}/void`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error('Failed to void sale')
      }
    } catch (error) {
      console.error('Error voiding sale:', error)
      throw error
    }
  }

  async generateReceipt(saleId: string): Promise<string> {
    try {
      const response = await fetch(`${this.baseUrl}/sale/${saleId}/receipt`)

      if (!response.ok) {
        throw new Error('Failed to generate receipt')
      }

      const data = await response.json()
      return data.receipt_html
    } catch (error) {
      console.error('Error generating receipt:', error)
      throw error
    }
  }

  async applyDiscount(saleId: string, discount: Discount): Promise<Sale> {
    try {
      const response = await fetch(`${this.baseUrl}/sale/${saleId}/discount`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(discount),
      })

      if (!response.ok) {
        throw new Error('Failed to apply discount')
      }

      return await response.json()
    } catch (error) {
      console.error('Error applying discount:', error)
      throw error
    }
  }

  async getSale(saleId: string): Promise<Sale> {
    try {
      const response = await fetch(`${this.baseUrl}/sale/${saleId}`)

      if (!response.ok) {
        throw new Error('Failed to fetch sale')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching sale:', error)
      throw error
    }
  }

  async getSales(
    startDate?: Date,
    endDate?: Date
  ): Promise<Sale[]> {
    try {
      const params = new URLSearchParams()
      if (startDate) params.append('start_date', startDate.toISOString())
      if (endDate) params.append('end_date', endDate.toISOString())

      const url = `${this.baseUrl}/sales${params.toString() ? `?${params.toString()}` : ''}`
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to fetch sales')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching sales:', error)
      throw error
    }
  }
}

export const posService = new POSService()
