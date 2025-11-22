'use client';

import { useState, useEffect } from 'react';
import { CheckCircle2, XCircle, AlertCircle } from 'lucide-react';

interface VerifyPhraseScreenProps {
  seedPhrase: string;
  onNext: () => void;
  onBack: () => void;
}

export default function VerifyPhraseScreen({ seedPhrase, onNext, onBack }: VerifyPhraseScreenProps) {
  const [selectedWords, setSelectedWords] = useState<string[]>([]);
  const [shuffledWords, setShuffledWords] = useState<string[]>([]);
  const [verificationIndices, setVerificationIndices] = useState<number[]>([]);
  const [isCorrect, setIsCorrect] = useState<boolean | null>(null);
  const [attempts, setAttempts] = useState(0);

  const words = seedPhrase ? seedPhrase.split(' ') : [];
  const maxAttempts = 3;

  useEffect(() => {
    // Select 3 random positions to verify
    const indices: number[] = [];
    while (indices.length < 3) {
      const rand = Math.floor(Math.random() * words.length);
      if (!indices.includes(rand)) {
        indices.push(rand);
      }
    }
    indices.sort((a, b) => a - b);
    setVerificationIndices(indices);

    // Create shuffled word options (correct words + some random wrong words)
    const correctWords = indices.map(i => words[i]);
    const wrongWords = words
      .filter((_, i) => !indices.includes(i))
      .sort(() => Math.random() - 0.5)
      .slice(0, 6);

    const allOptions = [...correctWords, ...wrongWords]
      .sort(() => Math.random() - 0.5);

    setShuffledWords(allOptions);
  }, [seedPhrase]);

  const handleWordClick = (word: string) => {
    if (selectedWords.includes(word)) {
      setSelectedWords(selectedWords.filter(w => w !== word));
    } else if (selectedWords.length < 3) {
      setSelectedWords([...selectedWords, word]);
    }
  };

  const handleVerify = () => {
    const correctWords = verificationIndices.map(i => words[i]);
    const allCorrect = selectedWords.every((word, index) => word === correctWords[index]);

    setIsCorrect(allCorrect);
    setAttempts(attempts + 1);

    if (allCorrect) {
      setTimeout(() => {
        onNext();
      }, 1500);
    }
  };

  const handleReset = () => {
    setSelectedWords([]);
    setIsCorrect(null);
  };

  return (
    <div className="space-y-6 animate-fadeIn">
      <div className="text-center">
        <div className="inline-flex p-4 rounded-2xl bg-purple-500/20 mb-4">
          <CheckCircle2 className="w-8 h-8 text-purple-400" />
        </div>
        <h2 className="text-3xl font-bold text-white mb-2">
          Verify Your Backup
        </h2>
        <p className="text-gray-300">
          Select the words in the correct order
        </p>
      </div>

      {/* Instructions */}
      <div className="bg-blue-500/10 border border-blue-500/30 rounded-xl p-4">
        <h3 className="font-semibold text-blue-400 mb-2 flex items-center gap-2">
          <AlertCircle className="w-5 h-5" />
          Verification Test
        </h3>
        <p className="text-sm text-blue-300">
          Please select the words at positions:{' '}
          <span className="font-bold">
            {verificationIndices.map(i => `#${i + 1}`).join(', ')}
          </span>
        </p>
      </div>

      {/* Selected Words Display */}
      <div className="space-y-2">
        <h3 className="text-sm font-semibold text-gray-400">Your Selection:</h3>
        <div className="grid grid-cols-3 gap-3">
          {verificationIndices.map((index, i) => (
            <div
              key={index}
              className={`p-3 rounded-xl border-2 text-center min-h-[60px] flex items-center justify-center ${
                selectedWords[i]
                  ? 'bg-purple-500/20 border-purple-500/50'
                  : 'bg-white/5 border-white/10 border-dashed'
              }`}
            >
              <div>
                <div className="text-xs text-gray-400 mb-1">#{index + 1}</div>
                <div className="text-white font-mono text-sm">
                  {selectedWords[i] || '---'}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Word Options */}
      <div className="space-y-2">
        <h3 className="text-sm font-semibold text-gray-400">Select Words:</h3>
        <div className="grid grid-cols-3 gap-2">
          {shuffledWords.map((word, index) => {
            const isSelected = selectedWords.includes(word);
            return (
              <button
                key={index}
                onClick={() => handleWordClick(word)}
                disabled={isCorrect !== null}
                className={`p-3 rounded-xl border-2 font-mono text-sm transition-all ${
                  isSelected
                    ? 'bg-purple-500/30 border-purple-500 text-white scale-95'
                    : 'bg-white/5 border-white/10 text-gray-300 hover:border-white/30 hover:bg-white/10'
                } disabled:opacity-50 disabled:cursor-not-allowed`}
              >
                {word}
              </button>
            );
          })}
        </div>
      </div>

      {/* Verification Result */}
      {isCorrect !== null && (
        <div
          className={`p-4 rounded-xl border-2 flex items-center gap-3 animate-fadeIn ${
            isCorrect
              ? 'bg-green-500/10 border-green-500/30'
              : 'bg-red-500/10 border-red-500/30'
          }`}
        >
          {isCorrect ? (
            <>
              <CheckCircle2 className="w-6 h-6 text-green-400" />
              <div>
                <h3 className="font-semibold text-green-400">Perfect!</h3>
                <p className="text-sm text-green-300">
                  Your recovery phrase is verified
                </p>
              </div>
            </>
          ) : (
            <>
              <XCircle className="w-6 h-6 text-red-400" />
              <div className="flex-1">
                <h3 className="font-semibold text-red-400">Incorrect</h3>
                <p className="text-sm text-red-300">
                  {attempts >= maxAttempts
                    ? 'Please go back and review your phrase'
                    : 'Please try again'}
                </p>
              </div>
            </>
          )}
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

        {isCorrect === false ? (
          <button
            onClick={handleReset}
            disabled={attempts >= maxAttempts}
            className="flex-1 py-3 bg-purple-500/20 text-purple-300 rounded-xl hover:bg-purple-500/30 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Try Again ({maxAttempts - attempts} left)
          </button>
        ) : (
          <button
            onClick={handleVerify}
            disabled={selectedWords.length !== 3 || isCorrect !== null}
            className="flex-1 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-xl font-semibold disabled:opacity-50 disabled:cursor-not-allowed hover:scale-105 transition-transform"
          >
            Verify
          </button>
        )}
      </div>
    </div>
  );
}
