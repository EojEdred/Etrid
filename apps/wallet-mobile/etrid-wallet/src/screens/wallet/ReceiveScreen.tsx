import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, Share, Alert } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import QRCode from 'react-native-qrcode-svg';
import * as Clipboard from 'expo-clipboard';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';

export const ReceiveScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const { address } = useAuth();
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    if (address) {
      await Clipboard.setStringAsync(address);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleShare = async () => {
    try {
      await Share.share({
        message: `My Ëtrid address: ${address}`,
        title: 'Ëtrid Address',
      });
    } catch (error) {
      console.error('Error sharing address:', error);
    }
  };

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Text style={styles.backButton}>←</Text>
        </TouchableOpacity>
        <Text style={styles.title}>Receive ETR</Text>
        <View style={{ width: 40 }} />
      </View>

      <View style={styles.content}>
        <Text style={styles.instruction}>Scan QR code to receive ETR</Text>

        <View style={styles.qrContainer}>
          {address && (
            <QRCode value={address} size={240} backgroundColor={colors.background} color={colors.text} />
          )}
        </View>

        <View style={styles.addressContainer}>
          <Text style={styles.addressLabel}>Your Address</Text>
          <Text style={styles.address}>{address}</Text>
        </View>

        <TouchableOpacity style={styles.copyButton} onPress={handleCopy}>
          <Text style={styles.copyButtonText}>{copied ? '✓ Copied!' : '📋 Copy Address'}</Text>
        </TouchableOpacity>

        <TouchableOpacity style={styles.shareButton} onPress={handleShare}>
          <Text style={styles.shareButtonText}>Share Address</Text>
        </TouchableOpacity>
      </View>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: colors.background },
  header: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', paddingHorizontal: spacing.lg, paddingVertical: spacing.md },
  backButton: { fontSize: 28, color: colors.text },
  title: { ...typography.h2, color: colors.text },
  content: { flex: 1, alignItems: 'center', paddingHorizontal: spacing.lg, paddingTop: spacing.xl },
  instruction: { ...typography.body, color: colors.textSecondary, marginBottom: spacing.xl, textAlign: 'center' },
  qrContainer: { backgroundColor: colors.surface, padding: spacing.lg, borderRadius: borderRadius.lg, marginBottom: spacing.xl },
  addressContainer: { width: '100%', backgroundColor: colors.surface, borderRadius: borderRadius.md, padding: spacing.md, marginBottom: spacing.lg },
  addressLabel: { ...typography.bodySmall, color: colors.textSecondary, marginBottom: spacing.xs },
  address: { ...typography.body, color: colors.text, fontWeight: '600' },
  copyButton: { backgroundColor: colors.primary, paddingVertical: spacing.md, paddingHorizontal: spacing.xl, borderRadius: borderRadius.lg, marginBottom: spacing.md },
  copyButtonText: { ...typography.h3, color: colors.background },
  shareButton: { paddingVertical: spacing.md },
  shareButtonText: { ...typography.body, color: colors.primary, fontWeight: '600' },
});
