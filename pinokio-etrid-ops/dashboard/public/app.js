// Etrid Operations Center - Frontend Application

const socket = io();
let currentTab = 'status';

// Initialize
document.addEventListener('DOMContentLoaded', () => {
  initTabs();
  initSocketHandlers();
  initEventHandlers();
  fetchInitialData();
});

// Tab Management
function initTabs() {
  const tabs = document.querySelectorAll('.tab');
  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      const tabName = tab.dataset.tab;
      switchTab(tabName);
    });
  });
}

function switchTab(tabName) {
  // Update tab buttons
  document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
  document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

  // Update tab content
  document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
  document.getElementById(`${tabName}-tab`).classList.add('active');

  currentTab = tabName;

  // Load data for new tab
  if (tabName === 'nodes') {
    fetchNodes();
  }
}

// Socket.IO Handlers
function initSocketHandlers() {
  socket.on('connect', () => {
    updateConnectionStatus(true);
  });

  socket.on('disconnect', () => {
    updateConnectionStatus(false);
  });

  socket.on('status-update', (data) => {
    updateStatus(data);
    updateLastUpdate();
  });

  socket.on('health-check-started', () => {
    document.getElementById('health-results').innerHTML =
      '<p class="loading">Running health check...</p>';
  });

  socket.on('health-check-complete', (data) => {
    displayHealthResults(data);
  });

  socket.on('health-check-error', (data) => {
    document.getElementById('health-results').innerHTML =
      `<p class="status-offline">Error: ${data.error}</p>`;
  });

  socket.on('logs-fetching', () => {
    document.getElementById('logs-content').innerHTML =
      '<p class="loading">Fetching logs...</p>';
  });

  socket.on('logs-complete', (data) => {
    displayLogs(data);
  });

  socket.on('logs-error', (data) => {
    document.getElementById('logs-content').innerHTML =
      `<p class="status-offline">Error: ${data.error}</p>`;
  });
}

// Event Handlers
function initEventHandlers() {
  // Health check button
  document.getElementById('run-health-check').addEventListener('click', () => {
    socket.emit('run-health-check', {
      chains: 'all',
      autoFix: false
    });
  });

  // Fetch logs button
  document.getElementById('fetch-logs').addEventListener('click', () => {
    const chain = document.getElementById('log-chain').value;
    const timeframe = document.getElementById('log-timeframe').value;

    socket.emit('fetch-logs', {
      chains: chain,
      since: timeframe,
      analyze: false
    });
  });

  // Analyze logs button
  document.getElementById('analyze-logs').addEventListener('click', () => {
    const chain = document.getElementById('log-chain').value;
    const timeframe = document.getElementById('log-timeframe').value;

    document.getElementById('logs-content').innerHTML =
      '<p class="loading">Fetching and analyzing logs with AI...</p>';

    socket.emit('fetch-logs', {
      chains: chain,
      since: timeframe,
      analyze: true
    });
  });
}

// Data Fetching
async function fetchInitialData() {
  await fetchStatus();
}

async function fetchStatus() {
  try {
    const response = await fetch('/api/status');
    const data = await response.json();
    updateStatus(data);
    updateLastUpdate();
  } catch (err) {
    console.error('Error fetching status:', err);
  }
}

async function fetchNodes() {
  try {
    const response = await fetch('/api/nodes');
    const data = await response.json();
    displayNodes(data);
  } catch (err) {
    console.error('Error fetching nodes:', err);
    document.getElementById('nodes-list').innerHTML =
      '<p class="status-offline">Error loading nodes</p>';
  }
}

// UI Update Functions
function updateConnectionStatus(connected) {
  const badge = document.getElementById('connection-status');
  if (connected) {
    badge.textContent = 'Connected';
    badge.classList.remove('offline');
    badge.classList.add('online');
  } else {
    badge.textContent = 'Disconnected';
    badge.classList.remove('online');
    badge.classList.add('offline');
  }
}

function updateLastUpdate() {
  const now = new Date();
  const timeStr = now.toLocaleTimeString();
  document.getElementById('last-update').textContent = `Last update: ${timeStr}`;
}

function updateStatus(data) {
  const result = data.result || data;

  // Update summary
  if (result.summary) {
    document.getElementById('total-nodes').textContent = result.summary.total || 0;
    document.getElementById('online-nodes').textContent = result.summary.online || 0;
    document.getElementById('syncing-nodes').textContent = result.summary.syncing || 0;
    document.getElementById('offline-nodes').textContent = result.summary.offline || 0;
  }

  // Update FlareChain status
  if (result.flarechain) {
    displayChainStatus('flarechain-status', result.flarechain);
  }

  // Update PBC status
  if (result.pbcs && result.pbcs.length > 0) {
    const pbcHtml = result.pbcs
      .map(pbc => {
        const status = pbc.status?.nodes?.[0]?.status || 'unknown';
        const statusClass = `status-${status}`;
        return `
          <div class="status-item">
            <span>${pbc.name}</span>
            <span class="${statusClass}">${status}</span>
          </div>
        `;
      })
      .join('');
    document.getElementById('pbc-status').innerHTML = pbcHtml;
  }

  // Update detailed status
  displayDetailedStatus(result);
}

function displayChainStatus(elementId, chainData) {
  const element = document.getElementById(elementId);

  if (!chainData.nodes || chainData.nodes.length === 0) {
    element.innerHTML = '<p class="info">No nodes configured</p>';
    return;
  }

  const html = chainData.nodes
    .map(node => {
      const statusClass = `status-${node.status}`;
      return `
        <div class="status-item">
          <span>${node.name}</span>
          <span class="${statusClass}">${node.status}</span>
        </div>
      `;
    })
    .join('');

  element.innerHTML = html;
}

function displayDetailedStatus(data) {
  const container = document.getElementById('detailed-status');

  let html = '<div class="nodes-table-container"><table class="nodes-table">';
  html += '<thead><tr><th>Chain</th><th>Node</th><th>Status</th><th>Block</th><th>Peers</th></tr></thead><tbody>';

  // FlareChain nodes
  if (data.flarechain?.nodes) {
    data.flarechain.nodes.forEach(node => {
      html += createNodeRow('FlareChain', node);
    });
  }

  // PBC nodes
  if (data.pbcs) {
    data.pbcs.forEach(pbc => {
      if (pbc.status?.nodes) {
        pbc.status.nodes.forEach(node => {
          html += createNodeRow(pbc.name, node);
        });
      }
    });
  }

  html += '</tbody></table></div>';
  container.innerHTML = html;
}

function createNodeRow(chain, node) {
  const statusClass = `status-${node.status}`;
  return `
    <tr>
      <td>${chain}</td>
      <td>${node.name}</td>
      <td class="${statusClass}">${node.status}</td>
      <td>${node.blockHeight || '-'}</td>
      <td>${node.peers !== null ? node.peers : '-'}</td>
    </tr>
  `;
}

function displayHealthResults(data) {
  const results = data.result || data;
  const container = document.getElementById('health-results');

  if (!results.chains) {
    container.innerHTML = '<p class="info">No health data available</p>';
    return;
  }

  let html = '<div class="health-summary">';

  // Overall summary
  html += `<h4>Summary</h4>`;
  html += `<p>Total Chains: ${Object.keys(results.chains).length}</p>`;
  html += `<p>Critical Issues: <span class="status-offline">${results.criticalIssues?.length || 0}</span></p>`;
  html += `<p>Warnings: <span class="status-syncing">${results.issues?.length || 0}</span></p>`;

  // Critical issues
  if (results.criticalIssues && results.criticalIssues.length > 0) {
    html += '<h4 class="status-offline">Critical Issues</h4>';
    html += '<ul>';
    results.criticalIssues.forEach(issue => {
      html += `<li><strong>${issue.chain}:</strong> ${issue.message}</li>`;
    });
    html += '</ul>';
  }

  // Warnings
  if (results.issues && results.issues.length > 0) {
    html += '<h4 class="status-syncing">Warnings</h4>';
    html += '<ul>';
    results.issues.forEach(issue => {
      html += `<li><strong>${issue.chain}:</strong> ${issue.message}</li>`;
    });
    html += '</ul>';
  }

  // All good
  if ((!results.criticalIssues || results.criticalIssues.length === 0) &&
      (!results.issues || results.issues.length === 0)) {
    html += '<p class="status-online">✅ All systems healthy!</p>';
  }

  html += '</div>';
  container.innerHTML = html;
}

function displayLogs(data) {
  const logs = data.result || data;
  const container = document.getElementById('logs-content');

  if (!logs.logs) {
    container.innerHTML = '<p class="info">No logs available</p>';
    return;
  }

  let html = '';

  // If analysis is included
  if (logs.analysis) {
    html += '<div class="log-analysis">';
    html += '<h4>🤖 AI Analysis</h4>';
    html += `<pre>${logs.analysis}</pre>`;
    html += '</div><hr>';
  }

  // Display logs
  for (const [chain, chainLogs] of Object.entries(logs.logs)) {
    html += `<h4>${chain}</h4>`;

    chainLogs.forEach(nodeLog => {
      html += `<h5>${nodeLog.node}</h5>`;

      if (nodeLog.error) {
        html += `<p class="log-error">Error: ${nodeLog.error}</p>`;
      } else if (nodeLog.lines) {
        nodeLog.lines.forEach(line => {
          const lineClass = line.toLowerCase().includes('error') ? 'log-error' :
                           line.toLowerCase().includes('warn') ? 'log-warning' : '';
          html += `<div class="log-line ${lineClass}">${escapeHtml(line)}</div>`;
        });
      }
    });
  }

  container.innerHTML = html;
}

function displayNodes(nodes) {
  const container = document.getElementById('nodes-list');

  if (!nodes || nodes.length === 0) {
    container.innerHTML = '<p class="info">No nodes configured</p>';
    return;
  }

  let html = '<table class="nodes-table">';
  html += '<thead><tr><th>Name</th><th>Chain</th><th>Cloud</th><th>IP</th><th>Actions</th></tr></thead>';
  html += '<tbody>';

  nodes.forEach(node => {
    html += `
      <tr>
        <td>${node.name}</td>
        <td>${node.chain || '-'}</td>
        <td>${node.cloud || '-'}</td>
        <td>${node.ip || '-'}</td>
        <td>
          <button class="btn" onclick="connectToNode('${node.name}', '${node.cloud}')">SSH</button>
        </td>
      </tr>
    `;
  });

  html += '</tbody></table>';
  container.innerHTML = html;
}

// Utility Functions
function escapeHtml(text) {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

async function connectToNode(nodeName, cloud) {
  const command = prompt(`Enter command to run on ${nodeName} (or leave empty for interactive):`);

  try {
    const response = await fetch('/api/connect', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ node: nodeName, cloud, command })
    });

    const result = await response.json();
    alert(`Connected to ${nodeName}\n\nResult:\n${JSON.stringify(result, null, 2)}`);
  } catch (err) {
    alert(`Error: ${err.message}`);
  }
}

// Auto-refresh status every 30 seconds
setInterval(() => {
  if (currentTab === 'status') {
    fetchStatus();
  }
}, 30000);
