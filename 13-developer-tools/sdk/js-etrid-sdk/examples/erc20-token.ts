/**
 * ERC20 Token Contract Example
 *
 * This example demonstrates how to:
 * - Deploy an ERC20 token contract
 * - Transfer tokens between accounts
 * - Approve spending
 * - Check balances and allowances
 * - Mint and burn tokens (owner only)
 */

import { EtridClient } from '../src';
import * as fs from 'fs';
import * as path from 'path';

// Contract ABI and WASM (you'll need to build the contract first)
const CONTRACT_PATH = '../../contracts/etwasm-examples/03-erc20-token/target/ink';

async function main() {
    console.log('🚀 ERC20 Token Example\n');

    // 1. Connect to Ëtrid node
    const client = new EtridClient('ws://localhost:9944');
    await client.connect();
    console.log('✅ Connected to Ëtrid node\n');

    // 2. Setup accounts
    const alice = client.createAccount('//Alice');
    const bob = client.createAccount('//Bob');
    const charlie = client.createAccount('//Charlie');

    console.log('📝 Accounts:');
    console.log('  Alice:', alice.address);
    console.log('  Bob:', bob.address);
    console.log('  Charlie:', charlie.address);
    console.log('');

    // 3. Load contract ABI and WASM
    const abi = JSON.parse(
        fs.readFileSync(path.join(__dirname, CONTRACT_PATH, 'erc20_token.json'), 'utf8')
    );
    const wasm = fs.readFileSync(path.join(__dirname, CONTRACT_PATH, 'erc20_token.wasm'));

    // 4. Deploy ERC20 contract
    console.log('📦 Deploying ERC20 token contract...');
    const contract = await client.deployContract({
        abi,
        wasm,
        constructor: 'new',
        args: [
            1000000,              // initial_supply
            'Example Token',      // name
            'EXT',               // symbol
            18                   // decimals
        ],
        signer: alice,
        gasLimit: 100000000000,
    });

    console.log('✅ Contract deployed at:', contract.address);
    console.log('');

    // 5. Query token information
    console.log('📊 Token Information:');

    const name = await contract.query('name', []);
    console.log('  Name:', name.output);

    const symbol = await contract.query('symbol', []);
    console.log('  Symbol:', symbol.output);

    const decimals = await contract.query('decimals', []);
    console.log('  Decimals:', decimals.output);

    const totalSupply = await contract.query('totalSupply', []);
    console.log('  Total Supply:', totalSupply.output);
    console.log('');

    // 6. Check Alice's initial balance
    const aliceBalance = await contract.query('balanceOf', [alice.address]);
    console.log('💰 Initial Balances:');
    console.log('  Alice:', aliceBalance.output, 'tokens');
    console.log('');

    // 7. Transfer tokens from Alice to Bob
    console.log('💸 Transferring 100 tokens from Alice to Bob...');
    const transferTx = await contract.tx('transfer', [bob.address, 100], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await transferTx.wait();
    console.log('✅ Transfer complete. TX hash:', transferTx.hash);
    console.log('');

    // 8. Check balances after transfer
    const aliceBalanceAfter = await contract.query('balanceOf', [alice.address]);
    const bobBalance = await contract.query('balanceOf', [bob.address]);

    console.log('💰 Balances after transfer:');
    console.log('  Alice:', aliceBalanceAfter.output, 'tokens');
    console.log('  Bob:', bobBalance.output, 'tokens');
    console.log('');

    // 9. Alice approves Charlie to spend 50 tokens
    console.log('✅ Alice approving Charlie to spend 50 tokens...');
    const approveTx = await contract.tx('approve', [charlie.address, 50], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await approveTx.wait();
    console.log('✅ Approval complete');
    console.log('');

    // 10. Check allowance
    const allowance = await contract.query('allowance', [alice.address, charlie.address]);
    console.log('🔑 Charlie\'s allowance to spend Alice\'s tokens:', allowance.output);
    console.log('');

    // 11. Charlie transfers 25 tokens from Alice to Bob
    console.log('💸 Charlie transferring 25 tokens from Alice to Bob (using allowance)...');
    const transferFromTx = await contract.tx(
        'transferFrom',
        [alice.address, bob.address, 25],
        {
            signer: charlie,
            gasLimit: 10000000000,
        }
    );

    await transferFromTx.wait();
    console.log('✅ TransferFrom complete');
    console.log('');

    // 12. Check final balances
    const aliceBalanceFinal = await contract.query('balanceOf', [alice.address]);
    const bobBalanceFinal = await contract.query('balanceOf', [bob.address]);
    const allowanceFinal = await contract.query('allowance', [alice.address, charlie.address]);

    console.log('💰 Final Balances:');
    console.log('  Alice:', aliceBalanceFinal.output, 'tokens');
    console.log('  Bob:', bobBalanceFinal.output, 'tokens');
    console.log('  Remaining allowance:', allowanceFinal.output);
    console.log('');

    // 13. Mint new tokens (owner only)
    console.log('🏭 Minting 1000 new tokens to Alice (owner only)...');
    const mintTx = await contract.tx('mint', [alice.address, 1000], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await mintTx.wait();
    console.log('✅ Minting complete');

    const totalSupplyAfterMint = await contract.query('totalSupply', []);
    console.log('  New total supply:', totalSupplyAfterMint.output);
    console.log('');

    // 14. Burn tokens
    console.log('🔥 Burning 500 tokens from Alice...');
    const burnTx = await contract.tx('burn', [500], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await burnTx.wait();
    console.log('✅ Burning complete');

    const totalSupplyAfterBurn = await contract.query('totalSupply', []);
    console.log('  New total supply:', totalSupplyAfterBurn.output);
    console.log('');

    // 15. Listen to Transfer events (example)
    console.log('👂 Listening to Transfer events for 10 seconds...');
    contract.on('Transfer', (event) => {
        console.log('  🔔 Transfer event:', {
            from: event.from || 'null (mint)',
            to: event.to || 'null (burn)',
            value: event.value,
        });
    });

    // Keep alive for 10 seconds to catch events
    await new Promise(resolve => setTimeout(resolve, 10000));

    // 16. Cleanup
    await client.disconnect();
    console.log('\n✅ Disconnected from node');
    console.log('\n🎉 Example complete!');
}

// Run the example
main().catch((error) => {
    console.error('❌ Error:', error);
    process.exit(1);
});
