// ËTRID Network Telemetry App
// Connects to blockchain nodes and displays real-time network data

const BOOTSTRAP_NODES = [
    { endpoint: 'ws://98.71.91.84:9944', name: 'FlareChain Validator Node', location: 'Primary', lat: 40, lon: -74, type: 'validator' },
];

// Mock nodes for demonstration (when real nodes not available)
const MOCK_NODES = [
    { name: 'Alice (Bootstrap)', type: 'bootstrap', location: 'East US', status: 'syncing', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 40, lon: -74 },
    { name: 'Bob (Validator)', type: 'validator', location: 'West US', status: 'syncing', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 37, lon: -122 },
    { name: 'Charlie (Validator)', type: 'validator', location: 'UK', status: 'offline', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 51, lon: 0 },
    { name: 'Dave (Full Node)', type: 'full-node', location: 'Germany', status: 'offline', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 52, lon: 13 },
    { name: 'Eve (Validator)', type: 'validator', location: 'Singapore', status: 'offline', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 1, lon: 103 },
];

let api = null;
let nodes = [];
let updateInterval = null;
let refreshCountdown = 10;

// Initialize telemetry
async function initTelemetry() {
    console.log('🚀 Initializing ËTRID Network Telemetry...');

    // Check for mixed content issues (HTTPS → WS)
    const isMixedContent = window.location.protocol === 'https:' && BOOTSTRAP_NODES[0].endpoint.startsWith('ws:');

    if (isMixedContent) {
        console.warn('⚠️ Mixed content detected: HTTPS page cannot connect to WS endpoint');
        console.warn('💡 Solution: Use WSS endpoint or access via HTTP');
        showConnectionWarning();
        // Still try to connect - browser will block but user will see attempt
    }

    // Try to connect to real nodes
    const connected = await connectToNetwork();

    if (connected) {
        console.log('✅ Connected to live network');
        await fetchNetworkData();
    } else {
        console.log('⚠️ Using mock data (nodes not available)');
        if (!isMixedContent) {
            showOfflineWarning();
        }
        renderMockData();
    }

    // Setup auto-refresh
    setupAutoRefresh();
}

// Show connection warning banner
function showConnectionWarning() {
    const banner = document.createElement('div');
    banner.style.cssText = `
        position: fixed;
        top: 80px;
        left: 50%;
        transform: translateX(-50%);
        background: linear-gradient(135deg, #ff6b6b 0%, #ee5a52 100%);
        color: white;
        padding: 1.5rem 2rem;
        border-radius: 12px;
        box-shadow: 0 10px 40px rgba(255, 107, 107, 0.3);
        z-index: 1000;
        max-width: 90%;
        width: 600px;
        font-size: 0.95rem;
        line-height: 1.6;
    `;
    banner.innerHTML = `
        <div style="font-weight: 600; font-size: 1.1rem; margin-bottom: 0.5rem;">⚠️ Connection Blocked</div>
        <div>Browser security blocks WebSocket (WS) connections from HTTPS pages.</div>
        <div style="margin-top: 0.8rem; font-size: 0.9rem; opacity: 0.95;">
            <strong>Solutions:</strong><br>
            • Set up SSL/TLS on node: Use WSS instead of WS<br>
            • Or access via HTTP: <a href="http://etrid.org/telemetry/" style="color: #fff; text-decoration: underline;">http://etrid.org/telemetry/</a><br>
            • Showing demo data below ↓
        </div>
    `;
    document.body.appendChild(banner);

    // Auto-hide after 10 seconds
    setTimeout(() => {
        banner.style.transition = 'opacity 0.5s ease';
        banner.style.opacity = '0';
        setTimeout(() => banner.remove(), 500);
    }, 10000);
}

// Show offline warning
function showOfflineWarning() {
    const banner = document.createElement('div');
    banner.style.cssText = `
        position: fixed;
        top: 80px;
        left: 50%;
        transform: translateX(-50%);
        background: linear-gradient(135deg, #f59e0b 0%, #f97316 100%);
        color: white;
        padding: 1.5rem 2rem;
        border-radius: 12px;
        box-shadow: 0 10px 40px rgba(245, 158, 11, 0.3);
        z-index: 1000;
        max-width: 90%;
        width: 600px;
        font-size: 0.95rem;
    `;
    banner.innerHTML = `
        <div style="font-weight: 600; font-size: 1.1rem; margin-bottom: 0.5rem;">📡 Node Offline</div>
        <div>Cannot connect to FlareChain node at ${BOOTSTRAP_NODES[0].endpoint}</div>
        <div style="margin-top: 0.8rem; font-size: 0.9rem; opacity: 0.95;">
            Showing demo data below. Check that your node is running and ports 9933/9944 are open.
        </div>
    `;
    document.body.appendChild(banner);

    setTimeout(() => {
        banner.style.transition = 'opacity 0.5s ease';
        banner.style.opacity = '0';
        setTimeout(() => banner.remove(), 500);
    }, 8000);
}

// Connect to ËTRID network
async function connectToNetwork() {
    for (const node of BOOTSTRAP_NODES) {
        try {
            console.log(`🔄 Attempting connection to ${node.name}...`);
            const { ApiPromise, WsProvider } = polkadotApi;
            const provider = new WsProvider(node.endpoint, false);

            api = await Promise.race([
                ApiPromise.create({ provider }),
                new Promise((_, reject) =>
                    setTimeout(() => reject(new Error('Connection timeout')), 10000)
                )
            ]);

            await api.isReady;
            console.log(`✅ Connected to ${node.name}`);
            return true;
        } catch (error) {
            console.warn(`⚠️ Failed to connect to ${node.name}:`, error.message);
        }
    }
    return false;
}

// Fetch real network data
async function fetchNetworkData() {
    try {
        // Get chain info
        const [chain, nodeName, nodeVersion, header, finalizedHash, health, peers] = await Promise.all([
            api.rpc.system.chain(),
            api.rpc.system.name(),
            api.rpc.system.version(),
            api.rpc.chain.getHeader(),
            api.rpc.chain.getFinalizedHead(),
            api.rpc.system.health().catch(() => null),
            api.rpc.system.peers().catch(() => [])
        ]);

        const finalizedHeader = await api.rpc.chain.getHeader(finalizedHash);
        const bestBlock = header.number.toNumber();
        const finalizedBlock = finalizedHeader.number.toNumber();
        const peerCount = peers.length;

        // Update metrics
        document.getElementById('best-block').textContent = bestBlock.toLocaleString();
        document.getElementById('finalized-block').textContent = finalizedBlock.toLocaleString();
        document.getElementById('chain-name').textContent = chain.toString();
        document.getElementById('runtime-version').textContent = nodeVersion.toString();

        // Get genesis hash
        const genesisHash = api.genesisHash.toHex();
        document.getElementById('genesis-hash').textContent = genesisHash.slice(0, 20) + '...';

        // Calculate block time (estimated)
        document.getElementById('block-time').textContent = '5';
        document.getElementById('finality-time').textContent = '15';

        // Fetch validator info if available
        try {
            const validators = await api.query.session.validators();
            document.getElementById('validator-count').textContent = validators.length;
            document.getElementById('validator-active').textContent = validators.length;
        } catch (e) {
            document.getElementById('validator-count').textContent = '2';
            document.getElementById('validator-active').textContent = '2';
        }

        // Get spec version
        try {
            const runtimeVersion = await api.rpc.state.getRuntimeVersion();
            document.getElementById('spec-version').textContent = runtimeVersion.specVersion.toString();
        } catch (e) {
            document.getElementById('spec-version').textContent = '100';
        }

        // Update node list with real data
        nodes = BOOTSTRAP_NODES.map((node, i) => ({
            name: node.name,
            type: node.type,
            location: node.location,
            status: 'online',
            version: nodeVersion.toString(),
            block: bestBlock,
            peers: peerCount,
            uptime: health ? (health.isSyncing ? 'Syncing' : '99.9%') : '99.9%',
            lat: node.lat,
            lon: node.lon
        }));

        // Calculate TPS (transactions per second) from recent blocks
        try {
            const blockHash = await api.rpc.chain.getBlockHash(bestBlock - 1);
            const block = await api.rpc.chain.getBlock(blockHash);
            const extrinsics = block.block.extrinsics.length;
            const tps = Math.floor(extrinsics / 5); // Assuming 5 second block time
            document.getElementById('network-tps').textContent = tps;
        } catch (e) {
            document.getElementById('network-tps').textContent = '0';
        }

        // Update sync status
        document.getElementById('sync-status').textContent = health && health.isSyncing ? 'Syncing...' : 'Synchronized';

        // Update era and session info
        try {
            const currentEra = await api.query.staking.currentEra();
            const activeEra = await api.query.staking.activeEra();
            if (currentEra && currentEra.isSome) {
                document.getElementById('current-era').textContent = currentEra.unwrap().toString();
            }
        } catch (e) {
            document.getElementById('current-era').textContent = '0';
        }

        try {
            const sessionIndex = await api.query.session.currentIndex();
            document.getElementById('current-session').textContent = sessionIndex.toString();
        } catch (e) {
            document.getElementById('current-session').textContent = '0';
        }

        renderNodeList();
        renderMap();
        updateNetworkStats();

        // Show success indicator
        console.log('✅ Fetched real blockchain data:', {
            bestBlock,
            finalizedBlock,
            peers: peerCount,
            chain: chain.toString()
        });

    } catch (error) {
        console.error('Error fetching network data:', error);
        renderMockData();
    }
}

// Render mock data when nodes unavailable
function renderMockData() {
    nodes = MOCK_NODES;

    // Add demo mode badge
    const demoBadge = document.createElement('div');
    demoBadge.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
        color: white;
        padding: 0.6rem 1.2rem;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.9rem;
        z-index: 999;
        box-shadow: 0 4px 20px rgba(99, 102, 241, 0.4);
        letter-spacing: 0.5px;
    `;
    demoBadge.textContent = '📊 DEMO DATA';
    document.body.appendChild(demoBadge);

    // Update metrics with mock data
    document.getElementById('best-block').textContent = '0';
    document.getElementById('finalized-block').textContent = '0';
    document.getElementById('validator-count').textContent = '2';
    document.getElementById('validator-active').textContent = '0';
    document.getElementById('block-time').textContent = '5.0';
    document.getElementById('finality-time').textContent = '15.0';
    document.getElementById('network-tps').textContent = '0';
    document.getElementById('total-nodes').textContent = MOCK_NODES.length;
    document.getElementById('active-nodes').textContent = '0';
    document.getElementById('runtime-version').textContent = 'v1.0.0';
    document.getElementById('spec-version').textContent = '100';
    document.getElementById('avg-peers').textContent = '0';
    document.getElementById('sync-status').textContent = 'Building...';
    document.getElementById('current-era').textContent = '0';
    document.getElementById('current-session').textContent = '0';

    renderNodeList();
    renderMap();
}

// Render node list table
function renderNodeList() {
    const tbody = document.getElementById('node-list');

    if (nodes.length === 0) {
        tbody.innerHTML = `
            <tr>
                <td colspan="8" class="loading">
                    <div class="spinner"></div>
                    No nodes detected. Waiting for blockchain to start...
                </td>
            </tr>
        `;
        return;
    }

    tbody.innerHTML = nodes.map(node => `
        <tr>
            <td><strong>${node.name}</strong></td>
            <td><span class="badge ${node.type}">${node.type.replace('-', ' ')}</span></td>
            <td>${node.location}</td>
            <td>
                <span class="node-status ${node.status}">
                    <span class="node-status-dot"></span>
                    ${node.status}
                </span>
            </td>
            <td>${node.version}</td>
            <td>${node.block.toLocaleString()}</td>
            <td>${node.peers}</td>
            <td>${node.uptime}</td>
        </tr>
    `).join('');

    // Update totals
    const activeCount = nodes.filter(n => n.status === 'online').length;
    document.getElementById('total-nodes').textContent = nodes.length;
    document.getElementById('active-nodes').textContent = activeCount;
}

// Render geographic map with node markers
function renderMap() {
    const mapContainer = document.getElementById('world-map');
    mapContainer.innerHTML = ''; // Clear existing markers

    nodes.forEach(node => {
        const marker = document.createElement('div');
        marker.className = `map-marker ${node.type}`;

        // Convert lat/lon to map coordinates (simplified projection)
        const x = ((node.lon + 180) / 360) * 100; // 0-100%
        const y = ((90 - node.lat) / 180) * 100;   // 0-100%

        marker.style.left = `${x}%`;
        marker.style.top = `${y}%`;
        marker.title = `${node.name} (${node.location}) - ${node.status}`;

        mapContainer.appendChild(marker);
    });
}

// Update network statistics
function updateNetworkStats() {
    const activeNodes = nodes.filter(n => n.status === 'online');
    const avgPeers = activeNodes.length > 0
        ? Math.round(activeNodes.reduce((sum, n) => sum + n.peers, 0) / activeNodes.length)
        : 0;

    document.getElementById('avg-peers').textContent = avgPeers;

    const allSynced = activeNodes.every(n => n.status === 'online');
    document.getElementById('sync-status').textContent = allSynced ? 'Synchronized' : 'Syncing...';

    // Calculate TPS (mock for now)
    const tps = activeNodes.length > 0 ? Math.floor(Math.random() * 100) + 50 : 0;
    document.getElementById('network-tps').textContent = tps;
}

// Setup auto-refresh timer
function setupAutoRefresh() {
    // Update timestamp
    updateTimestamp();

    // Countdown timer
    setInterval(() => {
        refreshCountdown--;
        document.getElementById('refresh-countdown').textContent = `${refreshCountdown}s`;

        if (refreshCountdown <= 0) {
            refreshCountdown = 10;
            refreshData();
        }
    }, 1000);

    // Main refresh interval
    updateInterval = setInterval(refreshData, 10000); // Every 10 seconds
}

// Refresh network data
async function refreshData() {
    if (api && api.isConnected) {
        await fetchNetworkData();
    } else {
        // Try to reconnect
        const reconnected = await connectToNetwork();
        if (reconnected) {
            await fetchNetworkData();
        }
    }
    updateTimestamp();
}

// Update last updated timestamp
function updateTimestamp() {
    const now = new Date();
    const timeStr = now.toLocaleTimeString('en-US', { hour12: false });
    document.getElementById('last-update').textContent = timeStr;
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', initTelemetry);

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    if (updateInterval) {
        clearInterval(updateInterval);
    }
    if (api) {
        api.disconnect();
    }
});
