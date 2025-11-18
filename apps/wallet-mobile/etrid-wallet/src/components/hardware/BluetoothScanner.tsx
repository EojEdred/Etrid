/**
 * BluetoothScanner - Bluetooth device scanning UI
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ActivityIndicator,
  TouchableOpacity,
} from 'react-native';
import { LedgerDevice } from '../../types/hardware.types';

interface BluetoothScannerProps {
  isScanning: boolean;
  devices: LedgerDevice[];
  onDevicePress: (device: LedgerDevice) => void;
  onScan?: () => void;
}

const BluetoothScanner: React.FC<BluetoothScannerProps> = ({
  isScanning,
  devices,
  onDevicePress,
  onScan,
}) => {
  return (
    <View style={styles.container}>
      {isScanning && (
        <View style={styles.scanningContainer}>
          <ActivityIndicator size="large" color="#4CAF50" />
          <Text style={styles.scanningText}>Searching for Ledger devices...</Text>
          <Text style={styles.hint}>
            Make sure your Ledger is turned on and Bluetooth is enabled
          </Text>
        </View>
      )}

      {!isScanning && devices.length === 0 && (
        <View style={styles.emptyContainer}>
          <Text style={styles.emptyIcon}>📡</Text>
          <Text style={styles.emptyTitle}>No devices found</Text>
          <Text style={styles.emptyText}>
            Make sure your Ledger is:
          </Text>
          <Text style={styles.emptyText}>• Turned on</Text>
          <Text style={styles.emptyText}>• Bluetooth enabled</Text>
          <Text style={styles.emptyText}>• Within range</Text>

          {onScan && (
            <TouchableOpacity style={styles.scanButton} onPress={onScan}>
              <Text style={styles.scanButtonText}>Scan Again</Text>
            </TouchableOpacity>
          )}
        </View>
      )}

      {!isScanning && devices.length > 0 && (
        <View style={styles.devicesContainer}>
          <Text style={styles.devicesTitle}>
            Found {devices.length} device{devices.length !== 1 ? 's' : ''}
          </Text>

          {devices.map((device) => (
            <TouchableOpacity
              key={device.id}
              style={styles.deviceCard}
              onPress={() => onDevicePress(device)}
              activeOpacity={0.7}
            >
              <View style={styles.deviceIcon}>
                <Text style={styles.deviceIconText}>🔐</Text>
              </View>

              <View style={styles.deviceInfo}>
                <Text style={styles.deviceName}>{device.name || 'Ledger Device'}</Text>
                <Text style={styles.deviceId}>{device.id}</Text>
              </View>

              <Text style={styles.connectArrow}>›</Text>
            </TouchableOpacity>
          ))}

          {onScan && (
            <TouchableOpacity
              style={[styles.scanButton, styles.scanAgainButton]}
              onPress={onScan}
            >
              <Text style={styles.scanButtonText}>Scan Again</Text>
            </TouchableOpacity>
          )}
        </View>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
  },
  scanningContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  scanningText: {
    marginTop: 20,
    fontSize: 18,
    fontWeight: '600',
    color: '#1a1a1a',
  },
  hint: {
    marginTop: 12,
    fontSize: 14,
    color: '#666',
    textAlign: 'center',
    paddingHorizontal: 40,
  },
  emptyContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 40,
  },
  emptyIcon: {
    fontSize: 60,
    marginBottom: 20,
  },
  emptyTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 12,
  },
  emptyText: {
    fontSize: 14,
    color: '#666',
    marginVertical: 4,
    textAlign: 'center',
  },
  devicesContainer: {
    flex: 1,
  },
  devicesTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 16,
  },
  deviceCard: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginBottom: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  deviceIcon: {
    width: 50,
    height: 50,
    borderRadius: 25,
    backgroundColor: '#f5f5f5',
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 16,
  },
  deviceIconText: {
    fontSize: 24,
  },
  deviceInfo: {
    flex: 1,
  },
  deviceName: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1a1a1a',
    marginBottom: 4,
  },
  deviceId: {
    fontSize: 12,
    color: '#999',
    fontFamily: 'monospace',
  },
  connectArrow: {
    fontSize: 30,
    color: '#ccc',
    fontWeight: 'bold',
  },
  scanButton: {
    backgroundColor: '#4CAF50',
    paddingVertical: 14,
    paddingHorizontal: 32,
    borderRadius: 24,
    marginTop: 20,
    alignSelf: 'center',
  },
  scanAgainButton: {
    marginTop: 24,
  },
  scanButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
});

export default BluetoothScanner;
