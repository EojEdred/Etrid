#!/usr/bin/env node

/**
 * ËTRID Pinokio Integration
 * Unified interface for Web UIs and Validator Management
 */

module.exports = {
  title: "ËTRID Complete Suite",
  description: "Complete ËTRID Web UI Suite and Distributed Validator Management",
  icon: "https://etrid.org/assets/logos/etrid-primary-logo.svg",
  menu: async (kernel) => {
    return {
      "🌐 Web Applications": {
        "⚡ Lightning Landing": { method: "shell.run", params: { message: "open http://localhost:3000" } },
        "👨‍🍳 MasterChef Dashboard": { method: "shell.run", params: { message: "open http://localhost:3001" } },
        "🛡️ Validator Dashboard": { method: "shell.run", params: { message: "open http://localhost:3002" } },
        "👁️ Watchtower Monitor": { method: "shell.run", params: { message: "open http://localhost:3003" } },
        "💼 Wallet Web": { method: "shell.run", params: { message: "open http://localhost:3004" } }
      },
      "🔧 Validator Management": {
        "📋 List All Validators": { method: "shell.run", params: { message: "node pinokio/validator-cli.js list" } },
        "📊 Check All Status": { method: "shell.run", params: { message: "node pinokio/validator-cli.js status-all" } },
        "🤖 AI Monitoring": { method: "shell.run", params: { message: "node pinokio/ai-validator-monitor.js monitor" } },
        "🔄 Continuous Monitoring": { method: "shell.run", params: { message: "node pinokio/ai-validator-monitor.js continuous 10" } }
      },
      "📊 Reports": {
        "📂 View Reports": { method: "shell.run", params: { message: "ls -lh pinokio/reports/" } }
      },
      "📖 Documentation": {
        "📘 Validator CLI": { method: "shell.run", params: { message: "node pinokio/validator-cli.js" } },
        "📗 AI Monitor": { method: "shell.run", params: { message: "node pinokio/ai-validator-monitor.js" } },
        "📙 README": { method: "shell.run", params: { message: "cat pinokio/README.md" } }
      }
    };
  },
  install: [
    {
      method: "shell.run",
      params: {
        message: "echo '📦 Installing ËTRID Complete Suite...'",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "cd pinokio && npm install",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "echo '🌐 Building Web UIs...'",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "cd apps/lightning-landing && npm install && npm run build",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "cd apps/masterchef-dashboard && npm install && npm run build",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "cd apps/validator-dashboard && npm install && npm run build",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "cd apps/watchtower-monitor && npm install && npm run build",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "cd apps/wallet-web/etrid-crypto-website && npm install --legacy-peer-deps && npm run build",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "echo '✅ ËTRID Complete Suite installed successfully!'",
      }
    }
  ],
  run: [
    {
      method: "shell.run",
      params: {
        message: "echo '🚀 Starting ËTRID Complete Suite...'",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "./scripts/start-all-web-uis.sh",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "echo '🤖 Starting AI Validator Monitoring...'",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "node pinokio/ai-validator-monitor.js continuous 10 &",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "echo '✅ ËTRID Suite is running!'",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "echo '🌐 Web UIs: http://localhost:3000-3004'",
      }
    },
    {
      method: "shell.run",
      params: {
        message: "echo '📊 Use the menu above to manage validators'",
      }
    }
  ]
};
