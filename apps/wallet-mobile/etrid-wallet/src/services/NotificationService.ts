import * as Notifications from 'expo-notifications';
import { Platform } from 'react-native';

// Configure notification handler
Notifications.setNotificationHandler({
  handleNotification: async () => ({
    shouldShowAlert: true,
    shouldPlaySound: true,
    shouldSetBadge: true,
  }),
});

export interface NotificationPreferences {
  transactionConfirmed: boolean;
  proposalVotingEndsSoon: boolean;
  rewardsReceived: boolean;
  gpuRentalExpiring: boolean;
  bridgeCompleted: boolean;
  priceAlerts: boolean;
}

/**
 * Service for handling local and push notifications
 */
class NotificationService {
  private preferences: NotificationPreferences = {
    transactionConfirmed: true,
    proposalVotingEndsSoon: true,
    rewardsReceived: true,
    gpuRentalExpiring: true,
    bridgeCompleted: true,
    priceAlerts: true,
  };

  /**
   * Initialize notifications
   */
  public async initialize(): Promise<void> {
    try {
      const { status: existingStatus } = await Notifications.getPermissionsAsync();
      let finalStatus = existingStatus;

      if (existingStatus !== 'granted') {
        const { status } = await Notifications.requestPermissionsAsync();
        finalStatus = status;
      }

      if (finalStatus !== 'granted') {
        console.warn('Notification permission not granted');
        return;
      }

      if (Platform.OS === 'android') {
        await Notifications.setNotificationChannelAsync('default', {
          name: 'default',
          importance: Notifications.AndroidImportance.MAX,
          vibrationPattern: [0, 250, 250, 250],
          lightColor: '#FF231F7C',
        });
      }

      console.log('Notifications initialized');
    } catch (error) {
      console.error('Failed to initialize notifications:', error);
    }
  }

  /**
   * Set notification preferences
   */
  public setPreferences(preferences: Partial<NotificationPreferences>): void {
    this.preferences = { ...this.preferences, ...preferences };
  }

  /**
   * Get notification preferences
   */
  public getPreferences(): NotificationPreferences {
    return { ...this.preferences };
  }

  /**
   * Schedule transaction confirmed notification
   */
  public async scheduleTransactionConfirmed(
    txHash: string,
    type: 'send' | 'receive' | 'stake' | 'swap'
  ): Promise<void> {
    if (!this.preferences.transactionConfirmed) return;

    try {
      await Notifications.scheduleNotificationAsync({
        content: {
          title: '✅ Transaction Confirmed',
          body: `Your ${type} transaction has been confirmed on-chain`,
          data: { txHash, type },
          sound: 'default',
        },
        trigger: null, // Immediate
      });
    } catch (error) {
      console.error('Failed to schedule transaction notification:', error);
    }
  }

  /**
   * Schedule proposal voting ends soon notification
   */
  public async scheduleProposalEndingSoon(
    proposalId: number,
    title: string,
    hoursLeft: number
  ): Promise<void> {
    if (!this.preferences.proposalVotingEndsSoon) return;

    try {
      await Notifications.scheduleNotificationAsync({
        content: {
          title: '⏰ Proposal Voting Ends Soon',
          body: `"${title}" voting ends in ${hoursLeft} hours`,
          data: { proposalId, type: 'proposal_ending' },
          sound: 'default',
        },
        trigger: { seconds: 60 }, // 1 minute delay
      });
    } catch (error) {
      console.error('Failed to schedule proposal notification:', error);
    }
  }

  /**
   * Schedule rewards received notification
   */
  public async scheduleRewardsReceived(amount: string, asset: string = 'ÉTR'): Promise<void> {
    if (!this.preferences.rewardsReceived) return;

    try {
      await Notifications.scheduleNotificationAsync({
        content: {
          title: '💰 Staking Rewards Received',
          body: `You earned ${amount} ${asset} today!`,
          data: { amount, asset, type: 'rewards' },
          sound: 'default',
        },
        trigger: null, // Immediate
      });
    } catch (error) {
      console.error('Failed to schedule rewards notification:', error);
    }
  }

  /**
   * Schedule GPU rental expiring notification
   */
  public async scheduleGPURentalExpiring(
    rentalId: string,
    gpuModel: string,
    hoursLeft: number
  ): Promise<void> {
    if (!this.preferences.gpuRentalExpiring) return;

    try {
      await Notifications.scheduleNotificationAsync({
        content: {
          title: '⏰ GPU Rental Expiring',
          body: `Your ${gpuModel} rental expires in ${hoursLeft} hours`,
          data: { rentalId, gpuModel, type: 'gpu_expiring' },
          sound: 'default',
        },
        trigger: { seconds: hoursLeft * 3600 - 3600 }, // 1 hour before expiry
      });
    } catch (error) {
      console.error('Failed to schedule GPU rental notification:', error);
    }
  }

  /**
   * Schedule bridge completed notification
   */
  public async scheduleBridgeCompleted(
    txId: string,
    amount: string,
    direction: 'to_fabric' | 'from_fabric'
  ): Promise<void> {
    if (!this.preferences.bridgeCompleted) return;

    try {
      const directionText = direction === 'to_fabric' ? 'to Fabric' : 'from Fabric';

      await Notifications.scheduleNotificationAsync({
        content: {
          title: '🌉 Bridge Transfer Complete',
          body: `${amount} ËDSC successfully bridged ${directionText}`,
          data: { txId, amount, direction, type: 'bridge_complete' },
          sound: 'default',
        },
        trigger: null, // Immediate
      });
    } catch (error) {
      console.error('Failed to schedule bridge notification:', error);
    }
  }

  /**
   * Schedule price alert notification
   */
  public async schedulePriceAlert(
    asset: string,
    currentPrice: number,
    targetPrice: number,
    condition: 'above' | 'below'
  ): Promise<void> {
    if (!this.preferences.priceAlerts) return;

    try {
      const conditionText = condition === 'above' ? 'risen above' : 'fallen below';

      await Notifications.scheduleNotificationAsync({
        content: {
          title: `📈 Price Alert: ${asset}`,
          body: `${asset} has ${conditionText} $${targetPrice} (now $${currentPrice})`,
          data: { asset, currentPrice, targetPrice, condition, type: 'price_alert' },
          sound: 'default',
        },
        trigger: null, // Immediate
      });
    } catch (error) {
      console.error('Failed to schedule price alert:', error);
    }
  }

  /**
   * Schedule generic notification
   */
  public async scheduleNotification(
    title: string,
    body: string,
    data?: Record<string, any>,
    triggerSeconds?: number
  ): Promise<void> {
    try {
      await Notifications.scheduleNotificationAsync({
        content: {
          title,
          body,
          data: data || {},
          sound: 'default',
        },
        trigger: triggerSeconds ? { seconds: triggerSeconds } : null,
      });
    } catch (error) {
      console.error('Failed to schedule notification:', error);
    }
  }

  /**
   * Cancel all scheduled notifications
   */
  public async cancelAllNotifications(): Promise<void> {
    try {
      await Notifications.cancelAllScheduledNotificationsAsync();
      console.log('All notifications cancelled');
    } catch (error) {
      console.error('Failed to cancel notifications:', error);
    }
  }

  /**
   * Get notification permission status
   */
  public async getPermissionStatus(): Promise<string> {
    try {
      const { status } = await Notifications.getPermissionsAsync();
      return status;
    } catch (error) {
      console.error('Failed to get permission status:', error);
      return 'undetermined';
    }
  }

  /**
   * Request notification permissions
   */
  public async requestPermissions(): Promise<boolean> {
    try {
      const { status } = await Notifications.requestPermissionsAsync();
      return status === 'granted';
    } catch (error) {
      console.error('Failed to request permissions:', error);
      return false;
    }
  }

  /**
   * Handle notification response (when user taps notification)
   */
  public addNotificationResponseListener(
    handler: (response: Notifications.NotificationResponse) => void
  ): Notifications.Subscription {
    return Notifications.addNotificationResponseReceivedListener(handler);
  }

  /**
   * Handle notification received (when app is in foreground)
   */
  public addNotificationReceivedListener(
    handler: (notification: Notifications.Notification) => void
  ): Notifications.Subscription {
    return Notifications.addNotificationReceivedListener(handler);
  }
}

export default new NotificationService();
