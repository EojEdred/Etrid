import React, {useEffect} from 'react';
import {StatusBar} from 'react-native';
import RootNavigator from './navigation/RootNavigator';
import {pushNotificationService} from './services/PushNotificationService';
import {biometricService} from './services/BiometricService';

export default function App() {
  useEffect(() => {
    // Initialize services
    initializeServices();
  }, []);

  const initializeServices = async () => {
    try {
      // Initialize push notifications
      await pushNotificationService.initialize();
      console.log('[App] Push notifications initialized');

      // Check biometric availability
      const {available, biometryType} =
        await biometricService.isBiometricAvailable();
      console.log(
        `[App] Biometrics available: ${available}, type: ${biometryType}`,
      );
    } catch (error) {
      console.error('[App] Service initialization error:', error);
    }
  };

  return (
    <>
      <StatusBar barStyle="light-content" backgroundColor="#1a0033" />
      <RootNavigator />
    </>
  );
}
