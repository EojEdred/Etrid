"use client"

import type React from "react"

import { Button } from "@/components/ui/button"
import { ChevronDown } from "lucide-react"
import Link from "next/link"
import { ModeToggle } from "@/components/mode-toggle"
import UnicornSceneWrapper from "@/components/unicorn-scene"

/**
 * Enhanced Hero Section with 3D Background
 *
 * This replaces the simple particle animation with a full 3D Unicorn Studio scene
 *
 * To use:
 * 1. Create a 3D scene at https://unicorn.studio
 * 2. Replace "YOUR_PROJECT_ID" below with your actual project ID
 * 3. Import this component in your page instead of the regular Hero
 */
export default function HeroWith3D() {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden">
      {/* Mode Toggle - Top Right */}
      <div className="absolute top-4 right-4 z-50">
        <ModeToggle />
      </div>

      {/* 3D Animated Background */}
      <UnicornSceneWrapper
        projectId="YOUR_PROJECT_ID" // Replace with your Unicorn Studio project ID
        className="absolute inset-0 z-0"
        fps={60}
        lazyLoad={true}
        altText="Animated blockchain network visualization"
      />

      {/* Optional: Gradient overlay for better text contrast */}
      <div className="absolute inset-0 bg-gradient-to-b from-background/50 via-background/30 to-background/50 z-[1]" />

      {/* Content */}
      <div className="relative z-10 container mx-auto px-4 text-center">
        <div className="max-w-5xl mx-auto space-y-8">
          {/* Logo */}
          <h1 className="text-7xl md:text-9xl font-bold tracking-tight drop-shadow-2xl">
            ËTRID
          </h1>

          {/* Tagline */}
          <p className="text-2xl md:text-3xl font-semibold text-balance drop-shadow-lg">
            The Free and Open Decentralized Democracy of Stakeholders
          </p>

          {/* Subtitle */}
          <p className="text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto text-balance drop-shadow-md">
            Multichain blockchain with on-chain governance and adaptive consensus
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center pt-8">
            <Button
              size="lg"
              className="bg-primary text-primary-foreground hover:bg-primary/90 glow text-lg px-8 py-6"
            >
              Launch App
            </Button>
            <Link href="/governance">
              <Button
                size="lg"
                variant="outline"
                className="border-2 border-accent text-accent hover:bg-accent/10 text-lg px-8 py-6 bg-transparent backdrop-blur-sm"
              >
                Governance
              </Button>
            </Link>
            <Link href="/swap">
              <Button
                size="lg"
                variant="outline"
                className="border-2 border-success text-success hover:bg-success/10 text-lg px-8 py-6 bg-transparent backdrop-blur-sm"
              >
                Swap
              </Button>
            </Link>
            <Button
              size="lg"
              variant="outline"
              className="border-2 border-white text-white hover:bg-white/10 text-lg px-8 py-6 bg-transparent backdrop-blur-sm"
            >
              Read Whitepaper
            </Button>
          </div>
        </div>
      </div>

      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce z-10">
        <ChevronDown className="w-8 h-8 text-white/60" />
      </div>
    </section>
  )
}
