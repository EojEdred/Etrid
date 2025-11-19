import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  Image,
  Dimensions,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

const {width} = Dimensions.get('window');
const ITEM_WIDTH = (width - 48) / 2;

const MOCK_NFTS = [
  {
    id: '1',
    name: 'Cosmic Tiger #42',
    collection: 'Cosmic Cats',
    image: 'https://picsum.photos/400/400?random=1',
    price: '2.5 ÉTR',
    chain: 'FlareChain',
  },
  {
    id: '2',
    name: 'Cyber Punk #128',
    collection: 'Future Punks',
    image: 'https://picsum.photos/400/400?random=2',
    price: '5.0 ÉTR',
    chain: 'FlareChain',
  },
  {
    id: '3',
    name: 'Abstract Art #007',
    collection: 'Modern Art',
    image: 'https://picsum.photos/400/400?random=3',
    price: '1.8 ÉTR',
    chain: 'FlareChain',
  },
  {
    id: '4',
    name: 'Digital Landscape #55',
    collection: 'Landscapes',
    image: 'https://picsum.photos/400/400?random=4',
    price: '3.2 ÉTR',
    chain: 'FlareChain',
  },
];

export default function NFTGalleryScreen({navigation}: any) {
  const [activeTab, setActiveTab] = useState<'owned' | 'created'>('owned');

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.headerTitle}>NFT Gallery</Text>
        <View style={styles.headerIcons}>
          <TouchableOpacity
            onPress={() => navigation.navigate('NFTMarketplace')}>
            <Icon name="shopping-bag" size={24} color={colors.text} />
          </TouchableOpacity>
          <TouchableOpacity>
            <Icon name="plus-circle" size={24} color={colors.text} />
          </TouchableOpacity>
        </View>
      </View>

      {/* Stats */}
      <View style={styles.stats}>
        <StatCard label="Total NFTs" value="12" />
        <StatCard label="Total Value" value="24.5 ÉTR" />
        <StatCard label="Collections" value="5" />
      </View>

      {/* Tabs */}
      <View style={styles.tabs}>
        <TouchableOpacity
          style={[styles.tab, activeTab === 'owned' && styles.tabActive]}
          onPress={() => setActiveTab('owned')}>
          <Text
            style={[
              styles.tabText,
              activeTab === 'owned' && styles.tabTextActive,
            ]}>
            Owned
          </Text>
        </TouchableOpacity>
        <TouchableOpacity
          style={[styles.tab, activeTab === 'created' && styles.tabActive]}
          onPress={() => setActiveTab('created')}>
          <Text
            style={[
              styles.tabText,
              activeTab === 'created' && styles.tabTextActive,
            ]}>
            Created
          </Text>
        </TouchableOpacity>
      </View>

      {/* NFT Grid */}
      <ScrollView style={styles.content}>
        <View style={styles.grid}>
          {MOCK_NFTS.map(nft => (
            <NFTCard
              key={nft.id}
              nft={nft}
              onPress={() =>
                navigation.navigate('NFTDetail', {nftId: nft.id})
              }
            />
          ))}
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function StatCard({label, value}: {label: string; value: string}) {
  return (
    <View style={styles.statCard}>
      <Text style={styles.statValue}>{value}</Text>
      <Text style={styles.statLabel}>{label}</Text>
    </View>
  );
}

function NFTCard({nft, onPress}: any) {
  return (
    <TouchableOpacity style={styles.nftCard} onPress={onPress}>
      <Image source={{uri: nft.image}} style={styles.nftImage} />
      <View style={styles.nftInfo}>
        <Text style={styles.nftName} numberOfLines={1}>
          {nft.name}
        </Text>
        <Text style={styles.nftCollection} numberOfLines={1}>
          {nft.collection}
        </Text>
        <View style={styles.nftPrice}>
          <Icon name="trending-up" size={12} color={colors.success} />
          <Text style={styles.nftPriceText}>{nft.price}</Text>
        </View>
      </View>
    </TouchableOpacity>
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
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  headerIcons: {
    flexDirection: 'row',
    gap: theme.spacing.md,
  },
  stats: {
    flexDirection: 'row',
    paddingHorizontal: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  statCard: {
    flex: 1,
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  statValue: {
    fontSize: theme.fontSize.xl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginBottom: 4,
  },
  statLabel: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  tabs: {
    flexDirection: 'row',
    padding: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  tab: {
    flex: 1,
    paddingVertical: 12,
    alignItems: 'center',
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
  },
  tabActive: {
    backgroundColor: colors.primary,
  },
  tabText: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.textSecondary,
  },
  tabTextActive: {
    color: colors.text,
  },
  content: {
    flex: 1,
  },
  grid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    paddingHorizontal: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  nftCard: {
    width: ITEM_WIDTH,
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    overflow: 'hidden',
    marginBottom: theme.spacing.sm,
  },
  nftImage: {
    width: '100%',
    height: ITEM_WIDTH,
    backgroundColor: colors.surface,
  },
  nftInfo: {
    padding: theme.spacing.sm,
  },
  nftName: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  nftCollection: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
    marginBottom: 8,
  },
  nftPrice: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
  },
  nftPriceText: {
    fontSize: theme.fontSize.xs,
    fontWeight: theme.fontWeight.semibold,
    color: colors.success,
  },
});
