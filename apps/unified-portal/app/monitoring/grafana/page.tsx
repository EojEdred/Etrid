'use client'

import { useState } from 'react'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { BarChart3, ExternalLink } from 'lucide-react'
import { Button } from '@/components/ui/button'

type DashboardType = 'primearc' | 'validators' | 'network'

const dashboards = {
  primearc: {
    name: 'PrimeArc Core Chain',
    id: 'primearc-dashboard',
    description: 'Core blockchain metrics, block production, and consensus'
  },
  validators: {
    name: 'Validators',
    id: 'validator-dashboard',
    description: 'Validator performance, rewards, and uptime tracking'
  },
  network: {
    name: 'Network Overview',
    id: 'network-dashboard',
    description: 'Network health, peer connections, and system resources'
  }
}

export default function GrafanaPage() {
  const [selectedDashboard, setSelectedDashboard] = useState<DashboardType>('primearc')

  return (
    <div className="container mx-auto p-8 h-screen flex flex-col">
      {/* Header */}
      <div className="mb-6">
        <div className="flex items-center justify-between">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <BarChart3 className="h-8 w-8 text-purple-500" />
              <h1 className="text-4xl font-bold">Grafana Dashboards</h1>
            </div>
            <p className="text-zinc-600 dark:text-zinc-400">
              Visual metrics and analytics for ËTRID network
            </p>
          </div>
          <Button
            variant="outline"
            onClick={() => window.open('http://localhost:3100', '_blank')}
          >
            <ExternalLink className="w-4 h-4 mr-2" />
            Open Full Grafana UI
          </Button>
        </div>
      </div>

      {/* Dashboard Selector */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>Select Dashboard</CardTitle>
        </CardHeader>
        <CardContent>
          <Tabs value={selectedDashboard} onValueChange={(v) => setSelectedDashboard(v as DashboardType)}>
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="primearc">PrimeArc Core</TabsTrigger>
              <TabsTrigger value="validators">Validators</TabsTrigger>
              <TabsTrigger value="network">Network</TabsTrigger>
            </TabsList>
          </Tabs>
          <p className="text-sm text-muted-foreground mt-3">
            {dashboards[selectedDashboard].description}
          </p>
        </CardContent>
      </Card>

      {/* Embedded Grafana Dashboard */}
      <Card className="flex-1 flex flex-col">
        <CardHeader>
          <CardTitle>{dashboards[selectedDashboard].name}</CardTitle>
        </CardHeader>
        <CardContent className="flex-1 p-0">
          <div className="w-full h-full min-h-[600px] bg-muted/10 rounded-lg overflow-hidden">
            <iframe
              src={`http://localhost:3100/d/${dashboards[selectedDashboard].id}?orgId=1&refresh=10s&kiosk`}
              className="w-full h-full border-0"
              title={`Grafana - ${dashboards[selectedDashboard].name}`}
              allow="fullscreen"
            />
          </div>
          <div className="p-4 text-center text-sm text-muted-foreground border-t mt-4">
            Note: Grafana must be running on localhost:3100. If you see an error, ensure Grafana is started.
          </div>
        </CardContent>
      </Card>

      {/* Dashboard Info */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
        {Object.entries(dashboards).map(([key, dashboard]) => (
          <Card key={key} className={selectedDashboard === key ? 'border-purple-500 border-2' : ''}>
            <CardHeader>
              <CardTitle className="text-base">{dashboard.name}</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground">{dashboard.description}</p>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}
