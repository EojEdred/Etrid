import React from 'react';
import { View, Text, StyleSheet, ScrollView, RefreshControl, Dimensions } from 'react-native';
import { usePortfolio } from '../hooks/usePortfolio';
import { LineChart, PieChart } from 'react-native-chart-kit';
import { colors, spacing, typography, borderRadius } from '../theme/theme';

const { width } = Dimensions.get('window');

export default function PortfolioScreen() {
  const { portfolio, loading, refresh } = usePortfolio();

  const chartConfig = {
    backgroundColor: colors.primary,
    backgroundGradientFrom: colors.primary,
    backgroundGradientTo: '#8B7BE7',
    decimalPlaces: 2,
    color: (opacity = 1) => `rgba(255, 255, 255, ${opacity})`,
    labelColor: (opacity = 1) => `rgba(255, 255, 255, ${opacity})`,
    style: { borderRadius: borderRadius.md },
    propsForDots: {
      r: '4',
      strokeWidth: '2',
      stroke: colors.primary,
    },
  };

  // Mock 7-day chart data
  const lineData = {
    labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
    datasets: [
      {
        data: [
          portfolio?.totalValue || 0,
          (portfolio?.totalValue || 0) * 0.95,
          (portfolio?.totalValue || 0) * 0.98,
          (portfolio?.totalValue || 0) * 1.02,
          (portfolio?.totalValue || 0) * 0.99,
          (portfolio?.totalValue || 0) * 1.05,
          portfolio?.totalValue || 0,
        ],
      },
    ],
  };

  const pieData = portfolio?.assets.map((asset, index) => ({
    name: asset.symbol,
    population: asset.allocation,
    color: getChartColor(index),
    legendFontColor: colors.text,
    legendFontSize: 12,
  })) || [];

  return (
    <ScrollView
      style={styles.container}
      refreshControl={<RefreshControl refreshing={loading} onRefresh={refresh} />}
    >
      <View style={styles.header}>
        <Text style={styles.title}>Portfolio</Text>
      </View>

      <View style={styles.totalValueCard}>
        <Text style={styles.totalLabel}>Total Portfolio Value</Text>
        <Text style={styles.totalValue}>{portfolio?.totalValueFormatted || '$0.00'}</Text>
        {portfolio && (
          <View style={styles.changeContainer}>
            <Text style={[styles.changeText, portfolio.change24h >= 0 ? styles.changePositive : styles.changeNegative]}>
              {portfolio.change24h >= 0 ? '+' : ''}{portfolio.change24h.toFixed(2)}
            </Text>
            <Text style={[styles.changePercent, portfolio.change24h >= 0 ? styles.changePositive : styles.changeNegative]}>
              ({portfolio.change24hPercent >= 0 ? '+' : ''}{portfolio.change24hPercent.toFixed(2)}%)
            </Text>
          </View>
        )}
      </View>

      {portfolio && portfolio.totalValue > 0 && (
        <>
          <View style={styles.chartSection}>
            <Text style={styles.sectionTitle}>7-Day Performance</Text>
            <LineChart
              data={lineData}
              width={width - spacing.lg * 2}
              height={220}
              chartConfig={chartConfig}
              bezier
              style={styles.chart}
            />
          </View>

          <View style={styles.chartSection}>
            <Text style={styles.sectionTitle}>Asset Allocation</Text>
            <PieChart
              data={pieData}
              width={width - spacing.lg * 2}
              height={220}
              chartConfig={chartConfig}
              accessor="population"
              backgroundColor="transparent"
              paddingLeft="15"
              absolute
            />
          </View>

          <View style={styles.assetsSection}>
            <Text style={styles.sectionTitle}>Your Assets</Text>
            {portfolio.assets.map(asset => (
              <AssetItem key={asset.symbol} asset={asset} />
            ))}
          </View>
        </>
      )}

      {(!portfolio || portfolio.totalValue === 0) && !loading && (
        <View style={styles.emptyState}>
          <Text style={styles.emptyIcon}>📊</Text>
          <Text style={styles.emptyText}>No assets yet</Text>
          <Text style={styles.emptySubtext}>Your portfolio will appear here</Text>
        </View>
      )}
    </ScrollView>
  );
}

const AssetItem: React.FC<{ asset: any }> = ({ asset }) => (
  <View style={styles.assetItem}>
    <View style={styles.assetLeft}>
      <Text style={styles.assetSymbol}>{asset.symbol}</Text>
      <Text style={styles.assetBalance}>{asset.balanceFormatted}</Text>
    </View>
    <View style={styles.assetRight}>
      <Text style={styles.assetValue}>{asset.usdValueFormatted}</Text>
      <Text style={[styles.assetChange, asset.change24h >= 0 ? styles.changePositive : styles.changeNegative]}>
        {asset.change24h >= 0 ? '+' : ''}{asset.change24h.toFixed(2)}%
      </Text>
    </View>
  </View>
);

const getChartColor = (index: number) => {
  const chartColors = ['#6C5CE7', '#00B894', '#FD79A8', '#FDCB6E', '#74B9FF', '#A29BFE', '#FF7675'];
  return chartColors[index % chartColors.length];
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: colors.background },
  header: { padding: spacing.lg, paddingTop: spacing.xl },
  title: { ...typography.h1, color: colors.text },
  totalValueCard: { backgroundColor: colors.surface, borderRadius: borderRadius.lg, padding: spacing.lg, marginHorizontal: spacing.lg, marginBottom: spacing.lg },
  totalLabel: { ...typography.bodySmall, color: colors.textSecondary, marginBottom: spacing.xs },
  totalValue: { ...typography.h1, fontSize: 36, color: colors.text, marginBottom: spacing.xs },
  changeContainer: { flexDirection: 'row', alignItems: 'center' },
  changeText: { ...typography.body, fontWeight: '600', marginRight: spacing.xs },
  changePercent: { ...typography.bodySmall },
  changePositive: { color: colors.success },
  changeNegative: { color: colors.error },
  chartSection: { paddingHorizontal: spacing.lg, marginBottom: spacing.xl },
  sectionTitle: { ...typography.h3, color: colors.text, marginBottom: spacing.md },
  chart: { borderRadius: borderRadius.md },
  assetsSection: { paddingHorizontal: spacing.lg, marginBottom: spacing.xl },
  assetItem: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', backgroundColor: colors.surface, borderRadius: borderRadius.md, padding: spacing.md, marginBottom: spacing.sm },
  assetLeft: {},
  assetSymbol: { ...typography.h3, color: colors.text, marginBottom: spacing.xs / 2 },
  assetBalance: { ...typography.bodySmall, color: colors.textSecondary },
  assetRight: { alignItems: 'flex-end' },
  assetValue: { ...typography.body, color: colors.text, fontWeight: '600', marginBottom: spacing.xs / 2 },
  assetChange: { ...typography.bodySmall },
  emptyState: { paddingVertical: spacing.xxl * 2, alignItems: 'center' },
  emptyIcon: { fontSize: 80, marginBottom: spacing.lg },
  emptyText: { ...typography.h2, color: colors.text, marginBottom: spacing.xs },
  emptySubtext: { ...typography.body, color: colors.textSecondary },
});
