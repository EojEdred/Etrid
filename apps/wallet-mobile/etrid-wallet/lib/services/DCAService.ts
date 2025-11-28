import type { DCASchedule, CryptoAsset, DCAFrequency, ApiResponse, PaginatedResponse } from "@/lib/types/features"

/**
 * DCAService - Dollar-Cost Averaging schedule management
 * Handles recurring crypto purchases
 */
export class DCAService {
  private baseUrl: string

  constructor(baseUrl: string = "/api/dca") {
    this.baseUrl = baseUrl
  }

  /**
   * Create a new DCA schedule
   */
  async createSchedule(schedule: {
    asset: CryptoAsset
    amountUsd: number
    frequency: DCAFrequency
    paymentMethodId: string
    startDate: Date
    endDate?: Date
  }): Promise<DCASchedule> {
    try {
      const response = await fetch(`${this.baseUrl}/schedules`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(schedule),
      })

      const data: ApiResponse<DCASchedule> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to create DCA schedule")
      }

      return data.data
    } catch (error) {
      console.error("Error creating DCA schedule:", error)
      throw error
    }
  }

  /**
   * Get all DCA schedules for the current user
   */
  async getSchedules(includeInactive: boolean = false): Promise<DCASchedule[]> {
    try {
      const params = new URLSearchParams({
        includeInactive: includeInactive.toString(),
      })

      const response = await fetch(`${this.baseUrl}/schedules?${params}`)
      const data: ApiResponse<DCASchedule[]> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get DCA schedules")
      }

      return data.data
    } catch (error) {
      console.error("Error getting DCA schedules:", error)
      throw error
    }
  }

  /**
   * Get a specific DCA schedule by ID
   */
  async getSchedule(id: string): Promise<DCASchedule> {
    try {
      const response = await fetch(`${this.baseUrl}/schedules/${id}`)
      const data: ApiResponse<DCASchedule> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get DCA schedule")
      }

      return data.data
    } catch (error) {
      console.error("Error getting DCA schedule:", error)
      throw error
    }
  }

  /**
   * Update a DCA schedule
   */
  async updateSchedule(
    id: string,
    updates: {
      amountUsd?: number
      frequency?: DCAFrequency
      paymentMethodId?: string
      endDate?: Date
    },
  ): Promise<DCASchedule> {
    try {
      const response = await fetch(`${this.baseUrl}/schedules/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(updates),
      })

      const data: ApiResponse<DCASchedule> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to update DCA schedule")
      }

      return data.data
    } catch (error) {
      console.error("Error updating DCA schedule:", error)
      throw error
    }
  }

  /**
   * Pause a DCA schedule
   */
  async pauseSchedule(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/schedules/${id}/pause`, {
        method: "POST",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to pause DCA schedule")
      }
    } catch (error) {
      console.error("Error pausing DCA schedule:", error)
      throw error
    }
  }

  /**
   * Resume a paused DCA schedule
   */
  async resumeSchedule(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/schedules/${id}/resume`, {
        method: "POST",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to resume DCA schedule")
      }
    } catch (error) {
      console.error("Error resuming DCA schedule:", error)
      throw error
    }
  }

  /**
   * Delete a DCA schedule
   */
  async deleteSchedule(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/schedules/${id}`, {
        method: "DELETE",
      })

      const data: ApiResponse<void> = await response.json()

      if (!data.success) {
        throw new Error(data.error || "Failed to delete DCA schedule")
      }
    } catch (error) {
      console.error("Error deleting DCA schedule:", error)
      throw error
    }
  }

  /**
   * Get DCA purchase history for a schedule
   */
  async getScheduleHistory(
    id: string,
    page: number = 1,
    pageSize: number = 20,
  ): Promise<
    PaginatedResponse<{
      date: Date
      amountUsd: number
      cryptoAmount: number
      exchangeRate: number
      status: "completed" | "failed"
    }>
  > {
    try {
      const params = new URLSearchParams({
        page: page.toString(),
        pageSize: pageSize.toString(),
      })

      const response = await fetch(`${this.baseUrl}/schedules/${id}/history?${params}`)
      const data: ApiResponse<
        PaginatedResponse<{
          date: Date
          amountUsd: number
          cryptoAmount: number
          exchangeRate: number
          status: "completed" | "failed"
        }>
      > = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get schedule history")
      }

      return data.data
    } catch (error) {
      console.error("Error getting schedule history:", error)
      throw error
    }
  }

  /**
   * Calculate next run date based on frequency
   */
  calculateNextRunDate(lastRunDate: Date, frequency: DCAFrequency): Date {
    const next = new Date(lastRunDate)

    switch (frequency) {
      case "daily":
        next.setDate(next.getDate() + 1)
        break
      case "weekly":
        next.setDate(next.getDate() + 7)
        break
      case "biweekly":
        next.setDate(next.getDate() + 14)
        break
      case "monthly":
        next.setMonth(next.getMonth() + 1)
        break
    }

    return next
  }

  /**
   * Get DCA statistics
   */
  async getStats(): Promise<{
    totalSchedules: number
    activeSchedules: number
    totalInvested: number
    totalPurchases: number
  }> {
    try {
      const response = await fetch(`${this.baseUrl}/stats`)
      const data: ApiResponse<{
        totalSchedules: number
        activeSchedules: number
        totalInvested: number
        totalPurchases: number
      }> = await response.json()

      if (!data.success || !data.data) {
        throw new Error(data.error || "Failed to get DCA stats")
      }

      return data.data
    } catch (error) {
      console.error("Error getting DCA stats:", error)
      throw error
    }
  }
}

// Export singleton instance
export const dcaService = new DCAService()
