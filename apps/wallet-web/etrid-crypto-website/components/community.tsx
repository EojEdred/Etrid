"use client"

import type React from "react"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { MessageCircle, Twitter, Send } from "lucide-react"
import { useState } from "react"

const socialLinks = [
  {
    name: "Discord",
    icon: MessageCircle,
    members: "12.5K",
    color: "bg-[#5865F2]",
  },
  {
    name: "Twitter",
    icon: Twitter,
    members: "25K",
    color: "bg-[#1DA1F2]",
  },
  {
    name: "Telegram",
    icon: Send,
    members: "18K",
    color: "bg-[#0088cc]",
  },
]

export default function Community() {
  const [email, setEmail] = useState("")

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    console.log("Newsletter signup:", email)
    setEmail("")
  }

  return (
    <section className="py-24 px-4 bg-background">
      <div className="container mx-auto max-w-5xl">
        <h2 className="text-4xl md:text-5xl font-bold text-center mb-16">Join the Community</h2>

        {/* Social Links */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-16">
          {socialLinks.map((social) => {
            const Icon = social.icon
            return (
              <button
                key={social.name}
                className={`${social.color} rounded-2xl p-8 hover:scale-105 transition-all duration-300 text-white`}
              >
                <Icon className="w-12 h-12 mb-4 mx-auto" />
                <h3 className="text-2xl font-semibold mb-2">{social.name}</h3>
                <p className="text-white/80">{social.members} members</p>
              </button>
            )
          })}
        </div>

        {/* Newsletter */}
        <div className="glass rounded-2xl p-8 md:p-12 text-center">
          <h3 className="text-2xl md:text-3xl font-semibold mb-4">Stay Updated</h3>
          <p className="text-muted-foreground mb-8 max-w-2xl mx-auto">
            Get updates on launch and governance events. Be the first to know about Consensus Day and network upgrades.
          </p>
          <form onSubmit={handleSubmit} className="flex flex-col sm:flex-row gap-4 max-w-md mx-auto">
            <Input
              type="email"
              placeholder="Enter your email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="flex-1 bg-background/50 border-border"
              required
            />
            <Button type="submit" className="bg-primary text-primary-foreground hover:bg-primary/90">
              Subscribe
            </Button>
          </form>
        </div>
      </div>
    </section>
  )
}
