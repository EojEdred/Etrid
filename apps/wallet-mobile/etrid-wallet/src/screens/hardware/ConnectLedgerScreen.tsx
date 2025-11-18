/**
 * ConnectLedgerScreen - Bluetooth scanning and Ledger pairing
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React, { useEffect } from 'react';
import { View, StyleSheet, SafeAreaView, Alert } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useLedger } from '../../hooks/useLedger';
import BluetoothScanner from '../../components/hardware/BluetoothScanner';
import LedgerInstructions from '../../components/hardware/LedgerInstructions';

const ConnectLedgerScreen: React.FC = () => {
  const navigation = useNavigation();
  const { devices, isScanning, scanForDevices, connectDevice } = useLedger();

  useEffect(() => {
    scanForDevices();
  }, []);

  const handleDevicePress = async (device: any) => {
    try {
      await connectDevice(device.id);
      Alert.alert('Success', 'Ledger connected successfully!', [
        { text: 'OK', onPress: () => navigation.navigate('LedgerAccounts' as never) },
      ]);
    } catch (error) {
      Alert.alert('Error', 'Failed to connect to Ledger');
    }
  };

  return (
    <SafeAreaView style={styles.container}>
      <LedgerInstructions type="connect" />
      <BluetoothScanner
        isScanning={isScanning}
        devices={devices}
        onDevicePress={handleDevicePress}
        onScan={scanForDevices}
      />
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f5f5f5' },
});

export default ConnectLedgerScreen;
