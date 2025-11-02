#!/usr/bin/env node

/**
 * Set ETR Lock Account on FlareChain Mainnet
 *
 * This script configures the lock account for pallet-etr-lock,
 * which will hold all locked ETR tokens during cross-chain bridging.
 */

const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

// Configuration
const FLARECHAIN_WS_URL = process.env.FLARECHAIN_WS || 'ws://98.71.91.84:9944';
const SUDO_SEED = process.env.SUDO_SEED; // Foundation multisig seed phrase

// Lock account options:
// Option 1: Create new dedicated bridge account
// Option 2: Use foundation multisig
// Option 3: Use community LP pool
const LOCK_ACCOUNT_TYPE = process.env.LOCK_ACCOUNT_TYPE || 'dedicated'; // 'dedicated', 'multisig', or 'address'
const LOCK_ACCOUNT_ADDRESS = process.env.LOCK_ACCOUNT_ADDRESS; // If using existing address

async function main() {
    console.log('🔐 ETR Lock Account Setup');
    console.log('========================\n');

    // Wait for crypto libraries
    await cryptoWaitReady();

    // Connect to FlareChain
    console.log(`📡 Connecting to FlareChain: ${FLARECHAIN_WS_URL}`);
    const provider = new WsProvider(FLARECHAIN_WS_URL);
    const api = await ApiPromise.create({ provider });

    console.log(`✅ Connected to FlareChain ${await api.rpc.system.chain()}`);
    console.log(`   Version: ${await api.rpc.system.version()}\n`);

    // Initialize keyring
    const keyring = new Keyring({ type: 'sr25519' });

    // Get sudo account (foundation multisig or governance)
    if (!SUDO_SEED) {
        console.error('❌ Error: SUDO_SEED environment variable not set');
        console.log('\nUsage:');
        console.log('  SUDO_SEED="your seed phrase" node set-lock-account.js');
        process.exit(1);
    }

    const sudoAccount = keyring.addFromUri(SUDO_SEED);
    console.log(`🔑 Sudo Account: ${sudoAccount.address}`);

    // Determine lock account
    let lockAccount;
    let lockAccountAddress;

    switch (LOCK_ACCOUNT_TYPE) {
        case 'dedicated':
            // Create new dedicated account for bridge locking
            console.log('\n📦 Creating new dedicated bridge lock account...');
            lockAccount = keyring.addFromUri('//EtrBridgeLock');
            lockAccountAddress = lockAccount.address;
            console.log(`   Address: ${lockAccountAddress}`);
            console.log(`   ⚠️  SAVE THIS SEED: //EtrBridgeLock`);
            break;

        case 'multisig':
            // Use foundation multisig
            console.log('\n🏛️  Using foundation multisig as lock account...');
            lockAccount = sudoAccount;
            lockAccountAddress = sudoAccount.address;
            console.log(`   Address: ${lockAccountAddress}`);
            break;

        case 'address':
            // Use provided address
            if (!LOCK_ACCOUNT_ADDRESS) {
                console.error('❌ Error: LOCK_ACCOUNT_ADDRESS required when using address type');
                process.exit(1);
            }
            lockAccountAddress = LOCK_ACCOUNT_ADDRESS;
            console.log(`\n🎯 Using provided address as lock account...`);
            console.log(`   Address: ${lockAccountAddress}`);
            break;

        default:
            console.error('❌ Invalid LOCK_ACCOUNT_TYPE. Use: dedicated, multisig, or address');
            process.exit(1);
    }

    // Check current lock account
    console.log('\n🔍 Checking current lock account configuration...');
    const currentLockAccount = await api.query.etrLock.lockAccount();

    if (currentLockAccount.isSome) {
        console.log(`   Current: ${currentLockAccount.unwrap().toString()}`);
        console.log('   ⚠️  Lock account is already set!');

        const readline = require('readline').createInterface({
            input: process.stdin,
            output: process.stdout
        });

        const answer = await new Promise(resolve => {
            readline.question('\n   Overwrite? (yes/no): ', resolve);
        });
        readline.close();

        if (answer.toLowerCase() !== 'yes') {
            console.log('\n❌ Cancelled by user');
            process.exit(0);
        }
    } else {
        console.log('   Current: None (not set)');
    }

    // Set lock account via sudo
    console.log('\n🚀 Setting lock account...');

    const tx = api.tx.sudo.sudo(
        api.tx.etrLock.setLockAccount(lockAccountAddress)
    );

    // Sign and send
    const unsub = await tx.signAndSend(sudoAccount, ({ status, events }) => {
        console.log(`   Transaction status: ${status.type}`);

        if (status.isInBlock) {
            console.log(`   ✅ Included in block: ${status.asInBlock.toHex()}`);
        }

        if (status.isFinalized) {
            console.log(`   ✅ Finalized in block: ${status.asFinalized.toHex()}`);

            // Check for success
            events.forEach(({ event }) => {
                const { section, method, data } = event;

                if (section === 'sudo' && method === 'Sudid') {
                    console.log('\n🎉 Success! Lock account set successfully');
                } else if (section === 'system' && method === 'ExtrinsicFailed') {
                    console.log('\n❌ Transaction failed!');
                    console.log(`   Error: ${data.toString()}`);
                }
            });

            unsub();
            verifyAndExit();
        }
    });

    async function verifyAndExit() {
        // Verify the account was set
        console.log('\n🔍 Verifying lock account...');
        const newLockAccount = await api.query.etrLock.lockAccount();

        if (newLockAccount.isSome && newLockAccount.unwrap().toString() === lockAccountAddress) {
            console.log(`✅ Lock account verified: ${lockAccountAddress}`);
            console.log('\n📊 Current Lock Status:');

            const totalLocked = await api.query.etrLock.totalLocked();
            console.log(`   Total Locked: ${totalLocked.toString()} (${formatBalance(totalLocked)} ETR)`);

            // Check balance of lock account
            const { data: balance } = await api.query.system.account(lockAccountAddress);
            console.log(`   Lock Account Balance: ${formatBalance(balance.free)} ETR`);

            console.log('\n✅ Setup complete!');
            console.log('\n📝 Next steps:');
            console.log('   1. Fund the lock account with some ETR for transaction fees');
            console.log('   2. Deploy PBC smart contracts on external chains');
            console.log('   3. Configure and start the relayer service');
            console.log('   4. Test bridge with small amount (100 ETR)');
        } else {
            console.log('❌ Verification failed! Lock account not set correctly');
        }

        process.exit(0);
    }

    function formatBalance(balance) {
        // ETR has 18 decimals
        const balanceStr = balance.toString();
        if (balanceStr.length <= 18) {
            return `0.${balanceStr.padStart(18, '0')}`;
        }
        const whole = balanceStr.slice(0, -18);
        const decimal = balanceStr.slice(-18);
        return `${whole}.${decimal}`;
    }
}

main().catch(console.error);
