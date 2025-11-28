import type {
  ExpenseInput,
  Expense,
  ExpenseFilter,
  CategoryBreakdown,
} from '@/lib/types/business'

export class ExpenseService {
  private baseUrl = '/api/expenses'

  async addExpense(expense: ExpenseInput): Promise<Expense> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(expense),
      })

      if (!response.ok) {
        throw new Error('Failed to add expense')
      }

      return await response.json()
    } catch (error) {
      console.error('Error adding expense:', error)
      throw error
    }
  }

  async getExpenses(filter?: ExpenseFilter): Promise<Expense[]> {
    try {
      const params = new URLSearchParams()
      if (filter?.category) params.append('category', filter.category)
      if (filter?.start_date)
        params.append('start_date', filter.start_date.toISOString())
      if (filter?.end_date)
        params.append('end_date', filter.end_date.toISOString())
      if (filter?.team_member_id)
        params.append('team_member_id', filter.team_member_id)
      if (filter?.reimbursable !== undefined)
        params.append('reimbursable', filter.reimbursable.toString())

      const url = `${this.baseUrl}${params.toString() ? `?${params.toString()}` : ''}`
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to fetch expenses')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching expenses:', error)
      throw error
    }
  }

  async getExpense(id: string): Promise<Expense> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`)

      if (!response.ok) {
        throw new Error('Failed to fetch expense')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching expense:', error)
      throw error
    }
  }

  async updateExpense(
    id: string,
    updates: Partial<Expense>
  ): Promise<Expense> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (!response.ok) {
        throw new Error('Failed to update expense')
      }

      return await response.json()
    } catch (error) {
      console.error('Error updating expense:', error)
      throw error
    }
  }

  async deleteExpense(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to delete expense')
      }
    } catch (error) {
      console.error('Error deleting expense:', error)
      throw error
    }
  }

  async getExpensesByCategory(): Promise<CategoryBreakdown[]> {
    try {
      const response = await fetch(`${this.baseUrl}/by-category`)

      if (!response.ok) {
        throw new Error('Failed to fetch expenses by category')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching expenses by category:', error)
      throw error
    }
  }

  async exportToCSV(filter?: ExpenseFilter): Promise<Blob> {
    try {
      const params = new URLSearchParams()
      if (filter?.category) params.append('category', filter.category)
      if (filter?.start_date)
        params.append('start_date', filter.start_date.toISOString())
      if (filter?.end_date)
        params.append('end_date', filter.end_date.toISOString())
      if (filter?.team_member_id)
        params.append('team_member_id', filter.team_member_id)

      const url = `${this.baseUrl}/export/csv${params.toString() ? `?${params.toString()}` : ''}`
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to export expenses')
      }

      return await response.blob()
    } catch (error) {
      console.error('Error exporting expenses:', error)
      throw error
    }
  }

  async markReimbursed(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/reimburse`, {
        method: 'POST',
      })

      if (!response.ok) {
        throw new Error('Failed to mark expense as reimbursed')
      }
    } catch (error) {
      console.error('Error marking expense as reimbursed:', error)
      throw error
    }
  }
}

export const expenseService = new ExpenseService()
