#!/usr/bin/env node
// Calculate Foundation Multisig Address (5-of-7)
// Uses proper Substrate multisig derivation

const { encodeAddress, blake2AsU8a, sortAddresses } = require('@polkadot/util-crypto');
const { u8aConcat, compactToU8a, stringToU8a } = require('@polkadot/util');

// Foundation multisig signers (from genesis-accounts)
const signers = [
  '5E4zHGvcWFDa4dEuZLjzF5KJZzUpf4xqFqSCoqo322WiVy3R',
  '5CygHu9BwkS9EwY49jkmEhRkfJMgDDfxw5CPtCmPFF1vMGTf',
  '5Fexmdas1xPFs8m5a35jMeFHZhQBGGBvhWdZKQCdSpjaMXHC',
  '5GTNL8VcraBPhbcphCYY8iic6HQtzVo8gaj6XSxLp3MM2AnG',
  '5F6eJYDMUutgVVk2nvjXh86818Q3pxGP9oyUyL1NkRC8z93W',
  '5D2nWVuKFRCaMVMoazv82FhguEUZyaQvyBg8Yw9wXzqWZLvd',
  '5EvuMC31n7zM2QViBqxdAmMSpMXrCAyre9hSSKPKKkxoSTxM',
];

const threshold = 5; // 5-of-7 multisig
const ss58Format = 42; // Substrate format

console.log('\n╔════════════════════════════════════════════════════════════╗');
console.log('║       Foundation Multisig Address (5-of-7)                ║');
console.log('╚════════════════════════════════════════════════════════════╝\n');

// Sort addresses (required for deterministic multisig derivation)
const sortedSigners = sortAddresses(signers, ss58Format);

console.log('Signers (sorted):');
sortedSigners.forEach((signer, i) => {
  console.log(`  ${i+1}. ${signer}`);
});
console.log('');
console.log(`Threshold: ${threshold} of ${signers.length}`);
console.log('');

// Decode addresses to public keys
const { decodeAddress } = require('@polkadot/util-crypto');
const publicKeys = sortedSigners.map(addr => decodeAddress(addr, false, ss58Format));

// Substrate multisig derivation:
// blake2_256("modlpy/utilisuba" + threshold_compact + concat(sorted_pubkeys))
const PREFIX = stringToU8a('modlpy/utilisuba');
const thresholdU8a = compactToU8a(threshold);
const concatenatedKeys = u8aConcat(...publicKeys);

const multisigPublicKey = blake2AsU8a(
  u8aConcat(PREFIX, thresholdU8a, concatenatedKeys)
);

const multisigAddress = encodeAddress(multisigPublicKey, ss58Format);

console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('\n🔐 Foundation Multisig Address:\n');
console.log(`   ${multisigAddress}`);
console.log('');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');
console.log('⚠️  NEXT STEPS:');
console.log('');
console.log('1. Update flarechain_mainnet_genesis.json:');
console.log(`   Replace "sudo": {"key": "..."} with:`);
console.log(`   "sudo": {"key": "${multisigAddress}"}`);
console.log('');
console.log('2. Copy genesis config to runtime:');
console.log('   cp flarechain_mainnet_genesis.json \\');
console.log('      05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json');
console.log('');
console.log('3. Build mainnet binary:');
console.log('   cargo build --release');
console.log('');
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
console.log('');
