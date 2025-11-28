/**
 * Accounts Integration Tests
 * 
 * Prerequisites: Running Ëtrid node with --dev flag
 */

import { EtridClient } from '../../src/client/EtridClient';
import { AccountsWrapper } from '../../src/wrappers/AccountsWrapper';
import { Keyring } from '@polkadot/keyring';

describe('Accounts Integration Tests', () => {
  let client: EtridClient;
  let accounts: AccountsWrapper;
  let alice: any;
  let bob: any;
  
  beforeAll(async () => {
    // Connect to local dev node
    client = new EtridClient('ws://127.0.0.1:9944');
    accounts = new AccountsWrapper(client.api);
    
    // Wait for connection
    await client.api.isReady;
    
    // Create test accounts
    const keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
  });
  
  afterAll(() => {
    if (client) {
      client.close();
    }
  });
  
  describe('Balance Operations', () => {
    it('should query account balance', async () => {
      const balance = await accounts.getBalance(alice.address);
      
      expect(balance).toBeDefined();
      expect(balance.free).toBeGreaterThan(0n);
      expect(typeof balance.free).toBe('bigint');
      expect(typeof balance.reserved).toBe('bigint');
    });
    
    it('should query nonce', async () => {
      const nonce = await accounts.getNonce(alice.address);
      
      expect(typeof nonce).toBe('number');
      expect(nonce).toBeGreaterThanOrEqual(0);
    });
  });
  
  describe('Transfers', () => {
    it('should transfer funds between accounts', async () => {
      // Get initial balances
      const bobInitial = await accounts.getBalance(bob.address);
      
      // Transfer amount
      const amount = 10n * 10n**18n; // 10 ÉTR
      
      // Send transfer
      const txHash = await accounts.transfer(
        alice,
        bob.address,
        amount
      );
      
      expect(txHash).toBeDefined();
      expect(typeof txHash).toBe('string');
      
      // Wait a bit for block finalization
      await new Promise(resolve => setTimeout(resolve, 6000));
      
      // Check Bob's new balance
      const bobFinal = await accounts.getBalance(bob.address);
      expect(bobFinal.free).toBeGreaterThan(bobInitial.free);
    }, 30000);
    
    it('should transfer with memo', async () => {
      const amount = 5n * 10n**18n;
      const memo = 'Test payment';
      
      const txHash = await accounts.transferWithMemo(
        alice,
        bob.address,
        amount,
        memo
      );
      
      expect(txHash).toBeDefined();
    });
  });
  
  describe('Batch Operations', () => {
    it('should perform batch transfers', async () => {
      const charlie = new Keyring({ type: 'sr25519' })
        .addFromUri('//Charlie');
      
      const recipients = [
        { address: bob.address, amount: 1n * 10n**18n },
        { address: charlie.address, amount: 2n * 10n**18n },
      ];
      
      const txHash = await accounts.batchTransfer(alice, recipients);
      
      expect(txHash).toBeDefined();
    }, 30000);
  });
  
  describe('Account Information', () => {
    it('should get account info', async () => {
      const info = await accounts.getAccountInfo(alice.address);
      
      expect(info).toBeDefined();
      expect(info.nonce).toBeGreaterThanOrEqual(0);
      expect(info.consumers).toBeGreaterThanOrEqual(0);
      expect(info.providers).toBeGreaterThanOrEqual(0);
    });
  });
  
  describe('Existential Deposit', () => {
    it('should get existential deposit', async () => {
      const ed = await accounts.getExistentialDeposit();
      
      expect(ed).toBeGreaterThan(0n);
      expect(typeof ed).toBe('bigint');
    });
    
    it('should check if account exists', async () => {
      const exists = await accounts.accountExists(alice.address);
      expect(exists).toBe(true);
      
      // Non-existent account
      const fake = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
      const doesNotExist = await accounts.accountExists(fake);
      expect(doesNotExist).toBe(false);
    });
  });
});
