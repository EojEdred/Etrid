"use client"

import { Shield, Calendar, Link2, Coins } from "lucide-react"
import { useEffect, useRef, useState } from "react"

const features = [
  {
    icon: Shield,
    title: "Ascending Scale of Finality",
    description:
      "Novel consensus combining stake with time-weighted voting to prevent centralization and ensure network security.",
  },
  {
    icon: Calendar,
    title: "Consensus Day Governance",
    description: "Annual on-chain voting event where the community decides fiscal policy and upgrades democratically.",
  },
  {
    icon: Link2,
    title: "Partition Burst Chains",
    description: "Unlimited sidechains for application-specific use cases with high throughput and scalability.",
  },
  {
    icon: Coins,
    title: "Dual Token System",
    description:
      "ÉTR for governance and EDSC stablecoin for everyday transactions, providing flexibility and stability.",
  },
]

export default function Features() {
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
              <div
                key={feature.title}
                className={`glass rounded-2xl p-8 hover:scale-105 transition-all duration-300 ${
                  isVisible ? "animate-fade-in-up" : "opacity-0"
                }`}
                style={{ animationDelay: `${index * 100}ms` }}
              >
                <div className="w-16 h-16 rounded-xl bg-primary/20 flex items-center justify-center mb-6">
                  <Icon className="w-8 h-8 text-primary" />
                </div>
                <h3 className="text-2xl font-semibold mb-4">{feature.title}</h3>
                <p className="text-muted-foreground leading-relaxed">{feature.description}</p>
              </div>
            )
          })}
        </div>
      </div>
    </section>
  )
}
