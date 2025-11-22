'use client';

import { useState } from 'react';
import { Plus, Download, ArrowRight, AlertCircle } from 'lucide-react';

interface ImportWalletFormProps {
  onNext: (data: any) => void;
  onBack: () => void;
}

function ImportWalletForm({ onNext, onBack }: ImportWalletFormProps) {
  const [seedPhrase, setSeedPhrase] = useState('');
  const [error, setError] = useState('');

  const handleImport = () => {
    setError('');

    // Validate seed phrase
    const isValid = validateSeedPhrase(seedPhrase);

    if (isValid) {
      onNext({ mode: 'import', seedPhrase: seedPhrase.trim() });
    } else {
      setError('Invalid recovery phrase. Please enter 12 or 24 words separated by spaces.');
    }
  };

  return (
    <div className="space-y-6 animate-fadeIn">
      <div className="text-center">
        <h2 className="text-2xl font-bold text-white mb-2">
          Import Wallet
        </h2>
        <p className="text-sm text-gray-300">
          Enter your 12 or 24-word recovery phrase
        </p>
      </div>

      <div>
        <textarea
          value={seedPhrase}
          onChange={(e) => {
            setSeedPhrase(e.target.value);
            setError('');
          }}
          placeholder="word1 word2 word3 ..."
          className="w-full h-40 p-4 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-500 focus:border-purple-500 focus:outline-none resize-none font-mono"
        />
        {error && (
          <div className="mt-2 flex items-start gap-2 text-red-400 text-sm">
            <AlertCircle className="w-4 h-4 flex-shrink-0 mt-0.5" />
            <span>{error}</span>
          </div>
        )}
      </div>

      <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-xl p-4">
        <h3 className="font-semibold text-yellow-400 mb-2 flex items-center gap-2">
          <AlertCircle className="w-5 h-5" />
          Security Reminder
        </h3>
        <p className="text-sm text-yellow-300">
          Make sure you are in a private place and no one is watching your screen.
        </p>
      </div>

      <div className="flex gap-3">
        <button
          onClick={onBack}
          className="flex-1 py-3 bg-white/10 text-white rounded-xl hover:bg-white/20 transition-colors"
        >
          Back
        </button>
        <button
          onClick={handleImport}
          disabled={!seedPhrase.trim()}
          className="flex-1 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold disabled:opacity-50 disabled:cursor-not-allowed hover:scale-105 transition-transform"
        >
          Import
        </button>
      </div>
    </div>
  );
}

interface CreateWalletScreenProps {
  onNext: (data: any) => void;
  onBack: () => void;
}

export default function CreateWalletScreen({ onNext, onBack }: CreateWalletScreenProps) {
  const [mode, setMode] = useState<'create' | 'import' | null>(null);

  const handleCreate = async () => {
    // Generate new wallet
    const seedPhrase = generateSeedPhrase(); // 12-word mnemonic
    onNext({ mode: 'create', seedPhrase });
  };

  const handleImport = () => {
    setMode('import');
  };

  if (mode === 'import') {
    return <ImportWalletForm onNext={onNext} onBack={() => setMode(null)} />;
  }

  return (
    <div className="space-y-8 animate-fadeIn">
      <div className="text-center">
        <h2 className="text-3xl font-bold text-white mb-3">
          Create Your Wallet
        </h2>
        <p className="text-gray-300">
          Choose how you'd like to get started
        </p>
      </div>

      <div className="space-y-4">
        {/* Create New */}
        <button
          onClick={handleCreate}
          className="w-full p-6 bg-gradient-to-r from-purple-500/20 to-blue-500/20 border-2 border-purple-500/50 rounded-2xl hover:border-purple-400 transition-all group"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-xl bg-purple-500/30 flex items-center justify-center group-hover:scale-110 transition-transform">
              <Plus className="w-6 h-6 text-purple-400" />
            </div>
            <div className="flex-1 text-left">
              <h3 className="text-lg font-semibold text-white mb-1">
                Create New Wallet
              </h3>
              <p className="text-sm text-gray-300">
                Start fresh with a new wallet
              </p>
            </div>
            <ArrowRight className="w-5 h-5 text-gray-400 group-hover:text-purple-400 group-hover:translate-x-1 transition-all" />
          </div>
        </button>

        {/* Import Existing */}
        <button
          onClick={handleImport}
          className="w-full p-6 bg-white/5 border-2 border-white/10 rounded-2xl hover:border-white/30 transition-all group"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-xl bg-white/10 flex items-center justify-center group-hover:scale-110 transition-transform">
              <Download className="w-6 h-6 text-gray-400" />
            </div>
            <div className="flex-1 text-left">
              <h3 className="text-lg font-semibold text-white mb-1">
                Import Existing Wallet
              </h3>
              <p className="text-sm text-gray-300">
                Use your recovery phrase or private key
              </p>
            </div>
            <ArrowRight className="w-5 h-5 text-gray-400 group-hover:text-white group-hover:translate-x-1 transition-all" />
          </div>
        </button>
      </div>

      <button
        onClick={onBack}
        className="w-full py-3 text-gray-400 hover:text-white transition-colors"
      >
        Back
      </button>
    </div>
  );
}

function generateSeedPhrase(): string {
  // Mock: Generate 12-word mnemonic (in production, use proper BIP39 library)
  const words = [
    'abandon', 'ability', 'able', 'about', 'above', 'absent',
    'absorb', 'abstract', 'absurd', 'abuse', 'access', 'accident'
  ];
  return words.join(' ');
}

function validateSeedPhrase(phrase: string): boolean {
  const words = phrase.trim().split(/\s+/);
  const validLength = words.length === 12 || words.length === 24;
  const allWordsPresent = words.every(word => word.length > 0);
  return validLength && allWordsPresent;
}
