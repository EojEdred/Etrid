/**
 * EtwasmVMWrapper Unit Tests
 */

import {
  EtwasmVMWrapper,
  GAS_CONSTANTS,
} from '../../src/wrappers/EtwasmVMWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('EtwasmVMWrapper', () => {
  let wrapper: EtwasmVMWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new EtwasmVMWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('uploadCode', () => {
    it('should upload WASM bytecode successfully', async () => {
      const codeHash = '0x1234567890abcdef';
      const wasmCode = new Uint8Array([0x00, 0x61, 0x73, 0x6d]);

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('CodeUploaded', [codeHash, '100000']),
        ]));
      });

      mockApi.tx = {
        etwasmVm: {
          uploadCode: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.uploadCode(
        accounts.alice,
        wasmCode,
        GAS_CONSTANTS.DEFAULT_GAS
      );

      expect(result.codeHash).toBe(codeHash);
      expect(result.gasUsed).toBe(100000n);
    });

    it('should fail with invalid WASM', async () => {
      const invalidWasm = new Uint8Array([0xff, 0xff, 0xff, 0xff]);

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
        etwasmVm: {
          uploadCode: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'etwasmVm',
        name: 'InvalidWasm',
        docs: ['WASM bytecode is invalid'],
      }));

      await expect(
        wrapper.uploadCode(accounts.alice, invalidWasm, GAS_CONSTANTS.DEFAULT_GAS)
      ).rejects.toThrow();
    });
  });

  describe('instantiate', () => {
    it('should instantiate contract successfully', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const codeHash = '0xabc123';

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ContractInstantiated', [contractAddress, codeHash]),
        ]));
      });

      mockApi.tx = {
        etwasmVm: {
          instantiate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const address = await wrapper.instantiate(accounts.alice, {
        codeHash,
        args: ['MyToken', 'MTK'],
        value: 0n,
        gasLimit: GAS_CONSTANTS.DEFAULT_GAS,
      });

      expect(address).toBe(contractAddress);
    });
  });

  describe('deployContract', () => {
    it('should deploy contract in one step', async () => {
      const codeHash = '0xcode123';
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const wasmCode = new Uint8Array([0x00, 0x61, 0x73, 0x6d]);

      // Mock upload
      const mockUploadSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('CodeUploaded', [codeHash, '50000']),
        ]));
      });

      // Mock instantiate
      const mockInstantiateSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ContractInstantiated', [contractAddress, codeHash]),
        ]));
      });

      mockApi.tx = {
        etwasmVm: {
          uploadCode: jest.fn(() => ({
            signAndSend: mockUploadSend,
          })),
          instantiate: jest.fn(() => ({
            signAndSend: mockInstantiateSend,
          })),
        },
      };

      const deployment = await wrapper.deployContract(
        accounts.alice,
        wasmCode,
        ['MyToken', 'MTK', 18],
        0n,
        GAS_CONSTANTS.DEFAULT_GAS
      );

      expect(deployment.address).toBe(contractAddress);
      expect(deployment.codeHash).toBe(codeHash);
      expect(deployment.gasUsed).toBe(50000n);
    });
  });

  describe('callContract', () => {
    it('should call contract method successfully', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ContractCalled', [contractAddress, 'transfer', '75000']),
        ]));
      });

      mockApi.tx = {
        etwasmVm: {
          callContract: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.callContract(
        accounts.alice,
        contractAddress,
        'transfer',
        [accounts.bob.address, 100n * 10n**18n],
        0n,
        500_000n
      );

      expect(result.txHash).toBeDefined();
      expect(result.gasUsed).toBe(75000n);
    });

    it('should fail with insufficient gas', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

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
        etwasmVm: {
          callContract: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'etwasmVm',
        name: 'InsufficientGas',
        docs: ['Gas limit too low for contract execution'],
      }));

      await expect(
        wrapper.callContract(
          accounts.alice,
          contractAddress,
          'complexMethod',
          [],
          0n,
          10000n // Too low
        )
      ).rejects.toThrow();
    });
  });

  describe('queryContract', () => {
    it('should query contract state without gas', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const balance = 1000n * 10n**18n;

      mockApi.query = {
        etwasmVm: {
          queryContract: jest.fn().mockResolvedValue({
            result: balance.toString(),
          }),
        },
      };

      const result = await wrapper.queryContract(
        contractAddress,
        'balanceOf',
        [accounts.alice.address]
      );

      expect(result).toBeDefined();
    });
  });

  describe('estimateGas', () => {
    it('should estimate gas usage', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

      mockApi.query = {
        etwasmVm: {
          estimateGas: jest.fn().mockResolvedValue({
            gasUsed: '100000',
          }),
        },
      };

      const estimate = await wrapper.estimateGas(
        contractAddress,
        'transfer',
        [accounts.bob.address, 100n * 10n**18n]
      );

      expect(estimate.estimated).toBe(100000n);
      expect(estimate.withBuffer).toBeGreaterThan(100000n);
      expect(estimate.bufferPercent).toBe(20);
    });

    it('should respect max gas limit', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

      mockApi.query = {
        etwasmVm: {
          estimateGas: jest.fn().mockResolvedValue({
            gasUsed: '2000000', // Higher than TX limit
          }),
        },
      };

      const estimate = await wrapper.estimateGas(
        contractAddress,
        'veryExpensiveMethod',
        []
      );

      expect(estimate.maxPossible).toBe(GAS_CONSTANTS.TX_LIMIT);
    });
  });

  describe('getContractInfo', () => {
    it('should return contract metadata', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const codeHash = '0xcode123';

      const mockContractInfo = {
        codeHash,
        deployer: accounts.alice.address,
        deployedAt: Date.now(),
      };

      mockApi.query = {
        etwasmVm: {
          contractInfo: jest.fn().mockResolvedValue(mockQueryResult(mockContractInfo)),
        },
      };

      const info = await wrapper.getContractInfo(contractAddress);

      expect(info.codeHash).toBe(codeHash);
      expect(info.deployer).toBe(accounts.alice.address);
    });

    it('should return null for non-existent contract', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

      mockApi.query = {
        etwasmVm: {
          contractInfo: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      const info = await wrapper.getContractInfo(contractAddress);

      expect(info).toBeNull();
    });
  });

  describe('getCodeHash', () => {
    it('should return code hash for contract', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const codeHash = '0xcode123';

      mockApi.query = {
        etwasmVm: {
          codeHash: jest.fn().mockResolvedValue(codeHash),
        },
      };

      const hash = await wrapper.getCodeHash(contractAddress);

      expect(hash).toBe(codeHash);
    });
  });

  describe('isContract', () => {
    it('should return true for contract address', async () => {
      const contractAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

      mockApi.query = {
        etwasmVm: {
          contractInfo: jest.fn().mockResolvedValue(mockQueryResult({ codeHash: '0x123' })),
        },
      };

      const isContract = await wrapper.isContract(contractAddress);

      expect(isContract).toBe(true);
    });

    it('should return false for regular address', async () => {
      mockApi.query = {
        etwasmVm: {
          contractInfo: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      const isContract = await wrapper.isContract(accounts.alice.address);

      expect(isContract).toBe(false);
    });
  });
});
