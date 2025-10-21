"use client"

import type React from "react"

import { Button } from "@/components/ui/button"
import { ChevronDown } from "lucide-react"
import { useEffect, useState } from "react"
import Link from "next/link"

export default function Hero() {
  const [particles, setParticles] = useState<
    Array<{ id: number; x: number; y: number; duration: number; tx: number; ty: number }>
  >([])

  useEffect(() => {
    // Generate random particles
    const newParticles = Array.from({ length: 50 }, (_, i) => ({
      id: i,
      x: Math.random() * 100,
      y: Math.random() * 100,
      duration: 10 + Math.random() * 20,
      tx: (Math.random() - 0.5) * 100,
      ty: (Math.random() - 0.5) * 100,
    }))
    setParticles(newParticles)
  }, [])

  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden gradient-bg">
      {/* Animated particles */}
      <div className="absolute inset-0 overflow-hidden">
        {particles.map((particle) => (
          <div
            key={particle.id}
            className="particle absolute w-1 h-1 bg-white rounded-full"
            style={
              {
                left: `${particle.x}%`,
                top: `${particle.y}%`,
                "--duration": `${particle.duration}s`,
                "--tx": `${particle.tx}px`,
                "--ty": `${particle.ty}px`,
              } as React.CSSProperties
            }
          />
        ))}
      </div>

      {/* Content */}
      <div className="relative z-10 container mx-auto px-4 text-center">
        <div className="max-w-5xl mx-auto space-y-8">
          {/* Logo */}
          <h1 className="text-7xl md:text-9xl font-bold tracking-tight">ËTRID</h1>

          {/* Tagline */}
          <p className="text-2xl md:text-3xl font-semibold text-balance">
            The Free and Open Decentralized Democracy of Stakeholders
          </p>

          {/* Subtitle */}
          <p className="text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto text-balance">
            Multichain blockchain with on-chain governance and adaptive consensus
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center pt-8">
            <Button size="lg" className="bg-primary text-primary-foreground hover:bg-primary/90 glow text-lg px-8 py-6">
              Launch App
            </Button>
            <Link href="/governance">
              <Button
                size="lg"
                variant="outline"
                className="border-2 border-accent text-accent hover:bg-accent/10 text-lg px-8 py-6 bg-transparent"
              >
                Governance
              </Button>
            </Link>
            <Link href="/swap">
              <Button
                size="lg"
                variant="outline"
                className="border-2 border-success text-success hover:bg-success/10 text-lg px-8 py-6 bg-transparent"
              >
                Swap
              </Button>
            </Link>
            <Button
              size="lg"
              variant="outline"
              className="border-2 border-white text-white hover:bg-white/10 text-lg px-8 py-6 bg-transparent"
            >
              Read Whitepaper
            </Button>
          </div>
        </div>
      </div>

      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce">
        <ChevronDown className="w-8 h-8 text-white/60" />
      </div>
    </section>
  )
}
