/**
 * Firebase Cloud Messaging Service Worker
 *
 * This service worker handles background push notifications for the Ëtrid Wallet PWA.
 * It must be placed in the /public directory and registered at the root level.
 *
 * IMPORTANT: Update the firebaseConfig object with your actual Firebase credentials.
 */

// Import Firebase scripts
importScripts('https://www.gstatic.com/firebasejs/10.7.1/firebase-app-compat.js');
importScripts('https://www.gstatic.com/firebasejs/10.7.1/firebase-messaging-compat.js');

// Initialize Firebase in service worker
// IMPORTANT: Replace these values with your actual Firebase config
firebase.initializeApp({
  apiKey: "YOUR_API_KEY",
  authDomain: "YOUR_PROJECT_ID.firebaseapp.com",
  projectId: "YOUR_PROJECT_ID",
  storageBucket: "YOUR_PROJECT_ID.appspot.com",
  messagingSenderId: "YOUR_MESSAGING_SENDER_ID",
  appId: "YOUR_APP_ID"
});

// Retrieve Firebase Messaging instance
const messaging = firebase.messaging();

// Handle background messages
messaging.onBackgroundMessage((payload) => {
  console.log('[firebase-messaging-sw.js] Background message received:', payload);

  // Extract notification data
  const notificationTitle = payload.notification?.title || 'Ëtrid Wallet';
  const notificationBody = payload.notification?.body || 'You have a new notification';
  const notificationImage = payload.notification?.image || '/icons/icon-192x192.png';
  const notificationIcon = payload.notification?.icon || '/icons/icon-192x192.png';
  const notificationBadge = '/icons/badge-72x72.png';

  // Notification options
  const notificationOptions = {
    body: notificationBody,
    icon: notificationIcon,
    badge: notificationBadge,
    image: notificationImage,
    tag: payload.data?.tag || 'default',
    data: {
      url: payload.data?.url || '/',
      ...payload.data,
    },
    requireInteraction: payload.data?.requireInteraction === 'true',
    actions: [
      {
        action: 'view',
        title: 'View',
        icon: '/icons/icon-view.png',
      },
      {
        action: 'dismiss',
        title: 'Dismiss',
        icon: '/icons/icon-dismiss.png',
      },
    ],
    vibrate: [200, 100, 200],
    silent: false,
  };

  // Show notification
  return self.registration.showNotification(notificationTitle, notificationOptions);
});

// Handle notification clicks
self.addEventListener('notificationclick', (event) => {
  console.log('[firebase-messaging-sw.js] Notification clicked:', event);

  // Close notification
  event.notification.close();

  // Get the URL to open
  const urlToOpen = event.notification.data?.url || '/';

  // Handle action buttons
  if (event.action === 'dismiss') {
    // Just close the notification (already done above)
    return;
  }

  // Default action (notification click or "view" action)
  event.waitUntil(
    clients.matchAll({ type: 'window', includeUncontrolled: true }).then((clientList) => {
      // Check if there's already a window open
      for (const client of clientList) {
        if (client.url === urlToOpen && 'focus' in client) {
          return client.focus();
        }
      }

      // If no window is open, open a new one
      if (clients.openWindow) {
        return clients.openWindow(urlToOpen);
      }
    })
  );
});

// Handle push events (alternative to onBackgroundMessage)
self.addEventListener('push', (event) => {
  console.log('[firebase-messaging-sw.js] Push event received:', event);

  if (!event.data) {
    console.log('[firebase-messaging-sw.js] Push event has no data');
    return;
  }

  try {
    const payload = event.data.json();
    console.log('[firebase-messaging-sw.js] Push payload:', payload);

    // This is typically handled by onBackgroundMessage above
    // Include this handler as a fallback
  } catch (error) {
    console.error('[firebase-messaging-sw.js] Error parsing push payload:', error);
  }
});

// Service worker activation
self.addEventListener('activate', (event) => {
  console.log('[firebase-messaging-sw.js] Service worker activated');
  event.waitUntil(clients.claim());
});

// Service worker installation
self.addEventListener('install', (event) => {
  console.log('[firebase-messaging-sw.js] Service worker installed');
  self.skipWaiting();
});

// Handle messages from the client
self.addEventListener('message', (event) => {
  console.log('[firebase-messaging-sw.js] Message from client:', event.data);

  if (event.data && event.data.type === 'GET_VERSION') {
    event.ports[0].postMessage({
      type: 'VERSION',
      version: '1.0.0',
    });
  }
});

console.log('[firebase-messaging-sw.js] Firebase Messaging Service Worker loaded');
