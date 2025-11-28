"use client"

import { Calendar, DollarSign, Pause, Play, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Switch } from "@/components/ui/switch"
import type { DCASchedule } from "@/lib/types/features"

interface DCAScheduleItemProps {
  schedule: DCASchedule
  onPause?: (id: string) => void
  onResume?: (id: string) => void
  onDelete?: (id: string) => void
  onClick?: (id: string) => void
}

export function DCAScheduleItem({ schedule, onPause, onResume, onDelete, onClick }: DCAScheduleItemProps) {
  const formatFrequency = (frequency: string) => {
    const map: Record<string, string> = {
      daily: "Every Day",
      weekly: "Every Week",
      biweekly: "Every 2 Weeks",
      monthly: "Every Month",
    }
    return map[frequency] || frequency
  }

  const formatDate = (date: Date) => {
    return new Date(date).toLocaleDateString("en-US", { month: "short", day: "numeric", year: "numeric" })
  }

  const handleToggle = async () => {
    if (schedule.isActive && onPause) {
      await onPause(schedule.id)
    } else if (!schedule.isActive && onResume) {
      await onResume(schedule.id)
    }
  }

  return (
    <div
      className={`glass p-4 rounded-2xl ${onClick ? "cursor-pointer hover:glass-strong" : ""}`}
      onClick={() => onClick?.(schedule.id)}
    >
      <div className="flex items-start gap-4">
        <div className="w-12 h-12 rounded-xl bg-accent/10 flex items-center justify-center shrink-0">
          <DollarSign className="w-6 h-6 text-accent" />
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-start justify-between gap-2 mb-2">
            <div>
              <p className="font-semibold">Buy ${schedule.amountUsd} {schedule.asset}</p>
              <p className="text-sm text-muted-foreground">{formatFrequency(schedule.frequency)}</p>
            </div>

            <div className="flex items-center gap-2">
              <Switch
                checked={schedule.isActive}
                onCheckedChange={handleToggle}
                onClick={(e) => e.stopPropagation()}
              />
              {schedule.isActive ? (
                <Badge variant="default" className="bg-green-500">
                  Active
                </Badge>
              ) : (
                <Badge variant="secondary">Paused</Badge>
              )}
            </div>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-2 gap-4 mb-3">
            <div>
              <p className="text-xs text-muted-foreground">Total Purchased</p>
              <p className="text-sm font-semibold">${schedule.totalPurchased.toFixed(2)}</p>
            </div>
            <div>
              <p className="text-xs text-muted-foreground">Purchases</p>
              <p className="text-sm font-semibold">{schedule.purchaseCount}</p>
            </div>
          </div>

          {/* Next Run */}
          {schedule.isActive && (
            <div className="flex items-center gap-2 text-sm mb-3">
              <Calendar className="w-4 h-4 text-muted-foreground" />
              <span className="text-muted-foreground">Next: {formatDate(schedule.nextRunDate)}</span>
            </div>
          )}

          {/* End Date */}
          {schedule.endDate && (
            <p className="text-xs text-muted-foreground">
              Ends: {formatDate(schedule.endDate)}
            </p>
          )}

          {/* Actions */}
          {(onPause || onResume || onDelete) && (
            <div className="flex items-center gap-2 mt-3 pt-3 border-t border-border">
              {onDelete && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation()
                    onDelete(schedule.id)
                  }}
                  className="text-destructive"
                >
                  <Trash2 className="w-4 h-4 mr-2" />
                  Delete
                </Button>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
