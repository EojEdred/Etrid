# Progressive Web App (PWA) Setup - Ëtrid Wallet

Complete PWA implementation with offline support, installability, and native-like features.

## Overview

The Ëtrid Wallet is now a fully-featured Progressive Web App with:
- ✅ Offline support with intelligent caching
- ✅ Installable on mobile and desktop
- ✅ Push notifications
- ✅ Background sync
- ✅ Network detection
- ✅ IndexedDB for offline data storage
- ✅ App shortcuts
- ✅ Protocol handlers

## File Structure

```
apps/wallet-mobile/etrid-wallet/
├── app/
│   ├── layout.tsx                 # Updated with PWA meta tags
│   ├── sw-register.tsx            # Service worker registration
│   └── offline/
│       └── page.tsx               # Offline fallback page
├── components/
│   └── pwa/
│       ├── InstallPrompt.tsx      # Install prompt UI
│       └── OnlineStatus.tsx       # Network status indicator
├── lib/
│   ├── db/
│   │   └── IndexedDBService.ts    # Offline data storage
│   └── notifications/
│       └── PushService.ts         # Push notification service
├── hooks/
│   └── usePWA.ts                  # PWA React hooks
├── public/
│   ├── manifest.json              # Web App Manifest
│   ├── sw.js                      # Service Worker
│   ├── icons/                     # App icons (to be added)
│   ├── splash/                    # Splash screens (to be added)
│   └── screenshots/               # App screenshots (to be added)
└── next.config.mjs                # Updated with PWA config
```

## Setup Instructions

### 1. Generate App Icons

You need to create app icons in various sizes. Use a tool like [PWA Asset Generator](https://github.com/elegantapp/pwa-asset-generator) or create them manually:

**Required icon sizes:**
- 72x72, 96x96, 128x128, 144x144, 152x152, 192x192, 384x384, 512x512

**Command to generate icons (using pwa-asset-generator):**
```bash
npx pwa-asset-generator logo.png public/icons --icon-only --padding "20%" --background "#1a0033"
```

**Additional icons needed:**
- `apple-touch-icon.png` (180x180)
- `badge-72x72.png` (72x72 - for notification badge)
- `send-96x96.png` (96x96 - for shortcuts)
- `scan-96x96.png` (96x96 - for shortcuts)
- `card-96x96.png` (96x96 - for shortcuts)

### 2. Generate Splash Screens

Create splash screens for iOS devices:

```bash
npx pwa-asset-generator logo.png public/splash --splash-only --background "#1a0033" --splash-color "#8b5cf6"
```

### 3. Add Screenshots

Take screenshots of your app and add them to `public/screenshots/`:
- `home.png` (1170x2532 - iPhone 13 Pro)
- `trading.png` (1170x2532 - iPhone 13 Pro)

These will be shown in the app store-like install prompt on supported browsers.

### 4. Environment Variables

Add to your `.env.local`:

```env
NEXT_PUBLIC_VAPID_PUBLIC_KEY=your_vapid_public_key_here
VAPID_PRIVATE_KEY=your_vapid_private_key_here
```

**Generate VAPID keys:**
```bash
npx web-push generate-vapid-keys
```

### 5. Test Locally

```bash
npm run build
npm start
```

Then:
1. Open Chrome DevTools → Application → Manifest
2. Check the manifest is loaded correctly
3. Test service worker registration
4. Test offline mode (DevTools → Network → Offline)

### 6. Deploy Checklist

- [ ] All icons generated and placed in `/public/icons/`
- [ ] Splash screens generated and placed in `/public/splash/`
- [ ] Screenshots added to `/public/screenshots/`
- [ ] VAPID keys configured in environment variables
- [ ] HTTPS enabled (required for service workers)
- [ ] Tested installation on mobile devices
- [ ] Tested offline functionality
- [ ] Tested push notifications

## Features

### 1. Offline Support

The service worker implements multiple caching strategies:

**Cache-First (Static Assets):**
- Images, fonts, styles
- Cached for 30 days

**Network-First (API Calls):**
- API requests
- Falls back to cache if offline
- Cached for 5 minutes

**Network-Only (HTML Pages):**
- Navigation requests
- Falls back to `/offline` page if offline

### 2. Install Prompt

The app automatically shows an install prompt 30 seconds after the user visits. Features:
- Beautiful branded UI
- "Install" and "Later" options
- Only shows on supported browsers
- Respects user's choice

**Usage:**
```tsx
import InstallPrompt from '@/components/pwa/InstallPrompt';

// Already included in layout.tsx
```

### 3. Online/Offline Detection

Real-time network status indicator. Shows:
- "Back online!" when connection restored (3-second banner)
- "You're offline" when connection lost (persistent banner)

**Usage:**
```tsx
import OnlineStatus from '@/components/pwa/OnlineStatus';

// Already included in layout.tsx
```

### 4. IndexedDB Storage

Offline data persistence for:
- Transactions
- Balances
- Pending operations

**Usage:**
```typescript
import { indexedDBService } from '@/lib/db/IndexedDBService';

// Initialize
await indexedDBService.initialize();

// Save balance
await indexedDBService.saveBalance('ETH', { amount: 1.5, usd: 3000 });

// Get balance
const balance = await indexedDBService.getBalance('ETH');

// Save pending transaction
await indexedDBService.savePendingTransaction({
  to: '0x...',
  amount: 0.5,
  asset: 'ETH'
});

// Get pending transactions
const pending = await indexedDBService.getPendingTransactions();
```

### 5. Push Notifications

Full push notification support with VAPID.

**Usage:**
```typescript
import { pushService } from '@/lib/notifications/PushService';

// Request permission
const granted = await pushService.requestPermission();

// Subscribe to push
const subscription = await pushService.subscribeToPush();

// Send to backend
await pushService.sendSubscriptionToBackend(subscription);

// Show notification
await pushService.showNotification('Transaction Complete', {
  body: 'Your ETH transfer was successful',
  data: { url: '/transactions' }
});
```

### 6. PWA Hooks

React hooks for easy PWA integration:

**usePWA Hook:**
```typescript
import { usePWA } from '@/hooks/usePWA';

function MyComponent() {
  const {
    isInstalled,
    isOnline,
    canInstall,
    isUpdateAvailable,
    install,
    update
  } = usePWA();

  if (canInstall) {
    return <button onClick={install}>Install App</button>;
  }

  if (isUpdateAvailable) {
    return <button onClick={update}>Update Available</button>;
  }

  return <div>Status: {isOnline ? 'Online' : 'Offline'}</div>;
}
```

**useNetworkStatus Hook:**
```typescript
import { useNetworkStatus } from '@/hooks/usePWA';

function NetworkInfo() {
  const { isOnline, effectiveType } = useNetworkStatus();

  return (
    <div>
      Status: {isOnline ? 'Online' : 'Offline'}
      {effectiveType && <span>Connection: {effectiveType}</span>}
    </div>
  );
}
```

### 7. App Shortcuts

Quick actions from the home screen icon:
- **Send Money** → `/send`
- **Scan QR** → `/scan`
- **AU Bloccard** → `/card`

These appear when users long-press the app icon on mobile.

### 8. Protocol Handlers

Handle `web+etrid://` links:
```
web+etrid://send?address=0x1234...
```

This allows deep linking from other apps and websites.

### 9. Share Target

Users can share crypto addresses to the app:
```javascript
navigator.share({
  title: 'Crypto Address',
  text: '0x1234...',
  url: 'https://etrid.com/send'
});
```

## Testing

### Local Testing

```bash
# Build the app
npm run build

# Serve with HTTPS (required for PWA)
npm start

# Or use a local HTTPS server
npx serve@latest out -p 3000
```

### Chrome DevTools

1. **Application Tab:**
   - Check Manifest
   - Inspect Service Worker
   - View Cache Storage
   - Test IndexedDB

2. **Lighthouse:**
   - Run PWA audit
   - Aim for 100% PWA score

3. **Network Tab:**
   - Toggle offline mode
   - Test caching strategies

### Mobile Testing

**Android:**
1. Enable USB debugging
2. Use Chrome Remote Debugging
3. Test install prompt
4. Test offline mode
5. Test push notifications

**iOS:**
1. Use Safari Web Inspector
2. Test "Add to Home Screen"
3. Test offline mode
4. Note: iOS has limited PWA support

## Production Deployment

### Vercel / Netlify

1. Push to repository
2. Deploy automatically
3. Ensure HTTPS is enabled
4. Configure environment variables

### Custom Server

1. Serve with HTTPS
2. Configure correct MIME types:
   ```
   manifest.json → application/manifest+json
   sw.js → application/javascript
   ```
3. Set security headers:
   ```
   X-Content-Type-Options: nosniff
   Strict-Transport-Security: max-age=31536000
   ```

## Updating the Service Worker

When you make changes to the service worker:

1. **Update Version:**
   ```javascript
   // public/sw.js
   const CACHE_NAME = 'etrid-wallet-v1.0.1'; // Increment version
   ```

2. **Clear Old Caches:**
   The activation event automatically clears old caches

3. **Force Update:**
   ```javascript
   // In browser console
   navigator.serviceWorker.getRegistrations().then(registrations => {
     registrations.forEach(registration => registration.update());
   });
   ```

## Troubleshooting

### Service Worker Not Registering

- Check HTTPS is enabled
- Check browser console for errors
- Verify `/sw.js` is accessible
- Clear browser cache and reload

### Install Prompt Not Showing

- Requires HTTPS
- Must not be already installed
- Browser must support install prompt
- Check browser console for errors

### Push Notifications Not Working

- Verify VAPID keys are correct
- Check notification permission granted
- Ensure service worker is active
- Test on supported browsers (Chrome, Firefox)

### Offline Mode Not Working

- Check service worker is registered
- Verify cache is populated
- Test network offline in DevTools
- Check `/offline` page exists

## Browser Support

| Feature | Chrome | Firefox | Safari | Edge |
|---------|--------|---------|--------|------|
| Install Prompt | ✅ | ✅ | ⚠️ | ✅ |
| Service Worker | ✅ | ✅ | ✅ | ✅ |
| Push Notifications | ✅ | ✅ | ❌ | ✅ |
| Background Sync | ✅ | ❌ | ❌ | ✅ |
| App Shortcuts | ✅ | ❌ | ❌ | ✅ |

✅ = Fully Supported | ⚠️ = Partial Support | ❌ = Not Supported

## Resources

- [MDN PWA Guide](https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps)
- [web.dev PWA](https://web.dev/progressive-web-apps/)
- [PWA Builder](https://www.pwabuilder.com/)
- [Workbox (Advanced Caching)](https://developers.google.com/web/tools/workbox)

## Support

For issues or questions about the PWA implementation, check:
1. Browser console for errors
2. Chrome DevTools → Application tab
3. Network tab for failed requests
4. This documentation

---

**Last Updated:** 2025-11-19
**Version:** 1.0.0
