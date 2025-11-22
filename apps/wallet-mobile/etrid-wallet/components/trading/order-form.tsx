"use client"

import { useState, useEffect } from 'react'
import { cn } from '@/lib/utils'
import { Card } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Slider } from '@/components/ui/slider'
import type { OrderInput, OrderType } from '@/lib/types/trading'

interface OrderFormProps {
  pair: string
  currentPrice: number
  availableBalance: number
  currency: string
  onSubmit: (order: OrderInput) => Promise<void>
  className?: string
}

export function OrderForm({
  pair,
  currentPrice,
  availableBalance,
  currency,
  onSubmit,
  className,
}: OrderFormProps) {
  const [side, setSide] = useState<'buy' | 'sell'>('buy')
  const [orderType, setOrderType] = useState<OrderType>('limit')
  const [amount, setAmount] = useState('')
  const [price, setPrice] = useState(currentPrice.toString())
  const [stopPrice, setStopPrice] = useState('')
  const [slippage, setSlippage] = useState(0.5)
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    if (orderType === 'market') {
      setPrice(currentPrice.toString())
    }
  }, [currentPrice, orderType])

  const calculateTotal = () => {
    const amountNum = parseFloat(amount) || 0
    const priceNum = parseFloat(price) || currentPrice
    return amountNum * priceNum
  }

  const calculateFee = () => {
    return calculateTotal() * 0.001 // 0.1% fee
  }

  const setAmountPercentage = (percentage: number) => {
    const total = availableBalance * percentage
    if (side === 'buy') {
      const priceNum = parseFloat(price) || currentPrice
      setAmount((total / priceNum).toFixed(6))
    } else {
      setAmount(total.toFixed(6))
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    setLoading(true)
    try {
      const order: OrderInput = {
        pair,
        type: orderType,
        side,
        amount: parseFloat(amount),
        ...(orderType !== 'market' && { price: parseFloat(price) }),
        ...(orderType === 'stop_loss' || orderType === 'stop_limit') && {
          stop_price: parseFloat(stopPrice),
        },
        slippage_tolerance: slippage,
      }

      await onSubmit(order)

      // Reset form
      setAmount('')
      if (orderType !== 'market') {
        setPrice(currentPrice.toString())
      }
      setStopPrice('')
    } catch (error) {
      console.error('Order submission error:', error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <Card className={cn('p-4', className)}>
      <Tabs value={side} onValueChange={(v) => setSide(v as 'buy' | 'sell')}>
        <TabsList className="w-full grid grid-cols-2">
          <TabsTrigger value="buy" className="text-green-500">Buy</TabsTrigger>
          <TabsTrigger value="sell" className="text-red-500">Sell</TabsTrigger>
        </TabsList>

        <form onSubmit={handleSubmit} className="mt-4 space-y-4">
          {/* Order Type */}
          <div className="space-y-2">
            <Label>Order Type</Label>
            <Select value={orderType} onValueChange={(v) => setOrderType(v as OrderType)}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="market">Market</SelectItem>
                <SelectItem value="limit">Limit</SelectItem>
                <SelectItem value="stop_loss">Stop Loss</SelectItem>
                <SelectItem value="stop_limit">Stop Limit</SelectItem>
              </SelectContent>
            </Select>
          </div>

          {/* Price (for limit orders) */}
          {orderType !== 'market' && (
            <div className="space-y-2">
              <Label>Price</Label>
              <div className="relative">
                <Input
                  type="number"
                  step="0.01"
                  value={price}
                  onChange={(e) => setPrice(e.target.value)}
                  placeholder="0.00"
                  required
                />
                <span className="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-muted-foreground">
                  {currency}
                </span>
              </div>
            </div>
          )}

          {/* Stop Price (for stop orders) */}
          {(orderType === 'stop_loss' || orderType === 'stop_limit') && (
            <div className="space-y-2">
              <Label>Stop Price</Label>
              <Input
                type="number"
                step="0.01"
                value={stopPrice}
                onChange={(e) => setStopPrice(e.target.value)}
                placeholder="0.00"
                required
              />
            </div>
          )}

          {/* Amount */}
          <div className="space-y-2">
            <Label>Amount</Label>
            <Input
              type="number"
              step="0.000001"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.00"
              required
            />

            {/* Quick percentage buttons */}
            <div className="grid grid-cols-4 gap-2">
              {[0.25, 0.5, 0.75, 1].map((pct) => (
                <Button
                  key={pct}
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => setAmountPercentage(pct)}
                >
                  {pct * 100}%
                </Button>
              ))}
            </div>
          </div>

          {/* Slippage (for market orders) */}
          {orderType === 'market' && (
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label>Slippage Tolerance</Label>
                <span className="text-sm text-muted-foreground">{slippage}%</span>
              </div>
              <Slider
                value={[slippage]}
                onValueChange={(values) => setSlippage(values[0])}
                min={0.1}
                max={5}
                step={0.1}
              />
            </div>
          )}

          {/* Summary */}
          <div className="space-y-2 p-3 rounded-lg bg-muted/50">
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Available</span>
              <span className="font-medium">
                {availableBalance.toFixed(6)} {side === 'buy' ? currency : pair.split('/')[0]}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Total</span>
              <span className="font-medium">
                {calculateTotal().toFixed(2)} {currency}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-muted-foreground">Fee (0.1%)</span>
              <span className="font-medium">
                {calculateFee().toFixed(2)} {currency}
              </span>
            </div>
          </div>

          {/* Submit Button */}
          <Button
            type="submit"
            className="w-full"
            variant={side === 'buy' ? 'default' : 'destructive'}
            disabled={loading || !amount || (orderType !== 'market' && !price)}
          >
            {loading ? 'Processing...' : `${side === 'buy' ? 'Buy' : 'Sell'} ${pair.split('/')[0]}`}
          </Button>
        </form>
      </Tabs>
    </Card>
  )
}
