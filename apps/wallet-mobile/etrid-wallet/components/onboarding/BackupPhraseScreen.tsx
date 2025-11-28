'use client';

import { useState } from 'react';
import { Eye, EyeOff, Copy, Check, AlertTriangle } from 'lucide-react';

interface BackupPhraseScreenProps {
  seedPhrase: string;
  onNext: () => void;
  onBack: () => void;
}

export default function BackupPhraseScreen({ seedPhrase, onNext, onBack }: BackupPhraseScreenProps) {
  const [revealed, setRevealed] = useState(false);
  const [copied, setCopied] = useState(false);
  const [confirmed, setConfirmed] = useState(false);

  const words = seedPhrase ? seedPhrase.split(' ') : [];

  const handleCopy = () => {
    navigator.clipboard.writeText(seedPhrase);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="space-y-6 animate-fadeIn">
      <div className="text-center">
        <div className="inline-flex p-4 rounded-2xl bg-yellow-500/20 mb-4">
          <AlertTriangle className="w-8 h-8 text-yellow-400" />
        </div>
        <h2 className="text-3xl font-bold text-white mb-2">
          Backup Your Wallet
        </h2>
        <p className="text-gray-300">
          Write down these 12 words in order and keep them safe
        </p>
      </div>

      {/* Warning */}
      <div className="bg-red-500/10 border border-red-500/30 rounded-xl p-4">
        <h3 className="font-semibold text-red-400 mb-2">⚠️ Important</h3>
        <ul className="text-sm text-red-300 space-y-1">
          <li>• Never share your recovery phrase with anyone</li>
          <li>• Store it somewhere safe and offline</li>
          <li>• Anyone with this phrase can access your funds</li>
          <li>• Ëtrid will never ask for your recovery phrase</li>
        </ul>
      </div>

      {/* Seed Phrase Display */}
      <div className="relative">
        <div className={`grid grid-cols-3 gap-3 p-4 bg-white/5 border border-white/10 rounded-xl transition-all ${!revealed && 'blur-sm select-none'}`}>
          {words.map((word, index) => (
            <div key={index} className="flex items-center gap-2 p-2 bg-white/5 rounded-lg">
              <span className="text-xs text-gray-400 w-6">{index + 1}.</span>
              <span className="text-white font-mono text-sm">{word}</span>
            </div>
          ))}
        </div>

        {!revealed && (
          <div className="absolute inset-0 flex items-center justify-center">
            <button
              onClick={() => setRevealed(true)}
              className="px-6 py-3 bg-purple-500 text-white rounded-xl font-semibold flex items-center gap-2 hover:scale-105 transition-transform shadow-lg"
            >
              <Eye className="w-5 h-5" />
              Reveal Phrase
            </button>
          </div>
        )}
      </div>

      {/* Actions */}
      {revealed && (
        <div className="space-y-3 animate-fadeIn">
          <button
            onClick={handleCopy}
            className="w-full py-3 bg-white/10 text-white rounded-xl flex items-center justify-center gap-2 hover:bg-white/20 transition-colors"
          >
            {copied ? (
              <>
                <Check className="w-5 h-5 text-green-400" />
                <span className="text-green-400">Copied!</span>
              </>
            ) : (
              <>
                <Copy className="w-5 h-5" />
                Copy to Clipboard
              </>
            )}
          </button>

          <label className="flex items-start gap-3 p-4 bg-white/5 rounded-xl cursor-pointer hover:bg-white/10 transition-colors">
            <input
              type="checkbox"
              checked={confirmed}
              onChange={(e) => setConfirmed(e.target.checked)}
              className="mt-1 w-5 h-5 rounded border-gray-600 text-purple-500 focus:ring-purple-500 cursor-pointer"
            />
            <span className="text-sm text-gray-300">
              I have written down my recovery phrase and stored it in a safe place.
              I understand that if I lose it, I will lose access to my wallet.
            </span>
          </label>
        </div>
      )}

      {/* Navigation */}
      <div className="flex gap-3">
        <button
          onClick={onBack}
          className="flex-1 py-3 bg-white/10 text-white rounded-xl hover:bg-white/20 transition-colors"
        >
          Back
        </button>
        <button
          onClick={() => onNext()}
          disabled={!confirmed}
          className="flex-1 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold disabled:opacity-50 disabled:cursor-not-allowed hover:scale-105 transition-transform"
        >
          Continue
        </button>
      </div>
    </div>
  );
}
