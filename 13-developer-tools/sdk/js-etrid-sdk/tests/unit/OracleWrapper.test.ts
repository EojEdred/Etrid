/**
 * OracleWrapper Unit Tests
 */

import { OracleWrapper } from '../../src/wrappers/OracleWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('OracleWrapper', () => {
  let wrapper: OracleWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new OracleWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('getPrice', () => {
    it('should return current price for asset', async () => {
      const mockPrice = {
        price: (50000n * 10n**18n).toString(),
        timestamp: Date.now(),
        source: 'aggregated',
      };

      mockApi.query = {
        oracle: {
          prices: jest.fn().mockResolvedValue(mockQueryResult(mockPrice)),
        },
      };

      const price = await wrapper.getPrice('BTC/USD');

      expect(price).toBe(50000n * 10n**18n);
    });

    it('should throw error for non-existent price feed', async () => {
      mockApi.query = {
        oracle: {
          prices: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      await expect(wrapper.getPrice('INVALID/USD')).rejects.toThrow();
    });
  });

  describe('getPriceWithMetadata', () => {
    it('should return price with full metadata', async () => {
      const mockPrice = {
        price: (50000n * 10n**18n).toString(),
        timestamp: 1234567890,
        confidence: 95,
        sources: 5,
        deviation: 50,
      };

      mockApi.query = {
        oracle: {
          pricesWithMetadata: jest.fn().mockResolvedValue(mockQueryResult(mockPrice)),
        },
      };

      const priceData = await wrapper.getPriceWithMetadata('BTC/USD');

      expect(priceData.price).toBe(50000n * 10n**18n);
      expect(priceData.confidence).toBe(95);
      expect(priceData.sources).toBe(5);
    });
  });

  describe('getTWAP', () => {
    it('should calculate time-weighted average price', async () => {
      const period = 3600; // 1 hour
      const mockTwap = (49500n * 10n**18n).toString();

      mockApi.query = {
        oracle: {
          twap: jest.fn().mockResolvedValue(mockTwap),
        },
      };

      const twap = await wrapper.getTWAP('BTC/USD', period);

      expect(twap).toBe(49500n * 10n**18n);
    });

    it('should throw error for invalid period', async () => {
      await expect(wrapper.getTWAP('BTC/USD', 0)).rejects.toThrow();
      await expect(wrapper.getTWAP('BTC/USD', -100)).rejects.toThrow();
    });
  });

  describe('submitPrice', () => {
    it('should submit price as oracle provider', async () => {
      const pair = 'BTC/USD';
      const price = 50000n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('PriceSubmitted', [pair, price.toString(), accounts.alice.address]),
        ]));
      });

      mockApi.tx = {
        oracle: {
          submitPrice: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.submitPrice(accounts.alice, pair, price);

      expect(txHash).toBeDefined();
    });

    it('should fail if not authorized oracle', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 1 },
          },
        });
      });

      mockApi.tx = {
        oracle: {
          submitPrice: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'oracle',
        name: 'NotAuthorized',
        docs: ['Only authorized oracles can submit prices'],
      }));

      await expect(
        wrapper.submitPrice(accounts.bob, 'BTC/USD', 50000n * 10n**18n)
      ).rejects.toThrow();
    });
  });

  describe('getSupportedPairs', () => {
    it('should return all supported trading pairs', async () => {
      const mockPairs = [
        'BTC/USD',
        'ETH/USD',
        'BNB/USD',
        'SOL/USD',
        'XRP/USD',
      ];

      mockApi.query = {
        oracle: {
          supportedPairs: jest.fn().mockResolvedValue(mockPairs),
        },
      };

      const pairs = await wrapper.getSupportedPairs();

      expect(pairs).toHaveLength(5);
      expect(pairs).toContain('BTC/USD');
      expect(pairs).toContain('ETH/USD');
    });
  });

  describe('getOracleProviders', () => {
    it('should return active oracle providers', async () => {
      const mockProviders = [
        {
          address: accounts.alice.address,
          reputation: 95,
          submittedPrices: 1000,
          active: true,
        },
        {
          address: accounts.bob.address,
          reputation: 88,
          submittedPrices: 500,
          active: true,
        },
      ];

      mockApi.query = {
        oracle: {
          providers: jest.fn().mockResolvedValue(mockProviders),
        },
      };

      const providers = await wrapper.getOracleProviders();

      expect(providers).toHaveLength(2);
      expect(providers[0].reputation).toBe(95);
    });
  });

  describe('getPriceHistory', () => {
    it('should return historical prices', async () => {
      const mockHistory = [
        { price: '49000000000000000000000', timestamp: 1000 },
        { price: '49500000000000000000000', timestamp: 2000 },
        { price: '50000000000000000000000', timestamp: 3000 },
      ];

      mockApi.query = {
        oracle: {
          priceHistory: jest.fn().mockResolvedValue(mockHistory),
        },
      };

      const history = await wrapper.getPriceHistory('BTC/USD', 3);

      expect(history).toHaveLength(3);
      expect(history[2].price).toBe(50000n * 10n**18n);
    });
  });

  describe('subscribeToPriceUpdates', () => {
    it('should subscribe to price updates', async () => {
      const callback = jest.fn();
      const unsubscribe = jest.fn();

      mockApi.query.oracle.prices = jest.fn((pair, cb) => {
        cb({
          price: (50000n * 10n**18n).toString(),
          timestamp: Date.now(),
        });
        return Promise.resolve(unsubscribe);
      });

      const unsub = await wrapper.subscribeToPriceUpdates('BTC/USD', callback);

      expect(callback).toHaveBeenCalled();
      expect(unsub).toBe(unsubscribe);
    });
  });

  describe('getPriceDeviation', () => {
    it('should calculate price deviation from TWAP', async () => {
      const currentPrice = 51000n * 10n**18n;
      const twap = 50000n * 10n**18n;

      mockApi.query = {
        oracle: {
          prices: jest.fn().mockResolvedValue(mockQueryResult({
            price: currentPrice.toString(),
          })),
          twap: jest.fn().mockResolvedValue(twap.toString()),
        },
      };

      const deviation = await wrapper.getPriceDeviation('BTC/USD', 3600);

      expect(deviation).toBeCloseTo(2, 1); // ~2% deviation
    });
  });

  describe('getAggregatedPrice', () => {
    it('should return aggregated price from multiple sources', async () => {
      const mockAggregated = {
        price: (50000n * 10n**18n).toString(),
        sources: [
          { provider: accounts.alice.address, price: '49900000000000000000000' },
          { provider: accounts.bob.address, price: '50100000000000000000000' },
        ],
        median: (50000n * 10n**18n).toString(),
        mean: (50000n * 10n**18n).toString(),
      };

      mockApi.query = {
        oracle: {
          aggregatedPrice: jest.fn().mockResolvedValue(mockAggregated),
        },
      };

      const aggregated = await wrapper.getAggregatedPrice('BTC/USD');

      expect(aggregated.price).toBe(50000n * 10n**18n);
      expect(aggregated.sources).toHaveLength(2);
      expect(aggregated.median).toBe(50000n * 10n**18n);
    });
  });

  describe('getUpdateFrequency', () => {
    it('should return price update frequency', async () => {
      mockApi.query = {
        oracle: {
          updateFrequency: jest.fn().mockResolvedValue('60'), // 60 seconds
        },
      };

      const frequency = await wrapper.getUpdateFrequency('BTC/USD');

      expect(frequency).toBe(60);
    });
  });

  describe('getLastUpdateTime', () => {
    it('should return last price update timestamp', async () => {
      const mockTime = Date.now() - 30000; // 30 seconds ago

      mockApi.query = {
        oracle: {
          lastUpdate: jest.fn().mockResolvedValue(mockTime.toString()),
        },
      };

      const lastUpdate = await wrapper.getLastUpdateTime('BTC/USD');

      expect(lastUpdate).toBe(mockTime);
    });
  });

  describe('isStale', () => {
    it('should return false for fresh price', async () => {
      const recentTime = Date.now() - 30000; // 30 seconds ago

      mockApi.query = {
        oracle: {
          lastUpdate: jest.fn().mockResolvedValue(recentTime.toString()),
          updateFrequency: jest.fn().mockResolvedValue('60'),
        },
      };

      const isStale = await wrapper.isStale('BTC/USD');

      expect(isStale).toBe(false);
    });

    it('should return true for stale price', async () => {
      const oldTime = Date.now() - 300000; // 5 minutes ago

      mockApi.query = {
        oracle: {
          lastUpdate: jest.fn().mockResolvedValue(oldTime.toString()),
          updateFrequency: jest.fn().mockResolvedValue('60'),
        },
      };

      const isStale = await wrapper.isStale('BTC/USD');

      expect(isStale).toBe(true);
    });
  });

  describe('registerAsOracle', () => {
    it('should register account as oracle provider', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('OracleRegistered', [accounts.alice.address]),
        ]));
      });

      mockApi.tx = {
        oracle: {
          register: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.registerAsOracle(accounts.alice, 1000n * 10n**18n);

      expect(txHash).toBeDefined();
    });
  });

  describe('deregisterOracle', () => {
    it('should deregister oracle provider', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('OracleDeregistered', [accounts.alice.address]),
        ]));
      });

      mockApi.tx = {
        oracle: {
          deregister: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.deregisterOracle(accounts.alice);

      expect(txHash).toBeDefined();
    });
  });

  describe('getOracleRewards', () => {
    it('should return oracle rewards earned', async () => {
      mockApi.query = {
        oracle: {
          rewards: jest.fn().mockResolvedValue((100n * 10n**18n).toString()),
        },
      };

      const rewards = await wrapper.getOracleRewards(accounts.alice.address);

      expect(rewards).toBe(100n * 10n**18n);
    });
  });

  describe('claimOracleRewards', () => {
    it('should claim oracle rewards', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('RewardsClaimed', [accounts.alice.address, '100000000000000000000']),
        ]));
      });

      mockApi.tx = {
        oracle: {
          claimRewards: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.claimOracleRewards(accounts.alice);

      expect(txHash).toBeDefined();
    });
  });
});
