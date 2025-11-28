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
    <section className="py-20 px-4 max-w-7xl mx-auto">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4 text-white">
          <span className="bg-gradient-to-r from-purple-400 to-blue-400 bg-clip-text text-transparent">
            14 Blockchains
          </span>
          , One Network
        </h2>
        <p className="text-xl text-zinc-400">All major chains connected through Lightning</p>
      </div>

      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-7 gap-4">
        {chains.map((chain, index) => (
          <motion.div
            key={index}
            className="bg-zinc-900 rounded-xl p-4 border border-purple-500/20 hover:border-purple-500/50 transition-all duration-300 text-center hover:shadow-lg hover:shadow-purple-500/20"
            initial={{ opacity: 0, scale: 0.9 }}
            whileInView={{ opacity: 1, scale: 1 }}
            viewport={{ once: true }}
            transition={{ delay: index * 0.05 }}
          >
            <div className="font-bold text-lg mb-1 text-white">{chain.symbol}</div>
            <div className="text-sm text-zinc-400">{chain.name}</div>
            <div className="mt-2 text-xs text-green-400 font-medium">{chain.status}</div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
