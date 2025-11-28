'use client';

import { motion } from 'framer-motion';
import { FaBolt, FaRocket } from 'react-icons/fa';

export default function Hero() {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden network-bg">
      {/* Animated background elements */}
      <div className="absolute inset-0 opacity-30">
        <div className="absolute top-20 left-20 w-64 h-64 bg-purple-500 rounded-full filter blur-3xl animate-pulse" />
        <div className="absolute bottom-20 right-20 w-96 h-96 bg-blue-500 rounded-full filter blur-3xl animate-pulse delay-1000" />
      </div>

      <div className="relative z-10 section-container text-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
        >
          {/* Lightning bolt icon */}
          <motion.div
            className="inline-flex items-center justify-center w-24 h-24 mb-8 lightning-gradient rounded-full glow"
            animate={{ rotate: [0, 5, -5, 0] }}
            transition={{ duration: 2, repeat: Infinity }}
          >
            <FaBolt className="text-5xl text-white" />
          </motion.div>

          {/* Main headline */}
          <h1 className="text-5xl md:text-7xl font-bold mb-6 leading-tight">
            Lightning-Fast Payments
            <br />
            <span className="gradient-text">Across 14 Blockchains</span>
          </h1>

          {/* Subheading */}
          <p className="text-xl md:text-2xl text-gray-300 mb-12 max-w-3xl mx-auto">
            Instant, low-cost cross-chain transfers powered by ÉTRID Lightning Network.
            Connect Bitcoin, Ethereum, Solana, and 11 more chains with one click.
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <motion.button
              className="btn-primary inline-flex items-center gap-2"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              <FaRocket />
              Try Demo
            </motion.button>
            <motion.button
              className="btn-secondary"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              Read Docs
            </motion.button>
          </div>

          {/* Quick stats */}
          <motion.div
            className="mt-16 grid grid-cols-2 md:grid-cols-4 gap-8 max-w-4xl mx-auto"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.5, duration: 0.8 }}
          >
            {[
              { value: '14', label: 'Blockchains' },
              { value: '<1s', label: 'Settlement Time' },
              { value: '0.1%', label: 'Avg. Fee' },
              { value: '99.9%', label: 'Uptime' },
            ].map((stat, index) => (
              <div key={index} className="text-center">
                <div className="text-3xl md:text-4xl font-bold gradient-text mb-2">
                  {stat.value}
                </div>
                <div className="text-sm text-gray-400">{stat.label}</div>
              </div>
            ))}
          </motion.div>
        </motion.div>
      </div>

      {/* Scroll indicator */}
      <motion.div
        className="absolute bottom-8 left-1/2 transform -translate-x-1/2"
        animate={{ y: [0, 10, 0] }}
        transition={{ duration: 1.5, repeat: Infinity }}
      >
        <div className="w-6 h-10 border-2 border-purple-500 rounded-full flex items-start justify-center p-2">
          <div className="w-1 h-2 bg-purple-500 rounded-full animate-pulse" />
        </div>
      </motion.div>
    </section>
  );
}
