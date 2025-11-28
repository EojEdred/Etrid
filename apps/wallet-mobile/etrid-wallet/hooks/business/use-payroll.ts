"use client"

import { useState, useEffect } from 'react'
import { payrollService } from '@/lib/services/PayrollService'
import type { Payroll, PayrollInput, PayrollSchedule } from '@/lib/types/business'

export function usePayroll() {
  const [payrolls, setPayrolls] = useState<Payroll[]>([])
  const [schedule, setSchedule] = useState<PayrollSchedule | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchPayrolls = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await payrollService.getPayrollHistory()
      setPayrolls(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const fetchSchedule = async () => {
    try {
      const data = await payrollService.getPayrollSchedule()
      setSchedule(data)
    } catch (err) {
      console.error('Error fetching schedule:', err)
    }
  }

  const createPayroll = async (payroll: PayrollInput) => {
    try {
      setError(null)
      const newPayroll = await payrollService.createPayroll(payroll)
      setPayrolls([newPayroll, ...payrolls])
      return newPayroll
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const executePayroll = async (id: string) => {
    try {
      setError(null)
      const result = await payrollService.executePayroll(id)
      await fetchPayrolls()
      return result
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const updateSchedule = async (newSchedule: PayrollSchedule) => {
    try {
      setError(null)
      await payrollService.updatePayrollSchedule(newSchedule)
      setSchedule(newSchedule)
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchPayrolls()
    fetchSchedule()
  }, [])

  return {
    payrolls,
    schedule,
    loading,
    error,
    refetch: fetchPayrolls,
    createPayroll,
    executePayroll,
    updateSchedule,
  }
}
