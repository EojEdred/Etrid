'use client'

import { useState } from 'react'
import Link from 'next/link'
import { FileText, BookOpen, HelpCircle, Shield, Settings, Home, BarChart3, Shield as ShieldIcon } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'

const docsSections = [
  {
    title: 'Getting Started',
    icon: Home,
    description: 'Begin your journey with ETRID',
    href: '/docs/guide',
    content: `# Getting Started with ETRID

## 1. Account Setup
- Create or import your wallet
- Store your recovery phrase securely
- Verify your wallet address

## 2. Get ETR Tokens
- Purchase ETR from exchanges
- Transfer to your ETRID wallet
- Verify token balance

## 3. Stake Tokens
- Minimum stake: 64 ETR
- Navigate to Staking section
- Select amount and confirm

## 4. Verify Transactions
- Check transaction status
- Monitor staking rewards
- Track validator performance`
  },
  {
    title: 'Staking Guide',
    icon: BarChart3,
    description: 'Earn rewards by staking ETR',
    href: '/docs/staking',
    content: `# Staking Guide

## Understanding Staking
- Stake 64+ ETR to earn rewards
- Rewards distributed per-block
- Unbonding period: 21 days

## How to Stake
1. Navigate to Staking section
2. Select amount to stake (min 64 ETR)
3. Confirm transaction
4. Monitor rewards

## Validator Selection
- Choose validators based on:
  - Commission rate
  - Uptime
  - Performance

## Rewards Calculation
- APY: 8-15% depending on network participation
- Distributed per-block
- Compounded automatically if selected`
  },
  {
    title: 'Validator Guide',
    icon: ShieldIcon,
    description: 'Run a validator node',
    href: '/docs/validator',
    content: `# Validator Guide

## Requirements
- Minimum 64 ETR stake (recommended 1000+)
- Dedicated server hardware
- 24/7 uptime capability
- Network connectivity

## Setup Process
1. Stake minimum ETR
2. Configure validator node
3. Register with network
4. Monitor performance

## Responsibilities
- Validate blocks and transactions
- Maintain 100% uptime
- Update software regularly
- Secure validator keys

## Rewards
- Block validation fees
- Commission on staking rewards
- Performance bonuses`
  },
  {
    title: 'FAQ',
    icon: HelpCircle,
    description: 'Frequently asked questions',
    href: '/docs/faq',
    content: `# Frequently Asked Questions

## General

**Q: What is the minimum stake amount?**
A: The minimum stake amount is 64 ETR.

**Q: How often are rewards distributed?**
A: Rewards are distributed per-block (not daily). 

**Q: What is the unbonding period?**
A: The unbonding period is 21 days.

**Q: How many validators are there?**
A: The number of validators is variable based on network needs.

## Staking

**Q: Can I delegate to multiple validators?**
A: Yes, you can delegate to multiple validators to diversify risk.

**Q: What is slashing?**
A: Slashing is a penalty mechanism where part of a validator's stake is destroyed for malicious behavior or poor performance.

## Security

**Q: Is it safe to store tokens in a software wallet?**
A: Software wallets are convenient but more vulnerable than hardware wallets. For large amounts, consider using a hardware wallet.

**Q: What happens if I lose my mnemonic phrase?**
A: If you lose your mnemonic phrase, you will permanently lose access to your wallet and any funds in it. Always keep your mnemonic phrase secure and private.`
  }
]

export default function DocsPage() {
  const [activeSection, setActiveSection] = useState(0)

  return (
    <div className="min-h-screen gradient-bg-animated">
      {/* Header */}
      <header className="sticky top-0 z-50 glass border-b border-white/10">
        <div className="container mx-auto px-4 py-3">
          <div className="flex items-center justify-between">
            <Link href="/" className="flex items-center gap-2">
              <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-cyan-500 to-purple-600 flex items-center justify-center">
                <FileText className="w-5 h-5 text-white" />
              </div>
              <span className="font-bold text-lg hidden sm:inline gradient-text">ETRID DOCS</span>
            </Link>

            <div className="flex items-center gap-3">
              <Link href="/">
                <Button variant="outline" className="border-white/20 hover:bg-white/10">
                  <Home className="w-4 h-4 mr-2" />
                  Console
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        <div className="max-w-6xl mx-auto">
          <div className="text-center mb-12">
            <h1 className="text-4xl md:text-5xl font-bold gradient-text bg-gradient-to-r from-cyan-400 via-purple-500 to-pink-500 bg-clip-text text-transparent mb-4">
              Documentation
            </h1>
            <p className="text-white/70 text-lg max-w-2xl mx-auto">
              Learn how to use the ETRID network, stake tokens, run validators, and more.
            </p>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
            {/* Navigation */}
            <div className="lg:col-span-1">
              <Card className="glass-card border-0">
                <CardHeader>
                  <CardTitle>Documentation Sections</CardTitle>
                </CardHeader>
                <CardContent className="p-0">
                  <div className="space-y-1">
                    {docsSections.map((section, index) => {
                      const Icon = section.icon
                      return (
                        <button
                          key={index}
                          onClick={() => setActiveSection(index)}
                          className={`w-full text-left p-4 rounded-lg transition-colors flex items-center gap-3 ${
                            activeSection === index 
                              ? 'bg-white/10 text-white' 
                              : 'hover:bg-white/5 text-white/70'
                          }`}
                        >
                          <Icon className="w-5 h-5" />
                          <div className="text-left">
                            <div className="font-medium">{section.title}</div>
                            <div className="text-xs text-white/60">{section.description}</div>
                          </div>
                        </button>
                      )
                    })}
                  </div>
                </CardContent>
              </Card>
            </div>

            {/* Content */}
            <div className="lg:col-span-3">
              <Card className="glass-card border-0">
                <CardHeader>
                  <CardTitle className="flex items-center gap-3">
                    {(() => {
                      const Icon = docsSections[activeSection].icon;
                      return <Icon className="w-6 h-6 text-cyan-400" />;
                    })()}
                    {docsSections[activeSection].title}
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="prose prose-invert max-w-none">
                    {docsSections[activeSection].content.split('\n').map((line, i) => {
                      if (line.startsWith('# ')) {
                        return <h1 key={i} className="text-2xl font-bold mb-4 text-white">{line.substring(2)}</h1>
                      } else if (line.startsWith('## ')) {
                        return <h2 key={i} className="text-xl font-semibold mb-3 text-cyan-300">{line.substring(3)}</h2>
                      } else if (line.startsWith('**Q:')) {
                        return <p key={i} className="font-semibold text-white mb-1">{line.substring(3)}</p>
                      } else if (line.startsWith('**A:')) {
                        return <p key={i} className="text-white/80 mb-4 ml-2">{line.substring(3)}</p>
                      } else if (line.startsWith('- ')) {
                        return <li key={i} className="text-white/80 mb-1">{line.substring(2)}</li>
                      } else if (line.trim() === '') {
                        return <br key={i} />
                      } else {
                        return <p key={i} className="text-white/80 mb-2">{line}</p>
                      }
                    })}
                  </div>
                </CardContent>
              </Card>
            </div>
          </div>
        </div>
      </main>
    </div>
  )
}