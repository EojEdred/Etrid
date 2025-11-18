import { useEffect, useState } from 'react';
import * as Notifications from 'expo-notifications';
import NotificationService, { NotificationPreferences } from '../services/NotificationService';

/**
 * Hook for handling notifications
 */
export function useNotifications() {
  const [preferences, setPreferences] = useState<NotificationPreferences>(
    NotificationService.getPreferences()
  );
  const [permissionStatus, setPermissionStatus] = useState<string>('undetermined');

  /**
   * Initialize notifications
   */
  useEffect(() => {
    const init = async () => {
      await NotificationService.initialize();
      const status = await NotificationService.getPermissionStatus();
      setPermissionStatus(status);
    };

    init();
  }, []);

  /**
   * Request notification permissions
   */
  const requestPermissions = async (): Promise<boolean> => {
    const granted = await NotificationService.requestPermissions();
    if (granted) {
      setPermissionStatus('granted');
    }
    return granted;
  };

  /**
   * Update preferences
   */
  const updatePreferences = (newPreferences: Partial<NotificationPreferences>) => {
    NotificationService.setPreferences(newPreferences);
    setPreferences(NotificationService.getPreferences());
  };

  /**
   * Schedule transaction confirmed notification
   */
  const notifyTransactionConfirmed = async (
    txHash: string,
    type: 'send' | 'receive' | 'stake' | 'swap'
  ) => {
    await NotificationService.scheduleTransactionConfirmed(txHash, type);
  };

  /**
   * Schedule proposal ending soon notification
   */
  const notifyProposalEndingSoon = async (
    proposalId: number,
    title: string,
    hoursLeft: number
  ) => {
    await NotificationService.scheduleProposalEndingSoon(proposalId, title, hoursLeft);
  };

  /**
   * Schedule rewards received notification
   */
  const notifyRewardsReceived = async (amount: string, asset: string = 'ÉTR') => {
    await NotificationService.scheduleRewardsReceived(amount, asset);
  };

  /**
   * Schedule GPU rental expiring notification
   */
  const notifyGPURentalExpiring = async (
    rentalId: string,
    gpuModel: string,
    hoursLeft: number
  ) => {
    await NotificationService.scheduleGPURentalExpiring(rentalId, gpuModel, hoursLeft);
  };

  /**
   * Schedule bridge completed notification
   */
  const notifyBridgeCompleted = async (
    txId: string,
    amount: string,
    direction: 'to_fabric' | 'from_fabric'
  ) => {
    await NotificationService.scheduleBridgeCompleted(txId, amount, direction);
  };

  /**
   * Schedule price alert notification
   */
  const notifyPriceAlert = async (
    asset: string,
    currentPrice: number,
    targetPrice: number,
    condition: 'above' | 'below'
  ) => {
    await NotificationService.schedulePriceAlert(asset, currentPrice, targetPrice, condition);
  };

  /**
   * Handle notification response (when user taps notification)
   */
  const onNotificationResponse = (
    handler: (response: Notifications.NotificationResponse) => void
  ): Notifications.Subscription => {
    return NotificationService.addNotificationResponseListener(handler);
  };

  /**
   * Handle notification received (when app is in foreground)
   */
  const onNotificationReceived = (
    handler: (notification: Notifications.Notification) => void
  ): Notifications.Subscription => {
    return NotificationService.addNotificationReceivedListener(handler);
  };

  return {
    preferences,
    permissionStatus,
    requestPermissions,
    updatePreferences,
    notifyTransactionConfirmed,
    notifyProposalEndingSoon,
    notifyRewardsReceived,
    notifyGPURentalExpiring,
    notifyBridgeCompleted,
    notifyPriceAlert,
    onNotificationResponse,
    onNotificationReceived,
  };
}
