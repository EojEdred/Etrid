'use client';

import React from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { Apple, Download, Wallet, Globe, Vote, ArrowLeftRight, Zap, Shield, TrendingUp, ArrowRight, BarChart3 } from 'lucide-react';

export default function HeroSection() {
  return (
    <div className="min-h-screen bg-[#0a0014] text-white">
      {/* Navigation Links */}
      <nav className="relative z-10 px-6 py-4">
        <div className="mx-auto max-w-7xl flex items-center justify-between">
          <Link href="/" className="flex items-center gap-2">
            <Image src="/etrid-logo.png" alt="ETRID" width={40} height={40} className="rounded-xl" />
            <span className="text-xl font-bold bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] bg-clip-text text-transparent">ETRID</span>
          </Link>
          <div className="flex items-center gap-2 flex-wrap justify-end">
            <Link href="/" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
              <Wallet className="w-4 h-4" />
              <span className="hidden sm:inline">Wallet</span>
            </Link>
            <Link href="/staking" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
              <BarChart3 className="w-4 h-4" />
              <span className="hidden sm:inline">Staking</span>
            </Link>
            <Link href="/governance" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
              <Vote className="w-4 h-4" />
              <span className="hidden sm:inline">Governance</span>
            </Link>
            <Link href="/swap" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
              <ArrowLeftRight className="w-4 h-4" />
              <span className="hidden sm:inline">Swap</span>
            </Link>
            <Link href="/lightning" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
              <Zap className="w-4 h-4" />
              <span className="hidden sm:inline">Lightning</span>
            </Link>
            <Link href="/validator" className="flex items-center gap-1.5 px-3 py-2 rounded-lg text-white/70 hover:text-white hover:bg-white/10 transition-all text-sm">
              <Shield className="w-4 h-4" />
              <span className="hidden sm:inline">Validator</span>
            </Link>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="relative overflow-hidden px-6 py-16 md:py-24">
        {/* Background gradient effects */}
        <div className="absolute inset-0 overflow-hidden">
          <div className="absolute -top-40 -right-40 h-96 w-96 rounded-full bg-gradient-to-br from-[#66D9E6]/20 to-[#4DB3CC]/10 blur-3xl" />
          <div className="absolute -bottom-40 -left-40 h-96 w-96 rounded-full bg-gradient-to-br from-[#4DB3CC]/20 to-[#66D9E6]/10 blur-3xl" />
        </div>

        <div className="relative mx-auto max-w-7xl">
          {/* Logo and Main Content */}
          <div className="flex flex-col items-center text-center">
            {/* ËTRID Logo */}
            <div className="mb-8 animate-fade-in">
              <div className="relative">
                <div className="absolute inset-0 bg-gradient-to-br from-[#66D9E6] to-[#4DB3CC] blur-2xl opacity-50" />
                <Image
                  src="/etrid-logo.png"
                  alt="ETRID"
                  width={160}
                  height={160}
                  className="relative rounded-3xl shadow-2xl"
                  priority
                />
              </div>
            </div>

            {/* Main Heading */}
            <h1 className="mb-4 text-5xl font-bold leading-tight md:text-7xl lg:text-8xl">
              <span className="bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] bg-clip-text text-transparent">
                ËTRID
              </span>
            </h1>

            {/* Tagline */}
            <p className="mb-6 text-2xl font-semibold text-[#66D9E6] md:text-3xl">
              The Future of Decentralized Finance
            </p>

            {/* Subtitle */}
            <p className="mb-12 max-w-3xl text-lg leading-relaxed text-gray-300 md:text-xl">
              Experience the next generation of crypto with ËTRID — your all-in-one multi-chain wallet
              for seamless DeFi, governance participation, staking rewards, and cross-chain transactions.
            </p>

            {/* CTA Buttons */}
            <div className="flex flex-col gap-4 sm:flex-row">
              {/* iOS Button */}
              <a
                href="https://testflight.apple.com/join/etrid-wallet"
                className="group relative overflow-hidden rounded-2xl bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] p-[2px] transition-all hover:scale-105 hover:shadow-2xl hover:shadow-[#66D9E6]/50"
              >
                <div className="flex items-center gap-3 rounded-2xl bg-[#0a0014] px-8 py-4 transition-all group-hover:bg-opacity-80">
                  <Apple className="h-6 w-6" />
                  <div className="text-left">
                    <div className="text-xs text-gray-400">Download for</div>
                    <div className="text-lg font-semibold">iOS</div>
                  </div>
                </div>
              </a>

              {/* Android Button */}
              <a
                href="https://github.com/etrid/wallet/releases"
                className="group relative overflow-hidden rounded-2xl bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] p-[2px] transition-all hover:scale-105 hover:shadow-2xl hover:shadow-[#66D9E6]/50"
              >
                <div className="flex items-center gap-3 rounded-2xl bg-[#0a0014] px-8 py-4 transition-all group-hover:bg-opacity-80">
                  <Download className="h-6 w-6" />
                  <div className="text-left">
                    <div className="text-xs text-gray-400">Download for</div>
                    <div className="text-lg font-semibold">Android</div>
                  </div>
                </div>
              </a>
            </div>

            {/* Trust Indicators */}
            <div className="mt-12 flex flex-wrap items-center justify-center gap-6 text-sm text-gray-400">
              <div className="flex items-center gap-2">
                <Shield className="h-5 w-5 text-[#66D9E6]" />
                <span>Military-Grade Security</span>
              </div>
              <div className="flex items-center gap-2">
                <Zap className="h-5 w-5 text-[#66D9E6]" />
                <span>Lightning Fast</span>
              </div>
              <div className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5 text-[#66D9E6]" />
                <span>10K+ Active Users</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="relative px-6 py-20">
        <div className="mx-auto max-w-7xl">
          {/* Section Header */}
          <div className="mb-16 text-center">
            <h2 className="mb-4 text-4xl font-bold md:text-5xl">
              Built for the{' '}
              <span className="bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] bg-clip-text text-transparent">
                Future
              </span>
            </h2>
            <p className="text-lg text-gray-400">
              Everything you need for decentralized finance in one powerful wallet
            </p>
          </div>

          {/* Feature Cards Grid */}
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            {/* Multi-Chain Support Card */}
            <div className="group relative overflow-hidden rounded-3xl bg-gradient-to-br from-white/5 to-white/[0.02] p-[1px] backdrop-blur-xl transition-all hover:scale-105">
              <div className="relative h-full rounded-3xl bg-[#0a0014]/90 p-8 backdrop-blur-xl">
                <div className="mb-6 inline-flex rounded-2xl bg-gradient-to-br from-[#66D9E6]/20 to-[#4DB3CC]/10 p-4">
                  <Globe className="h-8 w-8 text-[#66D9E6]" />
                </div>
                <h3 className="mb-3 text-2xl font-bold">Multi-Chain Support</h3>
                <p className="mb-4 leading-relaxed text-gray-400">
                  Seamlessly manage Bitcoin, Ethereum, Solana, and more — all in one wallet.
                </p>
                <div className="flex flex-wrap gap-2">
                  <span className="rounded-full bg-[#66D9E6]/10 px-3 py-1 text-xs font-medium text-[#66D9E6]">
                    BTC
                  </span>
                  <span className="rounded-full bg-[#66D9E6]/10 px-3 py-1 text-xs font-medium text-[#66D9E6]">
                    ETH
                  </span>
                  <span className="rounded-full bg-[#66D9E6]/10 px-3 py-1 text-xs font-medium text-[#66D9E6]">
                    SOL
                  </span>
                  <span className="rounded-full bg-[#66D9E6]/10 px-3 py-1 text-xs font-medium text-[#66D9E6]">
                    +20
                  </span>
                </div>
              </div>
            </div>

            {/* DeFi Integration Card */}
            <div className="group relative overflow-hidden rounded-3xl bg-gradient-to-br from-white/5 to-white/[0.02] p-[1px] backdrop-blur-xl transition-all hover:scale-105">
              <div className="relative h-full rounded-3xl bg-[#0a0014]/90 p-8 backdrop-blur-xl">
                <div className="mb-6 inline-flex rounded-2xl bg-gradient-to-br from-[#66D9E6]/20 to-[#4DB3CC]/10 p-4">
                  <Wallet className="h-8 w-8 text-[#66D9E6]" />
                </div>
                <h3 className="mb-3 text-2xl font-bold">DeFi Integration</h3>
                <p className="leading-relaxed text-gray-400">
                  Access lending, borrowing, yield farming, and liquidity pools directly from your wallet.
                </p>
                <div className="mt-4 flex items-center gap-2 text-[#66D9E6]">
                  <span className="text-sm font-medium">Explore DeFi</span>
                  <ArrowRight className="h-4 w-4 transition-transform group-hover:translate-x-1" />
                </div>
              </div>
            </div>

            {/* Governance & Consensus Card */}
            <div className="group relative overflow-hidden rounded-3xl bg-gradient-to-br from-white/5 to-white/[0.02] p-[1px] backdrop-blur-xl transition-all hover:scale-105">
              <div className="relative h-full rounded-3xl bg-[#0a0014]/90 p-8 backdrop-blur-xl">
                <div className="mb-6 inline-flex rounded-2xl bg-gradient-to-br from-[#66D9E6]/20 to-[#4DB3CC]/10 p-4">
                  <Vote className="h-8 w-8 text-[#66D9E6]" />
                </div>
                <h3 className="mb-3 text-2xl font-bold">Governance & Consensus</h3>
                <p className="leading-relaxed text-gray-400">
                  Participate in network governance and earn rewards through our unique consensus mechanism.
                </p>
                <div className="mt-4 flex items-center gap-2 text-[#66D9E6]">
                  <span className="text-sm font-medium">Start Voting</span>
                  <ArrowRight className="h-4 w-4 transition-transform group-hover:translate-x-1" />
                </div>
              </div>
            </div>

            {/* Cross-Chain Bridge Card */}
            <div className="group relative overflow-hidden rounded-3xl bg-gradient-to-br from-white/5 to-white/[0.02] p-[1px] backdrop-blur-xl transition-all hover:scale-105">
              <div className="relative h-full rounded-3xl bg-[#0a0014]/90 p-8 backdrop-blur-xl">
                <div className="mb-6 inline-flex rounded-2xl bg-gradient-to-br from-[#66D9E6]/20 to-[#4DB3CC]/10 p-4">
                  <ArrowLeftRight className="h-8 w-8 text-[#66D9E6]" />
                </div>
                <h3 className="mb-3 text-2xl font-bold">Cross-Chain Bridge</h3>
                <p className="leading-relaxed text-gray-400">
                  Transfer assets seamlessly between different blockchains with our secure bridge technology.
                </p>
                <div className="mt-4 flex items-center gap-2 text-[#66D9E6]">
                  <span className="text-sm font-medium">Bridge Assets</span>
                  <ArrowRight className="h-4 w-4 transition-transform group-hover:translate-x-1" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Final CTA Section */}
      <section className="relative px-6 py-20">
        <div className="mx-auto max-w-4xl">
          <div className="relative overflow-hidden rounded-3xl bg-gradient-to-br from-[#66D9E6]/20 to-[#4DB3CC]/10 p-[2px]">
            <div className="rounded-3xl bg-[#0a0014]/80 p-12 text-center backdrop-blur-xl">
              <h2 className="mb-4 text-3xl font-bold md:text-4xl">
                Ready to Start Your DeFi Journey?
              </h2>
              <p className="mb-8 text-lg text-gray-300">
                Join thousands of users already experiencing the future of finance
              </p>
              <div className="flex flex-col items-center justify-center gap-4 sm:flex-row">
                <a
                  href="https://testflight.apple.com/join/etrid-wallet"
                  className="flex items-center gap-2 rounded-xl bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] px-8 py-4 font-semibold transition-all hover:scale-105 hover:shadow-2xl hover:shadow-[#66D9E6]/50"
                >
                  <Apple className="h-5 w-5" />
                  Download for iOS
                </a>
                <a
                  href="https://github.com/etrid/wallet/releases"
                  className="flex items-center gap-2 rounded-xl bg-gradient-to-r from-[#66D9E6] to-[#4DB3CC] px-8 py-4 font-semibold transition-all hover:scale-105 hover:shadow-2xl hover:shadow-[#66D9E6]/50"
                >
                  <Download className="h-5 w-5" />
                  Download for Android
                </a>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
