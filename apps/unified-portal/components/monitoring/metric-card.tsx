import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

interface MetricCardProps {
  title: string
  value: string | number
  query?: string
  loading?: boolean
  error?: string
  chart?: React.ReactNode
}

export function MetricCard({ title, value, query, loading, error, chart }: MetricCardProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm font-medium text-muted-foreground">
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {loading ? (
          <div className="animate-pulse">
            <div className="h-8 bg-gray-200 dark:bg-gray-700 rounded w-20"></div>
          </div>
        ) : error ? (
          <div className="text-red-600 text-sm">{error}</div>
        ) : (
          <>
            <div className="text-2xl font-bold mb-2">{value}</div>
            {query && (
              <code className="text-xs text-muted-foreground bg-muted px-2 py-1 rounded">
                {query}
              </code>
            )}
            {chart && (
              <div className="mt-4">
                {chart}
              </div>
            )}
          </>
        )}
      </CardContent>
    </Card>
  )
}
