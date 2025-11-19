'use client';

import { useState, useEffect } from 'react';
import { WifiOff, Wifi } from 'lucide-react';

export default function OnlineStatus() {
  const [isOnline, setIsOnline] = useState(true);
  const [showBanner, setShowBanner] = useState(false);

  useEffect(() => {
    const handleOnline = () => {
      setIsOnline(true);
      setShowBanner(true);
      setTimeout(() => setShowBanner(false), 3000);
    };

    const handleOffline = () => {
      setIsOnline(false);
      setShowBanner(true);
    };

    setIsOnline(navigator.onLine);

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  if (!showBanner) return null;

  return (
    <div className={`fixed top-4 left-4 right-4 z-50 ${
      isOnline ? 'bg-green-500' : 'bg-red-500'
    } text-white px-4 py-3 rounded-lg shadow-lg flex items-center gap-3`}>
      {isOnline ? (
        <>
          <Wifi className="w-5 h-5" />
          <span>Back online!</span>
        </>
      ) : (
        <>
          <WifiOff className="w-5 h-5" />
          <span>You're offline. Some features may be limited.</span>
        </>
      )}
    </div>
  );
}
