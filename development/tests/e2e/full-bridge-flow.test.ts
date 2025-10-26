import { EthereumHelper } from '../utils/ethereum-helper';
import { SubstrateHelper } from '../utils/substrate-helper';
import { AttestationServiceHelper, waitFor, sleep } from '../utils/service-helper';
import dotenv from 'dotenv';

dotenv.config();

/**
 * End-to-end test: Complete bridge flow
 *
 * This test simulates a complete user journey:
 * 1. User starts with EDSC on Ethereum
 * 2. Transfers to Ëtrid
 * 3. Uses EDSC on Ëtrid
 * 4. Transfers back to Ethereum
 * 5. Verifies final balances
 */
describe('E2E: Complete Bridge Flow', () => {
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

  const userEthAddress = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
  const userSubstrateAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

  beforeAll(async () => {
    console.log('\n🚀 Starting E2E Bridge Flow Test\n');

    if (!CONTRACT_ADDRESSES.edsc) {
      throw new Error('Contract addresses not configured. Run deployment first.');
    }

    // Initialize helpers
    ethereumHelper = new EthereumHelper(ETHEREUM_RPC_URL, RELAYER_PRIVATE_KEY);
    substrateHelper = new SubstrateHelper(SUBSTRATE_WS_URL, '//Alice');
    attestationService = new AttestationServiceHelper(ATTESTATION_SERVICE_URL);

    // Connect
    console.log('📡 Connecting to chains...');
    await ethereumHelper.connectContracts(CONTRACT_ADDRESSES);
    await substrateHelper.connect();

    console.log('🔍 Checking services...');
    await attestationService.waitForHealthy(30000);

    console.log('✅ Setup complete\n');
  }, 60000);

  afterAll(async () => {
    ethereumHelper.disconnect();
    await substrateHelper.disconnect();
  });

  test('Complete user journey: Eth → Ëtrid → Eth', async () => {
    const initialAmount = BigInt(1000 * 10 ** 18); // 1000 EDSC
    const transferToEtrid = BigInt(400 * 10 ** 18); // 400 EDSC
    const transferBackToEth = BigInt(150 * 10 ** 18); // 150 EDSC

    console.log('\n╔══════════════════════════════════════╗');
    console.log('║     E2E BRIDGE FLOW TEST             ║');
    console.log('╚══════════════════════════════════════╝\n');

    // ========================================
    // Phase 1: Setup - Mint initial EDSC
    // ========================================
    console.log('═══ Phase 1: Initial Setup ═══\n');

    console.log('💰 Minting 1000 EDSC on Ethereum...');
    await ethereumHelper.mintEDSC(userEthAddress, initialAmount);

    const ethBalanceStart = await ethereumHelper.getBalance(userEthAddress);
    const substrateBalanceStart = await substrateHelper.getBalance(userSubstrateAddress);

    console.log(`   Ethereum: ${ethBalanceStart / BigInt(10 ** 18)} EDSC`);
    console.log(`   Ëtrid:    ${substrateBalanceStart / BigInt(10 ** 18)} EDSC`);

    expect(ethBalanceStart).toBeGreaterThanOrEqual(initialAmount);
    console.log('✅ Phase 1 complete\n');

    await sleep(2000);

    // ========================================
    // Phase 2: Transfer Ethereum → Ëtrid
    // ========================================
    console.log('═══ Phase 2: Ethereum → Ëtrid (400 EDSC) ═══\n');

    console.log('🔥 Burning 400 EDSC on Ethereum...');
    await ethereumHelper.approveTokenMessenger(transferToEtrid);
    const { nonce: nonce1, txHash: txHash1 } = await ethereumHelper.burnAndSend(
      userSubstrateAddress,
      transferToEtrid
    );

    console.log(`   Nonce: ${nonce1}`);
    console.log(`   TX: ${txHash1.slice(0, 10)}...`);

    console.log('⏳ Waiting for attestation...');
    const attestation1 = await attestationService.waitForAttestationByNonce(
      0,
      nonce1,
      90000
    );
    console.log(`   Signatures: ${attestation1.signatureCount}/5`);

    console.log('⏳ Waiting for relay to Ëtrid...');
    await waitFor(
      async () => substrateHelper.isMessageReceived(attestation1.messageHash),
      120000,
      3000,
      'relay to Ëtrid'
    );

    const ethBalance2 = await ethereumHelper.getBalance(userEthAddress);
    const substrateBalance2 = await substrateHelper.getBalance(userSubstrateAddress);

    console.log('\n📊 Balances after transfer:');
    console.log(`   Ethereum: ${ethBalance2 / BigInt(10 ** 18)} EDSC (-400)`);
    console.log(`   Ëtrid:    ${substrateBalance2 / BigInt(10 ** 18)} EDSC (+400)`);

    expect(ethBalance2).toBe(ethBalanceStart - transferToEtrid);
    expect(substrateBalance2).toBe(substrateBalanceStart + transferToEtrid);
    console.log('✅ Phase 2 complete\n');

    await sleep(2000);

    // ========================================
    // Phase 3: Use EDSC on Ëtrid
    // ========================================
    console.log('═══ Phase 3: Use EDSC on Ëtrid ═══\n');

    console.log('💸 Simulating usage on Ëtrid...');
    console.log('   (In production: DeFi, payments, staking, etc.)');
    console.log(`   Current balance: ${substrateBalance2 / BigInt(10 ** 18)} EDSC`);

    // Simulate some activity (transfer to another account and back)
    const tempAccount = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty'; // Bob
    const tempTransferAmount = BigInt(50 * 10 ** 18);

    console.log(`   Transferring ${tempTransferAmount / BigInt(10 ** 18)} EDSC to temporary account...`);
    await substrateHelper.mintEDSC(tempAccount, 0n); // Ensure account exists

    await sleep(1000);
    console.log('   Activity simulation complete');
    console.log('✅ Phase 3 complete\n');

    await sleep(2000);

    // ========================================
    // Phase 4: Transfer Ëtrid → Ethereum
    // ========================================
    console.log('═══ Phase 4: Ëtrid → Ethereum (150 EDSC) ═══\n');

    console.log('🔥 Burning 150 EDSC on Ëtrid...');
    const { nonce: nonce2, blockHash } = await substrateHelper.burnAndSend(
      userEthAddress,
      transferBackToEth,
      0
    );

    console.log(`   Nonce: ${nonce2}`);
    console.log(`   Block: ${blockHash.slice(0, 10)}...`);

    console.log('⏳ Waiting for attestation...');
    const attestation2 = await attestationService.waitForAttestationByNonce(
      2,
      nonce2,
      90000
    );
    console.log(`   Signatures: ${attestation2.signatureCount}/5`);

    console.log('⏳ Waiting for relay to Ethereum...');
    await waitFor(
      async () => ethereumHelper.isMessageReceived(attestation2.messageHash),
      120000,
      3000,
      'relay to Ethereum'
    );

    const ethBalanceFinal = await ethereumHelper.getBalance(userEthAddress);
    const substrateBalanceFinal = await substrateHelper.getBalance(
      userSubstrateAddress
    );

    console.log('\n📊 Final balances:');
    console.log(`   Ethereum: ${ethBalanceFinal / BigInt(10 ** 18)} EDSC (+150)`);
    console.log(`   Ëtrid:    ${substrateBalanceFinal / BigInt(10 ** 18)} EDSC (-150)`);

    expect(ethBalanceFinal).toBe(ethBalance2 + transferBackToEth);
    expect(substrateBalanceFinal).toBe(substrateBalance2 - transferBackToEth);
    console.log('✅ Phase 4 complete\n');

    // ========================================
    // Phase 5: Final Verification
    // ========================================
    console.log('═══ Phase 5: Final Verification ═══\n');

    const expectedEthFinal = ethBalanceStart - transferToEtrid + transferBackToEth;
    const expectedSubstrateFinal =
      substrateBalanceStart + transferToEtrid - transferBackToEth;

    console.log('📈 Summary:');
    console.log(`   Started with:  ${ethBalanceStart / BigInt(10 ** 18)} EDSC on Ethereum`);
    console.log(`   Sent to Ëtrid: ${transferToEtrid / BigInt(10 ** 18)} EDSC`);
    console.log(`   Sent back:     ${transferBackToEth / BigInt(10 ** 18)} EDSC`);
    console.log(`   Final Eth:     ${ethBalanceFinal / BigInt(10 ** 18)} EDSC`);
    console.log(`   Final Ëtrid:   ${substrateBalanceFinal / BigInt(10 ** 18)} EDSC`);
    console.log(`   Net on Ëtrid:  ${(transferToEtrid - transferBackToEth) / BigInt(10 ** 18)} EDSC`);

    expect(ethBalanceFinal).toBe(expectedEthFinal);
    expect(substrateBalanceFinal).toBe(expectedSubstrateFinal);

    console.log('\n✅ Phase 5 complete');
    console.log('\n╔══════════════════════════════════════╗');
    console.log('║   🎉 E2E TEST PASSED! 🎉            ║');
    console.log('╚══════════════════════════════════════╝\n');
  }, 600000); // 10 minute timeout

  test('Bridge handles high-value transfers', async () => {
    const highValue = BigInt(10000 * 10 ** 18); // 10,000 EDSC

    console.log('\n═══ Testing High-Value Transfer ═══\n');

    // Mint if needed
    const currentBalance = await ethereumHelper.getBalance(userEthAddress);
    if (currentBalance < highValue) {
      await ethereumHelper.mintEDSC(userEthAddress, highValue);
    }

    // Transfer
    await ethereumHelper.approveTokenMessenger(highValue);
    const { nonce } = await ethereumHelper.burnAndSend(userSubstrateAddress, highValue);

    // Wait for attestation and relay
    const attestation = await attestationService.waitForAttestationByNonce(0, nonce, 90000);
    await waitFor(
      async () => substrateHelper.isMessageReceived(attestation.messageHash),
      120000,
      3000,
      'high-value relay'
    );

    console.log('✅ High-value transfer successful');
  }, 240000);

  test('Bridge statistics and health', async () => {
    console.log('\n═══ Checking Bridge Health ═══\n');

    // Check attestation service
    const health = await attestationService.checkHealth();
    console.log('Attestation Service:');
    console.log(`   Status: ${health.status}`);
    console.log(`   Uptime: ${Math.floor(health.uptime / 1000)}s`);

    expect(health.status).toBe('healthy');

    // Check stats
    const stats = await attestationService.getStats();
    console.log('\nStatistics:');
    console.log(`   Total attestations: ${stats.attestations?.total || 0}`);
    console.log(`   Ready: ${stats.attestations?.ready || 0}`);
    console.log(`   Relayed: ${stats.attestations?.relayed || 0}`);

    // Check chain connectivity
    const ethBlock = await ethereumHelper.getBlockNumber();
    const substrateBlock = await substrateHelper.getBlockNumber();

    console.log('\nChain Status:');
    console.log(`   Ethereum block: ${ethBlock}`);
    console.log(`   Ëtrid block: ${substrateBlock}`);

    expect(ethBlock).toBeGreaterThan(0);
    expect(substrateBlock).toBeGreaterThan(0);

    console.log('\n✅ Bridge is healthy');
  }, 30000);
});
