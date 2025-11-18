/**
 * ConnectedDevicesScreen - Manage connected hardware wallets
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity, FlatList, SafeAreaView } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useLedger } from '../../hooks/useLedger';
import DeviceCard from '../../components/hardware/DeviceCard';
import { HardwareWallet } from '../../types/hardware.types';

const ConnectedDevicesScreen: React.FC = () => {
  const navigation = useNavigation();
  const { connectedDevice, disconnect } = useLedger();

  const mockDevices: HardwareWallet[] = connectedDevice
    ? [
        {
          id: connectedDevice.id,
          type: 'ledger',
          model: 'Nano X',
          name: connectedDevice.name || 'Ledger Nano X',
          firmware: '2.1.0',
          connected: true,
          batteryLevel: connectedDevice.batteryLevel || 85,
          accounts: [],
        },
      ]
    : [];

  const handleManage = (device: HardwareWallet) => {
    navigation.navigate('LedgerAccounts' as never, { device } as never);
  };

  const handleDisconnect = async (device: HardwareWallet) => {
    await disconnect();
  };

  const handleConnect = () => {
    navigation.navigate('ConnectLedger' as never);
  };

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Hardware Wallets</Text>
        <Text style={styles.subtitle}>{mockDevices.length} device(s) connected</Text>
      </View>

      {mockDevices.length > 0 ? (
        <FlatList
          data={mockDevices}
          keyExtractor={(item) => item.id}
          renderItem={({ item }) => (
            <DeviceCard device={item} onManage={handleManage} onDisconnect={handleDisconnect} />
          )}
        />
      ) : (
        <View style={styles.emptyContainer}>
          <Text style={styles.emptyIcon}>🔐</Text>
          <Text style={styles.emptyTitle}>No Devices Connected</Text>
          <Text style={styles.emptyText}>Connect your Ledger to get started</Text>
        </View>
      )}

      <TouchableOpacity style={styles.connectButton} onPress={handleConnect}>
        <Text style={styles.connectButtonText}>+ Connect New Device</Text>
      </TouchableOpacity>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f5f5f5' },
  header: { padding: 20, backgroundColor: '#fff' },
  title: { fontSize: 28, fontWeight: 'bold', color: '#1a1a1a' },
  subtitle: { fontSize: 14, color: '#666', marginTop: 4 },
  emptyContainer: { flex: 1, justifyContent: 'center', alignItems: 'center', paddingHorizontal: 40 },
  emptyIcon: { fontSize: 60, marginBottom: 20 },
  emptyTitle: { fontSize: 20, fontWeight: 'bold', color: '#1a1a1a', marginBottom: 8 },
  emptyText: { fontSize: 14, color: '#666', textAlign: 'center' },
  connectButton: { backgroundColor: '#4CAF50', margin: 20, padding: 16, borderRadius: 12 },
  connectButtonText: { color: '#fff', fontSize: 16, fontWeight: 'bold', textAlign: 'center' },
});

export default ConnectedDevicesScreen;
