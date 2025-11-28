'use client';

import { useState } from 'react';
import { CreditCard, TrendingUp, Image, Users, ArrowLeft, ArrowRight, Sparkles } from 'lucide-react';

interface Tour {
  icon: React.ReactNode;
  title: string;
  description: string;
  color: string;
}

const tours: Tour[] = [
  {
    icon: <CreditCard className="w-12 h-12" />,
    title: 'AU Bloccard',
    description: 'Get a crypto-backed debit card and spend up to 60% of your collateral value anywhere.',
    color: 'from-purple-400 to-purple-600',
  },
  {
    icon: <TrendingUp className="w-12 h-12" />,
    title: 'Trade & Earn',
    description: 'Trade crypto with 0.1% fees, earn up to 15% APY on lending, and automate with DCA bots.',
    color: 'from-blue-400 to-blue-600',
  },
  {
    icon: <Image className="w-12 h-12" />,
    title: 'NFTs & Metaverse',
    description: 'Buy, sell, and mint NFTs. Manage your virtual land and wearables across multiple metaverses.',
    color: 'from-pink-400 to-pink-600',
  },
  {
    icon: <Users className="w-12 h-12" />,
    title: 'Social Features',
    description: 'Claim your @username.etrid, split bills with friends, and recover your wallet with guardians.',
    color: 'from-green-400 to-green-600',
  },
];

interface FeatureTourScreenProps {
  onNext: () => void;
  onSkip: () => void;
  onBack: () => void;
}

export default function FeatureTourScreen({ onNext, onSkip, onBack }: FeatureTourScreenProps) {
  const [currentIndex, setCurrentIndex] = useState(0);

  const isLast = currentIndex === tours.length - 1;
  const tour = tours[currentIndex];

  return (
    <div className="space-y-8 animate-fadeIn">
      {/* Feature Card */}
      <div className="text-center">
        <div className={`inline-flex p-6 rounded-3xl bg-gradient-to-r ${tour.color} mb-6 shadow-lg`}>
          {tour.icon}
        </div>

        <h2 className="text-3xl font-bold text-white mb-4">
          {tour.title}
        </h2>

        <p className="text-lg text-gray-300 max-w-md mx-auto leading-relaxed">
          {tour.description}
        </p>
      </div>

      {/* Additional Info based on feature */}
      <div className="bg-white/5 rounded-xl p-4 border border-white/10">
        <div className="flex items-start gap-3">
          <Sparkles className="w-5 h-5 text-yellow-400 flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="font-semibold text-white mb-1">Coming Soon</h3>
            <p className="text-sm text-gray-300">
              {currentIndex === 0 && "Apply for your AU Bloccard in the Cards section"}
              {currentIndex === 1 && "Access trading and earning in the DeFi section"}
              {currentIndex === 2 && "Explore the NFT marketplace in the Gallery"}
              {currentIndex === 3 && "Set up your profile in the Settings"}
            </p>
          </div>
        </div>
      </div>

      {/* Dots Indicator */}
      <div className="flex justify-center gap-2">
        {tours.map((_, index) => (
          <button
            key={index}
            onClick={() => setCurrentIndex(index)}
            className={`h-2 rounded-full transition-all ${
              index === currentIndex
                ? 'w-8 bg-purple-500'
                : 'w-2 bg-white/30 hover:bg-white/50'
            }`}
            aria-label={`Go to slide ${index + 1}`}
          />
        ))}
      </div>

      {/* Navigation */}
      <div className="flex gap-3">
        {currentIndex > 0 ? (
          <button
            onClick={() => setCurrentIndex((prev) => prev - 1)}
            className="flex-1 py-3 bg-white/10 text-white rounded-xl flex items-center justify-center gap-2 hover:bg-white/20 transition-colors"
          >
            <ArrowLeft className="w-5 h-5" />
            Previous
          </button>
        ) : (
          <button
            onClick={onBack}
            className="flex-1 py-3 bg-white/10 text-white rounded-xl hover:bg-white/20 transition-colors"
          >
            Back
          </button>
        )}

        {isLast ? (
          <button
            onClick={() => onNext()}
            className="flex-1 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold hover:scale-105 transition-transform"
          >
            Finish Tour
          </button>
        ) : (
          <button
            onClick={() => setCurrentIndex((prev) => prev + 1)}
            className="flex-1 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold flex items-center justify-center gap-2 hover:scale-105 transition-transform"
          >
            Next
            <ArrowRight className="w-5 h-5" />
          </button>
        )}
      </div>

      <button
        onClick={() => onSkip()}
        className="w-full py-2 text-gray-400 hover:text-white transition-colors"
      >
        Skip Tour
      </button>
    </div>
  );
}
