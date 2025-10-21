"use client"

import { Globe, GitBranch, Zap } from "lucide-react"
import { useState } from "react"

const layers = [
  {
    icon: Globe,
    title: "Layer 1: Flare Chain",
    subtitle: "Main Chain",
    description: "The foundational layer providing security, consensus, and governance for the entire Ëtrid ecosystem.",
  },
  {
    icon: GitBranch,
    title: "Layer 2: Partition Burst Chains",
    subtitle: "Sidechains",
    description:
      "Application-specific chains that enable unlimited scalability and customization for diverse use cases.",
  },
  {
    icon: Zap,
    title: "Layer 3: State Channels",
    subtitle: "Lightning Fast",
    description:
      "Off-chain payment channels for instant, low-cost transactions with final settlement on the main chain.",
  },
]

export default function Architecture() {
  const [expandedIndex, setExpandedIndex] = useState<number | null>(null)

  return (
    <section className="py-24 px-4 bg-background">
      <div className="container mx-auto max-w-7xl">
        <h2 className="text-4xl md:text-5xl font-bold text-center mb-16">Three-Layer Architecture</h2>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {layers.map((layer, index) => {
            const Icon = layer.icon
            const isExpanded = expandedIndex === index

            return (
              <div
                key={layer.title}
                className="glass rounded-2xl p-8 cursor-pointer hover:scale-105 transition-all duration-300"
                onClick={() => setExpandedIndex(isExpanded ? null : index)}
              >
                <div className="flex flex-col items-center text-center space-y-6">
                  <div className="w-20 h-20 rounded-full bg-primary/20 flex items-center justify-center">
                    <Icon className="w-10 h-10 text-primary" />
                  </div>
                  <div>
                    <p className="text-sm text-primary font-semibold mb-2">{layer.subtitle}</p>
                    <h3 className="text-xl font-semibold mb-4">{layer.title}</h3>
                  </div>
                  <div
                    className={`overflow-hidden transition-all duration-300 ${
                      isExpanded ? "max-h-96 opacity-100" : "max-h-0 opacity-0"
                    }`}
                  >
                    <p className="text-muted-foreground leading-relaxed">{layer.description}</p>
                  </div>
                  <button className="text-primary text-sm font-semibold">
                    {isExpanded ? "Show Less" : "Learn More"}
                  </button>
                </div>
              </div>
            )
          })}
        </div>
      </div>
    </section>
  )
}
