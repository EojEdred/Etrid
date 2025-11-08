#!/usr/bin/env node

/**
 * ËTRID Validator Status API
 * Express API endpoint for serving validator metrics to the web dashboard
 */

const express = require('express');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = process.env.VALIDATOR_API_PORT || 3100;

// Enable CORS
app.use(cors());
app.use(express.json());

// Path to the latest report
const REPORTS_DIR = path.join(__dirname, '../../pinokio/reports');

/**
 * Get the latest validator report
 */
function getLatestReport() {
  try {
    if (!fs.existsSync(REPORTS_DIR)) {
      return null;
    }

    const files = fs.readdirSync(REPORTS_DIR)
      .filter(f => f.startsWith('validator-report-') && f.endsWith('.json'))
      .map(f => ({
        name: f,
        time: fs.statSync(path.join(REPORTS_DIR, f)).mtime.getTime()
      }))
      .sort((a, b) => b.time - a.time);

    if (files.length === 0) {
      return null;
    }

    const latestFile = path.join(REPORTS_DIR, files[0].name);
    return JSON.parse(fs.readFileSync(latestFile, 'utf8'));
  } catch (error) {
    console.error('Error reading report:', error);
    return null;
  }
}

/**
 * GET /api/validator-status
 * Returns the current status of all validators
 */
app.get('/api/validator-status', (req, res) => {
  const report = getLatestReport();

  if (!report) {
    return res.status(404).json({
      error: 'No validator report available',
      message: 'Run the AI monitoring tool to generate reports'
    });
  }

  res.json({
    timestamp: report.timestamp,
    summary: report.summary,
    validators: report.validators.map(v => ({
      id: v.validator.id,
      name: v.validator.name,
      region: v.validator.region,
      role: v.validator.role,
      health: v.health,
      metrics: v.metrics,
      alerts: v.alerts,
      accessible: v.accessible
    }))
  });
});

/**
 * GET /api/validator-status/:id
 * Returns the status of a specific validator
 */
app.get('/api/validator-status/:id', (req, res) => {
  const report = getLatestReport();

  if (!report) {
    return res.status(404).json({
      error: 'No validator report available'
    });
  }

  const validator = report.validators.find(v => v.validator.id === parseInt(req.params.id));

  if (!validator) {
    return res.status(404).json({
      error: 'Validator not found'
    });
  }

  res.json({
    id: validator.validator.id,
    name: validator.validator.name,
    region: validator.validator.region,
    role: validator.validator.role,
    health: validator.health,
    metrics: validator.metrics,
    alerts: validator.alerts,
    accessible: validator.accessible,
    timestamp: report.timestamp
  });
});

/**
 * GET /api/network-summary
 * Returns network-wide summary statistics
 */
app.get('/api/network-summary', (req, res) => {
  const report = getLatestReport();

  if (!report) {
    return res.status(404).json({
      error: 'No validator report available'
    });
  }

  res.json({
    timestamp: report.timestamp,
    summary: report.summary,
    recommendations: report.recommendations || []
  });
});

/**
 * GET /api/recommendations
 * Returns AI-generated recommendations
 */
app.get('/api/recommendations', (req, res) => {
  const report = getLatestReport();

  if (!report) {
    return res.status(404).json({
      error: 'No validator report available'
    });
  }

  res.json({
    timestamp: report.timestamp,
    recommendations: report.recommendations || []
  });
});

/**
 * GET /api/health
 * API health check
 */
app.get('/api/health', (req, res) => {
  res.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
    reportsAvailable: fs.existsSync(REPORTS_DIR) && fs.readdirSync(REPORTS_DIR).length > 0
  });
});

/**
 * GET /api/regions
 * Returns validator distribution by region
 */
app.get('/api/regions', (req, res) => {
  const report = getLatestReport();

  if (!report) {
    return res.status(404).json({
      error: 'No validator report available'
    });
  }

  const regionMap = {};

  report.validators.forEach(v => {
    const region = v.validator.region;
    if (!regionMap[region]) {
      regionMap[region] = {
        region,
        count: 0,
        healthy: 0,
        warning: 0,
        critical: 0,
        totalHealth: 0
      };
    }

    regionMap[region].count++;
    regionMap[region].totalHealth += v.health;

    if (v.health >= 80) regionMap[region].healthy++;
    else if (v.health >= 50) regionMap[region].warning++;
    else regionMap[region].critical++;
  });

  const regions = Object.values(regionMap).map(r => ({
    ...r,
    averageHealth: Math.round(r.totalHealth / r.count)
  }));

  res.json({ regions });
});

/**
 * Error handling
 */
app.use((err, req, res, next) => {
  console.error('API Error:', err);
  res.status(500).json({
    error: 'Internal server error',
    message: err.message
  });
});

// Start server
app.listen(PORT, () => {
  console.log(`✅ ËTRID Validator Status API running on http://localhost:${PORT}`);
  console.log(`📊 Endpoints:`);
  console.log(`   GET /api/validator-status       - All validators`);
  console.log(`   GET /api/validator-status/:id   - Specific validator`);
  console.log(`   GET /api/network-summary        - Network summary`);
  console.log(`   GET /api/recommendations        - AI recommendations`);
  console.log(`   GET /api/regions                - Regional distribution`);
  console.log(`   GET /api/health                 - API health check`);
});

module.exports = app;
