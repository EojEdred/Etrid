/**
 * Firebase Service for React Native
 *
 * Handles Firebase integration for iOS and Android:
 * - Push Notifications (FCM)
 * - Analytics
 * - Crashlytics
 */

import messaging, {
  FirebaseMessagingTypes,
} from '@react-native-firebase/messaging';
import analytics from '@react-native-firebase/analytics';
import crashlytics from '@react-native-firebase/crashlytics';
import { Platform, Alert } from 'react-native';

/**
 * Notification handler callback type
 */
export type NotificationHandler = (
  notification: FirebaseMessagingTypes.RemoteMessage
) => void;

/**
 * Firebase Service Class
 */
export class FirebaseService {
  private fcmToken: string | null = null;
  private foregroundHandler: NotificationHandler | null = null;
  private backgroundHandler: NotificationHandler | null = null;

  /**
   * Initialize Firebase services
   * Call this on app startup
   */
  async initialize(): Promise<string | null> {
    try {
      console.log('Firebase: Initializing...');

      // Request notification permission
      const token = await this.requestPermission();

      if (token) {
        this.fcmToken = token;
        console.log('Firebase: Initialized with token:', token);

        // Set up notification listeners
        this.setupNotificationListeners();

        // Enable Crashlytics collection
        await crashlytics().setCrashlyticsCollectionEnabled(true);
        console.log('Firebase: Crashlytics enabled');

        return token;
      } else {
        console.warn('Firebase: Failed to get FCM token');
        return null;
      }
    } catch (error) {
      console.error('Firebase: Initialization error:', error);
      this.reportError(error as Error);
      return null;
    }
  }

  /**
   * Request notification permission
   */
  async requestPermission(): Promise<string | null> {
    try {
      const authStatus = await messaging().requestPermission();

      const enabled =
        authStatus === messaging.AuthorizationStatus.AUTHORIZED ||
        authStatus === messaging.AuthorizationStatus.PROVISIONAL;

      if (enabled) {
        console.log('Firebase: Notification permission granted');

        // Get FCM token
        const token = await messaging().getToken();
        console.log('Firebase: FCM Token:', token);

        return token;
      } else {
        console.log('Firebase: Notification permission denied');
        return null;
      }
    } catch (error) {
      console.error('Firebase: Error requesting permission:', error);
      return null;
    }
  }

  /**
   * Get current FCM token
   */
  getToken(): string | null {
    return this.fcmToken;
  }

  /**
   * Refresh FCM token
   */
  async refreshToken(): Promise<string | null> {
    try {
      const token = await messaging().getToken();
      this.fcmToken = token;
      console.log('Firebase: Token refreshed:', token);
      return token;
    } catch (error) {
      console.error('Firebase: Error refreshing token:', error);
      return null;
    }
  }

  /**
   * Delete FCM token (for logout)
   */
  async deleteToken(): Promise<boolean> {
    try {
      await messaging().deleteToken();
      this.fcmToken = null;
      console.log('Firebase: Token deleted');
      return true;
    } catch (error) {
      console.error('Firebase: Error deleting token:', error);
      return false;
    }
  }

  /**
   * Set up notification listeners
   */
  setupNotificationListeners(): void {
    // Foreground messages
    messaging().onMessage(async (remoteMessage) => {
      console.log('Firebase: Foreground message received:', remoteMessage);

      // Show local notification
      this.showLocalNotification(remoteMessage);

      // Call custom handler
      if (this.foregroundHandler) {
        this.foregroundHandler(remoteMessage);
      }

      // Log analytics
      this.logNotificationReceived(remoteMessage);
    });

    // Background message handler (already set in index.js, but kept here for reference)
    messaging().setBackgroundMessageHandler(async (remoteMessage) => {
      console.log('Firebase: Background message received:', remoteMessage);

      // Call custom handler
      if (this.backgroundHandler) {
        this.backgroundHandler(remoteMessage);
      }

      // Log analytics
      this.logNotificationReceived(remoteMessage);
    });

    // Notification opened app (from quit state)
    messaging().onNotificationOpenedApp((remoteMessage) => {
      console.log('Firebase: Notification opened app:', remoteMessage);
      this.handleNotificationOpen(remoteMessage);
    });

    // Check if app was opened by notification
    messaging()
      .getInitialNotification()
      .then((remoteMessage) => {
        if (remoteMessage) {
          console.log('Firebase: App opened by notification:', remoteMessage);
          this.handleNotificationOpen(remoteMessage);
        }
      });

    // Token refresh listener
    messaging().onTokenRefresh((token) => {
      console.log('Firebase: Token refreshed:', token);
      this.fcmToken = token;
      // TODO: Send new token to your backend
    });

    console.log('Firebase: Notification listeners set up');
  }

  /**
   * Set foreground notification handler
   */
  setForegroundHandler(handler: NotificationHandler): void {
    this.foregroundHandler = handler;
  }

  /**
   * Set background notification handler
   */
  setBackgroundHandler(handler: NotificationHandler): void {
    this.backgroundHandler = handler;
  }

  /**
   * Show local notification (for foreground messages)
   */
  private showLocalNotification(
    remoteMessage: FirebaseMessagingTypes.RemoteMessage
  ): void {
    const title = remoteMessage.notification?.title || 'Ëtrid Wallet';
    const body = remoteMessage.notification?.body || 'You have a new notification';

    Alert.alert(title, body, [
      {
        text: 'Dismiss',
        style: 'cancel',
      },
      {
        text: 'View',
        onPress: () => this.handleNotificationOpen(remoteMessage),
      },
    ]);
  }

  /**
   * Handle notification open (navigation)
   */
  private handleNotificationOpen(
    remoteMessage: FirebaseMessagingTypes.RemoteMessage
  ): void {
    // TODO: Implement navigation based on notification data
    const data = remoteMessage.data;

    console.log('Firebase: Handling notification open with data:', data);

    // Log analytics
    this.logNotificationOpened(remoteMessage);

    // Example navigation logic:
    // if (data?.screen) {
    //   navigation.navigate(data.screen, data.params);
    // }
  }

  // ============= Analytics Methods =============

  /**
   * Log analytics event
   */
  async logEvent(eventName: string, params?: Record<string, any>): Promise<void> {
    try {
      await analytics().logEvent(eventName, params);
      console.log('Firebase Analytics: Event logged:', eventName, params);
    } catch (error) {
      console.error('Firebase Analytics: Error logging event:', error);
    }
  }

  /**
   * Log screen view
   */
  async logScreenView(screenName: string, screenClass?: string): Promise<void> {
    try {
      await analytics().logScreenView({
        screen_name: screenName,
        screen_class: screenClass || screenName,
      });
      console.log('Firebase Analytics: Screen view logged:', screenName);
    } catch (error) {
      console.error('Firebase Analytics: Error logging screen view:', error);
    }
  }

  /**
   * Set user ID for analytics
   */
  async setUserId(userId: string): Promise<void> {
    try {
      await analytics().setUserId(userId);
      await crashlytics().setUserId(userId);
      console.log('Firebase: User ID set:', userId);
    } catch (error) {
      console.error('Firebase: Error setting user ID:', error);
    }
  }

  /**
   * Set user properties
   */
  async setUserProperties(properties: Record<string, any>): Promise<void> {
    try {
      await analytics().setUserProperties(properties);
      console.log('Firebase Analytics: User properties set:', properties);
    } catch (error) {
      console.error('Firebase Analytics: Error setting user properties:', error);
    }
  }

  /**
   * Log notification received event
   */
  private async logNotificationReceived(
    notification: FirebaseMessagingTypes.RemoteMessage
  ): Promise<void> {
    await this.logEvent('notification_received', {
      notification_id: notification.messageId,
      notification_type: notification.data?.type || 'general',
      platform: Platform.OS,
    });
  }

  /**
   * Log notification opened event
   */
  private async logNotificationOpened(
    notification: FirebaseMessagingTypes.RemoteMessage
  ): Promise<void> {
    await this.logEvent('notification_opened', {
      notification_id: notification.messageId,
      notification_type: notification.data?.type || 'general',
      platform: Platform.OS,
    });
  }

  // ============= Crashlytics Methods =============

  /**
   * Report error to Crashlytics
   */
  reportError(error: Error): void {
    try {
      crashlytics().recordError(error);
      console.log('Firebase Crashlytics: Error reported:', error.message);
    } catch (err) {
      console.error('Firebase Crashlytics: Error reporting error:', err);
    }
  }

  /**
   * Log message to Crashlytics
   */
  log(message: string): void {
    try {
      crashlytics().log(message);
    } catch (error) {
      console.error('Firebase Crashlytics: Error logging message:', error);
    }
  }

  /**
   * Set custom attribute for crash reports
   */
  async setAttribute(key: string, value: string): Promise<void> {
    try {
      await crashlytics().setAttribute(key, value);
    } catch (error) {
      console.error('Firebase Crashlytics: Error setting attribute:', error);
    }
  }

  /**
   * Set multiple custom attributes
   */
  async setAttributes(attributes: Record<string, string>): Promise<void> {
    try {
      await crashlytics().setAttributes(attributes);
    } catch (error) {
      console.error('Firebase Crashlytics: Error setting attributes:', error);
    }
  }

  /**
   * Force a crash (testing only!)
   */
  forceCrash(): void {
    if (__DEV__) {
      console.warn('Firebase Crashlytics: Forcing crash for testing');
      crashlytics().crash();
    } else {
      console.warn('Firebase Crashlytics: forceCrash() should only be used in development');
    }
  }

  /**
   * Test crash reporting (development only)
   */
  async testCrashReporting(): Promise<void> {
    if (!__DEV__) {
      console.warn('Test crash reporting should only be used in development');
      return;
    }

    console.log('Firebase Crashlytics: Testing error reporting...');

    // Test non-fatal error
    this.reportError(new Error('Test non-fatal error'));

    // Log some breadcrumbs
    this.log('User clicked test button');
    this.log('Navigation to test screen');

    // Set custom attributes
    await this.setAttribute('test_attribute', 'test_value');

    console.log('Firebase Crashlytics: Test complete');
  }

  // ============= Utility Methods =============

  /**
   * Check if device supports notifications
   */
  async isNotificationSupported(): Promise<boolean> {
    try {
      const enabled = await messaging().isDeviceRegisteredForRemoteMessages();
      return enabled;
    } catch (error) {
      return false;
    }
  }

  /**
   * Get notification permission status
   */
  async getPermissionStatus(): Promise<number> {
    try {
      const authStatus = await messaging().hasPermission();
      return authStatus;
    } catch (error) {
      return messaging().AuthorizationStatus.NOT_DETERMINED;
    }
  }

  /**
   * Subscribe to topic
   */
  async subscribeToTopic(topic: string): Promise<void> {
    try {
      await messaging().subscribeToTopic(topic);
      console.log(`Firebase: Subscribed to topic: ${topic}`);
    } catch (error) {
      console.error(`Firebase: Error subscribing to topic ${topic}:`, error);
    }
  }

  /**
   * Unsubscribe from topic
   */
  async unsubscribeFromTopic(topic: string): Promise<void> {
    try {
      await messaging().unsubscribeFromTopic(topic);
      console.log(`Firebase: Unsubscribed from topic: ${topic}`);
    } catch (error) {
      console.error(`Firebase: Error unsubscribing from topic ${topic}:`, error);
    }
  }

  /**
   * Get badge count (iOS only)
   */
  async getBadgeCount(): Promise<number> {
    if (Platform.OS === 'ios') {
      try {
        const count = await messaging().getAPNSToken();
        return count ? 0 : 0; // iOS doesn't expose badge count directly
      } catch (error) {
        return 0;
      }
    }
    return 0;
  }

  /**
   * Set badge count (iOS only)
   */
  async setBadgeCount(count: number): Promise<void> {
    if (Platform.OS === 'ios') {
      try {
        // Note: This requires a native module or library like react-native-notification-badge
        console.log(`Firebase: Setting badge count to ${count} (requires native implementation)`);
      } catch (error) {
        console.error('Firebase: Error setting badge count:', error);
      }
    }
  }

  /**
   * Clear all notifications (Android only)
   */
  clearAllNotifications(): void {
    if (Platform.OS === 'android') {
      try {
        // Note: This requires a native module or library
        console.log('Firebase: Clearing all notifications (requires native implementation)');
      } catch (error) {
        console.error('Firebase: Error clearing notifications:', error);
      }
    }
  }
}

// Export singleton instance
export const firebaseService = new FirebaseService();

// Export for manual instantiation if needed
export default FirebaseService;
