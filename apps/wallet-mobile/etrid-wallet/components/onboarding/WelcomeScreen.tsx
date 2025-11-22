import { ArrowRight, Wallet, Shield, Zap } from 'lucide-react';

interface FeatureItemProps {
  icon: React.ReactNode;
  title: string;
  description: string;
}

function FeatureItem({ icon, title, description }: FeatureItemProps) {
  return (
    <div className="flex items-start gap-4 text-left bg-white/5 backdrop-blur-sm rounded-xl p-4 border border-white/10">
      <div className="flex-shrink-0 w-12 h-12 rounded-xl bg-purple-500/20 flex items-center justify-center text-purple-400">
        {icon}
      </div>
      <div>
        <h3 className="font-semibold text-white mb-1">{title}</h3>
        <p className="text-sm text-gray-300">{description}</p>
      </div>
    </div>
  );
}

interface WelcomeScreenProps {
  onNext: () => void;
}

export default function WelcomeScreen({ onNext }: WelcomeScreenProps) {
  return (
    <div className="text-center space-y-8 animate-fadeIn">
      {/* Logo */}
      <div className="inline-flex items-center justify-center w-24 h-24 rounded-3xl bg-gradient-to-r from-purple-500 to-blue-500 mb-4">
        <Wallet className="w-12 h-12 text-white" />
      </div>

      {/* Title */}
      <div>
        <h1 className="text-4xl font-bold text-white mb-4">
          Welcome to Ëtrid
        </h1>
        <p className="text-xl text-gray-300">
          The Complete DeFi Wallet
        </p>
      </div>

      {/* Features */}
      <div className="space-y-4">
        <FeatureItem
          icon={<Shield className="w-6 h-6" />}
          title="Bank-Grade Security"
          description="Your keys, encrypted and stored securely"
        />
        <FeatureItem
          icon={<Zap className="w-6 h-6" />}
          title="18 Powerful Features"
          description="Trading, NFTs, DeFi lending, and more"
        />
        <FeatureItem
          icon={<Wallet className="w-6 h-6" />}
          title="Your Money, Your Control"
          description="Non-custodial wallet - you own your assets"
        />
      </div>

      {/* CTA */}
      <button
        onClick={onNext}
        className="w-full py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold flex items-center justify-center gap-2 hover:scale-105 transition-transform"
      >
        Get Started
        <ArrowRight className="w-5 h-5" />
      </button>

      <p className="text-sm text-gray-400">
        By continuing, you agree to our{' '}
        <a href="/terms" className="text-purple-400 hover:underline">
          Terms of Service
        </a>{' '}
        and{' '}
        <a href="/privacy" className="text-purple-400 hover:underline">
          Privacy Policy
        </a>
      </p>
    </div>
  );
}
