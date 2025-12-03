import React, {useState} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  Switch,
  Alert,
  Linking,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * SettingsScreen - Matches iOS Swift SettingsView exactly
 *
 * Sections:
 * - Wallet (Manage Accounts, View Recovery Phrase, Backup)
 * - Security (Security Settings, Biometrics, Change Passcode)
 * - Network (Network, RPC Settings)
 * - Preferences (Currency, Notifications, Appearance)
 * - About (About ETRID, Website, Documentation, Terms, Privacy)
 * - Danger Zone (Delete Wallet)
 *
 * Additional Services bundled from original RN app as "Coming Soon":
 * - Social features
 * - NFT features
 * - Trading features
 */

interface SettingsItem {
  id: string;
  title: string;
  icon: string;
  value?: string;
  showArrow?: boolean;
  showBadge?: boolean;
  badgeText?: string;
  onPress?: () => void;
  isDestructive?: boolean;
  isToggle?: boolean;
  toggleValue?: boolean;
  onToggle?: (value: boolean) => void;
}

export default function SettingsScreen({navigation}: any) {
  const [biometricsEnabled, setBiometricsEnabled] = useState(true);
  const [notificationsEnabled, setNotificationsEnabled] = useState(true);

  const walletSection: SettingsItem[] = [
    {
      id: 'accounts',
      title: 'Manage Accounts',
      icon: 'users',
      showArrow: true,
      onPress: () => Alert.alert('Manage Accounts', 'Account management coming soon'),
    },
    {
      id: 'recovery',
      title: 'View Recovery Phrase',
      icon: 'key',
      showArrow: true,
      onPress: () => Alert.alert('Recovery Phrase', 'Authentication required to view recovery phrase'),
    },
    {
      id: 'backup',
      title: 'Backup Wallet',
      icon: 'shield',
      showBadge: true,
      badgeText: '!',
      showArrow: true,
      onPress: () => Alert.alert('Backup Required', 'Your wallet is not backed up. Backup now to protect your funds.'),
    },
  ];

  const securitySection: SettingsItem[] = [
    {
      id: 'security',
      title: 'Security Settings',
      icon: 'lock',
      showArrow: true,
      onPress: () => navigation.navigate('Security'),
    },
    {
      id: 'biometrics',
      title: 'Biometrics',
      icon: 'smartphone',
      isToggle: true,
      toggleValue: biometricsEnabled,
      onToggle: setBiometricsEnabled,
    },
    {
      id: 'passcode',
      title: 'Change Passcode',
      icon: 'lock',
      showArrow: true,
      onPress: () => Alert.alert('Change Passcode', 'Passcode change coming soon'),
    },
  ];

  const networkSection: SettingsItem[] = [
    {
      id: 'network',
      title: 'Network',
      icon: 'globe',
      value: 'Mainnet',
      showArrow: true,
      onPress: () => Alert.alert('Network', 'Network switching coming soon'),
    },
    {
      id: 'rpc',
      title: 'RPC Settings',
      icon: 'server',
      showArrow: true,
      onPress: () => Alert.alert('RPC Settings', 'Custom RPC configuration coming soon'),
    },
  ];

  const preferencesSection: SettingsItem[] = [
    {
      id: 'currency',
      title: 'Currency',
      icon: 'dollar-sign',
      value: 'USD',
      showArrow: true,
      onPress: () => Alert.alert('Currency', 'Currency selection coming soon'),
    },
    {
      id: 'notifications',
      title: 'Notifications',
      icon: 'bell',
      isToggle: true,
      toggleValue: notificationsEnabled,
      onToggle: setNotificationsEnabled,
    },
    {
      id: 'appearance',
      title: 'Appearance',
      icon: 'sun',
      value: 'System',
      showArrow: true,
      onPress: () => Alert.alert('Appearance', 'Theme selection coming soon'),
    },
  ];

  const aboutSection: SettingsItem[] = [
    {
      id: 'about',
      title: 'About ETRID',
      icon: 'info',
      showArrow: true,
      onPress: () => Alert.alert('ETRID Wallet', 'Version 1.0.0\n\nA next-generation blockchain wallet featuring multi-chain bridges, democratic governance, and innovative consensus mechanisms.'),
    },
    {
      id: 'website',
      title: 'Website',
      icon: 'globe',
      showArrow: true,
      onPress: () => Linking.openURL('https://etrid.network'),
    },
    {
      id: 'docs',
      title: 'Documentation',
      icon: 'book',
      showArrow: true,
      onPress: () => Linking.openURL('https://docs.etrid.network'),
    },
    {
      id: 'terms',
      title: 'Terms of Service',
      icon: 'file-text',
      showArrow: true,
      onPress: () => Alert.alert('Terms of Service', 'Terms of Service document'),
    },
    {
      id: 'privacy',
      title: 'Privacy Policy',
      icon: 'shield',
      showArrow: true,
      onPress: () => navigation.navigate('Privacy'),
    },
  ];

  const comingSoonSection: SettingsItem[] = [
    {
      id: 'social',
      title: 'Social Features',
      icon: 'users',
      showBadge: true,
      badgeText: 'Soon',
      showArrow: true,
      onPress: () => Alert.alert('Coming Soon', 'Social features including contacts, social payments, and community features are in development.'),
    },
    {
      id: 'nft',
      title: 'NFT Gallery',
      icon: 'image',
      showBadge: true,
      badgeText: 'Beta',
      showArrow: true,
      onPress: () => Alert.alert('Beta', 'NFT Gallery is currently in beta testing.'),
    },
    {
      id: 'trading',
      title: 'Advanced Trading',
      icon: 'trending-up',
      showBadge: true,
      badgeText: 'Soon',
      showArrow: true,
      onPress: () => Alert.alert('Coming Soon', 'Advanced trading features including limit orders and charts are in development.'),
    },
  ];

  const deleteWallet = () => {
    Alert.alert(
      'Delete Wallet',
      'Are you sure you want to delete your wallet? This action cannot be undone. Make sure you have backed up your recovery phrase.',
      [
        {text: 'Cancel', style: 'cancel'},
        {
          text: 'Delete',
          style: 'destructive',
          onPress: () => Alert.alert('Wallet Deleted', 'Your wallet has been deleted.'),
        },
      ]
    );
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Wallet Section */}
        <SettingsSection title="Wallet" items={walletSection} />

        {/* Security Section */}
        <SettingsSection title="Security" items={securitySection} />

        {/* Network Section */}
        <SettingsSection title="Network" items={networkSection} />

        {/* Preferences Section */}
        <SettingsSection title="Preferences" items={preferencesSection} />

        {/* Coming Soon Section - bundled extra features */}
        <SettingsSection title="Coming Soon" items={comingSoonSection} />

        {/* About Section */}
        <SettingsSection title="About" items={aboutSection} />

        {/* Danger Zone */}
        <View style={styles.section}>
          <TouchableOpacity style={styles.deleteButton} onPress={deleteWallet}>
            <Icon name="trash-2" size={20} color={colors.error} />
            <Text style={styles.deleteButtonText}>Delete Wallet</Text>
          </TouchableOpacity>
        </View>

        {/* App Version */}
        <View style={styles.versionContainer}>
          <Text style={styles.versionText}>Etrid Wallet v1.0.0</Text>
          <Text style={styles.buildText}>Build 2024.12.02</Text>
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function SettingsSection({title, items}: {title: string; items: SettingsItem[]}) {
  return (
    <View style={styles.section}>
      <Text style={styles.sectionTitle}>{title}</Text>
      <View style={styles.sectionContent}>
        {items.map((item, index) => (
          <SettingsRow key={item.id} item={item} isLast={index === items.length - 1} />
        ))}
      </View>
    </View>
  );
}

function SettingsRow({item, isLast}: {item: SettingsItem; isLast: boolean}) {
  return (
    <TouchableOpacity
      style={[styles.settingsRow, !isLast && styles.settingsRowBorder]}
      onPress={item.onPress}
      disabled={item.isToggle}>
      <View style={styles.settingsRowLeft}>
        <Icon name={item.icon} size={20} color={colors.primary} style={styles.settingsIcon} />
        <Text style={[styles.settingsRowTitle, item.isDestructive && styles.destructiveText]}>
          {item.title}
        </Text>
        {item.showBadge && (
          <View style={[styles.badge, item.badgeText === 'Beta' && styles.badgeBeta, item.badgeText === 'Soon' && styles.badgeSoon]}>
            <Text style={[styles.badgeText, item.badgeText === 'Beta' && styles.badgeTextBeta, item.badgeText === 'Soon' && styles.badgeTextSoon]}>
              {item.badgeText}
            </Text>
          </View>
        )}
      </View>
      <View style={styles.settingsRowRight}>
        {item.value && <Text style={styles.settingsRowValue}>{item.value}</Text>}
        {item.isToggle && (
          <Switch
            value={item.toggleValue}
            onValueChange={item.onToggle}
            trackColor={{false: colors.border, true: colors.primary}}
            thumbColor="#FFFFFF"
          />
        )}
        {item.showArrow && <Icon name="chevron-right" size={20} color={colors.textSecondary} />}
      </View>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
  },
  section: {
    paddingHorizontal: theme.spacing.md,
    marginBottom: theme.spacing.lg,
  },
  sectionTitle: {
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
    color: colors.textSecondary,
    marginBottom: theme.spacing.sm,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  sectionContent: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    overflow: 'hidden',
  },
  settingsRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingVertical: theme.spacing.md,
    paddingHorizontal: theme.spacing.md,
  },
  settingsRowBorder: {
    borderBottomWidth: 1,
    borderBottomColor: colors.border,
  },
  settingsRowLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    flex: 1,
  },
  settingsIcon: {
    marginRight: theme.spacing.md,
  },
  settingsRowTitle: {
    fontSize: theme.fontSize.md,
    color: colors.text,
  },
  destructiveText: {
    color: colors.error,
  },
  settingsRowRight: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
  },
  settingsRowValue: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  badge: {
    backgroundColor: colors.warning + '30',
    paddingHorizontal: 6,
    paddingVertical: 2,
    borderRadius: 4,
    marginLeft: 8,
  },
  badgeBeta: {
    backgroundColor: colors.info + '30',
  },
  badgeSoon: {
    backgroundColor: colors.warning + '30',
  },
  badgeText: {
    fontSize: 10,
    fontWeight: '600',
    color: colors.warning,
  },
  badgeTextBeta: {
    color: colors.info,
  },
  badgeTextSoon: {
    color: colors.warning,
  },
  deleteButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: `${colors.error}15`,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    gap: 8,
  },
  deleteButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.error,
  },
  versionContainer: {
    alignItems: 'center',
    paddingVertical: theme.spacing.xl,
  },
  versionText: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  buildText: {
    fontSize: theme.fontSize.xs,
    color: colors.textMuted,
    marginTop: 4,
  },
});
