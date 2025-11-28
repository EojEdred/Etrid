'use client';

import { useState, useEffect } from 'react';
import { Activity, Smartphone, Wifi, Database, Bell, Download } from 'lucide-react';
import {
  isPWA,
  isIOS,
  isAndroid,
  getDisplayMode,
  supportsServiceWorker,
  supportsPushNotifications,
  supportsIndexedDB,
  getConnectionType,
  getCacheSize,
  formatBytes,
  getStorageEstimate,
} from '@/lib/pwa/utils';

interface PWAStatusProps {
  showDetails?: boolean;
}

export default function PWAStatus({ showDetails = false }: PWAStatusProps) {
  const [isClient, setIsClient] = useState(false);
  const [cacheSize, setCacheSize] = useState<string>('Calculating...');
  const [storageInfo, setStorageInfo] = useState<string>('');

  useEffect(() => {
    setIsClient(true);

    // Get cache size
    getCacheSize().then(size => {
      setCacheSize(formatBytes(size));
    });

    // Get storage info
    getStorageEstimate().then(estimate => {
      if (estimate) {
        const usage = formatBytes(estimate.usage);
        const quota = formatBytes(estimate.quota);
        const percent = Math.round((estimate.usage / estimate.quota) * 100);
        setStorageInfo(`${usage} / ${quota} (${percent}%)`);
      }
    });
  }, []);

  if (!isClient) return null;

  const capabilities = [
    {
      name: 'Running as PWA',
      icon: Smartphone,
      status: isPWA(),
      color: isPWA() ? 'text-green-400' : 'text-gray-400',
    },
    {
      name: 'Service Worker',
      icon: Activity,
      status: supportsServiceWorker(),
      color: supportsServiceWorker() ? 'text-green-400' : 'text-red-400',
    },
    {
      name: 'Push Notifications',
      icon: Bell,
      status: supportsPushNotifications(),
      color: supportsPushNotifications() ? 'text-green-400' : 'text-yellow-400',
    },
    {
      name: 'Offline Storage',
      icon: Database,
      status: supportsIndexedDB(),
      color: supportsIndexedDB() ? 'text-green-400' : 'text-red-400',
    },
  ];

  if (!showDetails) {
    // Compact status badge
    const allSupported = capabilities.every(cap => cap.status);
    return (
      <div className={`inline-flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium ${
        allSupported ? 'bg-green-500/20 text-green-400' : 'bg-yellow-500/20 text-yellow-400'
      }`}>
        <Activity className="w-3 h-3" />
        <span>PWA {allSupported ? 'Active' : 'Limited'}</span>
      </div>
    );
  }

  return (
    <div className="bg-white/5 border border-white/10 rounded-2xl p-6">
      <h3 className="text-lg font-semibold text-white mb-4">PWA Status</h3>

      {/* Capabilities Grid */}
      <div className="grid grid-cols-2 gap-4 mb-6">
        {capabilities.map((cap) => (
          <div key={cap.name} className="flex items-start gap-3">
            <div className={`p-2 rounded-lg bg-white/5 ${cap.color}`}>
              <cap.icon className="w-5 h-5" />
            </div>
            <div>
              <div className="text-sm font-medium text-white">{cap.name}</div>
              <div className={`text-xs ${cap.color}`}>
                {cap.status ? 'Supported' : 'Not Available'}
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Device Info */}
      <div className="space-y-3 pt-4 border-t border-white/10">
        <div className="flex justify-between text-sm">
          <span className="text-gray-400">Platform</span>
          <span className="text-white font-medium">
            {isIOS() ? 'iOS' : isAndroid() ? 'Android' : 'Desktop'}
          </span>
        </div>

        <div className="flex justify-between text-sm">
          <span className="text-gray-400">Display Mode</span>
          <span className="text-white font-medium capitalize">{getDisplayMode()}</span>
        </div>

        <div className="flex justify-between text-sm">
          <span className="text-gray-400">Connection</span>
          <span className="text-white font-medium capitalize">{getConnectionType()}</span>
        </div>

        <div className="flex justify-between text-sm">
          <span className="text-gray-400">Cache Size</span>
          <span className="text-white font-medium">{cacheSize}</span>
        </div>

        {storageInfo && (
          <div className="flex justify-between text-sm">
            <span className="text-gray-400">Storage Used</span>
            <span className="text-white font-medium">{storageInfo}</span>
          </div>
        )}
      </div>
    </div>
  );
}
