# PWA Quick Command Reference

Quick reference for common PWA setup and testing commands.

---

## Icon & Asset Generation

### Generate All Icons (Automated)
```bash
# Install PWA asset generator
npm install -g pwa-asset-generator

# Generate all icons from logo
pwa-asset-generator path/to/logo.png public/icons \
  --icon-only \
  --padding "20%" \
  --background "#1a0033" \
  --favicon \
  --quality 90

# Generate with maskable support
pwa-asset-generator path/to/logo.png public/icons \
  --icon-only \
  --padding "20%" \
  --background "#1a0033" \
  --maskable \
  --favicon
```

### Generate Splash Screens
```bash
pwa-asset-generator path/to/logo.png public/splash \
  --splash-only \
  --background "#1a0033" \
  --opaque false \
  --quality 90
```

### Generate Everything at Once
```bash
pwa-asset-generator path/to/logo.png public \
  --icon-only \
  --splash-only \
  --padding "20%" \
  --background "#1a0033" \
  --quality 90
```

---

## VAPID Keys for Push Notifications

### Generate VAPID Keys
```bash
npx web-push generate-vapid-keys
```

### Output Example
```
=======================================
Public Key:
BEl62iUYgUiv...

Private Key:
bdSiGcM9b...
=======================================
```

### Add to Environment
```bash
# .env.local
echo "NEXT_PUBLIC_VAPID_PUBLIC_KEY=your_public_key" >> .env.local
echo "VAPID_PRIVATE_KEY=your_private_key" >> .env.local
```

---

## Development & Testing

### Build and Test Locally
```bash
# Development mode (SW disabled by default in dev)
npm run dev

# Production build (required for PWA testing)
npm run build
npm start
```

### Test with HTTPS Locally
```bash
# Option 1: Using serve with SSL
npx serve@latest out -p 3000 --ssl

# Option 2: Using http-server with SSL
npx http-server out -p 3000 --ssl

# Option 3: Using Next.js with custom server
# (requires custom server setup)
```

### Test Offline Mode
```bash
# In Chrome DevTools:
# 1. Open DevTools (F12)
# 2. Network tab
# 3. Toggle "Offline" checkbox
# 4. Reload page
```

---

## Service Worker Management

### Register Service Worker (Browser Console)
```javascript
navigator.serviceWorker.register('/sw.js')
  .then(reg => console.log('SW registered:', reg))
  .catch(err => console.error('SW registration failed:', err));
```

### Check Service Worker Status
```javascript
navigator.serviceWorker.ready.then(reg => {
  console.log('Service Worker is ready');
  console.log('Scope:', reg.scope);
  console.log('Active:', reg.active);
});
```

### Unregister Service Worker
```javascript
navigator.serviceWorker.getRegistrations()
  .then(registrations => {
    registrations.forEach(reg => reg.unregister());
  });
```

### Update Service Worker
```javascript
navigator.serviceWorker.ready.then(reg => {
  reg.update();
});
```

### Force Update and Reload
```javascript
navigator.serviceWorker.ready.then(reg => {
  reg.update().then(() => {
    if (reg.waiting) {
      reg.waiting.postMessage({ type: 'SKIP_WAITING' });
      window.location.reload();
    }
  });
});
```

---

## Cache Management

### List All Caches
```javascript
caches.keys().then(names => {
  console.log('Cache names:', names);
});
```

### Get Cache Size
```javascript
async function getCacheSize() {
  const cacheNames = await caches.keys();
  let totalSize = 0;

  for (const name of cacheNames) {
    const cache = await caches.open(name);
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

getCacheSize().then(size => {
  console.log('Total cache size:', (size / 1024 / 1024).toFixed(2), 'MB');
});
```

### Clear All Caches
```javascript
caches.keys().then(names => {
  names.forEach(name => {
    caches.delete(name);
    console.log('Deleted cache:', name);
  });
});
```

### Clear Specific Cache
```javascript
caches.delete('etrid-wallet-v1.0.0').then(() => {
  console.log('Cache cleared');
});
```

---

## IndexedDB Management

### Check IndexedDB Databases
```javascript
indexedDB.databases().then(dbs => {
  console.log('Databases:', dbs);
});
```

### Delete Database
```javascript
indexedDB.deleteDatabase('etrid-wallet-db');
```

### View Database Contents (Browser Console)
```javascript
import { indexedDBService } from '@/lib/db/IndexedDBService';

// Get all transactions
indexedDBService.getTransactions().then(console.log);

// Get all balances
indexedDBService.getAllBalances().then(console.log);

// Get pending operations
indexedDBService.getPendingTransactions().then(console.log);
```

---

## Push Notifications Testing

### Request Permission
```javascript
Notification.requestPermission().then(permission => {
  console.log('Permission:', permission);
});
```

### Show Test Notification
```javascript
navigator.serviceWorker.ready.then(reg => {
  reg.showNotification('Test Notification', {
    body: 'This is a test',
    icon: '/icons/icon-192x192.png',
    badge: '/icons/badge-72x72.png',
    vibrate: [200, 100, 200]
  });
});
```

### Subscribe to Push
```javascript
navigator.serviceWorker.ready.then(reg => {
  reg.pushManager.subscribe({
    userVisibleOnly: true,
    applicationServerKey: 'your-vapid-public-key'
  }).then(subscription => {
    console.log('Subscription:', JSON.stringify(subscription));
  });
});
```

### Get Current Subscription
```javascript
navigator.serviceWorker.ready.then(reg => {
  reg.pushManager.getSubscription().then(subscription => {
    console.log('Current subscription:', subscription);
  });
});
```

---

## Lighthouse PWA Audit

### Run Lighthouse (Chrome DevTools)
```bash
# 1. Open DevTools (F12)
# 2. Click "Lighthouse" tab
# 3. Select "Progressive Web App"
# 4. Click "Generate report"
```

### Run Lighthouse CLI
```bash
# Install Lighthouse
npm install -g lighthouse

# Run audit
lighthouse https://localhost:3000 --view

# Run with specific categories
lighthouse https://localhost:3000 --only-categories=pwa --view

# Save report
lighthouse https://localhost:3000 --output=html --output-path=./lighthouse-report.html
```

---

## Manifest Validation

### Validate Manifest Online
```bash
# Use PWA Builder
open https://www.pwabuilder.com/

# Use Web.dev Manifest Validator
open https://web.dev/add-manifest/
```

### Check Manifest in DevTools
```bash
# Chrome DevTools:
# Application → Manifest
# Check for errors and warnings
```

---

## Mobile Testing

### Android Remote Debugging
```bash
# 1. Enable USB debugging on Android device
# 2. Connect device via USB
# 3. Open Chrome on desktop
# 4. Navigate to: chrome://inspect
# 5. Click "Inspect" on your device
```

### iOS Safari Debugging
```bash
# 1. Enable Web Inspector on iOS:
#    Settings → Safari → Advanced → Web Inspector
# 2. Connect iPhone/iPad via USB
# 3. Open Safari on Mac
# 4. Develop → [Your Device] → [Your Page]
```

---

## Network Testing

### Check Connection Type
```javascript
const connection = navigator.connection || navigator.mozConnection || navigator.webkitConnection;
if (connection) {
  console.log('Type:', connection.effectiveType);
  console.log('Downlink:', connection.downlink);
  console.log('RTT:', connection.rtt);
}
```

### Simulate Slow Connection
```bash
# Chrome DevTools:
# 1. Open DevTools (F12)
# 2. Network tab
# 3. Throttling dropdown
# 4. Select "Slow 3G" or "Fast 3G"
```

---

## Storage Management

### Check Storage Quota
```javascript
navigator.storage.estimate().then(estimate => {
  console.log('Used:', estimate.usage / 1024 / 1024, 'MB');
  console.log('Quota:', estimate.quota / 1024 / 1024, 'MB');
  console.log('Percentage:', (estimate.usage / estimate.quota * 100).toFixed(2), '%');
});
```

### Request Persistent Storage
```javascript
navigator.storage.persist().then(granted => {
  console.log('Persistent storage granted:', granted);
});
```

### Check if Storage is Persistent
```javascript
navigator.storage.persisted().then(isPersisted => {
  console.log('Storage is persistent:', isPersisted);
});
```

---

## Deployment

### Vercel Deployment
```bash
# Install Vercel CLI
npm install -g vercel

# Deploy
vercel

# Deploy to production
vercel --prod
```

### Netlify Deployment
```bash
# Install Netlify CLI
npm install -g netlify-cli

# Build
npm run build

# Deploy
netlify deploy

# Deploy to production
netlify deploy --prod
```

---

## Debugging

### Enable Service Worker Debugging
```javascript
// Add to sw.js
self.addEventListener('install', (event) => {
  console.log('[SW] Install event');
});

self.addEventListener('activate', (event) => {
  console.log('[SW] Activate event');
});

self.addEventListener('fetch', (event) => {
  console.log('[SW] Fetch:', event.request.url);
});
```

### View Service Worker Logs
```bash
# Chrome DevTools:
# Application → Service Workers → Click "Inspect"
# Opens DevTools for Service Worker
```

### Check PWA Installation Status
```javascript
// Check if running as PWA
const isPWA = window.matchMedia('(display-mode: standalone)').matches;
console.log('Is PWA:', isPWA);

// Check if iOS standalone
const isIOSPWA = window.navigator.standalone === true;
console.log('Is iOS PWA:', isIOSPWA);
```

---

## Useful Browser URLs

### Chrome
```
chrome://serviceworker-internals/
chrome://inspect/#service-workers
chrome://flags/#enable-desktop-pwas
```

### Firefox
```
about:serviceworkers
about:debugging#/runtime/this-firefox
```

### Edge
```
edge://serviceworker-internals/
edge://inspect/#service-workers
```

---

## Performance Testing

### Test Cache Performance
```javascript
console.time('cache-fetch');
caches.match('/').then(() => {
  console.timeEnd('cache-fetch');
});
```

### Test Network vs Cache
```javascript
// Network request
console.time('network');
fetch('/api/data').then(() => console.timeEnd('network'));

// Cache request
console.time('cache');
caches.match('/api/data').then(() => console.timeEnd('cache'));
```

---

## Common Issues & Fixes

### Clear Everything and Start Fresh
```javascript
// Unregister service workers
navigator.serviceWorker.getRegistrations()
  .then(regs => regs.forEach(reg => reg.unregister()));

// Clear caches
caches.keys().then(names => names.forEach(name => caches.delete(name)));

// Clear IndexedDB
indexedDB.deleteDatabase('etrid-wallet-db');

// Clear local storage
localStorage.clear();
sessionStorage.clear();

// Reload
window.location.reload();
```

### Force Update Service Worker
```javascript
// Skip waiting and activate immediately
self.skipWaiting();
self.clients.claim();
```

---

## Production Checklist

### Pre-Deployment
```bash
# 1. Generate icons
pwa-asset-generator logo.png public/icons --icon-only

# 2. Generate splash screens
pwa-asset-generator logo.png public/splash --splash-only

# 3. Generate VAPID keys
npx web-push generate-vapid-keys

# 4. Build production
npm run build

# 5. Test locally
npm start

# 6. Run Lighthouse audit
lighthouse https://localhost:3000 --only-categories=pwa

# 7. Deploy
vercel --prod
```

---

**Quick Tip:** Bookmark this file for easy reference during development and debugging!

**Last Updated:** 2025-11-19
