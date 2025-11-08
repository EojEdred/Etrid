module.exports = {
  title: "ÉTRID Web UI Suite",
  description: "Complete web interface suite for ÉTRID blockchain - includes Lightning Landing, Validator Dashboard, Watchtower Monitor, and MasterChef Dashboard",
  icon: "icon.png",
  menu: async (kernel) => {
    return {
      web: "http://localhost:3000",
      shell: "npm --version"
    }
  },
  install: [
    {
      method: "shell.run",
      params: {
        message: "Installing Lightning Landing dependencies...",
        venv: "env",
        path: "apps/lightning-landing",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        venv: "env",
        path: "apps/lightning-landing"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Building Lightning Landing...",
        venv: "env",
        path: "apps/lightning-landing",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        venv: "env",
        path: "apps/lightning-landing"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Installing Validator Dashboard dependencies...",
        venv: "env",
        path: "apps/validator-dashboard",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        venv: "env",
        path: "apps/validator-dashboard"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Building Validator Dashboard...",
        venv: "env",
        path: "apps/validator-dashboard",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        venv: "env",
        path: "apps/validator-dashboard"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Installing Watchtower Monitor dependencies...",
        venv: "env",
        path: "apps/watchtower-monitor",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        venv: "env",
        path: "apps/watchtower-monitor"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Building Watchtower Monitor...",
        venv: "env",
        path: "apps/watchtower-monitor",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        venv: "env",
        path: "apps/watchtower-monitor"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Installing MasterChef Dashboard dependencies...",
        venv: "env",
        path: "apps/masterchef-dashboard",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm install",
        venv: "env",
        path: "apps/masterchef-dashboard"
      }
    },
    {
      method: "shell.run",
      params: {
        message: "Building MasterChef Dashboard...",
        venv: "env",
        path: "apps/masterchef-dashboard",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm run build",
        venv: "env",
        path: "apps/masterchef-dashboard"
      }
    }
  ],
  run: [
    {
      method: "shell.run",
      params: {
        message: "Starting Lightning Landing on port 3000...",
        venv: "env",
        path: "apps/lightning-landing",
        on: [{
          event: null,
          return: true
        }]
      }
    },
    {
      method: "shell.run",
      params: {
        message: "npm start",
        venv: "env",
        path: "apps/lightning-landing",
        daemon: true
      }
    }
  ]
}
