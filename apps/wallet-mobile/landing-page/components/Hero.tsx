'use client';

import { ArrowRight, Download, Check } from 'lucide-react';
import Link from 'next/link';

export default function Hero() {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden bg-gradient-to-b from-[#1a0033] via-[#2d0055] to-[#4a0080]">
      {/* Animated background */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute -top-1/2 -left-1/2 w-full h-full bg-purple-500/10 rounded-full blur-3xl animate-pulse"></div>
        <div className="absolute -bottom-1/2 -right-1/2 w-full h-full bg-blue-500/10 rounded-full blur-3xl animate-pulse delay-1000"></div>
      </div>

      <div className="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
        <div className="grid lg:grid-cols-2 gap-12 items-center">
          {/* Left column - Text */}
          <div className="text-center lg:text-left">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-purple-500/20 border border-purple-500/30 mb-6">
              <span className="flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-2 w-2 rounded-full bg-purple-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-purple-500"></span>
              </span>
              <span className="text-sm text-purple-200">18 Features • $23M ARR Potential</span>
            </div>

            <h1 className="text-5xl md:text-7xl font-bold text-white mb-6 leading-tight">
              The Complete
              <span className="block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-blue-400">
                DeFi Wallet
              </span>
            </h1>

            <p className="text-xl text-gray-300 mb-8 max-w-2xl">
              Your crypto secured debit card, NFT marketplace, advanced trading,
              and 15 more features in one beautiful app. The only wallet you&apos;ll ever need.
            </p>

            {/* Value props */}
            <div className="grid sm:grid-cols-2 gap-4 mb-8">
              {[
                'Crypto Debit Card',
                'Trade with 0.1% fees',
                'Earn 15% APY',
                'NFT Marketplace',
              ].map((feature) => (
                <div key={feature} className="flex items-center gap-2 text-gray-200">
                  <Check className="w-5 h-5 text-green-400" />
                  <span>{feature}</span>
                </div>
              ))}
            </div>

            {/* CTA Buttons */}
            <div className="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
              <Link
                href="/download"
                className="group px-8 py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold flex items-center justify-center gap-2 hover:scale-105 transition-transform"
              >
                <Download className="w-5 h-5" />
                Get Started Free
                <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </Link>

              <Link
                href="#features"
                className="px-8 py-4 bg-white/10 backdrop-blur-sm text-white rounded-xl font-semibold border border-white/20 hover:bg-white/20 transition-colors"
              >
                Explore Features
              </Link>
            </div>

            {/* Social proof */}
            <div className="mt-12 flex items-center gap-8 justify-center lg:justify-start flex-wrap">
              <div>
                <div className="text-3xl font-bold text-white">100K+</div>
                <div className="text-sm text-gray-400">Early Users</div>
              </div>
              <div className="w-px h-12 bg-white/20"></div>
              <div>
                <div className="text-3xl font-bold text-white">4.8★</div>
                <div className="text-sm text-gray-400">App Rating</div>
              </div>
              <div className="w-px h-12 bg-white/20"></div>
              <div>
                <div className="text-3xl font-bold text-white">$1B+</div>
                <div className="text-sm text-gray-400">Volume</div>
              </div>
            </div>
          </div>

          {/* Right column - Phone mockup */}
          <div className="relative">
            <div className="relative mx-auto w-[300px] h-[600px]">
              {/* Phone frame */}
              <div className="absolute inset-0 bg-gradient-to-b from-gray-800 to-gray-900 rounded-[3rem] shadow-2xl border-8 border-gray-900">
                {/* Notch */}
                <div className="absolute top-0 left-1/2 -translate-x-1/2 w-40 h-6 bg-gray-900 rounded-b-3xl"></div>

                {/* Screenshot */}
                <div className="absolute inset-2 bg-gradient-to-b from-[#1a0033] to-[#4a0080] rounded-[2.5rem] overflow-hidden">
                  {/* Wallet UI preview */}
                  <div className="p-6 space-y-4">
                    <div className="flex justify-between items-center">
                      <h2 className="text-white font-bold">Ëtrid Wallet</h2>
                      <div className="flex gap-2">
                        <div className="w-6 h-6 rounded-full bg-white/10"></div>
                        <div className="w-6 h-6 rounded-full bg-white/10"></div>
                      </div>
                    </div>

                    <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-6">
                      <div className="text-sm text-gray-300 mb-2">Total Balance</div>
                      <div className="text-3xl font-bold text-white mb-1">$12,543.67</div>
                      <div className="text-sm text-green-400">+$234.56 (1.9%)</div>
                    </div>

                    <div className="grid grid-cols-4 gap-3">
                      {['Buy', 'Card', 'Save', 'Lend'].map((action) => (
                        <div key={action} className="flex flex-col items-center">
                          <div className="w-12 h-12 rounded-2xl bg-purple-500/20 mb-2"></div>
                          <div className="text-xs text-white">{action}</div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              </div>

              {/* Floating elements */}
              <div className="absolute -top-4 -right-4 bg-green-500 text-white px-3 py-1 rounded-full text-sm font-semibold animate-bounce">
                +15% APY
              </div>
              <div className="absolute -bottom-4 -left-4 bg-blue-500 text-white px-3 py-1 rounded-full text-sm font-semibold animate-bounce delay-500">
                0.1% Fees
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
