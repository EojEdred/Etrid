"use client"

import { useState } from 'react'
import { ArrowLeft, Play, Pause, Trash2, TrendingUp, DollarSign } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Switch } from '@/components/ui/switch'
import { Badge } from '@/components/ui/badge'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useDCABot } from '@/hooks/use-dca-bot'
import type { DCABotInput } from '@/lib/types/trading'

interface DCABotScreenProps {
  onBack: () => void
}

export function DCABotScreen({ onBack }: DCABotScreenProps) {
  const { bots, createBot, startBot, stopBot, deleteBot, getBotMetrics } = useDCABot()

  const [showCreate, setShowCreate] = useState(false)
  const [formData, setFormData] = useState<Partial<DCABotInput>>({
    pair: 'ETRID/USDT',
    amount_per_order: 100,
    frequency: 'daily',
    price_deviation_limit: 5,
  })

  const handleCreateBot = async () => {
    try {
      await createBot(formData as DCABotInput)
      setShowCreate(false)
      setFormData({
        pair: 'ETRID/USDT',
        amount_per_order: 100,
        frequency: 'daily',
        price_deviation_limit: 5,
      })
    } catch (error) {
      console.error('Failed to create bot:', error)
    }
  }

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background border-b p-4">
        <div className="flex items-center justify-between">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <h1 className="text-xl font-bold">DCA Bots</h1>
          <Button onClick={() => setShowCreate(true)}>Create Bot</Button>
        </div>
      </header>

      {/* Main Content */}
      <main className="p-4 space-y-6">
        {/* Create Bot Form */}
        {showCreate && (
          <Card className="p-4 space-y-4">
            <h2 className="font-semibold">Create New DCA Bot</h2>

            <div className="space-y-3">
              {/* Trading Pair */}
              <div className="space-y-2">
                <Label>Trading Pair</Label>
                <Select
                  value={formData.pair}
                  onValueChange={(value) => setFormData({ ...formData, pair: value })}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="ETRID/USDT">ETRID/USDT</SelectItem>
                    <SelectItem value="BTC/USDT">BTC/USDT</SelectItem>
                    <SelectItem value="ETH/USDT">ETH/USDT</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Amount Per Order */}
              <div className="space-y-2">
                <Label>Amount Per Order (USDT)</Label>
                <Input
                  type="number"
                  value={formData.amount_per_order}
                  onChange={(e) =>
                    setFormData({ ...formData, amount_per_order: parseFloat(e.target.value) })
                  }
                />
              </div>

              {/* Frequency */}
              <div className="space-y-2">
                <Label>Frequency</Label>
                <Select
                  value={formData.frequency}
                  onValueChange={(value: any) => setFormData({ ...formData, frequency: value })}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="hourly">Hourly</SelectItem>
                    <SelectItem value="daily">Daily</SelectItem>
                    <SelectItem value="weekly">Weekly</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Price Deviation Limit */}
              <div className="space-y-2">
                <Label>Price Deviation Limit (%)</Label>
                <Input
                  type="number"
                  value={formData.price_deviation_limit}
                  onChange={(e) =>
                    setFormData({
                      ...formData,
                      price_deviation_limit: parseFloat(e.target.value),
                    })
                  }
                  placeholder="Optional"
                />
                <p className="text-xs text-muted-foreground">
                  Skip orders if price deviates more than this percentage
                </p>
              </div>

              {/* Actions */}
              <div className="flex gap-2">
                <Button variant="outline" className="flex-1" onClick={() => setShowCreate(false)}>
                  Cancel
                </Button>
                <Button className="flex-1" onClick={handleCreateBot}>
                  Create Bot
                </Button>
              </div>
            </div>
          </Card>
        )}

        {/* Active Bots */}
        <Tabs defaultValue="active" className="w-full">
          <TabsList className="w-full grid grid-cols-2">
            <TabsTrigger value="active">
              Active ({bots.filter((b) => b.active).length})
            </TabsTrigger>
            <TabsTrigger value="inactive">
              Inactive ({bots.filter((b) => !b.active).length})
            </TabsTrigger>
          </TabsList>

          <TabsContent value="active" className="mt-4 space-y-3">
            {bots
              .filter((bot) => bot.active)
              .map((bot) => (
                <BotCard
                  key={bot.id}
                  bot={bot}
                  onToggle={() => stopBot(bot.id)}
                  onDelete={() => deleteBot(bot.id)}
                  onViewMetrics={() => getBotMetrics(bot.id)}
                />
              ))}

            {bots.filter((b) => b.active).length === 0 && (
              <div className="text-center py-12 text-muted-foreground">
                No active bots
              </div>
            )}
          </TabsContent>

          <TabsContent value="inactive" className="mt-4 space-y-3">
            {bots
              .filter((bot) => !bot.active)
              .map((bot) => (
                <BotCard
                  key={bot.id}
                  bot={bot}
                  onToggle={() => startBot(bot.id)}
                  onDelete={() => deleteBot(bot.id)}
                  onViewMetrics={() => getBotMetrics(bot.id)}
                />
              ))}

            {bots.filter((b) => !b.active).length === 0 && (
              <div className="text-center py-12 text-muted-foreground">
                No inactive bots
              </div>
            )}
          </TabsContent>
        </Tabs>
      </main>
    </div>
  )
}

function BotCard({
  bot,
  onToggle,
  onDelete,
  onViewMetrics,
}: {
  bot: any
  onToggle: () => void
  onDelete: () => void
  onViewMetrics: () => void
}) {
  const pnlPercentage =
    bot.total_invested > 0
      ? ((bot.average_price * bot.total_tokens - bot.total_invested) / bot.total_invested) * 100
      : 0

  return (
    <Card className="p-4">
      <div className="space-y-4">
        {/* Header */}
        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-2">
              <h3 className="font-semibold">{bot.pair}</h3>
              <Badge variant={bot.active ? 'default' : 'secondary'}>
                {bot.active ? 'Active' : 'Paused'}
              </Badge>
            </div>
            <p className="text-sm text-muted-foreground mt-1">
              {bot.amount_per_order} USDT • {bot.frequency}
            </p>
          </div>

          <div className="flex gap-1">
            <Button variant="ghost" size="icon-sm" onClick={onToggle}>
              {bot.active ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
            </Button>
            <Button variant="ghost" size="icon-sm" onClick={onDelete}>
              <Trash2 className="w-4 h-4 text-destructive" />
            </Button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-3 gap-4">
          <div>
            <p className="text-xs text-muted-foreground">Invested</p>
            <p className="font-semibold">${bot.total_invested.toFixed(2)}</p>
          </div>
          <div>
            <p className="text-xs text-muted-foreground">Avg Price</p>
            <p className="font-semibold">${bot.average_price.toFixed(4)}</p>
          </div>
          <div>
            <p className="text-xs text-muted-foreground">P&L</p>
            <p
              className={`font-semibold ${
                pnlPercentage >= 0 ? 'text-green-500' : 'text-red-500'
              }`}
            >
              {pnlPercentage >= 0 ? '+' : ''}
              {pnlPercentage.toFixed(2)}%
            </p>
          </div>
        </div>

        {/* Next Run */}
        {bot.active && (
          <div className="text-xs text-muted-foreground">
            Next run: {new Date(bot.next_run).toLocaleString()}
          </div>
        )}
      </div>
    </Card>
  )
}
