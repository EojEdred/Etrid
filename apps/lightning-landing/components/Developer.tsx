'use client';

import { motion } from 'framer-motion';

export default function Developer() {
  const codeExample = `npm install @etrid/lightning-sdk

import { LightningClient } from '@etrid/lightning-sdk';

const client = new LightningClient('mainnet');

// Create invoice
const invoice = await client.createInvoice({
  amount: 1000000, // 0.001 ETH
  description: 'Payment for services',
  expiresIn: 3600
});

// Send payment
const result = await client.sendPayment(invoice);`;

  return (
    <section className="section-container">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          For <span className="gradient-text">Developers</span>
        </h2>
        <p className="text-xl text-gray-400">Easy integration with comprehensive SDKs</p>
      </div>

      <motion.div className="max-w-4xl mx-auto card" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }}>
        <pre className="bg-gray-900 p-6 rounded-lg overflow-x-auto">
          <code className="text-sm text-gray-300">{codeExample}</code>
        </pre>
        <div className="mt-6 flex gap-4">
          <button className="btn-primary">Documentation</button>
          <button className="btn-secondary">GitHub</button>
        </div>
      </motion.div>
    </section>
  );
}
