'use client';

import { Shield, Lock, Eye, AlertTriangle, CheckCircle2 } from 'lucide-react';

interface SecurityPointProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  type: 'do' | 'dont';
}

function SecurityPoint({ icon, title, description, type }: SecurityPointProps) {
  const colors = type === 'do'
    ? 'bg-green-500/10 border-green-500/30 text-green-400'
    : 'bg-red-500/10 border-red-500/30 text-red-400';

  return (
    <div className={`flex items-start gap-3 p-4 rounded-xl border ${colors}`}>
      <div className="flex-shrink-0 mt-0.5">
        {icon}
      </div>
      <div>
        <h3 className="font-semibold text-white mb-1">{title}</h3>
        <p className="text-sm text-gray-300">{description}</p>
      </div>
    </div>
  );
}

interface SecurityScreenProps {
  onNext: () => void;
  onBack: () => void;
}

export default function SecurityScreen({ onNext, onBack }: SecurityScreenProps) {
  return (
    <div className="space-y-6 animate-fadeIn">
      <div className="text-center">
        <div className="inline-flex p-4 rounded-2xl bg-blue-500/20 mb-4">
          <Shield className="w-8 h-8 text-blue-400" />
        </div>
        <h2 className="text-3xl font-bold text-white mb-2">
          Security First
        </h2>
        <p className="text-gray-300">
          Understanding wallet security basics
        </p>
      </div>

      {/* Key Concepts */}
      <div className="space-y-3">
        <h3 className="text-lg font-semibold text-white mb-2">What You Need to Know</h3>

        <div className="space-y-3">
          <div className="p-4 bg-white/5 rounded-xl border border-white/10">
            <div className="flex items-start gap-3 mb-2">
              <Lock className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
              <div>
                <h4 className="font-semibold text-white mb-1">You Control Your Keys</h4>
                <p className="text-sm text-gray-300">
                  Your wallet is secured by a recovery phrase (seed phrase). This is the ONLY way to recover your wallet.
                </p>
              </div>
            </div>
          </div>

          <div className="p-4 bg-white/5 rounded-xl border border-white/10">
            <div className="flex items-start gap-3 mb-2">
              <Eye className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
              <div>
                <h4 className="font-semibold text-white mb-1">No One Can Help You Recover</h4>
                <p className="text-sm text-gray-300">
                  Ëtrid cannot reset your password or recover your wallet. Only you have access.
                </p>
              </div>
            </div>
          </div>

          <div className="p-4 bg-white/5 rounded-xl border border-white/10">
            <div className="flex items-start gap-3 mb-2">
              <Shield className="w-5 h-5 text-purple-400 flex-shrink-0 mt-0.5" />
              <div>
                <h4 className="font-semibold text-white mb-1">You Are Responsible</h4>
                <p className="text-sm text-gray-300">
                  Keep your recovery phrase safe. Anyone with it can access all your funds.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Best Practices */}
      <div className="space-y-3">
        <h3 className="text-lg font-semibold text-white mb-2">Security Best Practices</h3>

        <SecurityPoint
          type="do"
          icon={<CheckCircle2 className="w-5 h-5" />}
          title="DO: Write it down offline"
          description="Store your recovery phrase on paper in a safe place"
        />

        <SecurityPoint
          type="do"
          icon={<CheckCircle2 className="w-5 h-5" />}
          title="DO: Use biometric locks"
          description="Enable Face ID or fingerprint for quick secure access"
        />

        <SecurityPoint
          type="dont"
          icon={<AlertTriangle className="w-5 h-5" />}
          title="DON'T: Share your phrase"
          description="Never share your recovery phrase with anyone, including Ëtrid support"
        />

        <SecurityPoint
          type="dont"
          icon={<AlertTriangle className="w-5 h-5" />}
          title="DON'T: Store it digitally"
          description="Avoid screenshots, cloud storage, or digital notes"
        />
      </div>

      {/* Navigation */}
      <div className="flex gap-3 pt-2">
        <button
          onClick={onBack}
          className="flex-1 py-3 bg-white/10 text-white rounded-xl hover:bg-white/20 transition-colors"
        >
          Back
        </button>
        <button
          onClick={onNext}
          className="flex-1 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold hover:scale-105 transition-transform"
        >
          I Understand
        </button>
      </div>
    </div>
  );
}
