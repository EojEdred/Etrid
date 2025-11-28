/**
 * Firebase Configuration
 *
 * Initializes Firebase services for the Ëtrid Wallet PWA:
 * - Cloud Messaging (Push Notifications)
 * - Analytics
 * - Performance Monitoring
 */

import { initializeApp, getApps, FirebaseApp, FirebaseOptions } from 'firebase/app';
import { getMessaging, getToken, onMessage, Messaging, isSupported as isMessagingSupported } from 'firebase/messaging';
import { getAnalytics, Analytics, isSupported as isAnalyticsSupported } from 'firebase/analytics';
import { getPerformance, FirebasePerformance } from 'firebase/performance';

// Firebase configuration from environment variables
const firebaseConfig: FirebaseOptions = {
  apiKey: process.env.NEXT_PUBLIC_FIREBASE_API_KEY,
  authDomain: process.env.NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN,
  projectId: process.env.NEXT_PUBLIC_FIREBASE_PROJECT_ID,
  storageBucket: process.env.NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: process.env.NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
  appId: process.env.NEXT_PUBLIC_FIREBASE_APP_ID,
  measurementId: process.env.NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID,
};

// Validate configuration
function validateConfig(): boolean {
  const requiredKeys: (keyof FirebaseOptions)[] = [
    'apiKey',
    'authDomain',
    'projectId',
    'storageBucket',
    'messagingSenderId',
    'appId',
  ];

  for (const key of requiredKeys) {
    if (!firebaseConfig[key]) {
      console.warn(`Firebase: Missing required config key: ${key}`);
      return false;
    }
  }

  return true;
}

// Global instances
let app: FirebaseApp | null = null;
let messaging: Messaging | null = null;
let analytics: Analytics | null = null;
let performance: FirebasePerformance | null = null;

/**
 * Initialize Firebase
 * Only runs in browser environment
 */
export async function initializeFirebase(): Promise<void> {
  // Only run in browser
  if (typeof window === 'undefined') {
    return;
  }

  // Check if already initialized
  if (getApps().length > 0) {
    console.log('Firebase: Already initialized');
    return;
  }

  // Validate configuration
  if (!validateConfig()) {
    console.error('Firebase: Invalid configuration. Please check your .env.local file.');
    return;
  }

  try {
    // Initialize Firebase app
    app = initializeApp(firebaseConfig);
    console.log('Firebase: App initialized');

    // Initialize Analytics (only in production)
    if (process.env.NODE_ENV === 'production' && await isAnalyticsSupported()) {
      analytics = getAnalytics(app);
      console.log('Firebase: Analytics initialized');
    }

    // Initialize Performance Monitoring (only in production)
    if (process.env.NODE_ENV === 'production') {
      performance = getPerformance(app);
      console.log('Firebase: Performance monitoring initialized');
    }

    // Initialize Messaging (if supported)
    if (await isMessagingSupported()) {
      messaging = getMessaging(app);
      console.log('Firebase: Messaging initialized');
    } else {
      console.warn('Firebase: Messaging not supported in this browser');
    }
  } catch (error) {
    console.error('Firebase: Initialization error:', error);
  }
}

/**
 * Get Firebase app instance
 */
export function getFirebaseApp(): FirebaseApp | null {
  return app;
}

/**
 * Get Messaging instance
 */
export function getFirebaseMessaging(): Messaging | null {
  return messaging;
}

/**
 * Get Analytics instance
 */
export function getFirebaseAnalytics(): Analytics | null {
  return analytics;
}

/**
 * Get Performance instance
 */
export function getFirebasePerformance(): FirebasePerformance | null {
  return performance;
}

/**
 * Request notification permission and get FCM token
 *
 * @returns FCM token or null if permission denied
 */
export async function requestNotificationPermission(): Promise<string | null> {
  if (!messaging) {
    console.warn('Firebase: Messaging not initialized');
    return null;
  }

  try {
    // Check if service worker is supported
    if (!('serviceWorker' in navigator)) {
      console.warn('Firebase: Service workers not supported');
      return null;
    }

    // Request permission
    const permission = await Notification.requestPermission();
    console.log('Firebase: Notification permission:', permission);

    if (permission === 'granted') {
      // Get FCM token
      const vapidKey = process.env.NEXT_PUBLIC_FIREBASE_VAPID_KEY;

      if (!vapidKey) {
        console.error('Firebase: VAPID key not configured');
        return null;
      }

      const token = await getToken(messaging, {
        vapidKey,
        serviceWorkerRegistration: await navigator.serviceWorker.ready,
      });

      if (token) {
        console.log('Firebase: FCM token obtained:', token);
        return token;
      } else {
        console.warn('Firebase: No FCM token available');
        return null;
      }
    } else if (permission === 'denied') {
      console.warn('Firebase: Notification permission denied by user');
      return null;
    } else {
      console.warn('Firebase: Notification permission dismissed');
      return null;
    }
  } catch (error) {
    console.error('Firebase: Error getting FCM token:', error);
    return null;
  }
}

/**
 * Listen for foreground messages
 *
 * @param callback Function to call when message received
 * @returns Unsubscribe function
 */
export function onForegroundMessage(
  callback: (payload: any) => void
): (() => void) | null {
  if (!messaging) {
    console.warn('Firebase: Messaging not initialized');
    return null;
  }

  try {
    const unsubscribe = onMessage(messaging, (payload) => {
      console.log('Firebase: Foreground message received:', payload);
      callback(payload);
    });

    return unsubscribe;
  } catch (error) {
    console.error('Firebase: Error setting up foreground message listener:', error);
    return null;
  }
}

/**
 * Register service worker for background messages
 */
export async function registerServiceWorker(): Promise<ServiceWorkerRegistration | null> {
  if (typeof window === 'undefined' || !('serviceWorker' in navigator)) {
    return null;
  }

  try {
    const registration = await navigator.serviceWorker.register(
      '/firebase-messaging-sw.js'
    );
    console.log('Firebase: Service worker registered:', registration);
    return registration;
  } catch (error) {
    console.error('Firebase: Service worker registration failed:', error);
    return null;
  }
}

/**
 * Check if notifications are supported
 */
export function isNotificationSupported(): boolean {
  return (
    typeof window !== 'undefined' &&
    'Notification' in window &&
    'serviceWorker' in navigator &&
    'PushManager' in window
  );
}

/**
 * Get current notification permission status
 */
export function getNotificationPermission(): NotificationPermission | null {
  if (typeof window === 'undefined' || !('Notification' in window)) {
    return null;
  }
  return Notification.permission;
}

/**
 * Check if Firebase is properly configured
 */
export function isFirebaseConfigured(): boolean {
  return validateConfig();
}

// Auto-initialize on import (browser only)
if (typeof window !== 'undefined') {
  initializeFirebase();
}

// Export instances for direct access (backwards compatibility)
export { app, messaging, analytics, performance };
