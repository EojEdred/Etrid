const WebSocket = require('ws');
const http = require('http');

// Store connected nodes
const nodes = new Map();
const clients = new Set();

// Create HTTP server for web UI
const server = http.createServer((req, res) => {
    if (req.url === '/') {
        res.writeHead(200, { 'Content-Type': 'text/html' });
        res.end(getWebUI());
    } else if (req.url === '/api/nodes') {
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(Array.from(nodes.values())));
    } else {
        res.writeHead(404);
        res.end('Not found');
    }
});

// WebSocket server for validators to submit data
const wssSubmit = new WebSocket.Server({ server, path: '/submit' });
wssSubmit.on('connection', (ws, req) => {
    console.log(`✅ Validator connected from ${req.socket.remoteAddress}`);
    let nodeId = null;

    ws.on('message', (message) => {
        try {
            // Substrate telemetry sends array-based messages: [id, [type, ...data]]
            const data = JSON.parse(message);

            if (Array.isArray(data) && data.length >= 2) {
                nodeId = data[0]; // First element is node ID
                const payload = data[1]; // Second element is [message_type, ...data]

                if (Array.isArray(payload) && payload.length > 0) {
                    const msgType = payload[0];

                    // Message type 1 or 5: System info
                    // [id, [1, name, impl, version, os, arch, cpu, memory, ...]]
                    if (msgType === 1 || msgType === 5) {
                        const name = payload[1] || 'Unknown';
                        const impl = payload[2] || '';
                        const version = payload[3] || '1.0.0';

                        if (!nodes.has(nodeId)) {
                            nodes.set(nodeId, {
                                id: nodeId,
                                name: name,
                                implementation: impl,
                                version: version,
                                validator: true,
                                best: 0,
                                finalized: 0,
                                peers: 0,
                                txs: 0,
                                timestamp: Date.now()
                            });
                            console.log(`📝 New node registered: ${name} (${nodeId})`);
                        }
                    }

                    // Message type 2: Block imported
                    // [id, [2, best_block, best_hash, timestamp, propagation_time]]
                    if (msgType === 2) {
                        if (nodes.has(nodeId)) {
                            const node = nodes.get(nodeId);
                            node.best = payload[1] || node.best;
                            node.timestamp = Date.now();
                            nodes.set(nodeId, node);
                        }
                    }

                    // Message type 3: Block finalized
                    // [id, [3, finalized_block, finalized_hash]]
                    if (msgType === 3) {
                        if (nodes.has(nodeId)) {
                            const node = nodes.get(nodeId);
                            node.finalized = payload[1] || node.finalized;
                            node.timestamp = Date.now();
                            nodes.set(nodeId, node);
                        }
                    }

                    // Message type 4: Node stats
                    // [id, [4, peers, txpool_size]]
                    if (msgType === 4) {
                        if (nodes.has(nodeId)) {
                            const node = nodes.get(nodeId);
                            node.peers = payload[1] || 0;
                            node.txs = payload[2] || 0;
                            node.timestamp = Date.now();
                            nodes.set(nodeId, node);
                        }
                    }

                    // Broadcast updated node list to web clients
                    broadcastToClients({
                        type: 'node_update',
                        nodes: Array.from(nodes.values())
                    });
                }
            }
        } catch (e) {
            console.error(`❌ Error parsing message:`, e.message);
            if (message) {
                const preview = message.toString().substring(0, 200);
                console.error(`   Raw message preview: ${preview}`);
            }
        }
    });

    ws.on('close', () => {
        console.log(`⚠️  Validator disconnected (${nodeId || 'unknown'})`);
        if (nodeId && nodes.has(nodeId)) {
            // Mark as offline but keep in list for 60 seconds
            setTimeout(() => {
                if (nodes.has(nodeId)) {
                    const node = nodes.get(nodeId);
                    if (Date.now() - node.timestamp > 60000) {
                        nodes.delete(nodeId);
                        console.log(`🗑️  Removed stale node: ${node.name}`);
                    }
                }
            }, 60000);
        }
    });

    ws.on('error', (error) => {
        console.error(`❌ WebSocket error:`, error.message);
    });
});

// WebSocket server for web clients to receive feed
const wssFeed = new WebSocket.Server({ server, path: '/feed' });
wssFeed.on('connection', (ws) => {
    console.log('👁️  Web client connected');
    clients.add(ws);

    // Send current nodes immediately
    ws.send(JSON.stringify({
        type: 'node_list',
        nodes: Array.from(nodes.values())
    }));

    ws.on('close', () => {
        clients.delete(ws);
        console.log('👋 Web client disconnected');
    });
});

function broadcastToClients(data) {
    const message = JSON.stringify(data);
    clients.forEach(client => {
        if (client.readyState === WebSocket.OPEN) {
            client.send(message);
        }
    });
}

function getWebUI() {
    return `
<!DOCTYPE html>
<html><head><title>ËTRID Telemetry</title>
<style>
body { font-family: Arial; background: #0a0e1a; color: #fff; padding: 20px; }
h1 { color: #3b82f6; }
.stats { display: flex; gap: 20px; margin: 20px 0; flex-wrap: wrap; }
.stat { background: #1a1f35; padding: 15px; border-radius: 8px; flex: 1; min-width: 150px; }
.stat-label { color: #9ca3af; font-size: 0.9em; }
.stat-value { color: #3b82f6; font-size: 2em; font-weight: bold; }
.node { background: #1a1f35; padding: 15px; margin: 10px 0; border-radius: 8px; }
.online { border-left: 4px solid #22c55e; }
.offline { border-left: 4px solid #ef4444; opacity: 0.6; }
</style></head><body>
<h1>🚀 ËTRID Network Telemetry</h1>
<div class="stats">
  <div class="stat"><div class="stat-label">Connected Validators</div><div class="stat-value" id="total">0</div></div>
  <div class="stat"><div class="stat-label">Best Block</div><div class="stat-value" id="best">0</div></div>
  <div class="stat"><div class="stat-label">Finalized</div><div class="stat-value" id="finalized">0</div></div>
</div>
<div id="nodes"></div>
<script>
const ws = new WebSocket('ws://' + location.host + '/feed');
ws.onmessage = (e) => {
    const data = JSON.parse(e.data);
    if (data.nodes) {
        const now = Date.now();
        const online = data.nodes.filter(n => now - n.timestamp < 30000);
        document.getElementById('total').textContent = online.length;

        const bestBlock = Math.max(...data.nodes.map(n => n.best || 0), 0);
        const finalized = Math.max(...data.nodes.map(n => n.finalized || 0), 0);
        document.getElementById('best').textContent = bestBlock;
        document.getElementById('finalized').textContent = finalized;

        document.getElementById('nodes').innerHTML = data.nodes.map(n => {
            const isOnline = now - n.timestamp < 30000;
            return '<div class="node ' + (isOnline ? 'online' : 'offline') + '">' +
                '<strong>' + n.name + '</strong> ' + (isOnline ? '🟢' : '🔴') + '<br>' +
                'Block: #' + (n.best || 0) + ' | Finalized: #' + (n.finalized || 0) + ' | Peers: ' + (n.peers || 0) +
                '<br><small>' + n.version + '</small></div>';
        }).join('');
    }
};
ws.onopen = () => console.log('Connected to telemetry');
ws.onerror = (e) => console.error('WebSocket error:', e);
ws.onclose = () => {
    console.log('Disconnected, reconnecting in 3s...');
    setTimeout(() => location.reload(), 3000);
};
</script></body></html>`;
}

// Clean up old nodes every minute
setInterval(() => {
    const now = Date.now();
    for (const [id, node] of nodes.entries()) {
        if (now - node.timestamp > 300000) { // 5 minutes
            nodes.delete(id);
            console.log(`🗑️  Removed stale node: ${node.name} (${id})`);
        }
    }
}, 60000);

const PORT = 8000;
server.listen(PORT, '0.0.0.0', () => {
    console.log(`🚀 ËTRID Telemetry Server running on port ${PORT}`);
    console.log(`📊 Web UI: http://localhost:${PORT}`);
    console.log(`📡 Submit endpoint: ws://localhost:${PORT}/submit`);
    console.log(`📡 Feed endpoint: ws://localhost:${PORT}/feed`);
    console.log(`📋 Substrate telemetry protocol supported`);
});
