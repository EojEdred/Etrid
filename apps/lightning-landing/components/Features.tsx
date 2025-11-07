'use client';

import { motion } from 'framer-motion';
import { FaBolt, FaLink, FaDollarSign, FaShieldAlt, FaChartLine, FaCode } from 'react-icons/fa';

const features = [
  {
    icon: FaBolt,
    title: 'Instant Payments',
    description: 'Settle transactions in under 1 second across all 14 supported blockchains with Lightning Network speed.',
  },
  {
    icon: FaLink,
    title: 'Cross-Chain',
    description: 'Seamlessly route payments between Bitcoin, Ethereum, Solana, and 11 other chains without bridges.',
  },
  {
    icon: FaDollarSign,
    title: 'Low Fees',
    description: 'Pay minimal fees (< 0.1%) compared to on-chain transactions. Save up to 99% on transaction costs.',
  },
  {
    icon: FaShieldAlt,
    title: 'Secure',
    description: 'Enterprise-grade security with watchtowers, fraud proofs, and multi-sig protection mechanisms.',
  },
  {
    icon: FaChartLine,
    title: 'Scalable',
    description: 'Handle millions of transactions per second with Layer 2 architecture and optimistic rollups.',
  },
  {
    icon: FaCode,
    title: 'BOLT-11 Compatible',
    description: 'Fully compatible with BOLT-11 invoice standard. Works with existing Lightning infrastructure.',
  },
];

export default function Features() {
  return (
    <section className="section-container">
      <div className="text-center mb-16">
        <motion.h2
          className="text-4xl md:text-5xl font-bold mb-4"
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
        >
          Why Choose <span className="gradient-text">ÉTRID Lightning</span>
        </motion.h2>
        <motion.p
          className="text-xl text-gray-400 max-w-2xl mx-auto"
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
          viewport={{ once: true }}
          transition={{ delay: 0.2 }}
        >
          The most advanced Lightning Network implementation spanning multiple blockchains
        </motion.p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        {features.map((feature, index) => (
          <motion.div
            key={index}
            className="card"
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ delay: index * 0.1 }}
          >
            <div className="flex items-start gap-4">
              <div className="flex-shrink-0 w-12 h-12 lightning-gradient rounded-lg flex items-center justify-center">
                <feature.icon className="text-2xl text-white" />
              </div>
              <div>
                <h3 className="text-xl font-bold mb-2">{feature.title}</h3>
                <p className="text-gray-400">{feature.description}</p>
              </div>
            </div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
