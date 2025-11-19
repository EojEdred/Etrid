/**
 * Etrid Operations Center Dashboard Server
 * Web interface for managing Etrid blockchain infrastructure
 */

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const path = require('path');
const axios = require('axios');
const cookieParser = require('cookie-parser');

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

const PORT = process.env.PORT || 8080;

// Initialize Etrid API with Auth System
const EtridAPI = require('../api/etrid/index.js');
const api = new EtridAPI();

// Middleware
app.use(express.json());
app.use(cookieParser());
app.use(express.static(path.join(__dirname, 'public')));

// API Routes

// ========================================
// Authentication Routes
// ========================================

// Register new user
app.post('/api/auth/register', async (req, res) => {
  try {
    const { name, email, password, organization } = req.body;
    const user = await api.auth.register({ name, email, password, organization });
    res.json({ success: true, user });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Login
app.post('/api/auth/login', async (req, res) => {
  try {
    const { email, password, remember } = req.body;
    const result = await api.auth.login(email, password, remember);

    // Set cookie for web access
    res.cookie('accessToken', result.accessToken, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      maxAge: remember ? 30 * 24 * 60 * 60 * 1000 : 7 * 24 * 60 * 60 * 1000
    });

    res.json(result);
  } catch (err) {
    res.status(401).json({ error: err.message });
  }
});

// Logout
app.post('/api/auth/logout', async (req, res) => {
  try {
    const token = req.headers.authorization?.replace('Bearer ', '') || req.cookies?.accessToken;
    if (token) {
      api.auth.logout(token);
    }
    res.clearCookie('accessToken');
    res.json({ success: true });
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Refresh token
app.post('/api/auth/refresh', async (req, res) => {
  try {
    const { refreshToken } = req.body;
    const result = await api.auth.refreshAccessToken(refreshToken);

    res.cookie('accessToken', result.accessToken, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      maxAge: 7 * 24 * 60 * 60 * 1000
    });

    res.json(result);
  } catch (err) {
    res.status(401).json({ error: err.message });
  }
});

// Get current user
app.get('/api/auth/me', api.auth.middleware(), async (req, res) => {
  res.json({ user: req.user });
});

// Update profile
app.put('/api/auth/profile', api.auth.middleware(), async (req, res) => {
  try {
    const updates = req.body;
    const user = await api.auth.updateProfile(req.user.id, updates);
    res.json({ success: true, user });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Change password
app.post('/api/auth/change-password', api.auth.middleware(), async (req, res) => {
  try {
    const { currentPassword, newPassword } = req.body;
    await api.auth.changePassword(req.user.id, currentPassword, newPassword);
    res.json({ success: true });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Request password reset
app.post('/api/auth/forgot-password', async (req, res) => {
  try {
    const { email } = req.body;
    await api.auth.requestPasswordReset(email);
    res.json({ success: true });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Reset password
app.post('/api/auth/reset-password', async (req, res) => {
  try {
    const { token, newPassword } = req.body;
    await api.auth.resetPassword(token, newPassword);
    res.json({ success: true });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Verify email
app.get('/api/auth/verify-email/:token', async (req, res) => {
  try {
    await api.auth.verifyEmail(req.params.token);
    res.redirect('/login.html?verified=true');
  } catch (err) {
    res.redirect('/login.html?verified=false');
  }
});

// Regenerate API key
app.post('/api/auth/regenerate-api-key', api.auth.middleware(), async (req, res) => {
  try {
    const result = await api.auth.regenerateApiKey(req.user.id);
    res.json(result);
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// ========================================
// User Node Management Routes (Protected)
// ========================================

// Get user's nodes
app.get('/api/user/nodes', api.auth.middleware(), async (req, res) => {
  try {
    const chain = req.query.chain || null;
    const nodes = await api.database.getUserNodes(req.user.id, chain);
    res.json({ nodes });
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Add node to user account
app.post('/api/user/nodes', api.auth.middleware(), async (req, res) => {
  try {
    const { chain, nodeName, nodeConfig } = req.body;

    // Check tier limits
    const userNodes = await api.database.getUserNodes(req.user.id);
    const tierLimits = { free: 5, pro: 50, enterprise: Infinity };
    const limit = tierLimits[req.user.tier] || 5;

    if (userNodes.length >= limit) {
      return res.status(403).json({
        error: 'Node limit reached',
        limit,
        current: userNodes.length,
        tier: req.user.tier
      });
    }

    await api.database.saveUserNode(req.user.id, chain, nodeName, nodeConfig);
    res.json({ success: true });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// Delete user's node
app.delete('/api/user/nodes/:chain/:nodeName', api.auth.middleware(), async (req, res) => {
  try {
    const { chain, nodeName } = req.params;
    await api.database.deleteUserNode(req.user.id, chain, nodeName);
    res.json({ success: true });
  } catch (err) {
    res.status(400).json({ error: err.message });
  }
});

// ========================================
// Protected Node Operations
// ========================================

// Get all node status (Protected)
app.get('/api/status', api.auth.middleware(), async (req, res) => {
  try {
    // Get user's nodes only
    const userNodes = await api.database.getUserNodes(req.user.id);
    const result = await callPinokioAPI('etrid.status', {
      chains: 'all',
      verbose: true,
      userNodes: userNodes.map(n => n.node_name)
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Get health check results (Protected)
app.get('/api/health', api.auth.middleware(), async (req, res) => {
  try {
    const userNodes = await api.database.getUserNodes(req.user.id);
    const result = await callPinokioAPI('etrid.healthcheck', {
      chains: 'all',
      fix: false,
      userNodes: userNodes.map(n => n.node_name)
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Get logs (Protected)
app.get('/api/logs', api.auth.middleware(), async (req, res) => {
  try {
    const { chains = 'all', since = '1h', analyze = false } = req.query;
    const userNodes = await api.database.getUserNodes(req.user.id);
    const result = await callPinokioAPI('etrid.logs', {
      chains,
      since,
      analyze: analyze === 'true',
      userNodes: userNodes.map(n => n.node_name)
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// List nodes (Protected)
app.get('/api/nodes', api.auth.middleware(), async (req, res) => {
  try {
    const userNodes = await api.database.getUserNodes(req.user.id);
    const result = await callPinokioAPI('etrid.list', {
      format: 'json',
      userNodes: userNodes.map(n => n.node_name)
    });
    res.json(result);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
});

// Execute command (Protected)
app.post('/api/exec', api.auth.middleware(), async (req, res) => {
  try {
    const { nodes, command, parallel = true } = req.body;

    // Verify user owns these nodes
    const userNodes = await api.database.getUserNodes(req.user.id);
    const userNodeNames = userNodes.map(n => n.node_name);
    const requestedNodes = Array.isArray(nodes) ? nodes : [nodes];

    const unauthorized = requestedNodes.filter(n => !userNodeNames.includes(n));
    if (unauthorized.length > 0) {
      return res.status(403).json({
        error: 'Access denied to nodes',
        unauthorized
      });
    }

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

// SSH connect (Protected)
app.post('/api/connect', api.auth.middleware(), async (req, res) => {
  try {
    const { node, cloud, command } = req.body;

    // Verify user owns this node
    const userNodes = await api.database.getUserNodes(req.user.id);
    const userNodeNames = userNodes.map(n => n.node_name);

    if (!userNodeNames.includes(node)) {
      return res.status(403).json({ error: 'Access denied to this node' });
    }

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
  const [namespace, methodName] = method.split('.');

  // Call the method on the initialized API instance
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

// WebSocket for real-time updates with authentication
io.on('connection', async (socket) => {
  console.log('Client connected:', socket.id);

  // Authenticate WebSocket connection
  let user = null;
  try {
    const token = socket.handshake.auth.token || socket.handshake.headers.cookie?.match(/accessToken=([^;]+)/)?.[1];
    if (token) {
      const decoded = api.auth.verifyToken(token);
      user = await api.database.getUserById(decoded.userId);
    }
  } catch (err) {
    console.error('WebSocket auth error:', err.message);
  }

  if (!user) {
    socket.emit('error', { message: 'Authentication required' });
    socket.disconnect();
    return;
  }

  // Store user info on socket
  socket.user = user;

  // Start periodic status updates for user's nodes
  const statusInterval = setInterval(async () => {
    try {
      const userNodes = await api.database.getUserNodes(user.id);
      const status = await callPinokioAPI('etrid.status', {
        chains: 'all',
        verbose: false,
        userNodes: userNodes.map(n => n.node_name)
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

  // Handle custom events (with authorization checks)
  socket.on('run-health-check', async (data) => {
    try {
      socket.emit('health-check-started');

      const userNodes = await api.database.getUserNodes(user.id);
      const result = await callPinokioAPI('etrid.healthcheck', {
        chains: data.chains || 'all',
        fix: data.autoFix || false,
        userNodes: userNodes.map(n => n.node_name)
      });

      socket.emit('health-check-complete', result);
    } catch (err) {
      socket.emit('health-check-error', { error: err.message });
    }
  });

  socket.on('fetch-logs', async (data) => {
    try {
      socket.emit('logs-fetching');

      const userNodes = await api.database.getUserNodes(user.id);
      const result = await callPinokioAPI('etrid.logs', {
        chains: data.chains || 'all',
        since: data.since || '1h',
        analyze: data.analyze || false,
        userNodes: userNodes.map(n => n.node_name)
      });

      socket.emit('logs-complete', result);
    } catch (err) {
      socket.emit('logs-error', { error: err.message });
    }
  });
});

// Initialize database and start server
async function startServer() {
  try {
    console.log('🔧 Initializing database...');

    // Initialize database with all tables
    await api.database.init();
    console.log('✅ Database initialized successfully');

    // Start the server
    server.listen(PORT, () => {
      console.log(`🚀 Etrid Operations Center running on http://localhost:${PORT}`);
      console.log(`📊 Dashboard accessible from any browser`);
      console.log(`🔗 Use Pinokio's Local Share for remote access`);
      console.log(`\n💡 Access the dashboard at:`);
      console.log(`   - Login: http://localhost:${PORT}/login.html`);
      console.log(`   - Register: http://localhost:${PORT}/register.html`);
    });
  } catch (err) {
    console.error('❌ Failed to start server:', err);
    process.exit(1);
  }
}

// Start the server
startServer();

// Handle shutdown
process.on('SIGTERM', () => {
  console.log('Shutting down gracefully...');
  server.close(() => {
    console.log('Server closed');
    process.exit(0);
  });
});
