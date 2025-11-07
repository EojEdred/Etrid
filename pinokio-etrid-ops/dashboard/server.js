/**
 * Etrid Operations Center Dashboard Server
 * Web interface for managing Etrid blockchain infrastructure
 */

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const path = require('path');
const axios = require('axios');

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

const PORT = process.env.PORT || 8080;

// Middleware
app.use(express.json());
app.use(express.static(path.join(__dirname, 'public')));

// API Routes

// Get all node status
app.get('/api/status', async (req, res) => {
  try {
    // Call Pinokio API
    const result = await callPinokioAPI('etrid.status', {
      chains: 'all',
      verbose: true
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Get health check results
app.get('/api/health', async (req, res) => {
  try {
    const result = await callPinokioAPI('etrid.healthcheck', {
      chains: 'all',
      fix: false
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Get logs
app.get('/api/logs', async (req, res) => {
  try {
    const { chains = 'all', since = '1h', analyze = false } = req.query;
    const result = await callPinokioAPI('etrid.logs', {
      chains,
      since,
      analyze: analyze === 'true'
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// List nodes
app.get('/api/nodes', async (req, res) => {
  try {
    const result = await callPinokioAPI('etrid.list', {
      format: 'json'
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Execute command
app.post('/api/exec', async (req, res) => {
  try {
    const { nodes, command, parallel = true } = req.body;
    const result = await callPinokioAPI('etrid.exec', {
      nodes,
      command,
      parallel
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// SSH connect
app.post('/api/connect', async (req, res) => {
  try {
    const { node, cloud, command } = req.body;
    const result = await callPinokioAPI('etrid.connect', {
      node,
      cloud,
      command
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Helper function to call Pinokio API
async function callPinokioAPI(method, params) {
  // This is a placeholder - in practice, you'd call the Pinokio API
  // through the kernel or via IPC/RPC
  const [namespace, methodName] = method.split('.');

  // Load the API module
  const EtridAPI = require('../api/etrid/index.js');
  const api = new EtridAPI();

  // Call the method
  const outputs = [];
  const result = await api[methodName](
    { params },
    (data) => {
      outputs.push(data);
    },
    null // kernel
  );

  return {
    result,
    outputs
  };
}

// WebSocket for real-time updates
io.on('connection', (socket) => {
  console.log('Client connected:', socket.id);

  // Start periodic status updates
  const statusInterval = setInterval(async () => {
    try {
      const status = await callPinokioAPI('etrid.status', {
        chains: 'all',
        verbose: false
      });
      socket.emit('status-update', status);
    } catch (err) {
      console.error('Error fetching status:', err);
    }
  }, 30000); // Every 30 seconds

  socket.on('disconnect', () => {
    console.log('Client disconnected:', socket.id);
    clearInterval(statusInterval);
  });

  // Handle custom events
  socket.on('run-health-check', async (data) => {
    try {
      socket.emit('health-check-started');

      const result = await callPinokioAPI('etrid.healthcheck', {
        chains: data.chains || 'all',
        fix: data.autoFix || false
      });

      socket.emit('health-check-complete', result);
    } catch (err) {
      socket.emit('health-check-error', { error: err.message });
    }
  });

  socket.on('fetch-logs', async (data) => {
    try {
      socket.emit('logs-fetching');

      const result = await callPinokioAPI('etrid.logs', {
        chains: data.chains || 'all',
        since: data.since || '1h',
        analyze: data.analyze || false
      });

      socket.emit('logs-complete', result);
    } catch (err) {
      socket.emit('logs-error', { error: err.message });
    }
  });
});

// Start server
server.listen(PORT, () => {
  console.log(`🚀 Etrid Operations Center running on http://localhost:${PORT}`);
  console.log(`📊 Dashboard accessible from any browser`);
  console.log(`🔗 Use Pinokio's Local Share for remote access`);
});

// Handle shutdown
process.on('SIGTERM', () => {
  console.log('Shutting down gracefully...');
  server.close(() => {
    console.log('Server closed');
    process.exit(0);
  });
});
