'use client';

import { WifiOff } from 'lucide-react';

export default function OfflinePage() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-[#1a0033] to-[#4a0080] flex items-center justify-center p-4">
      <div className="text-center">
        <div className="inline-flex items-center justify-center w-24 h-24 rounded-full bg-white/10 mb-6">
          <WifiOff className="w-12 h-12 text-purple-400" />
        </div>

        <h1 className="text-2xl font-bold text-white mb-2">
          You're Offline
        </h1>

        <p className="text-gray-300 mb-6 max-w-md">
          Some features require an internet connection. Please check your connection and try again.
        </p>

        <button
          onClick={() => window.location.reload()}
          className="px-6 py-3 bg-purple-500 text-white rounded-lg font-medium"
        >
          Try Again
        </button>

        <div className="mt-8 text-sm text-gray-400">
          <p>You can still access:</p>
          <ul className="mt-2 space-y-1">
            <li>Your wallet balance (cached)</li>
            <li>Transaction history (cached)</li>
            <li>Settings</li>
          </ul>
        </div>
      </div>
    </div>
  );
}
