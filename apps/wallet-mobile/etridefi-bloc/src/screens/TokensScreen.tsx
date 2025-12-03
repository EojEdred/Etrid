import React, {useState} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  TextInput,
  RefreshControl,
  SectionList,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

// Token categories matching iOS Swift app exactly
type TokenCategory = 'all' | 'native' | 'wETR' | 'bridged' | 'stablecoins';

const CATEGORIES: {key: TokenCategory; label: string}[] = [
  {key: 'all', label: 'All'},
  {key: 'native', label: 'Native'},
  {key: 'wETR', label: 'wETR'},
  {key: 'bridged', label: 'Bridged'},
  {key: 'stablecoins', label: 'Stablecoins'},
];

// Sample token data matching iOS Swift app structure
const MY_HOLDINGS = [
  {id: '1', symbol: 'ETR', name: 'ETRID', amount: '5,234.56', usdValue: '$41,876.48', category: 'native'},
  {id: '2', symbol: 'EDSC', name: 'ETRID Stablecoin', amount: '2,500.00', usdValue: '$2,500.00', category: 'stablecoins'},
  {id: '3', symbol: 'wETR', name: 'Wrapped ETRID (ETH)', amount: '100.00', usdValue: '$800.00', category: 'wETR'},
  {id: '4', symbol: 'bSOL', name: 'Bridged SOL', amount: '5.25', usdValue: '$787.50', category: 'bridged', originalChain: 'Solana'},
];

const AVAILABLE_TOKENS = [
  {id: '10', symbol: 'ETR', name: 'ETRID', category: 'native'},
  {id: '11', symbol: 'EDSC', name: 'ETRID Stablecoin', category: 'stablecoins'},
  {id: '12', symbol: 'wETR-ETH', name: 'Wrapped ETRID (Ethereum)', category: 'wETR'},
  {id: '13', symbol: 'wETR-SOL', name: 'Wrapped ETRID (Solana)', category: 'wETR'},
  {id: '14', symbol: 'wETR-BNB', name: 'Wrapped ETRID (BNB Chain)', category: 'wETR'},
  {id: '15', symbol: 'bSOL', name: 'Bridged SOL', category: 'bridged', originalChain: 'Solana'},
  {id: '16', symbol: 'bBNB', name: 'Bridged BNB', category: 'bridged', originalChain: 'BNB Chain'},
  {id: '17', symbol: 'bXRP', name: 'Bridged XRP', category: 'bridged', originalChain: 'XRP Ledger'},
  {id: '18', symbol: 'bTRX', name: 'Bridged TRX', category: 'bridged', originalChain: 'Tron'},
  {id: '19', symbol: 'bADA', name: 'Bridged ADA', category: 'bridged', originalChain: 'Cardano'},
  {id: '20', symbol: 'bXLM', name: 'Bridged XLM', category: 'bridged', originalChain: 'Stellar'},
  {id: '21', symbol: 'bDOGE', name: 'Bridged DOGE', category: 'bridged', originalChain: 'Dogecoin'},
  {id: '22', symbol: 'USDT', name: 'Tether USD', category: 'stablecoins'},
  {id: '23', symbol: 'USDC', name: 'USD Coin', category: 'stablecoins'},
];

export default function TokensScreen({navigation}: any) {
  const [searchText, setSearchText] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<TokenCategory>('all');
  const [refreshing, setRefreshing] = useState(false);

  const onRefresh = async () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 1000);
  };

  const filterTokens = (tokens: any[]) => {
    return tokens.filter(token => {
      const matchesSearch =
        searchText === '' ||
        token.name.toLowerCase().includes(searchText.toLowerCase()) ||
        token.symbol.toLowerCase().includes(searchText.toLowerCase());

      const matchesCategory =
        selectedCategory === 'all' || token.category === selectedCategory;

      return matchesSearch && matchesCategory;
    });
  };

  const filteredHoldings = filterTokens(MY_HOLDINGS);
  const filteredAvailable = filterTokens(AVAILABLE_TOKENS);

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Search Bar */}
      <View style={styles.searchContainer}>
        <Icon name="search" size={18} color={colors.textSecondary} />
        <TextInput
          style={styles.searchInput}
          placeholder="Search tokens"
          placeholderTextColor={colors.textSecondary}
          value={searchText}
          onChangeText={setSearchText}
        />
        {searchText.length > 0 && (
          <TouchableOpacity onPress={() => setSearchText('')}>
            <Icon name="x" size={18} color={colors.textSecondary} />
          </TouchableOpacity>
        )}
      </View>

      {/* Category Filter - Matches iOS Swift exactly */}
      <ScrollView
        horizontal
        showsHorizontalScrollIndicator={false}
        contentContainerStyle={styles.categoriesContainer}>
        {CATEGORIES.map(cat => (
          <TouchableOpacity
            key={cat.key}
            style={[
              styles.categoryChip,
              selectedCategory === cat.key && styles.categoryChipSelected,
            ]}
            onPress={() => setSelectedCategory(cat.key)}>
            <Text
              style={[
                styles.categoryChipText,
                selectedCategory === cat.key && styles.categoryChipTextSelected,
              ]}>
              {cat.label}
            </Text>
          </TouchableOpacity>
        ))}
      </ScrollView>

      <ScrollView
        style={styles.scrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
        }>
        {/* My Holdings Section */}
        {filteredHoldings.length > 0 && (
          <View style={styles.section}>
            <Text style={styles.sectionTitle}>My Holdings</Text>
            {filteredHoldings.map(token => (
              <TokenHoldingRow
                key={token.id}
                token={token}
                onPress={() => navigation.navigate('TokenDetail', {token})}
              />
            ))}
          </View>
        )}

        {/* Available Tokens Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Available Tokens</Text>
          {filteredAvailable.map(token => (
            <AvailableTokenRow
              key={token.id}
              token={token}
              onBuy={() => navigation.navigate('BuyToken', {token})}
            />
          ))}
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function TokenHoldingRow({token, onPress}: {token: any; onPress: () => void}) {
  return (
    <TouchableOpacity style={styles.tokenRow} onPress={onPress}>
      <View style={styles.tokenIcon}>
        <Text style={styles.tokenIconText}>{token.symbol.charAt(0)}</Text>
      </View>
      <View style={styles.tokenInfo}>
        <Text style={styles.tokenName}>{token.name}</Text>
        <View style={styles.tokenMeta}>
          <Text style={styles.tokenSymbol}>{token.symbol}</Text>
          {token.originalChain && (
            <View style={styles.bridgedBadge}>
              <Text style={styles.bridgedBadgeText}>Bridged</Text>
            </View>
          )}
        </View>
      </View>
      <View style={styles.tokenAmounts}>
        <Text style={styles.tokenAmount}>{token.amount}</Text>
        <Text style={styles.tokenUsdValue}>{token.usdValue}</Text>
      </View>
    </TouchableOpacity>
  );
}

function AvailableTokenRow({token, onBuy}: {token: any; onBuy: () => void}) {
  return (
    <View style={styles.tokenRow}>
      <View style={styles.tokenIcon}>
        <Text style={styles.tokenIconText}>{token.symbol.charAt(0)}</Text>
      </View>
      <View style={styles.tokenInfo}>
        <Text style={styles.tokenName}>{token.name}</Text>
        <View style={styles.tokenMeta}>
          <Text style={styles.tokenSymbol}>{token.symbol}</Text>
          {token.originalChain && (
            <Text style={styles.fromChain}>from {token.originalChain}</Text>
          )}
        </View>
      </View>
      <TouchableOpacity style={styles.buyButton} onPress={onBuy}>
        <Text style={styles.buyButtonText}>Buy</Text>
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  searchContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    marginHorizontal: theme.spacing.md,
    marginTop: theme.spacing.sm,
    paddingHorizontal: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    height: 44,
  },
  searchInput: {
    flex: 1,
    marginLeft: theme.spacing.sm,
    color: colors.text,
    fontSize: theme.fontSize.md,
  },
  categoriesContainer: {
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.md,
    gap: 12,
  },
  categoryChip: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    backgroundColor: colors.glass,
    borderRadius: 20,
    marginRight: 8,
  },
  categoryChipSelected: {
    backgroundColor: colors.primary,
  },
  categoryChipText: {
    color: colors.text,
    fontSize: theme.fontSize.sm,
    fontWeight: '500',
  },
  categoryChipTextSelected: {
    color: '#FFFFFF',
    fontWeight: '600',
  },
  scrollView: {
    flex: 1,
  },
  section: {
    paddingHorizontal: theme.spacing.md,
    marginBottom: theme.spacing.lg,
  },
  sectionTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.textSecondary,
    marginBottom: theme.spacing.sm,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  tokenRow: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  tokenIcon: {
    width: 44,
    height: 44,
    borderRadius: 22,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.md,
  },
  tokenIconText: {
    color: '#FFFFFF',
    fontSize: 18,
    fontWeight: '700',
  },
  tokenInfo: {
    flex: 1,
  },
  tokenName: {
    color: colors.text,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    marginBottom: 4,
  },
  tokenMeta: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
  },
  tokenSymbol: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
  },
  bridgedBadge: {
    backgroundColor: `${colors.primary}33`,
    paddingHorizontal: 6,
    paddingVertical: 2,
    borderRadius: 4,
  },
  bridgedBadgeText: {
    color: colors.primary,
    fontSize: 10,
    fontWeight: '600',
  },
  fromChain: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
  },
  tokenAmounts: {
    alignItems: 'flex-end',
  },
  tokenAmount: {
    color: colors.text,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    marginBottom: 4,
  },
  tokenUsdValue: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
  },
  buyButton: {
    backgroundColor: colors.primary,
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 8,
  },
  buyButtonText: {
    color: '#FFFFFF',
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
  },
});
