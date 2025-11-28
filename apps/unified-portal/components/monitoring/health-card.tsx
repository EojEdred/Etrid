import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

type HealthStatus = 'healthy' | 'warning' | 'critical' | 'offline'

interface HealthCardProps {
  system: string
  status: HealthStatus
  metrics?: Record<string, string | number>
  lastUpdate?: string
}

const statusConfig = {
  healthy: {
    color: 'text-green-600',
    bgColor: 'bg-green-50 dark:bg-green-950',
    borderColor: 'border-green-200 dark:border-green-800',
    icon: '✓'
  },
  warning: {
    color: 'text-yellow-600',
    bgColor: 'bg-yellow-50 dark:bg-yellow-950',
    borderColor: 'border-yellow-200 dark:border-yellow-800',
    icon: '⚠'
  },
  critical: {
    color: 'text-red-600',
    bgColor: 'bg-red-50 dark:bg-red-950',
    borderColor: 'border-red-200 dark:border-red-800',
    icon: '✗'
  },
  offline: {
    color: 'text-gray-600',
    bgColor: 'bg-gray-50 dark:bg-gray-950',
    borderColor: 'border-gray-200 dark:border-gray-800',
    icon: '○'
  }
}

export function HealthCard({ system, status, metrics, lastUpdate }: HealthCardProps) {
  const config = statusConfig[status]

  return (
    <Card className={`${config.borderColor} border-2`}>
      <CardHeader className={config.bgColor}>
        <CardTitle className="flex items-center justify-between">
          <span className="text-base">{system}</span>
          <span className={`${config.color} text-2xl font-bold`}>
            {config.icon}
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="p-6">
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium">Status</span>
            <span className={`${config.color} font-semibold capitalize`}>
              {status}
            </span>
          </div>
          {metrics && Object.entries(metrics).map(([key, value]) => (
            <div key={key} className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground capitalize">
                {key.replace(/([A-Z])/g, ' $1').trim()}
              </span>
              <span className="font-medium">{value}</span>
            </div>
          ))}
          {lastUpdate && (
            <div className="text-xs text-muted-foreground pt-2 border-t">
              Updated: {lastUpdate}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
