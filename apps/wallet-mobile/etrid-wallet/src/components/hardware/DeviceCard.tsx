/**
 * DeviceCard - Connected hardware device info card
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { HardwareWallet } from '../../types/hardware.types';
import BatteryIndicator from './BatteryIndicator';

interface DeviceCardProps {
  device: HardwareWallet;
  onManage?: (device: HardwareWallet) => void;
  onDisconnect?: (device: HardwareWallet) => void;
}

const DeviceCard: React.FC<DeviceCardProps> = ({ device, onManage, onDisconnect }) => {
  const getDeviceIcon = (type: string): string => {
    switch (type) {
      case 'ledger':
        return '🔐';
      case 'degn':
        return '💎';
      default:
        return '📱';
    }
  };

  return (
    <View style={styles.card}>
      <View style={styles.header}>
        <View style={styles.deviceInfo}>
          <Text style={styles.icon}>{getDeviceIcon(device.type)}</Text>
          <View style={styles.nameContainer}>
            <Text style={styles.name}>{device.name}</Text>
            <Text style={styles.model}>{device.model}</Text>
          </View>
        </View>

        {device.batteryLevel !== undefined && (
          <BatteryIndicator level={device.batteryLevel} />
        )}
      </View>

      <View style={styles.details}>
        <View style={styles.detailRow}>
          <Text style={styles.detailLabel}>Status</Text>
          <View style={styles.statusContainer}>
            <View style={[styles.statusDot, device.connected && styles.statusConnected]} />
            <Text style={styles.detailValue}>
              {device.connected ? 'Connected' : 'Disconnected'}
            </Text>
          </View>
        </View>

        <View style={styles.detailRow}>
          <Text style={styles.detailLabel}>Accounts</Text>
          <Text style={styles.detailValue}>{device.accounts.length}</Text>
        </View>

        <View style={styles.detailRow}>
          <Text style={styles.detailLabel}>Firmware</Text>
          <Text style={styles.detailValue}>{device.firmware}</Text>
        </View>
      </View>

      <View style={styles.actions}>
        {onManage && (
          <TouchableOpacity
            style={[styles.button, styles.manageButton]}
            onPress={() => onManage(device)}
          >
            <Text style={styles.manageButtonText}>Manage</Text>
          </TouchableOpacity>
        )}

        {onDisconnect && device.connected && (
          <TouchableOpacity
            style={[styles.button, styles.disconnectButton]}
            onPress={() => onDisconnect(device)}
          >
            <Text style={styles.disconnectButtonText}>Disconnect</Text>
          </TouchableOpacity>
        )}
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  card: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginHorizontal: 16,
    marginVertical: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 16,
  },
  deviceInfo: {
    flexDirection: 'row',
    alignItems: 'center',
    flex: 1,
  },
  icon: {
    fontSize: 40,
    marginRight: 12,
  },
  nameContainer: {
    flex: 1,
  },
  name: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 4,
  },
  model: {
    fontSize: 14,
    color: '#666',
  },
  details: {
    paddingVertical: 12,
    borderTopWidth: 1,
    borderTopColor: '#f0f0f0',
  },
  detailRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: 8,
  },
  detailLabel: {
    fontSize: 14,
    color: '#666',
  },
  detailValue: {
    fontSize: 14,
    fontWeight: '600',
    color: '#1a1a1a',
  },
  statusContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  statusDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: '#ccc',
    marginRight: 6,
  },
  statusConnected: {
    backgroundColor: '#4CAF50',
  },
  actions: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: 16,
    gap: 12,
  },
  button: {
    flex: 1,
    paddingVertical: 12,
    borderRadius: 8,
    alignItems: 'center',
  },
  manageButton: {
    backgroundColor: '#4CAF50',
  },
  manageButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
  disconnectButton: {
    backgroundColor: '#fff',
    borderWidth: 1,
    borderColor: '#F44336',
  },
  disconnectButtonText: {
    color: '#F44336',
    fontSize: 16,
    fontWeight: '600',
  },
});

export default DeviceCard;
