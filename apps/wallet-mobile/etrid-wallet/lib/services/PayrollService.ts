import type {
  PayrollInput,
  Payroll,
  PayrollSchedule,
} from '@/lib/types/business'

export class PayrollService {
  private baseUrl = '/api/payroll'

  async createPayroll(payroll: PayrollInput): Promise<Payroll> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payroll),
      })

      if (!response.ok) {
        throw new Error('Failed to create payroll')
      }

      return await response.json()
    } catch (error) {
      console.error('Error creating payroll:', error)
      throw error
    }
  }

  async executePayroll(id: string): Promise<{ transaction_ids: string[] }> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/execute`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error('Failed to execute payroll')
      }

      return await response.json()
    } catch (error) {
      console.error('Error executing payroll:', error)
      throw error
    }
  }

  async getPayrollHistory(): Promise<Payroll[]> {
    try {
      const response = await fetch(`${this.baseUrl}/history`)

      if (!response.ok) {
        throw new Error('Failed to fetch payroll history')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching payroll history:', error)
      throw error
    }
  }

  async getPayroll(id: string): Promise<Payroll> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`)

      if (!response.ok) {
        throw new Error('Failed to fetch payroll')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching payroll:', error)
      throw error
    }
  }

  async getPayrollSchedule(): Promise<PayrollSchedule> {
    try {
      const response = await fetch(`${this.baseUrl}/schedule`)

      if (!response.ok) {
        throw new Error('Failed to fetch payroll schedule')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching payroll schedule:', error)
      throw error
    }
  }

  async updatePayrollSchedule(
    schedule: PayrollSchedule
  ): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/schedule`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(schedule),
      })

      if (!response.ok) {
        throw new Error('Failed to update payroll schedule')
      }
    } catch (error) {
      console.error('Error updating payroll schedule:', error)
      throw error
    }
  }

  async cancelPayroll(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to cancel payroll')
      }
    } catch (error) {
      console.error('Error cancelling payroll:', error)
      throw error
    }
  }
}

export const payrollService = new PayrollService()
