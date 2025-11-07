'use client';

import { motion } from 'framer-motion';

const chains = [
  { name: 'Bitcoin', symbol: 'BTC', status: 'Live' },
  { name: 'Ethereum', symbol: 'ETH', status: 'Live' },
  { name: 'Solana', symbol: 'SOL', status: 'Live' },
  { name: 'Cardano', symbol: 'ADA', status: 'Live' },
  { name: 'Polkadot', symbol: 'DOT', status: 'Live' },
  { name: 'Avalanche', symbol: 'AVAX', status: 'Live' },
  { name: 'Polygon', symbol: 'MATIC', status: 'Live' },
  { name: 'Algorand', symbol: 'ALGO', status: 'Live' },
  { name: 'Cosmos', symbol: 'ATOM', status: 'Live' },
  { name: 'Tezos', symbol: 'XTZ', status: 'Live' },
  { name: 'Flare', symbol: 'FLR', status: 'Live' },
  { name: 'Hedera', symbol: 'HBAR', status: 'Live' },
  { name: 'NEAR', symbol: 'NEAR', status: 'Live' },
  { name: 'Aptos', symbol: 'APT', status: 'Live' },
];

export default function SupportedChains() {
  return (
    <section className="section-container">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          <span className="gradient-text">14 Blockchains</span>, One Network
        </h2>
        <p className="text-xl text-gray-400">All major chains connected through Lightning</p>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-7 gap-4">
        {chains.map((chain, index) => (
          <motion.div
            key={index}
            className="card text-center"
            initial={{ opacity: 0, scale: 0.9 }}
            whileInView={{ opacity: 1, scale: 1 }}
            viewport={{ once: true }}
            transition={{ delay: index * 0.05 }}
          >
            <div className="font-bold text-lg mb-1">{chain.symbol}</div>
            <div className="text-sm text-gray-400">{chain.name}</div>
            <div className="mt-2 text-xs text-green-400">{chain.status}</div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
