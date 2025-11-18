# Phase 4 Quick Start Guide
## Ëtrid Mobile Wallet - Advanced Features

This guide shows how to use the Phase 4 services, hooks, and utilities.

---

## 🔧 Using GPU Marketplace

### Search and Rent GPUs

```typescript
import { useGPU } from './src/hooks/useGPU';

function MyComponent() {
  const { gpus, loading, searchGPUs, rentGPU } = useGPU();

  // Search GPUs with filters
  const handleSearch = async () => {
    await searchGPUs({
      minVRAM: 16,
      maxPrice: 5,
      sortBy: 'price',
      sortOrder: 'asc',
    });
  };

  // Rent a GPU
  const handleRent = async (gpuId: string) => {
    const success = await rentGPU(gpuId, 24); // 24 hours
    if (success) {
      alert('GPU rented successfully!');
    }
  };

  return (
    <View>
      {gpus.map(gpu => (
        <GPUCard key={gpu.id} gpu={gpu} onRent={handleRent} />
      ))}
    </View>
  );
}
```

### Register Your GPU (for providers)

```typescript
const { registerGPU } = useGPU();

const handleRegister = async () => {
  const gpuId = await registerGPU({
    model: 'RTX 4090',
    vram: 24,
    computeUnits: 82.6,
    clockSpeed: 2520,
    pricePerHour: '2.5',
    provider: 'MyCompany',
    reputation: 'Gold',
    availability: '24/7',
    uptime: 99.9,
    location: 'US-East',
    attestation: {
      tpmQuote: '0x...',
      benchmarkScore: 95000,
      verified: true,
    },
    available: true,
    minRentalHours: 1,
    maxRentalHours: 720,
  });

  console.log('GPU registered with ID:', gpuId);
};
```

---

## 🌉 Using Hyperledger Bridge

### Bridge Assets to Fabric

```typescript
import { useHyperledger } from './src/hooks/useHyperledger';

function BridgeComponent() {
  const { networks, bridgeToFabric, bridgeHistory } = useHyperledger();

  // Connect to Fabric network
  const handleConnect = async () => {
    await connectToNetwork(
      'MyFabricNetwork',
      'mychannel',
      'Org1',
      {
        certPem: '-----BEGIN CERTIFICATE-----...',
        keyPem: '-----BEGIN PRIVATE KEY-----...',
      }
    );
  };

  // Bridge 100 ËDSC to Fabric
  const handleBridge = async () => {
    const tx = await bridgeToFabric('100', networks[0].id, 'EDSC');

    // Monitor progress
    console.log('Bridge initiated:', tx.txId);
    console.log('Status:', tx.status);
    console.log('Audit trail:', tx.auditTrail);
  };

  return (
    <View>
      <Button title="Bridge to Fabric" onPress={handleBridge} />
      {bridgeHistory.map(tx => (
        <BridgeTransactionCard key={tx.txId} transaction={tx} />
      ))}
    </View>
  );
}
```

---

## ⚡ Using ETH PBC Precompiles

### Wrap/Unwrap ETH

```typescript
import { useETHPBC } from './src/hooks/useETHPBC';

function WrapComponent() {
  const { balance, wrapETH, unwrapETH } = useETHPBC();

  // Wrap 1 ETH
  const handleWrap = async () => {
    const success = await wrapETH('1000000000000000000'); // 1 ETH in wei
    if (success) {
      alert('ETH wrapped successfully!');
    }
  };

  // Unwrap 0.5 wETH
  const handleUnwrap = async () => {
    const success = await unwrapETH('500000000000000000'); // 0.5 ETH in wei
    if (success) {
      alert('wETH unwrapped successfully!');
    }
  };

  return (
    <View>
      <Text>ETH: {balance?.eth}</Text>
      <Text>wETH: {balance?.weth}</Text>
      <Button title="Wrap ETH" onPress={handleWrap} />
      <Button title="Unwrap wETH" onPress={handleUnwrap} />
    </View>
  );
}
```

### Call Precompiles

```typescript
const { getOraclePrice, voteOnProposal, stakeTokens } = useETHPBC();

// Get ETH price from oracle (0x800)
const price = await getOraclePrice('ETH');
console.log('ETH price:', price);

// Vote on governance proposal (0x801)
await voteOnProposal(1, true); // Proposal 1, vote YES

// Stake 1000 ÉTR (0x802)
await stakeTokens('1000000000000000000000'); // 1000 tokens
```

---

## 📊 Using Analytics

### Track User Actions

```typescript
import AnalyticsService from './src/services/AnalyticsService';
import { useAnalytics, useScreenTracking } from './src/hooks/useAnalytics';

function MyScreen() {
  // Auto-track screen view
  useScreenTracking('MyScreen', { source: 'navigation' });

  const { trackButton, trackTransaction } = useAnalytics();

  const handleButtonClick = () => {
    trackButton('SendButton', 'MyScreen', { amount: '10' });
    // ... send logic
  };

  const handleTransaction = async () => {
    try {
      // ... transaction logic
      trackTransaction('send', '10', 'EDSC', true, { recipient: '0x...' });
    } catch (error) {
      trackTransaction('send', '10', 'EDSC', false, { error: error.message });
    }
  };

  return <View>...</View>;
}
```

---

## 🔔 Using Notifications

### Schedule Notifications

```typescript
import { useNotifications } from './src/hooks/useNotifications';

function NotificationComponent() {
  const {
    preferences,
    updatePreferences,
    notifyTransactionConfirmed,
    notifyGPURentalExpiring,
  } = useNotifications();

  // Update preferences
  const handleToggle = () => {
    updatePreferences({
      transactionConfirmed: !preferences.transactionConfirmed,
    });
  };

  // Send notification when transaction confirms
  const handleTxConfirmed = async (txHash: string) => {
    await notifyTransactionConfirmed(txHash, 'send');
  };

  // Send notification when GPU rental is expiring
  const handleRentalExpiring = async (rentalId: string) => {
    await notifyGPURentalExpiring(rentalId, 'RTX 4090', 2); // 2 hours left
  };

  return <View>...</View>;
}
```

### Handle Notification Taps

```typescript
import { useEffect } from 'react';

function App() {
  const { onNotificationResponse } = useNotifications();

  useEffect(() => {
    const subscription = onNotificationResponse((response) => {
      const data = response.notification.request.content.data;

      if (data.type === 'gpu_expiring') {
        navigation.navigate('MyGPURentals', { rentalId: data.rentalId });
      } else if (data.type === 'proposal_ending') {
        navigation.navigate('ProposalDetail', { proposalId: data.proposalId });
      }
    });

    return () => subscription.remove();
  }, []);

  return <View>...</View>;
}
```

---

## 💾 Using Cache Manager

### Cache Data

```typescript
import CacheManager, { CacheKeys, CacheTTLs } from './src/utils/CacheManager';

// Cache balance for 5 minutes
await CacheManager.set(CacheKeys.BALANCE, balanceData, CacheTTLs.BALANCE);

// Get cached balance
const cachedBalance = await CacheManager.get(CacheKeys.BALANCE);

// Get or fetch (auto-cache)
const balance = await CacheManager.getOrFetch(
  CacheKeys.BALANCE,
  () => fetchBalanceFromAPI(),
  CacheTTLs.BALANCE
);

// Invalidate cache after transaction
await CacheManager.invalidatePattern('balance');
```

---

## 📈 Using Performance Monitor

### Track Performance

```typescript
import PerformanceMonitor from './src/utils/PerformanceMonitor';

// Track screen render
function MyScreen() {
  useEffect(() => {
    const endTracking = PerformanceMonitor.trackScreenRender('MyScreen');
    return endTracking;
  }, []);

  return <View>...</View>;
}

// Measure async operation
const result = await PerformanceMonitor.measure(
  'fetch_gpu_list',
  async () => {
    return await GPUService.searchGPUs(filters);
  },
  { filters }
);

// Track API call
const data = await PerformanceMonitor.trackAPICall(
  '/api/gpus',
  'GET',
  () => fetch('/api/gpus')
);

// Get performance report
const report = PerformanceMonitor.getReport();
console.log('Slow operations:', report.slowOperations);
console.log('Average duration:', report.averageDuration);
```

---

## 🖼️ Using Image Optimizer

### Optimize Images

```typescript
import ImageOptimizer from './src/utils/ImageOptimizer';

function ImageComponent({ uri }: { uri: string }) {
  const [localUri, setLocalUri] = useState<string>(uri);

  useEffect(() => {
    const loadImage = async () => {
      const cached = await ImageOptimizer.getCachedImage(uri);
      setLocalUri(cached);
    };
    loadImage();
  }, [uri]);

  return <Image source={{ uri: localUri }} />;
}

// Preload images
await ImageOptimizer.preloadImages([
  'https://example.com/gpu1.jpg',
  'https://example.com/gpu2.jpg',
]);

// Clear cache
await ImageOptimizer.clearCache();

// Get cache stats
const stats = ImageOptimizer.getCacheStats();
console.log('Cached images:', stats.entries);
console.log('Cache size:', stats.totalSizeMB, 'MB');
```

---

## 🔐 Security Best Practices

### Secure Key Management

```typescript
import KeychainService from './src/services/KeychainService';

// Create wallet
const { address, mnemonic } = await KeychainService.createWallet();

// NEVER log or display mnemonic in production!
// console.log('Mnemonic:', mnemonic); // ❌ DON'T DO THIS

// Get keypair for signing
const keypair = await KeychainService.loadKeypair();

// Sign transaction
const signature = await KeychainService.signMessage('Hello');

// Export private key (with user authentication)
const authenticated = await LocalAuthentication.authenticateAsync();
if (authenticated.success) {
  const privateKey = await KeychainService.exportPrivateKey();
  // Show to user for backup
}
```

---

## 🧪 Testing Examples

### Service Tests

```typescript
import GPUService from './src/services/GPUService';

describe('GPUService', () => {
  it('should search GPUs with filters', async () => {
    const gpus = await GPUService.searchGPUs({
      minVRAM: 16,
      maxPrice: 5,
    });

    expect(gpus).toBeDefined();
    expect(gpus.every(gpu => gpu.vram >= 16)).toBe(true);
    expect(gpus.every(gpu => parseFloat(gpu.pricePerHour) <= 5)).toBe(true);
  });

  it('should rent GPU successfully', async () => {
    const rental = await GPUService.rentGPU('gpu-123', 24);

    expect(rental).toBeDefined();
    expect(rental.gpuId).toBe('gpu-123');
    expect(rental.durationHours).toBe(24);
    expect(rental.sshCredentials).toBeDefined();
  });
});
```

### Hook Tests

```typescript
import { renderHook, act } from '@testing-library/react-hooks';
import { useGPU } from './src/hooks/useGPU';

describe('useGPU', () => {
  it('should load GPUs on mount', async () => {
    const { result, waitForNextUpdate } = renderHook(() => useGPU());

    expect(result.current.loading).toBe(true);

    await waitForNextUpdate();

    expect(result.current.loading).toBe(false);
    expect(result.current.gpus).toBeDefined();
  });

  it('should rent GPU', async () => {
    const { result } = renderHook(() => useGPU());

    await act(async () => {
      const success = await result.current.rentGPU('gpu-123', 24);
      expect(success).toBe(true);
    });
  });
});
```

---

## 📱 Navigation Setup

### Add Routes

```typescript
import { createStackNavigator } from '@react-navigation/stack';
import GPUMarketplaceScreen from './src/screens/gpu/GPUMarketplaceScreen';
import GPUDetailScreen from './src/screens/gpu/GPUDetailScreen';
import RentGPUScreen from './src/screens/gpu/RentGPUScreen';
import MyGPURentalsScreen from './src/screens/gpu/MyGPURentalsScreen';
import RegisterGPUScreen from './src/screens/gpu/RegisterGPUScreen';

const Stack = createStackNavigator();

function GPUNavigator() {
  return (
    <Stack.Navigator>
      <Stack.Screen name="GPUMarketplace" component={GPUMarketplaceScreen} />
      <Stack.Screen name="GPUDetail" component={GPUDetailScreen} />
      <Stack.Screen name="RentGPU" component={RentGPUScreen} />
      <Stack.Screen name="MyGPURentals" component={MyGPURentalsScreen} />
      <Stack.Screen name="RegisterGPU" component={RegisterGPUScreen} />
    </Stack.Navigator>
  );
}
```

---

## 🎨 Component Examples

### GPU Card Component

```typescript
import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import { GPUSpec } from './src/services/GPUService';
import ReputationBadge from './src/components/advanced/ReputationBadge';

interface GPUCardProps {
  gpu: GPUSpec;
  onPress: (gpuId: string) => void;
}

export function GPUCard({ gpu, onPress }: GPUCardProps) {
  return (
    <TouchableOpacity
      style={styles.card}
      onPress={() => onPress(gpu.id!)}
    >
      <View style={styles.header}>
        <Text style={styles.model}>{gpu.model}</Text>
        <ReputationBadge reputation={gpu.reputation} />
      </View>

      <Text style={styles.spec}>VRAM: {gpu.vram}GB</Text>
      <Text style={styles.spec}>Compute: {gpu.computeUnits} TFLOPS</Text>
      <Text style={styles.spec}>Location: {gpu.location}</Text>

      <View style={styles.footer}>
        <Text style={styles.price}>{gpu.pricePerHour} ËDSC/hr</Text>
        <View style={[styles.badge, gpu.available ? styles.available : styles.unavailable]}>
          <Text style={styles.badgeText}>
            {gpu.available ? 'Available' : 'In Use'}
          </Text>
        </View>
      </View>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: '#FFF',
    borderRadius: 12,
    padding: 16,
    marginBottom: 12,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 12,
  },
  model: {
    fontSize: 18,
    fontWeight: 'bold',
  },
  spec: {
    fontSize: 14,
    color: '#666',
    marginBottom: 4,
  },
  footer: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginTop: 12,
    paddingTop: 12,
    borderTopWidth: 1,
    borderTopColor: '#F0F0F0',
  },
  price: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#007AFF',
  },
  badge: {
    paddingVertical: 4,
    paddingHorizontal: 12,
    borderRadius: 8,
  },
  available: { backgroundColor: '#34C759' },
  unavailable: { backgroundColor: '#FF3B30' },
  badgeText: {
    fontSize: 12,
    fontWeight: '600',
    color: '#FFF',
  },
});
```

---

## 🚀 Deployment Checklist

### Pre-Production
- [ ] Test all GPU rental flows
- [ ] Test Hyperledger bridge transactions
- [ ] Test all ETH PBC precompiles
- [ ] Verify notification delivery
- [ ] Check cache performance
- [ ] Monitor memory usage
- [ ] Test on iOS and Android
- [ ] Verify analytics tracking
- [ ] Test error handling
- [ ] Security audit

### Production
- [ ] Enable analytics
- [ ] Configure push notifications
- [ ] Set up error reporting (Sentry)
- [ ] Monitor performance metrics
- [ ] Set cache TTLs appropriately
- [ ] Enable crash reporting
- [ ] Configure API endpoints
- [ ] Test with real transactions
- [ ] Deploy to TestFlight/Internal Testing
- [ ] Final QA pass

---

## 📞 Support

For issues or questions:
- GitHub: [etrid/etrid](https://github.com/etrid/etrid)
- Discord: [Ëtrid Community](https://discord.gg/etrid)
- Email: support@etrid.io

---

**Version**: 1.0.0
**Last Updated**: November 18, 2025
**Status**: Production Ready
