import { Metadata } from 'next';
import { Apple, Chrome, Smartphone, Download, Check, Shield, Zap, Globe } from 'lucide-react';

export const metadata: Metadata = {
  title: 'Download | Ëtrid Mobile Wallet',
  description: 'Download Ëtrid Mobile Wallet for Web, iOS, and Android. Available on all platforms.',
};

export default function DownloadPage() {
  return (
    <main className="pt-16">
      {/* Hero Section */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] via-[#2d0055] to-[#4a0080]">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <Download className="w-16 h-16 text-purple-400 mx-auto mb-6" />
          <h1 className="text-5xl md:text-7xl font-bold text-white mb-6">
            Download
            <span className="block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-blue-400 mt-2">
              Ëtrid Wallet
            </span>
          </h1>
          <p className="text-xl text-gray-300 max-w-3xl mx-auto">
            Available on Web, iOS, and Android. Start managing your crypto in minutes.
          </p>
        </div>
      </section>

      {/* Download Options */}
      <section className="py-24 bg-[#1a0033]">
        <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid md:grid-cols-3 gap-8">
            {/* Web App */}
            <div className="bg-white/5 backdrop-blur-sm rounded-3xl p-8 border border-white/10 text-center">
              <div className="inline-flex p-6 rounded-3xl bg-gradient-to-r from-blue-500 to-cyan-500 mb-6">
                <Chrome className="w-12 h-12 text-white" />
              </div>
              <div className="inline-block px-3 py-1 rounded-full bg-green-500/20 text-green-300 text-sm font-semibold mb-4">
                Available Now
              </div>
              <h2 className="text-3xl font-bold text-white mb-4">Web App</h2>
              <p className="text-gray-300 mb-6">
                Use Ëtrid Wallet in any modern browser. No installation required.
              </p>
              <ul className="space-y-3 mb-8 text-left">
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Instant access</span>
                </li>
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>PWA support</span>
                </li>
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Works offline</span>
                </li>
              </ul>
              <a
                href="https://wallet.etrid.com"
                className="block w-full py-4 px-6 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold hover:scale-105 transition-transform"
              >
                Launch Web App
              </a>
              <p className="text-sm text-gray-400 mt-4">
                Chrome, Safari, Firefox, Edge
              </p>
            </div>

            {/* iOS App */}
            <div className="bg-white/5 backdrop-blur-sm rounded-3xl p-8 border border-white/10 text-center">
              <div className="inline-flex p-6 rounded-3xl bg-gradient-to-r from-gray-700 to-gray-900 mb-6">
                <Apple className="w-12 h-12 text-white" />
              </div>
              <div className="inline-block px-3 py-1 rounded-full bg-purple-500/20 text-purple-300 text-sm font-semibold mb-4">
                Coming Soon
              </div>
              <h2 className="text-3xl font-bold text-white mb-4">iOS App</h2>
              <p className="text-gray-300 mb-6">
                Native iOS app for iPhone and iPad with Face ID support.
              </p>
              <ul className="space-y-3 mb-8 text-left">
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Face ID / Touch ID</span>
                </li>
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Native performance</span>
                </li>
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Apple Wallet integration</span>
                </li>
              </ul>
              <button
                disabled
                className="block w-full py-4 px-6 bg-white/10 text-gray-400 rounded-xl font-semibold cursor-not-allowed"
              >
                Notify Me
              </button>
              <p className="text-sm text-gray-400 mt-4">
                iOS 13 or later
              </p>
            </div>

            {/* Android App */}
            <div className="bg-white/5 backdrop-blur-sm rounded-3xl p-8 border border-white/10 text-center">
              <div className="inline-flex p-6 rounded-3xl bg-gradient-to-r from-green-500 to-teal-500 mb-6">
                <Smartphone className="w-12 h-12 text-white" />
              </div>
              <div className="inline-block px-3 py-1 rounded-full bg-purple-500/20 text-purple-300 text-sm font-semibold mb-4">
                Coming Soon
              </div>
              <h2 className="text-3xl font-bold text-white mb-4">Android App</h2>
              <p className="text-gray-300 mb-6">
                Native Android app with biometric authentication.
              </p>
              <ul className="space-y-3 mb-8 text-left">
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Fingerprint / Face unlock</span>
                </li>
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Material Design</span>
                </li>
                <li className="flex items-start gap-2 text-gray-300">
                  <Check className="w-5 h-5 text-green-400 mt-0.5 flex-shrink-0" />
                  <span>Google Pay support</span>
                </li>
              </ul>
              <button
                disabled
                className="block w-full py-4 px-6 bg-white/10 text-gray-400 rounded-xl font-semibold cursor-not-allowed"
              >
                Notify Me
              </button>
              <p className="text-sm text-gray-400 mt-4">
                Android 8 or later
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] to-[#4a0080]">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-white text-center mb-16">
            Why Choose Ëtrid Wallet?
          </h2>
          <div className="grid md:grid-cols-3 gap-8">
            <div className="text-center">
              <div className="inline-flex p-4 rounded-2xl bg-purple-500/20 mb-6">
                <Shield className="w-8 h-8 text-purple-400" />
              </div>
              <h3 className="text-2xl font-bold text-white mb-3">
                Bank-Grade Security
              </h3>
              <p className="text-gray-300">
                Your keys, encrypted and stored securely. Multi-signature support for extra protection.
              </p>
            </div>
            <div className="text-center">
              <div className="inline-flex p-4 rounded-2xl bg-blue-500/20 mb-6">
                <Zap className="w-8 h-8 text-blue-400" />
              </div>
              <h3 className="text-2xl font-bold text-white mb-3">
                Lightning Fast
              </h3>
              <p className="text-gray-300">
                Optimized for speed with instant transactions and real-time updates.
              </p>
            </div>
            <div className="text-center">
              <div className="inline-flex p-4 rounded-2xl bg-green-500/20 mb-6">
                <Globe className="w-8 h-8 text-green-400" />
              </div>
              <h3 className="text-2xl font-bold text-white mb-3">
                Multi-Chain Support
              </h3>
              <p className="text-gray-300">
                Support for Ethereum, Bitcoin, and 100+ other blockchains.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Installation Guide */}
      <section className="py-24 bg-[#1a0033]">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-4xl font-bold text-white text-center mb-12">
            Quick Start Guide
          </h2>
          <div className="space-y-8">
            <div className="flex gap-6">
              <div className="flex-shrink-0 w-12 h-12 rounded-full bg-gradient-to-r from-purple-500 to-blue-500 flex items-center justify-center text-white font-bold text-xl">
                1
              </div>
              <div>
                <h3 className="text-2xl font-semibold text-white mb-2">
                  Choose Your Platform
                </h3>
                <p className="text-gray-300">
                  Select Web App for instant access, or wait for our mobile apps coming soon.
                </p>
              </div>
            </div>
            <div className="flex gap-6">
              <div className="flex-shrink-0 w-12 h-12 rounded-full bg-gradient-to-r from-purple-500 to-blue-500 flex items-center justify-center text-white font-bold text-xl">
                2
              </div>
              <div>
                <h3 className="text-2xl font-semibold text-white mb-2">
                  Create Your Wallet
                </h3>
                <p className="text-gray-300">
                  Set up your account in under 2 minutes. Secure your recovery phrase.
                </p>
              </div>
            </div>
            <div className="flex gap-6">
              <div className="flex-shrink-0 w-12 h-12 rounded-full bg-gradient-to-r from-purple-500 to-blue-500 flex items-center justify-center text-white font-bold text-xl">
                3
              </div>
              <div>
                <h3 className="text-2xl font-semibold text-white mb-2">
                  Start Using Features
                </h3>
                <p className="text-gray-300">
                  Buy crypto, trade, send to friends, explore NFTs, and more.
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="py-24 bg-gradient-to-b from-[#1a0033] to-[#4a0080]">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-4xl md:text-5xl font-bold text-white mb-6">
            Ready to Get Started?
          </h2>
          <p className="text-xl text-gray-300 mb-8">
            Join thousands of users managing their crypto with Ëtrid Wallet.
          </p>
          <a
            href="https://wallet.etrid.com"
            className="inline-flex items-center gap-2 px-8 py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold text-lg hover:scale-105 transition-transform"
          >
            <Chrome className="w-6 h-6" />
            Launch Web App Now
          </a>
        </div>
      </section>
    </main>
  );
}
