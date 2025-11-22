import { CheckCircle2, ArrowRight, Sparkles } from 'lucide-react';
import Link from 'next/link';

export default function CompleteScreen() {
  return (
    <div className="text-center space-y-8 animate-fadeIn">
      {/* Success Animation */}
      <div className="relative inline-flex items-center justify-center mb-4">
        <div className="absolute inset-0 bg-green-500/20 rounded-full blur-xl animate-pulse" />
        <div className="relative inline-flex items-center justify-center w-24 h-24 rounded-full bg-green-500/20">
          <CheckCircle2 className="w-16 h-16 text-green-400" />
        </div>
      </div>

      <div>
        <h1 className="text-4xl font-bold text-white mb-4">
          You're All Set!
        </h1>
        <p className="text-xl text-gray-300">
          Your Ëtrid wallet is ready to use
        </p>
      </div>

      {/* Completion Checklist */}
      <div className="space-y-3 text-left max-w-md mx-auto">
        <div className="flex items-center gap-3 p-4 bg-white/5 rounded-xl border border-green-500/30">
          <CheckCircle2 className="w-5 h-5 text-green-400 flex-shrink-0" />
          <span className="text-white">Wallet created securely</span>
        </div>
        <div className="flex items-center gap-3 p-4 bg-white/5 rounded-xl border border-green-500/30">
          <CheckCircle2 className="w-5 h-5 text-green-400 flex-shrink-0" />
          <span className="text-white">Recovery phrase backed up</span>
        </div>
        <div className="flex items-center gap-3 p-4 bg-white/5 rounded-xl border border-green-500/30">
          <CheckCircle2 className="w-5 h-5 text-green-400 flex-shrink-0" />
          <span className="text-white">Biometrics enabled</span>
        </div>
        <div className="flex items-center gap-3 p-4 bg-white/5 rounded-xl border border-green-500/30">
          <CheckCircle2 className="w-5 h-5 text-green-400 flex-shrink-0" />
          <span className="text-white">Features tour completed</span>
        </div>
      </div>

      {/* Next Steps */}
      <div className="bg-gradient-to-r from-purple-500/10 to-blue-500/10 rounded-xl p-4 border border-purple-500/30">
        <div className="flex items-start gap-3">
          <Sparkles className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
          <div className="text-left">
            <h3 className="font-semibold text-white mb-1">What's Next?</h3>
            <p className="text-sm text-gray-300">
              Fund your wallet to start trading, earning, and exploring DeFi
            </p>
          </div>
        </div>
      </div>

      {/* CTA Button */}
      <Link
        href="/"
        className="block w-full py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold flex items-center justify-center gap-2 hover:scale-105 transition-transform shadow-lg"
      >
        Start Using Ëtrid
        <ArrowRight className="w-5 h-5" />
      </Link>

      {/* Optional: Show wallet address */}
      <div className="pt-4">
        <p className="text-xs text-gray-500">
          Your wallet is secured and encrypted on this device
        </p>
      </div>
    </div>
  );
}
