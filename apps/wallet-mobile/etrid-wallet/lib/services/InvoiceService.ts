import type {
  InvoiceInput,
  Invoice,
  InvoiceFilter,
} from '@/lib/types/business'

export class InvoiceService {
  private baseUrl = '/api/invoices'

  async createInvoice(invoice: InvoiceInput): Promise<Invoice> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(invoice),
      })

      if (!response.ok) {
        throw new Error('Failed to create invoice')
      }

      return await response.json()
    } catch (error) {
      console.error('Error creating invoice:', error)
      throw error
    }
  }

  async getInvoices(filter?: InvoiceFilter): Promise<Invoice[]> {
    try {
      const params = new URLSearchParams()
      if (filter?.status) params.append('status', filter.status)
      if (filter?.start_date)
        params.append('start_date', filter.start_date.toISOString())
      if (filter?.end_date)
        params.append('end_date', filter.end_date.toISOString())
      if (filter?.client) params.append('client', filter.client)

      const url = `${this.baseUrl}${params.toString() ? `?${params.toString()}` : ''}`
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to fetch invoices')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching invoices:', error)
      throw error
    }
  }

  async getInvoice(id: string): Promise<Invoice> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`)

      if (!response.ok) {
        throw new Error('Failed to fetch invoice')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching invoice:', error)
      throw error
    }
  }

  async updateInvoice(
    id: string,
    updates: Partial<Invoice>
  ): Promise<Invoice> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (!response.ok) {
        throw new Error('Failed to update invoice')
      }

      return await response.json()
    } catch (error) {
      console.error('Error updating invoice:', error)
      throw error
    }
  }

  async sendInvoice(id: string, recipient: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/send`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ recipient }),
      })

      if (!response.ok) {
        throw new Error('Failed to send invoice')
      }
    } catch (error) {
      console.error('Error sending invoice:', error)
      throw error
    }
  }

  async markPaid(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/mark-paid`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error('Failed to mark invoice as paid')
      }
    } catch (error) {
      console.error('Error marking invoice as paid:', error)
      throw error
    }
  }

  async generatePDF(id: string): Promise<Blob> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/pdf`)

      if (!response.ok) {
        throw new Error('Failed to generate PDF')
      }

      return await response.blob()
    } catch (error) {
      console.error('Error generating PDF:', error)
      throw error
    }
  }

  async deleteInvoice(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to delete invoice')
      }
    } catch (error) {
      console.error('Error deleting invoice:', error)
      throw error
    }
  }
}

export const invoiceService = new InvoiceService()
