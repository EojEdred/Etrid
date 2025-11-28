import * as React from "react"
import { cn } from "./utils"
import { Badge } from "./badge"
import { Activity, AlertCircle } from "lucide-react"

interface NetworkStatusProps extends React.HTMLAttributes<HTMLDivElement> {
  status: 'connected' | 'connecting' | 'disconnected'
  peersCount?: number
  showPeers?: boolean
}

export function NetworkStatus({
  status,
  peersCount,
  showPeers = true,
  className,
  ...props
}: NetworkStatusProps) {
  const statusConfig = {
    connected: {
      icon: Activity,
      text: 'Connected',
      variant: 'success' as const
    },
    connecting: {
      icon: Activity,
      text: 'Connecting',
      variant: 'warning' as const
    },
    disconnected: {
      icon: AlertCircle,
      text: 'Disconnected',
      variant: 'destructive' as const
    }
  }

  const config = statusConfig[status]
  const Icon = config.icon

  return (
    <div
      className={cn("inline-flex items-center gap-2", className)}
      {...props}
    >
      <Badge variant={config.variant}>
        <Icon className="h-3 w-3 mr-1" />
        {config.text}
      </Badge>
      {showPeers && peersCount !== undefined && status === 'connected' && (
        <span className="text-sm text-muted-foreground">
          {peersCount} {peersCount === 1 ? 'peer' : 'peers'}
        </span>
      )}
    </div>
  )
}
