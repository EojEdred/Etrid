'use client';

import { motion } from 'framer-motion';
import { useState } from 'react';
import QRCode from 'qrcode.react';

export default function Demo() {
  const [invoice, setInvoice] = useState('lnetrid1eth:1000000000000000000:0x123...:1735689600:abcdef');

  return (
    <section className="section-container bg-gray-900/50">
      <div className="text-center mb-16">
        <h2 className="text-4xl md:text-5xl font-bold mb-4">
          Try the <span className="gradient-text">Demo</span>
        </h2>
        <p className="text-xl text-gray-400">Create and scan Lightning invoices in seconds</p>
      </div>

      <div className="max-w-2xl mx-auto card">
        <div className="grid md:grid-cols-2 gap-8">
          <div>
            <h3 className="text-xl font-bold mb-4">Create Invoice</h3>
            <input type="number" placeholder="Amount" className="w-full p-3 bg-gray-700 rounded mb-3" />
            <input type="text" placeholder="Description" className="w-full p-3 bg-gray-700 rounded mb-3" />
            <button className="btn-primary w-full">Generate QR Code</button>
          </div>
          <div className="flex items-center justify-center">
            <div className="bg-white p-4 rounded-lg">
              <QRCode value={invoice} size={200} />
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
