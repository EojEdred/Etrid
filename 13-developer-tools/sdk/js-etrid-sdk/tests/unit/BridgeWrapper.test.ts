/**
 * BridgeWrapper Unit Tests
 */

import {
  BridgeWrapper,
  SupportedChain,
  BridgeStatus,
} from '../../src/wrappers/BridgeWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('BridgeWrapper', () => {
  let wrapper: BridgeWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new BridgeWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('bridge', () => {
    it('should initiate cross-chain bridge successfully', async () => {
      const transferId = '0xbridge123';
      const amount = 100n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('BridgeInitiated', [
            transferId,
            SupportedChain.Ethereum,
            SupportedChain.BNB,
            amount.toString(),
          ]),
        ]));
      });

      mockApi.tx = {
        bridge: {
          initiateBridge: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.bridge(
        accounts.alice,
        SupportedChain.Ethereum,
        SupportedChain.BNB,
        amount,
        '0xrecipient123'
      );

      expect(result.transferId).toBe(transferId);
      expect(result.amount).toBe(amount);
    });

    it('should fail with unsupported source chain', async () => {
      await expect(
        wrapper.bridge(
          accounts.alice,
          'UNSUPPORTED' as SupportedChain,
          SupportedChain.Ethereum,
          1000n,
          '0xrecipient'
        )
      ).rejects.toThrow();
    });

    it('should fail with unsupported destination chain', async () => {
      await expect(
        wrapper.bridge(
          accounts.alice,
          SupportedChain.Ethereum,
          'UNSUPPORTED' as SupportedChain,
          1000n,
          '0xrecipient'
        )
      ).rejects.toThrow();
    });

    it('should fail with amount below minimum', async () => {
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
        bridge: {
          initiateBridge: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'bridge',
        name: 'AmountBelowMinimum',
        docs: ['Transfer amount below minimum threshold'],
      }));

      await expect(
        wrapper.bridge(
          accounts.alice,
          SupportedChain.Ethereum,
          SupportedChain.BNB,
          100n, // Too low
          '0xrecipient'
        )
      ).rejects.toThrow();
    });
  });

  describe('getTransferStatus', () => {
    it('should return transfer status', async () => {
      const transferId = '0xbridge123';
      const mockStatus = {
        id: transferId,
        from: SupportedChain.Ethereum,
        to: SupportedChain.BNB,
        amount: (100n * 10n**18n).toString(),
        status: BridgeStatus.Confirmed,
        initiatedAt: Date.now(),
        confirmedAt: Date.now() + 60000,
      };

      mockApi.query = {
        bridge: {
          transfers: jest.fn().mockResolvedValue(mockQueryResult(mockStatus)),
        },
      };

      const status = await wrapper.getTransferStatus(transferId);

      expect(status).toBeDefined();
      expect(status?.status).toBe(BridgeStatus.Confirmed);
      expect(status?.from).toBe(SupportedChain.Ethereum);
    });

    it('should return null for non-existent transfer', async () => {
      mockApi.query = {
        bridge: {
          transfers: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      const status = await wrapper.getTransferStatus('0xnonexistent');

      expect(status).toBeNull();
    });
  });

  describe('getSupportedChains', () => {
    it('should return all supported chains', async () => {
      const chains = await wrapper.getSupportedChains();

      expect(chains).toHaveLength(13);
      expect(chains).toContain(SupportedChain.Bitcoin);
      expect(chains).toContain(SupportedChain.Ethereum);
      expect(chains).toContain(SupportedChain.Solana);
    });
  });

  describe('getBridgeFee', () => {
    it('should calculate bridge fee correctly', async () => {
      const amount = 100n * 10n**18n;

      mockApi.query = {
        bridge: {
          feeRate: jest.fn().mockResolvedValue('100'), // 1% = 100 basis points
        },
      };

      const fee = await wrapper.getBridgeFee(
        SupportedChain.Ethereum,
        SupportedChain.BNB,
        amount
      );

      expect(fee).toBe(1n * 10n**18n); // 1% of 100
    });

    it('should return zero for same-chain transfer', async () => {
      const fee = await wrapper.getBridgeFee(
        SupportedChain.Ethereum,
        SupportedChain.Ethereum,
        1000n
      );

      expect(fee).toBe(0n);
    });
  });

  describe('getMinimumBridgeAmount', () => {
    it('should return minimum bridge amount', async () => {
      mockApi.query = {
        bridge: {
          minimumAmount: jest.fn().mockResolvedValue((10n * 10n**18n).toString()),
        },
      };

      const minimum = await wrapper.getMinimumBridgeAmount(
        SupportedChain.Ethereum,
        SupportedChain.BNB
      );

      expect(minimum).toBe(10n * 10n**18n);
    });
  });

  describe('getMaximumBridgeAmount', () => {
    it('should return maximum bridge amount', async () => {
      mockApi.query = {
        bridge: {
          maximumAmount: jest.fn().mockResolvedValue((1000000n * 10n**18n).toString()),
        },
      };

      const maximum = await wrapper.getMaximumBridgeAmount(
        SupportedChain.Ethereum,
        SupportedChain.BNB
      );

      expect(maximum).toBe(1000000n * 10n**18n);
    });
  });

  describe('getUserTransfers', () => {
    it('should return all transfers for user', async () => {
      const mockTransfers = [
        {
          id: '0xtransfer1',
          from: SupportedChain.Ethereum,
          to: SupportedChain.BNB,
          amount: '100000000000000000000',
          status: BridgeStatus.Confirmed,
        },
        {
          id: '0xtransfer2',
          from: SupportedChain.Bitcoin,
          to: SupportedChain.Ethereum,
          amount: '50000000000000000000',
          status: BridgeStatus.Pending,
        },
      ];

      mockApi.query = {
        bridge: {
          userTransfers: jest.fn().mockResolvedValue(mockTransfers),
        },
      };

      const transfers = await wrapper.getUserTransfers(accounts.alice.address);

      expect(transfers).toHaveLength(2);
      expect(transfers[0].id).toBe('0xtransfer1');
      expect(transfers[1].status).toBe(BridgeStatus.Pending);
    });

    it('should return empty array for new user', async () => {
      mockApi.query = {
        bridge: {
          userTransfers: jest.fn().mockResolvedValue([]),
        },
      };

      const transfers = await wrapper.getUserTransfers(accounts.charlie.address);

      expect(transfers).toHaveLength(0);
    });
  });

  describe('getPendingTransfers', () => {
    it('should return only pending transfers', async () => {
      const mockTransfers = [
        {
          id: '0xtransfer1',
          status: BridgeStatus.Pending,
        },
        {
          id: '0xtransfer2',
          status: BridgeStatus.Pending,
        },
      ];

      mockApi.query = {
        bridge: {
          pendingTransfers: jest.fn().mockResolvedValue(mockTransfers),
        },
      };

      const pending = await wrapper.getPendingTransfers();

      expect(pending).toHaveLength(2);
      pending.forEach(transfer => {
        expect(transfer.status).toBe(BridgeStatus.Pending);
      });
    });
  });

  describe('getBridgeStatistics', () => {
    it('should return bridge statistics', async () => {
      const mockStats = {
        totalTransfers: 1000,
        totalVolume: (1000000n * 10n**18n).toString(),
        activeTransfers: 50,
        completedTransfers: 900,
        failedTransfers: 50,
        averageFee: (1n * 10n**18n).toString(),
      };

      mockApi.query = {
        bridge: {
          statistics: jest.fn().mockResolvedValue(mockStats),
        },
      };

      const stats = await wrapper.getBridgeStatistics();

      expect(stats.totalTransfers).toBe(1000);
      expect(stats.totalVolume).toBe(1000000n * 10n**18n);
      expect(stats.completedTransfers).toBe(900);
    });
  });

  describe('estimateBridgeTime', () => {
    it('should estimate bridge time in seconds', async () => {
      mockApi.query = {
        bridge: {
          averageBridgeTime: jest.fn().mockResolvedValue('300'), // 5 minutes
        },
      };

      const estimatedTime = await wrapper.estimateBridgeTime(
        SupportedChain.Ethereum,
        SupportedChain.BNB
      );

      expect(estimatedTime).toBe(300);
    });

    it('should return higher time for slower chains', async () => {
      mockApi.query = {
        bridge: {
          averageBridgeTime: jest.fn().mockResolvedValue('1800'), // 30 minutes
        },
      };

      const estimatedTime = await wrapper.estimateBridgeTime(
        SupportedChain.Bitcoin,
        SupportedChain.Ethereum
      );

      expect(estimatedTime).toBeGreaterThan(600);
    });
  });

  describe('getChainInfo', () => {
    it('should return chain information', async () => {
      const mockChainInfo = {
        name: 'Ethereum',
        symbol: 'ETH',
        decimals: 18,
        blockTime: 12,
        confirmations: 12,
        active: true,
      };

      mockApi.query = {
        bridge: {
          chainInfo: jest.fn().mockResolvedValue(mockChainInfo),
        },
      };

      const info = await wrapper.getChainInfo(SupportedChain.Ethereum);

      expect(info.name).toBe('Ethereum');
      expect(info.symbol).toBe('ETH');
      expect(info.decimals).toBe(18);
    });
  });

  describe('cancelBridge', () => {
    it('should cancel pending bridge transfer', async () => {
      const transferId = '0xbridge123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('BridgeCancelled', [transferId]),
        ]));
      });

      mockApi.tx = {
        bridge: {
          cancel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.cancelBridge(accounts.alice, transferId);

      expect(txHash).toBeDefined();
    });

    it('should fail to cancel confirmed transfer', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 2 },
          },
        });
      });

      mockApi.tx = {
        bridge: {
          cancel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'bridge',
        name: 'CannotCancelConfirmed',
        docs: ['Cannot cancel confirmed transfers'],
      }));

      await expect(
        wrapper.cancelBridge(accounts.alice, '0xconfirmed123')
      ).rejects.toThrow();
    });
  });

  describe('retryBridge', () => {
    it('should retry failed bridge transfer', async () => {
      const transferId = '0xbridge123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('BridgeRetried', [transferId]),
        ]));
      });

      mockApi.tx = {
        bridge: {
          retry: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.retryBridge(accounts.alice, transferId);

      expect(txHash).toBeDefined();
    });
  });

  describe('subscribeToBridgeEvents', () => {
    it('should subscribe to bridge events', async () => {
      const callback = jest.fn();
      const unsubscribe = jest.fn();

      mockApi.query.system.events = jest.fn((cb) => {
        cb([
          {
            event: {
              section: 'bridge',
              method: 'BridgeInitiated',
              data: ['0xtransfer1', 'ETH', 'BNB', '1000000000000000000'],
            },
          },
        ]);
        return Promise.resolve(unsubscribe);
      });

      const unsub = await wrapper.subscribeToBridgeEvents(callback);

      expect(callback).toHaveBeenCalled();
      expect(unsub).toBe(unsubscribe);
    });
  });

  describe('getBridgeHealth', () => {
    it('should return bridge health status', async () => {
      const mockHealth = {
        operational: true,
        activeChains: 13,
        pendingTransfers: 50,
        averageConfirmationTime: 300,
        successRate: 98.5,
      };

      mockApi.query = {
        bridge: {
          health: jest.fn().mockResolvedValue(mockHealth),
        },
      };

      const health = await wrapper.getBridgeHealth();

      expect(health.operational).toBe(true);
      expect(health.activeChains).toBe(13);
      expect(health.successRate).toBeGreaterThan(95);
    });
  });
});
