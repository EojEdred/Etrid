'use client';

import React from 'react';
import Link from 'next/link';

export default function Footer() {
  return (
    <footer className="bg-slate-900/50 backdrop-blur-lg border-t border-white/10 mt-12">
      <div className="max-w-7xl mx-auto px-6 py-6">
        <div className="flex items-center justify-between text-sm text-gray-400">
          <p>© 2025 Etrid Watchtower Monitor. All rights reserved.</p>
          <div className="flex gap-6">
            <Link href="#" className="hover:text-white transition-colors">
              Documentation
            </Link>
            <Link href="#" className="hover:text-white transition-colors">
              Support
            </Link>
            <Link href="#" className="hover:text-white transition-colors">
              GitHub
            </Link>
          </div>
        </div>
      </div>
    </footer>
  );
}
