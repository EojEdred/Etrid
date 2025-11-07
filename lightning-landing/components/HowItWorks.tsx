'use client';

import { motion } from 'framer-motion';

const steps = [
  { number: '1', title: 'Open Channel', description: 'Open a Lightning channel on any of the 14 supported PBCs with just one transaction' },
  { number: '2', title: 'Route Payment', description: 'Lightning Network finds the optimal path across chains automatically' },
  { number: '3', title: 'Instant Settlement', description: 'Payment settles in under 1 second with cryptographic proof' },
  { number: '4', title: 'Close & Settle', description: 'Close channel anytime and settle final balances on-chain' },
];

export default function HowItWorks() {
  return (
    <section className="section-container bg-gray-900/50">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          How It <span className="gradient-text">Works</span>
        </h2>
        <p className="text-xl text-gray-400">Simple, fast, and secure cross-chain payments in 4 steps</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        {steps.map((step, index) => (
          <motion.div
            key={index}
            className="text-center"
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ delay: index * 0.1 }}
          >
            <div className="w-16 h-16 mx-auto mb-4 lightning-gradient rounded-full flex items-center justify-center text-2xl font-bold glow">
              {step.number}
            </div>
            <h3 className="text-xl font-bold mb-2">{step.title}</h3>
            <p className="text-gray-400">{step.description}</p>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
