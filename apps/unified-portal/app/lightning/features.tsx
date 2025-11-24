'use client';

import { motion } from 'framer-motion';
import { Zap, Link2, DollarSign, Shield, TrendingUp, Code } from 'lucide-react';

const features = [
  {
    icon: Zap,
    title: 'Instant Payments',
    description: 'Settle transactions in under 1 second across all 14 supported blockchains with Lightning Network speed.',
  },
  {
    icon: Link2,
    title: 'Cross-Chain',
    description: 'Seamlessly route payments between Bitcoin, Ethereum, Solana, and 11 other chains without bridges.',
  },
  {
    icon: DollarSign,
    title: 'Low Fees',
    description: 'Pay minimal fees (< 0.1%) compared to on-chain transactions. Save up to 99% on transaction costs.',
  },
  {
    icon: Shield,
    title: 'Secure',
    description: 'Enterprise-grade security with watchtowers, fraud proofs, and multi-sig protection mechanisms.',
  },
  {
    icon: TrendingUp,
    title: 'Scalable',
    description: 'Handle millions of transactions per second with Layer 2 architecture and optimistic rollups.',
  },
  {
    icon: Code,
    title: 'BOLT-11 Compatible',
    description: 'Fully compatible with BOLT-11 invoice standard. Works with existing Lightning infrastructure.',
  },
];

export default function Features() {
  return (
    <section className="py-20 px-4 max-w-7xl mx-auto">
      <div className="text-center mb-16">
        <motion.h2
          className="text-4xl md:text-5xl font-bold mb-4 text-white"
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
        >
          Why Choose{' '}
          <span className="bg-gradient-to-r from-purple-400 to-blue-400 bg-clip-text text-transparent">
            ÉTRID Lightning
          </span>
        </motion.h2>
        <motion.p
          className="text-xl text-zinc-400 max-w-2xl mx-auto"
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          viewport={{ once: true }}
          transition={{ delay: 0.2 }}
        >
          The most advanced Lightning Network implementation spanning multiple blockchains
        </motion.p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        {features.map((feature, index) => {
          const Icon = feature.icon;
          return (
            <motion.div
              key={index}
              className="bg-zinc-900 rounded-xl p-6 border border-purple-500/20 hover:border-purple-500/50 transition-all duration-300 hover:shadow-lg hover:shadow-purple-500/20"
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ delay: index * 0.1 }}
            >
              <div className="flex items-start gap-4">
                <div className="flex-shrink-0 w-12 h-12 bg-gradient-to-r from-purple-500 to-blue-500 rounded-lg flex items-center justify-center">
                  <Icon className="w-6 h-6 text-white" />
                </div>
                <div>
                  <h3 className="text-xl font-bold mb-2 text-white">{feature.title}</h3>
                  <p className="text-zinc-400">{feature.description}</p>
                </div>
              </div>
            </motion.div>
          );
        })}
      </div>
    </section>
  );
}
