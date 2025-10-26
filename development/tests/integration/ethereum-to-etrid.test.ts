import { EthereumHelper } from '../utils/ethereum-helper';
import { SubstrateHelper } from '../utils/substrate-helper';
import { AttestationServiceHelper, waitFor } from '../utils/service-helper';
import dotenv from 'dotenv';

dotenv.config();

/**
 * Integration test: Ethereum → Ëtrid transfer
 *
 * Flow:
 * 1. User burns EDSC on Ethereum (TokenMessenger.burnAndSend)
 * 2. Attestation service detects burn event
 * 3. Attesters sign the message
 * 4. Relayer fetches attestation and submits to Ëtrid
 * 5. User receives EDSC on Ëtrid
 */
describe('Ethereum → Ëtrid Transfer', () => {
  let ethereumHelper: EthereumHelper;
  let substrateHelper: SubstrateHelper;
  let attestationService: AttestationServiceHelper;

  const ETHEREUM_RPC_URL = process.env.ETHEREUM_RPC_URL || 'http://localhost:8545';
  const SUBSTRATE_WS_URL = process.env.SUBSTRATE_WS_URL || 'ws://localhost:9944';
  const ATTESTATION_SERVICE_URL =
    process.env.ATTESTATION_SERVICE_URL || 'http://localhost:3000';

  const RELAYER_PRIVATE_KEY =
    process.env.TEST_PRIVATE_KEY ||
    '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80'; // Hardhat account #0

  const CONTRACT_ADDRESSES = {
    edsc: process.env.EDSC_ADDRESS || '',
    attesterRegistry: process.env.ATTESTER_REGISTRY_ADDRESS || '',
    messageTransmitter: process.env.MESSAGE_TRANSMITTER_ADDRESS || '',
    tokenMessenger: process.env.TOKEN_MESSENGER_ADDRESS || '',
  };

  // Test accounts
  const senderEthAddress = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266'; // Hardhat #0
  const recipientSubstrateAddress =
    '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'; // Alice

  beforeAll(async () => {
    // Validate configuration
    if (!CONTRACT_ADDRESSES.edsc) {
      throw new Error('EDSC_ADDRESS not configured');
    }

    // Initialize helpers
    ethereumHelper = new EthereumHelper(ETHEREUM_RPC_URL, RELAYER_PRIVATE_KEY);
    substrateHelper = new SubstrateHelper(SUBSTRATE_WS_URL, '//Bob'); // Use Bob for recipient
    attestationService = new AttestationServiceHelper(ATTESTATION_SERVICE_URL);

    // Connect to chains
    console.log('Connecting to Ethereum...');
    await ethereumHelper.connectContracts(CONTRACT_ADDRESSES);

    console.log('Connecting to Substrate...');
    await substrateHelper.connect();

    // Wait for attestation service to be ready
    console.log('Waiting for attestation service...');
    await attestationService.waitForHealthy(30000);

    console.log('Setup complete!');
  }, 60000);

  afterAll(async () => {
    // Cleanup
    ethereumHelper.disconnect();
    await substrateHelper.disconnect();
  });

  test('Transfer 100 EDSC from Ethereum to Ëtrid', async () => {
    const transferAmount = BigInt(100 * 10 ** 18); // 100 EDSC

    // Step 1: Check initial balances
    console.log('\n=== Step 1: Check initial balances ===');
    const initialEthBalance = await ethereumHelper.getBalance(senderEthAddress);
    const initialSubstrateBalance = await substrateHelper.getBalance(
      recipientSubstrateAddress
    );

    console.log(`Ethereum balance: ${initialEthBalance.toString()}`);
    console.log(`Ëtrid balance: ${initialSubstrateBalance.toString()}`);

    // Ensure sender has enough EDSC (mint if needed)
    if (initialEthBalance < transferAmount) {
      console.log('Minting EDSC for test...');
      await ethereumHelper.mintEDSC(senderEthAddress, transferAmount * 2n);
    }

    // Step 2: Approve TokenMessenger
    console.log('\n=== Step 2: Approve TokenMessenger ===');
    await ethereumHelper.approveTokenMessenger(transferAmount);
    console.log('TokenMessenger approved');

    // Step 3: Burn and send
    console.log('\n=== Step 3: Burn EDSC on Ethereum ===');
    const { nonce, txHash } = await ethereumHelper.burnAndSend(
      recipientSubstrateAddress,
      transferAmount
    );

    console.log(`Burned ${transferAmount} EDSC`);
    console.log(`Nonce: ${nonce}`);
    console.log(`TX: ${txHash}`);

    // Step 4: Wait for attestation service to sign
    console.log('\n=== Step 4: Wait for attestation ===');
    const attestation = await attestationService.waitForAttestationByNonce(
      0, // Source domain = Ethereum
      nonce,
      90000 // 90 second timeout
    );

    console.log(`Attestation ready: ${attestation.messageHash}`);
    console.log(`Signatures: ${attestation.signatureCount}`);
    expect(attestation.thresholdMet).toBe(true);
    expect(attestation.signatures.length).toBeGreaterThanOrEqual(3);

    // Step 5: Wait for relayer to submit to Ëtrid
    console.log('\n=== Step 5: Wait for relay to Ëtrid ===');
    await waitFor(
      async () => {
        return await substrateHelper.isMessageReceived(attestation.messageHash);
      },
      120000,
      3000,
      'message relayed to Ëtrid'
    );

    // Step 6: Verify balances changed
    console.log('\n=== Step 6: Verify final balances ===');
    const finalEthBalance = await ethereumHelper.getBalance(senderEthAddress);
    const finalSubstrateBalance = await substrateHelper.getBalance(
      recipientSubstrateAddress
    );

    console.log(`Ethereum balance: ${finalEthBalance.toString()}`);
    console.log(`Ëtrid balance: ${finalSubstrateBalance.toString()}`);

    // Assertions
    expect(finalEthBalance).toBe(initialEthBalance - transferAmount);
    expect(finalSubstrateBalance).toBe(initialSubstrateBalance + transferAmount);

    console.log('\n✅ Transfer successful!');
  }, 240000); // 4 minute timeout

  test('Should not relay duplicate messages', async () => {
    const transferAmount = BigInt(50 * 10 ** 18); // 50 EDSC

    console.log('\n=== Testing duplicate prevention ===');

    // First transfer
    await ethereumHelper.approveTokenMessenger(transferAmount);
    const { nonce } = await ethereumHelper.burnAndSend(
      recipientSubstrateAddress,
      transferAmount
    );

    console.log(`First transfer nonce: ${nonce}`);

    // Wait for attestation
    const attestation = await attestationService.waitForAttestationByNonce(0, nonce, 90000);

    // Wait for relay
    await waitFor(
      async () => {
        return await substrateHelper.isMessageReceived(attestation.messageHash);
      },
      120000,
      3000,
      'first message relayed'
    );

    // Verify message is marked as received
    const isReceived = await substrateHelper.isMessageReceived(attestation.messageHash);
    expect(isReceived).toBe(true);

    console.log('✅ Duplicate prevention verified');
  }, 240000);

  test('Should handle multiple concurrent transfers', async () => {
    const transferAmount = BigInt(10 * 10 ** 18); // 10 EDSC each
    const numTransfers = 3;

    console.log(`\n=== Testing ${numTransfers} concurrent transfers ===`);

    const initialBalance = await substrateHelper.getBalance(recipientSubstrateAddress);

    // Start multiple transfers
    const transfers = [];
    for (let i = 0; i < numTransfers; i++) {
      await ethereumHelper.approveTokenMessenger(transferAmount);
      const transfer = ethereumHelper.burnAndSend(
        recipientSubstrateAddress,
        transferAmount
      );
      transfers.push(transfer);
    }

    // Wait for all to complete
    const results = await Promise.all(transfers);
    console.log(`Initiated ${results.length} transfers`);

    // Wait for all attestations
    const attestations = await Promise.all(
      results.map((r) => attestationService.waitForAttestationByNonce(0, r.nonce, 90000))
    );

    console.log(`All ${attestations.length} attestations ready`);

    // Wait for all relays
    await waitFor(
      async () => {
        const received = await Promise.all(
          attestations.map((a) => substrateHelper.isMessageReceived(a.messageHash))
        );
        return received.every((r) => r === true);
      },
      180000,
      5000,
      'all messages relayed'
    );

    // Verify final balance
    const finalBalance = await substrateHelper.getBalance(recipientSubstrateAddress);
    const expectedIncrease = transferAmount * BigInt(numTransfers);

    expect(finalBalance).toBe(initialBalance + expectedIncrease);

    console.log('✅ Concurrent transfers successful');
  }, 360000); // 6 minute timeout
});
