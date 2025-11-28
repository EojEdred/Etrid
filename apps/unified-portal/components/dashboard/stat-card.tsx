import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { LucideIcon } from "lucide-react"

interface StatCardProps {
  title: string
  value: string
  status: "healthy" | "syncing" | "warning" | "error"
  icon?: LucideIcon
}

const statusStyles = {
  healthy: "success",
  syncing: "secondary",
  warning: "warning",
  error: "destructive",
} as const

const statusLabels = {
  healthy: "Healthy",
  syncing: "Syncing",
  warning: "Warning",
  error: "Error",
} as const

export function StatCard({ title, value, status, icon: Icon }: StatCardProps) {
  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm font-medium text-zinc-500 dark:text-zinc-400">
            {title}
          </CardTitle>
          <Badge variant={statusStyles[status]} className="text-xs">
            {statusLabels[status]}
          </Badge>
        </div>
      </CardHeader>
      <CardContent>
        <div className="flex items-center justify-between">
          <div className="text-2xl font-bold">{value}</div>
          {Icon && <Icon className="h-8 w-8 text-zinc-400" />}
        </div>
      </CardContent>
    </Card>
  )
}
