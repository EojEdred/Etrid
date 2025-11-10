#!/usr/bin/env node
/**
 * Test Extrinsic Submission Without Finality
 *
 * This script tests whether we can submit extrinsics to FlareChain
 * even though GRANDPA finality is stuck at block 0.
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

// Gizzi node RPC endpoint (via SSH tunnel)
// Run: ssh -i ~/.ssh/gizzi-validator -f -N -L 9944:localhost:9944 ubuntu@64.181.215.19
const GIZZI_RPC = 'ws://localhost:9944';

// Controller account seeds from validator-keys-complete.json
const GIZZI_CONTROLLER_SEED = '0xc4868255b3554037608552fdba89b2f589e747ad5364a909407fbedfa8ed6cee';
const EOJ_CONTROLLER_SEED = '0x67d7d6f3df3d9f1204664ffb64e5d257c14643a8a9699371244ef2a0e7983815';

async function main() {
  console.log('='.repeat(80));
  console.log('FlareChain Extrinsic Submission Test (Without Finality)');
  console.log('='.repeat(80));
  console.log();

  try {
    // Initialize crypto
    console.log('[1/7] Initializing crypto...');
    await cryptoWaitReady();
    console.log('✓ Crypto ready');
    console.log();

    // Connect to Gizzi
    console.log('[2/7] Connecting to Gizzi RPC...');
    console.log(`      Endpoint: ${GIZZI_RPC}`);

    const provider = new WsProvider(GIZZI_RPC, false); // autoConnectMs = false

    // Set timeout for connection
    const connectTimeout = setTimeout(() => {
      console.log('✗ Connection timeout after 10 seconds');
      process.exit(1);
    }, 10000);

    const api = await ApiPromise.create({
      provider,
      throwOnConnect: true
    });

    clearTimeout(connectTimeout);
    console.log('✓ Connected to FlareChain');
    console.log();

    // Query chain info
    console.log('[3/7] Querying chain information...');
    const [chain, nodeName, nodeVersion, lastHeader] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version(),
      api.rpc.chain.getHeader()
    ]);

    console.log(`      Chain: ${chain}`);
    console.log(`      Node: ${nodeName} v${nodeVersion}`);
    console.log(`      Latest Block: #${lastHeader.number}`);
    console.log();

    // Query sudo account
    console.log('[4/7] Querying Sudo account from chain state...');
    const sudoKey = await api.query.sudo.key();
    console.log(`      Current Sudo Account: ${sudoKey.toString()}`);
    console.log();

    // Query GRANDPA state
    console.log('[5/7] Checking GRANDPA finality status...');
    try {
      const grandpaStalled = await api.query.grandpa.stalled();
      const currentSetId = await api.query.grandpa.currentSetId();
      console.log(`      GRANDPA Set ID: ${currentSetId.toString()}`);
      console.log(`      Stalled: ${grandpaStalled.toString()}`);
    } catch (e) {
      console.log(`      GRANDPA query error: ${e.message}`);
    }
    console.log();

    // Create keyring and test account
    console.log('[6/7] Creating test account from Gizzi controller...');
    const keyring = new Keyring({ type: 'sr25519' });
    const testAccount = keyring.addFromUri(GIZZI_CONTROLLER_SEED);
    console.log(`      Test Account: ${testAccount.address}`);

    // Query account balance
    const { data: balance } = await api.query.system.account(testAccount.address);
    console.log(`      Free Balance: ${balance.free.toHuman()}`);
    console.log();

    // Test 1: Submit system.remark
    console.log('[7/7] Testing extrinsic submission...');
    console.log('      Creating system.remark("FINALITY_TEST") extrinsic...');

    const remarkTx = api.tx.system.remark('FINALITY_TEST');
    const remarkHash = remarkTx.hash.toHex();
    console.log(`      Extrinsic Hash: ${remarkHash}`);

    let txIncludedInBlock = false;
    let txFinalized = false;
    let inclusionBlock = null;
    let blockWatchTimeout = null;

    console.log('      Submitting and watching for inclusion...');
    console.log();

    // Set a timeout for watching
    blockWatchTimeout = setTimeout(() => {
      console.log('      ⚠ Stopping watch after 30 seconds');
      console.log();
    }, 30000);

    const unsub = await remarkTx.signAndSend(testAccount, ({ status, events }) => {
      console.log(`      Status: ${status.type}`);

      if (status.isInBlock) {
        txIncludedInBlock = true;
        inclusionBlock = status.asInBlock.toHex();
        console.log(`      ✓ Included in block: ${inclusionBlock}`);
        console.log(`      Events:`);

        events.forEach(({ event }) => {
          const { section, method, data } = event;
          console.log(`        - ${section}.${method}:`, data.toString());
        });
        console.log();
      }

      if (status.isFinalized) {
        txFinalized = true;
        console.log(`      ✓ Finalized in block: ${status.asFinalized.toHex()}`);
        clearTimeout(blockWatchTimeout);
        unsub();
      }

      if (status.isInvalid || status.isDropped || status.isUsurped) {
        console.log(`      ✗ Transaction ${status.type}`);
        clearTimeout(blockWatchTimeout);
        unsub();
      }
    });

    // Wait for inclusion (but not necessarily finalization)
    await new Promise(resolve => {
      const checkInterval = setInterval(() => {
        if (txIncludedInBlock || txFinalized) {
          clearInterval(checkInterval);
          clearTimeout(blockWatchTimeout);
          resolve();
        }
      }, 1000);

      // Also resolve after timeout
      setTimeout(() => {
        clearInterval(checkInterval);
        resolve();
      }, 35000);
    });

    // Wait a bit more to see if finality happens
    if (txIncludedInBlock && !txFinalized) {
      console.log('      Waiting 10 more seconds to check for finalization...');
      await new Promise(resolve => setTimeout(resolve, 10000));
    }

    console.log();
    console.log('='.repeat(80));
    console.log('TEST RESULTS:');
    console.log('='.repeat(80));
    console.log(`Extrinsic submitted: YES`);
    console.log(`Included in block: ${txIncludedInBlock ? 'YES - ' + inclusionBlock : 'NO'}`);
    console.log(`Finalized: ${txFinalized ? 'YES' : 'NO (Expected with GRANDPA stuck)'}`);
    console.log();

    if (txIncludedInBlock && !txFinalized) {
      console.log('✓ SUCCESS: Extrinsics CAN be included without finality!');
      console.log();
      console.log('IMPLICATIONS:');
      console.log('  - Block production via Aura is working');
      console.log('  - Extrinsics are being processed and included');
      console.log('  - State transitions are happening');
      console.log('  - Finality is not required for extrinsic execution');
      console.log();
      console.log('NEXT STEPS:');
      console.log('  1. We CAN submit a runtime upgrade via sudo.setCode()');
      console.log('  2. The upgrade will be included in a block and executed');
      console.log('  3. Finality is not required for the upgrade to take effect');
      console.log('  4. After upgrade, new runtime can fix GRANDPA authorities');
    } else if (!txIncludedInBlock) {
      console.log('✗ FAILED: Extrinsic not included in any block');
      console.log();
      console.log('This indicates a more serious problem beyond GRANDPA finality.');
    }

    console.log();
    console.log('='.repeat(80));
    console.log('SUDO KEY INFORMATION:');
    console.log('='.repeat(80));
    console.log(`Current Sudo Account: ${sudoKey.toString()}`);
    console.log(`Gizzi Controller: 5CAyFg27EJwoTJcj1KHravoqjidEn4XqciKM5q9ukbVSzSbW`);
    console.log(`Eoj Controller: 5HQTgrkRhd5h5VE2SsL76S9jAf2xZRCaEoVcFiyGxSPAFciq`);
    console.log();

    if (sudoKey.toString() === '5CAyFg27EJwoTJcj1KHravoqjidEn4XqciKM5q9ukbVSzSbW') {
      console.log('✓ Sudo is Gizzi Controller - WE HAVE THE KEY!');
      console.log(`  Seed: ${GIZZI_CONTROLLER_SEED}`);
    } else if (sudoKey.toString() === '5HQTgrkRhd5h5VE2SsL76S9jAf2xZRCaEoVcFiyGxSPAFciq') {
      console.log('✓ Sudo is Eoj Controller - WE HAVE THE KEY!');
      console.log(`  Seed: ${EOJ_CONTROLLER_SEED}`);
    } else if (sudoKey.toString() === '5HCvaHrCfXDasyQNRCdJ4jRtcwMmdkPDZEAF3LqF77qf5JtP') {
      console.log('⚠ Sudo is genesis account - PRIVATE KEY NOT FOUND IN SECRETS');
      console.log('  This key was defined in genesis but private key was not preserved');
      console.log('  BLOCKER: Cannot submit sudo extrinsics without this key');
    } else {
      console.log('⚠ Sudo is UNKNOWN account - need to locate private key');
    }

    console.log();
    await api.disconnect();
    process.exit(0);

  } catch (error) {
    console.error();
    console.error('ERROR:', error.message);
    console.error();
    console.error('Stack trace:', error.stack);
    process.exit(1);
  }
}

main().catch(console.error);
