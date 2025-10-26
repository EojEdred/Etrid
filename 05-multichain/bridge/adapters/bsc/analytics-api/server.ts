import express from "express";
import cors from "cors";
import {
  getLatestMetrics,
  getLatestPools,
  getTVLHistory,
  getPoolTVLHistory,
  getAPRHistory,
  getRecentEvents,
  getActiveAlerts,
  getDatabaseStats,
} from "../scripts/lib/database";

/**
 * Analytics REST API
 *
 * Exposes MasterChef metrics via HTTP endpoints
 *
 * Endpoints:
 *   GET  /api/metrics/:network          - Latest metrics
 *   GET  /api/pools/:network             - Latest pool data
 *   GET  /api/tvl/:network               - TVL history
 *   GET  /api/tvl/:network/pool/:poolId  - Pool TVL history
 *   GET  /api/apr/:network/pool/:poolId  - Pool APR history
 *   GET  /api/events/:network            - Recent events
 *   GET  /api/alerts/:network            - Active alerts
 *   GET  /api/stats                      - Database statistics
 *   GET  /api/health                     - Health check
 *   GET  /metrics                        - Prometheus metrics
 *
 * Usage:
 *   npm run api
 */

const app = express();
const PORT = parseInt(process.env.API_PORT || "3000");

// Middleware
app.use(cors());
app.use(express.json());

// Request logging
app.use((req, res, next) => {
  console.log(`${new Date().toISOString()} - ${req.method} ${req.path}`);
  next();
});

// ===== API Endpoints =====

// Latest metrics
app.get("/api/metrics/:network", (req, res) => {
  try {
    const { network } = req.params;

    if (network !== "mainnet" && network !== "testnet") {
      return res.status(400).json({ error: "Invalid network. Use 'mainnet' or 'testnet'" });
    }

    const metrics = getLatestMetrics(network);

    if (!metrics) {
      return res.status(404).json({ error: `No metrics found for ${network}` });
    }

    res.json({
      success: true,
      data: metrics,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Latest pool data
app.get("/api/pools/:network", (req, res) => {
  try {
    const { network } = req.params;

    if (network !== "mainnet" && network !== "testnet") {
      return res.status(400).json({ error: "Invalid network" });
    }

    const pools = getLatestPools(network);

    res.json({
      success: true,
      data: pools,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// TVL history
app.get("/api/tvl/:network", (req, res) => {
  try {
    const { network } = req.params;
    const days = parseInt(req.query.days as string) || 30;

    if (network !== "mainnet" && network !== "testnet") {
      return res.status(400).json({ error: "Invalid network" });
    }

    const history = getTVLHistory(network, days);

    res.json({
      success: true,
      data: history,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Pool TVL history
app.get("/api/tvl/:network/pool/:poolId", (req, res) => {
  try {
    const { network, poolId } = req.params;
    const days = parseInt(req.query.days as string) || 30;

    if (network !== "mainnet" && network !== "testnet") {
      return res.status(400).json({ error: "Invalid network" });
    }

    const history = getPoolTVLHistory(network, parseInt(poolId), days);

    res.json({
      success: true,
      data: history,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Pool APR history
app.get("/api/apr/:network/pool/:poolId", (req, res) => {
  try {
    const { network, poolId } = req.params;
    const days = parseInt(req.query.days as string) || 30;

    if (network !== "mainnet" && network !== "testnet") {
      return res.status(400).json({ error: "Invalid network" });
    }

    const history = getAPRHistory(network, parseInt(poolId), days);

    res.json({
      success: true,
      data: history,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Recent events
app.get("/api/events/:network", (req, res) => {
  try {
    const { network } = req.params;
    const limit = parseInt(req.query.limit as string) || 100;

    if (network !== "mainnet" && network !== "testnet") {
      return res.status(400).json({ error: "Invalid network" });
    }

    const events = getRecentEvents(network, limit);

    res.json({
      success: true,
      data: events,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Active alerts
app.get("/api/alerts/:network", (req, res) => {
  try {
    const { network } = req.params;

    const alerts = getActiveAlerts(network);

    res.json({
      success: true,
      data: alerts,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Database statistics
app.get("/api/stats", (req, res) => {
  try {
    const stats = getDatabaseStats();

    res.json({
      success: true,
      data: stats,
    });
  } catch (error: any) {
    res.status(500).json({ error: error.message });
  }
});

// Health check
app.get("/api/health", (req, res) => {
  const stats = getDatabaseStats();

  const health = {
    status: "ok",
    timestamp: new Date().toISOString(),
    database: {
      connected: true,
      snapshots: stats.metrics_snapshots,
    },
  };

  res.json(health);
});

// Prometheus metrics endpoint
app.get("/metrics", (req, res) => {
  try {
    const mainnetMetrics = getLatestMetrics("mainnet");
    const mainnetPools = getLatestPools("mainnet");

    if (!mainnetMetrics) {
      return res.status(404).send("# No metrics available");
    }

    let output = "# HELP masterchef_balance MasterChef ÉTR balance\n";
    output += "# TYPE masterchef_balance gauge\n";
    output += `masterchef_balance{network="mainnet"} ${mainnetMetrics.masterchef_balance}\n\n`;

    output += "# HELP masterchef_days_remaining Days of rewards remaining\n";
    output += "# TYPE masterchef_days_remaining gauge\n";
    output += `masterchef_days_remaining{network="mainnet"} ${mainnetMetrics.days_remaining}\n\n`;

    if (mainnetMetrics.total_tvl_usd) {
      output += "# HELP masterchef_tvl_total Total Value Locked in USD\n";
      output += "# TYPE masterchef_tvl_total gauge\n";
      output += `masterchef_tvl_total{network="mainnet"} ${mainnetMetrics.total_tvl_usd}\n\n`;
    }

    output += "# HELP masterchef_pools_total Total number of pools\n";
    output += "# TYPE masterchef_pools_total gauge\n";
    output += `masterchef_pools_total{network="mainnet"} ${mainnetMetrics.total_pools}\n\n`;

    output += "# HELP masterchef_paused Contract paused status (1=paused, 0=active)\n";
    output += "# TYPE masterchef_paused gauge\n";
    output += `masterchef_paused{network="mainnet"} ${mainnetMetrics.is_paused ? 1 : 0}\n\n`;

    // Pool metrics
    for (const pool of mainnetPools) {
      if (pool.tvl_usd) {
        output += `masterchef_pool_tvl{network="mainnet",pool_id="${pool.pool_id}",symbol="${pool.lp_symbol}"} ${pool.tvl_usd}\n`;
      }

      if (pool.apr_percent) {
        output += `masterchef_pool_apr{network="mainnet",pool_id="${pool.pool_id}",symbol="${pool.lp_symbol}"} ${pool.apr_percent}\n`;
      }
    }

    res.setHeader("Content-Type", "text/plain");
    res.send(output);
  } catch (error: any) {
    res.status(500).send(`# Error: ${error.message}`);
  }
});

// 404 handler
app.use((req, res) => {
  res.status(404).json({ error: "Endpoint not found" });
});

// Start server
app.listen(PORT, () => {
  console.log("\n🚀 ANALYTICS API SERVER STARTED\n");
  console.log(`   Port: ${PORT}`);
  console.log(`   URL: http://localhost:${PORT}\n`);
  console.log("📋 Available Endpoints:\n");
  console.log("   GET  /api/metrics/:network");
  console.log("   GET  /api/pools/:network");
  console.log("   GET  /api/tvl/:network?days=30");
  console.log("   GET  /api/tvl/:network/pool/:poolId?days=30");
  console.log("   GET  /api/apr/:network/pool/:poolId?days=30");
  console.log("   GET  /api/events/:network?limit=100");
  console.log("   GET  /api/alerts/:network");
  console.log("   GET  /api/stats");
  console.log("   GET  /api/health");
  console.log("   GET  /metrics (Prometheus)\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
});

export default app;
