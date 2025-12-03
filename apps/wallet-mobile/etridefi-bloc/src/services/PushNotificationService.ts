import messaging from '@react-native-firebase/messaging';
import PushNotification from 'react-native-push-notification';
import {Platform} from 'react-native';

export class PushNotificationService {
  private fcmToken: string | null = null;

  /**
   * Initialize push notifications
   */
  async initialize(): Promise<string | null> {
    try {
      // Request permission
      const authStatus = await messaging().requestPermission();
      const enabled =
        authStatus === messaging.AuthorizationStatus.AUTHORIZED ||
        authStatus === messaging.AuthorizationStatus.PROVISIONAL;

      if (!enabled) {
        console.log('[PushNotification] Permission not granted');
        return null;
      }

      // Get FCM token
      this.fcmToken = await messaging().getToken();
      console.log('[PushNotification] FCM Token:', this.fcmToken);

      // Configure local notifications
      this.configurePushNotifications();

      // Set up notification handlers
      this.setupNotificationHandlers();

      return this.fcmToken;
    } catch (error) {
      console.error('[PushNotification] Initialization error:', error);
      return null;
    }
  }

  /**
   * Configure local push notifications
   */
  private configurePushNotifications() {
    PushNotification.configure({
      onRegister: token => {
        console.log('[PushNotification] Device Token:', token);
      },
      onNotification: notification => {
        console.log('[PushNotification] Notification received:', notification);
        notification.finish(PushNotification.FetchResult.NoData);
      },
      permissions: {
        alert: true,
        badge: true,
        sound: true,
      },
      popInitialNotification: true,
      requestPermissions: Platform.OS === 'ios',
    });

    // Create default channel for Android
    if (Platform.OS === 'android') {
      PushNotification.createChannel(
        {
          channelId: 'etrid-wallet-default',
          channelName: 'Ëtrid Wallet Notifications',
          channelDescription: 'Default notification channel',
          playSound: true,
          soundName: 'default',
          importance: 4,
          vibrate: true,
        },
        created => console.log(`[PushNotification] Channel created: ${created}`),
      );
    }
  }

  /**
   * Set up notification handlers
   */
  private setupNotificationHandlers() {
    // Handle foreground messages
    messaging().onMessage(async remoteMessage => {
      console.log('[PushNotification] Foreground message:', remoteMessage);

      // Show local notification
      PushNotification.localNotification({
        channelId: 'etrid-wallet-default',
        title: remoteMessage.notification?.title || 'New notification',
        message: remoteMessage.notification?.body || '',
        playSound: true,
        soundName: 'default',
      });
    });

    // Handle background messages
    messaging().setBackgroundMessageHandler(async remoteMessage => {
      console.log('[PushNotification] Background message:', remoteMessage);
    });

    // Handle notification opened app
    messaging().onNotificationOpenedApp(remoteMessage => {
      console.log('[PushNotification] Notification opened app:', remoteMessage);
    });

    // Handle app opened from quit state
    messaging()
      .getInitialNotification()
      .then(remoteMessage => {
        if (remoteMessage) {
          console.log('[PushNotification] App opened from quit state:', remoteMessage);
        }
      });
  }

  /**
   * Send local notification
   */
  sendLocalNotification(title: string, message: string, data?: any) {
    PushNotification.localNotification({
      channelId: 'etrid-wallet-default',
      title,
      message,
      playSound: true,
      soundName: 'default',
      userInfo: data,
    });
  }

  /**
   * Schedule local notification
   */
  scheduleNotification(title: string, message: string, date: Date, data?: any) {
    PushNotification.localNotificationSchedule({
      channelId: 'etrid-wallet-default',
      title,
      message,
      date,
      playSound: true,
      soundName: 'default',
      userInfo: data,
    });
  }

  /**
   * Cancel all notifications
   */
  cancelAllNotifications() {
    PushNotification.cancelAllLocalNotifications();
  }

  /**
   * Get FCM token
   */
  getToken(): string | null {
    return this.fcmToken;
  }
}

// Export singleton instance
export const pushNotificationService = new PushNotificationService();
