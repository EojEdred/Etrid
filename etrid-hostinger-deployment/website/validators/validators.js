// ËTRID Validator Leaderboard
// Connects to blockchain and displays validator statistics

const BOOTSTRAP_NODES = [
    'ws://20.186.91.207:9944',  // Alice (Azure VM)
    'ws://172.177.44.73:9944',  // Bob (Azure VM)
];

let api = null;
let validators = [];
let currentFilter = 'all';

// Mock validator data (used when blockchain not available)
const MOCK_VALIDATORS = [
    {
        address: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        name: 'Alice Node',
        location: 'East US',
        stake: 2500000,
        commission: 5.0,
        performance: 99.8,
        blocks: 12453,
        status: 'active',
        apy: 18.5
    },
    {
        address: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        name: 'Bob Node',
        location: 'West US',
        stake: 2300000,
        commission: 7.0,
        performance: 99.5,
        blocks: 11892,
        status: 'active',
        apy: 17.2
    },
    {
        address: '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
        name: 'Charlie Validator',
        location: 'Europe',
        stake: 2100000,
        commission: 3.0,
        performance: 99.9,
        blocks: 12678,
        status: 'active',
        apy: 19.1
    },
    {
        address: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
        name: 'Dave Staking',
        location: 'Asia',
        stake: 1950000,
        commission: 8.0,
        performance: 98.9,
        blocks: 10234,
        status: 'active',
        apy: 16.5
    },
    {
        address: '5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw',
        name: 'Eve Validator',
        location: 'South America',
        stake: 1850000,
        commission: 6.0,
        performance: 99.2,
        blocks: 11456,
        status: 'active',
        apy: 17.8
    },
    {
        address: '5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL',
        name: 'Ferdie Node',
        location: 'Australia',
        stake: 1750000,
        commission: 5.5,
        performance: 99.6,
        blocks: 11789,
        status: 'active',
        apy: 18.1
    },
    {
        address: '5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY',
        name: 'Genesis Validator',
        location: 'Europe',
        stake: 1650000,
        commission: 4.0,
        performance: 99.7,
        blocks: 12012,
        status: 'active',
        apy: 18.8
    },
    {
        address: '5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT',
        name: 'Helix Staking',
        location: 'North America',
        stake: 1550000,
        commission: 7.5,
        performance: 98.5,
        blocks: 9876,
        status: 'active',
        apy: 16.2
    },
    {
        address: '5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc',
        name: 'Infinity Node',
        location: 'Asia',
        stake: 1450000,
        commission: 6.5,
        performance: 99.3,
        blocks: 11234,
        status: 'active',
        apy: 17.5
    },
    {
        address: '5FfBQ3kwXrbdyoqLPvcXRp7ikWydXawpNs2Ceu3WwFdhZ8W4',
        name: 'Juno Validator',
        location: 'Europe',
        stake: 1350000,
        commission: 5.0,
        performance: 99.4,
        blocks: 11567,
        status: 'active',
        apy: 18.0
    },
    {
        address: '5DqAvikdpfRdk5rR35ZobZhqaC5bJXZcEuvzGtexAZP1hU3T',
        name: 'Kronos Staking',
        location: 'Middle East',
        stake: 1250000,
        commission: 8.5,
        performance: 97.8,
        blocks: 8967,
        status: 'active',
        apy: 15.8
    },
    {
        address: '5EsSUH5NpPfLfpbvqHBBfZqGPCPG8sPbyfPGz8pPpvDJE8Pz',
        name: 'Luna Validator',
        location: 'Africa',
        stake: 1150000,
        commission: 4.5,
        performance: 99.1,
        blocks: 11123,
        status: 'active',
        apy: 18.3
    },
    {
        address: '5CRoMkCy6kFZGQ7jf7xrLW4WF5UjU3rkxP7UBkq5zSYkTVWU',
        name: 'Matrix Node',
        location: 'Asia',
        stake: 1100000,
        commission: 6.0,
        performance: 99.0,
        blocks: 10890,
        status: 'active',
        apy: 17.7
    },
    {
        address: '5D5PhZQNJzcJXVBxwJxZcsutjKPqUPydrvpu6HeiBfMaeKQu',
        name: 'Nexus Validator',
        location: 'North America',
        stake: 1050000,
        commission: 7.0,
        performance: 98.7,
        blocks: 10123,
        status: 'active',
        apy: 16.8
    },
    {
        address: '5G9z8Qp8x2fPJJZsqPXRpZFk2XHqLp8xKe9dXnGPQtMJVc3M',
        name: 'Omega Staking',
        location: 'Europe',
        stake: 1000000,
        commission: 5.0,
        performance: 99.2,
        blocks: 11045,
        status: 'active',
        apy: 18.0
    },
    {
        address: '5GbQN7Kj3mKNXVUvHvD9pXDPUqCjEpKHXTQqBe8x3LKPCJvX',
        name: 'Phoenix Node',
        location: 'South America',
        stake: 950000,
        commission: 9.0,
        performance: 97.5,
        blocks: 8456,
        status: 'active',
        apy: 15.2
    },
    {
        address: '5DPqVdFpPxq7G9NbZn8dPvBh9xMYPe9qFfQnKsRx7VjKVuH3',
        name: 'Quantum Validator',
        location: 'Australia',
        stake: 900000,
        commission: 4.0,
        performance: 99.5,
        blocks: 11234,
        status: 'active',
        apy: 18.6
    },
    {
        address: '5HT8ZkRnVk3TkPcQcPJ8xVf9QN8MjKqHr4pFaGCxRtKsL9Ym',
        name: 'Relay Node',
        location: 'Asia',
        stake: 850000,
        commission: 6.5,
        performance: 98.9,
        blocks: 10012,
        status: 'active',
        apy: 17.1
    },
    {
        address: '5C5CjAR9jK8Yq3tLbWxJnPvzP8xQfMnL4KrTbVgPeHxKjLmN',
        name: 'Sigma Staking',
        location: 'Europe',
        stake: 800000,
        commission: 5.5,
        performance: 99.1,
        blocks: 10789,
        status: 'active',
        apy: 17.9
    },
    {
        address: '5EvQKjC4xJPCkCL9pLxPr3gY8RqN8HrVzTbGxXnPQJdLKvMc',
        name: 'Titan Validator',
        location: 'North America',
        stake: 750000,
        commission: 7.5,
        performance: 98.3,
        blocks: 9456,
        status: 'active',
        apy: 16.4
    },
    {
        address: '5HKPmK9GYtE5m6cBvxYeAyGLTTSVk3qQJH7VzCxLpJcXnWfR',
        name: 'Unity Node',
        location: 'Middle East',
        stake: 700000,
        commission: 8.0,
        performance: 97.9,
        blocks: 8890,
        status: 'active',
        apy: 15.9
    },
];

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

// Fetch validators from blockchain
async function fetchValidators() {
    if (!api || !api.isConnected) {
        validators = MOCK_VALIDATORS;
        renderValidators();
        updateStats();
        return;
    }

    try {
        // Get validator addresses
        const validatorAddresses = await api.query.session.validators();

        // Fetch detailed info for each validator
        const validatorData = await Promise.all(
            validatorAddresses.map(async (address, index) => {
                try {
                    // In real implementation, fetch actual validator data
                    // For now, use mock data with real addresses
                    return {
                        address: address.toString(),
                        name: `Validator ${index + 1}`,
                        location: 'Unknown',
                        stake: MOCK_VALIDATORS[index]?.stake || 1000000,
                        commission: MOCK_VALIDATORS[index]?.commission || 5.0,
                        performance: MOCK_VALIDATORS[index]?.performance || 99.0,
                        blocks: MOCK_VALIDATORS[index]?.blocks || 10000,
                        status: 'active',
                        apy: MOCK_VALIDATORS[index]?.apy || 18.0
                    };
                } catch (error) {
                    return null;
                }
            })
        );

        validators = validatorData.filter(v => v !== null);
        renderValidators();
        updateStats();
    } catch (error) {
        console.error('Error fetching validators:', error);
        validators = MOCK_VALIDATORS;
        renderValidators();
        updateStats();
    }
}

// Render validators table
function renderValidators() {
    const container = document.getElementById('validators-list');

    let filteredValidators = validators;
    if (currentFilter === 'active') {
        filteredValidators = validators.filter(v => v.status === 'active');
    } else if (currentFilter === 'high-performance') {
        filteredValidators = validators.filter(v => v.performance >= 99.0);
    }

    // Sort by stake
    filteredValidators.sort((a, b) => b.stake - a.stake);

    container.innerHTML = filteredValidators.map((validator, index) => `
        <div class="grid grid-cols-12 gap-4 p-4 border-b border-white/5 hover:bg-white/5 transition-colors">
            <!-- Rank -->
            <div class="col-span-1 flex items-center">
                <div class="w-8 h-8 rounded-full ${getRankColor(index + 1)} flex items-center justify-center font-bold text-sm">
                    ${index + 1}
                </div>
            </div>

            <!-- Validator Info -->
            <div class="col-span-3 flex items-center">
                <div>
                    <div class="font-semibold">${validator.name}</div>
                    <div class="text-xs text-gray-500 font-mono">${truncateAddress(validator.address)}</div>
                    <div class="text-xs text-gray-400 mt-1">📍 ${validator.location}</div>
                </div>
            </div>

            <!-- Stake -->
            <div class="col-span-2 flex items-center">
                <div>
                    <div class="font-semibold">${formatStake(validator.stake)} ÉTR</div>
                    <div class="text-xs text-gray-400">${formatUSD(validator.stake * 2.45)}</div>
                </div>
            </div>

            <!-- Commission -->
            <div class="col-span-2 flex items-center">
                <div>
                    <div class="font-semibold">${validator.commission}%</div>
                    <div class="text-xs text-gray-400">APY: ${validator.apy}%</div>
                </div>
            </div>

            <!-- Performance -->
            <div class="col-span-2 flex items-center">
                <div class="w-full">
                    <div class="flex justify-between mb-1">
                        <span class="font-semibold">${validator.performance}%</span>
                        <span class="text-xs text-gray-400">${validator.blocks.toLocaleString()} blocks</span>
                    </div>
                    <div class="w-full bg-white/10 rounded-full h-1.5">
                        <div class="bg-gradient-to-r from-etrid-blue to-etrid-purple h-1.5 rounded-full" style="width: ${validator.performance}%"></div>
                    </div>
                </div>
            </div>

            <!-- Status -->
            <div class="col-span-2 flex items-center justify-end">
                <button class="px-4 py-2 rounded-lg bg-gradient-to-r from-etrid-blue to-etrid-purple hover:opacity-90 transition-opacity text-sm font-medium">
                    Stake
                </button>
            </div>
        </div>
    `).join('');
}

// Update network statistics
function updateStats() {
    const totalStake = validators.reduce((sum, v) => sum + v.stake, 0);
    const avgAPY = validators.reduce((sum, v) => sum + v.apy, 0) / validators.length;

    document.getElementById('total-validators').textContent = validators.length;
    document.getElementById('total-staked').textContent = `${formatStake(totalStake)} ÉTR`;
    document.getElementById('staking-ratio').textContent = `${((totalStake / 100000000) * 100).toFixed(1)}%`;
    document.getElementById('staking-apy').textContent = `${avgAPY.toFixed(1)}%`;
}

// Filter button handlers
document.querySelectorAll('.filter-btn').forEach(button => {
    button.addEventListener('click', () => {
        // Update active state
        document.querySelectorAll('.filter-btn').forEach(btn => {
            btn.classList.remove('active', 'bg-etrid-blue/20', 'text-etrid-blue', 'border-etrid-blue/30');
            btn.classList.add('bg-white/5', 'text-gray-400');
        });

        button.classList.add('active', 'bg-etrid-blue/20', 'text-etrid-blue', 'border-etrid-blue/30');
        button.classList.remove('bg-white/5', 'text-gray-400');

        // Update filter
        currentFilter = button.dataset.filter;
        renderValidators();
    });
});

// Utility functions
function getRankColor(rank) {
    if (rank === 1) return 'bg-gradient-to-br from-yellow-400 to-yellow-600';
    if (rank === 2) return 'bg-gradient-to-br from-gray-300 to-gray-500';
    if (rank === 3) return 'bg-gradient-to-br from-orange-400 to-orange-600';
    return 'bg-white/10';
}

function truncateAddress(address) {
    return `${address.substring(0, 8)}...${address.substring(address.length - 8)}`;
}

function formatStake(amount) {
    if (amount >= 1000000) {
        return `${(amount / 1000000).toFixed(2)}M`;
    }
    return `${(amount / 1000).toFixed(0)}K`;
}

function formatUSD(amount) {
    if (amount >= 1000000) {
        return `$${(amount / 1000000).toFixed(2)}M`;
    }
    return `$${(amount / 1000).toFixed(0)}K`;
}

// Initialize
(async function init() {
    const connected = await connectToBlockchain();

    if (connected) {
        await fetchValidators();

        // Refresh every 30 seconds
        setInterval(fetchValidators, 30000);
    } else {
        console.log('Blockchain not available, showing mock data');
        validators = MOCK_VALIDATORS;
        renderValidators();
        updateStats();
    }
})();
