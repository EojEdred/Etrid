import Hero from './hero';
import Features from './features';
import HowItWorks from './how-it-works';
import SupportedChains from './supported-chains';
import Statistics from './statistics';

export default function LightningPage() {
  return (
    <main className="min-h-screen bg-black">
      <Hero />
      <Features />
      <HowItWorks />
      <SupportedChains />
      <Statistics />

      {/* Footer section */}
      <footer className="bg-zinc-900 border-t border-zinc-800 mt-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-8">
            <div>
              <h3 className="text-lg font-bold text-white mb-4">Documentation</h3>
              <ul className="space-y-2 text-zinc-400">
                <li><a href="#" className="hover:text-white transition">Getting Started</a></li>
                <li><a href="#" className="hover:text-white transition">API Reference</a></li>
                <li><a href="#" className="hover:text-white transition">Channel Management</a></li>
              </ul>
            </div>
            <div>
              <h3 className="text-lg font-bold text-white mb-4">Community</h3>
              <ul className="space-y-2 text-zinc-400">
                <li><a href="#" className="hover:text-white transition">Discord</a></li>
                <li><a href="#" className="hover:text-white transition">GitHub</a></li>
                <li><a href="#" className="hover:text-white transition">Twitter</a></li>
              </ul>
            </div>
            <div>
              <h3 className="text-lg font-bold text-white mb-4">Network</h3>
              <ul className="space-y-2 text-zinc-400">
                <li><a href="#" className="hover:text-white transition">Mainnet</a></li>
                <li><a href="#" className="hover:text-white transition">Testnet</a></li>
                <li><a href="#" className="hover:text-white transition">Status Page</a></li>
              </ul>
            </div>
          </div>
          <div className="border-t border-zinc-800 pt-8">
            <p className="text-center text-zinc-500 text-sm">
              ÉTRID Protocol &bull; Lightning Network &bull; Built with Next.js 16
            </p>
          </div>
        </div>
      </footer>
    </main>
  );
}
