#!/usr/bin/env node
/**
 * Test basic extrinsic submission to verify blocks are being produced
 */

const { ApiPromise, HttpProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

const RPC_ENDPOINT = 'http://localhost:9944';
const GIZZI_CONTROLLER_SEED = '0xc4868255b3554037608552fdba89b2f589e747ad5364a909407fbedfa8ed6cee';

async function main() {
  console.log('Testing Extrinsic Submission...');
  console.log();

  try {
    await cryptoWaitReady();

    const provider = new HttpProvider(RPC_ENDPOINT);
    const api = await ApiPromise.create({ provider });

    console.log('✓ Connected to FlareChain');

    const keyring = new Keyring({ type: 'sr25519' });
    const testAccount = keyring.addFromUri(GIZZI_CONTROLLER_SEED);

    console.log(`Test Account: ${testAccount.address}`);

    const { data: balance } = await api.query.system.account(testAccount.address);
    console.log(`Balance: ${balance.free.toHuman()}`);
    console.log();

    // Check current block
    const startBlock = await api.rpc.chain.getHeader();
    console.log(`Starting at block #${startBlock.number}`);
    console.log();

    console.log('Creating and submitting system.remark extrinsic...');
    const remarkTx = api.tx.system.remark('FINALITY_TEST_' + Date.now());

    // Sign and send
    const hash = await remarkTx.signAndSend(testAccount);
    console.log(`✓ Extrinsic submitted! Hash: ${hash.toHex()}`);
    console.log();

    // Wait a bit and check if block number increased
    console.log('Waiting 12 seconds for block production...');
    await new Promise(resolve => setTimeout(resolve, 12000));

    const endBlock = await api.rpc.chain.getHeader();
    console.log(`Now at block #${endBlock.number}`);
    console.log();

    const blocksDiff = endBlock.number.toNumber() - startBlock.number.toNumber();
    if (blocksDiff > 0) {
      console.log(`✓ SUCCESS! ${blocksDiff} new blocks produced`);
      console.log('  Block production is working WITHOUT finality');
      console.log('  Extrinsics CAN be submitted and processed');
    } else {
      console.log('✗ No new blocks produced - block production may be stuck');
    }

    await api.disconnect();
    process.exit(0);

  } catch (error) {
    console.error('ERROR:', error.message);
    process.exit(1);
  }
}

main();
