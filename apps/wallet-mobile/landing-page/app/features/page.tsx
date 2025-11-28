import { Metadata } from 'next';
import Features from '@/components/Features';
import {
  CreditCard, DollarSign, Users, Image, TrendingUp,
  Coins, Target, Shield, Eye, Briefcase, Store,
  Globe, Building2, BarChart3, Bell, Palette, Gamepad2, Check
} from 'lucide-react';

export const metadata: Metadata = {
  title: '18 Powerful Features | Ëtrid Mobile Wallet',
  description: 'Explore all 18 features of Ëtrid Mobile Wallet: crypto debit card, NFT marketplace, advanced trading, lending, and more.',
};

const featureDetails = [
  {
    icon: DollarSign,
    title: 'Fiat On/Off Ramp',
    description: 'Seamlessly convert between fiat and crypto',
    benefits: [
      'Credit card payments accepted',
      'Direct bank transfers',
      '100+ fiat currencies supported',
      'Instant processing',
      'Competitive exchange rates'
    ],
    color: 'from-green-400 to-green-600',
  },
  {
    icon: CreditCard,
    title: 'AU Bloccard',
    description: 'Crypto-secured Visa debit card',
    benefits: [
      '60% LTV spending power',
      'No liquidation of crypto',
      'Earn rewards on purchases',
      'Global acceptance',
      'Virtual & physical cards'
    ],
    color: 'from-purple-400 to-purple-600',
  },
  {
    icon: Users,
    title: 'Social Features',
    description: 'Connect with friends and manage money together',
    benefits: [
      '@username.etrid handles',
      'Contacts & address book',
      'Bill splitting',
      'Social recovery',
      'Send crypto via username'
    ],
    color: 'from-blue-400 to-blue-600',
  },
  {
    icon: Image,
    title: 'NFT Marketplace',
    description: 'Buy, sell, and mint NFTs',
    benefits: [
      'Browse collections',
      'Create & mint NFTs',
      'Auction support',
      'Royalty management',
      'Multi-chain support'
    ],
    color: 'from-pink-400 to-pink-600',
  },
  {
    icon: TrendingUp,
    title: 'Advanced Trading',
    description: 'Professional-grade trading tools',
    benefits: [
      'Technical indicators',
      'DCA bots',
      'Limit orders',
      'Chart analysis',
      '0.1% trading fees'
    ],
    color: 'from-orange-400 to-orange-600',
  },
  {
    icon: Coins,
    title: 'Lending & Borrowing',
    description: 'Earn yield and access liquidity',
    benefits: [
      'Supply to earn 8-15% APY',
      'Borrow with collateral',
      'Flexible terms',
      'Auto-compound',
      'Risk management'
    ],
    color: 'from-yellow-400 to-yellow-600',
  },
];

export default function FeaturesPage() {
  return (
    <main className="pt-16">
      {/* Hero Section */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] via-[#2d0055] to-[#4a0080]">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h1 className="text-5xl md:text-7xl font-bold text-white mb-6">
            18 Powerful Features
            <span className="block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-blue-400 mt-2">
              One Wallet
            </span>
          </h1>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto mb-8">
            Everything you need for crypto finance, trading, NFTs, and more - all in one beautiful, easy-to-use app.
          </p>
          <div className="flex flex-wrap justify-center gap-4 text-sm text-gray-300">
            <div className="flex items-center gap-2">
              <Check className="w-5 h-5 text-green-400" />
              <span>No monthly fees</span>
            </div>
            <div className="flex items-center gap-2">
              <Check className="w-5 h-5 text-green-400" />
              <span>Bank-grade security</span>
            </div>
            <div className="flex items-center gap-2">
              <Check className="w-5 h-5 text-green-400" />
              <span>24/7 support</span>
            </div>
          </div>
        </div>
      </section>

      {/* Features Grid */}
      <Features />

      {/* Detailed Features */}
      <section className="py-24 bg-[#1a0033]">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-white text-center mb-16">
            Feature Deep Dive
          </h2>
          <div className="space-y-12">
            {featureDetails.map((feature, index) => (
              <div
                key={feature.title}
                className={`grid md:grid-cols-2 gap-8 items-center ${
                  index % 2 === 1 ? 'md:flex-row-reverse' : ''
                }`}
              >
                <div className={index % 2 === 1 ? 'md:order-2' : ''}>
                  <div className={`inline-flex p-4 rounded-2xl bg-gradient-to-r ${feature.color} mb-4`}>
                    <feature.icon className="w-8 h-8 text-white" />
                  </div>
                  <h3 className="text-3xl font-bold text-white mb-3">
                    {feature.title}
                  </h3>
                  <p className="text-xl text-gray-300 mb-6">
                    {feature.description}
                  </p>
                  <ul className="space-y-3">
                    {feature.benefits.map((benefit) => (
                      <li key={benefit} className="flex items-start gap-3 text-gray-300">
                        <Check className="w-5 h-5 text-green-400 mt-1 flex-shrink-0" />
                        <span>{benefit}</span>
                      </li>
                    ))}
                  </ul>
                </div>
                <div className={index % 2 === 1 ? 'md:order-1' : ''}>
                  <div className="bg-white/5 backdrop-blur-sm rounded-3xl p-8 border border-white/10 h-80 flex items-center justify-center">
                    <feature.icon className="w-32 h-32 text-purple-400/30" />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] to-[#4a0080]">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
            Ready to Experience All Features?
          </h2>
          <p className="text-xl text-gray-300 mb-8">
            Join thousands of users already enjoying the complete DeFi experience.
          </p>
          <a
            href="/download"
            className="inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold text-lg hover:scale-105 transition-transform"
          >
            Get Started Free
          </a>
        </div>
      </section>
    </main>
  );
}
