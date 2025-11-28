/**
 * PWA Utility Functions
 * Helper functions for Progressive Web App features
 */

/**
 * Check if the app is running as a PWA (standalone mode)
 */
export function isPWA(): boolean {
  if (typeof window === 'undefined') return false;

  return (
    window.matchMedia('(display-mode: standalone)').matches ||
    (window.navigator as any).standalone === true ||
    document.referrer.includes('android-app://')
  );
}

/**
 * Check if the device is iOS
 */
export function isIOS(): boolean {
  if (typeof window === 'undefined') return false;

  return /iPad|iPhone|iPod/.test(navigator.userAgent) && !(window as any).MSStream;
}

/**
 * Check if the device is Android
 */
export function isAndroid(): boolean {
  if (typeof window === 'undefined') return false;

  return /Android/.test(navigator.userAgent);
}

/**
 * Check if the browser supports PWA installation
 */
export function supportsInstallPrompt(): boolean {
  if (typeof window === 'undefined') return false;

  return 'BeforeInstallPromptEvent' in window;
}

/**
 * Check if service workers are supported
 */
export function supportsServiceWorker(): boolean {
  if (typeof window === 'undefined') return false;

  return 'serviceWorker' in navigator;
}

/**
 * Check if push notifications are supported
 */
export function supportsPushNotifications(): boolean {
  if (typeof window === 'undefined') return false;

  return 'PushManager' in window && 'Notification' in window;
}

/**
 * Check if IndexedDB is supported
 */
export function supportsIndexedDB(): boolean {
  if (typeof window === 'undefined') return false;

  return 'indexedDB' in window;
}

/**
 * Get the network connection type
 */
export function getConnectionType(): string {
  if (typeof window === 'undefined') return 'unknown';

  const connection = (navigator as any).connection ||
                     (navigator as any).mozConnection ||
                     (navigator as any).webkitConnection;

  if (!connection) return 'unknown';

  return connection.effectiveType || connection.type || 'unknown';
}

/**
 * Check if the device has a slow connection
 */
export function isSlowConnection(): boolean {
  const connectionType = getConnectionType();
  return ['slow-2g', '2g'].includes(connectionType);
}

/**
 * Get device pixel ratio
 */
export function getDevicePixelRatio(): number {
  if (typeof window === 'undefined') return 1;

  return window.devicePixelRatio || 1;
}

/**
 * Check if the app can use the cache
 */
export function canUseCache(): boolean {
  return supportsServiceWorker() && 'caches' in window;
}

/**
 * Clear all caches
 */
export async function clearAllCaches(): Promise<void> {
  if (!canUseCache()) return;

  const cacheNames = await caches.keys();
  await Promise.all(cacheNames.map(name => caches.delete(name)));
}

/**
 * Get cache size (approximate)
 */
export async function getCacheSize(): Promise<number> {
  if (!canUseCache()) return 0;

  let totalSize = 0;
  const cacheNames = await caches.keys();

  for (const cacheName of cacheNames) {
    const cache = await caches.open(cacheName);
    const requests = await cache.keys();

    for (const request of requests) {
      const response = await cache.match(request);
      if (response) {
        const blob = await response.blob();
        totalSize += blob.size;
      }
    }
  }

  return totalSize;
}

/**
 * Format bytes to human-readable format
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

/**
 * Request persistent storage
 */
export async function requestPersistentStorage(): Promise<boolean> {
  if (typeof navigator === 'undefined' || !navigator.storage || !navigator.storage.persist) {
    return false;
  }

  try {
    const isPersisted = await navigator.storage.persist();
    return isPersisted;
  } catch (error) {
    console.error('Error requesting persistent storage:', error);
    return false;
  }
}

/**
 * Check if storage is persistent
 */
export async function isStoragePersisted(): Promise<boolean> {
  if (typeof navigator === 'undefined' || !navigator.storage || !navigator.storage.persisted) {
    return false;
  }

  try {
    return await navigator.storage.persisted();
  } catch (error) {
    console.error('Error checking storage persistence:', error);
    return false;
  }
}

/**
 * Get storage estimate
 */
export async function getStorageEstimate(): Promise<{ usage: number; quota: number } | null> {
  if (typeof navigator === 'undefined' || !navigator.storage || !navigator.storage.estimate) {
    return null;
  }

  try {
    const estimate = await navigator.storage.estimate();
    return {
      usage: estimate.usage || 0,
      quota: estimate.quota || 0,
    };
  } catch (error) {
    console.error('Error getting storage estimate:', error);
    return null;
  }
}

/**
 * Check if the app should update
 */
export async function checkForUpdate(): Promise<boolean> {
  if (!supportsServiceWorker()) return false;

  try {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();

    return !!registration.waiting;
  } catch (error) {
    console.error('Error checking for update:', error);
    return false;
  }
}

/**
 * Skip waiting and activate new service worker
 */
export async function skipWaitingAndReload(): Promise<void> {
  if (!supportsServiceWorker()) return;

  try {
    const registration = await navigator.serviceWorker.ready;

    if (registration.waiting) {
      registration.waiting.postMessage({ type: 'SKIP_WAITING' });

      // Reload after new service worker takes control
      navigator.serviceWorker.addEventListener('controllerchange', () => {
        window.location.reload();
      });
    }
  } catch (error) {
    console.error('Error skipping waiting:', error);
  }
}

/**
 * Share content using Web Share API
 */
export async function share(data: ShareData): Promise<boolean> {
  if (typeof navigator === 'undefined' || !navigator.share) {
    return false;
  }

  try {
    await navigator.share(data);
    return true;
  } catch (error) {
    console.error('Error sharing:', error);
    return false;
  }
}

/**
 * Check if Web Share API is supported
 */
export function canShare(data?: ShareData): boolean {
  if (typeof navigator === 'undefined' || !navigator.share) {
    return false;
  }

  if (data && navigator.canShare) {
    return navigator.canShare(data);
  }

  return true;
}

/**
 * Vibrate device (if supported)
 */
export function vibrate(pattern: number | number[]): boolean {
  if (typeof navigator === 'undefined' || !navigator.vibrate) {
    return false;
  }

  try {
    navigator.vibrate(pattern);
    return true;
  } catch (error) {
    console.error('Error vibrating:', error);
    return false;
  }
}

/**
 * Keep screen awake using Wake Lock API
 */
export async function keepScreenAwake(): Promise<WakeLockSentinel | null> {
  if (typeof navigator === 'undefined' || !('wakeLock' in navigator)) {
    return null;
  }

  try {
    const wakeLock = await (navigator as any).wakeLock.request('screen');
    return wakeLock;
  } catch (error) {
    console.error('Error requesting wake lock:', error);
    return null;
  }
}

/**
 * Get display mode
 */
export function getDisplayMode(): 'browser' | 'standalone' | 'minimal-ui' | 'fullscreen' {
  if (typeof window === 'undefined') return 'browser';

  if (window.matchMedia('(display-mode: fullscreen)').matches) {
    return 'fullscreen';
  }
  if (window.matchMedia('(display-mode: standalone)').matches) {
    return 'standalone';
  }
  if (window.matchMedia('(display-mode: minimal-ui)').matches) {
    return 'minimal-ui';
  }
  return 'browser';
}

/**
 * Log PWA capabilities
 */
export function logPWACapabilities(): void {
  if (typeof console === 'undefined') return;

  console.group('PWA Capabilities');
  console.log('Is PWA:', isPWA());
  console.log('Is iOS:', isIOS());
  console.log('Is Android:', isAndroid());
  console.log('Display Mode:', getDisplayMode());
  console.log('Supports Install Prompt:', supportsInstallPrompt());
  console.log('Supports Service Worker:', supportsServiceWorker());
  console.log('Supports Push Notifications:', supportsPushNotifications());
  console.log('Supports IndexedDB:', supportsIndexedDB());
  console.log('Connection Type:', getConnectionType());
  console.log('Device Pixel Ratio:', getDevicePixelRatio());
  console.groupEnd();
}
