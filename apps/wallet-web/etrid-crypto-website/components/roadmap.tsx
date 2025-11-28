"use client"

import { CheckCircle2, Circle, Clock } from "lucide-react"

const milestones = [
  {
    quarter: "Q1 2026",
    title: "Testnet Launch",
    description: "Public testnet deployed with full functionality for community testing and feedback.",
    status: "completed",
  },
  {
    quarter: "Q2 2026",
    title: "Security Audits",
    description: "Comprehensive security audits by leading blockchain security firms.",
    status: "in-progress",
  },
  {
    quarter: "Q3 2026",
    title: "Mainnet Launch",
    description: "Official mainnet launch with full decentralization and token distribution.",
    status: "upcoming",
  },
  {
    quarter: "Q4 2026",
    title: "First Consensus Day",
    description: "Inaugural governance event where stakeholders vote on network parameters.",
    status: "upcoming",
  },
]

export default function Roadmap() {
  return (
    <section className="py-24 px-4 bg-gradient-to-b from-secondary/10 to-background">
      <div className="container mx-auto max-w-5xl">
        <h2 className="text-4xl md:text-5xl font-bold text-center mb-16">Launch Timeline</h2>

        <div className="relative">
          {/* Vertical line */}
          <div className="absolute left-8 top-0 bottom-0 w-0.5 bg-border hidden md:block" />

          <div className="space-y-12">
            {milestones.map((milestone, index) => (
              <div key={milestone.quarter} className="relative flex gap-8 items-start">
                {/* Icon */}
                <div className="relative z-10 flex-shrink-0">
                  {milestone.status === "completed" && <CheckCircle2 className="w-16 h-16 text-success" />}
                  {milestone.status === "in-progress" && <Clock className="w-16 h-16 text-warning" />}
                  {milestone.status === "upcoming" && <Circle className="w-16 h-16 text-muted-foreground" />}
                </div>

                {/* Content */}
                <div className="glass rounded-2xl p-8 flex-1">
                  <div className="flex items-center gap-4 mb-4">
                    <span className="text-sm font-semibold text-primary">{milestone.quarter}</span>
                    <span
                      className={`text-xs px-3 py-1 rounded-full ${
                        milestone.status === "completed"
                          ? "bg-success/20 text-success"
                          : milestone.status === "in-progress"
                            ? "bg-warning/20 text-warning"
                            : "bg-muted text-muted-foreground"
                      }`}
                    >
                      {milestone.status === "completed"
                        ? "✅ Completed"
                        : milestone.status === "in-progress"
                          ? "🟡 In Progress"
                          : "🔴 Upcoming"}
                    </span>
                  </div>
                  <h3 className="text-2xl font-semibold mb-3">{milestone.title}</h3>
                  <p className="text-muted-foreground leading-relaxed">{milestone.description}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  )
}
