'use client';

import { motion } from 'framer-motion';

const roadmap = [
  { quarter: 'Q1 2025', features: 'Multi-path payments, Submarine swaps' },
  { quarter: 'Q2 2025', features: 'Lightning DEX, Recurring payments' },
  { quarter: 'Q3 2025', features: 'Channel factories, Privacy features' },
  { quarter: 'Q4 2025', features: 'DAO governance, Gaming integration' },
];

export default function Roadmap() {
  return (
    <section className="section-container bg-gray-900/50">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          <span className="gradient-text">Roadmap</span>
        </h2>
        <p className="text-xl text-gray-400">What's coming next</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        {roadmap.map((item, index) => (
          <motion.div key={index} className="card" initial={{ opacity: 0, x: -20 }} whileInView={{ opacity: 1, x: 0 }} viewport={{ once: true }} transition={{ delay: index * 0.1 }}>
            <div className="text-2xl font-bold gradient-text mb-2">{item.quarter}</div>
            <p className="text-gray-400">{item.features}</p>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
