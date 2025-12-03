import React, {useState, useEffect, useCallback} from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  TextInput,
  ScrollView,
  Modal,
  ActivityIndicator,
  Alert,
  Linking,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '../theme';
import {
  Network,
  NETWORK_CONFIG,
  WRAPPED_ETR,
  SWAP_SUPPORTED_CHAINS,
} from '../services/ContractAddresses';
import {DEXService, SwapQuote} from '../services/DEXService';
import {BalanceService} from '../services/BalanceService';
import {PriceService} from '../services/PriceService';
import BigNumber from 'bignumber.js';

// Token model for swap
interface SwapToken {
  id: string;
  symbol: string;
  name: string;
  contractAddress: string | null;
  decimals: number;
  color: string;
}

// Predefined tokens
const TOKENS: Record<string, SwapToken> = {
  eth: {id: 'eth', symbol: 'ETH', name: 'Ethereum', contractAddress: null, decimals: 18, color: '#627EEA'},
  wetr: {id: 'wetr', symbol: 'wETR', name: 'Wrapped ETRID', contractAddress: null, decimals: 18, color: '#8B5CF6'},
  bnb: {id: 'bnb', symbol: 'BNB', name: 'BNB', contractAddress: null, decimals: 18, color: '#F3BA2F'},
  matic: {id: 'matic', symbol: 'MATIC', name: 'Polygon', contractAddress: null, decimals: 18, color: '#8247E5'},
  sol: {id: 'sol', symbol: 'SOL', name: 'Solana', contractAddress: null, decimals: 9, color: '#00FFA3'},
  usdt: {id: 'usdt', symbol: 'USDT', name: 'Tether USD', contractAddress: null, decimals: 6, color: '#26A17B'},
  usdc: {id: 'usdc', symbol: 'USDC', name: 'USD Coin', contractAddress: null, decimals: 6, color: '#2775CA'},
};

// Get tokens for a chain
function getTokensForChain(chain: Network): SwapToken[] {
  const wETRAddress = WRAPPED_ETR[chain];
  const wETRToken: SwapToken = {
    id: `wetr-${chain}`,
    symbol: 'wETR',
    name: 'Wrapped ETRID',
    contractAddress: wETRAddress || null,
    decimals: chain === Network.SOLANA ? 9 : 18,
    color: '#8B5CF6',
  };

  switch (chain) {
    case Network.ETHEREUM:
      return [TOKENS.eth, wETRToken, TOKENS.usdt, TOKENS.usdc];
    case Network.BNB_CHAIN:
      return [TOKENS.bnb, wETRToken, TOKENS.usdt, TOKENS.usdc];
    case Network.POLYGON:
      return [TOKENS.matic, wETRToken, TOKENS.usdt, TOKENS.usdc];
    case Network.ARBITRUM:
      return [TOKENS.eth, wETRToken, TOKENS.usdt, TOKENS.usdc];
    case Network.SOLANA:
      return [TOKENS.sol, wETRToken, TOKENS.usdc];
    default:
      return [wETRToken];
  }
}

export default function SwapScreen({navigation}: any) {
  // State
  const [selectedChain, setSelectedChain] = useState<Network>(Network.ETHEREUM);
  const [fromToken, setFromToken] = useState<SwapToken>(TOKENS.eth);
  const [toToken, setToToken] = useState<SwapToken>(TOKENS.wetr);
  const [fromAmount, setFromAmount] = useState('');
  const [slippage, setSlippage] = useState(0.5);
  const [currentQuote, setCurrentQuote] = useState<SwapQuote | null>(null);
  const [isLoadingQuote, setIsLoadingQuote] = useState(false);
  const [quoteError, setQuoteError] = useState<string | null>(null);
  const [transactionHash, setTransactionHash] = useState<string | null>(null);

  // Modals
  const [showSettings, setShowSettings] = useState(false);
  const [showTokenPicker, setShowTokenPicker] = useState(false);
  const [isSelectingFromToken, setIsSelectingFromToken] = useState(true);
  const [showConfirmation, setShowConfirmation] = useState(false);

  // Initialize services
  useEffect(() => {
    PriceService.startPriceUpdates();
    return () => {
      PriceService.stopPriceUpdates();
    };
  }, []);

  // Update tokens when chain changes
  useEffect(() => {
    const tokens = getTokensForChain(selectedChain);
    if (!tokens.find(t => t.symbol === fromToken.symbol)) {
      setFromToken(tokens[0]);
    }
    if (!tokens.find(t => t.symbol === toToken.symbol)) {
      setToToken(tokens.find(t => t.id !== fromToken.id) || tokens[0]);
    }
    setCurrentQuote(null);
  }, [selectedChain]);

  // Fetch quote when amount changes
  const fetchQuote = useCallback(async () => {
    if (!fromAmount || parseFloat(fromAmount) <= 0) {
      setCurrentQuote(null);
      return;
    }

    setIsLoadingQuote(true);
    setQuoteError(null);

    try {
      const quote = await DEXService.getQuote(
        fromToken.contractAddress || fromToken.symbol,
        toToken.contractAddress || toToken.symbol,
        fromAmount,
        selectedChain,
        slippage,
      );
      setCurrentQuote(quote);
    } catch (error: any) {
      setQuoteError(error.message || 'Failed to get quote');
    } finally {
      setIsLoadingQuote(false);
    }
  }, [fromAmount, fromToken, toToken, selectedChain, slippage]);

  // Debounced quote fetch
  useEffect(() => {
    const timer = setTimeout(fetchQuote, 500);
    return () => clearTimeout(timer);
  }, [fetchQuote]);

  // Swap tokens
  const swapTokens = () => {
    const temp = fromToken;
    setFromToken(toToken);
    setToToken(temp);
    setCurrentQuote(null);
  };

  // Execute swap
  const executeSwap = async () => {
    setShowConfirmation(false);

    // In production, this would:
    // 1. Build the swap transaction
    // 2. Request password/biometric
    // 3. Sign and broadcast

    // Simulated success
    setTimeout(() => {
      const hash = '0x' + Array.from({length: 64}, () =>
        '0123456789abcdef'[Math.floor(Math.random() * 16)],
      ).join('');
      setTransactionHash(hash);
      Alert.alert('Success', 'Swap transaction submitted!');
    }, 1500);
  };

  // Open explorer
  const openExplorer = (hash: string) => {
    const config = NETWORK_CONFIG[selectedChain];
    const url = `${config.explorerURL}/tx/${hash}`;
    Linking.openURL(url);
  };

  // Format number
  const formatNumber = (value: string, decimals: number = 6): string => {
    const bn = new BigNumber(value);
    if (bn.isNaN() || bn.isZero()) return '0.00';
    return bn.toFormat(decimals, BigNumber.ROUND_DOWN);
  };

  // Get estimated output
  const estimatedOutput = currentQuote ? formatNumber(currentQuote.toAmount) : '';

  // Is swap enabled
  const isSwapEnabled = !!fromAmount && parseFloat(fromAmount) > 0;

  // Available tokens
  const availableTokens = getTokensForChain(selectedChain);

  return (
    <LinearGradient colors={[colors.gradientStart, colors.gradientEnd]} style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation?.goBack()}>
          <Icon name="arrow-left" size={24} color={colors.text} />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Swap</Text>
        <TouchableOpacity onPress={() => setShowSettings(true)}>
          <Icon name="settings" size={24} color={colors.text} />
        </TouchableOpacity>
      </View>

      <ScrollView style={styles.content} showsVerticalScrollIndicator={false}>
        {/* Chain Selector */}
        <ScrollView horizontal showsHorizontalScrollIndicator={false} style={styles.chainSelector}>
          {SWAP_SUPPORTED_CHAINS.map(chain => (
            <TouchableOpacity
              key={chain}
              style={[
                styles.chainButton,
                selectedChain === chain && styles.chainButtonSelected,
              ]}
              onPress={() => setSelectedChain(chain)}>
              <Text style={styles.chainIcon}>{NETWORK_CONFIG[chain].icon}</Text>
              <Text style={[
                styles.chainName,
                selectedChain === chain && styles.chainNameSelected,
              ]}>
                {NETWORK_CONFIG[chain].symbol}
              </Text>
            </TouchableOpacity>
          ))}
        </ScrollView>

        {/* Swap Card */}
        <View style={styles.swapCard}>
          {/* From Token */}
          <View style={styles.tokenRow}>
            <Text style={styles.tokenLabel}>From</Text>
            <View style={styles.tokenInputRow}>
              <TextInput
                style={styles.tokenInput}
                placeholder="0.0"
                placeholderTextColor={colors.textMuted}
                keyboardType="decimal-pad"
                value={fromAmount}
                onChangeText={setFromAmount}
              />
              <TouchableOpacity
                style={styles.tokenSelector}
                onPress={() => {
                  setIsSelectingFromToken(true);
                  setShowTokenPicker(true);
                }}>
                <View style={[styles.tokenIcon, {backgroundColor: fromToken.color}]}>
                  <Text style={styles.tokenIconText}>{fromToken.symbol.charAt(0)}</Text>
                </View>
                <Text style={styles.tokenSymbol}>{fromToken.symbol}</Text>
                <Icon name="chevron-down" size={16} color={colors.text} />
              </TouchableOpacity>
            </View>
            <View style={styles.tokenMeta}>
              <Text style={styles.tokenBalance}>Balance: 0.00</Text>
              <TouchableOpacity onPress={() => setFromAmount('100')}>
                <Text style={styles.maxButton}>MAX</Text>
              </TouchableOpacity>
            </View>
          </View>

          {/* Swap Arrow */}
          <TouchableOpacity style={styles.swapArrowContainer} onPress={swapTokens}>
            <View style={styles.swapArrow}>
              <Icon name="arrow-down" size={20} color={colors.primary} />
            </View>
          </TouchableOpacity>

          {/* To Token */}
          <View style={styles.tokenRow}>
            <Text style={styles.tokenLabel}>To</Text>
            <View style={styles.tokenInputRow}>
              <Text style={styles.tokenOutputText}>
                {estimatedOutput || '0.0'}
              </Text>
              <TouchableOpacity
                style={styles.tokenSelector}
                onPress={() => {
                  setIsSelectingFromToken(false);
                  setShowTokenPicker(true);
                }}>
                <View style={[styles.tokenIcon, {backgroundColor: toToken.color}]}>
                  <Text style={styles.tokenIconText}>{toToken.symbol.charAt(0)}</Text>
                </View>
                <Text style={styles.tokenSymbol}>{toToken.symbol}</Text>
                <Icon name="chevron-down" size={16} color={colors.text} />
              </TouchableOpacity>
            </View>
            {currentQuote && (
              <Text style={styles.tokenUsdValue}>
                ~${formatNumber(new BigNumber(currentQuote.toAmount).multipliedBy(0.15).toString(), 2)}
              </Text>
            )}
          </View>
        </View>

        {/* Quote Details */}
        {currentQuote && (
          <View style={styles.quoteDetails}>
            <View style={styles.quoteRow}>
              <Text style={styles.quoteLabel}>Rate</Text>
              <Text style={styles.quoteValue}>
                1 {fromToken.symbol} = {DEXService.formatRate(currentQuote)} {toToken.symbol}
              </Text>
            </View>
            <View style={styles.quoteRow}>
              <Text style={styles.quoteLabel}>Price Impact</Text>
              <Text style={[
                styles.quoteValue,
                currentQuote.priceImpact > 1 ? styles.highImpact : styles.lowImpact,
              ]}>
                {DEXService.formatPriceImpact(currentQuote)}
              </Text>
            </View>
            <View style={styles.quoteRow}>
              <Text style={styles.quoteLabel}>Route</Text>
              <Text style={styles.quoteValue}>{currentQuote.route}</Text>
            </View>
            <View style={styles.quoteRow}>
              <Text style={styles.quoteLabel}>Est. Gas</Text>
              <Text style={styles.quoteValue}>~{currentQuote.estimatedGas} units</Text>
            </View>
            <View style={styles.quoteRow}>
              <Text style={styles.quoteLabel}>Min. Received</Text>
              <Text style={styles.quoteValue}>
                {formatNumber(DEXService.calculateMinimumReceived(currentQuote, slippage))} {toToken.symbol}
              </Text>
            </View>
          </View>
        )}

        {/* Error Banner */}
        {quoteError && (
          <View style={styles.errorBanner}>
            <Icon name="alert-triangle" size={18} color={colors.warning} />
            <Text style={styles.errorText}>{quoteError}</Text>
            <TouchableOpacity onPress={() => setQuoteError(null)}>
              <Icon name="x" size={18} color={colors.text} />
            </TouchableOpacity>
          </View>
        )}

        {/* Transaction Result */}
        {transactionHash && (
          <View style={styles.transactionResult}>
            <Icon name="check-circle" size={48} color={colors.success} />
            <Text style={styles.transactionTitle}>Transaction Submitted</Text>
            <Text style={styles.transactionHash} numberOfLines={1}>
              {transactionHash}
            </Text>
            <TouchableOpacity onPress={() => openExplorer(transactionHash)}>
              <Text style={styles.explorerLink}>View on Explorer</Text>
            </TouchableOpacity>
          </View>
        )}

        {/* Swap Button */}
        <TouchableOpacity
          style={[
            styles.swapButton,
            (!isSwapEnabled || isLoadingQuote) && styles.swapButtonDisabled,
          ]}
          disabled={!isSwapEnabled || isLoadingQuote}
          onPress={() => {
            if (currentQuote) {
              setShowConfirmation(true);
            } else {
              fetchQuote();
            }
          }}>
          {isLoadingQuote ? (
            <ActivityIndicator color={colors.text} />
          ) : (
            <Text style={styles.swapButtonText}>
              {currentQuote ? 'Swap' : 'Get Quote'}
            </Text>
          )}
        </TouchableOpacity>
      </ScrollView>

      {/* Settings Modal */}
      <Modal visible={showSettings} transparent animationType="slide">
        <View style={styles.modalOverlay}>
          <View style={styles.modalContent}>
            <View style={styles.modalHeader}>
              <Text style={styles.modalTitle}>Swap Settings</Text>
              <TouchableOpacity onPress={() => setShowSettings(false)}>
                <Icon name="x" size={24} color={colors.text} />
              </TouchableOpacity>
            </View>

            <Text style={styles.settingLabel}>Slippage Tolerance</Text>
            <View style={styles.slippageOptions}>
              {[0.1, 0.5, 1.0].map(value => (
                <TouchableOpacity
                  key={value}
                  style={[
                    styles.slippageButton,
                    slippage === value && styles.slippageButtonSelected,
                  ]}
                  onPress={() => setSlippage(value)}>
                  <Text style={[
                    styles.slippageText,
                    slippage === value && styles.slippageTextSelected,
                  ]}>
                    {value}%
                  </Text>
                </TouchableOpacity>
              ))}
              <TextInput
                style={styles.slippageInput}
                placeholder="Custom"
                placeholderTextColor={colors.textMuted}
                keyboardType="decimal-pad"
                value={![0.1, 0.5, 1.0].includes(slippage) ? slippage.toString() : ''}
                onChangeText={text => {
                  const value = parseFloat(text);
                  if (!isNaN(value)) setSlippage(value);
                }}
              />
            </View>
            <Text style={styles.settingHint}>
              Higher slippage increases chance of success but may result in worse rate.
            </Text>
          </View>
        </View>
      </Modal>

      {/* Token Picker Modal */}
      <Modal visible={showTokenPicker} transparent animationType="slide">
        <View style={styles.modalOverlay}>
          <View style={styles.modalContent}>
            <View style={styles.modalHeader}>
              <Text style={styles.modalTitle}>
                {isSelectingFromToken ? 'Swap From' : 'Swap To'}
              </Text>
              <TouchableOpacity onPress={() => setShowTokenPicker(false)}>
                <Icon name="x" size={24} color={colors.text} />
              </TouchableOpacity>
            </View>

            <ScrollView>
              {availableTokens.map(token => (
                <TouchableOpacity
                  key={token.id}
                  style={styles.tokenPickerItem}
                  onPress={() => {
                    if (isSelectingFromToken) {
                      setFromToken(token);
                    } else {
                      setToToken(token);
                    }
                    setShowTokenPicker(false);
                    setCurrentQuote(null);
                  }}>
                  <View style={[styles.tokenIcon, {backgroundColor: token.color}]}>
                    <Text style={styles.tokenIconText}>{token.symbol.substring(0, 2)}</Text>
                  </View>
                  <View style={styles.tokenPickerInfo}>
                    <Text style={styles.tokenPickerSymbol}>{token.symbol}</Text>
                    <Text style={styles.tokenPickerName}>{token.name}</Text>
                  </View>
                  <Text style={styles.tokenPickerBalance}>0.00</Text>
                </TouchableOpacity>
              ))}
            </ScrollView>
          </View>
        </View>
      </Modal>

      {/* Confirmation Modal */}
      <Modal visible={showConfirmation} transparent animationType="slide">
        <View style={styles.modalOverlay}>
          <View style={styles.modalContent}>
            <View style={styles.modalHeader}>
              <Text style={styles.modalTitle}>Confirm Swap</Text>
              <TouchableOpacity onPress={() => setShowConfirmation(false)}>
                <Icon name="x" size={24} color={colors.text} />
              </TouchableOpacity>
            </View>

            {currentQuote && (
              <>
                <View style={styles.confirmSwapPreview}>
                  <View style={styles.confirmTokenAmount}>
                    <Text style={styles.confirmAmount}>{fromAmount}</Text>
                    <Text style={styles.confirmSymbol}>{fromToken.symbol}</Text>
                  </View>
                  <Icon name="arrow-right" size={24} color={colors.textSecondary} />
                  <View style={styles.confirmTokenAmount}>
                    <Text style={styles.confirmAmount}>{formatNumber(currentQuote.toAmount)}</Text>
                    <Text style={styles.confirmSymbol}>{toToken.symbol}</Text>
                  </View>
                </View>

                <View style={styles.confirmDetails}>
                  <View style={styles.confirmRow}>
                    <Text style={styles.confirmLabel}>Rate</Text>
                    <Text style={styles.confirmValue}>
                      1 {fromToken.symbol} = {DEXService.formatRate(currentQuote)} {toToken.symbol}
                    </Text>
                  </View>
                  <View style={styles.confirmRow}>
                    <Text style={styles.confirmLabel}>Price Impact</Text>
                    <Text style={[
                      styles.confirmValue,
                      currentQuote.priceImpact > 1 ? styles.highImpact : styles.lowImpact,
                    ]}>
                      {DEXService.formatPriceImpact(currentQuote)}
                    </Text>
                  </View>
                  <View style={styles.confirmRow}>
                    <Text style={styles.confirmLabel}>Slippage</Text>
                    <Text style={styles.confirmValue}>{slippage}%</Text>
                  </View>
                </View>

                {currentQuote.priceImpact > 1 && (
                  <View style={styles.highImpactWarning}>
                    <Icon name="alert-triangle" size={18} color={colors.warning} />
                    <Text style={styles.highImpactText}>
                      High price impact! Consider smaller trades.
                    </Text>
                  </View>
                )}

                <TouchableOpacity style={styles.confirmButton} onPress={executeSwap}>
                  <Text style={styles.confirmButtonText}>Confirm Swap</Text>
                </TouchableOpacity>
              </>
            )}
          </View>
        </View>
      </Modal>
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
  content: {
    flex: 1,
    padding: theme.spacing.md,
  },
  chainSelector: {
    marginBottom: theme.spacing.md,
  },
  chainButton: {
    alignItems: 'center',
    justifyContent: 'center',
    width: 60,
    height: 60,
    borderRadius: 12,
    backgroundColor: colors.glass,
    marginRight: theme.spacing.sm,
  },
  chainButtonSelected: {
    backgroundColor: `${colors.primary}20`,
    borderWidth: 2,
    borderColor: colors.primary,
  },
  chainIcon: {
    fontSize: 20,
    fontWeight: 'bold',
    color: colors.text,
  },
  chainName: {
    fontSize: 10,
    color: colors.textSecondary,
    marginTop: 4,
  },
  chainNameSelected: {
    color: colors.primary,
  },
  swapCard: {
    backgroundColor: colors.glassStrong,
    borderRadius: theme.borderRadius.xl,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  tokenRow: {
    marginBottom: theme.spacing.sm,
  },
  tokenLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: theme.spacing.xs,
  },
  tokenInputRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  tokenInput: {
    flex: 1,
    fontSize: 28,
    fontWeight: 'bold',
    color: colors.text,
    padding: 0,
  },
  tokenOutputText: {
    flex: 1,
    fontSize: 28,
    fontWeight: 'bold',
    color: colors.text,
  },
  tokenSelector: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    paddingHorizontal: 12,
    paddingVertical: 8,
    borderRadius: 20,
    gap: 6,
  },
  tokenIcon: {
    width: 24,
    height: 24,
    borderRadius: 12,
    alignItems: 'center',
    justifyContent: 'center',
  },
  tokenIconText: {
    fontSize: 12,
    fontWeight: 'bold',
    color: colors.text,
  },
  tokenSymbol: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
  },
  tokenMeta: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginTop: theme.spacing.xs,
  },
  tokenBalance: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  maxButton: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.bold,
    color: colors.primary,
  },
  tokenUsdValue: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginTop: 4,
  },
  swapArrowContainer: {
    alignItems: 'center',
    marginVertical: -8,
    zIndex: 1,
  },
  swapArrow: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: colors.background,
    alignItems: 'center',
    justifyContent: 'center',
    borderWidth: 3,
    borderColor: colors.glassStrong,
  },
  quoteDetails: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  quoteRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: theme.spacing.sm,
  },
  quoteLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  quoteValue: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
  },
  lowImpact: {
    color: colors.success,
  },
  highImpact: {
    color: colors.error,
  },
  errorBanner: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: `${colors.warning}20`,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  errorText: {
    flex: 1,
    fontSize: theme.fontSize.sm,
    color: colors.text,
  },
  transactionResult: {
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.lg,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.md,
  },
  transactionTitle: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginTop: theme.spacing.sm,
  },
  transactionHash: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginTop: theme.spacing.xs,
  },
  explorerLink: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.bold,
    color: colors.primary,
    marginTop: theme.spacing.sm,
  },
  swapButton: {
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
    marginBottom: theme.spacing.xl,
  },
  swapButtonDisabled: {
    backgroundColor: colors.glass,
  },
  swapButtonText: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  modalOverlay: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.7)',
    justifyContent: 'flex-end',
  },
  modalContent: {
    backgroundColor: colors.background,
    borderTopLeftRadius: 20,
    borderTopRightRadius: 20,
    padding: theme.spacing.md,
    maxHeight: '80%',
  },
  modalHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.md,
    paddingBottom: theme.spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.border,
  },
  modalTitle: {
    fontSize: theme.fontSize.xl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  settingLabel: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: theme.spacing.sm,
  },
  slippageOptions: {
    flexDirection: 'row',
    gap: theme.spacing.sm,
    marginBottom: theme.spacing.sm,
  },
  slippageButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 8,
    backgroundColor: colors.glass,
  },
  slippageButtonSelected: {
    backgroundColor: colors.primary,
  },
  slippageText: {
    fontSize: theme.fontSize.md,
    color: colors.text,
  },
  slippageTextSelected: {
    fontWeight: theme.fontWeight.bold,
  },
  slippageInput: {
    flex: 1,
    backgroundColor: colors.glass,
    borderRadius: 8,
    paddingHorizontal: 12,
    paddingVertical: 8,
    color: colors.text,
    fontSize: theme.fontSize.md,
  },
  settingHint: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginTop: theme.spacing.xs,
  },
  tokenPickerItem: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: theme.spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.border,
  },
  tokenPickerInfo: {
    flex: 1,
    marginLeft: theme.spacing.sm,
  },
  tokenPickerSymbol: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  tokenPickerName: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  tokenPickerBalance: {
    fontSize: theme.fontSize.md,
    color: colors.text,
  },
  confirmSwapPreview: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.lg,
    gap: theme.spacing.md,
  },
  confirmTokenAmount: {
    alignItems: 'center',
  },
  confirmAmount: {
    fontSize: 24,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  confirmSymbol: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
    marginTop: 4,
  },
  confirmDetails: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  confirmRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: theme.spacing.xs,
  },
  confirmLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  confirmValue: {
    fontSize: theme.fontSize.sm,
    color: colors.text,
  },
  highImpactWarning: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: `${colors.warning}20`,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  highImpactText: {
    flex: 1,
    fontSize: theme.fontSize.sm,
    color: colors.text,
  },
  confirmButton: {
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  confirmButtonText: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
});
