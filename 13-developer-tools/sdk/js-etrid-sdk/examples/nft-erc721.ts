/**
 * NFT (ERC721) Contract Example
 *
 * This example demonstrates how to:
 * - Deploy an NFT collection contract
 * - Mint NFTs with metadata URIs
 * - Transfer NFTs between accounts
 * - Approve transfers
 * - Query ownership and balances
 * - Burn NFTs
 */

import { EtridClient } from '../src';
import * as fs from 'fs';
import * as path from 'path';

// Contract ABI and WASM
const CONTRACT_PATH = '../../contracts/etwasm-examples/06-nft-erc721/target/ink';

async function main() {
    console.log('🎨 NFT (ERC721) Example\n');

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
        fs.readFileSync(path.join(__dirname, CONTRACT_PATH, 'nft_erc721.json'), 'utf8')
    );
    const wasm = fs.readFileSync(path.join(__dirname, CONTRACT_PATH, 'nft_erc721.wasm'));

    // 4. Deploy NFT collection contract
    console.log('📦 Deploying NFT collection contract...');
    const contract = await client.deployContract({
        abi,
        wasm,
        constructor: 'new',
        args: [
            'Crypto Apes',  // name
            'CAPE'          // symbol
        ],
        signer: alice,
        gasLimit: 100000000000,
    });

    console.log('✅ Contract deployed at:', contract.address);
    console.log('');

    // 5. Query collection information
    console.log('📊 Collection Information:');

    const name = await contract.query('name', []);
    console.log('  Name:', name.output);

    const symbol = await contract.query('symbol', []);
    console.log('  Symbol:', symbol.output);

    const totalSupply = await contract.query('totalSupply', []);
    console.log('  Total Supply:', totalSupply.output);
    console.log('');

    // 6. Mint NFT #1 to Alice
    console.log('🎨 Minting NFT #1 to Alice...');
    const mint1Tx = await contract.tx('mint', [
        alice.address,
        1,
        'ipfs://QmHash1/metadata.json'  // Metadata URI
    ], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await mint1Tx.wait();
    console.log('✅ NFT #1 minted');
    console.log('');

    // 7. Mint NFT #2 to Alice
    console.log('🎨 Minting NFT #2 to Alice...');
    const mint2Tx = await contract.tx('mint', [
        alice.address,
        2,
        'ipfs://QmHash2/metadata.json'
    ], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await mint2Tx.wait();
    console.log('✅ NFT #2 minted');
    console.log('');

    // 8. Mint NFT #3 to Alice
    console.log('🎨 Minting NFT #3 to Alice...');
    const mint3Tx = await contract.tx('mint', [
        alice.address,
        3,
        'ipfs://QmHash3/metadata.json'
    ], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await mint3Tx.wait();
    console.log('✅ NFT #3 minted');
    console.log('');

    // 9. Check Alice's balance
    const aliceBalance = await contract.query('balanceOf', [alice.address]);
    console.log('💰 Alice\'s NFT balance:', aliceBalance.output);
    console.log('');

    // 10. Check ownership and metadata
    console.log('🔍 NFT Details:');
    for (let tokenId = 1; tokenId <= 3; tokenId++) {
        const owner = await contract.query('ownerOf', [tokenId]);
        const tokenUri = await contract.query('tokenUri', [tokenId]);

        console.log(`  NFT #${tokenId}:`);
        console.log('    Owner:', owner.output);
        console.log('    URI:', tokenUri.output);
    }
    console.log('');

    // 11. Transfer NFT #1 from Alice to Bob
    console.log('💸 Transferring NFT #1 from Alice to Bob...');
    const transferTx = await contract.tx('transferFrom', [
        alice.address,
        bob.address,
        1
    ], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await transferTx.wait();
    console.log('✅ Transfer complete');
    console.log('');

    // 12. Check balances after transfer
    const aliceBalanceAfter = await contract.query('balanceOf', [alice.address]);
    const bobBalance = await contract.query('balanceOf', [bob.address]);

    console.log('💰 Balances after transfer:');
    console.log('  Alice:', aliceBalanceAfter.output, 'NFTs');
    console.log('  Bob:', bobBalance.output, 'NFTs');
    console.log('');

    // 13. Verify ownership of NFT #1
    const nft1Owner = await contract.query('ownerOf', [1]);
    console.log('🔍 NFT #1 owner:', nft1Owner.output);
    console.log('  (should be Bob)');
    console.log('');

    // 14. Alice approves Charlie for NFT #2
    console.log('✅ Alice approving Charlie for NFT #2...');
    const approveTx = await contract.tx('approve', [charlie.address, 2], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await approveTx.wait();
    console.log('✅ Approval complete');
    console.log('');

    // 15. Check approved address for NFT #2
    const approved = await contract.query('getApproved', [2]);
    console.log('🔑 Approved address for NFT #2:', approved.output);
    console.log('');

    // 16. Charlie transfers NFT #2 from Alice to himself
    console.log('💸 Charlie transferring NFT #2 from Alice to himself...');
    const approvedTransferTx = await contract.tx('transferFrom', [
        alice.address,
        charlie.address,
        2
    ], {
        signer: charlie,
        gasLimit: 10000000000,
    });

    await approvedTransferTx.wait();
    console.log('✅ Approved transfer complete');
    console.log('');

    // 17. Set Alice's approval for all to Bob
    console.log('✅ Alice setting Bob as operator for all her NFTs...');
    const setApprovalTx = await contract.tx('setApprovalForAll', [bob.address, true], {
        signer: alice,
        gasLimit: 10000000000,
    });

    await setApprovalTx.wait();
    console.log('✅ Operator set');
    console.log('');

    // 18. Check operator status
    const isOperator = await contract.query('isApprovedForAll', [alice.address, bob.address]);
    console.log('🔑 Is Bob an operator for Alice?', isOperator.output);
    console.log('');

    // 19. Bob transfers NFT #3 from Alice to himself (as operator)
    console.log('💸 Bob transferring NFT #3 from Alice to himself (as operator)...');
    const operatorTransferTx = await contract.tx('transferFrom', [
        alice.address,
        bob.address,
        3
    ], {
        signer: bob,
        gasLimit: 10000000000,
    });

    await operatorTransferTx.wait();
    console.log('✅ Operator transfer complete');
    console.log('');

    // 20. Check final balances
    const finalAliceBalance = await contract.query('balanceOf', [alice.address]);
    const finalBobBalance = await contract.query('balanceOf', [bob.address]);
    const finalCharlieBalance = await contract.query('balanceOf', [charlie.address]);

    console.log('💰 Final Balances:');
    console.log('  Alice:', finalAliceBalance.output, 'NFTs');
    console.log('  Bob:', finalBobBalance.output, 'NFTs');
    console.log('  Charlie:', finalCharlieBalance.output, 'NFTs');
    console.log('');

    // 21. Check total supply
    const finalTotalSupply = await contract.query('totalSupply', []);
    console.log('📊 Total Supply:', finalTotalSupply.output);
    console.log('');

    // 22. Bob burns NFT #1
    console.log('🔥 Bob burning NFT #1...');
    const burnTx = await contract.tx('burn', [1], {
        signer: bob,
        gasLimit: 10000000000,
    });

    await burnTx.wait();
    console.log('✅ NFT #1 burned');
    console.log('');

    // 23. Check supply after burn
    const supplyAfterBurn = await contract.query('totalSupply', []);
    const bobBalanceAfterBurn = await contract.query('balanceOf', [bob.address]);

    console.log('📊 After burning NFT #1:');
    console.log('  Total Supply:', supplyAfterBurn.output);
    console.log('  Bob\'s balance:', bobBalanceAfterBurn.output, 'NFTs');
    console.log('');

    // 24. Try to query burned NFT (should return None)
    const burnedNftOwner = await contract.query('ownerOf', [1]);
    console.log('🔍 Owner of NFT #1 (burned):', burnedNftOwner.output || 'None');
    console.log('');

    // 25. Listen to Transfer events (example)
    console.log('👂 Listening to Transfer events for 10 seconds...');
    contract.on('Transfer', (event) => {
        console.log('  🔔 Transfer event:', {
            from: event.from || 'null (mint)',
            to: event.to || 'null (burn)',
            tokenId: event.tokenId,
        });
    });

    // Keep alive for 10 seconds to catch events
    await new Promise(resolve => setTimeout(resolve, 10000));

    // 26. Cleanup
    await client.disconnect();
    console.log('\n✅ Disconnected from node');
    console.log('\n🎉 NFT Example complete!');
}

// Run the example
main().catch((error) => {
    console.error('❌ Error:', error);
    process.exit(1);
});
