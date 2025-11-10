#!/usr/bin/env node
/**
 * Quick test to query sudo account and test basic RPC connectivity
 */

const { ApiPromise, HttpProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

const RPC_ENDPOINT = 'http://localhost:9944';

async function main() {
  console.log('Testing FlareChain RPC connection...');
  console.log(`Endpoint: ${RPC_ENDPOINT}`);
  console.log();

  try {
    await cryptoWaitReady();

    const provider = new HttpProvider(RPC_ENDPOINT);
    const api = await ApiPromise.create({ provider });

    console.log('✓ Connected to FlareChain');
    console.log();

    // Get chain info
    const [chain, lastHeader] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.chain.getHeader()
    ]);

    console.log(`Chain: ${chain}`);
    console.log(`Latest Block: #${lastHeader.number}`);
    console.log();

    // Query sudo
    console.log('Querying Sudo account...');
    const sudoKey = await api.query.sudo.key();
    console.log(`Sudo Account: ${sudoKey.toString()}`);
    console.log();

    // Query GRANDPA
    console.log('Querying GRANDPA state...');
    try {
      const currentSetId = await api.query.grandpa.currentSetId();
      console.log(`GRANDPA Set ID: ${currentSetId.toString()}`);

      const authorities = await api.query.grandpa.authorities();
      console.log(`GRANDPA Authorities Count: ${authorities.length}`);

      if (authorities.length === 0) {
        console.log('⚠ WARNING: ZERO GRANDPA AUTHORITIES - Finality is stuck!');
      }
    } catch (e) {
      console.log(`GRANDPA query error: ${e.message}`);
    }
    console.log();

    // Test with controller accounts
    console.log('Testing with controller accounts...');
    const keyring = new Keyring({ type: 'sr25519' });

    const GIZZI_CONTROLLER_SEED = '0xc4868255b3554037608552fdba89b2f589e747ad5364a909407fbedfa8ed6cee';
    const EOJ_CONTROLLER_SEED = '0x67d7d6f3df3d9f1204664ffb64e5d257c14643a8a9699371244ef2a0e7983815';

    const gizziController = keyring.addFromUri(GIZZI_CONTROLLER_SEED);
    const eojController = keyring.addFromUri(EOJ_CONTROLLER_SEED);

    console.log(`Gizzi Controller: ${gizziController.address}`);
    console.log(`Eoj Controller: ${eojController.address}`);
    console.log();

    // Check if we have sudo
    const sudoAddress = sudoKey.toString();
    console.log('='.repeat(60));
    if (sudoAddress === gizziController.address) {
      console.log('✓ SUDO IS GIZZI CONTROLLER - WE HAVE THE KEY!');
      console.log(`  We can submit sudo.setCode() using Gizzi controller`);
    } else if (sudoAddress === eojController.address) {
      console.log('✓ SUDO IS EOJ CONTROLLER - WE HAVE THE KEY!');
      console.log(`  We can submit sudo.setCode() using Eoj controller`);
    } else {
      console.log('✗ SUDO IS UNKNOWN ACCOUNT - BLOCKER');
      console.log(`  Sudo: ${sudoAddress}`);
      console.log(`  We do NOT have the private key for this account`);
      console.log(`  Cannot submit sudo.setCode() extrinsics`);
    }
    console.log('='.repeat(60));

    await api.disconnect();
    process.exit(0);

  } catch (error) {
    console.error('ERROR:', error.message);
    console.error(error.stack);
    process.exit(1);
  }
}

main();
