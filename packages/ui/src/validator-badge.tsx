import * as React from "react"
import { cn } from "./utils"
import { Badge } from "./badge"

interface ValidatorBadgeProps extends React.HTMLAttributes<HTMLDivElement> {
  status: 'active' | 'inactive' | 'offline'
  showText?: boolean
}

export function ValidatorBadge({
  status,
  showText = true,
  className,
  ...props
}: ValidatorBadgeProps) {
  const statusConfig = {
    active: {
      color: 'bg-green-500',
      text: 'Active',
      variant: 'success' as const
    },
    inactive: {
      color: 'bg-yellow-500',
      text: 'Inactive',
      variant: 'warning' as const
    },
    offline: {
      color: 'bg-red-500',
      text: 'Offline',
      variant: 'destructive' as const
    }
  }

  const config = statusConfig[status]

  if (!showText) {
    return (
      <span
        className={cn(
          "inline-block w-2 h-2 rounded-full",
          config.color,
          className
        )}
        title={config.text}
        {...props}
      />
    )
  }

  return (
    <Badge variant={config.variant} className={className} {...props}>
      <span className={cn("inline-block w-2 h-2 rounded-full mr-1.5", config.color)} />
      {config.text}
    </Badge>
  )
}
