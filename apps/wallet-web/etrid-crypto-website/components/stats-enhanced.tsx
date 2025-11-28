"use client"

/**
 * Enhanced Stats Component
 * Uses AnimatedCounter from Uiverse.io-inspired components
 */

import { AnimatedCounter } from "@/components/ui/animated-counter"
import { GlassCard } from "@/components/ui/glass-card"

const stats = [
  {
    value: 1000000000,
    suffix: " ÉTR",
    label: "Total Supply",
    color: "text-blue-400"
  },
  {
    value: 12.5,
    decimals: 1,
    suffix: "% APY",
    label: "Staking Rewards",
    color: "text-green-400"
  },
  {
    value: 3,
    suffix: " Seconds",
    label: "Block Time",
    color: "text-purple-400"
  },
  {
    value: 500,
    suffix: "+ Validators",
    label: "Network Security",
    color: "text-amber-400"
  },
]

export default function StatsEnhanced() {
  return (
    <section className="py-24 px-4 bg-gradient-to-b from-background to-secondary/10">
      <div className="container mx-auto max-w-7xl">
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
          {stats.map((stat, index) => (
            <GlassCard
              key={stat.label}
              variant="glow"
              className="p-8 text-center"
              style={{ animationDelay: `${index * 100}ms` } as React.CSSProperties}
            >
              <div className="space-y-4">
                <AnimatedCounter
                  value={stat.value}
                  decimals={stat.decimals || 0}
                  suffix={stat.suffix}
                  duration={2000}
                  className={`text-4xl md:text-5xl font-bold ${stat.color}`}
                />
                <p className="text-lg text-muted-foreground">{stat.label}</p>
              </div>

              {/* Animated bottom border */}
              <div className="mt-6 h-1 w-full bg-gradient-to-r from-transparent via-primary/50 to-transparent rounded-full" />
            </GlassCard>
          ))}
        </div>
      </div>
    </section>
  )
}

/**
 * Usage:
 * Replace the regular Stats import in page.tsx:
 *
 * Before:
 * import Stats from "@/components/stats"
 *
 * After:
 * import Stats from "@/components/stats-enhanced"
 */
