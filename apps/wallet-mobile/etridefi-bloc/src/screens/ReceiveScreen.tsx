import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  Share,
  Alert,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import QRCode from 'react-native-qrcode-svg';
import {colors, theme} from '@/theme';

export default function ReceiveScreen({navigation}: any) {
  const [token, setToken] = useState<'ETR' | 'EDSC'>('ETR');
  const address = 'etrid1abc2def3ghi4jkl5mno6pqr7stu8vwx9yz0';

  const handleShare = async () => {
    try {
      await Share.share({
        message: `Send ${token} to my wallet:\n${address}`,
        title: `My ${token} Address`,
      });
    } catch (error) {
      console.error('Error sharing:', error);
    }
  };

  const handleCopy = () => {
    // In a real app, you'd use Clipboard API
    Alert.alert('Copied!', 'Address copied to clipboard');
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Icon name="arrow-left" size={24} color={colors.text} />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Receive</Text>
        <View style={styles.tokenSelector}>
          <TouchableOpacity
            style={[styles.tokenButton, token === 'ETR' && styles.tokenButtonActive]}
            onPress={() => setToken('ETR')}>
            <Text
              style={[
                styles.tokenButtonText,
                token === 'ETR' && styles.tokenButtonTextActive,
              ]}>
              ÉTR
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[
              styles.tokenButton,
              token === 'EDSC' && styles.tokenButtonActive,
            ]}
            onPress={() => setToken('EDSC')}>
            <Text
              style={[
                styles.tokenButtonText,
                token === 'EDSC' && styles.tokenButtonTextActive,
              ]}>
              EDSC
            </Text>
          </TouchableOpacity>
        </View>
      </View>

      {/* Content */}
      <View style={styles.content}>
        <Text style={styles.title}>Receive {token}</Text>
        <Text style={styles.subtitle}>
          Share this QR code or address to receive {token}
        </Text>

        {/* QR Code */}
        <View style={styles.qrContainer}>
          <View style={styles.qrCode}>
            <QRCode value={address} size={200} backgroundColor="white" />
          </View>
        </View>

        {/* Address */}
        <View style={styles.addressCard}>
          <Text style={styles.addressLabel}>Your {token} Address</Text>
          <Text style={styles.addressText} numberOfLines={1}>
            {address}
          </Text>
        </View>

        {/* Action Buttons */}
        <View style={styles.actions}>
          <TouchableOpacity style={styles.actionButton} onPress={handleCopy}>
            <Icon name="copy" size={24} color={colors.primary} />
            <Text style={styles.actionButtonText}>Copy Address</Text>
          </TouchableOpacity>

          <TouchableOpacity style={styles.actionButton} onPress={handleShare}>
            <Icon name="share-2" size={24} color={colors.primary} />
            <Text style={styles.actionButtonText}>Share</Text>
          </TouchableOpacity>
        </View>

        {/* Info */}
        <View style={styles.infoCard}>
          <Icon name="info" size={20} color={colors.info} />
          <Text style={styles.infoText}>
            Only send {token} to this address. Sending other tokens may result
            in permanent loss.
          </Text>
        </View>
      </View>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: theme.spacing.md,
    paddingTop: 50,
  },
  headerTitle: {
    fontSize: theme.fontSize.xl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  tokenSelector: {
    flexDirection: 'row',
    backgroundColor: colors.surface,
    borderRadius: theme.borderRadius.md,
    padding: 4,
  },
  tokenButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: theme.borderRadius.sm,
  },
  tokenButtonActive: {
    backgroundColor: colors.primary,
  },
  tokenButtonText: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.textSecondary,
  },
  tokenButtonTextActive: {
    color: colors.text,
  },
  content: {
    flex: 1,
    padding: theme.spacing.md,
    alignItems: 'center',
  },
  title: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginTop: theme.spacing.lg,
  },
  subtitle: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    textAlign: 'center',
    marginTop: theme.spacing.sm,
    marginBottom: theme.spacing.xl,
  },
  qrContainer: {
    padding: theme.spacing.lg,
    backgroundColor: colors.glassStrong,
    borderRadius: theme.borderRadius.xl,
    marginBottom: theme.spacing.lg,
  },
  qrCode: {
    padding: theme.spacing.md,
    backgroundColor: 'white',
    borderRadius: theme.borderRadius.md,
  },
  addressCard: {
    width: '100%',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: colors.border,
    marginBottom: theme.spacing.lg,
  },
  addressLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: 8,
  },
  addressText: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
  },
  actions: {
    flexDirection: 'row',
    gap: theme.spacing.md,
    width: '100%',
    marginBottom: theme.spacing.lg,
  },
  actionButton: {
    flex: 1,
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
    gap: 8,
  },
  actionButtonText: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
  },
  infoCard: {
    flexDirection: 'row',
    backgroundColor: colors.info + '20',
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: colors.info + '40',
    gap: 12,
  },
  infoText: {
    flex: 1,
    fontSize: theme.fontSize.sm,
    color: colors.text,
    lineHeight: 20,
  },
});
