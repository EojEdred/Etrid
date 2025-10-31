// ËTRID Block Explorer
// Connects to ËTRID blockchain and displays blocks, transactions, and addresses

const BOOTSTRAP_NODES = [
    'ws://20.186.91.207:9944',  // Alice (Azure VM)
    'ws://172.177.44.73:9944',  // Bob (Azure VM)
];

let api = null;
let latestBlocks = [];
let latestTransactions = [];

// Connect to blockchain
async function connectToBlockchain() {
    for (const endpoint of BOOTSTRAP_NODES) {
        try {
            console.log(`Connecting to ${endpoint}...`);
            const { ApiPromise, WsProvider } = polkadotApi;
            const provider = new WsProvider(endpoint, false);

            api = await Promise.race([
                ApiPromise.create({ provider }),
                new Promise((_, reject) =>
                    setTimeout(() => reject(new Error('Timeout')), 10000)
                )
            ]);

            await api.isReady;
            console.log('Connected to ËTRID blockchain');
            return true;
        } catch (error) {
            console.warn(`Failed to connect to ${endpoint}`);
        }
    }
    return false;
}

// Fetch latest blocks
async function fetchLatestBlocks(count = 10) {
    if (!api || !api.isConnected) {
        return showMockBlocks();
    }

    try {
        const header = await api.rpc.chain.getHeader();
        const currentBlockNumber = header.number.toNumber();

        const blocks = [];
        for (let i = 0; i < count && currentBlockNumber - i >= 0; i++) {
            const blockHash = await api.rpc.chain.getBlockHash(currentBlockNumber - i);
            const block = await api.rpc.chain.getBlock(blockHash);
            const header = block.block.header;

            blocks.push({
                number: header.number.toNumber(),
                hash: blockHash.toHex(),
                parentHash: header.parentHash.toHex(),
                stateRoot: header.stateRoot.toHex(),
                extrinsicsRoot: header.extrinsicsRoot.toHex(),
                timestamp: new Date().getTime() - (i * 5000), // Estimate based on 5s block time
                extrinsics: block.block.extrinsics.length,
                validator: 'Validator ' + ((currentBlockNumber - i) % 21 + 1), // Mock validator for now
            });
        }

        latestBlocks = blocks;
        renderBlocks();
        updateStats();
    } catch (error) {
        console.error('Error fetching blocks:', error);
        showMockBlocks();
    }
}

// Fetch block by number or hash
async function fetchBlockDetails(identifier) {
    if (!api || !api.isConnected) {
        return null;
    }

    try {
        let blockHash;
        if (identifier.startsWith('0x')) {
            blockHash = identifier;
        } else {
            const blockNumber = parseInt(identifier);
            blockHash = await api.rpc.chain.getBlockHash(blockNumber);
        }

        const block = await api.rpc.chain.getBlock(blockHash);
        const header = block.block.header;

        return {
            number: header.number.toNumber(),
            hash: blockHash,
            parentHash: header.parentHash.toHex(),
            stateRoot: header.stateRoot.toHex(),
            extrinsicsRoot: header.extrinsicsRoot.toHex(),
            extrinsics: block.block.extrinsics.map((ext, index) => ({
                index,
                hash: ext.hash.toHex(),
                method: ext.method.method,
                section: ext.method.section,
                isSigned: ext.isSigned,
                signer: ext.isSigned ? ext.signer.toString() : 'Unsigned',
            })),
        };
    } catch (error) {
        console.error('Error fetching block details:', error);
        return null;
    }
}

// Render blocks list
function renderBlocks() {
    const container = document.getElementById('blocks-list');
    container.innerHTML = latestBlocks.map(block => `
        <div class="bg-white/5 border border-white/10 rounded-lg p-4 hover:bg-white/10 transition-all cursor-pointer" onclick="showBlockDetails(${block.number})">
            <div class="flex justify-between items-start mb-2">
                <div>
                    <span class="text-gray-400 text-sm">Block</span>
                    <div class="text-xl font-bold text-etrid-blue">#${block.number.toLocaleString()}</div>
                </div>
                <div class="text-right">
                    <span class="text-gray-400 text-sm">${formatTimestamp(block.timestamp)}</span>
                </div>
            </div>
            <div class="flex justify-between items-center text-sm">
                <div>
                    <span class="text-gray-400">Hash:</span>
                    <span class="text-gray-300 font-mono ml-2">${truncateHash(block.hash)}</span>
                </div>
                <div>
                    <span class="text-gray-400">${block.extrinsics} txs</span>
                </div>
            </div>
            <div class="mt-2 text-xs text-gray-500">
                Validator: ${block.validator}
            </div>
        </div>
    `).join('');
}

// Render transactions list
function renderTransactions() {
    const container = document.getElementById('transactions-list');

    if (latestTransactions.length === 0) {
        container.innerHTML = `
            <div class="bg-white/5 border border-white/10 rounded-lg p-8 text-center">
                <p class="text-gray-400">No recent transactions</p>
                <p class="text-sm text-gray-500 mt-2">Transactions will appear here when the network is active</p>
            </div>
        `;
        return;
    }

    container.innerHTML = latestTransactions.map(tx => `
        <div class="bg-white/5 border border-white/10 rounded-lg p-4 hover:bg-white/10 transition-all cursor-pointer" onclick="showTransactionDetails('${tx.hash}')">
            <div class="flex justify-between items-start mb-2">
                <div>
                    <span class="text-gray-400 text-sm">Transaction</span>
                    <div class="font-mono text-sm text-etrid-purple">${truncateHash(tx.hash)}</div>
                </div>
                <div class="text-right">
                    <span class="text-gray-400 text-sm">${formatTimestamp(tx.timestamp)}</span>
                </div>
            </div>
            <div class="flex justify-between items-center text-sm">
                <div>
                    <span class="px-2 py-1 rounded bg-etrid-blue/20 text-etrid-blue text-xs">${tx.method}</span>
                </div>
                <div class="text-gray-400">
                    Block #${tx.block.toLocaleString()}
                </div>
            </div>
        </div>
    `).join('');
}

// Show block details modal
async function showBlockDetails(blockNumber) {
    const details = await fetchBlockDetails(blockNumber.toString());

    if (!details) {
        alert('Failed to fetch block details');
        return;
    }

    const modal = document.getElementById('details-modal');
    const title = document.getElementById('modal-title');
    const content = document.getElementById('modal-content');

    title.textContent = `Block #${details.number.toLocaleString()}`;
    content.innerHTML = `
        <div class="space-y-4">
            <div class="bg-white/5 rounded-lg p-4">
                <div class="text-gray-400 text-sm mb-1">Block Hash</div>
                <div class="font-mono text-sm break-all">${details.hash}</div>
            </div>

            <div class="grid md:grid-cols-2 gap-4">
                <div class="bg-white/5 rounded-lg p-4">
                    <div class="text-gray-400 text-sm mb-1">Parent Hash</div>
                    <div class="font-mono text-xs break-all">${details.parentHash}</div>
                </div>
                <div class="bg-white/5 rounded-lg p-4">
                    <div class="text-gray-400 text-sm mb-1">State Root</div>
                    <div class="font-mono text-xs break-all">${truncateHash(details.stateRoot, 16)}</div>
                </div>
            </div>

            <div class="bg-white/5 rounded-lg p-4">
                <div class="text-gray-400 text-sm mb-1">Extrinsics Root</div>
                <div class="font-mono text-xs break-all">${details.extrinsicsRoot}</div>
            </div>

            <div class="bg-white/5 rounded-lg p-4">
                <h3 class="text-lg font-semibold mb-3">Extrinsics (${details.extrinsics.length})</h3>
                <div class="space-y-2 max-h-64 overflow-y-auto">
                    ${details.extrinsics.map(ext => `
                        <div class="bg-etrid-darker rounded p-3">
                            <div class="flex justify-between items-start mb-2">
                                <span class="text-gray-400 text-sm">#${ext.index}</span>
                                <span class="px-2 py-1 rounded bg-etrid-blue/20 text-etrid-blue text-xs">${ext.section}.${ext.method}</span>
                            </div>
                            <div class="font-mono text-xs text-gray-400 break-all">${ext.hash}</div>
                            <div class="text-xs text-gray-500 mt-1">
                                ${ext.isSigned ? `Signer: ${truncateHash(ext.signer, 12)}` : 'Unsigned'}
                            </div>
                        </div>
                    `).join('')}
                </div>
            </div>
        </div>
    `;

    modal.classList.remove('hidden');
}

// Show transaction details modal
function showTransactionDetails(txHash) {
    const modal = document.getElementById('details-modal');
    const title = document.getElementById('modal-title');
    const content = document.getElementById('modal-content');

    title.textContent = 'Transaction Details';
    content.innerHTML = `
        <div class="bg-white/5 rounded-lg p-4">
            <div class="text-gray-400 text-sm mb-1">Transaction Hash</div>
            <div class="font-mono text-sm break-all">${txHash}</div>
        </div>
        <div class="bg-yellow-500/10 border border-yellow-500/30 rounded-lg p-4 text-sm text-yellow-400">
            Full transaction details will be available when connected to live blockchain
        </div>
    `;

    modal.classList.remove('hidden');
}

// Update statistics
function updateStats() {
    if (latestBlocks.length > 0) {
        document.getElementById('latest-block').textContent = latestBlocks[0].number.toLocaleString();
        document.getElementById('finalized-block').textContent = (latestBlocks[0].number - 2).toLocaleString();

        // Calculate total transactions
        const totalTxs = latestBlocks.reduce((sum, block) => sum + block.extrinsics, 0);
        document.getElementById('total-txs').textContent = totalTxs.toLocaleString();
    }
}

// Mock data when blockchain not available
function showMockBlocks() {
    latestBlocks = Array.from({ length: 10 }, (_, i) => ({
        number: i,
        hash: '0x' + '0'.repeat(64),
        parentHash: '0x' + '0'.repeat(64),
        stateRoot: '0x' + '0'.repeat(64),
        extrinsicsRoot: '0x' + '0'.repeat(64),
        timestamp: new Date().getTime() - (i * 5000),
        extrinsics: 0,
        validator: 'Network Building...',
    }));

    renderBlocks();
    document.getElementById('latest-block').textContent = '0';
    document.getElementById('finalized-block').textContent = '0';
    document.getElementById('total-txs').textContent = '0';
}

// Search functionality
document.getElementById('search-button').addEventListener('click', performSearch);
document.getElementById('search-input').addEventListener('keypress', (e) => {
    if (e.key === 'Enter') performSearch();
});

async function performSearch() {
    const query = document.getElementById('search-input').value.trim();

    if (!query) return;

    // Determine search type
    if (query.startsWith('0x')) {
        // Hash search (block or transaction)
        if (query.length === 66) {
            const details = await fetchBlockDetails(query);
            if (details) {
                showBlockDetails(details.number);
            } else {
                alert('Block or transaction not found');
            }
        }
    } else if (!isNaN(query)) {
        // Block number search
        const blockNumber = parseInt(query);
        showBlockDetails(blockNumber);
    } else {
        // Address search
        alert('Address search coming soon');
    }
}

// Modal close
document.getElementById('close-modal').addEventListener('click', () => {
    document.getElementById('details-modal').classList.add('hidden');
});

// Utility functions
function truncateHash(hash, length = 8) {
    if (!hash) return '0x...';
    return `${hash.substring(0, length + 2)}...${hash.substring(hash.length - length)}`;
}

function formatTimestamp(timestamp) {
    const now = Date.now();
    const diff = now - timestamp;

    if (diff < 60000) return `${Math.floor(diff / 1000)}s ago`;
    if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
    if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
    return new Date(timestamp).toLocaleDateString();
}

// Initialize
(async function init() {
    const connected = await connectToBlockchain();

    if (connected) {
        await fetchLatestBlocks();

        // Subscribe to new blocks
        api.rpc.chain.subscribeNewHeads((header) => {
            console.log(`New block #${header.number}`);
            fetchLatestBlocks();
        });
    } else {
        console.log('Blockchain not available, showing mock data');
        showMockBlocks();
        renderTransactions();
    }
})();
