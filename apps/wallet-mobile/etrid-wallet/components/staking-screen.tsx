"use client"

import { useState } from "react"
import { Lock, Info, TrendingUp, AlertCircle } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { BottomNav } from "@/components/bottom-nav"

interface StakingScreenProps {
  onBack: () => void
  activeTab: string
  onTabChange: (tab: string) => void
}

const stakingTiers = [
  { min: 0, max: 63, name: "Common Peer", rewards: "No rewards", color: "text-muted-foreground" },
  { min: 64, max: 127, name: "Common Stake Peer", rewards: "Distribution pay", color: "text-success" },
  { min: 128, max: 255, name: "Flare Node", rewards: "Block rewards", color: "text-accent" },
  { min: 256, max: Number.POSITIVE_INFINITY, name: "Validity Node", rewards: "Extra rewards", color: "text-warning" },
]

export function StakingScreen({ onBack, activeTab, onTabChange }: StakingScreenProps) {
  const currentStake = 500
  const availableBalance = 734.56
  const [stakeAmount, setStakeAmount] = useState("")
  const [lockPeriod, setLockPeriod] = useState("90")

  const totalStake = currentStake + Number.parseFloat(stakeAmount || "0")
  const currentTier = stakingTiers.find((tier) => currentStake >= tier.min && currentStake <= tier.max)
  const nextTier = stakingTiers.find((tier) => totalStake >= tier.min && totalStake <= tier.max)

  const apyRates: Record<string, number> = {
    "0": 5,
    "30": 7,
    "90": 8.5,
    "365": 10,
  }

  const apy = apyRates[lockPeriod] || 5
  const dailyEarnings = (totalStake * (apy / 100)) / 365
  const monthlyEarnings = dailyEarnings * 30
  const annualEarnings = totalStake * (apy / 100)

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="p-6">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-12 h-12 rounded-full bg-primary/20 flex items-center justify-center">
            <Lock className="w-6 h-6 text-primary" />
          </div>
          <div className="flex-1">
            <h1 className="text-2xl font-bold">Stake ÉTR</h1>
            <p className="text-sm text-muted-foreground">Earn rewards by securing the network</p>
          </div>
          <Button variant="ghost" size="icon" className="text-muted-foreground">
            <Info className="w-5 h-5" />
          </Button>
        </div>

        {/* Current Staking Summary */}
        <div className="glass-strong rounded-2xl p-5 space-y-3">
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">Currently Staked</span>
            <span className="text-xs px-2 py-1 rounded-full bg-success/20 text-success">{currentTier?.name}</span>
          </div>
          <div>
            <p className="text-3xl font-bold">{currentStake.toFixed(2)} ÉTR</p>
            <p className="text-sm text-muted-foreground">≈ ${(currentStake * 8).toFixed(2)}</p>
          </div>
          <div className="flex items-center justify-between pt-3 border-t border-border">
            <span className="text-sm text-muted-foreground">Estimated APY</span>
            <span className="text-lg font-bold text-success">{apy}%</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <TrendingUp className="w-4 h-4 text-success" />
            <span className="text-success">+12.45 ÉTR earned this month</span>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="px-6 space-y-6">
        {/* Stake More Section */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <Label className="text-base font-semibold">Stake Amount</Label>
            <span className="text-sm text-muted-foreground">Available: {availableBalance.toFixed(2)} ÉTR</span>
          </div>

          <div className="glass rounded-2xl p-4">
            <div className="flex items-center gap-2 mb-4">
              <Input
                type="number"
                placeholder="0.00"
                value={stakeAmount}
                onChange={(e) => {
                  const value = Number.parseFloat(e.target.value)
                  if (value <= availableBalance) {
                    setStakeAmount(e.target.value)
                  }
                }}
                className="text-2xl font-bold bg-transparent border-0 p-0 h-auto focus-visible:ring-0"
              />
              <span className="text-xl font-semibold text-muted-foreground">ÉTR</span>
              <Button
                variant="ghost"
                size="sm"
                className="ml-auto text-accent"
                onClick={() => setStakeAmount(availableBalance.toString())}
              >
                Max
              </Button>
            </div>

            <Slider
              value={[Number.parseFloat(stakeAmount || "0")]}
              onValueChange={(value) => setStakeAmount(value[0].toString())}
              max={availableBalance}
              step={1}
              className="mb-2"
            />
          </div>
        </div>

        {/* Staking Tier Indicator */}
        <div className="space-y-3">
          <Label className="text-base font-semibold">Staking Tier</Label>
          <div className="space-y-2">
            {stakingTiers.map((tier, index) => {
              const isCurrentTier = totalStake >= tier.min && totalStake <= tier.max
              const isPassed = totalStake > tier.max

              return (
                <div
                  key={index}
                  className={`glass rounded-xl p-4 transition-all ${
                    isCurrentTier ? "border-2 border-accent bg-accent/5" : isPassed ? "opacity-50" : ""
                  }`}
                >
                  <div className="flex items-center justify-between">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <span className={`font-semibold ${tier.color}`}>{tier.name}</span>
                        {isCurrentTier && (
                          <span className="text-xs px-2 py-0.5 rounded-full bg-accent/20 text-accent">Current</span>
                        )}
                      </div>
                      <p className="text-xs text-muted-foreground">
                        {tier.min}-{tier.max === Number.POSITIVE_INFINITY ? "∞" : tier.max} ÉTR • {tier.rewards}
                      </p>
                    </div>
                    {!isPassed && !isCurrentTier && (
                      <span className="text-xs text-muted-foreground">+{(tier.min - totalStake).toFixed(0)} more</span>
                    )}
                  </div>
                </div>
              )
            })}
          </div>
        </div>

        {/* Rewards Projection */}
        {Number.parseFloat(stakeAmount || "0") > 0 && (
          <div className="glass-strong rounded-2xl p-5 space-y-3">
            <h3 className="font-semibold flex items-center gap-2">
              <TrendingUp className="w-5 h-5 text-success" />
              Projected Earnings
            </h3>
            <div className="grid grid-cols-3 gap-4">
              <div>
                <p className="text-xs text-muted-foreground mb-1">Daily</p>
                <p className="text-lg font-bold">{dailyEarnings.toFixed(2)} ÉTR</p>
              </div>
              <div>
                <p className="text-xs text-muted-foreground mb-1">Monthly</p>
                <p className="text-lg font-bold">{monthlyEarnings.toFixed(2)} ÉTR</p>
              </div>
              <div>
                <p className="text-xs text-muted-foreground mb-1">Annual</p>
                <p className="text-lg font-bold">{annualEarnings.toFixed(2)} ÉTR</p>
              </div>
            </div>
          </div>
        )}

        {/* Lock Period */}
        <div className="space-y-3">
          <Label className="text-base font-semibold">Lock Period</Label>
          <Select value={lockPeriod} onValueChange={setLockPeriod}>
            <SelectTrigger className="glass border-border h-14">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="0">No lock - 5% APY</SelectItem>
              <SelectItem value="30">30 days - 7% APY</SelectItem>
              <SelectItem value="90">90 days - 8.5% APY</SelectItem>
              <SelectItem value="365">365 days - 10% APY</SelectItem>
            </SelectContent>
          </Select>

          {lockPeriod !== "0" && (
            <div className="glass rounded-xl p-3 bg-warning/10 border-warning/20">
              <div className="flex gap-2">
                <AlertCircle className="w-4 h-4 text-warning shrink-0 mt-0.5" />
                <p className="text-xs text-muted-foreground">Early unstaking incurs a 5% penalty</p>
              </div>
            </div>
          )}
        </div>

        {/* Stake Button */}
        <Button
          className="w-full h-14 text-lg font-semibold"
          disabled={!stakeAmount || Number.parseFloat(stakeAmount) <= 0}
          style={{
            background: stakeAmount && Number.parseFloat(stakeAmount) > 0 ? "#00d9ff" : undefined,
            color: stakeAmount && Number.parseFloat(stakeAmount) > 0 ? "#000" : undefined,
          }}
        >
          Stake ÉTR
        </Button>

        {/* Unstake Section */}
        <div className="space-y-3">
          <button className="w-full text-left">
            <Label className="text-base font-semibold cursor-pointer">Your Staked Positions</Label>
          </button>

          <div className="glass rounded-2xl p-4">
            <div className="flex items-center justify-between mb-3">
              <div>
                <p className="font-semibold">{currentStake.toFixed(2)} ÉTR</p>
                <p className="text-xs text-muted-foreground">Locked for 90 days</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-success">+8.5% APY</p>
                <p className="text-xs text-muted-foreground">45 days remaining</p>
              </div>
            </div>
            <Button variant="outline" className="w-full glass border-border bg-transparent" disabled>
              Unstake (Available in 45 days)
            </Button>
          </div>
        </div>
      </main>

      {/* Bottom Navigation */}
      <BottomNav activeTab={activeTab} onTabChange={onTabChange} />
    </div>
  )
}
