"use client"

/**
 * Enhanced Features Component
 * Uses GlassCard from Uiverse.io-inspired components
 */

import { Shield, Calendar, Link2, Coins } from "lucide-react"
import { useEffect, useRef, useState } from "react"
import { GlassCard } from "@/components/ui/glass-card"

const features = [
  {
    icon: Shield,
    title: "Ascending Scale of Finality",
    description:
      "Novel consensus combining stake with time-weighted voting to prevent centralization and ensure network security.",
    color: "text-blue-400"
  },
  {
    icon: Calendar,
    title: "Consensus Day Governance",
    description: "Annual on-chain voting event where the community decides fiscal policy and upgrades democratically.",
    color: "text-purple-400"
  },
  {
    icon: Link2,
    title: "Partition Burst Chains",
    description: "Unlimited sidechains for application-specific use cases with high throughput and scalability.",
    color: "text-green-400"
  },
  {
    icon: Coins,
    title: "Dual Token System",
    description:
      "ÉTR for governance and EDSC stablecoin for everyday transactions, providing flexibility and stability.",
    color: "text-amber-400"
  },
]

export default function FeaturesEnhanced() {
  const [isVisible, setIsVisible] = useState(false)
  const sectionRef = useRef<HTMLElement>(null)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
        }
      },
      { threshold: 0.1 },
    )

    if (sectionRef.current) {
      observer.observe(sectionRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <section ref={sectionRef} className="py-24 px-4 bg-background">
      <div className="container mx-auto max-w-7xl">
        <h2 className="text-4xl md:text-5xl font-bold text-center mb-16">Why Ëtrid?</h2>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          {features.map((feature, index) => {
            const Icon = feature.icon
            return (
              <GlassCard
                key={feature.title}
                variant="glow"
                className={`p-8 ${
                  isVisible ? "animate-fade-in-up" : "opacity-0"
                }`}
                style={{ animationDelay: `${index * 100}ms` } as React.CSSProperties}
              >
                {/* Animated icon container */}
                <div className="relative w-16 h-16 rounded-xl bg-gradient-to-br from-primary/30 to-primary/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform">
                  <div className="absolute inset-0 bg-gradient-to-br from-primary/40 to-transparent rounded-xl animate-pulse" />
                  <Icon className={`w-8 h-8 ${feature.color} relative z-10`} />
                </div>

                <h3 className="text-2xl font-semibold mb-4">{feature.title}</h3>
                <p className="text-muted-foreground leading-relaxed">{feature.description}</p>

                {/* Decorative gradient line */}
                <div className="mt-6 h-1 w-full bg-gradient-to-r from-transparent via-primary/50 to-transparent rounded-full" />
              </GlassCard>
            )
          })}
        </div>
      </div>
    </section>
  )
}

/**
 * Usage:
 * Replace the regular Features import in page.tsx:
 *
 * Before:
 * import Features from "@/components/features"
 *
 * After:
 * import Features from "@/components/features-enhanced"
 */
