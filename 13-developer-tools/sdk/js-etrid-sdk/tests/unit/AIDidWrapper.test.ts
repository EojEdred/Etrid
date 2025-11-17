/**
 * AIDidWrapper Unit Tests
 */

import {
  AIDidWrapper,
  AIType,
  ReputationTier,
} from '../../src/wrappers/AIDidWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('AIDidWrapper', () => {
  let wrapper: AIDidWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new AIDidWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('registerAI', () => {
    it('should register new AI successfully', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('AIRegistered', [
            aiDid,
            accounts.alice.address,
            AIType.LLM,
          ]),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          registerAI: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.registerAI(
        accounts.alice,
        'TestAI',
        AIType.LLM,
        'https://api.testai.com',
        { version: '1.0', capabilities: ['text', 'code'] }
      );

      expect(result.did).toBe(aiDid);
    });

    it('should fail with duplicate name', async () => {
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
        aiDid: {
          registerAI: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'aiDid',
        name: 'AIAlreadyRegistered',
        docs: ['AI with this name already exists'],
      }));

      await expect(
        wrapper.registerAI(
          accounts.alice,
          'ExistingAI',
          AIType.LLM,
          'https://api.test.com',
          {}
        )
      ).rejects.toThrow();
    });
  });

  describe('getAIProfile', () => {
    it('should return AI profile', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockProfile = {
        name: 'TestAI',
        type: AIType.LLM,
        owner: accounts.alice.address,
        apiEndpoint: 'https://api.testai.com',
        metadata: { version: '1.0' },
        reputation: 750,
        tier: ReputationTier.Silver,
        active: true,
        registeredAt: Date.now(),
      };

      mockApi.query = {
        aiDid: {
          aiProfiles: jest.fn().mockResolvedValue(mockQueryResult(mockProfile)),
        },
      };

      const profile = await wrapper.getAIProfile(aiDid);

      expect(profile).toBeDefined();
      expect(profile?.name).toBe('TestAI');
      expect(profile?.type).toBe(AIType.LLM);
      expect(profile?.reputation).toBe(750);
    });

    it('should return null for non-existent AI', async () => {
      mockApi.query = {
        aiDid: {
          aiProfiles: jest.fn().mockResolvedValue(mockQueryResult(null)),
        },
      };

      const profile = await wrapper.getAIProfile('did:etrid:ai:nonexistent');

      expect(profile).toBeNull();
    });
  });

  describe('updateAIMetadata', () => {
    it('should update AI metadata', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const newMetadata = { version: '2.0', features: ['chat', 'completion'] };

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('AIMetadataUpdated', [aiDid]),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          updateMetadata: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.updateAIMetadata(
        accounts.alice,
        aiDid,
        newMetadata
      );

      expect(txHash).toBeDefined();
    });
  });

  describe('updateReputation', () => {
    it('should update AI reputation', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const points = 50;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ReputationUpdated', [aiDid, points]),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          updateReputation: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.updateReputation(
        accounts.alice,
        aiDid,
        points
      );

      expect(txHash).toBeDefined();
    });

    it('should prevent unauthorized reputation updates', async () => {
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
        aiDid: {
          updateReputation: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'aiDid',
        name: 'NotAuthorized',
        docs: ['Only AI owner can update reputation'],
      }));

      await expect(
        wrapper.updateReputation(
          accounts.bob,
          'did:etrid:ai:test123',
          50
        )
      ).rejects.toThrow();
    });
  });

  describe('getReputation', () => {
    it('should return AI reputation score', async () => {
      const aiDid = 'did:etrid:ai:test123';

      mockApi.query = {
        aiDid: {
          reputation: jest.fn().mockResolvedValue('850'),
        },
      };

      const reputation = await wrapper.getReputation(aiDid);

      expect(reputation).toBe(850);
    });
  });

  describe('getReputationTier', () => {
    it('should return Bronze tier for low reputation', async () => {
      const tier = await wrapper.getReputationTier(250);
      expect(tier).toBe(ReputationTier.Bronze);
    });

    it('should return Silver tier for medium reputation', async () => {
      const tier = await wrapper.getReputationTier(750);
      expect(tier).toBe(ReputationTier.Silver);
    });

    it('should return Gold tier for high reputation', async () => {
      const tier = await wrapper.getReputationTier(1500);
      expect(tier).toBe(ReputationTier.Gold);
    });

    it('should return Platinum tier for top reputation', async () => {
      const tier = await wrapper.getReputationTier(3000);
      expect(tier).toBe(ReputationTier.Platinum);
    });
  });

  describe('grantPermission', () => {
    it('should grant permission to operator', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('PermissionGranted', [aiDid, accounts.bob.address, 'read']),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          grantPermission: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.grantPermission(
        accounts.alice,
        aiDid,
        accounts.bob.address,
        'read'
      );

      expect(txHash).toBeDefined();
    });
  });

  describe('revokePermission', () => {
    it('should revoke permission from operator', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('PermissionRevoked', [aiDid, accounts.bob.address]),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          revokePermission: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.revokePermission(
        accounts.alice,
        aiDid,
        accounts.bob.address
      );

      expect(txHash).toBeDefined();
    });
  });

  describe('hasPermission', () => {
    it('should return true for authorized operator', async () => {
      const aiDid = 'did:etrid:ai:test123';

      mockApi.query = {
        aiDid: {
          permissions: jest.fn().mockResolvedValue(true),
        },
      };

      const hasPermission = await wrapper.hasPermission(
        aiDid,
        accounts.bob.address,
        'read'
      );

      expect(hasPermission).toBe(true);
    });

    it('should return false for unauthorized operator', async () => {
      mockApi.query = {
        aiDid: {
          permissions: jest.fn().mockResolvedValue(false),
        },
      };

      const hasPermission = await wrapper.hasPermission(
        'did:etrid:ai:test123',
        accounts.charlie.address,
        'write'
      );

      expect(hasPermission).toBe(false);
    });
  });

  describe('getPermissions', () => {
    it('should return all permissions for AI', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockPermissions = [
        { operator: accounts.bob.address, permission: 'read' },
        { operator: accounts.charlie.address, permission: 'execute' },
      ];

      mockApi.query = {
        aiDid: {
          allPermissions: jest.fn().mockResolvedValue(mockPermissions),
        },
      };

      const permissions = await wrapper.getPermissions(aiDid);

      expect(permissions).toHaveLength(2);
      expect(permissions[0].operator).toBe(accounts.bob.address);
    });
  });

  describe('deactivateAI', () => {
    it('should deactivate AI', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('AIDeactivated', [aiDid]),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          deactivate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.deactivateAI(accounts.alice, aiDid);

      expect(txHash).toBeDefined();
    });
  });

  describe('reactivateAI', () => {
    it('should reactivate AI', async () => {
      const aiDid = 'did:etrid:ai:test123';
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('AIReactivated', [aiDid]),
        ]));
      });

      mockApi.tx = {
        aiDid: {
          reactivate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.reactivateAI(accounts.alice, aiDid);

      expect(txHash).toBeDefined();
    });
  });

  describe('getAllAIs', () => {
    it('should return all registered AIs', async () => {
      const mockAIs = [
        {
          did: 'did:etrid:ai:test1',
          name: 'AI1',
          type: AIType.LLM,
          owner: accounts.alice.address,
        },
        {
          did: 'did:etrid:ai:test2',
          name: 'AI2',
          type: AIType.CV,
          owner: accounts.bob.address,
        },
      ];

      mockApi.query = {
        aiDid: {
          allAIs: jest.fn().mockResolvedValue(mockAIs),
        },
      };

      const allAIs = await wrapper.getAllAIs();

      expect(allAIs).toHaveLength(2);
      expect(allAIs[0].name).toBe('AI1');
    });
  });

  describe('getAIsByOwner', () => {
    it('should return AIs owned by address', async () => {
      const mockAIs = [
        {
          did: 'did:etrid:ai:test1',
          name: 'AI1',
          type: AIType.LLM,
        },
      ];

      mockApi.query = {
        aiDid: {
          aisByOwner: jest.fn().mockResolvedValue(mockAIs),
        },
      };

      const ownedAIs = await wrapper.getAIsByOwner(accounts.alice.address);

      expect(ownedAIs).toHaveLength(1);
      expect(ownedAIs[0].name).toBe('AI1');
    });
  });

  describe('getAIsByType', () => {
    it('should return AIs of specific type', async () => {
      const mockAIs = [
        {
          did: 'did:etrid:ai:test1',
          name: 'AI1',
          type: AIType.LLM,
        },
        {
          did: 'did:etrid:ai:test2',
          name: 'AI2',
          type: AIType.LLM,
        },
      ];

      mockApi.query = {
        aiDid: {
          aisByType: jest.fn().mockResolvedValue(mockAIs),
        },
      };

      const llmAIs = await wrapper.getAIsByType(AIType.LLM);

      expect(llmAIs).toHaveLength(2);
      llmAIs.forEach(ai => {
        expect(ai.type).toBe(AIType.LLM);
      });
    });
  });

  describe('verifyAI', () => {
    it('should verify AI ownership', async () => {
      const aiDid = 'did:etrid:ai:test123';

      mockApi.query = {
        aiDid: {
          aiProfiles: jest.fn().mockResolvedValue(mockQueryResult({
            owner: accounts.alice.address,
            active: true,
          })),
        },
      };

      const isValid = await wrapper.verifyAI(aiDid, accounts.alice.address);

      expect(isValid).toBe(true);
    });

    it('should return false for wrong owner', async () => {
      const aiDid = 'did:etrid:ai:test123';

      mockApi.query = {
        aiDid: {
          aiProfiles: jest.fn().mockResolvedValue(mockQueryResult({
            owner: accounts.alice.address,
            active: true,
          })),
        },
      };

      const isValid = await wrapper.verifyAI(aiDid, accounts.bob.address);

      expect(isValid).toBe(false);
    });

    it('should return false for deactivated AI', async () => {
      const aiDid = 'did:etrid:ai:test123';

      mockApi.query = {
        aiDid: {
          aiProfiles: jest.fn().mockResolvedValue(mockQueryResult({
            owner: accounts.alice.address,
            active: false,
          })),
        },
      };

      const isValid = await wrapper.verifyAI(aiDid, accounts.alice.address);

      expect(isValid).toBe(false);
    });
  });

  describe('getAICount', () => {
    it('should return total AI count', async () => {
      mockApi.query = {
        aiDid: {
          aiCount: jest.fn().mockResolvedValue('42'),
        },
      };

      const count = await wrapper.getAICount();

      expect(count).toBe(42);
    });
  });

  describe('getActiveAICount', () => {
    it('should return active AI count', async () => {
      mockApi.query = {
        aiDid: {
          activeAICount: jest.fn().mockResolvedValue('38'),
        },
      };

      const count = await wrapper.getActiveAICount();

      expect(count).toBe(38);
    });
  });
});
