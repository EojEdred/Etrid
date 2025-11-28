'use client';

import { useState, useEffect } from 'react';
import { X, Download } from 'lucide-react';

export default function InstallPrompt() {
  const [deferredPrompt, setDeferredPrompt] = useState<any>(null);
  const [showPrompt, setShowPrompt] = useState(false);
  const [isInstalled, setIsInstalled] = useState(false);

  useEffect(() => {
    // Check if already installed
    if (window.matchMedia('(display-mode: standalone)').matches) {
      setIsInstalled(true);
      return;
    }

    // Listen for install prompt
    const handler = (e: Event) => {
      e.preventDefault();
      setDeferredPrompt(e);

      // Show prompt after 30 seconds
      setTimeout(() => {
        setShowPrompt(true);
      }, 30000);
    };

    window.addEventListener('beforeinstallprompt', handler);

    return () => {
      window.removeEventListener('beforeinstallprompt', handler);
    };
  }, []);

  const handleInstall = async () => {
    if (!deferredPrompt) return;

    deferredPrompt.prompt();
    const { outcome } = await deferredPrompt.userChoice;

    if (outcome === 'accepted') {
      setShowPrompt(false);
      setDeferredPrompt(null);
    }
  };

  if (isInstalled || !showPrompt) return null;

  return (
    <div className="fixed bottom-20 left-4 right-4 z-50">
      <div className="glass-strong p-4 rounded-2xl border border-purple-500/30">
        <div className="flex items-start gap-4">
          <div className="flex-shrink-0 w-12 h-12 rounded-xl bg-purple-500/20 flex items-center justify-center">
            <Download className="w-6 h-6 text-purple-400" />
          </div>

          <div className="flex-1">
            <h3 className="font-semibold text-white mb-1">
              Install Ëtrid Wallet
            </h3>
            <p className="text-sm text-gray-300 mb-3">
              Add to home screen for faster access and offline support
            </p>

            <div className="flex gap-2">
              <button
                onClick={handleInstall}
                className="px-4 py-2 bg-purple-500 text-white rounded-lg font-medium"
              >
                Install
              </button>
              <button
                onClick={() => setShowPrompt(false)}
                className="px-4 py-2 bg-white/10 text-white rounded-lg"
              >
                Later
              </button>
            </div>
          </div>

          <button
            onClick={() => setShowPrompt(false)}
            className="flex-shrink-0 text-gray-400 hover:text-white"
          >
            <X className="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  );
}
