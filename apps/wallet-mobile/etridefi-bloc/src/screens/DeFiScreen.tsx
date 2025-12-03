import React, {useState} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  RefreshControl,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * DeFiScreen - Matches iOS Swift DeFiHubView exactly
 *
 * Features:
 * - Virtual Card (Coming Soon)
 * - ATM Locator
 * - Crypto Loans (Coming Soon)
 * - Yield Farming (Coming Soon)
 * - Swap (functional)
 *
 * Additional features from original RN app bundled as "Coming Soon":
 * - NFT Marketplace (Beta)
 * - Fiat On/Off Ramp (Coming Soon)
 * - Savings Goals (Coming Soon)
 * - Lending (Coming Soon)
 */

interface DeFiFeature {
  id: string;
  title: string;
  description: string;
  icon: string;
  status: 'active' | 'coming_soon' | 'beta';
  route?: string;
}

const DEFI_FEATURES: DeFiFeature[] = [
  {
    id: 'swap',
    title: 'Swap',
    description: 'Exchange tokens instantly on PrimeSwap DEX',
    icon: 'repeat',
    status: 'active',
    route: 'Swap',
  },
  {
    id: 'virtual_card',
    title: 'Virtual Card',
    description: 'Spend crypto anywhere with your virtual debit card',
    icon: 'credit-card',
    status: 'coming_soon',
  },
  {
    id: 'atm_locator',
    title: 'ATM Locator',
    description: 'Find crypto ATMs near you',
    icon: 'map-pin',
    status: 'active',
  },
  {
    id: 'crypto_loans',
    title: 'Crypto Loans',
    description: 'Borrow against your crypto holdings',
    icon: 'dollar-sign',
    status: 'coming_soon',
  },
  {
    id: 'yield_farming',
    title: 'Yield Farming',
    description: 'Earn rewards by providing liquidity',
    icon: 'trending-up',
    status: 'coming_soon',
  },
];

const ADDITIONAL_FEATURES: DeFiFeature[] = [
  {
    id: 'nft_marketplace',
    title: 'NFT Marketplace',
    description: 'Buy, sell, and trade NFTs',
    icon: 'image',
    status: 'beta',
  },
  {
    id: 'fiat_ramp',
    title: 'Fiat On/Off Ramp',
    description: 'Buy crypto with card or bank transfer',
    icon: 'credit-card',
    status: 'coming_soon',
  },
  {
    id: 'savings',
    title: 'Savings Goals',
    description: 'Set and track your savings goals',
    icon: 'target',
    status: 'coming_soon',
  },
  {
    id: 'lending',
    title: 'Lending',
    description: 'Lend your crypto and earn interest',
    icon: 'percent',
    status: 'coming_soon',
  },
];

export default function DeFiScreen({navigation}: any) {
  const [refreshing, setRefreshing] = useState(false);

  const onRefresh = async () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 1000);
  };

  const handleFeaturePress = (feature: DeFiFeature) => {
    if (feature.status === 'active' && feature.route) {
      navigation.navigate(feature.route);
    }
    // For coming_soon and beta, show a modal or toast
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
        }>
        {/* Main DeFi Features - Matches iOS Swift DeFiHubView */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>DeFi Services</Text>
          {DEFI_FEATURES.map(feature => (
            <FeatureCard
              key={feature.id}
              feature={feature}
              onPress={() => handleFeaturePress(feature)}
            />
          ))}
        </View>

        {/* Additional Services - Extra features bundled here */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Additional Services</Text>
          {ADDITIONAL_FEATURES.map(feature => (
            <FeatureCard
              key={feature.id}
              feature={feature}
              onPress={() => handleFeaturePress(feature)}
            />
          ))}
        </View>

        {/* Info Banner */}
        <View style={styles.infoBanner}>
          <Icon name="info" size={20} color={colors.info} />
          <Text style={styles.infoBannerText}>
            More DeFi features are being developed. Stay tuned for updates!
          </Text>
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function FeatureCard({
  feature,
  onPress,
}: {
  feature: DeFiFeature;
  onPress: () => void;
}) {
  const isDisabled = feature.status !== 'active';

  return (
    <TouchableOpacity
      style={[styles.featureCard, isDisabled && styles.featureCardDisabled]}
      onPress={onPress}
      disabled={isDisabled}>
      <View
        style={[
          styles.featureIcon,
          isDisabled && styles.featureIconDisabled,
        ]}>
        <Icon
          name={feature.icon}
          size={24}
          color={isDisabled ? colors.textSecondary : colors.primary}
        />
      </View>
      <View style={styles.featureInfo}>
        <View style={styles.featureTitleRow}>
          <Text
            style={[
              styles.featureTitle,
              isDisabled && styles.featureTitleDisabled,
            ]}>
            {feature.title}
          </Text>
          {feature.status === 'coming_soon' && (
            <View style={styles.comingSoonBadge}>
              <Text style={styles.comingSoonText}>Coming Soon</Text>
            </View>
          )}
          {feature.status === 'beta' && (
            <View style={styles.betaBadge}>
              <Text style={styles.betaText}>Beta</Text>
            </View>
          )}
        </View>
        <Text
          style={[
            styles.featureDescription,
            isDisabled && styles.featureDescriptionDisabled,
          ]}>
          {feature.description}
        </Text>
      </View>
      <Icon
        name="chevron-right"
        size={20}
        color={isDisabled ? colors.textMuted : colors.textSecondary}
      />
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
    padding: theme.spacing.md,
  },
  sectionTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.textSecondary,
    marginBottom: theme.spacing.md,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  featureCard: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  featureCardDisabled: {
    opacity: 0.7,
  },
  featureIcon: {
    width: 48,
    height: 48,
    borderRadius: 12,
    backgroundColor: `${colors.primary}20`,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.md,
  },
  featureIconDisabled: {
    backgroundColor: `${colors.textSecondary}20`,
  },
  featureInfo: {
    flex: 1,
  },
  featureTitleRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
    marginBottom: 4,
  },
  featureTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  featureTitleDisabled: {
    color: colors.textSecondary,
  },
  featureDescription: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  featureDescriptionDisabled: {
    color: colors.textMuted,
  },
  comingSoonBadge: {
    backgroundColor: colors.warning + '30',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 4,
  },
  comingSoonText: {
    fontSize: 10,
    fontWeight: '600',
    color: colors.warning,
  },
  betaBadge: {
    backgroundColor: colors.info + '30',
    paddingHorizontal: 8,
    paddingVertical: 2,
    borderRadius: 4,
  },
  betaText: {
    fontSize: 10,
    fontWeight: '600',
    color: colors.info,
  },
  infoBanner: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: `${colors.info}15`,
    marginHorizontal: theme.spacing.md,
    marginBottom: theme.spacing.xl,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    gap: 12,
  },
  infoBannerText: {
    flex: 1,
    fontSize: theme.fontSize.sm,
    color: colors.info,
  },
});
