'use client';

import { motion } from 'framer-motion';
import { FaShoppingCart, FaGamepad, FaStream, FaGlobe } from 'react-icons/fa';

const useCases = [
  { icon: FaShoppingCart, title: 'E-Commerce', description: 'Accept Lightning payments for instant settlement with zero chargeback risk' },
  { icon: FaGamepad, title: 'Gaming', description: 'Enable microtransactions for in-game purchases and player-to-player trading' },
  { icon: FaStream, title: 'Streaming', description: 'Per-second payments for content creators and streaming services' },
  { icon: FaGlobe, title: 'Remittances', description: 'Send money across borders instantly with minimal fees' },
];

export default function UseCases() {
  return (
    <section className="section-container">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          Real-World <span className="gradient-text">Use Cases</span>
        </h2>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        {useCases.map((useCase, index) => (
          <motion.div key={index} className="card text-center" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }} transition={{ delay: index * 0.1 }}>
            <useCase.icon className="text-5xl mx-auto mb-4 text-purple-500" />
            <h3 className="text-xl font-bold mb-2">{useCase.title}</h3>
            <p className="text-gray-400">{useCase.description}</p>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
