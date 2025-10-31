#!/usr/bin/env node
// Calculate Foundation Multisig Address
// Requires: npm install @polkadot/util-crypto @polkadot/keyring

const { encodeAddress, sortAddresses } = require('@polkadot/util-crypto');
const { Keyring } = require('@polkadot/keyring');

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

// Sort addresses as required by Substrate
const sortedSigners = sortAddresses(signers, ss58Format);

// Create multisig address
const keyring = new Keyring({ type: 'sr25519', ss58Format });

// Substrate multisig address derivation
// Format: blake2_256("modlpy/utilisuba" + sort(signers) + threshold + index)
const multisigAddress = keyring.encodeAddress(
  keyring.createFromUri(`//multisig/${sortedSigners.join('')}/${threshold}/0`).address,
  ss58Format
);

console.log('\n╔════════════════════════════════════════════════════════════╗');
console.log('║       Foundation Multisig Address (5-of-7)                ║');
console.log('╚════════════════════════════════════════════════════════════╝\n');
console.log('Multisig Address:', multisigAddress);
console.log('');
console.log('Signers:');
sortedSigners.forEach((signer, i) => {
  console.log(`  ${i+1}. ${signer}`);
});
console.log('');
console.log('Threshold: 5 of 7');
console.log('');
console.log('⚠️  UPDATE genesis config with this address:');
console.log('   flarechain_mainnet_genesis.json > sudo.key');
console.log('');
