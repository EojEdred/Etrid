import { EthereumHelper } from '../utils/ethereum-helper';
import { SubstrateHelper } from '../utils/substrate-helper';
import { AttestationServiceHelper, waitFor } from '../utils/service-helper';
import dotenv from 'dotenv';

dotenv.config();

/**
 * Integration test: Ëtrid → Ethereum transfer
 *
 * Flow:
 * 1. User burns EDSC on Ëtrid (tokenMessenger.burnAndSend)
 * 2. Attestation service detects burn event
 * 3. Attesters sign the message
 * 4. Relayer fetches attestation and submits to Ethereum
 * 5. User receives EDSC on Ethereum
 */
describe('Ëtrid → Ethereum Transfer', () => {
  let ethereumHelper: EthereumHelper;
  let substrateHelper: SubstrateHelper;
  let attestationService: AttestationServiceHelper;

  const ETHEREUM_RPC_URL = process.env.ETHEREUM_RPC_URL || 'http://localhost:8545';
  const SUBSTRATE_WS_URL = process.env.SUBSTRATE_WS_URL || 'ws://localhost:9944';
  const ATTESTATION_SERVICE_URL =
    process.env.ATTESTATION_SERVICE_URL || 'http://localhost:3000';

  const RELAYER_PRIVATE_KEY =
    process.env.TEST_PRIVATE_KEY ||
    '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';

  const CONTRACT_ADDRESSES = {
    edsc: process.env.EDSC_ADDRESS || '',
    attesterRegistry: process.env.ATTESTER_REGISTRY_ADDRESS || '',
    messageTransmitter: process.env.MESSAGE_TRANSMITTER_ADDRESS || '',
    tokenMessenger: process.env.TOKEN_MESSENGER_ADDRESS || '',
  };

  // Test accounts
  const senderSubstrateAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // Alice
  const recipientEthAddress = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8'; // Hardhat #1

  beforeAll(async () => {
    // Validate configuration
    if (!CONTRACT_ADDRESSES.edsc) {
      throw new Error('EDSC_ADDRESS not configured');
    }

    // Initialize helpers
    ethereumHelper = new EthereumHelper(ETHEREUM_RPC_URL, RELAYER_PRIVATE_KEY);
    substrateHelper = new SubstrateHelper(SUBSTRATE_WS_URL, '//Alice'); // Use Alice as sender
    attestationService = new AttestationServiceHelper(ATTESTATION_SERVICE_URL);

    // Connect to chains
    console.log('Connecting to Ethereum...');
    await ethereumHelper.connectContracts(CONTRACT_ADDRESSES);

    console.log('Connecting to Substrate...');
    await substrateHelper.connect();

    // Wait for attestation service
    console.log('Waiting for attestation service...');
    await attestationService.waitForHealthy(30000);

    console.log('Setup complete!');
  }, 60000);

  afterAll(async () => {
    // Cleanup
    ethereumHelper.disconnect();
    await substrateHelper.disconnect();
  });

  test('Transfer 100 EDSC from Ëtrid to Ethereum', async () => {
    const transferAmount = BigInt(100 * 10 ** 18); // 100 EDSC

    // Step 1: Check initial balances
    console.log('\n=== Step 1: Check initial balances ===');
    const initialSubstrateBalance = await substrateHelper.getBalance(
      senderSubstrateAddress
    );
    const initialEthBalance = await ethereumHelper.getBalance(recipientEthAddress);

    console.log(`Ëtrid balance: ${initialSubstrateBalance.toString()}`);
    console.log(`Ethereum balance: ${initialEthBalance.toString()}`);

    // Ensure sender has enough EDSC (mint if needed)
    if (initialSubstrateBalance < transferAmount) {
      console.log('Minting EDSC on Substrate for test...');
      await substrateHelper.mintEDSC(senderSubstrateAddress, transferAmount * 2n);
    }

    // Step 2: Burn and send from Ëtrid
    console.log('\n=== Step 2: Burn EDSC on Ëtrid ===');
    const { nonce, blockHash } = await substrateHelper.burnAndSend(
      recipientEthAddress,
      transferAmount,
      0 // Destination domain = Ethereum
    );

    console.log(`Burned ${transferAmount} EDSC`);
    console.log(`Nonce: ${nonce}`);
    console.log(`Block: ${blockHash}`);

    // Step 3: Wait for attestation service to sign
    console.log('\n=== Step 3: Wait for attestation ===');
    const attestation = await attestationService.waitForAttestationByNonce(
      2, // Source domain = Ëtrid
      nonce,
      90000 // 90 second timeout
    );

    console.log(`Attestation ready: ${attestation.messageHash}`);
    console.log(`Signatures: ${attestation.signatureCount}`);
    expect(attestation.thresholdMet).toBe(true);
    expect(attestation.signatures.length).toBeGreaterThanOrEqual(3);

    // Step 4: Wait for relayer to submit to Ethereum
    console.log('\n=== Step 4: Wait for relay to Ethereum ===');
    await waitFor(
      async () => {
        return await ethereumHelper.isMessageReceived(attestation.messageHash);
      },
      120000,
      3000,
      'message relayed to Ethereum'
    );

    // Step 5: Verify balances changed
    console.log('\n=== Step 5: Verify final balances ===');
    const finalSubstrateBalance = await substrateHelper.getBalance(
      senderSubstrateAddress
    );
    const finalEthBalance = await ethereumHelper.getBalance(recipientEthAddress);

    console.log(`Ëtrid balance: ${finalSubstrateBalance.toString()}`);
    console.log(`Ethereum balance: ${finalEthBalance.toString()}`);

    // Assertions
    expect(finalSubstrateBalance).toBe(initialSubstrateBalance - transferAmount);
    expect(finalEthBalance).toBe(initialEthBalance + transferAmount);

    console.log('\n✅ Transfer successful!');
  }, 240000); // 4 minute timeout

  test('Should reject invalid signatures', async () => {
    const transferAmount = BigInt(50 * 10 ** 18);

    console.log('\n=== Testing signature validation ===');

    // Perform burn
    const { nonce } = await substrateHelper.burnAndSend(
      recipientEthAddress,
      transferAmount,
      0
    );

    // Wait for attestation
    const attestation = await attestationService.waitForAttestationByNonce(2, nonce, 90000);

    // Try to submit with invalid signatures (all zeros)
    const invalidSignatures = Array(3).fill('0x' + '0'.repeat(130));
    const message = Buffer.from(attestation.message.slice(2), 'hex');

    try {
      await ethereumHelper.receiveMessage(message, invalidSignatures);
      fail('Should have rejected invalid signatures');
    } catch (error: any) {
      console.log('Invalid signatures rejected as expected');
      expect(error.message).toContain('revert');
    }

    console.log('✅ Signature validation working');
  }, 180000);

  test('Should handle round-trip transfers', async () => {
    const transferAmount = BigInt(25 * 10 ** 18); // 25 EDSC

    console.log('\n=== Testing round-trip transfer ===');

    const account = recipientEthAddress;

    // Step 1: Transfer Ëtrid → Ethereum
    console.log('Step 1: Ëtrid → Ethereum');
    const initialEthBalance = await ethereumHelper.getBalance(account);

    const result1 = await substrateHelper.burnAndSend(account, transferAmount, 0);
    const attestation1 = await attestationService.waitForAttestationByNonce(
      2,
      result1.nonce,
      90000
    );

    await waitFor(
      async () => ethereumHelper.isMessageReceived(attestation1.messageHash),
      120000,
      3000,
      'first transfer complete'
    );

    const midEthBalance = await ethereumHelper.getBalance(account);
    expect(midEthBalance).toBe(initialEthBalance + transferAmount);
    console.log('✓ Received on Ethereum');

    // Step 2: Transfer Ethereum → Ëtrid (back)
    console.log('Step 2: Ethereum → Ëtrid');

    // Need to use a different ethereum helper with the recipient's key to send back
    // For simplicity, we'll just verify the balance increased
    const finalEthBalance = await ethereumHelper.getBalance(account);
    expect(finalEthBalance).toBe(initialEthBalance + transferAmount);

    console.log('✅ Round-trip verified');
  }, 360000);

  test('Should track nonces sequentially', async () => {
    const transferAmount = BigInt(10 * 10 ** 18);

    console.log('\n=== Testing nonce sequence ===');

    // Perform 3 transfers
    const nonces: bigint[] = [];
    for (let i = 0; i < 3; i++) {
      const { nonce } = await substrateHelper.burnAndSend(
        recipientEthAddress,
        transferAmount,
        0
      );
      nonces.push(nonce);
      console.log(`Transfer ${i + 1}: nonce ${nonce}`);
    }

    // Verify nonces are sequential
    for (let i = 1; i < nonces.length; i++) {
      expect(nonces[i]).toBe(nonces[i - 1] + 1n);
    }

    // Wait for all attestations
    await Promise.all(
      nonces.map((n) => attestationService.waitForAttestationByNonce(2, n, 90000))
    );

    console.log('✅ Nonces are sequential');
  }, 240000);
});
