'use client';

import { useState } from 'react';
import { Fingerprint, Smartphone, Shield, CheckCircle2, Loader2 } from 'lucide-react';

interface SetupBiometricsScreenProps {
  onNext: () => void;
  onSkip: () => void;
  onBack: () => void;
}

export default function SetupBiometricsScreen({ onNext, onSkip, onBack }: SetupBiometricsScreenProps) {
  const [isEnabling, setIsEnabling] = useState(false);
  const [isEnabled, setIsEnabled] = useState(false);

  const handleEnable = async () => {
    setIsEnabling(true);

    // Simulate biometric authentication
    // In production, use platform-specific biometric APIs
    try {
      await new Promise(resolve => setTimeout(resolve, 1500));

      // Mock success
      setIsEnabled(true);
      setIsEnabling(false);

      // Auto-advance after short delay
      setTimeout(() => {
        onNext();
      }, 1000);
    } catch (error) {
      setIsEnabling(false);
      // Handle error
    }
  };

  return (
    <div className="space-y-6 animate-fadeIn">
      <div className="text-center">
        <div className="inline-flex p-4 rounded-2xl bg-green-500/20 mb-4">
          <Fingerprint className="w-8 h-8 text-green-400" />
        </div>
        <h2 className="text-3xl font-bold text-white mb-2">
          Enable Biometrics
        </h2>
        <p className="text-gray-300">
          Quickly and securely access your wallet
        </p>
      </div>

      {/* Benefits */}
      <div className="space-y-3">
        <div className="flex items-start gap-3 p-4 bg-white/5 rounded-xl border border-white/10">
          <Shield className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="font-semibold text-white mb-1">Enhanced Security</h3>
            <p className="text-sm text-gray-300">
              Your biometric data never leaves your device
            </p>
          </div>
        </div>

        <div className="flex items-start gap-3 p-4 bg-white/5 rounded-xl border border-white/10">
          <Smartphone className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="font-semibold text-white mb-1">Quick Access</h3>
            <p className="text-sm text-gray-300">
              Unlock your wallet with a tap or glance
            </p>
          </div>
        </div>

        <div className="flex items-start gap-3 p-4 bg-white/5 rounded-xl border border-white/10">
          <CheckCircle2 className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
          <div>
            <h3 className="font-semibold text-white mb-1">Optional Feature</h3>
            <p className="text-sm text-gray-300">
              You can always disable or change this later
            </p>
          </div>
        </div>
      </div>

      {/* Biometric Types */}
      <div className="bg-gradient-to-r from-purple-500/10 to-blue-500/10 border border-purple-500/30 rounded-xl p-4">
        <h3 className="font-semibold text-purple-400 mb-3 text-center">Supported Methods</h3>
        <div className="flex justify-center gap-6">
          <div className="text-center">
            <div className="w-12 h-12 rounded-xl bg-purple-500/20 flex items-center justify-center mx-auto mb-2">
              <Fingerprint className="w-6 h-6 text-purple-400" />
            </div>
            <p className="text-sm text-gray-300">Touch ID</p>
          </div>
          <div className="text-center">
            <div className="w-12 h-12 rounded-xl bg-blue-500/20 flex items-center justify-center mx-auto mb-2">
              <Smartphone className="w-6 h-6 text-blue-400" />
            </div>
            <p className="text-sm text-gray-300">Face ID</p>
          </div>
        </div>
      </div>

      {/* Status Message */}
      {isEnabled && (
        <div className="bg-green-500/10 border border-green-500/30 rounded-xl p-4 flex items-center gap-3 animate-fadeIn">
          <CheckCircle2 className="w-6 h-6 text-green-400" />
          <div>
            <h3 className="font-semibold text-green-400">Biometrics Enabled</h3>
            <p className="text-sm text-green-300">
              You can now unlock your wallet securely
            </p>
          </div>
        </div>
      )}

      {/* Navigation */}
      <div className="space-y-3">
        <button
          onClick={handleEnable}
          disabled={isEnabling || isEnabled}
          className="w-full py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold flex items-center justify-center gap-2 hover:scale-105 transition-transform disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
        >
          {isEnabling ? (
            <>
              <Loader2 className="w-5 h-5 animate-spin" />
              Enabling Biometrics...
            </>
          ) : isEnabled ? (
            <>
              <CheckCircle2 className="w-5 h-5" />
              Enabled
            </>
          ) : (
            <>
              <Fingerprint className="w-5 h-5" />
              Enable Biometrics
            </>
          )}
        </button>

        {!isEnabled && (
          <div className="flex gap-3">
            <button
              onClick={onBack}
              disabled={isEnabling}
              className="flex-1 py-3 bg-white/10 text-white rounded-xl hover:bg-white/20 transition-colors disabled:opacity-50"
            >
              Back
            </button>
            <button
              onClick={onSkip}
              disabled={isEnabling}
              className="flex-1 py-3 text-gray-400 hover:text-white transition-colors disabled:opacity-50"
            >
              Skip for Now
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
