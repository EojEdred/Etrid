// ËTRID Telemetry Feed Integration
// Connects to telemetry server WebSocket feed and displays all validators
// Enhanced for ASF consensus, PPFA block production, and committee metrics

const TELEMETRY_FEED = 'ws://98.71.91.84:30334/feed';
let ws = null;
let nodes = [];
let reconnectAttempts = 0;
const MAX_RECONNECT_ATTEMPTS = 10;
const RECONNECT_DELAY = 3000;
const CONNECTION_TIMEOUT = 10000; // 10 seconds
let connectionTimeout = null;
let usingDemoData = false;

// Committee and consensus tracking
let committeeSize = 21;
let currentEpoch = 0;
let lastProposer = null;
let blocksSinceFinality = 0;

// Demo data for fallback
const DEMO_NODES = [
    { id: 1, name: 'Gizzi-Director-01', validator: true, best: 142850, finalized: 142845, peers: 20, version: 'flarechain-1.0', timestamp: Date.now() },
    { id: 2, name: 'EojEdred-Director-02', validator: true, best: 142850, finalized: 142845, peers: 20, version: 'flarechain-1.0', timestamp: Date.now() },
    { id: 3, name: 'Audit-Director-03', validator: true, best: 142850, finalized: 142845, peers: 20, version: 'flarechain-1.0', timestamp: Date.now() },
    { id: 4, name: 'FlareNode-Governance', validator: true, best: 142849, finalized: 142844, peers: 19, version: 'flarechain-1.0', timestamp: Date.now() },
    { id: 5, name: 'FlareNode-Consensus', validator: true, best: 142849, finalized: 142844, peers: 19, version: 'flarechain-1.0', timestamp: Date.now() },
];

// Check for mixed content issues
function checkMixedContent() {
    const isHTTPS = window.location.protocol === 'https:';
    const isWS = TELEMETRY_FEED.startsWith('ws://');
    return isHTTPS && isWS;
}

// Show error banner
function showErrorBanner(type) {
    const banner = document.getElementById('error-banner');
    const bannerText = document.getElementById('error-banner-text');
    if (!banner || !bannerText) return;

    let message = '';
    let link = '';

    if (type === 'mixed-content') {
        message = '⚠️ Connection blocked by browser security (HTTPS→WS). ';
        link = '<a href="http://etrid.org/telemetry/" style="color: #fbbf24; text-decoration: underline;">Use HTTP version</a> or configure WSS.';
    } else if (type === 'timeout') {
        message = '⚠️ Connection timeout. Telemetry server may be offline. ';
        link = 'Showing demo data.';
    } else if (type === 'failed') {
        message = '❌ Connection failed after multiple attempts. ';
        link = '<a href="#" onclick="location.reload()" style="color: #3b82f6; text-decoration: underline;">Click to retry</a>';
    }

    bannerText.innerHTML = message + link;
    banner.style.display = 'flex';

    // Show demo data badge
    const demoBadge = document.getElementById('demo-badge');
    if (demoBadge) demoBadge.style.display = 'block';
}

// Hide error banner
function hideErrorBanner() {
    const banner = document.getElementById('error-banner');
    if (banner) banner.style.display = 'none';

    const demoBadge = document.getElementById('demo-badge');
    if (demoBadge) demoBadge.style.display = 'none';
}

// Initialize connection
function connectToTelemetry() {
    console.log('🔌 Connecting to ËTRID Telemetry...');

    // Check for mixed content
    if (checkMixedContent()) {
        console.warn('⚠️ Mixed content detected: HTTPS→WS');
        showErrorBanner('mixed-content');
        // Try anyway, browser might allow it
    }

    // Set connection timeout
    connectionTimeout = setTimeout(() => {
        console.error('⏱️ Connection timeout');
        if (ws && ws.readyState !== WebSocket.OPEN) {
            ws.close();
            showErrorBanner('timeout');
            loadDemoData();
        }
    }, CONNECTION_TIMEOUT);

    try {
        ws = new WebSocket(TELEMETRY_FEED);

        ws.onopen = () => {
            console.log('✅ Connected to telemetry feed');
            clearTimeout(connectionTimeout);
            reconnectAttempts = 0;
            usingDemoData = false;
            hideErrorBanner();
            updateStatus('Connected - ASF Active', 'success');
        };

        ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);

                if (data.type === 'node_list' || data.type === 'node_update') {
                    nodes = data.nodes || [];
                    usingDemoData = false;
                    analyzeConsensusHealth();
                    updateDisplay();
                }
            } catch (e) {
                console.error('Error parsing telemetry data:', e);
            }
        };

        ws.onerror = (error) => {
            console.error('❌ WebSocket error:', error);
            clearTimeout(connectionTimeout);
            updateStatus('Connection error', 'error');
        };

        ws.onclose = () => {
            console.log('⚠️  Connection closed');
            clearTimeout(connectionTimeout);
            updateStatus('Reconnecting...', 'warning');

            if (reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
                reconnectAttempts++;
                console.log(`🔄 Reconnecting (attempt ${reconnectAttempts})...`);
                setTimeout(connectToTelemetry, RECONNECT_DELAY);
            } else {
                updateStatus('Connection failed', 'error');
                showErrorBanner('failed');
                loadDemoData();
            }
        };
    } catch (error) {
        console.error('❌ Failed to create WebSocket:', error);
        clearTimeout(connectionTimeout);
        showErrorBanner('failed');
        loadDemoData();
    }
}

// Load demo data as fallback
function loadDemoData() {
    console.log('📊 Loading demo data');
    usingDemoData = true;
    nodes = DEMO_NODES;
    analyzeConsensusHealth();
    updateDisplay();

    // Update demo data periodically to simulate activity
    setInterval(() => {
        if (usingDemoData) {
            nodes = nodes.map(node => ({
                ...node,
                best: node.best + Math.floor(Math.random() * 2),
                finalized: node.finalized + (Math.random() > 0.5 ? 1 : 0),
                timestamp: Date.now() - Math.floor(Math.random() * 5000)
            }));
            analyzeConsensusHealth();
            updateDisplay();
        }
    }, 6000);
}

// Analyze consensus health and calculate ASF finality metrics
function analyzeConsensusHealth() {
    const now = Date.now();
    const online = nodes.filter(n => now - n.timestamp < 30000);

    // Track block proposers to identify PPFA rotation
    if (nodes.length > 0) {
        const latest = nodes.reduce((prev, current) =>
            (prev.best || 0) > (current.best || 0) ? prev : current
        );

        if (lastProposer !== latest.name) {
            lastProposer = latest.name;
            console.log(`📦 PPFA Proposer: ${latest.name} at block #${latest.best}`);
        }

        // Calculate blocks since last finality update
        const bestBlock = Math.max(...nodes.map(n => n.best || 0), 0);
        const finalizedBlock = Math.max(...nodes.map(n => n.finalized || 0), 0);
        blocksSinceFinality = bestBlock - finalizedBlock;
    }

    // Estimate current epoch (rough calculation: ~2400 blocks per epoch)
    if (nodes.length > 0) {
        const bestBlock = Math.max(...nodes.map(n => n.best || 0), 0);
        currentEpoch = Math.floor(bestBlock / 2400);
    }
}

// Calculate ASF finality confidence percentage
function calculateFinalityConfidence(blocksSinceFinality) {
    // ASF finality formula: exponential growth based on time/blocks
    // 0-5 blocks: 0-25% finality
    // 6-10 blocks: 25-60% finality
    // 11-20 blocks: 60-85% finality
    // 21+ blocks: 85-99%+ finality

    if (blocksSinceFinality <= 0) return 0;
    if (blocksSinceFinality <= 5) return Math.min(25, blocksSinceFinality * 5);
    if (blocksSinceFinality <= 10) return 25 + ((blocksSinceFinality - 5) * 7);
    if (blocksSinceFinality <= 20) return 60 + ((blocksSinceFinality - 10) * 2.5);
    return Math.min(99.9, 85 + ((blocksSinceFinality - 20) * 0.5));
}

// Update display with latest node data
function updateDisplay() {
    const now = Date.now();
    const validators = nodes.filter(n => n.validator);
    const online = nodes.filter(n => now - n.timestamp < 30000);
    const onlineValidators = validators.filter(n => now - n.timestamp < 30000);

    // Update stats
    const totalEl = document.getElementById('total-validators');
    const onlineEl = document.getElementById('online-validators');
    const committeeEl = document.getElementById('committee-participation');
    const blockEl = document.getElementById('best-block');
    const finalizedEl = document.getElementById('finalized-block');
    const finalityEl = document.getElementById('finality-confidence');
    const epochEl = document.getElementById('current-epoch');
    const proposerEl = document.getElementById('current-proposer');

    if (totalEl) totalEl.textContent = validators.length;
    if (onlineEl) onlineEl.textContent = `${onlineValidators.length}/${committeeSize}`;

    // Committee participation percentage
    const participation = Math.round((onlineValidators.length / committeeSize) * 100);
    if (committeeEl) {
        committeeEl.textContent = `${participation}%`;
        committeeEl.style.color = participation >= 67 ? '#22c55e' : participation >= 50 ? '#fbbf24' : '#ef4444';
    }

    // Get highest block and finalized
    const bestBlock = Math.max(...nodes.map(n => n.best || 0), 0);
    const finalizedBlock = Math.max(...nodes.map(n => n.finalized || 0), 0);

    if (blockEl) blockEl.textContent = bestBlock.toLocaleString();
    if (finalizedEl) finalizedEl.textContent = finalizedBlock.toLocaleString();

    // ASF Finality Confidence
    const finalityConfidence = calculateFinalityConfidence(blocksSinceFinality);
    if (finalityEl) {
        finalityEl.textContent = `${finalityConfidence.toFixed(1)}%`;
        finalityEl.style.color = finalityConfidence >= 85 ? '#22c55e' : finalityConfidence >= 60 ? '#fbbf24' : '#ef4444';
    }

    // Current epoch
    if (epochEl) epochEl.textContent = `#${currentEpoch}`;

    // Current proposer
    if (proposerEl) proposerEl.textContent = lastProposer || 'Detecting...';

    // Update node list
    renderNodeList();

    // Update consensus health indicators
    updateConsensusHealth(participation, finalityConfidence);
}

// Update consensus health section
function updateConsensusHealth(participation, finality) {
    const healthEl = document.getElementById('consensus-health');
    if (!healthEl) return;

    let status = 'Healthy';
    let statusClass = 'healthy';

    if (participation < 67 || finality < 60) {
        status = 'Degraded';
        statusClass = 'warning';
    }

    if (participation < 50 || finality < 40) {
        status = 'Critical';
        statusClass = 'critical';
    }

    healthEl.textContent = status;
    healthEl.className = 'consensus-status ' + statusClass;
}

// Render node list table with ËTRID-specific columns
function renderNodeList() {
    const tbody = document.getElementById('node-list');
    if (!tbody) return;

    const now = Date.now();

    if (nodes.length === 0) {
        tbody.innerHTML = `
            <tr>
                <td colspan="10" style="text-align: center; padding: 40px; color: #9ca3af;">
                    <div style="font-size: 1.2em; margin-bottom: 10px;">⏳ Waiting for validators to connect...</div>
                    <div style="font-size: 0.9em;">Validators will appear here once they start reporting telemetry data.</div>
                    <div style="font-size: 0.85em; margin-top: 10px; color: #6b7280;">ASF Consensus initialized • PPFA committee ready</div>
                </td>
            </tr>
        `;
        return;
    }

    // Determine validator roles based on naming conventions
    const getValidatorRole = (name) => {
        if (name.includes('Gizzi') || name.includes('EojEdred') || name.includes('Audit')) return 'Director';
        if (name.includes('EDSC') || name.includes('Economics') || name.includes('Ethics') || name.includes('Docs')) return 'ValidityNode';
        return 'FlareNode';
    };

    // Determine if validator is current proposer
    const isProposer = (name) => name === lastProposer;

    tbody.innerHTML = nodes
        .sort((a, b) => b.best - a.best) // Sort by block height
        .map((node, index) => {
            const isOnline = now - node.timestamp < 30000;
            const statusClass = isOnline ? 'online' : 'offline';
            const statusText = isOnline ? '🟢 Online' : '🔴 Offline';
            const lastSeen = formatTimestamp(now - node.timestamp);
            const role = getValidatorRole(node.name);
            const proposer = isProposer(node.name);

            // Calculate individual finality lag
            const blockLag = (node.best || 0) - (node.finalized || 0);
            const lagClass = blockLag <= 10 ? 'good' : blockLag <= 20 ? 'warning' : 'bad';

            return `
                <tr class="${statusClass}">
                    <td>
                        <strong>${escapeHtml(node.name || 'Unknown')}</strong>
                        ${proposer ? '<span class="badge proposer">📦 Proposing</span>' : ''}
                    </td>
                    <td><span class="role-badge role-${role.toLowerCase()}">${role}</span></td>
                    <td>${statusText}</td>
                    <td><strong>${(node.best || 0).toLocaleString()}</strong></td>
                    <td>${(node.finalized || 0).toLocaleString()}</td>
                    <td><span class="lag-${lagClass}">${blockLag}</span></td>
                    <td>${node.peers || 0}</td>
                    <td><code>${escapeHtml((node.version || 'Unknown').substring(0, 12))}</code></td>
                    <td><small>${lastSeen}</small></td>
                </tr>
            `;
        })
        .join('');
}

// Update connection status indicator
function updateStatus(message, type) {
    const statusEl = document.getElementById('connection-status');
    if (!statusEl) return;

    statusEl.textContent = message;
    statusEl.className = 'status ' + type;
}

// Format timestamp to human readable
function formatTimestamp(ms) {
    if (ms < 1000) return 'just now';
    if (ms < 60000) return Math.floor(ms / 1000) + 's ago';
    if (ms < 3600000) return Math.floor(ms / 60000) + 'm ago';
    return Math.floor(ms / 3600000) + 'h ago';
}

// Escape HTML to prevent XSS
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', connectToTelemetry);
} else {
    connectToTelemetry();
}

// Refresh display every 2 seconds for responsive UI
setInterval(() => {
    if (nodes.length > 0) {
        updateDisplay();
    }
}, 2000);
