/**
 * LightningBlocWrapper Unit Tests
 */

import { LightningBlocWrapper, ChannelStatus } from '../../src/wrappers/LightningBlocWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
  assertTxSuccess,
} from '../utils/testHelpers';

describe('LightningBlocWrapper', () => {
  let wrapper: LightningBlocWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new LightningBlocWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('openChannel', () => {
    it('should open channel successfully', async () => {
      const channelId = 'channel_123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ChannelOpened', [channelId, accounts.alice.address, accounts.bob.address]),
        ]));
      });

      mockApi.tx = {
        lightningBloc: {
          openChannel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.openChannel(
        accounts.alice,
        accounts.bob.address,
        100n * 10n**18n,
        100n * 10n**18n,
        86400 * 30
      );

      expect(result).toBe(channelId);
      expect(mockApi.tx.lightningBloc.openChannel).toHaveBeenCalledWith(
        accounts.bob.address,
        (100n * 10n**18n).toString(),
        (100n * 10n**18n).toString(),
        86400 * 30
      );
    });

    it('should fail with insufficient balance', async () => {
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
        lightningBloc: {
          openChannel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'lightningBloc',
        name: 'InsufficientBalance',
        docs: ['Insufficient balance to open channel'],
      }));

      await expect(
        wrapper.openChannel(
          accounts.alice,
          accounts.bob.address,
          1000000n * 10n**18n,
          100n * 10n**18n,
          86400
        )
      ).rejects.toThrow();
    });

    it('should validate deposit amounts', async () => {
      await expect(
        wrapper.openChannel(
          accounts.alice,
          accounts.bob.address,
          0n,
          100n * 10n**18n,
          86400
        )
      ).rejects.toThrow('Deposit amounts must be positive');
    });

    it('should validate duration', async () => {
      await expect(
        wrapper.openChannel(
          accounts.alice,
          accounts.bob.address,
          100n * 10n**18n,
          100n * 10n**18n,
          0
        )
      ).rejects.toThrow('Duration must be positive');
    });
  });

  describe('getChannel', () => {
    it('should return channel details', async () => {
      const channelId = 'channel_123';
      const mockChannelData = {
        participants: [accounts.alice.address, accounts.bob.address],
        balances: [(100n * 10n**18n).toString(), (100n * 10n**18n).toString()],
        totalBalance: (200n * 10n**18n).toString(),
        nonce: '0',
        duration: '86400',
        openedAt: '1000000',
        status: 'Open',
      };

      mockApi.query = {
        lightningBloc: {
          channels: jest.fn().mockResolvedValue(mockQueryResult(mockChannelData)),
        },
      };

      const channel = await wrapper.getChannel(channelId);

      expect(channel).toBeDefined();
      expect(channel?.channelId).toBe(channelId);
      expect(channel?.participants).toHaveLength(2);
      expect(channel?.totalBalance).toBe(200n * 10n**18n);
      expect(channel?.status).toBe(ChannelStatus.Open);
    });

    it('should return null for non-existent channel', async () => {
      mockApi.query = {
        lightningBloc: {
          channels: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      const channel = await wrapper.getChannel('nonexistent');
      expect(channel).toBeNull();
    });
  });

  describe('getChannelBalance', () => {
    it('should return balance breakdown', async () => {
      const channelId = 'channel_123';
      const mockChannelData = {
        balances: [(80n * 10n**18n).toString(), (120n * 10n**18n).toString()],
        totalBalance: (200n * 10n**18n).toString(),
      };

      mockApi.query = {
        lightningBloc: {
          channels: jest.fn().mockResolvedValue(mockQueryResult(mockChannelData)),
        },
      };

      const balance = await wrapper.getChannelBalance(channelId);

      expect(balance.myBalance).toBe(80n * 10n**18n);
      expect(balance.theirBalance).toBe(120n * 10n**18n);
      expect(balance.totalBalance).toBe(200n * 10n**18n);
    });
  });

  describe('routePayment', () => {
    it('should find route with single hop', async () => {
      const mockRoute = {
        hops: [
          {
            channelId: 'channel_123',
            from: accounts.alice.address,
            to: accounts.bob.address,
            amount: (10n * 10n**18n).toString(),
            fee: (10n * 10n**15n).toString(), // 0.01 ÉTR
          },
        ],
        totalFee: (10n * 10n**15n).toString(),
        totalAmount: (10n * 10n**18n + 10n * 10n**15n).toString(),
        estimatedTimeMs: '100',
      };

      mockApi.query = {
        lightningBloc: {
          routes: jest.fn().mockResolvedValue(mockQueryResult(mockRoute)),
        },
      };

      const route = await wrapper.routePayment(
        accounts.alice,
        accounts.bob.address,
        10n * 10n**18n,
        10
      );

      expect(route.hops).toHaveLength(1);
      expect(route.totalFee).toBe(10n * 10n**15n);
    });

    it('should fail when no route exists', async () => {
      mockApi.query = {
        lightningBloc: {
          routes: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      await expect(
        wrapper.routePayment(
          accounts.alice,
          accounts.charlie.address,
          10n * 10n**18n,
          10
        )
      ).rejects.toThrow('No route found');
    });
  });

  describe('closeChannel', () => {
    it('should close channel cooperatively', async () => {
      const channelId = 'channel_123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ChannelClosed', [channelId]),
        ]));
      });

      mockApi.tx = {
        lightningBloc: {
          closeChannel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.closeChannel(accounts.alice, channelId);

      expect(txHash).toBeDefined();
      expect(mockApi.tx.lightningBloc.closeChannel).toHaveBeenCalledWith(channelId);
    });
  });

  describe('estimateRoutingFee', () => {
    it('should calculate fees correctly', async () => {
      const channelId = 'channel_123';
      const amount = 100n * 10n**18n;

      const mockFeeEstimate = {
        baseFee: (1n * 10n**16n).toString(), // 0.01 ÉTR
        proportionalFee: (1n * 10n**17n).toString(), // 0.1 ÉTR
        totalFee: (11n * 10n**16n).toString(), // 0.11 ÉTR
      };

      mockApi.query = {
        lightningBloc: {
          feeEstimates: jest.fn().mockResolvedValue(mockQueryResult(mockFeeEstimate)),
        },
      };

      const estimate = await wrapper.estimateRoutingFee(channelId, amount);

      expect(estimate.baseFee).toBe(1n * 10n**16n);
      expect(estimate.proportionalFee).toBe(1n * 10n**17n);
      expect(estimate.totalFee).toBe(11n * 10n**16n);
    });
  });

  describe('getMyChannels', () => {
    it('should return user channels', async () => {
      const mockChannels = [
        {
          channelId: 'channel_1',
          participants: [accounts.alice.address, accounts.bob.address],
          totalBalance: (200n * 10n**18n).toString(),
          status: 'Open',
        },
        {
          channelId: 'channel_2',
          participants: [accounts.alice.address, accounts.charlie.address],
          totalBalance: (150n * 10n**18n).toString(),
          status: 'Open',
        },
      ];

      mockApi.query = {
        lightningBloc: {
          userChannels: jest.fn().mockResolvedValue(mockChannels),
        },
      };

      const channels = await wrapper.getMyChannels(accounts.alice.address);

      expect(channels).toHaveLength(2);
      expect(channels[0].channelId).toBe('channel_1');
      expect(channels[1].channelId).toBe('channel_2');
    });

    it('should return empty array for new user', async () => {
      mockApi.query = {
        lightningBloc: {
          userChannels: jest.fn().mockResolvedValue([]),
        },
      };

      const channels = await wrapper.getMyChannels(accounts.charlie.address);
      expect(channels).toHaveLength(0);
    });
  });

  describe('updateChannel', () => {
    it('should update channel state', async () => {
      const channelId = 'channel_123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ChannelUpdated', [channelId, '5']),
        ]));
      });

      mockApi.tx = {
        lightningBloc: {
          updateChannel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.updateChannel(
        accounts.alice,
        channelId,
        10n * 10n**18n,
        5,
        new Uint8Array(64)
      );

      expect(result.txHash).toBeDefined();
      expect(result.newNonce).toBe(5);
    });
  });

  describe('forceClose', () => {
    it('should force close with latest state', async () => {
      const channelId = 'channel_123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ChannelForceClose', [channelId]),
        ]));
      });

      mockApi.tx = {
        lightningBloc: {
          forceClose: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.forceClose(
        accounts.alice,
        channelId,
        10,
        [80n * 10n**18n, 120n * 10n**18n],
        new Uint8Array(64)
      );

      expect(txHash).toBeDefined();
    });
  });
});
