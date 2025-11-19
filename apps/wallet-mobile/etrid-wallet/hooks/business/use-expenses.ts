"use client"

import { useState, useEffect } from 'react'
import { expenseService } from '@/lib/services/ExpenseService'
import type {
  Expense,
  ExpenseInput,
  ExpenseFilter,
  CategoryBreakdown,
} from '@/lib/types/business'

export function useExpenses(filter?: ExpenseFilter) {
  const [expenses, setExpenses] = useState<Expense[]>([])
  const [categoryBreakdown, setCategoryBreakdown] = useState<CategoryBreakdown[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchExpenses = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await expenseService.getExpenses(filter)
      setExpenses(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const fetchCategoryBreakdown = async () => {
    try {
      const data = await expenseService.getExpensesByCategory()
      setCategoryBreakdown(data)
    } catch (err) {
      console.error('Error fetching category breakdown:', err)
    }
  }

  const addExpense = async (expense: ExpenseInput) => {
    try {
      setError(null)
      const newExpense = await expenseService.addExpense(expense)
      setExpenses([newExpense, ...expenses])
      await fetchCategoryBreakdown()
      return newExpense
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const updateExpense = async (id: string, updates: Partial<Expense>) => {
    try {
      setError(null)
      const updated = await expenseService.updateExpense(id, updates)
      setExpenses(expenses.map((exp) => (exp.id === id ? updated : exp)))
      await fetchCategoryBreakdown()
      return updated
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const deleteExpense = async (id: string) => {
    try {
      setError(null)
      await expenseService.deleteExpense(id)
      setExpenses(expenses.filter((exp) => exp.id !== id))
      await fetchCategoryBreakdown()
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const exportToCSV = async () => {
    try {
      const blob = await expenseService.exportToCSV(filter)
      const url = window.URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = `expenses-${new Date().toISOString()}.csv`
      a.click()
      window.URL.revokeObjectURL(url)
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchExpenses()
    fetchCategoryBreakdown()
  }, [filter?.category, filter?.start_date, filter?.end_date])

  return {
    expenses,
    categoryBreakdown,
    loading,
    error,
    refetch: fetchExpenses,
    addExpense,
    updateExpense,
    deleteExpense,
    exportToCSV,
  }
}
