import React, {useState} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  TextInput,
  Alert,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * BridgeScreen - Cross-chain bridge for token transfers
 * Matches iOS Swift BridgeView
 */

interface Chain {
  id: string;
  name: string;
  icon: string;
  color: string;
}

const chains: Chain[] = [
  {id: 'etrid', name: 'Ëtrid', icon: 'E', color: '#6366F1'},
  {id: 'eth', name: 'Ethereum', icon: 'E', color: '#627EEA'},
  {id: 'btc', name: 'Bitcoin', icon: 'B', color: '#F7931A'},
  {id: 'sol', name: 'Solana', icon: 'S', color: '#00D18C'},
  {id: 'bnb', name: 'BNB Chain', icon: 'B', color: '#F0B90B'},
];

export default function BridgeScreen({navigation}: any) {
  const [fromChain, setFromChain] = useState<Chain>(chains[0]);
  const [toChain, setToChain] = useState<Chain>(chains[1]);
  const [amount, setAmount] = useState('');
  const [showFromPicker, setShowFromPicker] = useState(false);
  const [showToPicker, setShowToPicker] = useState(false);

  const swapChains = () => {
    const temp = fromChain;
    setFromChain(toChain);
    setToChain(temp);
  };

  const handleBridge = () => {
    Alert.alert(
      'Bridge Coming Soon',
      `Cross-chain bridging from ${fromChain.name} to ${toChain.name} will be available soon.`,
      [{text: 'OK'}]
    );
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Bridge Header */}
        <View style={styles.header}>
          <Icon name="git-branch" size={32} color={colors.primary} />
          <Text style={styles.headerTitle}>Cross-Chain Bridge</Text>
          <Text style={styles.headerSubtitle}>
            Transfer tokens between blockchains
          </Text>
        </View>

        {/* From Chain */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>From</Text>
          <TouchableOpacity
            style={styles.chainSelector}
            onPress={() => setShowFromPicker(!showFromPicker)}>
            <View style={[styles.chainIcon, {backgroundColor: fromChain.color}]}>
              <Text style={styles.chainIconText}>{fromChain.icon}</Text>
            </View>
            <View style={styles.chainInfo}>
              <Text style={styles.chainName}>{fromChain.name}</Text>
              <Text style={styles.chainBalance}>Balance: 0.00</Text>
            </View>
            <Icon name="chevron-down" size={20} color={colors.textSecondary} />
          </TouchableOpacity>
          {showFromPicker && (
            <View style={styles.chainPicker}>
              {chains.filter(c => c.id !== toChain.id).map(chain => (
                <TouchableOpacity
                  key={chain.id}
                  style={styles.chainOption}
                  onPress={() => {
                    setFromChain(chain);
                    setShowFromPicker(false);
                  }}>
                  <View style={[styles.chainIcon, {backgroundColor: chain.color}]}>
                    <Text style={styles.chainIconText}>{chain.icon}</Text>
                  </View>
                  <Text style={styles.chainOptionText}>{chain.name}</Text>
                </TouchableOpacity>
              ))}
            </View>
          )}
        </View>

        {/* Swap Button */}
        <TouchableOpacity style={styles.swapButton} onPress={swapChains}>
          <Icon name="repeat" size={24} color={colors.primary} />
        </TouchableOpacity>

        {/* To Chain */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>To</Text>
          <TouchableOpacity
            style={styles.chainSelector}
            onPress={() => setShowToPicker(!showToPicker)}>
            <View style={[styles.chainIcon, {backgroundColor: toChain.color}]}>
              <Text style={styles.chainIconText}>{toChain.icon}</Text>
            </View>
            <View style={styles.chainInfo}>
              <Text style={styles.chainName}>{toChain.name}</Text>
              <Text style={styles.chainBalance}>Balance: 0.00</Text>
            </View>
            <Icon name="chevron-down" size={20} color={colors.textSecondary} />
          </TouchableOpacity>
          {showToPicker && (
            <View style={styles.chainPicker}>
              {chains.filter(c => c.id !== fromChain.id).map(chain => (
                <TouchableOpacity
                  key={chain.id}
                  style={styles.chainOption}
                  onPress={() => {
                    setToChain(chain);
                    setShowToPicker(false);
                  }}>
                  <View style={[styles.chainIcon, {backgroundColor: chain.color}]}>
                    <Text style={styles.chainIconText}>{chain.icon}</Text>
                  </View>
                  <Text style={styles.chainOptionText}>{chain.name}</Text>
                </TouchableOpacity>
              ))}
            </View>
          )}
        </View>

        {/* Amount Input */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Amount</Text>
          <View style={styles.inputContainer}>
            <TextInput
              style={styles.amountInput}
              value={amount}
              onChangeText={setAmount}
              placeholder="0.00"
              placeholderTextColor={colors.textMuted}
              keyboardType="decimal-pad"
            />
            <TouchableOpacity style={styles.maxButton}>
              <Text style={styles.maxButtonText}>MAX</Text>
            </TouchableOpacity>
          </View>
        </View>

        {/* Bridge Info */}
        <View style={styles.card}>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Estimated Time</Text>
            <Text style={styles.infoValue}>~10 minutes</Text>
          </View>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Bridge Fee</Text>
            <Text style={styles.infoValue}>0.1%</Text>
          </View>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>You'll Receive</Text>
            <Text style={styles.infoValue}>
              ~{amount ? (parseFloat(amount) * 0.999).toFixed(4) : '0.00'}
            </Text>
          </View>
        </View>

        {/* Coming Soon Badge */}
        <View style={styles.comingSoonBadge}>
          <Icon name="clock" size={16} color={colors.warning} />
          <Text style={styles.comingSoonText}>Cross-chain bridge coming soon</Text>
        </View>

        {/* Bridge Button */}
        <TouchableOpacity style={styles.bridgeButton} onPress={handleBridge}>
          <Text style={styles.bridgeButtonText}>Bridge Tokens</Text>
        </TouchableOpacity>

        {/* Supported Chains Info */}
        <View style={styles.supportedChains}>
          <Text style={styles.supportedTitle}>Supported Chains</Text>
          <View style={styles.chainLogos}>
            {chains.map(chain => (
              <View
                key={chain.id}
                style={[styles.smallChainIcon, {backgroundColor: chain.color}]}>
                <Text style={styles.smallChainText}>{chain.icon}</Text>
              </View>
            ))}
          </View>
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
    padding: theme.spacing.md,
  },
  header: {
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  headerTitle: {
    fontSize: theme.fontSize.xl,
    fontWeight: '700',
    color: colors.text,
    marginTop: theme.spacing.sm,
  },
  headerSubtitle: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
    marginTop: 4,
  },
  card: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.sm,
  },
  cardTitle: {
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
    color: colors.textSecondary,
    marginBottom: theme.spacing.sm,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  chainSelector: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: theme.spacing.md,
  },
  chainIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    alignItems: 'center',
    justifyContent: 'center',
  },
  chainIconText: {
    fontSize: 18,
    fontWeight: '700',
    color: '#FFFFFF',
  },
  chainInfo: {
    flex: 1,
  },
  chainName: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  chainBalance: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  chainPicker: {
    marginTop: theme.spacing.md,
    borderTopWidth: 1,
    borderTopColor: colors.border,
    paddingTop: theme.spacing.md,
  },
  chainOption: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: theme.spacing.sm,
    gap: theme.spacing.md,
  },
  chainOptionText: {
    fontSize: theme.fontSize.md,
    color: colors.text,
  },
  swapButton: {
    alignSelf: 'center',
    width: 48,
    height: 48,
    borderRadius: 24,
    backgroundColor: colors.glass,
    alignItems: 'center',
    justifyContent: 'center',
    marginVertical: -theme.spacing.sm,
    zIndex: 1,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    paddingHorizontal: theme.spacing.md,
  },
  amountInput: {
    flex: 1,
    fontSize: 20,
    fontWeight: '600',
    color: colors.text,
    paddingVertical: theme.spacing.md,
  },
  maxButton: {
    backgroundColor: colors.primary,
    paddingHorizontal: theme.spacing.sm,
    paddingVertical: 6,
    borderRadius: theme.borderRadius.sm,
  },
  maxButtonText: {
    fontSize: theme.fontSize.xs,
    fontWeight: '700',
    color: colors.text,
  },
  infoRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: theme.spacing.xs,
  },
  infoLabel: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  infoValue: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  comingSoonBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: `${colors.warning}20`,
    padding: theme.spacing.sm,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.md,
    gap: 8,
  },
  comingSoonText: {
    fontSize: theme.fontSize.sm,
    color: colors.warning,
    fontWeight: '600',
  },
  bridgeButton: {
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  bridgeButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: colors.text,
  },
  supportedChains: {
    alignItems: 'center',
    marginBottom: theme.spacing.xl,
  },
  supportedTitle: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: theme.spacing.sm,
  },
  chainLogos: {
    flexDirection: 'row',
    gap: theme.spacing.sm,
  },
  smallChainIcon: {
    width: 32,
    height: 32,
    borderRadius: 16,
    alignItems: 'center',
    justifyContent: 'center',
  },
  smallChainText: {
    fontSize: 14,
    fontWeight: '700',
    color: '#FFFFFF',
  },
});
