'use client';

import { useState } from 'react';
import Link from 'next/link';
import { Menu, X } from 'lucide-react';

export default function Navigation() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <nav className="fixed top-0 left-0 right-0 z-50 bg-[#1a0033]/80 backdrop-blur-lg border-b border-white/10">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Logo */}
          <Link href="/" className="flex items-center gap-2">
            <div className="w-8 h-8 rounded-lg bg-gradient-to-r from-purple-500 to-blue-500"></div>
            <span className="text-xl font-bold text-white">Ëtrid Wallet</span>
          </Link>

          {/* Desktop menu */}
          <div className="hidden md:flex items-center gap-8">
            <Link href="#features" className="text-gray-300 hover:text-white transition-colors">
              Features
            </Link>
            <Link href="/pricing" className="text-gray-300 hover:text-white transition-colors">
              Pricing
            </Link>
            <Link href="/docs" className="text-gray-300 hover:text-white transition-colors">
              Docs
            </Link>
            <Link
              href="/download"
              className="px-6 py-2 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-lg font-semibold hover:scale-105 transition-transform"
            >
              Get Started
            </Link>
          </div>

          {/* Mobile menu button */}
          <button
            className="md:hidden text-white"
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          >
            {mobileMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
          </button>
        </div>
      </div>

      {/* Mobile menu */}
      {mobileMenuOpen && (
        <div className="md:hidden bg-[#1a0033] border-t border-white/10">
          <div className="px-4 py-6 space-y-4">
            <Link href="#features" className="block text-gray-300 hover:text-white">
              Features
            </Link>
            <Link href="/pricing" className="block text-gray-300 hover:text-white">
              Pricing
            </Link>
            <Link href="/docs" className="block text-gray-300 hover:text-white">
              Docs
            </Link>
            <Link
              href="/download"
              className="block w-full text-center px-6 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-lg font-semibold"
            >
              Get Started
            </Link>
          </div>
        </div>
      )}
    </nav>
  );
}
