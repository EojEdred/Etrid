/**
 * ReserveVaultWrapper Unit Tests
 */

import {
  ReserveVaultWrapper,
  VaultStatus,
} from '../../src/wrappers/ReserveVaultWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('ReserveVaultWrapper', () => {
  let wrapper: ReserveVaultWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new ReserveVaultWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('createVault', () => {
    it('should create new vault successfully', async () => {
      const vaultId = 'vault-123';
      const collateralAmount = 1000n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('VaultCreated', [
            vaultId,
            accounts.alice.address,
            collateralAmount.toString(),
          ]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          createVault: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.createVault(
        accounts.alice,
        collateralAmount
      );

      expect(result.vaultId).toBe(vaultId);
      expect(result.collateral).toBe(collateralAmount);
    });

    it('should fail with insufficient collateral', async () => {
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
        reserveVault: {
          createVault: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'reserveVault',
        name: 'InsufficientCollateral',
        docs: ['Collateral amount below minimum'],
      }));

      await expect(
        wrapper.createVault(accounts.alice, 100n)
      ).rejects.toThrow();
    });
  });

  describe('getVault', () => {
    it('should return vault information', async () => {
      const vaultId = 'vault-123';
      const mockVault = {
        id: vaultId,
        owner: accounts.alice.address,
        collateral: (1000n * 10n**18n).toString(),
        debt: (500n * 10n**18n).toString(),
        status: VaultStatus.Active,
        createdAt: Date.now(),
      };

      mockApi.query = {
        reserveVault: {
          vaults: jest.fn().mockResolvedValue(mockQueryResult(mockVault)),
        },
      };

      const vault = await wrapper.getVault(vaultId);

      expect(vault).toBeDefined();
      expect(vault?.id).toBe(vaultId);
      expect(vault?.collateral).toBe(1000n * 10n**18n);
      expect(vault?.debt).toBe(500n * 10n**18n);
    });

    it('should return null for non-existent vault', async () => {
      mockApi.query = {
        reserveVault: {
          vaults: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      const vault = await wrapper.getVault('nonexistent');

      expect(vault).toBeNull();
    });
  });

  describe('depositCollateral', () => {
    it('should deposit additional collateral', async () => {
      const vaultId = 'vault-123';
      const amount = 500n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('CollateralDeposited', [vaultId, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          depositCollateral: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.depositCollateral(
        accounts.alice,
        vaultId,
        amount
      );

      expect(txHash).toBeDefined();
    });
  });

  describe('withdrawCollateral', () => {
    it('should withdraw collateral', async () => {
      const vaultId = 'vault-123';
      const amount = 200n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('CollateralWithdrawn', [vaultId, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          withdrawCollateral: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.withdrawCollateral(
        accounts.alice,
        vaultId,
        amount
      );

      expect(txHash).toBeDefined();
    });

    it('should fail if withdrawal would undercollateralize', async () => {
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
        reserveVault: {
          withdrawCollateral: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'reserveVault',
        name: 'UndercollateralizedVault',
        docs: ['Withdrawal would make vault unhealthy'],
      }));

      await expect(
        wrapper.withdrawCollateral(accounts.alice, 'vault-123', 1000n * 10n**18n)
      ).rejects.toThrow();
    });
  });

  describe('borrow', () => {
    it('should borrow against collateral', async () => {
      const vaultId = 'vault-123';
      const amount = 300n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('AssetBorrowed', [vaultId, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          borrow: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.borrow(
        accounts.alice,
        vaultId,
        amount
      );

      expect(txHash).toBeDefined();
    });

    it('should fail if borrow exceeds limit', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 3 },
          },
        });
      });

      mockApi.tx = {
        reserveVault: {
          borrow: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'reserveVault',
        name: 'BorrowLimitExceeded',
        docs: ['Borrow amount exceeds collateral ratio'],
      }));

      await expect(
        wrapper.borrow(accounts.alice, 'vault-123', 10000n * 10n**18n)
      ).rejects.toThrow();
    });
  });

  describe('repay', () => {
    it('should repay borrowed amount', async () => {
      const vaultId = 'vault-123';
      const amount = 100n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('DebtRepaid', [vaultId, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          repay: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.repay(
        accounts.alice,
        vaultId,
        amount
      );

      expect(txHash).toBeDefined();
    });
  });

  describe('getCollateralRatio', () => {
    it('should calculate collateral ratio', async () => {
      const vaultId = 'vault-123';
      const collateral = 1000n * 10n**18n;
      const debt = 500n * 10n**18n;

      mockApi.query = {
        reserveVault: {
          vaults: jest.fn().mockResolvedValue(mockQueryResult({
            collateral: collateral.toString(),
            debt: debt.toString(),
          })),
        },
      };

      const ratio = await wrapper.getCollateralRatio(vaultId);

      expect(ratio).toBe(200); // 200% collateralization
    });

    it('should return Infinity for zero debt', async () => {
      mockApi.query = {
        reserveVault: {
          vaults: jest.fn().mockResolvedValue(mockQueryResult({
            collateral: (1000n * 10n**18n).toString(),
            debt: '0',
          })),
        },
      };

      const ratio = await wrapper.getCollateralRatio('vault-123');

      expect(ratio).toBe(Infinity);
    });
  });

  describe('getHealthFactor', () => {
    it('should calculate vault health factor', async () => {
      const vaultId = 'vault-123';

      mockApi.query = {
        reserveVault: {
          healthFactor: jest.fn().mockResolvedValue('150'), // 1.5
        },
      };

      const health = await wrapper.getHealthFactor(vaultId);

      expect(health).toBe(1.5);
    });
  });

  describe('isLiquidatable', () => {
    it('should return false for healthy vault', async () => {
      mockApi.query = {
        reserveVault: {
          healthFactor: jest.fn().mockResolvedValue('150'), // 1.5 > 1.0
        },
      };

      const liquidatable = await wrapper.isLiquidatable('vault-123');

      expect(liquidatable).toBe(false);
    });

    it('should return true for unhealthy vault', async () => {
      mockApi.query = {
        reserveVault: {
          healthFactor: jest.fn().mockResolvedValue('90'), // 0.9 < 1.0
        },
      };

      const liquidatable = await wrapper.isLiquidatable('vault-123');

      expect(liquidatable).toBe(true);
    });
  });

  describe('liquidate', () => {
    it('should liquidate unhealthy vault', async () => {
      const vaultId = 'vault-123';

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('VaultLiquidated', [vaultId, accounts.bob.address]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          liquidate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.liquidate(accounts.bob, vaultId);

      expect(txHash).toBeDefined();
    });

    it('should fail to liquidate healthy vault', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 4 },
          },
        });
      });

      mockApi.tx = {
        reserveVault: {
          liquidate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'reserveVault',
        name: 'VaultHealthy',
        docs: ['Vault is healthy and cannot be liquidated'],
      }));

      await expect(
        wrapper.liquidate(accounts.bob, 'vault-123')
      ).rejects.toThrow();
    });
  });

  describe('getUserVaults', () => {
    it('should return all vaults owned by user', async () => {
      const mockVaults = [
        {
          id: 'vault-1',
          collateral: '1000000000000000000000',
          debt: '500000000000000000000',
          status: VaultStatus.Active,
        },
        {
          id: 'vault-2',
          collateral: '2000000000000000000000',
          debt: '1000000000000000000000',
          status: VaultStatus.Active,
        },
      ];

      mockApi.query = {
        reserveVault: {
          userVaults: jest.fn().mockResolvedValue(mockVaults),
        },
      };

      const vaults = await wrapper.getUserVaults(accounts.alice.address);

      expect(vaults).toHaveLength(2);
      expect(vaults[0].id).toBe('vault-1');
    });

    it('should return empty array for user with no vaults', async () => {
      mockApi.query = {
        reserveVault: {
          userVaults: jest.fn().mockResolvedValue([]),
        },
      };

      const vaults = await wrapper.getUserVaults(accounts.charlie.address);

      expect(vaults).toHaveLength(0);
    });
  });

  describe('getMinimumCollateral', () => {
    it('should return minimum collateral requirement', async () => {
      mockApi.query = {
        reserveVault: {
          minimumCollateral: jest.fn().mockResolvedValue((100n * 10n**18n).toString()),
        },
      };

      const minimum = await wrapper.getMinimumCollateral();

      expect(minimum).toBe(100n * 10n**18n);
    });
  });

  describe('getInterestRate', () => {
    it('should return current interest rate', async () => {
      mockApi.query = {
        reserveVault: {
          interestRate: jest.fn().mockResolvedValue('500'), // 5% = 500 basis points
        },
      };

      const rate = await wrapper.getInterestRate();

      expect(rate).toBe(5);
    });
  });

  describe('getLiquidationPenalty', () => {
    it('should return liquidation penalty rate', async () => {
      mockApi.query = {
        reserveVault: {
          liquidationPenalty: jest.fn().mockResolvedValue('1000'), // 10%
        },
      };

      const penalty = await wrapper.getLiquidationPenalty();

      expect(penalty).toBe(10);
    });
  });

  describe('calculateBorrowLimit', () => {
    it('should calculate maximum borrow amount', async () => {
      const collateral = 1000n * 10n**18n;

      mockApi.query = {
        reserveVault: {
          collateralRatio: jest.fn().mockResolvedValue('150'), // 150% required
        },
      };

      const limit = await wrapper.calculateBorrowLimit(collateral);

      expect(limit).toBe((1000n * 10n**18n * 100n) / 150n);
    });
  });

  describe('getAccruedInterest', () => {
    it('should calculate accrued interest on debt', async () => {
      const vaultId = 'vault-123';

      mockApi.query = {
        reserveVault: {
          accruedInterest: jest.fn().mockResolvedValue((25n * 10n**18n).toString()),
        },
      };

      const interest = await wrapper.getAccruedInterest(vaultId);

      expect(interest).toBe(25n * 10n**18n);
    });
  });

  describe('getTotalDebt', () => {
    it('should return total debt including interest', async () => {
      const vaultId = 'vault-123';
      const principal = 500n * 10n**18n;
      const interest = 25n * 10n**18n;

      mockApi.query = {
        reserveVault: {
          vaults: jest.fn().mockResolvedValue(mockQueryResult({
            debt: principal.toString(),
          })),
          accruedInterest: jest.fn().mockResolvedValue(interest.toString()),
        },
      };

      const totalDebt = await wrapper.getTotalDebt(vaultId);

      expect(totalDebt).toBe(principal + interest);
    });
  });

  describe('getVaultStatistics', () => {
    it('should return vault system statistics', async () => {
      const mockStats = {
        totalVaults: 100,
        activeVaults: 85,
        totalCollateral: (100000n * 10n**18n).toString(),
        totalDebt: (50000n * 10n**18n).toString(),
        averageCollateralRatio: 200,
        liquidatedVaults: 5,
      };

      mockApi.query = {
        reserveVault: {
          statistics: jest.fn().mockResolvedValue(mockStats),
        },
      };

      const stats = await wrapper.getVaultStatistics();

      expect(stats.totalVaults).toBe(100);
      expect(stats.activeVaults).toBe(85);
      expect(stats.totalCollateral).toBe(100000n * 10n**18n);
    });
  });

  describe('closeVault', () => {
    it('should close vault with zero debt', async () => {
      const vaultId = 'vault-123';

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('VaultClosed', [vaultId]),
        ]));
      });

      mockApi.tx = {
        reserveVault: {
          closeVault: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.closeVault(accounts.alice, vaultId);

      expect(txHash).toBeDefined();
    });

    it('should fail to close vault with outstanding debt', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 5 },
          },
        });
      });

      mockApi.tx = {
        reserveVault: {
          closeVault: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'reserveVault',
        name: 'OutstandingDebt',
        docs: ['Cannot close vault with outstanding debt'],
      }));

      await expect(
        wrapper.closeVault(accounts.alice, 'vault-123')
      ).rejects.toThrow();
    });
  });
});
