/**
 * Tests for TransactionBuilder
 */

import { describe, it, expect, beforeAll, afterAll } from '@jest/globals';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { TransactionBuilder } from '../src/builders/TransactionBuilder';
import { InvalidAmountError } from '../src/errors/EtridErrors';

describe('TransactionBuilder', () => {
  let api: ApiPromise;
  let keyring: Keyring;
  let alice: any;

  beforeAll(async () => {
    // Connect to local node or mock
    const provider = new WsProvider('ws://127.0.0.1:9944');
    api = await ApiPromise.create({ provider });

    keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
  });

  afterAll(async () => {
    await api.disconnect();
  });

  describe('Builder Pattern', () => {
    it('should create a transfer transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
      expect((result as any).extrinsic).toBeDefined();
    });

    it('should create a transfer keep-alive transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.transferKeepAlive('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should throw error for negative transfer amount', () => {
      const builder = new TransactionBuilder(api);

      expect(() => {
        builder.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', -1000n);
      }).toThrow(InvalidAmountError);
    });

    it('should throw error for zero transfer amount', () => {
      const builder = new TransactionBuilder(api);

      expect(() => {
        builder.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 0n);
      }).toThrow(InvalidAmountError);
    });
  });

  describe('Staking Operations', () => {
    it('should create a stake transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.stake('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create a bond additional transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.bondAdditional(500000000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create an unbond transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.unbond(500000000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create a nominate transaction', () => {
      const builder = new TransactionBuilder(api);
      const validators = [
        '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
      ];
      const result = builder.nominate(validators);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should throw error for empty validator list', () => {
      const builder = new TransactionBuilder(api);

      expect(() => {
        builder.nominate([]);
      }).toThrow('Must nominate at least one validator');
    });
  });

  describe('Governance Operations', () => {
    it('should create a vote transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.vote(1, true, 1000000000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create a propose transaction', () => {
      const builder = new TransactionBuilder(api);
      const call = api.tx.system.remark('test');
      const result = builder.propose('Test Proposal', 'This is a test', call);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should throw error for zero vote stake', () => {
      const builder = new TransactionBuilder(api);

      expect(() => {
        builder.vote(1, true, 0n);
      }).toThrow(InvalidAmountError);
    });
  });

  describe('Lightning Channel Operations', () => {
    it('should create an open channel transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.openChannel('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create a close channel transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.closeChannel('channel-123');

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create a channel payment transaction', () => {
      const builder = new TransactionBuilder(api);
      const result = builder.channelPayment('channel-123', 1000000n);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });
  });

  describe('Batch Operations', () => {
    it('should create a batch transaction', () => {
      const builder = new TransactionBuilder(api);
      const calls = [
        api.tx.system.remark('test1'),
        api.tx.system.remark('test2'),
      ];
      const result = builder.batch(calls);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });

    it('should create a batchAll transaction', () => {
      const builder = new TransactionBuilder(api);
      const calls = [
        api.tx.system.remark('test1'),
        api.tx.system.remark('test2'),
      ];
      const result = builder.batchAll(calls);

      expect(result).toBeInstanceOf(TransactionBuilder);
    });
  });

  describe('Transaction Options', () => {
    it('should set nonce', () => {
      const builder = new TransactionBuilder(api);
      const result = builder
        .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n)
        .withNonce(5);

      expect((result as any).options.nonce).toBe(5);
    });

    it('should set tip', () => {
      const builder = new TransactionBuilder(api);
      const result = builder
        .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n)
        .withTip(1000n);

      expect((result as any).options.tip).toBe(1000n);
    });

    it('should throw error for negative tip', () => {
      const builder = new TransactionBuilder(api);

      expect(() => {
        builder.withTip(-1000n);
      }).toThrow(InvalidAmountError);
    });

    it('should set mortality period', () => {
      const builder = new TransactionBuilder(api);
      const result = builder
        .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n)
        .withMortality(128);

      expect((result as any).options.mortality).toBe(128);
    });

    it('should make transaction immortal', () => {
      const builder = new TransactionBuilder(api);
      const result = builder
        .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n)
        .immortal();

      expect((result as any).options.mortality).toBe(0);
    });
  });

  describe('Transaction Estimation', () => {
    it('should estimate transaction fees', async () => {
      const builder = new TransactionBuilder(api);
      builder.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n);

      const fees = await builder.estimateFees(alice.address);

      expect(fees).toBeGreaterThan(0n);
    });

    it('should get transaction length', () => {
      const builder = new TransactionBuilder(api);
      builder.transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n);

      const length = builder.getLength();

      expect(length).toBeGreaterThan(0);
    });
  });

  describe('Builder Chaining', () => {
    it('should chain multiple options', () => {
      const builder = new TransactionBuilder(api);
      const result = builder
        .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n)
        .withNonce(5)
        .withTip(1000n)
        .withMortality(128);

      expect((result as any).options.nonce).toBe(5);
      expect((result as any).options.tip).toBe(1000n);
      expect((result as any).options.mortality).toBe(128);
    });
  });

  describe('Builder Clone', () => {
    it('should clone builder', () => {
      const builder = new TransactionBuilder(api);
      builder
        .transfer('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty', 1000000n)
        .withTip(1000n);

      const cloned = builder.clone();

      expect(cloned).toBeInstanceOf(TransactionBuilder);
      expect((cloned as any).options.tip).toBe(1000n);
    });
  });
});
