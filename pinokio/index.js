/**
 * ÉTRID Complete Pinokio Integration
 * Combines Web UIs + Validator Management + AI Tools
 */

module.exports = {
  title: "ÉTRID Complete Suite",
  description: "Full ÉTRID ecosystem: Web UIs, Validator Management, and AI-Powered Monitoring for 21 distributed validators",
  icon: "icon.png",
  version: "1.0.0",

  menu: async (kernel) => {
    return {
      "🌐 Web UIs": {
        "Lightning Landing": "http://localhost:3000",
        "MasterChef Dashboard": "http://localhost:3001",
        "Validator Dashboard": "http://localhost:3002",
        "Watchtower Monitor": "http://localhost:3003",
        "Wallet Web": "http://localhost:3004",
      },
      "🔧 Validator Tools": {
        "List Validators": {
          method: "shell.run",
          params: {
            message: "node pinokio/validator-cli.js list",
            on: [{ event: null }]
          }
        },
        "Monitor All Validators": {
          method: "shell.run",
          params: {
            message: "node pinokio/ai-validator-monitor.js monitor",
            on: [{ event: null }]
          }
        },
        "Continuous Monitoring": {
          method: "shell.run",
          params: {
            message: "node pinokio/ai-validator-monitor.js continuous 5",
            daemon: true
          }
        }
      },
      "📊 Reports": "file://./reports",
      "📖 Documentation": "file://./PINOKIO_README.md"
    }
  },

  // Installation steps
  install: [
    // ========================================
    // Web UI Installation
    // ========================================
    {
      method: "shell.run",
      params: {
        message: "📦 Installing Lightning Landing dependencies...",
        path: "apps/lightning-landing",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        path: "apps/lightning-landing"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "🔨 Building Lightning Landing...",
        path: "apps/lightning-landing",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        path: "apps/lightning-landing"
      }
    },

    {
      method: "shell.run",
      params: {
        message: "📦 Installing Validator Dashboard dependencies...",
        path: "apps/validator-dashboard",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        path: "apps/validator-dashboard"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "🔨 Building Validator Dashboard...",
        path: "apps/validator-dashboard",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        path: "apps/validator-dashboard"
      }
    },

    {
      method: "shell.run",
      params: {
        message: "📦 Installing Watchtower Monitor dependencies...",
        path: "apps/watchtower-monitor",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        path: "apps/watchtower-monitor"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "🔨 Building Watchtower Monitor...",
        path: "apps/watchtower-monitor",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        path: "apps/watchtower-monitor"
      }
    },

    {
      method: "shell.run",
      params: {
        message: "📦 Installing MasterChef Dashboard dependencies...",
        path: "apps/masterchef-dashboard",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        path: "apps/masterchef-dashboard"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "🔨 Building MasterChef Dashboard...",
        path: "apps/masterchef-dashboard",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        path: "apps/masterchef-dashboard"
      }
    },

    {
      method: "shell.run",
      params: {
        message: "📦 Installing Wallet Web dependencies...",
        path: "apps/wallet-web/etrid-crypto-website",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install --legacy-peer-deps",
        path: "apps/wallet-web/etrid-crypto-website"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "🔨 Building Wallet Web...",
        path: "apps/wallet-web/etrid-crypto-website",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        path: "apps/wallet-web/etrid-crypto-website"
      }
    },

    // ========================================
    // Validator Tools Setup
    // ========================================
    {
      method: "shell.run",
      params: {
        message: "🔧 Setting up Pinokio validator tools...",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "chmod +x pinokio/*.js",
        on: [{ event: null }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "mkdir -p reports",
        on: [{ event: null }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "✅ Installation complete!",
        on: [{ event: null, return: true }]
      }
    }
  ],

  // Run/Start steps
  run: [
    {
      method: "shell.run",
      params: {
        message: "🚀 Starting ÉTRID Web UI Suite...",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "./scripts/start-all-web-uis.sh",
        daemon: true
      }
    },
    {
      method: "shell.run",
      params: {
        message: "🤖 Starting AI Validator Monitor...",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "node pinokio/ai-validator-monitor.js continuous 10",
        daemon: true
      }
    },
    {
      method: "shell.run",
      params: {
        message: "✅ All services started!\n\nWeb UIs:\n  • Lightning Landing: http://localhost:3000\n  • MasterChef Dashboard: http://localhost:3001\n  • Validator Dashboard: http://localhost:3002\n  • Watchtower Monitor: http://localhost:3003\n  • Wallet Web: http://localhost:3004\n\nMonitoring: Active (checks every 10 minutes)\nReports: ./reports/",
        on: [{ event: null, return: true }]
      }
    }
  ],

  // Stop/Cleanup steps
  stop: [
    {
      method: "shell.run",
      params: {
        message: "🛑 Stopping all web UIs...",
        on: [{ event: null, return: true }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "./scripts/stop-all-web-uis.sh"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "pkill -f 'ai-validator-monitor'"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "✅ All services stopped!",
        on: [{ event: null, return: true }]
      }
    }
  ]
}
