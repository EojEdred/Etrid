'use client';

import { motion } from 'framer-motion';
import { useState, useEffect } from 'react';

export default function Statistics() {
  const [stats, setStats] = useState({
    tvl: 0,
    channels: 0,
    transactions: 0,
    avgFee: 0,
  });

  useEffect(() => {
    // Simulate animated counters
    const targets = { tvl: 1234567890, channels: 45678, transactions: 9876543, avgFee: 0.08 };
    const duration = 2000;
    const steps = 60;
    const interval = duration / steps;

    let step = 0;
    const timer = setInterval(() => {
      step++;
      const progress = step / steps;

      setStats({
        tvl: Math.floor(targets.tvl * progress),
        channels: Math.floor(targets.channels * progress),
        transactions: Math.floor(targets.transactions * progress),
        avgFee: parseFloat((targets.avgFee * progress).toFixed(2)),
      });

      if (step >= steps) clearInterval(timer);
    }, interval);

    return () => clearInterval(timer);
  }, []);

  const formatNumber = (num: number) => num.toLocaleString();

  return (
    <section className="section-container bg-gray-900/50">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          Network <span className="gradient-text">Statistics</span>
        </h2>
        <p className="text-xl text-gray-400">Real-time metrics from the ÉTRID Lightning Network</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        <motion.div className="card text-center" whileInView={{ opacity: 1, y: 0 }} initial={{ opacity: 0, y: 20 }} viewport={{ once: true }}>
          <div className="text-4xl font-bold gradient-text mb-2">${formatNumber(stats.tvl / 1000000)}M</div>
          <div className="text-gray-400">Total Value Locked</div>
        </motion.div>
        <motion.div className="card text-center" whileInView={{ opacity: 1, y: 0 }} initial={{ opacity: 0, y: 20 }} viewport={{ once: true }} transition={{ delay: 0.1 }}>
          <div className="text-4xl font-bold gradient-text mb-2">{formatNumber(stats.channels)}</div>
          <div className="text-gray-400">Active Channels</div>
        </motion.div>
        <motion.div className="card text-center" whileInView={{ opacity: 1, y: 0 }} initial={{ opacity: 0, y: 20 }} viewport={{ once: true }} transition={{ delay: 0.2 }}>
          <div className="text-4xl font-bold gradient-text mb-2">{formatNumber(stats.transactions)}M</div>
          <div className="text-gray-400">Total Transactions</div>
        </motion.div>
        <motion.div className="card text-center" whileInView={{ opacity: 1, y: 0 }} initial={{ opacity: 0, y: 20 }} viewport={{ once: true }} transition={{ delay: 0.3 }}>
          <div className="text-4xl font-bold gradient-text mb-2">{stats.avgFee}%</div>
          <div className="text-gray-400">Average Fee</div>
        </motion.div>
      </div>
    </section>
  );
}
