// ËTRID Validator Leaderboard
// Fetches real-time validator data from telemetry server

const TELEMETRY_API = 'http://98.71.91.84:8001/api/nodes';
const REFRESH_INTERVAL = 30000; // 30 seconds

let validators = [];
let currentFilter = 'all';
let networkStats = {
    totalValidators: 0,
    onlineValidators: 0,
    bestBlock: 0,
    finalizedBlock: 0,
    totalPeers: 0
};

// Fetch validators from telemetry API
async function fetchValidators() {
    try {
        const response = await fetch(TELEMETRY_API);

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }

        const nodes = await response.json();
        const now = Date.now();

        // Process validator data
        validators = nodes.map((node) => {
            const isOnline = (now - node.timestamp) < 60000; // Online if seen in last 60 seconds
            const performance = node.best > 0 ? ((node.finalized / node.best) * 100) : 0;

            return {
                address: node.network_id || node.id || 'N/A',
                name: node.name || `Validator-${node.id}`,
                location: extractLocation(node.name),
                stake: 1000000, // Default stake (not available from telemetry)
                commission: 5.0, // Default commission
                performance: Math.min(100, performance).toFixed(1),
                blocks: node.best || 0,
                finalizedBlocks: node.finalized || 0,
                peers: node.peers || 0,
                version: node.version || 'unknown',
                status: isOnline ? 'active' : 'offline',
                apy: 18.0, // Default APY
                uptime: calculateUptime(node.startup_time),
                lastSeen: node.timestamp
            };
        });

        // Sort by best block (performance ranking)
        validators.sort((a, b) => b.blocks - a.blocks);

        // Update network statistics
        const onlineValidators = validators.filter(v => v.status === 'active');
        networkStats = {
            totalValidators: validators.length,
            onlineValidators: onlineValidators.length,
            bestBlock: Math.max(...validators.map(v => v.blocks), 0),
            finalizedBlock: Math.max(...validators.map(v => v.finalizedBlocks), 0),
            totalPeers: validators.reduce((sum, v) => sum + v.peers, 0)
        };

        renderValidators();
        updateStats();

        console.log(`✅ Loaded ${validators.length} validators (${onlineValidators.length} online)`);
    } catch (error) {
        console.error('Error fetching validators:', error);
        showError('Unable to connect to telemetry server. Retrying...');
    }
}

// Extract location from validator name (if available)
function extractLocation(name) {
    const locationMap = {
        'alice': 'Azure East US',
        'bob': 'Azure West US',
        'charlie': 'Europe',
        'dave': 'Asia',
        'consensus-dev': 'Developer Network',
        'validator': 'Unknown'
    };

    const lowerName = (name || '').toLowerCase();
    for (const [key, location] of Object.entries(locationMap)) {
        if (lowerName.includes(key)) {
            return location;
        }
    }
    return 'Unknown';
}

// Calculate uptime from startup timestamp
function calculateUptime(startupTime) {
    if (!startupTime) return 'Unknown';

    const start = parseInt(startupTime);
    const now = Date.now();
    const uptimeMs = now - start;

    const days = Math.floor(uptimeMs / (1000 * 60 * 60 * 24));
    const hours = Math.floor((uptimeMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));

    if (days > 0) return `${days}d ${hours}h`;
    return `${hours}h`;
}

// Show error message
function showError(message) {
    const container = document.getElementById('validators-list');
    if (container) {
        container.innerHTML = `
            <div class="col-span-12 p-8 text-center">
                <div class="text-red-400 mb-2">⚠️ ${message}</div>
                <div class="text-gray-500 text-sm">Next refresh in ${REFRESH_INTERVAL / 1000} seconds...</div>
            </div>
        `;
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
    const avgAPY = validators.length > 0
        ? validators.reduce((sum, v) => sum + v.apy, 0) / validators.length
        : 0;

    document.getElementById('total-validators').textContent = networkStats.onlineValidators;
    document.getElementById('total-staked').textContent = `${formatStake(totalStake)} ÉTR`;
    document.getElementById('staking-ratio').textContent = `${((totalStake / 100000000) * 100).toFixed(1)}%`;
    document.getElementById('staking-apy').textContent = `${avgAPY.toFixed(1)}%`;

    // Update additional stats if elements exist
    const bestBlockEl = document.getElementById('best-block');
    const finalizedBlockEl = document.getElementById('finalized-block');
    const totalPeersEl = document.getElementById('total-peers');

    if (bestBlockEl) bestBlockEl.textContent = networkStats.bestBlock.toLocaleString();
    if (finalizedBlockEl) finalizedBlockEl.textContent = networkStats.finalizedBlock.toLocaleString();
    if (totalPeersEl) totalPeersEl.textContent = networkStats.totalPeers;
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
    console.log('🚀 ËTRID Validator Leaderboard - Loading from telemetry...');

    // Initial fetch
    await fetchValidators();

    // Auto-refresh every 30 seconds
    setInterval(fetchValidators, REFRESH_INTERVAL);

    console.log(`♻️ Auto-refresh enabled (every ${REFRESH_INTERVAL / 1000}s)`);
})();
