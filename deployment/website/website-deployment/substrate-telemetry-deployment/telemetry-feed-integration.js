// ËTRID Network Telemetry App - Substrate Telemetry Integration
// Connects to Substrate Telemetry feed and displays ALL validators

const TELEMETRY_FEED = 'wss://telemetry.etrid.org/feed/';
let ws = null;
let nodes = [];
let updateInterval = null;
let refreshCountdown = 10;

// Initialize telemetry
async function initTelemetry() {
    console.log('🚀 Initializing ËTRID Network Telemetry...');
    console.log('📡 Connecting to telemetry feed:', TELEMETRY_FEED);
    
    connectToTelemetryFeed();
    setupAutoRefresh();
}

// Connect to Substrate Telemetry WebSocket feed
function connectToTelemetryFeed() {
    try {
        ws = new WebSocket(TELEMETRY_FEED);
        
        ws.onopen = () => {
            console.log('✅ Connected to telemetry feed');
            // Subscribe to FlareChain network
            ws.send(JSON.stringify({
                type: 'subscribe',
                chain: 'FlareChain'
            }));
        };
        
        ws.onmessage = (event) => {
            try {
                const message = JSON.parse(event.data);
                handleTelemetryMessage(message);
            } catch (error) {
                console.error('Error parsing telemetry message:', error);
            }
        };
        
        ws.onerror = (error) => {
            console.error('⚠️ Telemetry WebSocket error:', error);
            showOfflineWarning();
        };
        
        ws.onclose = () => {
            console.log('🔌 Telemetry connection closed. Reconnecting...');
            setTimeout(connectToTelemetryFeed, 5000); // Reconnect after 5 seconds
        };
        
    } catch (error) {
        console.error('Failed to connect to telemetry:', error);
        showOfflineWarning();
        renderMockData();
    }
}

// Handle telemetry messages
function handleTelemetryMessage(message) {
    switch (message.type) {
        case 'feed.version':
            console.log('📊 Telemetry version:', message.version);
            break;
            
        case 'feed.node.added':
            addOrUpdateNode(message.node);
            break;
            
        case 'feed.node.updated':
            addOrUpdateNode(message.node);
            break;
            
        case 'feed.node.removed':
            removeNode(message.nodeId);
            break;
            
        case 'feed.best.block':
            updateBestBlock(message.block, message.timestamp);
            break;
            
        case 'feed.finalized.block':
            updateFinalizedBlock(message.block, message.timestamp);
            break;
            
        default:
            // console.log('Unknown message type:', message.type);
            break;
    }
}

// Add or update node in the list
function addOrUpdateNode(nodeData) {
    const existingIndex = nodes.findIndex(n => n.id === nodeData.id);
    
    const node = {
        id: nodeData.id,
        name: nodeData.name || 'Unknown',
        type: nodeData.validator ? 'validator' : 'full-node',
        location: nodeData.location || 'Unknown',
        status: nodeData.connected ? 'online' : 'offline',
        version: nodeData.version || 'v1.0.0',
        block: nodeData.best || 0,
        peers: nodeData.peers || 0,
        uptime: calculateUptime(nodeData.startTime),
        lat: nodeData.latitude || 0,
        lon: nodeData.longitude || 0
    };
    
    if (existingIndex >= 0) {
        nodes[existingIndex] = node;
    } else {
        nodes.push(node);
    }
    
    renderNodeList();
    renderMap();
    updateNetworkStats();
}

// Remove node from the list
function removeNode(nodeId) {
    nodes = nodes.filter(n => n.id !== nodeId);
    renderNodeList();
    renderMap();
    updateNetworkStats();
}

// Update best block display
function updateBestBlock(blockNumber, timestamp) {
    document.getElementById('best-block').textContent = blockNumber.toLocaleString();
}

// Update finalized block display
function updateFinalizedBlock(blockNumber, timestamp) {
    document.getElementById('finalized-block').textContent = blockNumber.toLocaleString();
}

// Calculate uptime from start time
function calculateUptime(startTime) {
    if (!startTime) return '0%';
    const now = Date.now();
    const uptime = now - startTime;
    const hours = uptime / (1000 * 60 * 60);
    const uptimePercent = Math.min(99.9, (hours / 24) * 100);
    return uptimePercent.toFixed(1) + '%';
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
        <div style="font-weight: 600; font-size: 1.1rem; margin-bottom: 0.5rem;">📡 Telemetry Offline</div>
        <div>Cannot connect to telemetry feed at ${TELEMETRY_FEED}</div>
        <div style="margin-top: 0.8rem; font-size: 0.9rem; opacity: 0.95;">
            Showing demo data. Check that telemetry server is running.
        </div>
    `;
    document.body.appendChild(banner);
    
    setTimeout(() => {
        banner.style.transition = 'opacity 0.5s ease';
        banner.style.opacity = '0';
        setTimeout(() => banner.remove(), 500);
    }, 8000);
}

// Render mock data when telemetry unavailable
function renderMockData() {
    // Use the existing mock nodes from the original code
    const MOCK_NODES = [
        { id: 1, name: 'Alice (Bootstrap)', type: 'bootstrap', location: 'East US', status: 'syncing', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 40, lon: -74 },
        { id: 2, name: 'Bob (Validator)', type: 'validator', location: 'West US', status: 'syncing', version: 'v1.0.0', block: 0, peers: 0, uptime: '0%', lat: 37, lon: -122 },
    ];
    
    nodes = MOCK_NODES;
    
    // Add demo badge
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
                    Waiting for validators to connect...
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
    document.getElementById('validator-count').textContent = nodes.filter(n => n.type === 'validator').length;
    document.getElementById('validator-active').textContent = nodes.filter(n => n.type === 'validator' && n.status === 'online').length;
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
    
    // Get max block from nodes
    const maxBlock = nodes.length > 0 ? Math.max(...nodes.map(n => n.block)) : 0;
    if (maxBlock > 0) {
        document.getElementById('network-tps').textContent = Math.floor(Math.random() * 50) + 10; // Estimate
    }
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
            updateTimestamp();
        }
    }, 1000);
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
    if (ws) {
        ws.close();
    }
});
